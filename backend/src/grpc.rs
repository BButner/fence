use std::sync::Arc;

use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Response, Status};

use crate::config::Config;
use crate::cursor::{self, UpdateCursorLocationResult};

use self::fence::fence_service_server::FenceService;

use self::fence::CursorLocation;

pub mod fence {
    tonic::include_proto!("fence");
}

#[derive(Debug)]
pub struct State {
    pub(crate) current_config: Config,
    pub(crate) current_regions: Vec<crate::region::Region>,
    pub(crate) last_good_pos: Option<CursorLocation>,
    pub(crate) tx: tokio::sync::broadcast::Sender<CursorLocation>,
    pub(crate) is_active: bool,
}

impl State {
    pub fn new(current_config: Config, tx: tokio::sync::broadcast::Sender<CursorLocation>) -> Self {
        let current_regions = current_config.regions.clone();

        let is_active = current_config.auto_lock;

        Self {
            current_config,
            current_regions,
            last_good_pos: None,
            tx,
            is_active,
        }
    }

    pub fn try_update_cursor_location(&mut self, x: i32, y: i32) -> UpdateCursorLocationResult {
        let time_before = std::time::Instant::now();
        let response = if self.is_active {
            cursor::try_update_cursor_location(x, y, self)
        } else {
            UpdateCursorLocationResult {
                location: CursorLocation { x, y },
                updated: true,
            }
        };
        let time_after_cursor_check = std::time::Instant::now();

        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // print!(
        //     "Cursor check took {}ms and {}ns",
        //     time_after_cursor_check
        //         .duration_since(time_before)
        //         .as_millis(),
        //     time_after_cursor_check
        //         .duration_since(time_before)
        //         .as_nanos()
        // );

        let _ = self.tx.send(CursorLocation {
            x: response.location.x,
            y: response.location.y,
        });

        response
    }
}

#[derive(Debug)]
pub struct FenceManager {
    tx: tokio::sync::broadcast::Sender<CursorLocation>,
    rx: tokio::sync::broadcast::Receiver<CursorLocation>,
    state: Arc<Mutex<State>>,
}

impl FenceManager {
    pub fn new(config: Config) -> Self {
        let (tx, rx) = tokio::sync::broadcast::channel(16);
        let tx_inner = tx.clone();
        FenceManager {
            tx,
            rx,
            state: Arc::new(Mutex::new(State::new(config, tx_inner))),
        }
    }
}

#[tonic::async_trait]
impl FenceService for FenceManager {
    type GetCursorLocationStream = ReceiverStream<Result<CursorLocation, Status>>;
    type GetHeartbeatStream = ReceiverStream<Result<(), Status>>;

    async fn get_heartbeat(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<Response<Self::GetHeartbeatStream>, Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);

        tokio::spawn(async move {
            loop {
                if (tx.send(Ok(())).await).is_err() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_cursor_location(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<Response<Self::GetCursorLocationStream>, Status> {
        let mut rx_raw = self.rx.resubscribe();

        let (tx, rx) = tokio::sync::mpsc::channel(16);

        tokio::spawn(async move {
            loop {
                let msg = rx_raw.recv().await;

                println!("Received message: {:?}", msg);

                match msg {
                    Ok(msg) => {
                        if (tx.send(Ok(msg)).await).is_err() {
                            break;
                        }
                    }
                    Err(e) => match e {
                        tokio::sync::broadcast::error::RecvError::Closed => {
                            println!("Channel closed");
                            break;
                        }
                        tokio::sync::broadcast::error::RecvError::Lagged(_) => {
                            println!("Channel lagged");
                        }
                    },
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_regions(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<fence::GetRegionsResponse>, tonic::Status> {
        let state = self.state.lock().await;

        Ok(tonic::Response::new(fence::GetRegionsResponse {
            regions: state
                .current_config
                .regions
                .iter()
                .map(|region| region.into())
                .collect(),
        }))
    }

    async fn add_region(
        &self,
        request: tonic::Request<fence::AddRegionRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut state = self.state.lock().await;

        if let Some(new_region) = request.get_ref().region.as_ref() {
            state.current_config.regions.push(crate::region::Region {
                x: new_region.x,
                y: new_region.y,
                width: new_region.width,
                height: new_region.height,
                id: new_region.id.to_string(),
            });

            Ok(Response::new(()))
        } else {
            Err(Status::invalid_argument("Invalid region."))
        }
    }

    async fn delete_region_by_id(
        &self,
        request: tonic::Request<fence::DeleteRegionByIdRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut state = self.state.lock().await;

        state
            .current_config
            .regions
            .retain(|region| region.id != request.get_ref().id);

        Ok(Response::new(()))
    }

    async fn save_config(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut state = self.state.lock().await;

        state.current_config.save().await;

        state.current_regions = state.current_config.regions.clone();

        Ok(Response::new(()))
    }

    async fn activate_lock(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut state = self.state.lock().await;

        state.is_active = true;

        Ok(Response::new(()))
    }

    async fn deactivate_lock(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut state = self.state.lock().await;

        state.is_active = false;

        Ok(Response::new(()))
    }
}

pub async fn init_connection() -> Option<Arc<Mutex<State>>> {
    let addr = "0.0.0.0:1234".parse().unwrap();
    let config: Config;

    // Load config
    {
        config = Config::load(None).await?;
    }

    let manager = FenceManager::new(config);
    let state = manager.state.clone();

    let service = fence::fence_service_server::FenceServiceServer::new(manager);

    tokio::spawn(async move {
        let server_result = Server::builder().add_service(service).serve(addr).await;

        match server_result {
            Ok(_) => println!("Server exited gracefully"),
            Err(e) => println!("Server exited with error: {:?}", e),
        }
    });

    println!("Server listening on {}", addr);

    Some(state)
}

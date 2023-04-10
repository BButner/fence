use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Response, Status};

use crate::STATE;

use self::fence::fence_service_server::FenceService;

use self::fence::CursorLocation;

pub mod fence {
    tonic::include_proto!("fence");
}

#[derive(Debug)]
pub struct FenceManager {
    tx: tokio::sync::broadcast::Sender<CursorLocation>,
    rx: tokio::sync::broadcast::Receiver<CursorLocation>,
}

impl FenceManager {
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::broadcast::channel(16);
        FenceManager { tx, rx }
    }
}

#[tonic::async_trait]
impl FenceService for FenceManager {
    type GetCursorLocationStream = ReceiverStream<Result<CursorLocation, Status>>;

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
                        if let Err(_) = tx.send(Ok(msg)).await {
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
        let state = STATE.lock().await;
        let current_config = state.current_config.as_ref().unwrap();
        let mut regions = Vec::new();

        // TODO: Make this better...
        for region in &current_config.regions {
            regions.push(fence::Region {
                id: region.id.to_string(),
                x: region.x,
                y: region.y,
                width: region.width,
                height: region.height,
            });
        }

        Ok(tonic::Response::new(fence::GetRegionsResponse { regions }))
    }

    async fn add_region(
        &self,
        request: tonic::Request<fence::AddRegionRequest>,
    ) -> Result<tonic::Response<()>, tonic::Status> {
        let mut state = STATE.lock().await;
        let new_region = request.get_ref().region.as_ref().unwrap();

        state
            .current_config
            .as_mut()
            .unwrap()
            .regions
            .push(crate::region::Region {
                x: new_region.x,
                y: new_region.y,
                width: new_region.width,
                height: new_region.height,
                id: new_region.id.to_string(),
            });

        Ok(Response::new(()))
    }
}

pub async fn init_connection() -> tokio::sync::broadcast::Sender<CursorLocation> {
    let addr = "0.0.0.0:1234".parse().unwrap();

    let manager = FenceManager::new();
    let tx = manager.tx.clone();

    let service = fence::fence_service_server::FenceServiceServer::new(manager);

    tokio::spawn(async move {
        let server_result = Server::builder().add_service(service).serve(addr).await;

        match server_result {
            Ok(_) => println!("Server exited gracefully"),
            Err(e) => println!("Server exited with error: {:?}", e),
        }
    });

    println!("Server listening on {}", addr);

    tx
}

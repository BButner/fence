use self::fence::fence_service_client::FenceServiceClient;

pub mod fence {
    tonic::include_proto!("fence");
}

pub struct GrpcClient {
    pub client: FenceServiceClient<tonic::transport::Channel>,
}

pub async fn connect_client(hostname: &str) -> Result<GrpcClient, Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_shared(format!("http://{}", hostname))?
        .connect()
        .await?;
    Ok(GrpcClient {
        client: FenceServiceClient::new(channel),
    })
}

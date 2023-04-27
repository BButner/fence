use tonic::transport::Channel;

use self::fence::fence_service_client::FenceServiceClient;

pub mod fence {
    tonic::include_proto!("fence");
}

pub struct GrpcClient {
    pub client: FenceServiceClient<tonic::transport::Channel>,
}

pub async fn connect_client(hostname: &str) -> Result<GrpcClient, Box<dyn std::error::Error>> {
    let channel = Channel::builder(hostname.parse().unwrap())
        .connect()
        .await?;

    let limit = 10 * 1024 * 1024;

    let client = FenceServiceClient::new(channel).max_decoding_message_size(limit);

    Ok(GrpcClient { client })
}

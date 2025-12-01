use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tonic::{Request, Response, Status, transport::Server};

use crate::wubrag::{
    FilePath, IndexResult,
    wub_rag_server::{WubRag, WubRagServer},
};

pub mod wubrag {
    tonic::include_proto!("wubrag");
}

#[derive(Debug, Default)]
pub struct Wub {}

#[tonic::async_trait]
impl WubRag for Wub {
    async fn index(&self, request: Request<FilePath>) -> Result<Response<IndexResult>, Status> {
        println!("Got a request: {:?}", request);

        let reply = IndexResult {
            ok: true,
            err: "".to_string(),
        };

        Ok(Response::new(reply))
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5001);
    let rag = Wub::default();
    Server::builder()
        .add_service(WubRagServer::new(rag))
        .serve(socket)
        .await?;

    Ok(())
}

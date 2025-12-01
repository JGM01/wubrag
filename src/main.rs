use std::{
    fs::File,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};

use tonic::{Request, Response, Status, transport::Server};
use wubraglib::{
    chunking::chunk_all_documents, document::grab_all_documents, embedding::Embedder,
    indexing::Index,
};

use crate::wubrag::{
    FilePath, IndexResult, SearchKey, SearchResult,
    wub_rag_server::{WubRag, WubRagServer},
};

pub mod wubrag {
    tonic::include_proto!("wubrag");
}

#[derive(Debug, Default)]
pub struct Wub {}

#[tonic::async_trait]
impl WubRag for Wub {
    async fn search(&self, request: Request<SearchKey>) -> Result<Response<SearchResult>, Status> {
        let req: SearchKey = request.into_inner();
        let key = req.text;

        let docs = grab_all_documents(Path::new(&key));

        todo!()
    }

    async fn index(&self, request: Request<FilePath>) -> Result<Response<IndexResult>, Status> {
        println!("Got a request: {:?}", request);

        let req: FilePath = request.into_inner();
        let p = req.path;

        let docs = grab_all_documents(Path::new(&p));
        let (mut chunks, _) = chunk_all_documents(&docs);

        let mut embedder = Embedder::new();

        let embeddings = embedder.embed_chunks(&mut chunks);

        let index = Index::new(chunks, embeddings);

        let embedder = fastembed::TextEmbedding::try_new(fastembed::InitOptions::new(
            fastembed::EmbeddingModel::AllMiniLML6V2,
        ))
        .expect("failed to init embedder");

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

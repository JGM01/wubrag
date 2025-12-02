use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
    sync::Arc,
};
use tokio::sync::RwLock;
use tonic::{Request, Response, Status, transport::Server};
use wubraglib::{
    chunking::chunk_all_documents, document::grab_all_documents, embedding::Embedder,
    indexing::Index,
};

/// Generated proto code (from OUT_DIR/wubrag.rs)
pub mod wubrag {
    tonic::include_proto!("wubrag");
}

/// Descriptor bytes included at compile-time (from OUT_DIR/wubrag_descriptor.bin)
/// This MUST be at item level (outside functions). Do NOT put this inside `main`.
pub mod descriptors {
    tonic::include_file_descriptor_set!("wubrag_descriptor");
}

/// Re-export the generated proto types for convenient use
use crate::wubrag::{
    FilePath, IndexResult, SearchKey, SearchResult,
    wub_rag_server::{WubRag, WubRagServer},
};

pub struct Wub {
    state: Arc<RwLock<RAGState>>,
}

impl Wub {
    fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(RAGState::new())),
        }
    }
}

pub struct RAGState {
    index: Option<Index<'static>>,
    embedder: Embedder,
}

impl RAGState {
    fn new() -> Self {
        Self {
            index: None,
            embedder: Embedder::new(),
        }
    }
}

#[tonic::async_trait]
impl WubRag for Wub {
    async fn search(&self, request: Request<SearchKey>) -> Result<Response<SearchResult>, Status> {
        let req: SearchKey = request.into_inner();
        let query_text = req.text;

        let mut state = self.state.write().await;

        if state.index.is_none() {
            return Err(Status::failed_precondition(
                "No index available. Please index documents first.",
            ));
        }

        let query_embedding = state
            .embedder
            .embed_chunks(&[wubraglib::chunking::Chunk {
                id: [0; 32],
                doc_id: [0; 32],
                text: query_text.clone(),
                chunk_type: "query",
                char_count: query_text.len(),
            }])
            .into_iter()
            .next()
            .ok_or_else(|| Status::internal("Failed to generate query embedding"))?;

        let k = 5;
        let ef_search = 50;
        let results = state
            .index
            .as_ref()
            .unwrap()
            .search(&query_embedding, k, ef_search);

        let result_strings: Vec<String> = results
            .iter()
            .map(|(idx, similarity)| {
                let chunk = state.index.as_ref().unwrap().retrieve(*idx);
                format!(
                    "Score: {:.4}\nFile: {}\nType: {}\n---\n{}\n",
                    similarity,
                    chunk
                        .doc_id
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .take(8)
                        .collect::<String>(),
                    chunk.chunk_type,
                    chunk.text
                )
            })
            .collect();

        let reply = SearchResult {
            result_strings: result_strings.clone(),
            string_amt: result_strings.len() as i32,
        };

        Ok(Response::new(reply))
    }

    async fn index(&self, request: Request<FilePath>) -> Result<Response<IndexResult>, Status> {
        println!("Got an index request: {:?}", request);

        let req: FilePath = request.into_inner();
        let p = req.path;

        let docs = grab_all_documents(Path::new(&p));
        if docs.is_empty() {
            return Ok(Response::new(IndexResult {
                ok: false,
                err: "No documents found at the specified path".to_string(),
            }));
        }

        let (chunks, _) = chunk_all_documents(&docs);
        if chunks.is_empty() {
            return Ok(Response::new(IndexResult {
                ok: false,
                err: "No chunks produced from documents".to_string(),
            }));
        }

        let mut state = self.state.write().await;

        let embeddings = state.embedder.embed_chunks(&chunks);

        let index = Index::new(chunks, embeddings);

        state.index = Some(index);

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
    let rag = Wub::new();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(descriptors::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    println!("WubRAG server starting on {}", socket);

    Server::builder()
        .add_service(reflection)
        .add_service(WubRagServer::new(rag))
        .serve(socket)
        .await?;

    Ok(())
}

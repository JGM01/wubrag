use hnsw_rs::{hnsw::Hnsw, prelude::DistCosine};

use crate::chunking::Chunk;

pub struct Index<'a> {
    pub chunks: Vec<Chunk>,
    hnsw_index: Hnsw<'a, f32, DistCosine>,
}

impl<'a> Index<'a> {
    pub fn new(chunks: Vec<Chunk>, embeddings: Vec<Vec<f32>>) -> Self {
        let embedding_dim = embeddings.first().unwrap().len();

        let max_elements = embeddings.len();
        let ef_construction = 200;
        let max_nb_connection = 16;

        let hnsw_index = Hnsw::<f32, DistCosine>::new(
            max_nb_connection,
            max_elements,
            ef_construction,
            embedding_dim,
            DistCosine {},
        );

        for (idx, embedding) in embeddings.iter().enumerate() {
            hnsw_index.insert((embedding.as_slice(), idx));
        }

        Self { chunks, hnsw_index }
    }

    pub fn search(&self, query: &[f32], k: usize, ef_search: usize) -> Vec<(usize, f32)> {
        let neighbors = self.hnsw_index.search(query, k, ef_search);

        let results = neighbors
            .into_iter()
            .map(|neighbor| {
                let idx = neighbor.d_id;
                let distance = neighbor.distance;
                let similarity = 1.0 - distance;
                (idx, similarity)
            })
            .collect();

        results
    }

    pub fn retrieve(&self, idx: usize) -> &Chunk {
        &self.chunks[idx]
    }
}

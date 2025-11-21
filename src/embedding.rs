use fastembed::{InitOptions, TextEmbedding};

use crate::chunking::Chunk;

pub struct Embedder {
    model: TextEmbedding,
}

impl Embedder {
    pub fn new() -> Self {
        let model = TextEmbedding::try_new(
            InitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
                .with_show_download_progress(true),
        )
        .expect("model init failed");

        Self { model }
    }

    pub fn embed_chunks(&mut self, chunks: &[Chunk]) -> Vec<Vec<f32>> {
        let batch_size = 256;

        let mut all = Vec::with_capacity(chunks.len());

        for batch in chunks.chunks(batch_size) {
            let texts: Vec<&str> = batch.iter().map(|c| c.text.as_str()).collect();
            let embeddings = self.model.embed(texts, None).expect("batch failed");
            all.extend(embeddings);
        }

        all
    }
}

extern crate wubrag;

use std::{collections::HashSet, path::Path};

use wubrag::*;

#[test]
fn test_ids_are_unique() {
    let mut map: HashSet<u32> = HashSet::new();
    let docs = grab_all_documents(Path::new("tests/examples/example-rs"));
    for doc in &docs {
        assert!(!map.contains(&doc.id));
        map.insert(doc.id);
    }
    assert!(true);
}
#[test]
fn test_chunks() {
    let docs = grab_all_documents(Path::new("tests/examples/example-rs"));
    let _ = chunk_all_documents(&docs);
    assert!(true);
}
#[test]
fn test_run_query_returns_text() {
    let docs = grab_all_documents(Path::new("tests/examples/example-rs"));

    let (mut chunks, _id_to_idx) = chunk_all_documents(&docs);
    chunks.truncate(20);

    let embeddings = embed_chunks(&mut chunks);

    let index = Index::new(chunks, embeddings);

    let embedder = fastembed::TextEmbedding::try_new(fastembed::InitOptions::new(
        fastembed::EmbeddingModel::AllMiniLML6V2,
    ))
    .expect("failed to init embedder");

    let results = index.query(embedder, "example-rs", 1);

    for text in results.iter() {
        println!("{text}\n---\n");
    }

    assert!(!results.is_empty());
    assert!(results.iter().all(|s| !s.trim().is_empty()));
}

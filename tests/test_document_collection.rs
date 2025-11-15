extern crate wubrag;

use std::{collections::HashSet, path::Path};

use wubrag::*;

#[test]
fn test_ids_are_unique() {
    let mut map: HashSet<u32> = HashSet::new();
    let docs = grab_all_documents(Path::new("tests/examples/ratatui"));
    for doc in &docs {
        assert!(!map.contains(&doc.id));
        map.insert(doc.id);
    }
    assert!(true);
}
#[test]
fn test_chunks() {
    let docs = grab_all_documents(Path::new("tests/examples/ladybird"));
    let _ = chunk_all_documents(&docs);
    assert!(true);
}

#[test]
fn print_chunks() {
    let docs = grab_all_documents(Path::new("tests/examples/ladybird"));
    let chunks = chunk_all_documents(&docs);
    print_chunks_vec(&chunks.0);
    assert!(true);
}

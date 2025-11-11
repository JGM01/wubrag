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
    let (chunks, index) = chunk_all_documents(&docs);
    print_chunks_tree(&chunks, Some(&index));
    assert!(true);
}

#[test]
fn print_doc_names() {
    let docs = grab_all_documents(Path::new("tests/examples/ladybird"));
    print_document_names(&docs);
    assert!(true);
}

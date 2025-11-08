extern crate wubrag;

use std::{collections::HashSet, path::Path};

use wubrag::*;

#[test]
fn test_ids_are_unique() {
    let mut map: HashSet<u32> = HashSet::new();
    let docs = grab_all_documents(Path::new("tests/examples/ladybird"));
    for doc in &docs {
        assert!(!map.contains(&doc.id));
        map.insert(doc.id);
    }
    assert!(true);
}
#[test]
fn test_chunks() {
    let docs = grab_all_documents(Path::new("tests/examples/example-rs"));
    let (chunks, index) = chunk_all_documents(&docs);
    print_chunks_tree(&chunks, Some(&index));
    assert!(true);
}

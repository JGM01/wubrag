extern crate wubrag;

use std::path::Path;

use wubrag::*;

#[test]
fn test_print_stats() {
    let _ = grab_all_documents(Path::new("tests/examples/ladybird"));
    assert!(true);
}

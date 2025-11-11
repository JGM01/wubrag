use criterion::{Criterion, criterion_group, criterion_main};
use std::path::Path;

use wubrag::*;

fn bench_grab_documents(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/ratatui");

    c.bench_function("grab_all_documents", |b| {
        b.iter(|| {
            let docs = grab_all_documents(std::hint::black_box(&root_path));
            std::hint::black_box(docs);
        })
    });
}

criterion_group! {
    name = doc_benches;
    config = Criterion::default().sample_size(10);
    targets = bench_grab_documents
}

criterion_main!(doc_benches);

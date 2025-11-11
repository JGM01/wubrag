use criterion::{Criterion, criterion_group, criterion_main};
use std::path::Path;
use wubrag::*;

fn bench_chunk_documents(c: &mut Criterion) {
    let root_path = Path::new(".");
    let docs = grab_all_documents(std::hint::black_box(&root_path));

    c.bench_function("chunk_all_documents", |b| {
        b.iter(|| {
            let _ = chunk_all_documents(std::hint::black_box(&docs));
        })
    });
}

criterion_group! {
    name = doc_benches;
    config = Criterion::default().sample_size(10);
    targets = bench_chunk_documents
}

criterion_main!(doc_benches);

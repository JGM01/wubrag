use criterion::{Criterion, criterion_group, criterion_main};
use std::path::Path;
use wubrag::*;

fn bench_embed_coreutils(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/coreutils");
    let docs = grab_all_documents(std::hint::black_box(&root_path));
    let (mut chunks, _) = chunk_all_documents(std::hint::black_box(&docs));

    c.bench_function("embed_coreutils", |b| {
        b.iter(|| {
            let _ = embed_chunks(std::hint::black_box(&mut chunks));
        })
    });
}
fn bench_embed_ladybird(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/ladybird");
    let docs = grab_all_documents(std::hint::black_box(&root_path));
    let (mut chunks, _) = chunk_all_documents(std::hint::black_box(&docs));

    c.bench_function("embed_ladybird", |b| {
        b.iter(|| {
            let _ = embed_chunks(std::hint::black_box(&mut chunks));
        })
    });
}
fn bench_embed_dolphin(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/dolphin");
    let docs = grab_all_documents(std::hint::black_box(&root_path));
    let (mut chunks, _) = chunk_all_documents(std::hint::black_box(&docs));

    c.bench_function("embed_dolphin", |b| {
        b.iter(|| {
            let _ = embed_chunks(std::hint::black_box(&mut chunks));
        })
    });
}
fn bench_embed_ratatui(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/ratatui");
    let docs = grab_all_documents(std::hint::black_box(&root_path));
    let (mut chunks, _) = chunk_all_documents(std::hint::black_box(&docs));

    c.bench_function("embed_ratatui", |b| {
        b.iter(|| {
            let _ = embed_chunks(std::hint::black_box(&mut chunks));
        })
    });
}

criterion_group! {
    name = doc_benches;
    config = Criterion::default();
    targets = bench_embed_ratatui, bench_embed_dolphin, bench_embed_ladybird, bench_embed_coreutils
}

criterion_main!(doc_benches);

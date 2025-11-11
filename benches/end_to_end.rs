use criterion::{Criterion, criterion_group, criterion_main};
use std::path::Path;
use wubrag::*;

fn bench_end_to_end_ladybird(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/ladybird");

    c.bench_function("end_to_end_ladybird", |b| {
        b.iter(|| {
            let docs = grab_all_documents(std::hint::black_box(&root_path));
            let _ = chunk_all_documents(std::hint::black_box(&docs));
        })
    });
}
fn bench_end_to_end_ratatui(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/ratatui");

    c.bench_function("end_to_end_ratatui", |b| {
        b.iter(|| {
            let docs = grab_all_documents(std::hint::black_box(&root_path));
            let _ = chunk_all_documents(std::hint::black_box(&docs));
        })
    });
}
fn bench_end_to_end_dolphin(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/dolphin");

    c.bench_function("end_to_end_dolphin", |b| {
        b.iter(|| {
            let docs = grab_all_documents(std::hint::black_box(&root_path));
            let _ = chunk_all_documents(std::hint::black_box(&docs));
        })
    });
}
fn bench_end_to_end_coreutils(c: &mut Criterion) {
    let root_path = Path::new("tests/examples/coreutils");

    c.bench_function("end_to_end_coreutils", |b| {
        b.iter(|| {
            let docs = grab_all_documents(std::hint::black_box(&root_path));
            let _ = chunk_all_documents(std::hint::black_box(&docs));
        })
    });
}

criterion_group! {
    name = doc_benches;
    config = Criterion::default().sample_size(10);
    targets = bench_end_to_end_ladybird, bench_end_to_end_dolphin, bench_end_to_end_ratatui, bench_end_to_end_coreutils
}

criterion_main!(doc_benches);

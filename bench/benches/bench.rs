use std::{hint::black_box, path::PathBuf};

use criterion::{Criterion, criterion_group, criterion_main};
use get_dir::{FileTarget, GetDir, Target, tokio::GetDirAsyncExt as _};
use tokio::runtime::Runtime;

// root -> root/bench/benches/bench.rs
fn bench_get_dir(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get Dir");

    let root: PathBuf = GetDir::new()
        .target(Target::File(FileTarget::new("Cargo.lock")))
        .run_reverse()
        .unwrap();

    group.bench_function("Sync", |b| {
        b.iter(|| {
            let result: PathBuf = GetDir::new()
                .dir(&root)
                .target(Target::File(FileTarget::new("bench.rs")))
                .run()
                .unwrap();

            black_box(result);
        });
    });

    group.bench_function("Tokio", |b| {
        let runtime: Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        b.to_async(runtime).iter(async || {
            let result: PathBuf = GetDir::new()
                .dir(&root)
                .target(Target::File(FileTarget::new("bench.rs")))
                .run_async()
                .await
                .unwrap();

            black_box(result);
        });
    });

    group.finish();
}

// root/bench/benches/bench.rs -> root
fn bench_get_dir_reverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get Dir Reverse");

    let root: PathBuf = GetDir::new()
        .target(Target::File(FileTarget::new("bench.rs")))
        .run()
        .unwrap();

    group.bench_function("Sync", |b| {
        b.iter(|| {
            let result: PathBuf = GetDir::new()
                .dir(&root)
                .target(Target::File(FileTarget::new("Cargo.lock")))
                .run_reverse()
                .unwrap();

            black_box(result);
        });
    });

    group.bench_function("Tokio", |b| {
        let runtime: Runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        b.to_async(runtime).iter(async || {
            let result: PathBuf = GetDir::new()
                .dir(&root)
                .target(Target::File(FileTarget::new("Cargo.lock")))
                .run_reverse_async()
                .await
                .unwrap();

            black_box(result);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    // Get Dir
    bench_get_dir,
    // Get Dir Reverse
    bench_get_dir_reverse,
);
criterion_main!(benches);

//! Secret Sharing Performance Benchmarks
//!
//! This benchmark suite tests the performance of Shamir's Secret Sharing implementation:
//! - Basic operations: split and recover
//! - Scalability: different threshold and total_shares combinations
//! - Comparison: split vs split_with_seed

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mpc_framework_core::SecretSharing;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// Create a deterministic RNG for reproducible benchmarks
fn create_bench_rng() -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(12345)
}

/// Benchmark basic split operation
fn bench_split_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("secret_sharing/split_basic");

    let secret = 123456789u64;
    let threshold = 3;
    let total_shares = 5;

    group.throughput(Throughput::Elements(1));

    group.bench_function("split_t3_n5", |b| {
        let mut rng = create_bench_rng();
        b.iter(|| {
            SecretSharing::split(black_box(secret), threshold, total_shares, &mut rng)
                .expect("split should succeed")
        })
    });

    group.finish();
}

/// Benchmark basic recover operation
fn bench_recover_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("secret_sharing/recover_basic");

    let secret = 123456789u64;
    let threshold = 3;
    let total_shares = 5;

    // Pre-generate shares for recovery benchmark
    let mut rng = create_bench_rng();
    let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
        .expect("split should succeed");
    let shares = sharing.get_shares();

    group.throughput(Throughput::Elements(1));

    group.bench_function("recover_t3_n5", |b| {
        b.iter(|| {
            SecretSharing::recover(black_box(&shares[0..threshold]))
                .expect("recover should succeed")
        })
    });

    group.finish();
}

/// Benchmark scalability with different threshold values
fn bench_scalability_threshold(c: &mut Criterion) {
    let mut group = c.benchmark_group("secret_sharing/scalability_threshold");

    let secret = 987654321u64;
    let thresholds = [2, 3, 5, 10];

    for &threshold in &thresholds {
        let total_shares = threshold + 5; // Always have 5 extra shares

        group.bench_with_input(
            BenchmarkId::new("split", format!("t{}_n{}", threshold, total_shares)),
            &(threshold, total_shares),
            |b, &(t, n)| {
                let mut rng = create_bench_rng();
                b.iter(|| {
                    SecretSharing::split(black_box(secret), t, n, &mut rng)
                        .expect("split should succeed")
                })
            },
        );

        // Pre-generate shares for recovery
        let mut rng = create_bench_rng();
        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("split should succeed");
        let shares = sharing.get_shares().to_vec();

        group.bench_with_input(
            BenchmarkId::new("recover", format!("t{}_n{}", threshold, total_shares)),
            &threshold,
            |b, &t| {
                b.iter(|| {
                    SecretSharing::recover(black_box(&shares[0..t]))
                        .expect("recover should succeed")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark scalability with different total_shares values
fn bench_scalability_shares(c: &mut Criterion) {
    let mut group = c.benchmark_group("secret_sharing/scalability_shares");

    let secret = 555666777u64;
    let threshold = 3;
    let share_counts = [5, 10, 20, 50, 100];

    for &total_shares in &share_counts {
        group.bench_with_input(
            BenchmarkId::new("split", format!("t{}_n{}", threshold, total_shares)),
            &total_shares,
            |b, &n| {
                let mut rng = create_bench_rng();
                b.iter(|| {
                    SecretSharing::split(black_box(secret), threshold, n, &mut rng)
                        .expect("split should succeed")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark split vs split_with_seed comparison
fn bench_split_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("secret_sharing/split_comparison");

    let secret = 111222333u64;
    let threshold = 3;
    let total_shares = 5;
    let seed = 42u64;

    group.throughput(Throughput::Elements(1));

    group.bench_function("split_random", |b| {
        let mut rng = create_bench_rng();
        b.iter(|| {
            SecretSharing::split(black_box(secret), threshold, total_shares, &mut rng)
                .expect("split should succeed")
        })
    });

    group.bench_function("split_with_seed", |b| {
        b.iter(|| {
            SecretSharing::split_with_seed(black_box(secret), threshold, total_shares, seed)
                .expect("split_with_seed should succeed")
        })
    });

    group.finish();
}

/// Benchmark end-to-end split and recover
fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("secret_sharing/end_to_end");

    let secret = 999888777u64;
    let configs = [(2, 3), (3, 5), (5, 10), (10, 20)];

    for &(threshold, total_shares) in &configs {
        group.bench_with_input(
            BenchmarkId::new(
                "split_and_recover",
                format!("t{}_n{}", threshold, total_shares),
            ),
            &(threshold, total_shares),
            |b, &(t, n)| {
                let mut rng = create_bench_rng();
                b.iter(|| {
                    let sharing = SecretSharing::split(black_box(secret), t, n, &mut rng)
                        .expect("split should succeed");
                    let shares = sharing.get_shares();
                    SecretSharing::recover(&shares[0..t]).expect("recover should succeed")
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_split_basic,
    bench_recover_basic,
    bench_scalability_threshold,
    bench_scalability_shares,
    bench_split_comparison,
    bench_end_to_end,
);

criterion_main!(benches);

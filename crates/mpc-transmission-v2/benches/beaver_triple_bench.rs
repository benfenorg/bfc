//! Beaver Triple Performance Benchmarks
//!
//! This benchmark suite tests the performance of Beaver Triple operations:
//! - Basic operations: triple creation, verification, multiplication protocol
//! - Scalability: different threshold and share configurations
//! - Cache efficiency: BeaverTripleCache hit rate and retrieval speed

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mpc_transmission_v2::beaver::{BeaverMultiplication, BeaverTriple, BeaverTripleDistributor};
use mpc_transmission_v2::beaver_cache::BeaverTripleCache;
use mpc_transmission_v2::field::gf64_sss::FieldElement;
use mpc_transmission_v2::field::FieldElement as FieldElementTrait;
use mpc_transmission_v2::poly::Polynomial;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

/// Create a deterministic RNG for reproducible benchmarks
fn create_bench_rng() -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(12345)
}

/// Generate x-coordinates for share generation
fn generate_x_coords(count: usize, rng: &mut ChaCha20Rng) -> Vec<FieldElement> {
    use rand_core::RngCore;
    (0..count)
        .map(|_| FieldElementTrait::from_u64(rng.next_u64()))
        .collect()
}

/// Benchmark basic Beaver triple creation
fn bench_triple_creation_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/creation_basic");

    let threshold = 3;
    let total_shares = 5;
    let a = 7u64;
    let b = 11u64;

    group.throughput(Throughput::Elements(1));

    group.bench_function("new_t3_n5", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            BeaverTriple::new(
                black_box(a),
                black_box(b),
                threshold,
                total_shares,
                &mut rng,
            )
            .expect("triple creation should succeed")
        })
    });

    group.finish();
}

/// Benchmark Beaver triple creation with specified coordinates
fn bench_triple_creation_with_coords(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/creation_with_coords");

    let threshold = 3;
    let total_shares = 5;
    let a = 7u64;
    let b = 11u64;

    let mut rng = create_bench_rng();
    let x_coords = generate_x_coords(total_shares, &mut rng);

    group.throughput(Throughput::Elements(1));

    group.bench_function("new_with_coordinates_t3_n5", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            BeaverTriple::new_with_coordinates(
                black_box(a),
                black_box(b),
                black_box(&x_coords),
                threshold,
                &mut rng,
            )
            .expect("triple creation should succeed")
        })
    });

    group.finish();
}

/// Benchmark Beaver triple verification
fn bench_triple_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/verification");

    let threshold = 3;
    let total_shares = 5;

    // Pre-create a triple for verification
    let mut rng = create_bench_rng();
    let triple = BeaverTriple::new(7, 11, threshold, total_shares, &mut rng)
        .expect("triple creation should succeed");

    group.throughput(Throughput::Elements(1));

    group.bench_function("verify_t3_n5", |bench| {
        bench.iter(|| triple.verify().expect("verification should succeed"))
    });

    group.finish();
}

/// Benchmark scalability with different threshold values
fn bench_scalability_threshold(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/scalability_threshold");

    let thresholds = [2, 3, 5, 10];
    let a = 13u64;
    let b = 17u64;

    for &threshold in &thresholds {
        let total_shares = threshold + 5;

        group.bench_with_input(
            BenchmarkId::new("create", format!("t{}_n{}", threshold, total_shares)),
            &(threshold, total_shares),
            |bench, &(t, n)| {
                let mut rng = create_bench_rng();
                bench.iter(|| {
                    BeaverTriple::new(black_box(a), black_box(b), t, n, &mut rng)
                        .expect("triple creation should succeed")
                })
            },
        );

        // Pre-create triple for verification benchmark
        let mut rng = create_bench_rng();
        let triple =
            BeaverTriple::new(a, b, threshold, total_shares, &mut rng).expect("creation failed");

        group.bench_with_input(
            BenchmarkId::new("verify", format!("t{}_n{}", threshold, total_shares)),
            &triple,
            |bench, t| bench.iter(|| t.verify().expect("verification should succeed")),
        );
    }

    group.finish();
}

/// Benchmark Beaver multiplication protocol
fn bench_multiplication_protocol(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/multiplication");

    let threshold = 3;
    let total_shares = 5;
    let x_secret = 23u64;
    let y_secret = 29u64;

    // Generate consistent x-coordinates
    let mut rng = create_bench_rng();
    let x_coords: Vec<FieldElement> = (1..=total_shares)
        .map(|i| FieldElementTrait::from_u64(i as u64))
        .collect();

    // Create shares with consistent coordinates
    let poly_x = Polynomial::new(
        threshold - 1,
        FieldElementTrait::from_u64(x_secret),
        &mut rng,
    );
    let x_shares: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_x.evaluate(&x))).collect();

    let poly_y = Polynomial::new(
        threshold - 1,
        FieldElementTrait::from_u64(y_secret),
        &mut rng,
    );
    let y_shares: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_y.evaluate(&x))).collect();

    // Create Beaver triple with matching coordinates
    let triple = BeaverTriple::new_with_coordinates(7, 11, &x_coords, threshold, &mut rng)
        .expect("triple creation should succeed");

    group.throughput(Throughput::Elements(1));

    group.bench_function("multiply_with_beaver_t3_n5", |bench| {
        bench.iter(|| {
            BeaverMultiplication::multiply_with_beaver(
                black_box(&x_shares),
                black_box(&y_shares),
                black_box(&triple),
            )
            .expect("multiplication should succeed")
        })
    });

    group.finish();
}

/// Benchmark BeaverTripleDistributor
fn bench_distributor(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/distributor");

    let threshold = 3;
    let total_shares = 5;
    let cache_size = 10;

    group.throughput(Throughput::Elements(1));

    // Benchmark distributor creation
    group.bench_function("create_distributor", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            BeaverTripleDistributor::new(threshold, total_shares, cache_size, &mut rng)
                .expect("distributor creation should succeed")
        })
    });

    // Benchmark getting triples from distributor (cache hit)
    let mut rng = create_bench_rng();
    let mut distributor =
        BeaverTripleDistributor::new(threshold, total_shares, cache_size, &mut rng)
            .expect("distributor creation should succeed");

    group.bench_function("get_triple_from_cache", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            distributor
                .get_triple(&mut rng)
                .expect("get_triple should succeed")
        })
    });

    group.finish();
}

/// Benchmark BeaverTripleCache operations
fn bench_cache_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/cache");

    let threshold = 3;
    let total_shares = 5;
    let cache_capacity = 100;

    let mut rng = create_bench_rng();
    let x_coords: Vec<FieldElement> = (1..=total_shares)
        .map(|i| FieldElementTrait::from_u64(i as u64))
        .collect();

    group.throughput(Throughput::Elements(1));

    // Benchmark cache warmup
    group.bench_function("warmup_10", |bench| {
        bench.iter(|| {
            let mut cache = BeaverTripleCache::new(cache_capacity);
            let mut rng = create_bench_rng();
            cache
                .warmup(10, &x_coords, threshold, &mut rng)
                .expect("warmup should succeed")
        })
    });

    // Benchmark cache get (with warmup)
    let mut cache = BeaverTripleCache::new(cache_capacity);
    cache
        .warmup(50, &x_coords, threshold, &mut rng)
        .expect("warmup should succeed");

    group.bench_function("get_triple_cache_hit", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            cache
                .get_triple(&x_coords, threshold, &mut rng)
                .expect("get_triple should succeed")
        })
    });

    // Benchmark cache miss (empty cache)
    group.bench_function("get_triple_cache_miss", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            let mut empty_cache = BeaverTripleCache::new(cache_capacity);
            empty_cache
                .get_triple(&x_coords, threshold, &mut rng)
                .expect("get_triple should succeed")
        })
    });

    group.finish();
}

/// Benchmark end-to-end secure multiplication
fn bench_end_to_end_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("beaver_triple/end_to_end");

    let configs = [(2, 3), (3, 5), (5, 10)];

    for &(threshold, total_shares) in &configs {
        group.bench_with_input(
            BenchmarkId::new(
                "secure_multiply",
                format!("t{}_n{}", threshold, total_shares),
            ),
            &(threshold, total_shares),
            |bench, &(t, n)| {
                let mut rng = create_bench_rng();
                let x_coords: Vec<FieldElement> = (1..=n)
                    .map(|i| FieldElementTrait::from_u64(i as u64))
                    .collect();

                let x_secret = 5u64;
                let y_secret = 7u64;

                let poly_x =
                    Polynomial::new(t - 1, FieldElementTrait::from_u64(x_secret), &mut rng);
                let x_shares: Vec<(FieldElement, FieldElement)> =
                    x_coords.iter().map(|&x| (x, poly_x.evaluate(&x))).collect();

                let poly_y =
                    Polynomial::new(t - 1, FieldElementTrait::from_u64(y_secret), &mut rng);
                let y_shares: Vec<(FieldElement, FieldElement)> =
                    x_coords.iter().map(|&x| (x, poly_y.evaluate(&x))).collect();

                bench.iter(|| {
                    let mut rng = create_bench_rng();
                    let triple = BeaverTriple::new_with_coordinates(13, 17, &x_coords, t, &mut rng)
                        .expect("triple creation should succeed");
                    BeaverMultiplication::multiply_with_beaver(
                        black_box(&x_shares),
                        black_box(&y_shares),
                        black_box(&triple),
                    )
                    .expect("multiplication should succeed")
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_triple_creation_basic,
    bench_triple_creation_with_coords,
    bench_triple_verification,
    bench_scalability_threshold,
    bench_multiplication_protocol,
    bench_distributor,
    bench_cache_operations,
    bench_end_to_end_multiplication,
);

criterion_main!(benches);

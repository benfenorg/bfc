//! Homomorphic Operations Performance Benchmarks
//!
//! This benchmark suite tests the performance of homomorphic operations:
//! - Basic operations: add_shares, subtract_shares, multiply_by_constant
//! - Complex expressions: multi-step homomorphic computations
//! - Beaver multiplication: secure multiplication using Beaver triples

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mpc_transmission_v2::beaver::BeaverTriple;
use mpc_transmission_v2::field::gf64_sss::FieldElement;
use mpc_transmission_v2::field::FieldElement as FieldElementTrait;
use mpc_transmission_v2::math::HomomorphicOperations;
use mpc_transmission_v2::poly::Polynomial;
use mpc_transmission_v2::SecretSharing;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;

/// Create a deterministic RNG for reproducible benchmarks
fn create_bench_rng() -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(12345)
}

/// Generate compatible shares for two secrets with matching x-coordinates
fn create_compatible_shares(
    secret_a: u64,
    secret_b: u64,
    threshold: usize,
    total_shares: usize,
    rng: &mut ChaCha20Rng,
) -> (
    Vec<(FieldElement, FieldElement)>,
    Vec<(FieldElement, FieldElement)>,
    Vec<FieldElement>,
) {
    // Generate same x coordinates
    let x_coords: Vec<FieldElement> = (0..total_shares)
        .map(|_| FieldElementTrait::from_u64(rng.next_u64()))
        .collect();

    // Create polynomial for first secret and evaluate
    let poly_a = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(secret_a), rng);
    let shares_a: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_a.evaluate(&x))).collect();

    // Create polynomial for second secret and evaluate at same x coordinates
    let poly_b = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(secret_b), rng);
    let shares_b: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_b.evaluate(&x))).collect();

    (shares_a, shares_b, x_coords)
}

/// Benchmark basic add_shares operation
fn bench_add_shares_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/add_shares_basic");

    let secret_a = 100u64;
    let secret_b = 200u64;
    let threshold = 3;
    let total_shares = 5;

    let mut rng = create_bench_rng();
    let (shares_a, shares_b, _) =
        create_compatible_shares(secret_a, secret_b, threshold, total_shares, &mut rng);

    group.throughput(Throughput::Elements(1));

    group.bench_function("add_t3_n5", |bench| {
        bench.iter(|| {
            HomomorphicOperations::add_shares(black_box(&shares_a), black_box(&shares_b))
                .expect("add should succeed")
        })
    });

    group.finish();
}

/// Benchmark basic subtract_shares operation
fn bench_subtract_shares_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/subtract_shares_basic");

    let secret_a = 1000u64;
    let secret_b = 300u64;
    let threshold = 3;
    let total_shares = 5;

    let mut rng = create_bench_rng();
    let (shares_a, shares_b, _) =
        create_compatible_shares(secret_a, secret_b, threshold, total_shares, &mut rng);

    group.throughput(Throughput::Elements(1));

    group.bench_function("subtract_t3_n5", |bench| {
        bench.iter(|| {
            HomomorphicOperations::subtract_shares(black_box(&shares_a), black_box(&shares_b))
                .expect("subtract should succeed")
        })
    });

    group.finish();
}

/// Benchmark multiply_by_constant operation
fn bench_multiply_by_constant(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/multiply_by_constant");

    let secret = 50u64;
    let constant = 7u64;
    let threshold = 3;
    let total_shares = 5;

    let mut rng = create_bench_rng();
    let sharing =
        SecretSharing::split(secret, threshold, total_shares, &mut rng).expect("split failed");
    let shares = sharing.get_shares();

    group.throughput(Throughput::Elements(1));

    group.bench_function("multiply_const_t3_n5", |bench| {
        bench.iter(|| {
            HomomorphicOperations::multiply_by_constant(black_box(shares), black_box(constant))
                .expect("multiply should succeed")
        })
    });

    group.finish();
}

/// Benchmark add_constant operation
fn bench_add_constant(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/add_constant");

    let secret = 123u64;
    let constant = 77u64;
    let threshold = 3;
    let total_shares = 5;

    let mut rng = create_bench_rng();
    let sharing =
        SecretSharing::split(secret, threshold, total_shares, &mut rng).expect("split failed");
    let shares = sharing.get_shares().to_vec();

    group.throughput(Throughput::Elements(1));

    group.bench_function("add_const_t3_n5", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            HomomorphicOperations::add_constant(
                black_box(&shares),
                black_box(constant),
                threshold,
                &mut rng,
            )
            .expect("add_constant should succeed")
        })
    });

    group.finish();
}

/// Benchmark scalability with different share counts
fn bench_scalability_shares(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/scalability_shares");

    let secret_a = 500u64;
    let secret_b = 300u64;
    let threshold = 3;
    let share_counts = [5, 10, 20, 50, 100];

    for &total_shares in &share_counts {
        let mut rng = create_bench_rng();
        let (shares_a, shares_b, _) =
            create_compatible_shares(secret_a, secret_b, threshold, total_shares, &mut rng);

        group.bench_with_input(
            BenchmarkId::new("add", format!("n{}", total_shares)),
            &(shares_a.clone(), shares_b.clone()),
            |bench, (a, b)| {
                bench.iter(|| {
                    HomomorphicOperations::add_shares(black_box(a), black_box(b))
                        .expect("add should succeed")
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("subtract", format!("n{}", total_shares)),
            &(shares_a.clone(), shares_b.clone()),
            |bench, (a, b)| {
                bench.iter(|| {
                    HomomorphicOperations::subtract_shares(black_box(a), black_box(b))
                        .expect("subtract should succeed")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark complex expression: (a + b) - (c * 2) + 100
fn bench_complex_expression(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/complex_expression");

    let secret_a = 500u64;
    let secret_b = 300u64;
    let secret_c = 150u64;
    let threshold = 3;
    let total_shares = 5;

    // Create compatible shares with same x coordinates
    let mut rng = create_bench_rng();
    let x_coords: Vec<FieldElement> = (0..total_shares)
        .map(|_| FieldElementTrait::from_u64(rng.next_u64()))
        .collect();

    let poly_a = Polynomial::new(
        threshold - 1,
        FieldElementTrait::from_u64(secret_a),
        &mut rng,
    );
    let shares_a: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_a.evaluate(&x))).collect();

    let poly_b = Polynomial::new(
        threshold - 1,
        FieldElementTrait::from_u64(secret_b),
        &mut rng,
    );
    let shares_b: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_b.evaluate(&x))).collect();

    let poly_c = Polynomial::new(
        threshold - 1,
        FieldElementTrait::from_u64(secret_c),
        &mut rng,
    );
    let shares_c: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_c.evaluate(&x))).collect();

    group.throughput(Throughput::Elements(1));

    group.bench_function("expr_a_plus_b_minus_2c_plus_100", |bench| {
        let mut rng = create_bench_rng();
        bench.iter(|| {
            // Step 1: a + b
            let ab_sum =
                HomomorphicOperations::add_shares(&shares_a, &shares_b).expect("add failed");

            // Step 2: c * 2
            let c_doubled =
                HomomorphicOperations::multiply_by_constant(&shares_c, 2).expect("mul failed");

            // Step 3: (a + b) - (c * 2)
            let intermediate =
                HomomorphicOperations::subtract_shares(&ab_sum, &c_doubled).expect("sub failed");

            // Step 4: + 100
            HomomorphicOperations::add_constant(&intermediate, 100, threshold, &mut rng)
                .expect("add_const failed")
        })
    });

    group.finish();
}

/// Benchmark Beaver triple multiplication
fn bench_beaver_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/beaver_multiplication");

    let x_secret = 23u64;
    let y_secret = 29u64;
    let threshold = 3;
    let total_shares = 5;

    // Generate consistent x-coordinates
    let x_coords: Vec<FieldElement> = (1..=total_shares)
        .map(|i| FieldElementTrait::from_u64(i as u64))
        .collect();

    let mut rng = create_bench_rng();

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
            HomomorphicOperations::multiply_with_beaver_triple(
                black_box(&x_shares),
                black_box(&y_shares),
                black_box(&triple),
            )
            .expect("multiplication should succeed")
        })
    });

    group.finish();
}

/// Benchmark verification of homomorphic operations
fn bench_verification(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/verification");

    let secret_a = 100u64;
    let secret_b = 50u64;
    let expected_sum = secret_a + secret_b;
    let threshold = 3;
    let total_shares = 5;

    let mut rng = create_bench_rng();
    let (shares_a, shares_b, _) =
        create_compatible_shares(secret_a, secret_b, threshold, total_shares, &mut rng);

    let sum_shares =
        HomomorphicOperations::add_shares(&shares_a, &shares_b).expect("add should succeed");

    group.throughput(Throughput::Elements(1));

    group.bench_function("verify_homomorphic_op", |bench| {
        bench.iter(|| {
            HomomorphicOperations::verify_homomorphic_operation(
                black_box(secret_a),
                black_box(secret_b),
                black_box(&sum_shares),
                black_box(expected_sum),
                threshold,
            )
            .expect("verification should succeed")
        })
    });

    group.finish();
}

/// Benchmark comparison: direct constant multiplication vs Beaver multiplication
fn bench_multiplication_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_ops/multiplication_comparison");

    let x_secret = 7u64;
    let y_constant = 11u64;
    let threshold = 3;
    let total_shares = 5;

    let x_coords: Vec<FieldElement> = (1..=total_shares)
        .map(|i| FieldElementTrait::from_u64(i as u64))
        .collect();

    let mut rng = create_bench_rng();
    let poly_x = Polynomial::new(
        threshold - 1,
        FieldElementTrait::from_u64(x_secret),
        &mut rng,
    );
    let x_shares: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_x.evaluate(&x))).collect();

    // For Beaver multiplication, we need y_shares too
    let poly_y = Polynomial::new(
        threshold - 1,
        FieldElementTrait::from_u64(y_constant),
        &mut rng,
    );
    let y_shares: Vec<(FieldElement, FieldElement)> =
        x_coords.iter().map(|&x| (x, poly_y.evaluate(&x))).collect();

    let triple = BeaverTriple::new_with_coordinates(13, 17, &x_coords, threshold, &mut rng)
        .expect("triple creation should succeed");

    group.throughput(Throughput::Elements(1));

    // Direct constant multiplication (when one operand is public)
    group.bench_function("constant_multiplication", |bench| {
        bench.iter(|| {
            HomomorphicOperations::multiply_by_constant(black_box(&x_shares), black_box(y_constant))
                .expect("multiply should succeed")
        })
    });

    // Beaver multiplication (when both operands are secret)
    group.bench_function("beaver_multiplication", |bench| {
        bench.iter(|| {
            HomomorphicOperations::multiply_with_beaver_triple(
                black_box(&x_shares),
                black_box(&y_shares),
                black_box(&triple),
            )
            .expect("multiply should succeed")
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_add_shares_basic,
    bench_subtract_shares_basic,
    bench_multiply_by_constant,
    bench_add_constant,
    bench_scalability_shares,
    bench_complex_expression,
    bench_beaver_multiplication,
    bench_verification,
    bench_multiplication_comparison,
);

criterion_main!(benches);

//! Polynomial Operations Performance Benchmarks
//!
//! This benchmark suite tests the performance of polynomial operations:
//! - Basic operations: creation, evaluation (Horner's method), Lagrange interpolation
//! - Scalability: different polynomial degrees

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ff::Field;
use mpc_framework_core::field::gf64_sss::FieldElement;
use mpc_framework_core::field::FieldElement as FieldElementTrait;
use mpc_framework_core::poly::Polynomial;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;

/// Create a deterministic RNG for reproducible benchmarks
fn create_bench_rng() -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(12345)
}

/// Benchmark basic polynomial creation
fn bench_polynomial_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/creation");

    let intercept = FieldElementTrait::from_u64(42);

    group.throughput(Throughput::Elements(1));

    // Benchmark creating polynomials of different degrees
    let degrees = [1, 2, 5, 10, 20, 50];

    for &degree in &degrees {
        group.bench_with_input(
            BenchmarkId::new("new", format!("degree_{}", degree)),
            &degree,
            |bench, &d| {
                let mut rng = create_bench_rng();
                bench.iter(|| Polynomial::new(black_box(d), intercept, &mut rng))
            },
        );
    }

    group.finish();
}

/// Benchmark polynomial creation with u64 coefficients
fn bench_polynomial_creation_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/creation_u64");

    let intercept = FieldElementTrait::from_u64(42);

    group.throughput(Throughput::Elements(1));

    let degrees = [1, 2, 5, 10, 20];

    for &degree in &degrees {
        group.bench_with_input(
            BenchmarkId::new("new_with_u64_coeffs", format!("degree_{}", degree)),
            &degree,
            |bench, &d| {
                let mut rng = create_bench_rng();
                bench.iter(|| Polynomial::new_with_u64_coeffs(black_box(d), intercept, &mut rng))
            },
        );
    }

    group.finish();
}

/// Benchmark polynomial evaluation using Horner's method
fn bench_polynomial_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/evaluation");

    let intercept = FieldElementTrait::from_u64(100);
    let degrees = [1, 2, 5, 10, 20, 50, 100];

    for &degree in &degrees {
        let mut rng = create_bench_rng();
        let poly = Polynomial::new(degree, intercept, &mut rng);
        let x = FieldElementTrait::from_u64(rng.next_u64());

        group.bench_with_input(
            BenchmarkId::new("evaluate", format!("degree_{}", degree)),
            &(poly, x),
            |bench, (p, x_val)| bench.iter(|| p.evaluate(black_box(x_val))),
        );
    }

    group.finish();
}

/// Benchmark polynomial evaluation: standard vs Horner optimized
fn bench_evaluation_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/evaluation_comparison");

    let intercept = FieldElementTrait::from_u64(100);
    let degree = 10;

    let mut rng = create_bench_rng();
    let poly = Polynomial::new(degree, intercept, &mut rng);
    let x = FieldElementTrait::from_u64(rng.next_u64());

    group.throughput(Throughput::Elements(1));

    group.bench_function("evaluate_standard", |bench| {
        bench.iter(|| poly.evaluate(black_box(&x)))
    });

    group.bench_function("evaluate_horner_optimized", |bench| {
        bench.iter(|| poly.evaluate_horner_optimized(black_box(&x)))
    });

    group.finish();
}

/// Benchmark Lagrange interpolation
fn bench_lagrange_interpolation(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/interpolation");

    let intercept = FieldElementTrait::from_u64(42);
    let point_counts = [2, 3, 5, 10, 20, 50];

    for &num_points in &point_counts {
        let mut rng = create_bench_rng();
        let degree = num_points - 1;
        let poly = Polynomial::new(degree, intercept, &mut rng);

        // Generate evaluation points
        let points: Vec<(FieldElement, FieldElement)> = (1..=num_points)
            .map(|i| {
                let x = FieldElementTrait::from_u64(i as u64);
                let y = poly.evaluate(&x);
                (x, y)
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("interpolate", format!("{}_points", num_points)),
            &points,
            |bench, pts| {
                bench.iter(|| Polynomial::interpolate(black_box(pts)).expect("interpolate failed"))
            },
        );
    }

    group.finish();
}

/// Benchmark interpolation with random x-coordinates
fn bench_interpolation_random_coords(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/interpolation_random_coords");

    let intercept = FieldElementTrait::from_u64(42);
    let point_counts = [3, 5, 10, 20];

    for &num_points in &point_counts {
        let mut rng = create_bench_rng();
        let degree = num_points - 1;
        let poly = Polynomial::new(degree, intercept, &mut rng);

        // Generate random x-coordinates
        let points: Vec<(FieldElement, FieldElement)> = (0..num_points)
            .map(|_| {
                let x = FieldElementTrait::from_u64(rng.next_u64());
                let y = poly.evaluate(&x);
                (x, y)
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("interpolate_random", format!("{}_points", num_points)),
            &points,
            |bench, pts| {
                bench.iter(|| Polynomial::interpolate(black_box(pts)).expect("interpolate failed"))
            },
        );
    }

    group.finish();
}

/// Benchmark multiple evaluations of the same polynomial
fn bench_batch_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/batch_evaluation");

    let intercept = FieldElementTrait::from_u64(100);
    let degree = 10;

    let mut rng = create_bench_rng();
    let poly = Polynomial::new(degree, intercept, &mut rng);

    // Generate multiple x values
    let x_values: Vec<FieldElement> = (0..100)
        .map(|_| FieldElementTrait::from_u64(rng.next_u64()))
        .collect();

    let batch_sizes = [10, 50, 100];

    for &batch_size in &batch_sizes {
        let batch = &x_values[0..batch_size];

        group.bench_with_input(
            BenchmarkId::new("evaluate_batch", format!("{}_points", batch_size)),
            batch,
            |bench, xs| {
                bench.iter(|| {
                    xs.iter()
                        .map(|x| poly.evaluate(black_box(x)))
                        .collect::<Vec<_>>()
                })
            },
        );
    }

    group.finish();
}

/// Benchmark polynomial degree getter
fn bench_degree(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/degree");

    let intercept = FieldElementTrait::from_u64(42);
    let degrees = [1, 5, 10, 50, 100];

    for &degree in &degrees {
        let mut rng = create_bench_rng();
        let poly = Polynomial::new(degree, intercept, &mut rng);

        group.bench_with_input(
            BenchmarkId::new("degree", format!("degree_{}", degree)),
            &poly,
            |bench, p| bench.iter(|| p.degree()),
        );
    }

    group.finish();
}

/// Benchmark coefficients access
fn bench_coefficients_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/coefficients");

    let intercept = FieldElementTrait::from_u64(42);
    let degrees = [5, 10, 50, 100];

    for &degree in &degrees {
        let mut rng = create_bench_rng();
        let poly = Polynomial::new(degree, intercept, &mut rng);

        group.bench_with_input(
            BenchmarkId::new("get_coefficients", format!("degree_{}", degree)),
            &poly,
            |bench, p| bench.iter(|| p.coefficients()),
        );
    }

    group.finish();
}

/// Benchmark end-to-end: create polynomial, evaluate, then interpolate
fn bench_end_to_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/end_to_end");

    let intercept = FieldElementTrait::from_u64(12345);
    let configs = [(2, 3), (3, 5), (5, 10), (10, 20)];

    for &(degree, num_points) in &configs {
        group.bench_with_input(
            BenchmarkId::new(
                "create_evaluate_interpolate",
                format!("d{}_n{}", degree, num_points),
            ),
            &(degree, num_points),
            |bench, &(d, n)| {
                bench.iter(|| {
                    let mut rng = create_bench_rng();

                    // Create polynomial
                    let poly = Polynomial::new(d, intercept, &mut rng);

                    // Evaluate at n points
                    let points: Vec<(FieldElement, FieldElement)> = (1..=n)
                        .map(|i| {
                            let x = FieldElementTrait::from_u64(i as u64);
                            let y = poly.evaluate(&x);
                            (x, y)
                        })
                        .collect();

                    // Interpolate to recover f(0)
                    Polynomial::interpolate(black_box(&points)).expect("interpolate failed")
                })
            },
        );
    }

    group.finish();
}

/// Benchmark polynomial with zero intercept
fn bench_zero_intercept(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/zero_intercept");

    let zero_intercept = FieldElement::ZERO;
    let degree = 5;

    let mut rng = create_bench_rng();
    let poly = Polynomial::new(degree, zero_intercept, &mut rng);

    let x = FieldElementTrait::from_u64(42);

    group.throughput(Throughput::Elements(1));

    group.bench_function("evaluate_zero_intercept", |bench| {
        bench.iter(|| poly.evaluate(black_box(&x)))
    });

    group.finish();
}

/// Benchmark constant polynomial (degree 0)
fn bench_constant_polynomial(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial/constant");

    let intercept = FieldElementTrait::from_u64(42);

    let mut rng = create_bench_rng();
    let poly = Polynomial::new(0, intercept, &mut rng);

    let x = FieldElementTrait::from_u64(12345);

    group.throughput(Throughput::Elements(1));

    group.bench_function("evaluate_constant", |bench| {
        bench.iter(|| poly.evaluate(black_box(&x)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_polynomial_creation,
    bench_polynomial_creation_u64,
    bench_polynomial_evaluation,
    bench_evaluation_comparison,
    bench_lagrange_interpolation,
    bench_interpolation_random_coords,
    bench_batch_evaluation,
    bench_degree,
    bench_coefficients_access,
    bench_end_to_end,
    bench_zero_intercept,
    bench_constant_polynomial,
);

criterion_main!(benches);


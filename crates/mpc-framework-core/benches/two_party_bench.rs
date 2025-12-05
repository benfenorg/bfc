//! Two-Party Secret Sharing Performance Benchmarks
//!
//! This benchmark suite tests the performance of two-party (2,2) secret sharing
//! with homomorphic operations:
//! - Basic operations: split_to_two_value, recover_value
//! - Homomorphic operations: add_two_shared_secrets, sub_two_shared_secrets
//! - Beaver triple multiplication: mul_step1/2/3

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mpc_framework_core::two_party_share::{
    add_two_shared_secrets, recover_value, split_to_two_value,
    sub_two_shared_secrets, recover_two_shares,
    mul_step1_compute_masked_diff, mul_step2_reconstruct_masked_values, mul_step3_compute_result,
    generate_beaver_triple, bytes_to_share, recover_from_shares_internal,
};
use mpc_framework_core::poly::Polynomial;
use mpc_framework_core::field::FieldElement as FieldElementTrait;
use mpc_framework_core::field::gf64_sss::FieldElement;

// Constants for benchmarking
const BENCH_MASK_SECRET: u64 = 0x1234567890ABCDEFu64;
const BENCH_USER_ID: u64 = 1u64;
const BENCH_COORD_SEED: u64 = 0xABCDEF1234567890u64;
const THRESHOLD: usize = 2;

/// Benchmark basic split_to_two_value operation
fn bench_split_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/split_basic");

    let secret = 42u64;

    group.throughput(Throughput::Elements(1));

    group.bench_function("split_to_two_value", |bench| {
        bench.iter(|| {
            split_to_two_value(black_box(secret), BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED)
        })
    });

    group.finish();
}

/// Benchmark basic recover_value operation
fn bench_recover_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/recover_basic");

    let secret = 42u64;

    // Pre-generate shares
    let (hex1, hex2, _) = split_to_two_value(secret, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);

    group.throughput(Throughput::Elements(1));

    group.bench_function("recover_value", |bench| {
        bench.iter(|| {
            recover_value(
                black_box(hex1.clone()),
                black_box(hex2.clone()),
                BENCH_MASK_SECRET,
            )
            .expect("recover should succeed")
        })
    });

    group.finish();
}

/// Benchmark recover_two_shares operation
fn bench_recover_two_shares(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/recover_two_shares");

    let secret = 123456u64;

    // Pre-generate shares
    let (hex1, hex2, _) = split_to_two_value(secret, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);

    group.throughput(Throughput::Elements(1));

    group.bench_function("recover_two_shares", |bench| {
        bench.iter(|| {
            recover_two_shares(
                black_box(hex1.clone()),
                black_box(hex2.clone()),
                BENCH_MASK_SECRET,
            )
            .expect("recover should succeed")
        })
    });

    group.finish();
}

/// Benchmark split and recover round-trip
fn bench_split_recover_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/roundtrip");

    let secret = 999888777u64;

    group.throughput(Throughput::Elements(1));

    group.bench_function("split_and_recover", |bench| {
        bench.iter(|| {
            let (hex1, hex2, _) =
                split_to_two_value(black_box(secret), BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
            recover_value(hex1, hex2, BENCH_MASK_SECRET).expect("recover should succeed")
        })
    });

    group.finish();
}

/// Benchmark homomorphic add_two_shared_secrets
fn bench_add_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/homomorphic_add");

    let secret1 = 100u64;
    let secret2 = 50u64;

    // Pre-split secrets with same coord_seed
    let (hex1_a, hex2_a, seed) = split_to_two_value(secret1, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let (hex1_b, hex2_b, _) = split_to_two_value(secret2, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);

    group.throughput(Throughput::Elements(1));

    group.bench_function("add_two_shared_secrets", |bench| {
        bench.iter(|| {
            // Add on both share positions
            let _r1 = add_two_shared_secrets(
                black_box(hex1_a.clone()),
                black_box(hex1_b.clone()),
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            )
            .expect("add should succeed");
            let _r2 = add_two_shared_secrets(
                black_box(hex2_a.clone()),
                black_box(hex2_b.clone()),
                BENCH_MASK_SECRET,
                1,
                seed,
                seed,
            )
            .expect("add should succeed");
        })
    });

    group.finish();
}

/// Benchmark homomorphic sub_two_shared_secrets
fn bench_sub_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/homomorphic_subtract");

    let secret1 = 100u64;
    let secret2 = 30u64;

    // Pre-split secrets with same coord_seed
    let (hex1_a, hex2_a, seed) = split_to_two_value(secret1, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let (hex1_b, hex2_b, _) = split_to_two_value(secret2, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);

    group.throughput(Throughput::Elements(1));

    group.bench_function("sub_two_shared_secrets", |bench| {
        bench.iter(|| {
            let _r1 = sub_two_shared_secrets(
                black_box(hex1_a.clone()),
                black_box(hex1_b.clone()),
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            )
            .expect("sub should succeed");
            let _r2 = sub_two_shared_secrets(
                black_box(hex2_a.clone()),
                black_box(hex2_b.clone()),
                BENCH_MASK_SECRET,
                1,
                seed,
                seed,
            )
            .expect("sub should succeed");
        })
    });

    group.finish();
}

/// Benchmark Beaver triple generation
fn bench_beaver_triple_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/beaver_triple");

    group.throughput(Throughput::Elements(1));

    group.bench_function("generate_beaver_triple", |bench| {
        bench.iter(|| {
            generate_beaver_triple(black_box(BENCH_MASK_SECRET))
                .expect("beaver triple generation should succeed")
        })
    });

    group.finish();
}

/// Benchmark Beaver triple multiplication steps
fn bench_beaver_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/beaver_multiplication");

    let x = 7u64;
    let y = 11u64;

    // Pre-generate Beaver triple
    let triple = generate_beaver_triple(BENCH_MASK_SECRET).unwrap();

    // Create x and y shares using same coordinates as Beaver triple
    let x_field: FieldElement = FieldElementTrait::from_u64(x);
    let y_field: FieldElement = FieldElementTrait::from_u64(y);
    let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
    let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);

    let x_share_0 = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
    let x_share_1 = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));
    let y_share_0 = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
    let y_share_1 = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));

    // Pre-compute step 1 results for step 2/3 benchmarks
    let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
    let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
    let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
    let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

    let (d_open, e_open) = mul_step2_reconstruct_masked_values(
        &[d_share_0, d_share_1],
        &[e_share_0, e_share_1],
    ).unwrap();

    group.throughput(Throughput::Elements(1));

    // Benchmark step 1
    group.bench_function("mul_step1_compute_masked_diff", |bench| {
        bench.iter(|| {
            mul_step1_compute_masked_diff(black_box(&x_share_0), black_box(&triple.a_shares[0]))
        })
    });

    // Benchmark step 2
    group.bench_function("mul_step2_reconstruct_masked_values", |bench| {
        bench.iter(|| {
            mul_step2_reconstruct_masked_values(
                black_box(&[d_share_0, d_share_1]),
                black_box(&[e_share_0, e_share_1]),
            )
            .expect("reconstruct should succeed")
        })
    });

    // Benchmark step 3
    group.bench_function("mul_step3_compute_result", |bench| {
        bench.iter(|| {
            mul_step3_compute_result(
                black_box(&triple.a_shares[0]),
                black_box(&triple.b_shares[0]),
                black_box(&triple.c_shares[0]),
                black_box(d_open),
                black_box(e_open),
            )
        })
    });

    // Benchmark full multiplication
    group.bench_function("full_beaver_multiplication", |bench| {
        bench.iter(|| {
            // Step 1: Compute masked differences
            let d0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
            let d1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
            let e0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
            let e1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

            // Step 2: Reconstruct d and e
            let (d, e) = mul_step2_reconstruct_masked_values(
                &[d0, d1], &[e0, e1],
            ).unwrap();

            // Step 3: Compute result
            let r0 = mul_step3_compute_result(
                &triple.a_shares[0], &triple.b_shares[0], &triple.c_shares[0], d, e,
            );
            let r1 = mul_step3_compute_result(
                &triple.a_shares[1], &triple.b_shares[1], &triple.c_shares[1], d, e,
            );

            // Recover final result
            let s0 = bytes_to_share(&r0).unwrap();
            let s1 = bytes_to_share(&r1).unwrap();
            recover_from_shares_internal(&[s0, s1]).unwrap()
        })
    });

    group.finish();
}

/// Benchmark with different secret values
fn bench_different_secrets(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/different_secrets");

    let secrets = [0u64, 1, 100, 999999, 4294967293];

    for &secret in &secrets {
        group.bench_with_input(
            BenchmarkId::new("split", format!("secret_{}", secret)),
            &secret,
            |bench, &s| {
                bench.iter(|| {
                    split_to_two_value(black_box(s), BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark comparison of homomorphic add vs subtract
fn bench_homomorphic_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/homomorphic_comparison");

    let secret1 = 100u64;
    let secret2 = 50u64;

    // Pre-split secrets with same coord_seed
    let (hex1_a, hex2_a, seed) = split_to_two_value(secret1, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let (hex1_b, hex2_b, _) = split_to_two_value(secret2, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let _ = hex2_a;
    let _ = hex2_b;

    group.throughput(Throughput::Elements(1));

    group.bench_function("add", |bench| {
        bench.iter(|| {
            add_two_shared_secrets(
                black_box(hex1_a.clone()),
                black_box(hex1_b.clone()),
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            )
        })
    });

    group.bench_function("subtract", |bench| {
        bench.iter(|| {
            sub_two_shared_secrets(
                black_box(hex1_a.clone()),
                black_box(hex1_b.clone()),
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            )
        })
    });

    group.finish();
}

/// Benchmark with zero values
fn bench_zero_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/zero_operations");

    // Pre-split values with same coord_seed
    let (hex1_42, hex2_42, seed) = split_to_two_value(42u64, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let (hex1_0, hex2_0, _) = split_to_two_value(0u64, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let _ = hex2_42;
    let _ = hex2_0;

    group.throughput(Throughput::Elements(1));

    group.bench_function("add_with_zero", |bench| {
        bench.iter(|| {
            add_two_shared_secrets(
                black_box(hex1_42.clone()),
                black_box(hex1_0.clone()),
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            )
        })
    });

    group.bench_function("sub_to_zero", |bench| {
        bench.iter(|| {
            sub_two_shared_secrets(
                black_box(hex1_42.clone()),
                black_box(hex1_42.clone()),
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            )
        })
    });

    group.finish();
}

/// Benchmark complex expression using homomorphic operations
fn bench_complex_expression(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/complex_expression");

    // Expression: (a + b) - c
    let a = 100u64;
    let b = 50u64;
    let cv = 30u64;

    // Pre-split all values with same coord_seed
    let (hex1_a, hex2_a, seed) = split_to_two_value(a, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let (hex1_b, hex2_b, _) = split_to_two_value(b, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
    let (hex1_c, hex2_c, _) = split_to_two_value(cv, BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);

    group.throughput(Throughput::Elements(1));

    group.bench_function("expr_a_plus_b_minus_c", |bench| {
        bench.iter(|| {
            // Step 1: a + b (on both positions)
            let ab_bytes1 = add_two_shared_secrets(
                black_box(hex1_a.clone()),
                black_box(hex1_b.clone()),
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            ).expect("add should succeed");
            let ab_bytes2 = add_two_shared_secrets(
                black_box(hex2_a.clone()),
                black_box(hex2_b.clone()),
                BENCH_MASK_SECRET,
                1,
                seed,
                seed,
            ).expect("add should succeed");

            // Convert to shares
            let share1_ab = bytes_to_share(&ab_bytes1).unwrap();
            let share2_ab = bytes_to_share(&ab_bytes2).unwrap();

            // Get c shares
            let shares_c = recover_two_shares(hex1_c.clone(), hex2_c.clone(), BENCH_MASK_SECRET).unwrap();

            // Step 2: (a + b) - c (manually subtract on shares)
            let result_share1 = (share1_ab.0, share1_ab.1 - shares_c[0].1);
            let result_share2 = (share2_ab.0, share2_ab.1 - shares_c[1].1);

            recover_from_shares_internal(&[result_share1, result_share2]).unwrap()
        })
    });

    group.finish();
}

/// Benchmark large secret values
fn bench_large_values(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/large_values");

    // Values within the field modulus
    let large_values = [u32::MAX as u64, 18446744069414584320u64]; // MODULUS - 1

    for &value in &large_values {
        group.bench_with_input(
            BenchmarkId::new("split_large", format!("val_{}", value)),
            &value,
            |bench, &v| {
                bench.iter(|| {
                    split_to_two_value(black_box(v), BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark end-to-end workflow (split, homomorphic add, recover)
fn bench_end_to_end_workflow(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/end_to_end");

    let secret1 = 123u64;
    let secret2 = 456u64;

    group.throughput(Throughput::Elements(1));

    group.bench_function("full_homomorphic_add_workflow", |bench| {
        bench.iter(|| {
            // Split both secrets with same coord_seed
            let (hex1_a, hex2_a, seed) =
                split_to_two_value(black_box(secret1), BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);
            let (hex1_b, hex2_b, _) =
                split_to_two_value(black_box(secret2), BENCH_USER_ID, BENCH_MASK_SECRET, BENCH_COORD_SEED);

            // Homomorphic addition on both share positions
            let result_bytes1 = add_two_shared_secrets(
                hex1_a,
                hex1_b,
                BENCH_MASK_SECRET,
                0,
                seed,
                seed,
            )
            .expect("add should succeed");
            let result_bytes2 = add_two_shared_secrets(
                hex2_a,
                hex2_b,
                BENCH_MASK_SECRET,
                1,
                seed,
                seed,
            )
            .expect("add should succeed");

            // Convert bytes to shares and recover
            let share1 = bytes_to_share(&result_bytes1).unwrap();
            let share2 = bytes_to_share(&result_bytes2).unwrap();
            let result = recover_from_shares_internal(&[share1, share2]).unwrap();

            assert_eq!(result, secret1 + secret2);
            result
        })
    });

    group.finish();
}

/// Benchmark different user_ids
fn bench_different_user_ids(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/user_ids");

    let secret = 12345u64;
    let user_ids = [1u64, 100, u64::MAX];

    for &user_id in &user_ids {
        group.bench_with_input(
            BenchmarkId::new("split_user_id", format!("uid_{}", user_id)),
            &user_id,
            |bench, &uid| {
                bench.iter(|| {
                    split_to_two_value(black_box(secret), uid, BENCH_MASK_SECRET, BENCH_COORD_SEED)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark different mask_secrets
fn bench_different_masks(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/mask_secrets");

    let secret = 12345u64;
    let masks = [0u64, 0x1234567890ABCDEFu64, u64::MAX];

    for &mask in &masks {
        group.bench_with_input(
            BenchmarkId::new("split_mask", format!("mask_{:x}", mask)),
            &mask,
            |bench, &m| {
                bench.iter(|| split_to_two_value(black_box(secret), BENCH_USER_ID, m, BENCH_COORD_SEED))
            },
        );
    }

    group.finish();
}

/// Benchmark different coord_seeds
fn bench_different_coord_seeds(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_party/coord_seeds");

    let secret = 12345u64;
    let seeds = [0u64, 0xABCDEF1234567890u64, u64::MAX];

    for &seed in &seeds {
        group.bench_with_input(
            BenchmarkId::new("split_coord_seed", format!("seed_{:x}", seed)),
            &seed,
            |bench, &s| {
                bench.iter(|| split_to_two_value(black_box(secret), BENCH_USER_ID, BENCH_MASK_SECRET, s))
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_split_basic,
    bench_recover_basic,
    bench_recover_two_shares,
    bench_split_recover_roundtrip,
    bench_add_operation,
    bench_sub_operation,
    bench_beaver_triple_generation,
    bench_beaver_multiplication,
    bench_different_secrets,
    bench_homomorphic_comparison,
    bench_zero_operations,
    bench_complex_expression,
    bench_large_values,
    bench_end_to_end_workflow,
    bench_different_user_ids,
    bench_different_masks,
    bench_different_coord_seeds,
);

criterion_main!(benches);

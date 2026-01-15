//! XOR Encryption Performance Benchmarks
//!
//! This benchmark suite tests the performance of XOR encryption operations:
//! - Basic operations: single share encryption/decryption
//! - Batch operations: batch encryption/decryption
//! - End-to-end: encryption-aware homomorphic operations

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mpc_framework_core::encryption::{EncryptedHomomorphicOperations, XorEncryptionManager};
use mpc_framework_core::field::gf64_sss::FieldElement;
use mpc_framework_core::field::FieldElement as FieldElementTrait;
use mpc_framework_core::poly::Polynomial;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;
use std::env;
use std::fs;

/// Create a deterministic RNG for reproducible benchmarks
fn create_bench_rng() -> ChaCha20Rng {
    ChaCha20Rng::seed_from_u64(12345)
}

/// Create a temporary encryption config file for benchmarks
fn create_bench_config() -> (std::path::PathBuf, XorEncryptionManager) {
    let config_content = r#"{
  "bench_x_mask": {
    "key": "BENCH_X_MASK",
    "mask": "0x1234567890ABCDEF"
  },
  "bench_y_mask": {
    "key": "BENCH_Y_MASK",
    "mask": "0xFEDCBA0987654321"
  }
}"#;

    let temp_dir = env::temp_dir();
    let config_path = temp_dir.join("bench_encryption_config.json");
    fs::write(&config_path, config_content).expect("Failed to write config");

    let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
        .expect("Failed to load config");

    (config_path, manager)
}

/// Generate test shares for encryption benchmarks
fn generate_test_shares(
    count: usize,
    threshold: usize,
    rng: &mut ChaCha20Rng,
) -> Vec<(FieldElement, FieldElement)> {
    let secret = 12345u64;
    let x_coords: Vec<FieldElement> = (0..count)
        .map(|_| FieldElementTrait::from_u64(rng.next_u64()))
        .collect();

    let poly = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(secret), rng);

    x_coords.iter().map(|&x| (x, poly.evaluate(&x))).collect()
}

/// Benchmark single share encryption
fn bench_encrypt_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/single_encrypt");

    let (_config_path, manager) = create_bench_config();

    let share = (
        FieldElementTrait::from_u64(12345),
        FieldElementTrait::from_u64(67890),
    );

    group.throughput(Throughput::Elements(1));

    group.bench_function("encrypt_share", |bench| {
        bench.iter(|| {
            manager
                .encrypt_share(black_box(&share), "BENCH_X_MASK", "BENCH_Y_MASK")
                .expect("encryption should succeed")
        })
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark single share decryption
fn bench_decrypt_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/single_decrypt");

    let (_config_path, manager) = create_bench_config();

    let share = (
        FieldElementTrait::from_u64(12345),
        FieldElementTrait::from_u64(67890),
    );

    let encrypted_share = manager
        .encrypt_share(&share, "BENCH_X_MASK", "BENCH_Y_MASK")
        .expect("encryption should succeed");

    group.throughput(Throughput::Elements(1));

    group.bench_function("decrypt_share", |bench| {
        bench.iter(|| {
            manager
                .decrypt_share(black_box(&encrypted_share))
                .expect("decryption should succeed")
        })
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark batch encryption with different sizes
fn bench_batch_encrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/batch_encrypt");

    let (_config_path, manager) = create_bench_config();

    let threshold = 3;
    let batch_sizes = [5, 10, 20, 50, 100];

    for &size in &batch_sizes {
        let mut rng = create_bench_rng();
        let shares = generate_test_shares(size, threshold, &mut rng);

        group.bench_with_input(
            BenchmarkId::new("encrypt_shares", format!("n{}", size)),
            &shares,
            |bench, s| {
                bench.iter(|| {
                    manager
                        .encrypt_shares(black_box(s), "BENCH_X_MASK", "BENCH_Y_MASK")
                        .expect("encryption should succeed")
                })
            },
        );
    }

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark batch decryption with different sizes
fn bench_batch_decrypt(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/batch_decrypt");

    let (_config_path, manager) = create_bench_config();

    let threshold = 3;
    let batch_sizes = [5, 10, 20, 50, 100];

    for &size in &batch_sizes {
        let mut rng = create_bench_rng();
        let shares = generate_test_shares(size, threshold, &mut rng);
        let encrypted_shares = manager
            .encrypt_shares(&shares, "BENCH_X_MASK", "BENCH_Y_MASK")
            .expect("encryption should succeed");

        group.bench_with_input(
            BenchmarkId::new("decrypt_shares", format!("n{}", size)),
            &encrypted_shares,
            |bench, es| {
                bench.iter(|| {
                    manager
                        .decrypt_shares(black_box(es))
                        .expect("decryption should succeed")
                })
            },
        );
    }

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark encryption round-trip (encrypt then decrypt)
fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/roundtrip");

    let (_config_path, manager) = create_bench_config();

    let share = (
        FieldElementTrait::from_u64(12345),
        FieldElementTrait::from_u64(67890),
    );

    group.throughput(Throughput::Elements(1));

    group.bench_function("encrypt_decrypt", |bench| {
        bench.iter(|| {
            let encrypted = manager
                .encrypt_share(black_box(&share), "BENCH_X_MASK", "BENCH_Y_MASK")
                .expect("encryption should succeed");
            manager
                .decrypt_share(black_box(&encrypted))
                .expect("decryption should succeed")
        })
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark encrypted homomorphic addition
fn bench_encrypted_homomorphic_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/homomorphic_add");

    let (_config_path, manager) = create_bench_config();

    let threshold = 3;
    let total_shares = 5;
    let secret_a = 100u64;
    let secret_b = 200u64;

    let mut rng = create_bench_rng();

    // Create compatible shares with same x coordinates
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

    // Encrypt shares
    let encrypted_a = manager
        .encrypt_shares(&shares_a, "BENCH_X_MASK", "BENCH_Y_MASK")
        .expect("encryption should succeed");
    let encrypted_b = manager
        .encrypt_shares(&shares_b, "BENCH_X_MASK", "BENCH_Y_MASK")
        .expect("encryption should succeed");

    group.throughput(Throughput::Elements(1));

    group.bench_function("add_encrypted_shares", |bench| {
        bench.iter(|| {
            EncryptedHomomorphicOperations::add_encrypted_shares(
                black_box(&manager),
                black_box(&encrypted_a),
                black_box(&encrypted_b),
            )
            .expect("addition should succeed")
        })
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark encrypted homomorphic subtraction
fn bench_encrypted_homomorphic_subtract(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/homomorphic_subtract");

    let (_config_path, manager) = create_bench_config();

    let threshold = 3;
    let total_shares = 5;
    let secret_a = 1000u64;
    let secret_b = 300u64;

    let mut rng = create_bench_rng();

    // Create compatible shares with same x coordinates
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

    // Encrypt shares
    let encrypted_a = manager
        .encrypt_shares(&shares_a, "BENCH_X_MASK", "BENCH_Y_MASK")
        .expect("encryption should succeed");
    let encrypted_b = manager
        .encrypt_shares(&shares_b, "BENCH_X_MASK", "BENCH_Y_MASK")
        .expect("encryption should succeed");

    group.throughput(Throughput::Elements(1));

    group.bench_function("subtract_encrypted_shares", |bench| {
        bench.iter(|| {
            EncryptedHomomorphicOperations::subtract_encrypted_shares(
                black_box(&manager),
                black_box(&encrypted_a),
                black_box(&encrypted_b),
            )
            .expect("subtraction should succeed")
        })
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark config loading
fn bench_config_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/config_loading");

    let config_content = r#"{
  "mask1": {"key": "MASK1", "mask": "0x1234567890ABCDEF"},
  "mask2": {"key": "MASK2", "mask": "0xFEDCBA0987654321"},
  "mask3": {"key": "MASK3", "mask": "0xAAAABBBBCCCCDDDD"},
  "mask4": {"key": "MASK4", "mask": "0x1111222233334444"}
}"#;

    let temp_dir = env::temp_dir();
    let config_path = temp_dir.join("bench_config_loading.json");
    fs::write(&config_path, config_content).expect("Failed to write config");

    group.throughput(Throughput::Elements(1));

    group.bench_function("load_from_file", |bench| {
        bench.iter(|| {
            XorEncryptionManager::load_from_file(black_box(config_path.to_str().unwrap()))
                .expect("load should succeed")
        })
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(config_path);
}

/// Benchmark get_available_mask_keys
fn bench_get_mask_keys(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/get_mask_keys");

    let (_config_path, manager) = create_bench_config();

    group.throughput(Throughput::Elements(1));

    group.bench_function("get_available_mask_keys", |bench| {
        bench.iter(|| manager.get_available_mask_keys())
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

/// Benchmark end-to-end encrypted workflow
fn bench_end_to_end_encrypted(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption/end_to_end");

    let (_config_path, manager) = create_bench_config();

    let threshold = 3;
    let total_shares = 5;
    let secret_a = 500u64;
    let secret_b = 300u64;

    let mut rng = create_bench_rng();

    // Create compatible shares
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

    group.throughput(Throughput::Elements(1));

    group.bench_function("full_workflow_add", |bench| {
        bench.iter(|| {
            // Encrypt shares
            let encrypted_a = manager
                .encrypt_shares(black_box(&shares_a), "BENCH_X_MASK", "BENCH_Y_MASK")
                .expect("encryption should succeed");
            let encrypted_b = manager
                .encrypt_shares(black_box(&shares_b), "BENCH_X_MASK", "BENCH_Y_MASK")
                .expect("encryption should succeed");

            // Perform encrypted homomorphic operation
            let result = EncryptedHomomorphicOperations::add_encrypted_shares(
                &manager,
                &encrypted_a,
                &encrypted_b,
            )
            .expect("addition should succeed");

            // Recover result
            mpc_framework_core::SecretSharing::recover(&result[0..threshold])
                .expect("recovery should succeed")
        })
    });

    group.finish();

    // Cleanup
    let _ = fs::remove_file(_config_path);
}

criterion_group!(
    benches,
    bench_encrypt_single,
    bench_decrypt_single,
    bench_batch_encrypt,
    bench_batch_decrypt,
    bench_roundtrip,
    bench_encrypted_homomorphic_add,
    bench_encrypted_homomorphic_subtract,
    bench_config_loading,
    bench_get_mask_keys,
    bench_end_to_end_encrypted,
);

criterion_main!(benches);

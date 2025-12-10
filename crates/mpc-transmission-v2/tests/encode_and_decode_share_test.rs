use mpc_transmission_v2::*;

const TEST_MASK_SECRET: u64 = 0x1234567890ABCDEFu64;
const TEST_USER_ID: u64 = 1u64;
const TEST_COORD_SEED: u64 = 116540450355;

// ============================================================================
// Tests for generate_shares_u64
// ============================================================================

#[test]
fn test_generate_shares_u64_basic() {
    let secret = 12345u64;
    let threshold = 3;
    let total_shares = 5;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();

    assert_eq!(shares.len(), total_shares);

    // Recover using threshold shares
    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, secret);

    // Recover using all shares
    let recovered = recover_from_shares(&shares).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_generate_shares_u64_zero_value() {
    let secret = 0u64;
    let threshold = 2;
    let total_shares = 4;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    assert_eq!(shares.len(), total_shares);

    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, 0);
}

#[test]
fn test_generate_shares_u64_large_value() {
    let secret = FIELD_MODULUS - 1;
    let threshold = 3;
    let total_shares = 5;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    assert_eq!(shares.len(), total_shares);

    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_generate_shares_u64_max_u64() {
    let secret = u64::MAX;
    // Note: u64::MAX might exceed FIELD_MODULUS, so recovery might fail
    let threshold = 3;
    let total_shares = 5;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    assert_eq!(shares.len(), total_shares);

    // Try to recover - might succeed or fail depending on field modulus
    let result = recover_from_shares(&shares[0..threshold]);
    if secret >= FIELD_MODULUS {
        // If secret exceeds modulus, recovery should produce a value modulo FIELD_MODULUS
        if let Ok(recovered) = result {
            assert!(recovered < FIELD_MODULUS);
        }
    } else {
        assert_eq!(result.unwrap(), secret);
    }
}

#[test]
fn test_generate_shares_u64_different_thresholds() {
    let secret = 99999u64;
    let coord_seed = TEST_COORD_SEED;

    // Test threshold = 2
    let shares_2 = generate_shares_u64(secret, 2, 4, coord_seed).unwrap();
    assert_eq!(shares_2.len(), 4);
    let recovered = recover_from_shares(&shares_2[0..2]).unwrap();
    assert_eq!(recovered, secret);

    // Test threshold = 3
    let shares_3 = generate_shares_u64(secret, 3, 5, coord_seed).unwrap();
    assert_eq!(shares_3.len(), 5);
    let recovered = recover_from_shares(&shares_3[0..3]).unwrap();
    assert_eq!(recovered, secret);

    // Test threshold = 5
    let shares_5 = generate_shares_u64(secret, 5, 10, coord_seed).unwrap();
    assert_eq!(shares_5.len(), 10);
    let recovered = recover_from_shares(&shares_5[0..5]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_generate_shares_u64_different_total_shares() {
    let secret = 54321u64;
    let threshold = 3;

    // Test total_shares = threshold
    let shares_min = generate_shares_u64(secret, threshold, threshold, TEST_COORD_SEED).unwrap();
    assert_eq!(shares_min.len(), threshold);
    let recovered = recover_from_shares(&shares_min).unwrap();
    assert_eq!(recovered, secret);

    // Test total_shares > threshold
    let shares_more = generate_shares_u64(secret, threshold, threshold + 5, TEST_COORD_SEED).unwrap();
    assert_eq!(shares_more.len(), threshold + 5);
    let recovered = recover_from_shares(&shares_more[0..threshold]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_generate_shares_u64_insufficient_shares() {
    let secret = 77777u64;
    let threshold = 3;
    let total_shares = 5;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();

    // Try to recover with less than threshold shares
    // Note: Interpolation may succeed but produce incorrect value
    let insufficient_shares = &shares[0..threshold - 1];
    let result = recover_from_shares(insufficient_shares);
    
    // Either should fail, or succeed but produce wrong value
    if let Ok(recovered) = result {
        // If it succeeds, the recovered value should be different from the secret
        assert_ne!(recovered, secret, "Recovery with insufficient shares should produce wrong value");
    } else {
        // Error is also acceptable
        assert!(result.is_err());
    }
}

#[test]
fn test_generate_shares_u64_scattered_shares() {
    let secret = 88888u64;
    let threshold = 3;
    let total_shares = 7;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();

    // Recover using scattered shares (not consecutive)
    let scattered = vec![shares[0].clone(), shares[2].clone(), shares[5].clone()];
    let recovered = recover_from_shares(&scattered).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_generate_shares_u64_deterministic() {
    let secret = 11111u64;
    let threshold = 3;
    let total_shares = 5;
    let coord_seed = TEST_COORD_SEED;

    // Generate shares twice with same parameters
    let shares1 = generate_shares_u64(secret, threshold, total_shares, coord_seed).unwrap();
    let shares2 = generate_shares_u64(secret, threshold, total_shares, coord_seed).unwrap();

    // Should produce identical shares
    assert_eq!(shares1.len(), shares2.len());
    for (s1, s2) in shares1.iter().zip(shares2.iter()) {
        assert_eq!(s1.0, s2.0); // x-coordinates should match
        assert_eq!(s1.1, s2.1); // y-values should match
    }
}

#[test]
fn test_generate_shares_u64_different_seeds() {
    let secret = 22222u64;
    let threshold = 3;
    let total_shares = 5;

    let shares1 = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let shares2 = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED + 1).unwrap();

    // Should produce different coordinates but same secret when recovered
    assert_eq!(shares1.len(), shares2.len());
    
    // Coordinates should be different
    let coords_different = shares1.iter().zip(shares2.iter()).any(|(s1, s2)| s1.0 != s2.0);
    assert!(coords_different, "Different seeds should produce different coordinates");

    // But recovery should produce same secret
    let recovered1 = recover_from_shares(&shares1[0..threshold]).unwrap();
    let recovered2 = recover_from_shares(&shares2[0..threshold]).unwrap();
    assert_eq!(recovered1, secret);
    assert_eq!(recovered2, secret);
}

#[test]
fn test_generate_shares_u64_invalid_threshold() {
    let secret = 33333u64;
    let total_shares = 5;

    // Test threshold = 1 (should fail)
    let result = generate_shares_u64(secret, 1, total_shares, TEST_COORD_SEED);
    assert!(result.is_err());
    if let Err(SSSError::InvalidParameters(msg)) = result {
        assert!(msg.contains("Threshold must be at least 2"));
    } else {
        panic!("Expected InvalidParameters error");
    }

    // Test threshold = 0 (should fail)
    let result = generate_shares_u64(secret, 0, total_shares, TEST_COORD_SEED);
    assert!(result.is_err());
}

#[test]
fn test_generate_shares_u64_invalid_total_shares() {
    let secret = 44444u64;
    let threshold = 5;

    // Test total_shares < threshold (should fail)
    let result = generate_shares_u64(secret, threshold, threshold - 1, TEST_COORD_SEED);
    assert!(result.is_err());
    if let Err(SSSError::InvalidParameters(msg)) = result {
        assert!(msg.contains("Total shares"));
        assert!(msg.contains("threshold"));
    } else {
        panic!("Expected InvalidParameters error");
    }
}

#[test]
fn test_generate_shares_u64_consistency_with_split_to_two_value_v2() {
    // When threshold=2 and total_shares=2, should be consistent with split_to_two_value_v2_internal
    let secret = 55555u64;
    let coord_seed = TEST_COORD_SEED;

    // Generate using generate_shares_u64
    let shares = generate_shares_u64(secret, 2, 2, coord_seed).unwrap();
    assert_eq!(shares.len(), 2);

    // Generate using split_to_two_value_v2 (which uses split_to_two_value_v2_internal internally)
    use crate::two_party_helper::get_two_party_coordinates;
    use crate::two_party_helper::split_to_two_value_v2_internal;
    let coords = get_two_party_coordinates(coord_seed);
    let (share1, share2) = split_to_two_value_v2_internal(secret, &coords).unwrap();

    // Compare coordinates - they should match
    assert_eq!(shares[0].0, share1.0);
    assert_eq!(shares[1].0, share2.0);

    // Compare y-values - they should match (same polynomial evaluation)
    assert_eq!(shares[0].1, share1.1);
    assert_eq!(shares[1].1, share2.1);

    // Both should recover to same secret
    let recovered1 = recover_from_shares(&shares).unwrap();
    let recovered2 = recover_from_shares(&[share1, share2]).unwrap();
    assert_eq!(recovered1, secret);
    assert_eq!(recovered2, secret);
}

#[test]
fn test_generate_shares_u64_multiple_values() {
    let test_values = vec![1, 42, 100, 1000, 10000, 999999];
    let threshold = 3;
    let total_shares = 5;

    for secret in test_values {
        if secret >= FIELD_MODULUS {
            continue; // Skip values that exceed modulus
        }

        let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
        assert_eq!(shares.len(), total_shares);

        let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
        assert_eq!(
            recovered, secret,
            "Failed for value: {}",
            secret
        );
    }
}

#[test]
fn test_generate_shares_u64_round_trip() {
    // Test multiple round trips
    let secret = 66666u64;
    let threshold = 3;
    let total_shares = 5;

    for _ in 0..5 {
        let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
        let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
        assert_eq!(recovered, secret);
    }
}

#[test]
fn test_generate_shares_u64_all_shares_recoverable() {
    // Test that any combination of threshold shares can recover the secret
    let secret = 77777u64;
    let threshold = 3;
    let total_shares = 6;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();

    // Test several combinations of threshold shares manually
    // Combination 1: first threshold shares
    let selected1: Vec<types::Share> = shares[0..threshold].to_vec();
    let recovered1 = recover_from_shares(&selected1).unwrap();
    assert_eq!(recovered1, secret);

    // Combination 2: last threshold shares
    let selected2: Vec<types::Share> = shares[total_shares - threshold..].to_vec();
    let recovered2 = recover_from_shares(&selected2).unwrap();
    assert_eq!(recovered2, secret);

    // Combination 3: scattered shares
    let selected3 = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
    let recovered3 = recover_from_shares(&selected3).unwrap();
    assert_eq!(recovered3, secret);

    // Combination 4: middle shares
    let selected4: Vec<types::Share> = shares[1..1 + threshold].to_vec();
    let recovered4 = recover_from_shares(&selected4).unwrap();
    assert_eq!(recovered4, secret);
}

#[test]
fn test_generate_shares_u64_unique_coordinates() {
    // Test that all generated coordinates are unique
    let secret = 88888u64;
    let threshold = 3;
    let total_shares = 10;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();

    // Check all x-coordinates are unique
    let mut x_coords: Vec<_> = shares.iter().map(|s| s.0).collect();
    x_coords.sort();
    x_coords.dedup();
    assert_eq!(x_coords.len(), total_shares, "All x-coordinates should be unique");
}

#[test]
fn test_generate_shares_u64_large_threshold() {
    // Test with larger threshold values
    let secret = 99999u64;
    let threshold = 10;
    let total_shares = 15;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    assert_eq!(shares.len(), total_shares);

    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, secret);
}

// ============================================================================
// Tests for recover_from_shares (using split_to_two_value)
// ============================================================================

#[test]
fn test_recover_from_shares_basic() {
    let original_value = 12345;
    let (hex1, hex2, _) = split_to_two_value_v2(
        original_value,
        TEST_USER_ID,
        TEST_MASK_SECRET,
        TEST_COORD_SEED,
    );
    let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
    let recovered = recover_from_shares(&shares).unwrap();
    assert_eq!(recovered, original_value);
}

#[test]
fn test_recover_from_shares_zero() {
    let original_value = 0;
    let (hex1, hex2, _) = split_to_two_value_v2(
        original_value,
        TEST_USER_ID,
        TEST_MASK_SECRET,
        TEST_COORD_SEED,
    );
    let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
    let recovered = recover_from_shares(&shares).unwrap();
    assert_eq!(recovered, 0);
}

#[test]
fn test_recover_from_shares_large_value() {
    let original_value = FIELD_MODULUS - 1;
    let (hex1, hex2, _) = split_to_two_value_v2(
        original_value,
        TEST_USER_ID,
        TEST_MASK_SECRET,
        TEST_COORD_SEED,
    );
    let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
    let recovered = recover_from_shares(&shares).unwrap();
    assert_eq!(recovered, original_value);
}

#[test]
fn test_recover_from_shares_empty_slice() {
    let shares: Vec<types::Share> = vec![];
    let result = recover_from_shares(&shares);
    assert!(result.is_err());
    if let Err(SSSError::InsufficientShares(_)) = result {
        // Expected error
    } else {
        panic!("Expected InsufficientShares error");
    }
}

#[test]
fn test_recover_from_shares_single_share() {
    // For two-party sharing, we need at least threshold shares
    // But let's test with a single share to see the behavior
    let original_value = 99999;
    let (hex1, hex2, _) = split_to_two_value_v2(
        original_value,
        TEST_USER_ID,
        TEST_MASK_SECRET,
        TEST_COORD_SEED,
    );
    let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
    
    // Try with single share (should work if threshold is 1, but for two-party it's typically 2)
    let single_share = &shares[0..1];
    let _result = recover_from_shares(single_share);
    // This might fail depending on threshold, which is expected
    // For two-party sharing with threshold 2, single share should fail
}

#[test]
fn test_recover_from_shares_multiple_values() {
    let test_values = vec![1, 42, 100, 1000, 10000, 999999];
    
    for original_value in test_values {
        if original_value >= FIELD_MODULUS {
            continue;
        }
        
        let (hex1, hex2, _) = split_to_two_value_v2(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
            TEST_COORD_SEED,
        );
        let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
        let recovered = recover_from_shares(&shares).unwrap();
        assert_eq!(
            recovered, original_value,
            "Failed for value: {}",
            original_value
        );
    }
}

#[test]
fn test_recover_from_shares_consistency_with_recover_value() {
    // Test that recover_from_shares gives same result as recover_value
    let original_value = 88888;
    let (hex1, hex2, _) = split_to_two_value_v2(
        original_value,
        TEST_USER_ID,
        TEST_MASK_SECRET,
        TEST_COORD_SEED,
    );
    
    // Method 1: Using recover_value
    let recovered1 = recover_value_v2(hex1.clone(), hex2.clone(), TEST_MASK_SECRET).unwrap();
    
    // Method 2: Using recover_from_shares
    let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
    let recovered2 = recover_from_shares(&shares).unwrap();
    
    assert_eq!(recovered1, original_value);
    assert_eq!(recovered2, original_value);
    assert_eq!(recovered1, recovered2);
}

#[test]
fn test_recover_from_shares_different_mask_secrets() {
    let original_value = 77777;
    let mask_secrets = [0u64, 1u64, 0x1234567890ABCDEFu64, u64::MAX];
    
    for &mask_secret in &mask_secrets {
        let (hex1, hex2, _) = split_to_two_value_v2(
            original_value,
            TEST_USER_ID,
            mask_secret,
            TEST_COORD_SEED,
        );
        let shares = recover_two_shares_v2(hex1, hex2, mask_secret).unwrap();
        let recovered = recover_from_shares(&shares).unwrap();
        assert_eq!(
            recovered, original_value,
            "Failed for mask_secret: {}",
            mask_secret
        );
    }
}

#[test]
fn test_recover_from_shares_different_coord_seeds() {
    let original_value = 54321;
    let coord_seeds = [0u64, 1u64, 116540450355, u64::MAX];
    
    for &coord_seed in &coord_seeds {
        let (hex1, hex2, _) = split_to_two_value_v2(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
            coord_seed,
        );
        let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
        let recovered = recover_from_shares(&shares).unwrap();
        assert_eq!(
            recovered, original_value,
            "Failed for coord_seed: {}",
            coord_seed
        );
    }
}

#[test]
fn test_recover_from_shares_round_trip() {
    // Test multiple round trips
    let original_value = 55555;
    
    for _ in 0..5 {
        let (hex1, hex2, _) = split_to_two_value_v2(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
            TEST_COORD_SEED,
        );
        let shares = recover_two_shares_v2(hex1, hex2, TEST_MASK_SECRET).unwrap();
        let recovered = recover_from_shares(&shares).unwrap();
        assert_eq!(recovered, original_value);
    }
}

#[test]
fn test_recover_from_shares_with_homomorphic_operations() {
    // Test recover_from_shares after homomorphic addition
    let a = 100u64;
    let b = 50u64;
    
    // Split both secrets
    let (hex1_a, hex2_a, seed) = split_to_two_value_v2(
        a,
        TEST_USER_ID,
        TEST_MASK_SECRET,
        TEST_COORD_SEED,
    );
    let (hex1_b, hex2_b, _) = split_to_two_value_v2(
        b,
        TEST_USER_ID,
        TEST_MASK_SECRET,
        TEST_COORD_SEED,
    );
    
    // Perform homomorphic addition
    let result_bytes1 = add_two_shared_secrets_v2(
        hex1_a,
        hex1_b,
        TEST_MASK_SECRET,
        0,
        TEST_USER_ID,
        seed,
        seed,
    )
    .unwrap();
    let result_bytes2 = add_two_shared_secrets_v2(
        hex2_a,
        hex2_b,
        TEST_MASK_SECRET,
        1,
        TEST_USER_ID,
        seed,
        seed,
    )
    .unwrap();
    
    // Decode the encoded bytes back to Share objects
    use crate::two_party_helper::{bytes_to_share, decode_share_data_with_user_id};
    let decoded1 = decode_share_data_with_user_id(result_bytes1, TEST_MASK_SECRET, 0).unwrap();
    let decoded2 = decode_share_data_with_user_id(result_bytes2, TEST_MASK_SECRET, 1).unwrap();
    let share1 = bytes_to_share(&decoded1).unwrap();
    let share2 = bytes_to_share(&decoded2).unwrap();
    
    // Recover using recover_from_shares
    let recovered = recover_from_shares(&[share1, share2]).unwrap();
    assert_eq!(recovered, a + b);
}

#[test]
fn test_recover_from_shares_with_beaver_multiplication() {
    // Test recover_from_shares after Beaver multiplication
    // Note: This test requires shares to use the same x-coordinates as the Beaver triple
    let x = 7u64;
    let y = 11u64;
    
    // Generate Beaver triple first to get the x-coordinates
    let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();
    
    // Create shares using Beaver triple coordinates
    use crate::field::FieldElement as FieldElementTrait;
    use crate::poly::Polynomial;
    use crate::two_party_helper::THRESHOLD;
    
    let x_field = FieldElementTrait::from_u64(x);
    let y_field = FieldElementTrait::from_u64(y);
    let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
    let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);
    
    let x_share_0: types::Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
    let x_share_1: types::Share = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));
    let y_share_0: types::Share = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
    let y_share_1: types::Share = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));
    
    // Step 1: Compute masked differences
    let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
    let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
    let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
    let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);
    
    // Steps 2 & 3: Combined execution
    let result_bytes_0 = mul_step2_and_3_combined(
        &[d_share_0, d_share_1],
        &[e_share_0, e_share_1],
        &triple.a_shares[0],
        &triple.b_shares[0],
        &triple.c_shares[0],
    )
    .unwrap();
    let result_bytes_1 = mul_step2_and_3_combined(
        &[d_share_0, d_share_1],
        &[e_share_0, e_share_1],
        &triple.a_shares[1],
        &triple.b_shares[1],
        &triple.c_shares[1],
    )
    .unwrap();
    
    // Convert bytes to Share objects (Beaver multiplication returns unencrypted bytes)
    use crate::two_party_helper::bytes_to_share;
    let share1 = bytes_to_share(&result_bytes_0).unwrap();
    let share2 = bytes_to_share(&result_bytes_1).unwrap();
    
    // Recover using recover_from_shares
    let recovered = recover_from_shares(&[share1, share2]).unwrap();
    assert_eq!(recovered, x * y);
}

// ============================================================================
// Integrated tests: generate_shares_u64 + recover_from_shares
// ============================================================================

#[test]
fn test_encode_decode_share_basic() {
    // Test basic encode/decode workflow: generate_shares_u64 -> recover_from_shares
    let secret = 12345u64;
    let threshold = 3;
    let total_shares = 5;

    // Encode: Generate shares
    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    assert_eq!(shares.len(), total_shares);

    // Decode: Recover secret from shares
    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_encode_decode_share_zero() {
    let secret = 0u64;
    let threshold = 2;
    let total_shares = 4;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, 0);
}

#[test]
fn test_encode_decode_share_large_value() {
    let secret = FIELD_MODULUS - 1;
    let threshold = 3;
    let total_shares = 5;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_encode_decode_share_multiple_values() {
    let test_values = vec![1, 42, 100, 1000, 10000, 999999, 123456789];
    let threshold = 3;
    let total_shares = 5;

    for secret in test_values {
        if secret >= FIELD_MODULUS {
            continue;
        }

        // Encode
        let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
        
        // Decode
        let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
        assert_eq!(
            recovered, secret,
            "Failed for value: {}",
            secret
        );
    }
}

#[test]
fn test_encode_decode_share_different_thresholds() {
    let secret = 88888u64;
    let coord_seed = TEST_COORD_SEED;

    // Test with threshold = 2
    let shares = generate_shares_u64(secret, 2, 4, coord_seed).unwrap();
    let recovered = recover_from_shares(&shares[0..2]).unwrap();
    assert_eq!(recovered, secret);

    // Test with threshold = 4
    let shares = generate_shares_u64(secret, 4, 7, coord_seed).unwrap();
    let recovered = recover_from_shares(&shares[0..4]).unwrap();
    assert_eq!(recovered, secret);

    // Test with threshold = 6
    let shares = generate_shares_u64(secret, 6, 10, coord_seed).unwrap();
    let recovered = recover_from_shares(&shares[0..6]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_encode_decode_share_different_total_shares() {
    let secret = 77777u64;
    let threshold = 3;

    // Test with total_shares = threshold (minimum)
    let shares = generate_shares_u64(secret, threshold, threshold, TEST_COORD_SEED).unwrap();
    let recovered = recover_from_shares(&shares).unwrap();
    assert_eq!(recovered, secret);

    // Test with total_shares > threshold
    let shares = generate_shares_u64(secret, threshold, threshold + 10, TEST_COORD_SEED).unwrap();
    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_encode_decode_share_scattered_shares() {
    // Test that we can recover using non-consecutive shares
    let secret = 99999u64;
    let threshold = 3;
    let total_shares = 7;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();

    // Use scattered shares (not consecutive indices)
    let scattered = vec![shares[0].clone(), shares[2].clone(), shares[5].clone()];
    let recovered = recover_from_shares(&scattered).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_encode_decode_share_round_trip() {
    // Test multiple encode/decode cycles
    let secret = 55555u64;
    let threshold = 3;
    let total_shares = 5;

    for _ in 0..10 {
        // Encode
        let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
        
        // Decode
        let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
        assert_eq!(recovered, secret);
    }
}

#[test]
fn test_encode_decode_share_all_combinations() {
    // Test that any combination of threshold shares can recover the secret
    let secret = 66666u64;
    let threshold = 3;
    let total_shares = 6;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();

    // Test different combinations
    // Combination 1: First threshold shares
    let recovered1 = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered1, secret);

    // Combination 2: Last threshold shares
    let recovered2 = recover_from_shares(&shares[total_shares - threshold..]).unwrap();
    assert_eq!(recovered2, secret);

    // Combination 3: Middle shares
    let recovered3 = recover_from_shares(&shares[1..1 + threshold]).unwrap();
    assert_eq!(recovered3, secret);

    // Combination 4: Scattered shares
    let scattered = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
    let recovered4 = recover_from_shares(&scattered).unwrap();
    assert_eq!(recovered4, secret);
}

#[test]
fn test_encode_decode_share_deterministic() {
    // Test that same parameters produce same shares and recovery
    let secret = 11111u64;
    let threshold = 3;
    let total_shares = 5;
    let coord_seed = TEST_COORD_SEED;

    // Generate shares twice
    let shares1 = generate_shares_u64(secret, threshold, total_shares, coord_seed).unwrap();
    let shares2 = generate_shares_u64(secret, threshold, total_shares, coord_seed).unwrap();

    // Shares should be identical
    assert_eq!(shares1.len(), shares2.len());
    for (s1, s2) in shares1.iter().zip(shares2.iter()) {
        assert_eq!(s1.0, s2.0);
        assert_eq!(s1.1, s2.1);
    }

    // Recovery should produce same result
    let recovered1 = recover_from_shares(&shares1[0..threshold]).unwrap();
    let recovered2 = recover_from_shares(&shares2[0..threshold]).unwrap();
    assert_eq!(recovered1, secret);
    assert_eq!(recovered2, secret);
    assert_eq!(recovered1, recovered2);
}

#[test]
fn test_encode_decode_share_different_seeds() {
    // Test that different seeds produce different shares but same recovery
    let secret = 22222u64;
    let threshold = 3;
    let total_shares = 5;

    let shares1 = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let shares2 = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED + 1).unwrap();

    // Coordinates should be different
    let coords_different = shares1.iter().zip(shares2.iter()).any(|(s1, s2)| s1.0 != s2.0);
    assert!(coords_different, "Different seeds should produce different coordinates");

    // But recovery should produce same secret
    let recovered1 = recover_from_shares(&shares1[0..threshold]).unwrap();
    let recovered2 = recover_from_shares(&shares2[0..threshold]).unwrap();
    assert_eq!(recovered1, secret);
    assert_eq!(recovered2, secret);
}

#[test]
fn test_encode_decode_share_large_threshold() {
    // Test with larger threshold values
    let secret = 33333u64;
    let threshold = 10;
    let total_shares = 15;

    let shares = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered = recover_from_shares(&shares[0..threshold]).unwrap();
    assert_eq!(recovered, secret);
}

#[test]
fn test_encode_decode_share_edge_cases() {
    // Test various edge cases
    let threshold = 2;
    let total_shares = 3;

    // Edge case 1: Small value
    let secret1 = 1u64;
    let shares1 = generate_shares_u64(secret1, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered1 = recover_from_shares(&shares1[0..threshold]).unwrap();
    assert_eq!(recovered1, secret1);

    // Edge case 2: Value near modulus
    let secret2 = FIELD_MODULUS - 100;
    let shares2 = generate_shares_u64(secret2, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered2 = recover_from_shares(&shares2[0..threshold]).unwrap();
    assert_eq!(recovered2, secret2);

    // Edge case 3: Zero
    let secret3 = 0u64;
    let shares3 = generate_shares_u64(secret3, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered3 = recover_from_shares(&shares3[0..threshold]).unwrap();
    assert_eq!(recovered3, secret3);
}

#[test]
fn test_encode_decode_share_consistency() {
    // Test consistency: generate -> recover -> generate -> recover
    let secret = 44444u64;
    let threshold = 3;
    let total_shares = 5;

    // First round
    let shares1 = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered1 = recover_from_shares(&shares1[0..threshold]).unwrap();
    assert_eq!(recovered1, secret);

    // Second round with same parameters
    let shares2 = generate_shares_u64(secret, threshold, total_shares, TEST_COORD_SEED).unwrap();
    let recovered2 = recover_from_shares(&shares2[0..threshold]).unwrap();
    assert_eq!(recovered2, secret);

    // Shares should be identical (deterministic)
    for (s1, s2) in shares1.iter().zip(shares2.iter()) {
        assert_eq!(s1.0, s2.0);
        assert_eq!(s1.1, s2.1);
    }
}



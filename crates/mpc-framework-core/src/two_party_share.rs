//! Two-Party Secret Sharing Module
//!
//! Implements (2,2)-threshold secret sharing with:
//! - Blake2b256 cryptographic hashing for deterministic key derivation
//! - XOR encryption with deterministic shuffle permutations
//! - Compatible with mpc-transmission API (user_id, mask_secret parameters)
//!
//! # Constants
//! - THRESHOLD = 2 (both shares required for recovery)
//! - TOTAL_SHARES = 2 (exactly two shares generated)
//!
//! # Core Functions (same names as mpc-transmission)
//! - `split_to_two_value`: Split a secret into two hex-encoded shares
//! - `recover_value`: Recover secret from two hex-encoded shares
//! - `recover_two_shares`: Recover Share objects from hex strings
//! - `add_two_shared_secrets`: Add two secrets using shares
//! - `sub_two_shared_secrets`: Subtract two secrets using shares
//! - Beaver multiplication: Step-by-step API for secure multiplication using Beaver triples

use crate::error::SSSError;
use crate::field::gf64_sss::FieldElement;
use crate::field::FieldElement as FieldElementTrait;
use crate::types::Share;

// Re-export helper functions for internal use
pub use crate::two_party_helper::{
    bytes_to_share,
    decode_share_data_with_user_id,
    // Encoding/Decoding
    encode_share_data_with_user_id,
    // Beaver triple
    generate_beaver_triple,
    generate_beaver_triple_with_values,
    // Coordinate generation
    get_two_party_coordinates,
    recover_from_shares_internal,
    share_to_bytes,
    split_to_two_value_internal,
    // Constants
    THRESHOLD,
    TOTAL_SHARES,
};

// ============================================================================
// Public API (compatible with mpc-transmission)
// ============================================================================

/// Split a secret value into two hex-encoded shares
///
/// Compatible with mpc-transmission API. Uses fixed-seed polynomial generation.
///
/// # Arguments
/// * `value` - The u64 value to split
/// * `user_id` - User identifier for data interleaving
/// * `mask_secret` - Secret used for XOR masking and shuffle
/// * `coord_seed` - Seed for generating x-coordinates
///
/// # Returns
/// Tuple of (hex1, hex2, seed) where:
/// - `hex1` - First hex-encoded share
/// - `hex2` - Second hex-encoded share
/// - `seed` - The seed used for coordinate generation
///
/// # Example
/// ```ignore
/// let (hex1, hex2, seed) = split_to_two_value(12345, 1, 0x1234567890ABCDEF, 0xABCDEF);
/// ```
pub fn split_to_two_value(
    value: u64,
    user_id: u64,
    mask_secret: u64,
    coord_seed: u64,
) -> (String, String, u64) {
    // Step 1: Use coord_seed to generate x-coordinates (independent of value!)
    let coords = get_two_party_coordinates(coord_seed);

    // Step 2: Shamir split the ORIGINAL value (split first!)
    let (share1, share2) =
        split_to_two_value_internal(value, &coords).expect("Internal split should not fail");

    // Step 3: Convert shares to bytes
    let share1_bytes = share_to_bytes(&share1);
    let share2_bytes = share_to_bytes(&share2);

    // Step 4: Encode with mask + user_id interleaving + shuffle/XOR (then mask!)
    let encoded_share1 = encode_share_data_with_user_id(share1_bytes, mask_secret, user_id, 0);
    let encoded_share2 = encode_share_data_with_user_id(share2_bytes, mask_secret, user_id, 1);

    // Step 5: Return as hex strings
    (
        hex::encode(encoded_share1),
        hex::encode(encoded_share2),
        coord_seed,
    )
}

/// Recover secret value from two hex-encoded shares
///
/// # Arguments
/// * `value1` - First hex-encoded share
/// * `value2` - Second hex-encoded share
/// * `mask_secret` - Mask secret used during encoding
///
/// # Returns
/// * `Ok(u64)` - The recovered secret value
/// * `Err(SSSError)` - If decoding or recovery fails
pub fn recover_value(value1: String, value2: String, mask_secret: u64) -> Result<u64, SSSError> {
    // Step 1: Decode hex -> unmask -> get shares
    let shares = recover_two_shares(value1, value2, mask_secret)?;
    // Step 2: Interpolate to recover original value
    recover_from_shares_internal(&shares)
}

/// Recover Share objects from two hex-encoded strings
///
/// # Arguments
/// * `value1` - First hex-encoded share
/// * `value2` - Second hex-encoded share
/// * `mask_secret` - Mask secret used during encoding
///
/// # Returns
/// * `Ok(Vec<Share>)` - Vector containing two recovered Share objects
/// * `Err(SSSError)` - If decoding fails
pub fn recover_two_shares(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<Vec<Share>, SSSError> {
    // Decode hex strings
    let encoded_value1: Vec<u8> = hex::decode(&value1)
        .map_err(|e| SSSError::InvalidShareFormat(format!("Invalid hex in value1: {}", e)))?;
    let encoded_value2: Vec<u8> = hex::decode(&value2)
        .map_err(|e| SSSError::InvalidShareFormat(format!("Invalid hex in value2: {}", e)))?;

    // Decode share data
    let decoded_value1 = decode_share_data_with_user_id(encoded_value1, mask_secret, 0)?;
    let decoded_value2 = decode_share_data_with_user_id(encoded_value2, mask_secret, 1)?;

    // Convert to Share objects
    let share1 = bytes_to_share(&decoded_value1)?;
    let share2 = bytes_to_share(&decoded_value2)?;

    Ok(vec![share1, share2])
}

/// Add two shared secrets homomorphically (single share version)
///
/// Performs homomorphic addition on a single share position.
/// Call twice (index=0 and index=1) to process both shares.
///
/// # Arguments
/// * `hex_a` - Share of secret A (hex encoded)
/// * `hex_b` - Share of secret B (hex encoded)
/// * `mask_secret` - Mask secret used during encoding
/// * `index` - Share index (0 or 1)
/// * `coord_seed_a` - Coordinate seed used when creating A's shares
/// * `coord_seed_b` - Coordinate seed used when creating B's shares
///
/// # Returns
/// * `Ok(Vec<u8>)` - Raw share bytes of the sum
/// * `Err(SSSError)` - If coord_seeds don't match or decoding fails
pub fn add_two_shared_secrets(
    hex_a: String,
    hex_b: String,
    mask_secret: u64,
    index: u8,
    coord_seed_a: u64,
    coord_seed_b: u64,
) -> Result<Vec<u8>, SSSError> {
    // Verify same coordinate system
    if coord_seed_a != coord_seed_b {
        return Err(SSSError::InvalidParameters(
            "Coordinate seeds must match for homomorphic addition".into(),
        ));
    }

    // 1. Hex decode
    let bytes_a = hex::decode(&hex_a).map_err(|e| SSSError::InvalidParameters(e.to_string()))?;
    let bytes_b = hex::decode(&hex_b).map_err(|e| SSSError::InvalidParameters(e.to_string()))?;

    // 2. Decode (remove shuffle/XOR + user_id interleaving)
    let decoded_a = decode_share_data_with_user_id(bytes_a, mask_secret, index)?;
    let decoded_b = decode_share_data_with_user_id(bytes_b, mask_secret, index)?;

    // 3. Parse to (x, y) coordinates
    let share_a = bytes_to_share(&decoded_a)?;
    let share_b = bytes_to_share(&decoded_b)?;

    // 4. Verify x coordinates match (ensures same coordinate system)
    if share_a.0 != share_b.0 {
        return Err(SSSError::InvalidParameters(format!(
            "X coordinates must match for homomorphic addition. Got x_a={:?}, x_b={:?}",
            share_a.0, share_b.0
        )));
    }

    // 5. Add y coordinates in finite field
    let new_share = (share_a.0, share_a.1 + share_b.1);

    // 6. Return raw share bytes
    Ok(share_to_bytes(&new_share))
}

/// Subtract two shared secrets homomorphically (single share version)
///
/// Performs homomorphic subtraction on a single share position.
/// Call twice (index=0 and index=1) to process both shares.
///
/// # Arguments
/// * `hex_a` - Share of secret A (hex encoded, minuend)
/// * `hex_b` - Share of secret B (hex encoded, subtrahend)
/// * `mask_secret` - Mask secret used during encoding
/// * `index` - Share index (0 or 1)
/// * `coord_seed_a` - Coordinate seed used when creating A's shares
/// * `coord_seed_b` - Coordinate seed used when creating B's shares
///
/// # Returns
/// * `Ok(Vec<u8>)` - Raw share bytes of the difference
/// * `Err(SSSError)` - If coord_seeds don't match or decoding fails
pub fn sub_two_shared_secrets(
    hex_a: String,
    hex_b: String,
    mask_secret: u64,
    index: u8,
    coord_seed_a: u64,
    coord_seed_b: u64,
) -> Result<Vec<u8>, SSSError> {
    // Verify same coordinate system
    if coord_seed_a != coord_seed_b {
        return Err(SSSError::InvalidParameters(
            "Coordinate seeds must match for homomorphic subtraction".into(),
        ));
    }

    // 1. Hex decode
    let bytes_a = hex::decode(&hex_a).map_err(|e| SSSError::InvalidParameters(e.to_string()))?;
    let bytes_b = hex::decode(&hex_b).map_err(|e| SSSError::InvalidParameters(e.to_string()))?;

    // 2. Decode (remove shuffle/XOR + user_id interleaving)
    let decoded_a = decode_share_data_with_user_id(bytes_a, mask_secret, index)?;
    let decoded_b = decode_share_data_with_user_id(bytes_b, mask_secret, index)?;

    // 3. Parse to (x, y) coordinates
    let share_a = bytes_to_share(&decoded_a)?;
    let share_b = bytes_to_share(&decoded_b)?;

    // 4. Verify x coordinates match (ensures same coordinate system)
    if share_a.0 != share_b.0 {
        return Err(SSSError::InvalidParameters(format!(
            "X coordinates must match for homomorphic subtraction. Got x_a={:?}, x_b={:?}",
            share_a.0, share_b.0
        )));
    }

    // 5. Subtract y coordinates in finite field
    let new_share = (share_a.0, share_a.1 - share_b.1);

    // 6. Return raw share bytes
    Ok(share_to_bytes(&new_share))
}

// ============================================================================
// Beaver Triple Multiplication - Step-by-Step API
// ============================================================================

/// Step 1: Compute masked difference (local computation)
///
/// Each party computes [d]_i = [x]_i - [a]_i (or [e]_i = [y]_i - [b]_i)
/// This is done locally without any communication.
///
/// # Arguments
/// * `share_val` - Party's share of the value ([x]_i or [y]_i)
/// * `beaver_share` - Party's share of Beaver triple component ([a]_i or [b]_i)
///
/// # Returns
/// * Share - The masked difference share [d]_i or [e]_i
pub fn mul_step1_compute_masked_diff(share_val: &Share, beaver_share: &Share) -> Share {
    (share_val.0, share_val.1 - beaver_share.1)
}

/// Step 1 (with hex input): Compute masked difference from encrypted shares
///
/// This version accepts hex-encoded encrypted shares and decrypts them first.
/// Each party computes [d]_i = [x]_i - [a]_i (or [e]_i = [y]_i - [b]_i)
///
/// # Arguments
/// * `hex_share_val` - Hex-encoded encrypted share of the value
/// * `hex_beaver_share` - Hex-encoded encrypted share of Beaver triple component
/// * `mask_secret` - Mask secret used during encoding
/// * `index` - Share index (0 or 1)
/// * `coord_seed_val` - Coordinate seed used when creating value's shares
/// * `coord_seed_beaver` - Coordinate seed used when creating Beaver triple's shares
///
/// # Returns
/// * `Ok(Share)` - The masked difference share [d]_i or [e]_i
/// * `Err(SSSError)` - If decryption or computation fails
pub fn mul_step1_compute_masked_diff_from_hex(
    hex_share_val: String,
    hex_beaver_share: String,
    mask_secret: u64,
    index: u8,
    coord_seed_val: u64,
    coord_seed_beaver: u64,
) -> Result<Share, SSSError> {
    // Verify same coordinate system
    if coord_seed_val != coord_seed_beaver {
        return Err(SSSError::InvalidParameters(
            "Coordinate seeds must match for Beaver multiplication".into(),
        ));
    }

    // 1. Hex decode
    let bytes_val =
        hex::decode(&hex_share_val).map_err(|e| SSSError::InvalidParameters(e.to_string()))?;
    let bytes_beaver =
        hex::decode(&hex_beaver_share).map_err(|e| SSSError::InvalidParameters(e.to_string()))?;

    // 2. Decode (remove shuffle/XOR + user_id interleaving)
    let decoded_val = decode_share_data_with_user_id(bytes_val, mask_secret, index)?;
    let decoded_beaver = decode_share_data_with_user_id(bytes_beaver, mask_secret, index)?;

    // 3. Parse to (x, y) coordinates
    let share_val = bytes_to_share(&decoded_val)?;
    let beaver_share = bytes_to_share(&decoded_beaver)?;

    // 4. Verify x coordinates match
    if share_val.0 != beaver_share.0 {
        return Err(SSSError::InvalidParameters(format!(
            "X coordinates must match for Beaver multiplication. Got x_val={:?}, x_beaver={:?}",
            share_val.0, beaver_share.0
        )));
    }

    // 5. Compute masked difference
    Ok(mul_step1_compute_masked_diff(&share_val, &beaver_share))
}

/// Step 2: Reconstruct opened values d and e
///
/// After parties exchange their [d]_i and [e]_i shares, they reconstruct
/// the opened values d = x - a and e = y - b.
/// These values are safe to reveal because they are masked by random a and b.
///
/// # Arguments
/// * `d_shares` - All parties' shares of d: [[d]_0, [d]_1]
/// * `e_shares` - All parties' shares of e: [[e]_0, [e]_1]
///
/// # Returns
/// * `Ok((d, e))` - The reconstructed opened values
/// * `Err(SSSError)` - If reconstruction fails
pub fn mul_step2_reconstruct_masked_values(
    d_shares: &[Share],
    e_shares: &[Share],
) -> Result<(u64, u64), SSSError> {
    let d = recover_from_shares_internal(d_shares)?;
    let e = recover_from_shares_internal(e_shares)?;
    Ok((d, e))
}

/// Step 3: Compute final multiplication result (local computation)
///
/// Each party computes their share of the product:
/// [xy]_i = [c]_i + d * [b]_i + e * [a]_i + d * e
///
/// # Arguments
/// * `beaver_a` - Party's share of Beaver triple's a
/// * `beaver_b` - Party's share of Beaver triple's b
/// * `beaver_c` - Party's share of Beaver triple's c
/// * `d_open` - Opened value d = x - a
/// * `e_open` - Opened value e = y - b
///
/// # Returns
/// * `Vec<u8>` - Raw share bytes of the product
pub fn mul_step3_compute_result(
    beaver_a: &Share,
    beaver_b: &Share,
    beaver_c: &Share,
    d_open: u64,
    e_open: u64,
) -> Vec<u8> {
    let d: FieldElement = FieldElementTrait::from_u64(d_open);
    let e: FieldElement = FieldElementTrait::from_u64(e_open);

    // [xy]_i = [c]_i + d * [b]_i + e * [a]_i + d * e
    let result_y = beaver_c.1 + d * beaver_b.1 + e * beaver_a.1 + d * e;

    let new_share = (beaver_c.0, result_y);
    share_to_bytes(&new_share)
}

/// Combined Step 2 & 3: Reconstruct and compute result in one call
///
/// This is a convenience function that combines step 2 (reconstruct d, e) and
/// step 3 (compute final result) into a single operation.
/// Use this when you already have all shares available locally.
///
/// # Arguments
/// * `d_shares` - All parties' shares of d: [[d]_0, [d]_1]
/// * `e_shares` - All parties' shares of e: [[e]_0, [e]_1]
/// * `beaver_a` - Party's share of Beaver triple's a
/// * `beaver_b` - Party's share of Beaver triple's b
/// * `beaver_c` - Party's share of Beaver triple's c
///
/// # Returns
/// * `Ok(Vec<u8>)` - Raw share bytes of the product
/// * `Err(SSSError)` - If reconstruction fails
pub fn mul_step2_and_3_combined(
    d_shares: &[Share],
    e_shares: &[Share],
    beaver_a: &Share,
    beaver_b: &Share,
    beaver_c: &Share,
) -> Result<Vec<u8>, SSSError> {
    // Step 2: Reconstruct d and e
    let (d_open, e_open) = mul_step2_reconstruct_masked_values(d_shares, e_shares)?;

    // Step 3: Compute final result
    Ok(mul_step3_compute_result(
        beaver_a, beaver_b, beaver_c, d_open, e_open,
    ))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::poly::Polynomial;

    const TEST_MASK_SECRET: u64 = 0x1234567890ABCDEFu64;
    const TEST_USER_ID: u64 = 1u64;
    const TEST_COORD_SEED: u64 = 0xABCDEF1234567890u64;

    // ==================== Split and Recover Tests ====================

    #[test]
    fn test_split_to_two_value_returns_hex_and_seed() {
        let (hex1, hex2, seed) =
            split_to_two_value(12345, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        // Should be valid hex strings
        assert!(!hex1.is_empty());
        assert!(!hex2.is_empty());
        assert!(hex::decode(&hex1).is_ok());
        assert!(hex::decode(&hex2).is_ok());

        // The two shares should be different
        assert_ne!(hex1, hex2);

        // Seed should match input
        assert_eq!(seed, TEST_COORD_SEED);
    }

    #[test]
    fn test_split_and_recover_roundtrip() {
        let value = 12345u64;
        let (hex1, hex2, _) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let recovered = recover_value(hex1, hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_split_and_recover_zero() {
        let value = 0u64;
        let (hex1, hex2, _) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let recovered = recover_value(hex1, hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_split_and_recover_large_value() {
        // Note: u64::MAX exceeds the field modulus (18446744069414584321)
        // so we test with a large value within the field
        let value = 18446744069414584320u64; // MODULUS - 1
        let (hex1, hex2, _) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let recovered = recover_value(hex1, hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_recover_with_wrong_mask_fails() {
        let value = 12345u64;
        let (hex1, hex2, _) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        let wrong_mask = 0xFEDCBA0987654321u64;
        let result = recover_value(hex1, hex2, wrong_mask);

        // Should either fail or return wrong value
        match result {
            Ok(recovered) => assert_ne!(recovered, value),
            Err(_) => {} // Expected
        }
    }

    #[test]
    fn test_recover_invalid_hex() {
        let invalid_hex1 = "invalid_hex".to_string();
        let invalid_hex2 = "also_invalid".to_string();

        let result = recover_value(invalid_hex1, invalid_hex2, TEST_MASK_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn test_recover_empty_strings() {
        let empty1 = "".to_string();
        let empty2 = "".to_string();

        let result = recover_value(empty1, empty2, TEST_MASK_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn test_recover_two_shares_returns_shares() {
        let value = 12345u64;
        let (hex1, hex2, _) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        let shares = recover_two_shares(hex1, hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(shares.len(), 2);

        // Verify that recovered shares can reconstruct the original value
        let recovered = recover_from_shares_internal(&shares).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_different_user_ids_produce_different_shares() {
        let value = 12345u64;

        let (hex1_a, hex2_a, seed_a) =
            split_to_two_value(value, 1u64, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, seed_b) =
            split_to_two_value(value, 2u64, TEST_MASK_SECRET, TEST_COORD_SEED);

        // Different user_ids should produce different encoded shares
        assert_ne!(hex1_a, hex1_b);
        assert_ne!(hex2_a, hex2_b);

        // But seeds should be the same
        assert_eq!(seed_a, seed_b);

        // Both should recover to same value
        let recovered_a = recover_value(hex1_a, hex2_a, TEST_MASK_SECRET).unwrap();
        let recovered_b = recover_value(hex1_b, hex2_b, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered_a, recovered_b);
        assert_eq!(recovered_a, value);
    }

    #[test]
    fn test_split_is_deterministic() {
        let value = 12345u64;

        let (hex1_a, hex2_a, seed_a) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, seed_b) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        // Same inputs should produce same outputs
        assert_eq!(hex1_a, hex1_b);
        assert_eq!(hex2_a, hex2_b);
        assert_eq!(seed_a, seed_b);
    }

    // ==================== Homomorphic Addition Tests ====================

    #[test]
    fn test_add_two_shared_secrets() {
        let value1 = 100u64;
        let value2 = 200u64;

        // Split both values using same coord_seed (ensures same x-coordinates)
        let (hex1_a, hex2_a, seed_a) =
            split_to_two_value(value1, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, seed_b) =
            split_to_two_value(value2, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        // Homomorphic addition on each share position
        let result_bytes1 =
            add_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed_a, seed_b).unwrap();
        let result_bytes2 =
            add_two_shared_secrets(hex2_a, hex2_b, TEST_MASK_SECRET, 1, seed_a, seed_b).unwrap();

        // Convert bytes to shares and recover
        let share1 = bytes_to_share(&result_bytes1).unwrap();
        let share2 = bytes_to_share(&result_bytes2).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, value1 + value2);
    }

    #[test]
    fn test_add_with_zero() {
        let value1 = 42u64;
        let value2 = 0u64;

        let (hex1_a, hex2_a, seed_a) =
            split_to_two_value(value1, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, seed_b) =
            split_to_two_value(value2, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        let result_bytes1 =
            add_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed_a, seed_b).unwrap();
        let result_bytes2 =
            add_two_shared_secrets(hex2_a, hex2_b, TEST_MASK_SECRET, 1, seed_a, seed_b).unwrap();

        let share1 = bytes_to_share(&result_bytes1).unwrap();
        let share2 = bytes_to_share(&result_bytes2).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, value1);
    }

    #[test]
    fn test_add_different_coord_seeds_fails() {
        let value1 = 100u64;
        let value2 = 200u64;
        let seed1 = TEST_COORD_SEED;
        let seed2 = 0xFEDCBA0987654321u64;

        let (hex1_a, _, _) = split_to_two_value(value1, TEST_USER_ID, TEST_MASK_SECRET, seed1);
        let (hex1_b, _, _) = split_to_two_value(value2, TEST_USER_ID, TEST_MASK_SECRET, seed2);

        // Should fail because coord_seeds don't match
        let result = add_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed1, seed2);
        assert!(result.is_err());
    }

    // ==================== Homomorphic Subtraction Tests ====================

    #[test]
    fn test_sub_two_shared_secrets() {
        let value1 = 300u64;
        let value2 = 100u64;

        let (hex1_a, hex2_a, seed_a) =
            split_to_two_value(value1, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, seed_b) =
            split_to_two_value(value2, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        let result_bytes1 =
            sub_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed_a, seed_b).unwrap();
        let result_bytes2 =
            sub_two_shared_secrets(hex2_a, hex2_b, TEST_MASK_SECRET, 1, seed_a, seed_b).unwrap();

        let share1 = bytes_to_share(&result_bytes1).unwrap();
        let share2 = bytes_to_share(&result_bytes2).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, value1 - value2);
    }

    #[test]
    fn test_sub_equal_values() {
        let value = 42u64;

        let (hex1_a, hex2_a, seed_a) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, seed_b) =
            split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        let result_bytes1 =
            sub_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed_a, seed_b).unwrap();
        let result_bytes2 =
            sub_two_shared_secrets(hex2_a, hex2_b, TEST_MASK_SECRET, 1, seed_a, seed_b).unwrap();

        let share1 = bytes_to_share(&result_bytes1).unwrap();
        let share2 = bytes_to_share(&result_bytes2).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, 0);
    }

    // ==================== Beaver Triple Multiplication Tests ====================

    #[test]
    fn test_mul_step_by_step() {
        let x = 5u64;
        let y = 7u64;

        // Generate Beaver triple
        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        // Create proper shares for x and y using Beaver triple's x-coordinates
        let x_field: FieldElement = FieldElementTrait::from_u64(x);
        let y_field: FieldElement = FieldElementTrait::from_u64(y);

        // Create polynomial with fixed seed: degree=1 (for 2-threshold), intercept=secret
        let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
        let x_share_0: Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
        let x_share_1: Share = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));

        let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);
        let y_share_0: Share = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
        let y_share_1: Share = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));

        // Step 1: Each party computes masked differences (local)
        let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
        let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
        let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
        let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

        // Step 2: Exchange and reconstruct d, e (after network exchange)
        let (d_open, e_open) =
            mul_step2_reconstruct_masked_values(&[d_share_0, d_share_1], &[e_share_0, e_share_1])
                .unwrap();

        // Step 3: Each party computes result (local)
        let result_bytes_0 = mul_step3_compute_result(
            &triple.a_shares[0],
            &triple.b_shares[0],
            &triple.c_shares[0],
            d_open,
            e_open,
        );
        let result_bytes_1 = mul_step3_compute_result(
            &triple.a_shares[1],
            &triple.b_shares[1],
            &triple.c_shares[1],
            d_open,
            e_open,
        );

        // Verify result
        let share1 = bytes_to_share(&result_bytes_0).unwrap();
        let share2 = bytes_to_share(&result_bytes_1).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, x * y);
    }

    #[test]
    fn test_mul_step2_and_3_combined() {
        let x = 5u64;
        let y = 7u64;

        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        let x_field: FieldElement = FieldElementTrait::from_u64(x);
        let y_field: FieldElement = FieldElementTrait::from_u64(y);
        let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
        let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);
        let x_share_0: Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
        let x_share_1: Share = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));
        let y_share_0: Share = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
        let y_share_1: Share = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));

        // Step 1: Compute masked differences
        let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
        let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
        let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
        let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

        // Step 2 & 3: Combined (reconstruct and compute)
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

        // Verify result
        let share1 = bytes_to_share(&result_bytes_0).unwrap();
        let share2 = bytes_to_share(&result_bytes_1).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, x * y);
    }

    #[test]
    fn test_mul_by_zero() {
        let x = 100u64;
        let y = 0u64;

        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        // Create shares using Beaver triple coordinates
        let x_field: FieldElement = FieldElementTrait::from_u64(x);
        let y_field: FieldElement = FieldElementTrait::from_u64(y);
        let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
        let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);
        let x_share_0: Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
        let x_share_1: Share = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));
        let y_share_0: Share = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
        let y_share_1: Share = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));

        // Step 1
        let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
        let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
        let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
        let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

        // Step 2
        let (d_open, e_open) =
            mul_step2_reconstruct_masked_values(&[d_share_0, d_share_1], &[e_share_0, e_share_1])
                .unwrap();

        // Step 3
        let result_bytes_0 = mul_step3_compute_result(
            &triple.a_shares[0],
            &triple.b_shares[0],
            &triple.c_shares[0],
            d_open,
            e_open,
        );
        let result_bytes_1 = mul_step3_compute_result(
            &triple.a_shares[1],
            &triple.b_shares[1],
            &triple.c_shares[1],
            d_open,
            e_open,
        );

        let share1 = bytes_to_share(&result_bytes_0).unwrap();
        let share2 = bytes_to_share(&result_bytes_1).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_by_one() {
        let x = 42u64;
        let y = 1u64;

        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        // Create shares using Beaver triple coordinates
        let x_field: FieldElement = FieldElementTrait::from_u64(x);
        let y_field: FieldElement = FieldElementTrait::from_u64(y);
        let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
        let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);
        let x_share_0: Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
        let x_share_1: Share = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));
        let y_share_0: Share = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
        let y_share_1: Share = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));

        // Step 1
        let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
        let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
        let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
        let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

        // Step 2
        let (d_open, e_open) =
            mul_step2_reconstruct_masked_values(&[d_share_0, d_share_1], &[e_share_0, e_share_1])
                .unwrap();

        // Step 3
        let result_bytes_0 = mul_step3_compute_result(
            &triple.a_shares[0],
            &triple.b_shares[0],
            &triple.c_shares[0],
            d_open,
            e_open,
        );
        let result_bytes_1 = mul_step3_compute_result(
            &triple.a_shares[1],
            &triple.b_shares[1],
            &triple.c_shares[1],
            d_open,
            e_open,
        );

        let share1 = bytes_to_share(&result_bytes_0).unwrap();
        let share2 = bytes_to_share(&result_bytes_1).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, x * y);
    }

    // ==================== Complex Expression Tests ====================

    #[test]
    fn test_complex_homomorphic_expression() {
        // Test: (a + b) - c = result
        let a = 100u64;
        let b = 50u64;
        let c = 30u64;

        let (hex1_a, hex2_a, seed) =
            split_to_two_value(a, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, _) =
            split_to_two_value(b, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_c, hex2_c, _) =
            split_to_two_value(c, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        // Step 1: a + b (on both share positions)
        let ab_bytes1 =
            add_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed, seed).unwrap();
        let ab_bytes2 =
            add_two_shared_secrets(hex2_a, hex2_b, TEST_MASK_SECRET, 1, seed, seed).unwrap();

        // Convert raw bytes to shares
        let share1_ab = bytes_to_share(&ab_bytes1).unwrap();
        let share2_ab = bytes_to_share(&ab_bytes2).unwrap();

        // Also get c shares directly
        let shares_c = recover_two_shares(hex1_c, hex2_c, TEST_MASK_SECRET).unwrap();

        // Subtract manually on shares
        let result_share1 = (share1_ab.0, share1_ab.1 - shares_c[0].1);
        let result_share2 = (share2_ab.0, share2_ab.1 - shares_c[1].1);

        let result = recover_from_shares_internal(&[result_share1, result_share2]).unwrap();
        assert_eq!(result, (a + b) - c);
    }

    #[test]
    fn test_homomorphic_with_scalar_mul() {
        // Test: a + b = result, then verify the sum
        let a = 10u64;
        let b = 15u64;

        let (hex1_a, hex2_a, seed) =
            split_to_two_value(a, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, _) =
            split_to_two_value(b, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        // a + b
        let ab_bytes1 =
            add_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed, seed).unwrap();
        let ab_bytes2 =
            add_two_shared_secrets(hex2_a, hex2_b, TEST_MASK_SECRET, 1, seed, seed).unwrap();

        let share1 = bytes_to_share(&ab_bytes1).unwrap();
        let share2 = bytes_to_share(&ab_bytes2).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, a + b);
    }

    // ==================== Boundary Value Tests ====================

    #[test]
    fn test_split_boundary_values() {
        // Note: Values must be < field modulus (18446744069414584321)
        let test_cases = vec![
            0u64,
            1u64,
            18446744069414584320u64, // MODULUS - 1
            0x8000000000000000u64,   // Sign bit (within modulus)
            1000000000000000000u64,  // Large value within modulus
        ];

        for value in test_cases {
            let (hex1, hex2, _) =
                split_to_two_value(value, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
            let recovered = recover_value(hex1, hex2, TEST_MASK_SECRET).unwrap();
            assert_eq!(recovered, value, "Failed for value {}", value);
        }
    }

    // ==================== Additional Tests ====================

    #[test]
    fn test_multiple_sequential_operations() {
        // Test: ((a + b) - c) + d = result
        let a = 100u64;
        let b = 50u64;
        let c = 30u64;
        let d = 20u64;

        let (hex1_a, hex2_a, seed) =
            split_to_two_value(a, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_b, hex2_b, _) =
            split_to_two_value(b, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_c, hex2_c, _) =
            split_to_two_value(c, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);
        let (hex1_d, hex2_d, _) =
            split_to_two_value(d, TEST_USER_ID, TEST_MASK_SECRET, TEST_COORD_SEED);

        // a + b
        let ab_bytes1 =
            add_two_shared_secrets(hex1_a, hex1_b, TEST_MASK_SECRET, 0, seed, seed).unwrap();
        let ab_bytes2 =
            add_two_shared_secrets(hex2_a, hex2_b, TEST_MASK_SECRET, 1, seed, seed).unwrap();
        let share1_ab = bytes_to_share(&ab_bytes1).unwrap();
        let share2_ab = bytes_to_share(&ab_bytes2).unwrap();

        // (a + b) - c
        let shares_c = recover_two_shares(hex1_c, hex2_c, TEST_MASK_SECRET).unwrap();
        let share1_abc = (share1_ab.0, share1_ab.1 - shares_c[0].1);
        let share2_abc = (share2_ab.0, share2_ab.1 - shares_c[1].1);

        // ((a + b) - c) + d
        let shares_d = recover_two_shares(hex1_d, hex2_d, TEST_MASK_SECRET).unwrap();
        let share1_final = (share1_abc.0, share1_abc.1 + shares_d[0].1);
        let share2_final = (share2_abc.0, share2_abc.1 + shares_d[1].1);

        let result = recover_from_shares_internal(&[share1_final, share2_final]).unwrap();
        assert_eq!(result, ((a + b) - c) + d);
    }

    #[test]
    fn test_large_multiplication() {
        let x = 1000000u64;
        let y = 2000000u64;

        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        let x_field: FieldElement = FieldElementTrait::from_u64(x);
        let y_field: FieldElement = FieldElementTrait::from_u64(y);
        let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
        let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);
        let x_share_0: Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
        let x_share_1: Share = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));
        let y_share_0: Share = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
        let y_share_1: Share = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));

        let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
        let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
        let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
        let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

        let (d_open, e_open) =
            mul_step2_reconstruct_masked_values(&[d_share_0, d_share_1], &[e_share_0, e_share_1])
                .unwrap();

        let result_bytes_0 = mul_step3_compute_result(
            &triple.a_shares[0],
            &triple.b_shares[0],
            &triple.c_shares[0],
            d_open,
            e_open,
        );
        let result_bytes_1 = mul_step3_compute_result(
            &triple.a_shares[1],
            &triple.b_shares[1],
            &triple.c_shares[1],
            d_open,
            e_open,
        );

        let share1 = bytes_to_share(&result_bytes_0).unwrap();
        let share2 = bytes_to_share(&result_bytes_1).unwrap();
        let result = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(result, x * y);
    }

    // ==================== Beaver Multiplication with Hex Input Tests ====================

    #[test]
    fn test_mul_step1_from_hex() {
        let x = 7u64;
        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        // Create x shares using Beaver triple coordinates
        let x_field: FieldElement = FieldElementTrait::from_u64(x);
        let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
        let x_share_0: Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
        let x_share_1: Share = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));

        // Convert to hex-encoded shares (simulating encrypted shares)
        let x_bytes_0 = share_to_bytes(&x_share_0);
        let x_bytes_1 = share_to_bytes(&x_share_1);
        let a_bytes_0 = share_to_bytes(&triple.a_shares[0]);
        let a_bytes_1 = share_to_bytes(&triple.a_shares[1]);

        // Encode with user_id and mask
        let hex_x_0 = hex::encode(encode_share_data_with_user_id(
            x_bytes_0,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            0,
        ));
        let hex_x_1 = hex::encode(encode_share_data_with_user_id(
            x_bytes_1,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            1,
        ));
        let hex_a_0 = hex::encode(encode_share_data_with_user_id(
            a_bytes_0,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            0,
        ));
        let hex_a_1 = hex::encode(encode_share_data_with_user_id(
            a_bytes_1,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            1,
        ));

        // Test step1 with hex input
        let d_share_0_hex = mul_step1_compute_masked_diff_from_hex(
            hex_x_0,
            hex_a_0,
            TEST_MASK_SECRET,
            0,
            TEST_COORD_SEED,
            TEST_COORD_SEED,
        )
        .unwrap();
        let d_share_1_hex = mul_step1_compute_masked_diff_from_hex(
            hex_x_1,
            hex_a_1,
            TEST_MASK_SECRET,
            1,
            TEST_COORD_SEED,
            TEST_COORD_SEED,
        )
        .unwrap();

        // Compare with direct computation
        let d_share_0_direct = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
        let d_share_1_direct = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);

        assert_eq!(d_share_0_hex.0, d_share_0_direct.0);
        assert_eq!(d_share_0_hex.1, d_share_0_direct.1);
        assert_eq!(d_share_1_hex.0, d_share_1_direct.0);
        assert_eq!(d_share_1_hex.1, d_share_1_direct.1);
    }

    #[test]
    fn test_mul_step1_from_hex_different_coord_seeds_fails() {
        let x = 7u64;
        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        let x_field: FieldElement = FieldElementTrait::from_u64(x);
        let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
        let x_share_0: Share = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));

        let x_bytes_0 = share_to_bytes(&x_share_0);
        let a_bytes_0 = share_to_bytes(&triple.a_shares[0]);

        let hex_x_0 = hex::encode(encode_share_data_with_user_id(
            x_bytes_0,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            0,
        ));
        let hex_a_0 = hex::encode(encode_share_data_with_user_id(
            a_bytes_0,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            0,
        ));

        // Test with different coord_seeds
        let result = mul_step1_compute_masked_diff_from_hex(
            hex_x_0,
            hex_a_0,
            TEST_MASK_SECRET,
            0,
            TEST_COORD_SEED,
            0xFEDCBA0987654321u64,
        );
        assert!(result.is_err());
    }
}

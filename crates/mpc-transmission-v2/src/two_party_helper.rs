//! Two-Party Secret Sharing Helper Module
//!
//! Contains helper functions for two-party secret sharing operations.
//! These are internal utilities used by `two_party_share.rs`.

use crate::beaver::BeaverTriple;
use crate::error::SSSError;
use crate::field::gf64_sss::FieldElement;
use crate::field::FieldElement as FieldElementTrait;
use crate::poly::Polynomial;
use crate::types::Share;
use fastcrypto::hash::{Blake2b256, HashFunction};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore;

/// Threshold for two-party sharing (both shares required)
pub const THRESHOLD: usize = 2;

/// Total number of shares in two-party sharing
pub const TOTAL_SHARES: usize = 2;

// ============================================================================
// Encoding/Decoding Helper Functions
// ============================================================================

/// Get encoded number sequence from mask_secret using Blake2b256
///
/// This determines the interleaving order of share data and user_id bytes.
pub fn get_num_encoded(mask_secret: u64) -> Vec<u8> {
    Blake2b256::digest(&mask_secret.to_le_bytes())
        .as_ref()
        .to_vec()
}

/// Get XOR mask pattern [0, 1, 0, 1, ...]
///
/// Used for the second share's encoding.
pub fn get_xor_mask(len: usize) -> Vec<u8> {
    (0..len).map(|i| if i % 2 == 0 { 0 } else { 1 }).collect()
}

/// Convert u64 to little-endian bytes
pub fn u64_to_bytes(value: u64) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Generate deterministic shuffle permutations based on mask_secret
///
/// Uses Fisher-Yates algorithm with deterministic PRNG seeded by mask_secret.
///
/// # Returns
/// (forward_permutation, inverse_permutation)
/// - forward_permutation[i] = j means original position i goes to position j
/// - inverse_permutation[j] = i means position j contains element from original position i
pub fn generate_shuffle_permutations(length: usize, mask_secret: u64) -> (Vec<usize>, Vec<usize>) {
    if length == 0 {
        return (vec![], vec![]);
    }

    let mut forward_permutation: Vec<usize> = (0..length).collect();
    let mut seed = mask_secret;

    // Fisher-Yates shuffle algorithm
    for i in 0..length {
        seed = seed.wrapping_mul(1103515245u64).wrapping_add(12345u64);
        let j = i + ((seed as usize) % (length - i));
        forward_permutation.swap(i, j);
    }

    // Invert the mapping
    let mut actual_forward = vec![0; length];
    for (new_pos, &original_pos) in forward_permutation.iter().enumerate() {
        actual_forward[original_pos] = new_pos;
    }

    // Generate inverse permutation
    let mut inverse_permutation = vec![0; length];
    for (original_pos, &new_pos) in actual_forward.iter().enumerate() {
        inverse_permutation[new_pos] = original_pos;
    }

    (actual_forward, inverse_permutation)
}

/// Shuffle data using the forward permutation
pub fn shuffle_data(data: &[u8], forward_permutation: &[usize]) -> Vec<u8> {
    let mut shuffled = vec![0u8; data.len()];
    for (original_pos, &new_pos) in forward_permutation.iter().enumerate() {
        shuffled[new_pos] = data[original_pos];
    }
    shuffled
}

/// Unshuffle data using the inverse permutation
pub fn unshuffle_data(data: &[u8], inverse_permutation: &[usize]) -> Vec<u8> {
    let mut unshuffled = vec![0u8; data.len()];
    for (new_pos, &original_pos) in inverse_permutation.iter().enumerate() {
        unshuffled[original_pos] = data[new_pos];
    }
    unshuffled
}

/// Encode share data with user_id interleaving
///
/// # Algorithm
/// 1. Get num_encoded hash from mask_secret
/// 2. For each byte in share_data:
///    - If num_encoded[i] == 0: [share_data[i], user_id_bytes[i]]
///    - If num_encoded[i] != 0: [user_id_bytes[i], share_data[i]]
/// 3. Apply shuffle (index=0) or XOR mask (index=1)
pub fn encode_share_data_with_user_id(
    share_data: Vec<u8>,
    mask_secret: u64,
    user_id: u64,
    index: u8,
) -> Vec<u8> {
    let mut num_encoded = get_num_encoded(mask_secret);
    let mut user_id_bytes = u64_to_bytes(user_id);

    if num_encoded.len() < share_data.len() {
        num_encoded.extend(vec![0u8; share_data.len() - num_encoded.len()]);
    }
    if user_id_bytes.len() < share_data.len() {
        user_id_bytes.extend(vec![0u8; share_data.len() - user_id_bytes.len()]);
    }

    let mut result_data = Vec::with_capacity(share_data.len() * 2);
    for i in 0..share_data.len() {
        if num_encoded[i] == 0 {
            result_data.push(share_data[i]);
            result_data.push(user_id_bytes[i]);
        } else {
            result_data.push(user_id_bytes[i]);
            result_data.push(share_data[i]);
        }
    }

    if index == 0 {
        let (forward_permutation, _) =
            generate_shuffle_permutations(result_data.len(), mask_secret);
        shuffle_data(&result_data, &forward_permutation)
    } else {
        let xor_mask = get_xor_mask(result_data.len());
        result_data
            .iter()
            .zip(xor_mask.iter())
            .map(|(a, b)| a ^ b)
            .collect::<Vec<u8>>()
    }
}

/// Decode share data with user_id extraction
///
/// Reverses the encoding process to extract the original share data.
pub fn decode_share_data_with_user_id(
    share_data: Vec<u8>,
    mask_secret: u64,
    index: u8,
) -> Result<Vec<u8>, SSSError> {
    let unshuffled_data = if index == 0 {
        let (_, inverse_permutation) = generate_shuffle_permutations(share_data.len(), mask_secret);
        unshuffle_data(&share_data, &inverse_permutation)
    } else {
        let xor_mask = get_xor_mask(share_data.len());
        share_data
            .iter()
            .zip(xor_mask.iter())
            .map(|(a, b)| a ^ b)
            .collect::<Vec<u8>>()
    };

    let num_encoded = get_num_encoded(mask_secret);
    let mut result_data = Vec::with_capacity(unshuffled_data.len() / 2);
    for i in 0..unshuffled_data.len() / 2 {
        if i < num_encoded.len() && num_encoded[i] == 0 {
            result_data.push(unshuffled_data[i * 2]);
        } else {
            result_data.push(unshuffled_data[i * 2 + 1]);
        }
    }
    Ok(result_data)
}

// ============================================================================
// Share Conversion Functions
// ============================================================================

/// Convert Share (FieldElement, FieldElement) to bytes
///
/// Serializes the share as [x_bytes (8), y_bytes (8)] = 16 bytes
pub fn share_to_bytes(share: &Share) -> Vec<u8> {
    let x = FieldElementTrait::to_u64(&share.0);
    let y = FieldElementTrait::to_u64(&share.1);
    let mut bytes = Vec::with_capacity(16);
    bytes.extend_from_slice(&x.to_le_bytes());
    bytes.extend_from_slice(&y.to_le_bytes());
    bytes
}

/// Convert bytes to Share (FieldElement, FieldElement)
///
/// Deserializes 16 bytes back to a Share tuple.
pub fn bytes_to_share(data: &[u8]) -> Result<Share, SSSError> {
    if data.len() < 16 {
        return Err(SSSError::InvalidShareFormat(format!(
            "Expected at least 16 bytes, got {}",
            data.len()
        )));
    }

    let x = u64::from_le_bytes(
        data[0..8]
            .try_into()
            .map_err(|_| SSSError::InvalidShareFormat("Invalid x bytes".into()))?,
    );
    let y = u64::from_le_bytes(
        data[8..16]
            .try_into()
            .map_err(|_| SSSError::InvalidShareFormat("Invalid y bytes".into()))?,
    );

    Ok((
        FieldElementTrait::from_u64(x),
        FieldElementTrait::from_u64(y),
    ))
}

// ============================================================================
// Coordinate Generation Functions
// ============================================================================

/// Generate deterministic coordinates for two-party sharing
pub fn get_two_party_coordinates(seed: u64) -> [FieldElement; 2] {
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    [
        FieldElementTrait::from_u64(rng.next_u64()),
        FieldElementTrait::from_u64(rng.next_u64()),
    ]
}

/// Split a secret into two shares using specified coordinates (internal)
///
/// Uses fixed-seed polynomial generation for compatibility.
pub fn split_to_two_value_internal(
    secret: u64,
    coords: &[FieldElement; 2],
) -> Result<(Share, Share), SSSError> {
    let secret_fe = FieldElementTrait::from_u64(secret);
    let poly = Polynomial::new_with_fixed_seed(THRESHOLD - 1, secret_fe);

    let share1 = (coords[0], poly.evaluate(&coords[0]));
    let share2 = (coords[1], poly.evaluate(&coords[1]));

    Ok((share1, share2))
}

/// Recover secret from a slice of internal Share objects
pub fn recover_from_shares_internal(shares: &[Share]) -> Result<u64, SSSError> {
    if shares.is_empty() {
        return Err(SSSError::InsufficientShares(
            "At least one share is required".into(),
        ));
    }
    Polynomial::interpolate(shares).map(|s| FieldElementTrait::to_u64(&s))
}

// ============================================================================
// Beaver Triple Generation
// ============================================================================

/// Generate a Beaver triple for two-party secure multiplication
///
/// Creates random values (a, b) and computes c = a × b in the finite field.
///
/// # Arguments
/// * `mask_secret` - Secret used as RNG seed
///
/// # Returns
/// * `Ok(BeaverTriple)` - The generated Beaver triple
/// * `Err(SSSError)` - If generation fails
pub fn generate_beaver_triple(mask_secret: u64) -> Result<BeaverTriple, SSSError> {
    use crate::field::gf64_sss::random_element;

    let mut rng = ChaCha20Rng::seed_from_u64(mask_secret);

    // Generate fixed x-coordinates (same for all a, b, c shares)
    let x_coords: Vec<FieldElement> = (0..TOTAL_SHARES)
        .map(|_| random_element(&mut rng))
        .collect();

    // Generate random a and b
    let a = rng.next_u64();
    let b = rng.next_u64();

    // Use new_with_coordinates to ensure all shares use same x-coords
    BeaverTriple::new_with_coordinates(a, b, &x_coords, THRESHOLD, &mut rng)
}

/// Generate a Beaver triple with specified values
///
/// # Arguments
/// * `a` - First value
/// * `b` - Second value
/// * `mask_secret` - Secret used as RNG seed
///
/// # Returns
/// * `Ok(BeaverTriple)` - The generated Beaver triple where c = a × b
/// * `Err(SSSError)` - If generation fails
pub fn generate_beaver_triple_with_values(
    a: u64,
    b: u64,
    mask_secret: u64,
) -> Result<BeaverTriple, SSSError> {
    use crate::field::gf64_sss::random_element;

    let mut rng = ChaCha20Rng::seed_from_u64(mask_secret);

    // Generate fixed x-coordinates
    let x_coords: Vec<FieldElement> = (0..TOTAL_SHARES)
        .map(|_| random_element(&mut rng))
        .collect();

    BeaverTriple::new_with_coordinates(a, b, &x_coords, THRESHOLD, &mut rng)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MASK_SECRET: u64 = 0x1234567890ABCDEFu64;
    const TEST_USER_ID: u64 = 1u64;

    #[test]
    fn test_get_num_encoded() {
        let result = get_num_encoded(TEST_MASK_SECRET);
        assert_eq!(result.len(), 32); // Blake2b256 produces 32 bytes

        // Should be deterministic
        let result2 = get_num_encoded(TEST_MASK_SECRET);
        assert_eq!(result, result2);

        // Different inputs should produce different outputs
        let result3 = get_num_encoded(TEST_MASK_SECRET + 1);
        assert_ne!(result, result3);
    }

    #[test]
    fn test_get_xor_mask() {
        let mask = get_xor_mask(5);
        assert_eq!(mask, vec![0, 1, 0, 1, 0]);

        let mask2 = get_xor_mask(0);
        assert!(mask2.is_empty());
    }

    #[test]
    fn test_u64_to_bytes() {
        let bytes = u64_to_bytes(0x0102030405060708u64);
        assert_eq!(bytes, vec![0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01]);
    }

    #[test]
    fn test_shuffle_permutations_roundtrip() {
        let (forward, inverse) = generate_shuffle_permutations(10, TEST_MASK_SECRET);

        // Verify that applying inverse after forward restores original
        let original: Vec<usize> = (0..10).collect();
        let mut shuffled = vec![0; 10];
        for (i, &j) in forward.iter().enumerate() {
            shuffled[j] = original[i];
        }

        let mut restored = vec![0; 10];
        for (j, &i) in inverse.iter().enumerate() {
            restored[i] = shuffled[j];
        }

        assert_eq!(original, restored);
    }

    #[test]
    fn test_shuffle_data_changes_order() {
        let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let (forward, _) = generate_shuffle_permutations(data.len(), TEST_MASK_SECRET);
        let shuffled = shuffle_data(&data, &forward);

        // Data should be shuffled (different order)
        assert_ne!(data, shuffled);

        // Same length
        assert_eq!(data.len(), shuffled.len());
    }

    #[test]
    fn test_encode_decode_share_data_roundtrip() {
        let share_data = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        // Test index 0 (shuffle)
        let encoded0 =
            encode_share_data_with_user_id(share_data.clone(), TEST_MASK_SECRET, TEST_USER_ID, 0);
        let decoded0 = decode_share_data_with_user_id(encoded0, TEST_MASK_SECRET, 0).unwrap();
        assert_eq!(decoded0, share_data);

        // Test index 1 (XOR mask)
        let encoded1 =
            encode_share_data_with_user_id(share_data.clone(), TEST_MASK_SECRET, TEST_USER_ID, 1);
        let decoded1 = decode_share_data_with_user_id(encoded1, TEST_MASK_SECRET, 1).unwrap();
        assert_eq!(decoded1, share_data);
    }

    #[test]
    fn test_share_to_bytes_and_back() {
        let x = FieldElementTrait::from_u64(12345u64);
        let y = FieldElementTrait::from_u64(67890u64);
        let share: Share = (x, y);

        let bytes = share_to_bytes(&share);
        assert_eq!(bytes.len(), 16);

        let recovered = bytes_to_share(&bytes).unwrap();
        assert_eq!(FieldElementTrait::to_u64(&recovered.0), 12345u64);
        assert_eq!(FieldElementTrait::to_u64(&recovered.1), 67890u64);
    }

    #[test]
    fn test_get_two_party_coordinates() {
        let coords = get_two_party_coordinates(TEST_MASK_SECRET);
        assert_eq!(coords.len(), 2);

        // Coordinates should be different
        assert_ne!(coords[0], coords[1]);

        // Should be deterministic
        let coords2 = get_two_party_coordinates(TEST_MASK_SECRET);
        assert_eq!(coords[0], coords2[0]);
        assert_eq!(coords[1], coords2[1]);
    }

    #[test]
    fn test_split_and_recover_internal() {
        let secret = 12345u64;
        let coords = get_two_party_coordinates(TEST_MASK_SECRET);
        let (share1, share2) = split_to_two_value_internal(secret, &coords).unwrap();

        let recovered = recover_from_shares_internal(&[share1, share2]).unwrap();
        assert_eq!(recovered, secret);
    }

    #[test]
    fn test_generate_beaver_triple() {
        let triple = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        assert_eq!(triple.a_shares.len(), 2);
        assert_eq!(triple.b_shares.len(), 2);
        assert_eq!(triple.c_shares.len(), 2);

        // Verify c = a * b
        let a = recover_from_shares_internal(&triple.a_shares).unwrap();
        let b = recover_from_shares_internal(&triple.b_shares).unwrap();
        let c = recover_from_shares_internal(&triple.c_shares).unwrap();

        let a_field: FieldElement = FieldElementTrait::from_u64(a);
        let b_field: FieldElement = FieldElementTrait::from_u64(b);
        let expected_c = a_field * b_field;
        assert_eq!(c, FieldElementTrait::to_u64(&expected_c));
    }

    #[test]
    fn test_generate_beaver_triple_with_values() {
        let a = 5u64;
        let b = 7u64;
        let triple = generate_beaver_triple_with_values(a, b, TEST_MASK_SECRET).unwrap();

        // Verify recovered values
        let recovered_a = recover_from_shares_internal(&triple.a_shares).unwrap();
        let recovered_b = recover_from_shares_internal(&triple.b_shares).unwrap();
        let recovered_c = recover_from_shares_internal(&triple.c_shares).unwrap();

        assert_eq!(recovered_a, a);
        assert_eq!(recovered_b, b);

        let a_field: FieldElement = FieldElementTrait::from_u64(a);
        let b_field: FieldElement = FieldElementTrait::from_u64(b);
        let expected_c = a_field * b_field;
        assert_eq!(recovered_c, FieldElementTrait::to_u64(&expected_c));
    }

    #[test]
    fn test_beaver_triple_determinism() {
        let triple1 = generate_beaver_triple(TEST_MASK_SECRET).unwrap();
        let triple2 = generate_beaver_triple(TEST_MASK_SECRET).unwrap();

        // Same seed should produce same triple
        assert_eq!(triple1.a_shares[0].0, triple2.a_shares[0].0);
        assert_eq!(triple1.a_shares[0].1, triple2.a_shares[0].1);
    }
}

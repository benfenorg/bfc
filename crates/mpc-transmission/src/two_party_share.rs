use crate::error::SecretSharingError;
use crate::math::{add_shared_secrets, mul_shared_secrets, sub_shared_secrets};
use crate::share::Share;
use crate::{generate_shares_with_xor, recover_secret_with_xor};
use fastcrypto::hash::HashFunction;

const THRESHOLD: usize = 2;
const TOTAL_SHARES: usize = 2;

fn get_num_encoded(mask_secret: u64) -> Vec<u8> {
    fastcrypto::hash::Blake2b256::digest(&mask_secret.to_le_bytes())
        .as_ref()
        .to_vec()
}

fn u64_to_bytes(value: u64) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Generate a deterministic shuffle permutation based on mask_secret
/// Returns (forward_permutation, inverse_permutation)
/// forward_permutation[i] = j means original position i should go to position j
/// inverse_permutation[j] = i means position j contains element from original position i
fn generate_shuffle_permutations(length: usize, mask_secret: u64) -> (Vec<usize>, Vec<usize>) {
    if length == 0 {
        return (vec![], vec![]);
    }

    // Use mask_secret to seed a simple PRNG for deterministic shuffling
    // We'll track where each original position should go
    let mut forward_permutation: Vec<usize> = (0..length).collect();
    let mut seed = mask_secret;

    // Fisher-Yates shuffle algorithm for deterministic randomness
    // We need to track the mapping: original position -> new position
    // So we'll create a mapping array that we shuffle
    for i in 0..length {
        // Generate pseudo-random value
        seed = seed.wrapping_mul(1103515245u64).wrapping_add(12345u64);
        // Use modulo to get a valid index in range [i, length)
        let j = i + ((seed as usize) % (length - i));
        forward_permutation.swap(i, j);
    }

    // Now forward_permutation[i] represents which original element should be at position i
    // But we need forward_permutation[i] = j meaning original position i goes to position j
    // So we need to invert this mapping
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

/// Shuffle the data using the forward permutation
fn shuffle_data(data: &[u8], forward_permutation: &[usize]) -> Vec<u8> {
    let mut shuffled = vec![0u8; data.len()];
    for (original_pos, &new_pos) in forward_permutation.iter().enumerate() {
        shuffled[new_pos] = data[original_pos];
    }
    shuffled
}

/// Unshuffle the data using the inverse permutation
fn unshuffle_data(data: &[u8], inverse_permutation: &[usize]) -> Vec<u8> {
    let mut unshuffled = vec![0u8; data.len()];
    for (new_pos, &original_pos) in inverse_permutation.iter().enumerate() {
        unshuffled[original_pos] = data[new_pos];
    }
    unshuffled
}

fn encode_share_data(share_data: Vec<u8>, mask_secret: u64, user_id: u64) -> Vec<u8> {
    let mut num_encoded = get_num_encoded(mask_secret);
    let mut user_id_bytes = u64_to_bytes(user_id);
    // The length of result_data is twice that of share_data, meaning each byte in share_data corresponds to two bytes in result_data
    // For each index in share_data: if num_encoded[index] == 0, then result_data[index*2] = share_data[index] and result_data[index*2+1] = user_id[index]
    // If num_encoded[index] != 0, then result_data[index*2] = user_id[index] and result_data[index*2+1] = share_data[index]

    // Validate parameters: the lengths of num_encoded and user_id_bytes must be greater than or equal to share_data's length
    if num_encoded.len() < share_data.len() {
        // If num_encoded's length is less than share_data's length, extend num_encoded to match share_data's length
        num_encoded.extend(vec![0u8; share_data.len() - num_encoded.len()]);
    }
    if user_id_bytes.len() < share_data.len() {
        user_id_bytes.extend(vec![0u8; share_data.len() - user_id_bytes.len()]);
    }

    let mut result_data = Vec::with_capacity(share_data.len() * 2);
    for i in 0..share_data.len() {
        if num_encoded[i] == 0 {
            // If num_encoded[i] == 0: result_data[i*2] = share_data[i], result_data[i*2+1] = user_id_bytes[i]
            result_data.push(share_data[i]);
            result_data.push(user_id_bytes[i]);
        } else {
            // If num_encoded[i] != 0: result_data[i*2] = user_id_bytes[i], result_data[i*2+1] = share_data[i]
            result_data.push(user_id_bytes[i]);
            result_data.push(share_data[i]);
        }
    }

    // Apply shuffle permutation to the final result
    let (forward_permutation, _) = generate_shuffle_permutations(result_data.len(), mask_secret);
    shuffle_data(&result_data, &forward_permutation)
}

fn decode_share_data(share_data: Vec<u8>, mask_secret: u64) -> Result<Vec<u8>, SecretSharingError> {
    // First, unshuffle the data to restore original order
    let (_, inverse_permutation) = generate_shuffle_permutations(share_data.len(), mask_secret);
    let unshuffled_data = unshuffle_data(&share_data, &inverse_permutation);

    let num_encoded = get_num_encoded(mask_secret);
    let mut result_data = Vec::with_capacity(unshuffled_data.len() / 2);
    for i in 0..unshuffled_data.len() / 2 {
        if num_encoded[i] == 0 {
            result_data.push(unshuffled_data[i * 2]);
        } else {
            result_data.push(unshuffled_data[i * 2 + 1]);
        }
    }
    Ok(result_data)
}

/// Recover u64 secret
pub fn split_to_two_value(value: u64, user_id: u64, mask_secret: u64) -> (String, String) {
    let shares = generate_shares_with_xor(value, THRESHOLD, TOTAL_SHARES, mask_secret).unwrap();
    let share1: Vec<u8> = (&shares[0]).into();
    let share2: Vec<u8> = (&shares[1]).into();
    let encoded_share1 = encode_share_data(share1, mask_secret, user_id);
    let encoded_share2 = encode_share_data(share2, mask_secret, user_id);
    (hex::encode(encoded_share1), hex::encode(encoded_share2))
}

pub fn recover_value(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    let shares = recover_two_shares(value1, value2, mask_secret)?;
    let value = recover_secret_with_xor(&shares[..THRESHOLD], THRESHOLD, mask_secret)?;
    Ok(value)
}

pub fn recover_two_shares(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<Vec<Share>, SecretSharingError> {
    let encoded_value1: Vec<u8> =
        hex::decode(value1).map_err(|e| SecretSharingError::InvalidShare(e.to_string()))?;
    let encoded_value2: Vec<u8> =
        hex::decode(value2).map_err(|e| SecretSharingError::InvalidShare(e.to_string()))?;

    let decoded_value1 = decode_share_data(encoded_value1, mask_secret)?;
    let decoded_value2 = decode_share_data(encoded_value2, mask_secret)?;

    let share1: Share = decoded_value1.as_slice().try_into().map_err(|_| {
        SecretSharingError::InvalidShare("value1 convert to share failed".to_string())
    })?;
    let share2: Share = decoded_value2.as_slice().try_into().map_err(|_| {
        SecretSharingError::InvalidShare("value2 convert to share failed".to_string())
    })?;
    Ok(vec![share1, share2])
}

pub fn add_two_shared_secrets(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    if shares1.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(
            THRESHOLD as u8,
            shares1.len(),
        ));
    }
    if shares2.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(
            THRESHOLD as u8,
            shares2.len(),
        ));
    }
    add_shared_secrets(
        &shares1[..THRESHOLD],
        &shares2[..THRESHOLD],
        THRESHOLD,
        mask_secret,
        mask_secret,
    )
}

pub fn sub_two_shared_secrets(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    if shares1.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(
            THRESHOLD as u8,
            shares1.len(),
        ));
    }
    if shares2.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(
            THRESHOLD as u8,
            shares2.len(),
        ));
    }
    sub_shared_secrets(
        &shares1[..THRESHOLD],
        &shares2[..THRESHOLD],
        THRESHOLD,
        mask_secret,
        mask_secret,
    )
}

pub fn mul_two_shared_secrets(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    if shares1.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(
            THRESHOLD as u8,
            shares1.len(),
        ));
    }
    if shares2.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(
            THRESHOLD as u8,
            shares2.len(),
        ));
    }
    mul_shared_secrets(
        &shares1[..THRESHOLD],
        &shares2[..THRESHOLD],
        THRESHOLD,
        mask_secret,
        mask_secret,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::GF256;
    use crate::share::Share;

    // Helper function to create a valid share for testing
    fn create_test_share(x: u8, y_values: Vec<u8>) -> Share {
        Share {
            x: GF256(x),
            y: y_values.into_iter().map(GF256).collect(),
        }
    }

    #[test]
    fn test_encode_decode_share_data_roundtrip() {
        let share_data = vec![1u8, 2u8, 3u8, 4u8, 5u8];
        let mask_secret = 0x1234567890ABCDEFu64;
        let user_id = 1u64;

        let encoded = encode_share_data(share_data.clone(), mask_secret, user_id);
        let decoded = decode_share_data(encoded, mask_secret).unwrap();

        assert_eq!(decoded, share_data);
    }

    #[test]
    fn test_split_to_two_value_normal() {
        let value = 12345u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1, hex2) = split_to_two_value(value, user_id, mask_secret);

        // Verify that both hex strings are valid
        assert!(!hex1.is_empty());
        assert!(!hex2.is_empty());
        assert_ne!(hex1, hex2); // The two shares should be different

        // Verify that we can recover the original value
        let recovered = recover_value(hex1, hex2, mask_secret).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_split_to_two_value_zero() {
        let value = 0u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1, hex2) = split_to_two_value(value, user_id, mask_secret);

        // Verify that both hex strings are valid
        assert!(!hex1.is_empty());
        assert!(!hex2.is_empty());

        // Verify that we can recover the original value
        let recovered = recover_value(hex1, hex2, mask_secret).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_split_to_two_value_max() {
        let value = u64::MAX;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1, hex2) = split_to_two_value(value, user_id, mask_secret);

        // Verify that both hex strings are valid
        assert!(!hex1.is_empty());
        assert!(!hex2.is_empty());

        // Verify that we can recover the original value
        let recovered = recover_value(hex1, hex2, mask_secret).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_recover_value_normal() {
        let value = 12345u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1, hex2) = split_to_two_value(value, user_id, mask_secret);
        let recovered = recover_value(hex1, hex2, mask_secret).unwrap();

        assert_eq!(recovered, value);
    }

    #[test]
    fn test_recover_value_different_mask() {
        let value = 12345u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        let wrong_mask = 0xFEDCBA0987654321u64;

        let (hex1, hex2) = split_to_two_value(value, user_id, mask_secret);

        // Using wrong mask should fail or give wrong result
        let result = recover_value(hex1, hex2, wrong_mask);
        // This might succeed but give wrong result, or fail depending on implementation
        if let Ok(recovered) = result {
            assert_ne!(recovered, value);
        }
    }

    #[test]
    fn test_recover_value_invalid_hex() {
        let invalid_hex1 = "invalid_hex";
        let invalid_hex2 = "also_invalid";
        let mask_secret = 0x1234567890ABCDEFu64;

        let result = recover_value(
            invalid_hex1.to_string(),
            invalid_hex2.to_string(),
            mask_secret,
        );
        assert!(result.is_err());
        if let Err(SecretSharingError::InvalidShare(_)) = result {
            // Expected error type
        } else {
            panic!("Expected InvalidShare error");
        }
    }

    #[test]
    fn test_recover_value_empty_strings() {
        let empty1 = "";
        let empty2 = "";
        let mask_secret = 0x1234567890ABCDEFu64;

        let result = recover_value(empty1.to_string(), empty2.to_string(), mask_secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_recover_value_odd_length_hex() {
        let odd_hex1 = "123"; // Odd length hex
        let odd_hex2 = "456";
        let mask_secret = 0x1234567890ABCDEFu64;

        let result = recover_value(odd_hex1.to_string(), odd_hex2.to_string(), mask_secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_recover_two_shares_valid() {
        let value = 12345u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1, hex2) = split_to_two_value(value, user_id, mask_secret);
        let shares = recover_two_shares(hex1, hex2, mask_secret).unwrap();

        assert!(shares.len() == 2);
        // Verify that shares can be used to recover the original value
        let recovered =
            recover_secret_with_xor(&shares[..THRESHOLD], THRESHOLD, mask_secret).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_recover_two_shares_invalid_hex() {
        let invalid_hex1 = "not_hex";
        let invalid_hex2 = "also_not_hex";
        let mask_secret = 0x1234567890ABCDEFu64;

        let result = recover_two_shares(
            invalid_hex1.to_string(),
            invalid_hex2.to_string(),
            mask_secret,
        );
        assert!(result.is_err());
        if let Err(SecretSharingError::InvalidShare(_)) = result {
            // Expected error type
        } else {
            panic!("Expected InvalidShare error");
        }
    }

    #[test]
    fn test_recover_two_shares_invalid_share_format() {
        // Create hex strings that are valid hex but don't represent valid shares
        let invalid_hex1 = "00"; // Too short to be a valid share
        let invalid_hex2 = "01";
        let mask_secret = 0x1234567890ABCDEFu64;

        let result = recover_two_shares(
            invalid_hex1.to_string(),
            invalid_hex2.to_string(),
            mask_secret,
        );
        assert!(result.is_err());
        if let Err(SecretSharingError::InvalidShare(_)) = result {
            // Expected error type
        } else {
            panic!("Expected InvalidShare error");
        }
    }

    #[test]
    fn test_recover_two_shares_empty_strings() {
        let empty1 = "";
        let empty2 = "";
        let mask_secret = 0x1234567890ABCDEFu64;

        let result = recover_two_shares(empty1.to_string(), empty2.to_string(), mask_secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_two_shared_secrets_normal() {
        let value1 = 100u64;
        let value2 = 200u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = add_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, value1 + value2);
    }

    #[test]
    fn test_add_two_shared_secrets_zero_values() {
        let value1 = 0u64;
        let value2 = 0u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = add_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_add_two_shared_secrets_max_values() {
        let value1 = u64::MAX;
        let value2 = 1u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = add_two_shared_secrets(shares1, shares2, mask_secret);
        // This might overflow, which is expected behavior
        if let Ok(sum) = result {
            assert_eq!(sum, value1.wrapping_add(value2));
        }
    }

    #[test]
    fn test_sub_two_shared_secrets_normal() {
        let value1 = 300u64;
        let value2 = 100u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = sub_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, value1 - value2);
    }

    #[test]
    fn test_sub_two_shared_secrets_underflow() {
        let value1 = 100u64;
        let value2 = 300u64; // value2 > value1
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = sub_two_shared_secrets(shares1, shares2, mask_secret);
        // This should handle underflow gracefully (wrapping or error)
        if let Ok(diff) = result {
            assert_eq!(diff, value1.wrapping_sub(value2));
        }
    }

    #[test]
    fn test_mul_two_shared_secrets_normal() {
        let value1 = 15u64;
        let value2 = 20u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = mul_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, value1 * value2);
    }

    #[test]
    fn test_mul_two_shared_secrets_zero() {
        let value1 = 100u64;
        let value2 = 0u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = mul_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_two_shared_secrets_large_values() {
        let value1 = u64::MAX;
        let value2 = 2u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        let result = mul_two_shared_secrets(shares1, shares2, mask_secret);
        // This might overflow, which is expected behavior
        if let Ok(product) = result {
            assert_eq!(product, value1.wrapping_mul(value2));
        }
    }

    #[test]
    fn test_arithmetic_operations_consistency() {
        let value1 = 50u64;
        let value2 = 30u64;
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask_secret);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask_secret);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask_secret).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask_secret).unwrap();

        // Test that add and sub are inverse operations
        let add_result =
            add_two_shared_secrets(shares1.clone(), shares2.clone(), mask_secret).unwrap();
        let (add_hex1, add_hex2) = split_to_two_value(add_result, user_id, mask_secret);
        let add_result_shares = recover_two_shares(add_hex1, add_hex2, mask_secret).unwrap();
        let sub_result = sub_two_shared_secrets(add_result_shares, shares2, mask_secret).unwrap();
        assert_eq!(sub_result, value1);
    }

    #[test]
    fn test_split_to_two_value_boundary() -> Result<(), Box<dyn std::error::Error>> {
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        let resutl = &split_to_two_value(0, user_id, mask_secret);
        let shares = recover_two_shares(resutl.0.to_string(), resutl.1.to_string(), mask_secret)?;
        let value = recover_secret_with_xor(&shares[..THRESHOLD], THRESHOLD, mask_secret)?;
        assert_eq!(value, 0);

        let resutl = &split_to_two_value(u64::MAX, user_id, mask_secret);
        let shares = recover_two_shares(resutl.0.to_string(), resutl.1.to_string(), mask_secret)?;
        let value = recover_secret_with_xor(&shares[..THRESHOLD], THRESHOLD, mask_secret)?;
        assert_eq!(value, u64::MAX);
        Ok(())
    }

    #[test]
    fn test_invalid_share_vectors() {
        let user_id = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;

        // Test with insufficient shares (only one share instead of two)
        let single_share = vec![create_test_share(1, vec![1, 2, 3])];
        let (hex1, hex2) = split_to_two_value(100, user_id, mask_secret);
        let valid_shares = recover_two_shares(hex1, hex2, mask_secret).unwrap();

        // These should fail due to insufficient shares
        let add_result =
            add_two_shared_secrets(single_share.clone(), valid_shares.clone(), mask_secret);
        assert!(add_result.is_err());
        if let Err(SecretSharingError::InsufficientShares(required, provided)) = add_result {
            assert_eq!(required, 2);
            assert_eq!(provided, 1);
        } else {
            panic!("Expected InsufficientShares error");
        }

        let sub_result =
            sub_two_shared_secrets(single_share.clone(), valid_shares.clone(), mask_secret);
        assert!(sub_result.is_err());
        if let Err(SecretSharingError::InsufficientShares(required, provided)) = sub_result {
            assert_eq!(required, 2);
            assert_eq!(provided, 1);
        } else {
            panic!("Expected InsufficientShares error");
        }

        let mul_result = mul_two_shared_secrets(single_share, valid_shares, mask_secret);
        assert!(mul_result.is_err());
        if let Err(SecretSharingError::InsufficientShares(required, provided)) = mul_result {
            assert_eq!(required, 2);
            assert_eq!(provided, 1);
        } else {
            panic!("Expected InsufficientShares error");
        }
    }

    #[test]
    fn test_different_mask_secrets() {
        let value1 = 100u64;
        let value2 = 200u64;
        let user_id = 1u64;
        let mask1 = 0x1234567890ABCDEFu64;
        let mask2 = 0xFEDCBA0987654321u64;

        let (hex1_1, hex1_2) = split_to_two_value(value1, user_id, mask1);
        let (hex2_1, hex2_2) = split_to_two_value(value2, user_id, mask2);
        let shares1 = recover_two_shares(hex1_1, hex1_2, mask1).unwrap();
        let shares2 = recover_two_shares(hex2_1, hex2_2, mask2).unwrap();

        // Operations with different mask secrets might fail or give unexpected results
        let add_result = add_two_shared_secrets(shares1.clone(), shares2.clone(), mask1);
        let sub_result = sub_two_shared_secrets(shares1.clone(), shares2.clone(), mask1);
        let mul_result = mul_two_shared_secrets(shares1, shares2, mask1);

        // At least one of these should either fail or give a different result than expected
        let expected_sum = value1 + value2;
        let expected_diff = value1.wrapping_sub(value2);
        let expected_prod = value1 * value2;

        if let Ok(sum) = add_result {
            assert_ne!(sum, expected_sum);
        }
        if let Ok(diff) = sub_result {
            assert_ne!(diff, expected_diff);
        }
        if let Ok(prod) = mul_result {
            assert_ne!(prod, expected_prod);
        }
    }
}

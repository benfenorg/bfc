use crate::error::SecretSharingError;
use crate::math::{add_shared_secrets, mul_shared_secrets, sub_shared_secrets};
use crate::share::Share;
use crate::{generate_shares_with_xor, recover_secret_with_xor};

const THRESHOLD: usize = 2;
const TOTAL_SHARES: usize = 2;

/// Recover u64 secret
pub fn split_to_two_value(value: u64, mask_secret: u64) -> (String, String) {
    let shares = generate_shares_with_xor(value, THRESHOLD, TOTAL_SHARES, mask_secret).unwrap();
    let value1: Vec<u8> = (&shares[0]).into();
    let value2: Vec<u8> = (&shares[1]).into();
    let hex_value1 = hex::encode(value1);
    let hex_value2 = hex::encode(value2);
    (hex_value1, hex_value2)
}

pub fn recover_value(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    let shares = recover_two_shares(value1, value2)?;
    let value = recover_secret_with_xor(&shares[..THRESHOLD], THRESHOLD, mask_secret)?;
    Ok(value)
}

pub fn recover_two_shares(
    value1: String,
    value2: String,
) -> Result<Vec<Share>, SecretSharingError> {
    let value1: Vec<u8> =
        hex::decode(value1).map_err(|e| SecretSharingError::InvalidShare(e.to_string()))?;
    let value2: Vec<u8> =
        hex::decode(value2).map_err(|e| SecretSharingError::InvalidShare(e.to_string()))?;

    let share1: Share = value1.as_slice().try_into().map_err(|_| {
        SecretSharingError::InvalidShare("value1 convert to share failed".to_string())
    })?;
    let share2: Share = value2.as_slice().try_into().map_err(|_| {
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
        return Err(SecretSharingError::InsufficientShares(THRESHOLD as u8, shares1.len()));
    }
    if shares2.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(THRESHOLD as u8, shares2.len()));
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
        return Err(SecretSharingError::InsufficientShares(THRESHOLD as u8, shares1.len()));
    }
    if shares2.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(THRESHOLD as u8, shares2.len()));
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
        return Err(SecretSharingError::InsufficientShares(THRESHOLD as u8, shares1.len()));
    }
    if shares2.len() < THRESHOLD {
        return Err(SecretSharingError::InsufficientShares(THRESHOLD as u8, shares2.len()));
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
    use crate::share::Share;
    use crate::field::GF256;

    // Helper function to create a valid share for testing
    fn create_test_share(x: u8, y_values: Vec<u8>) -> Share {
        Share {
            x: GF256(x),
            y: y_values.into_iter().map(GF256).collect(),
        }
    }

    // Helper function to create shares from hex strings
    fn create_shares_from_hex(hex1: &str, hex2: &str) -> Vec<Share> {
        let value1: Vec<u8> = hex::decode(hex1).unwrap();
        let value2: Vec<u8> = hex::decode(hex2).unwrap();
        let share1: Share = value1.as_slice().try_into().unwrap();
        let share2: Share = value2.as_slice().try_into().unwrap();
        vec![share1, share2]
    }

    #[test]
    fn test_split_to_two_value_normal() {
        let value = 12345u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let (hex1, hex2) = split_to_two_value(value, mask_secret);
        
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
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let (hex1, hex2) = split_to_two_value(value, mask_secret);
        
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
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let (hex1, hex2) = split_to_two_value(value, mask_secret);
        
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
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let (hex1, hex2) = split_to_two_value(value, mask_secret);
        let recovered = recover_value(hex1, hex2, mask_secret).unwrap();
        
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_recover_value_different_mask() {
        let value = 12345u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        let wrong_mask = 0xFEDCBA0987654321u64;
        
        let (hex1, hex2) = split_to_two_value(value, mask_secret);
        
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
        
        let result = recover_value(invalid_hex1.to_string(), invalid_hex2.to_string(), mask_secret);
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
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let (hex1, hex2) = split_to_two_value(value, mask_secret);
        let shares = recover_two_shares(hex1, hex2).unwrap();
        
        assert!(shares.len() == 2);
        // Verify that shares can be used to recover the original value
        let recovered = recover_secret_with_xor(&shares[..THRESHOLD], THRESHOLD, mask_secret).unwrap();
        assert_eq!(recovered, value);
    }

    #[test]
    fn test_recover_two_shares_invalid_hex() {
        let invalid_hex1 = "not_hex";
        let invalid_hex2 = "also_not_hex";
        
        let result = recover_two_shares(invalid_hex1.to_string(), invalid_hex2.to_string());
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
        
        let result = recover_two_shares(invalid_hex1.to_string(), invalid_hex2.to_string());
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
        
        let result = recover_two_shares(empty1.to_string(), empty2.to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_add_two_shared_secrets_normal() {
        let value1 = 100u64;
        let value2 = 200u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
        let result = add_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, value1 + value2);
    }

    #[test]
    fn test_add_two_shared_secrets_zero_values() {
        let value1 = 0u64;
        let value2 = 0u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
        let result = add_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_add_two_shared_secrets_max_values() {
        let value1 = u64::MAX;
        let value2 = 1u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
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
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
        let result = sub_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, value1 - value2);
    }

    #[test]
    fn test_sub_two_shared_secrets_underflow() {
        let value1 = 100u64;
        let value2 = 300u64; // value2 > value1
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
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
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
        let result = mul_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, value1 * value2);
    }

    #[test]
    fn test_mul_two_shared_secrets_zero() {
        let value1 = 100u64;
        let value2 = 0u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
        let result = mul_two_shared_secrets(shares1, shares2, mask_secret).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_mul_two_shared_secrets_large_values() {
        let value1 = u64::MAX;
        let value2 = 2u64;
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
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
        let mask_secret = 0x1234567890ABCDEFu64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask_secret).0,
            &split_to_two_value(value1, mask_secret).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask_secret).0,
            &split_to_two_value(value2, mask_secret).1
        );
        
        // Test that add and sub are inverse operations
        let add_result = add_two_shared_secrets(shares1.clone(), shares2.clone(), mask_secret).unwrap();
        let sub_result = sub_two_shared_secrets(
            create_shares_from_hex(
                &split_to_two_value(add_result, mask_secret).0,
                &split_to_two_value(add_result, mask_secret).1
            ),
            shares2,
            mask_secret
        ).unwrap();
        assert_eq!(sub_result, value1);
    }

    #[test]
    fn test_invalid_share_vectors() {
        let mask_secret = 0x1234567890ABCDEFu64;
        
        // Test with insufficient shares (only one share instead of two)
        let single_share = vec![create_test_share(1, vec![1, 2, 3])];
        let valid_shares = create_shares_from_hex(
            &split_to_two_value(100, mask_secret).0,
            &split_to_two_value(100, mask_secret).1
        );
        
        // These should fail due to insufficient shares
        let add_result = add_two_shared_secrets(single_share.clone(), valid_shares.clone(), mask_secret);
        assert!(add_result.is_err());
        if let Err(SecretSharingError::InsufficientShares(required, provided)) = add_result {
            assert_eq!(required, 2);
            assert_eq!(provided, 1);
        } else {
            panic!("Expected InsufficientShares error");
        }
        
        let sub_result = sub_two_shared_secrets(single_share.clone(), valid_shares.clone(), mask_secret);
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
        let mask1 = 0x1234567890ABCDEFu64;
        let mask2 = 0xFEDCBA0987654321u64;
        
        let shares1 = create_shares_from_hex(
            &split_to_two_value(value1, mask1).0,
            &split_to_two_value(value1, mask1).1
        );
        let shares2 = create_shares_from_hex(
            &split_to_two_value(value2, mask2).0,
            &split_to_two_value(value2, mask2).1
        );
        
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

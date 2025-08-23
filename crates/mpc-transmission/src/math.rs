#[allow(unused_imports)]
use crate::{generate_shares_with_xor, recover_secret_with_xor, SecretSharingError, Share};

// Using approach one: first recover the original secret during computation, then perform calculation, output u64 calculation result
//
// Addition operation
pub fn add_shared_secrets(
    shares1: &[Share],
    shares2: &[Share],
    threshold: usize, // New parameter
    mask1: u64,
    mask2: u64,
) -> Result<u64, SecretSharingError> {
    let secret1 = recover_secret_with_xor(shares1, threshold, mask1)?;
    let secret2 = recover_secret_with_xor(shares2, threshold, mask2)?;
    secret1.checked_add(secret2)
        .ok_or_else(|| SecretSharingError::ArithmeticOverflow { 
            operation: "addition".to_string() 
        })
}

/// Subtraction operation
pub fn sub_shared_secrets(
    shares1: &[Share],
    shares2: &[Share],
    threshold: usize,
    mask1: u64,
    mask2: u64,
) -> Result<u64, SecretSharingError> {
    let secret1 = recover_secret_with_xor(shares1, threshold, mask1)?;
    let secret2 = recover_secret_with_xor(shares2, threshold, mask2)?;
    secret1.checked_sub(secret2)
        .ok_or_else(|| SecretSharingError::ArithmeticOverflow { 
            operation: "subtraction".to_string() 
        })
}

/// Multiplication operation
pub fn mul_shared_secrets(
    shares1: &[Share],
    shares2: &[Share],
    threshold: usize,
    mask1: u64,
    mask2: u64,
) -> Result<u64, SecretSharingError> {
    let secret1 = recover_secret_with_xor(shares1, threshold, mask1)?;
    let secret2 = recover_secret_with_xor(shares2, threshold, mask2)?;
    secret1.checked_mul(secret2)
        .ok_or_else(|| SecretSharingError::ArithmeticOverflow { 
            operation: "multiplication".to_string() 
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Fixed mask for testing (ensure all test cases use the same mask)
    const TEST_MASK: u64 = 0x12345678ABCDEF00;

    // Generate test data: shares of two secrets + threshold
    fn setup_test_secrets() -> (Vec<Share>, Vec<Share>, usize) {
        let threshold = 3;
        let total_shares = 5;
        let secret1 = 12345;
        let secret2 = 67890;

        let shares1 = generate_shares_with_xor(secret1, threshold, total_shares, TEST_MASK)
            .expect("Failed to generate shares1");
        let shares2 = generate_shares_with_xor(secret2, threshold, total_shares, TEST_MASK)
            .expect("Failed to generate shares2");

        (shares1, shares2, threshold)
    }

    // Test addition: secret1 + secret2
    #[test]
    fn test_addition() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        let result = add_shared_secrets(
            &shares1[..threshold], // Take first threshold shares
            &shares2[..threshold],
            threshold,
            TEST_MASK, // mask for shares1
            TEST_MASK, // mask for shares2
        )
        .unwrap();

        assert_eq!(result, 12345 + 67890);
    }

    // Test subtraction: secret1 - secret2 (should fail due to underflow)
    #[test]
    fn test_subtraction() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        let result = sub_shared_secrets(
            &shares1[..threshold],
            &shares2[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        );

        // 12345 - 67890 should underflow, so this should return an error
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SecretSharingError::ArithmeticOverflow { operation } if operation == "subtraction"));
    }

    // Test subtraction that doesn't overflow
    #[test]
    fn test_subtraction_no_overflow() {
        let threshold = 2;
        let total_shares = 3;
        let secret_large = 1000;
        let secret_small = 500;

        let shares_large = generate_shares_with_xor(secret_large, threshold, total_shares, TEST_MASK).unwrap();
        let shares_small = generate_shares_with_xor(secret_small, threshold, total_shares, TEST_MASK).unwrap();

        let result = sub_shared_secrets(
            &shares_large[..threshold],
            &shares_small[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();

        assert_eq!(result, 1000 - 500);
    }

    // Test multiplication: secret1 * secret2
    #[test]
    fn test_multiplication() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        let result = mul_shared_secrets(
            &shares1[..threshold],
            &shares2[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();

        assert_eq!(result, 12345 * 67890);
    }

    // Test edge cases (zero, maximum values, etc.)
    #[test]
    fn test_edge_cases() {
        let threshold = 2;
        let total_shares = 3;
        let secret_zero = 0;
        let secret_one = 1;
        let secret_max = u64::MAX;

        // Generate shares
        let shares_zero =
            generate_shares_with_xor(secret_zero, threshold, total_shares, TEST_MASK).unwrap();
        let shares_one =
            generate_shares_with_xor(secret_one, threshold, total_shares, TEST_MASK).unwrap();
        let shares_max =
            generate_shares_with_xor(secret_max, threshold, total_shares, TEST_MASK).unwrap();

        // 0 * MAX = 0 (no overflow)
        let mul_result = mul_shared_secrets(
            &shares_zero[..threshold],
            &shares_max[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();
        assert_eq!(mul_result, 0);

        // 1 - MAX should overflow (subtraction underflow)
        let sub_result = sub_shared_secrets(
            &shares_one[..threshold],
            &shares_max[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        );
        assert!(sub_result.is_err());
        assert!(matches!(sub_result.unwrap_err(), SecretSharingError::ArithmeticOverflow { operation } if operation == "subtraction"));

        // MAX + 1 should overflow (addition overflow)
        let add_result = add_shared_secrets(
            &shares_max[..threshold],
            &shares_one[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        );
        assert!(add_result.is_err());
        assert!(matches!(add_result.unwrap_err(), SecretSharingError::ArithmeticOverflow { operation } if operation == "addition"));
    }

    // Test multiplication overflow
    #[test]
    fn test_multiplication_overflow() {
        let threshold = 2;
        let total_shares = 3;
        let secret_large = u64::MAX / 2 + 1; // A large number that will cause overflow when multiplied by 2

        let shares_large = generate_shares_with_xor(secret_large, threshold, total_shares, TEST_MASK).unwrap();
        let shares_two = generate_shares_with_xor(2, threshold, total_shares, TEST_MASK).unwrap();

        // This should overflow
        let mul_result = mul_shared_secrets(
            &shares_large[..threshold],
            &shares_two[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        );
        assert!(mul_result.is_err());
        assert!(matches!(mul_result.unwrap_err(), SecretSharingError::ArithmeticOverflow { operation } if operation == "multiplication"));
    }

    // Test error when insufficient shares
    #[test]
    fn test_insufficient_shares() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        // Provided shares < threshold
        assert!(add_shared_secrets(
            &shares1[..threshold - 1], // One share less
            &shares2[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .is_err());

        assert!(sub_shared_secrets(
            &shares1[..threshold],
            &shares2[..threshold - 1],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .is_err());
    }
}

//! Share Converter Module
//!
//! Provides conversion functionality between mpc-transmission share format and mpc-transmission-v2 share format.
//!
//! # Important Notes
//! - The conversion process requires complete recovery of the original secret value, which temporarily exposes the secret
//! - The finite field modulus of mpc-transmission-v2 is 18446744069414584321
//! - If the mpc-transmission value >= modulus, conversion will fail
//!
//! # Usage
//! ```ignore
//! use mpc_framework_core::share_converter::*;
//!
//! // Convert from mpc-transmission format (version must be 1)
//! let (core_hex1, core_hex2, seed) = convert_from_transmission(
//!     transmission_hex1,
//!     transmission_hex2,
//!     mask_secret,
//!     user_id,
//!     coord_seed,
//!     1, // version: only version 1 is supported
//! )?;
//! ```

use crate::error::SSSError;
use crate::two_party_share::split_to_two_value;

/// mpc-transmission-v2 finite field modulus
pub const FIELD_MODULUS: u64 = 18446744069414584321;

// ============================================================================
// Public API
// ============================================================================

/// Recover the original secret value from mpc-transmission format shares
///
/// # Arguments
/// * `transmission_hex1` - First mpc-transmission format hex share
/// * `transmission_hex2` - Second mpc-transmission format hex share
/// * `mask_secret` - Mask secret key
///
/// # Returns
/// * `Ok(u64)` - Recovered original secret value
/// * `Err(SSSError)` - If decoding or recovery fails
pub fn recover_from_transmission_shares(
    transmission_hex1: &str,
    transmission_hex2: &str,
    mask_secret: u64,
) -> Result<u64, SSSError> {
    Ok(mpc_transmission::two_party_share::recover_value(
        transmission_hex1.to_string(),
        transmission_hex2.to_string(),
        mask_secret,
    )?)
}

/// Check if the given shares are in valid mpc-transmission format
///
/// This function validates whether two hex-encoded shares and a mask secret can be used
/// to successfully recover a secret value using the mpc-transmission format.
///
/// # Arguments
/// * `transmission_hex1` - First mpc-transmission format hex share
/// * `transmission_hex2` - Second mpc-transmission format hex share
/// * `mask_secret` - Mask secret key used for encoding
///
/// # Returns
/// * `true` - If the shares are valid and can successfully recover a value
/// * `false` - If the shares are invalid, malformed, or cannot recover a value
///
/// # Examples
/// ```ignore
/// use mpc_framework_core::share_converter::*;
///
/// // Valid shares
/// let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
///     12345, user_id, mask_secret
/// );
/// assert!(is_transmission_shares_format(&hex1, &hex2, mask_secret));
///
/// // Invalid hex format
/// assert!(!is_transmission_shares_format("invalid", "hex", mask_secret));
///
/// // Wrong mask secret
/// assert!(!is_transmission_shares_format(&hex1, &hex2, wrong_mask));
/// ```
pub fn is_transmission_shares_format(
    transmission_hex1: &str,
    transmission_hex2: &str,
    mask_secret: u64,
) -> bool {
    if recover_from_transmission_shares(transmission_hex1, transmission_hex2, mask_secret).is_err()
    {
        false
    } else {
        true
    }
}

/// Convert shares from mpc-transmission format to mpc-transmission-v2 format
///
/// # Arguments
/// * `transmission_hex1` - First mpc-transmission format hex share
/// * `transmission_hex2` - Second mpc-transmission format hex share
/// * `mask_secret` - Mask secret key
/// * `user_id` - User ID (for mpc-transmission-v2 encoding)
/// * `coord_seed` - Coordinate seed (for mpc-transmission-v2)
/// * `version` - Version number, only version 1 is supported
///
/// # Returns
/// * `Ok((String, String, u64))` - mpc-transmission-v2 format (hex1, hex2, seed)
/// * `Err(SSSError)` - If conversion fails
///
/// # Errors
/// - Returns error if version number is not 1
/// - Returns error if mpc-transmission value >= FIELD_MODULUS
pub fn convert_from_transmission_shares(
    transmission_hex1: &str,
    transmission_hex2: &str,
    mask_secret: u64,
    user_id: u64,
    coord_seed: u64,
    version: u8,
) -> Result<(String, String, u64), SSSError> {
    // Step 0: Check version number, only convert version 1
    if version != 1 {
        return Err(SSSError::InvalidParameters(format!(
            "Unsupported version: {}. Only version 1 is supported for conversion.",
            version
        )));
    }

    // Step 1: Recover original value from mpc-transmission format
    let value =
        recover_from_transmission_shares(transmission_hex1, transmission_hex2, mask_secret)?;

    // Step 2: Check if value is within mpc-transmission-v2 finite field range
    if value >= FIELD_MODULUS {
        return Err(SSSError::InvalidParameters(format!(
            "Value {} exceeds field modulus {}. Cannot convert to mpc-transmission-v2 format.",
            value, FIELD_MODULUS
        )));
    }

    // Step 3: Re-split using mpc-transmission-v2
    let (hex1, hex2, seed) = split_to_two_value(value, user_id, mask_secret, coord_seed);

    Ok((hex1, hex2, seed))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MASK_SECRET: u64 = 0x1234567890ABCDEFu64;
    const TEST_USER_ID: u64 = 1u64;
    const TEST_COORD_SEED: u64 = 0xABCDEF1234567890u64;

    #[test]
    fn test_convert_from_transmission_shares() {
        let original_value = 12345;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let (core_hex1, core_hex2, _) = convert_from_transmission_shares(
            &hex1,
            &hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        )
        .unwrap();
        let value = crate::two_party_share::recover_value(
            core_hex1.clone(),
            core_hex2.clone(),
            TEST_MASK_SECRET,
        )
        .unwrap();
        assert_eq!(value, 12345);
    }

    #[test]
    fn test_convert_zero_value() {
        let original_value = 0;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
            &hex1,
            &hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        )
        .unwrap();
        let value = crate::two_party_share::recover_value(
            core_hex1.clone(),
            core_hex2.clone(),
            TEST_MASK_SECRET,
        )
        .unwrap();
        assert_eq!(value, 0);
        assert_eq!(seed, TEST_COORD_SEED);
    }

    #[test]
    fn test_convert_max_valid_value() {
        // FIELD_MODULUS - 1 is the maximum valid value
        let original_value = FIELD_MODULUS - 1;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
            &hex1,
            &hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        )
        .unwrap();
        let value = crate::two_party_share::recover_value(
            core_hex1.clone(),
            core_hex2.clone(),
            TEST_MASK_SECRET,
        )
        .unwrap();
        assert_eq!(value, original_value);
        assert_eq!(seed, TEST_COORD_SEED);
    }

    #[test]
    fn test_convert_large_value() {
        let original_value = 18446744069414584320; // Very close to FIELD_MODULUS
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
            &hex1,
            &hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        )
        .unwrap();
        let value = crate::two_party_share::recover_value(
            core_hex1.clone(),
            core_hex2.clone(),
            TEST_MASK_SECRET,
        )
        .unwrap();
        assert_eq!(value, original_value);
        assert_eq!(seed, TEST_COORD_SEED);
    }

    #[test]
    fn test_convert_value_exceeds_modulus() {
        // This value exceeds FIELD_MODULUS, should fail
        let original_value = FIELD_MODULUS;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let result = convert_from_transmission_shares(
            &hex1,
            &hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        );
        assert!(result.is_err());
        if let Err(SSSError::InvalidParameters(msg)) = result {
            assert!(msg.contains("exceeds field modulus"));
        } else {
            panic!("Expected InvalidParameters error");
        }
    }

    #[test]
    fn test_convert_invalid_version() {
        let original_value = 12345;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );

        // Test version 0
        let result = convert_from_transmission_shares(
            &hex1,
            &hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            0,
        );
        assert!(result.is_err());
        if let Err(SSSError::InvalidParameters(msg)) = result {
            assert!(msg.contains("Unsupported version"));
        } else {
            panic!("Expected InvalidParameters error");
        }

        // Test version 2
        let result = convert_from_transmission_shares(
            &hex1,
            &hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            2,
        );
        assert!(result.is_err());
        if let Err(SSSError::InvalidParameters(msg)) = result {
            assert!(msg.contains("Unsupported version"));
        } else {
            panic!("Expected InvalidParameters error");
        }
    }

    #[test]
    fn test_convert_different_user_ids() {
        let original_value = 99999;
        let user_ids = [1u64, 2u64, 100u64, u64::MAX];

        for &user_id in &user_ids {
            let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
                original_value,
                user_id,
                TEST_MASK_SECRET,
            );
            let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
                &hex1,
                &hex2,
                TEST_MASK_SECRET,
                user_id,
                TEST_COORD_SEED,
                1,
            )
            .unwrap();
            let value = crate::two_party_share::recover_value(
                core_hex1.clone(),
                core_hex2.clone(),
                TEST_MASK_SECRET,
            )
            .unwrap();
            assert_eq!(value, original_value, "Failed for user_id: {}", user_id);
            assert_eq!(seed, TEST_COORD_SEED);
        }
    }

    #[test]
    fn test_convert_different_coord_seeds() {
        let original_value = 54321;
        let coord_seeds = [0u64, 1u64, 0xABCDEF1234567890u64, u64::MAX];

        for &coord_seed in &coord_seeds {
            let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
                original_value,
                TEST_USER_ID,
                TEST_MASK_SECRET,
            );
            let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
                &hex1,
                &hex2,
                TEST_MASK_SECRET,
                TEST_USER_ID,
                coord_seed,
                1,
            )
            .unwrap();
            let value = crate::two_party_share::recover_value(
                core_hex1.clone(),
                core_hex2.clone(),
                TEST_MASK_SECRET,
            )
            .unwrap();
            assert_eq!(
                value, original_value,
                "Failed for coord_seed: {}",
                coord_seed
            );
            assert_eq!(seed, coord_seed);
        }
    }

    #[test]
    fn test_convert_different_mask_secrets() {
        let original_value = 77777;
        let mask_secrets = [0u64, 1u64, 0x1234567890ABCDEFu64, u64::MAX];

        for &mask_secret in &mask_secrets {
            let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
                original_value,
                TEST_USER_ID,
                mask_secret,
            );
            let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
                &hex1,
                &hex2,
                mask_secret,
                TEST_USER_ID,
                TEST_COORD_SEED,
                1,
            )
            .unwrap();
            let value = crate::two_party_share::recover_value(
                core_hex1.clone(),
                core_hex2.clone(),
                mask_secret,
            )
            .unwrap();
            assert_eq!(
                value, original_value,
                "Failed for mask_secret: {}",
                mask_secret
            );
            assert_eq!(seed, TEST_COORD_SEED);
        }
    }

    #[test]
    fn test_recover_from_transmission_shares() {
        let original_value = 123456;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let recovered = recover_from_transmission_shares(&hex1, &hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered, original_value);
    }

    #[test]
    fn test_recover_from_transmission_shares_zero() {
        let original_value = 0;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let recovered = recover_from_transmission_shares(&hex1, &hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered, 0);
    }

    #[test]
    fn test_recover_from_transmission_shares_large_value() {
        let original_value = u64::MAX;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let recovered = recover_from_transmission_shares(&hex1, &hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered, original_value);
    }

    #[test]
    fn test_recover_from_transmission_shares_wrong_mask() {
        let original_value = 12345;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let wrong_mask = TEST_MASK_SECRET.wrapping_add(1);
        let result = recover_from_transmission_shares(&hex1, &hex2, wrong_mask);
        // Should fail or return wrong value - depends on implementation
        assert!(result.is_err() || result.unwrap() != original_value);
    }

    #[test]
    fn test_convert_multiple_values() {
        let test_values = vec![
            1,
            42,
            100,
            1000,
            10000,
            100000,
            1000000,
            10000000,
            100000000,
            1000000000,
            10000000000,
            100000000000,
            1000000000000,
            10000000000000,
            100000000000000,
            1000000000000000,
            10000000000000000,
            100000000000000000,
        ];

        for original_value in test_values {
            if original_value >= FIELD_MODULUS {
                continue; // Skip values that exceed modulus
            }

            let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
                original_value,
                TEST_USER_ID,
                TEST_MASK_SECRET,
            );
            let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
                &hex1,
                &hex2,
                TEST_MASK_SECRET,
                TEST_USER_ID,
                TEST_COORD_SEED,
                1,
            )
            .unwrap();
            let value = crate::two_party_share::recover_value(
                core_hex1.clone(),
                core_hex2.clone(),
                TEST_MASK_SECRET,
            )
            .unwrap();
            assert_eq!(
                value, original_value,
                "Failed for value: {}",
                original_value
            );
            assert_eq!(seed, TEST_COORD_SEED);
        }
    }

    #[test]
    fn test_convert_round_trip_consistency() {
        // Test that converting and recovering multiple times gives consistent results
        let original_value = 88888;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );

        // Convert multiple times with same parameters
        for _ in 0..5 {
            let (core_hex1, core_hex2, seed) = convert_from_transmission_shares(
                &hex1,
                &hex2,
                TEST_MASK_SECRET,
                TEST_USER_ID,
                TEST_COORD_SEED,
                1,
            )
            .unwrap();
            let value = crate::two_party_share::recover_value(
                core_hex1.clone(),
                core_hex2.clone(),
                TEST_MASK_SECRET,
            )
            .unwrap();
            assert_eq!(value, original_value);
            assert_eq!(seed, TEST_COORD_SEED);
        }
    }

    #[test]
    fn test_convert_invalid_hex_format() {
        // Test with invalid hex strings
        let invalid_hex1 = "not_a_hex_string";
        let invalid_hex2 = "also_not_hex";

        let result = convert_from_transmission_shares(
            invalid_hex1,
            invalid_hex2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_empty_hex_strings() {
        // Test with empty hex strings
        let empty1 = "";
        let empty2 = "";

        let result = convert_from_transmission_shares(
            empty1,
            empty2,
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_mismatched_shares() {
        // Test with shares from different secrets
        let value1 = 11111;
        let value2 = 22222;
        let (hex1_1, _hex2_1) = mpc_transmission::two_party_share::split_to_two_value(
            value1,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let (hex1_2, _hex2_2) = mpc_transmission::two_party_share::split_to_two_value(
            value2,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );

        // Try to convert with mismatched shares
        let result = convert_from_transmission_shares(
            &hex1_1,
            &hex1_2, // Wrong share - from different secret
            TEST_MASK_SECRET,
            TEST_USER_ID,
            TEST_COORD_SEED,
            1,
        );
        // Should either fail or produce wrong value
        if let Ok((core_hex1, core_hex2, _)) = result {
            let recovered =
                crate::two_party_share::recover_value(core_hex1, core_hex2, TEST_MASK_SECRET)
                    .unwrap();
            assert_ne!(recovered, value1);
            assert_ne!(recovered, value2);
        } else {
            // Error is also acceptable
            assert!(result.is_err());
        }
    }

    // ============================================================================
    // Tests for is_transmission_shares_format
    // ============================================================================

    #[test]
    fn test_is_transmission_shares_format_valid() {
        let original_value = 12345;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        assert!(is_transmission_shares_format(
            &hex1,
            &hex2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_core_share_is_transmission_shares_format() {
        let original_value = 12345;
        let (hex1, hex2, _) = split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
            TEST_COORD_SEED,
        );
        assert!(!is_transmission_shares_format(
            &hex1,
            &hex2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_is_transmission_shares_format_zero_value() {
        let original_value = 0;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        assert!(is_transmission_shares_format(
            &hex1,
            &hex2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_is_transmission_shares_format_large_value() {
        let original_value = u64::MAX;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        assert!(is_transmission_shares_format(
            &hex1,
            &hex2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_is_transmission_shares_format_wrong_mask_secret() {
        let original_value = 12345;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let wrong_mask = TEST_MASK_SECRET.wrapping_add(1);
        // Note: is_transmission_shares_format checks if format is valid (can decode),
        // not if the recovered value is correct. Wrong mask may still decode successfully
        // but produce incorrect value. The function behavior depends on implementation.
        let result = recover_from_transmission_shares(&hex1, &hex2, wrong_mask);
        if result.is_err() {
            assert!(!is_transmission_shares_format(&hex1, &hex2, wrong_mask));
        } else {
            // If it can recover (even with wrong value), format is considered valid
            assert!(is_transmission_shares_format(&hex1, &hex2, wrong_mask));
            assert_ne!(result.unwrap(), original_value);
        }
    }

    #[test]
    fn test_is_transmission_shares_format_invalid_hex() {
        let invalid_hex1 = "not_a_valid_hex_string";
        let invalid_hex2 = "also_not_valid";
        assert!(!is_transmission_shares_format(
            invalid_hex1,
            invalid_hex2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_is_transmission_shares_format_empty_strings() {
        let empty1 = "";
        let empty2 = "";
        assert!(!is_transmission_shares_format(
            empty1,
            empty2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_is_transmission_shares_format_partial_empty() {
        let original_value = 12345;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        // First share empty
        assert!(!is_transmission_shares_format("", &hex2, TEST_MASK_SECRET));
        // Second share empty
        assert!(!is_transmission_shares_format(&hex1, "", TEST_MASK_SECRET));
    }

    #[test]
    fn test_is_transmission_shares_format_mismatched_shares() {
        let value1 = 11111;
        let value2 = 22222;
        let (hex1_1, _hex2_1) = mpc_transmission::two_party_share::split_to_two_value(
            value1,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        let (hex1_2, _hex2_2) = mpc_transmission::two_party_share::split_to_two_value(
            value2,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );
        // Mismatched shares from different secrets
        // Note: is_transmission_shares_format checks format validity, not value correctness
        let result = recover_from_transmission_shares(&hex1_1, &hex1_2, TEST_MASK_SECRET);
        if result.is_err() {
            assert!(!is_transmission_shares_format(
                &hex1_1,
                &hex1_2,
                TEST_MASK_SECRET
            ));
        } else {
            // If it can recover (even with wrong value), format is considered valid
            assert!(is_transmission_shares_format(
                &hex1_1,
                &hex1_2,
                TEST_MASK_SECRET
            ));
            let recovered = result.unwrap();
            assert_ne!(recovered, value1);
            assert_ne!(recovered, value2);
        }
    }

    #[test]
    fn test_is_transmission_shares_format_odd_length_hex() {
        let odd_hex1 = "123"; // Odd length hex
        let odd_hex2 = "abc";
        assert!(!is_transmission_shares_format(
            odd_hex1,
            odd_hex2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_is_transmission_shares_format_invalid_hex_characters() {
        let invalid_hex1 = "GHIJKL"; // Contains invalid hex characters
        let invalid_hex2 = "MNOPQR";
        assert!(!is_transmission_shares_format(
            invalid_hex1,
            invalid_hex2,
            TEST_MASK_SECRET
        ));
    }

    #[test]
    fn test_is_transmission_shares_format_multiple_valid_values() {
        let test_values = vec![1, 42, 100, 1000, 10000, 999999, u64::MAX];
        for original_value in test_values {
            let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
                original_value,
                TEST_USER_ID,
                TEST_MASK_SECRET,
            );
            assert!(
                is_transmission_shares_format(&hex1, &hex2, TEST_MASK_SECRET),
                "Failed for value: {}",
                original_value
            );
        }
    }

    #[test]
    fn test_is_transmission_shares_format_different_mask_secrets() {
        let original_value = 77777;
        let mask_secrets = [0u64, 1u64, 0x1234567890ABCDEFu64, u64::MAX];
        for &mask_secret in &mask_secrets {
            let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
                original_value,
                TEST_USER_ID,
                mask_secret,
            );
            assert!(
                is_transmission_shares_format(&hex1, &hex2, mask_secret),
                "Failed for mask_secret: {}",
                mask_secret
            );
            // Wrong mask: check if it can still decode (format may still be valid)
            let wrong_mask = mask_secret.wrapping_add(1);
            let result = recover_from_transmission_shares(&hex1, &hex2, wrong_mask);
            let format_valid = is_transmission_shares_format(&hex1, &hex2, wrong_mask);
            if result.is_err() {
                assert!(
                    !format_valid,
                    "Should fail for wrong mask_secret: {}",
                    wrong_mask
                );
            } else {
                // If it can recover, format is valid (even if value is wrong)
                assert!(
                    format_valid,
                    "Format should be valid for wrong mask_secret: {}",
                    wrong_mask
                );
                assert_ne!(result.unwrap(), original_value);
            }
        }
    }

    #[test]
    fn test_is_transmission_shares_format_consistency_with_recover() {
        // Test that is_transmission_shares_format returns true exactly when recover succeeds
        let original_value = 88888;
        let (hex1, hex2) = mpc_transmission::two_party_share::split_to_two_value(
            original_value,
            TEST_USER_ID,
            TEST_MASK_SECRET,
        );

        // Valid case: should both succeed
        assert!(is_transmission_shares_format(
            &hex1,
            &hex2,
            TEST_MASK_SECRET
        ));
        let recovered = recover_from_transmission_shares(&hex1, &hex2, TEST_MASK_SECRET).unwrap();
        assert_eq!(recovered, original_value);

        // Invalid case: wrong mask
        // Note: is_transmission_shares_format returns true if recover succeeds,
        // regardless of whether the recovered value is correct
        let wrong_mask = TEST_MASK_SECRET.wrapping_add(1);
        let recover_result = recover_from_transmission_shares(&hex1, &hex2, wrong_mask);
        let format_valid = is_transmission_shares_format(&hex1, &hex2, wrong_mask);

        // Consistency check: format is valid if and only if recover succeeds
        assert_eq!(format_valid, recover_result.is_ok());

        if recover_result.is_ok() {
            // If recover succeeds, value may be wrong but format is valid
            assert_ne!(recover_result.unwrap(), original_value);
        }
    }
}

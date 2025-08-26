#[allow(unused_imports)]
// Standard library imports
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::path::PathBuf;
use sui_config::anonymous_privatekey_config::AnonymousPrivateKeyConfig;
use anyhow::anyhow;

// External crate imports
#[allow(unused_imports)]
use rand::random;

// Re-export sharks crate
use crate::error::SecretSharingError;
pub use field::GF256;
pub use share::Share;

// Re-export two_party_share functions
pub use two_party_share::{recover_two_shares, split_to_two_value, recover_value};

// Local module declarations
pub mod error;
mod field;
pub mod math;
pub mod poly;
pub mod read;
pub mod share;
pub mod two_party_share;

/// Generate secret shares
///
/// # Parameters
/// - `secret`: Secret data to protect (byte slice)
/// - `threshold`: Minimum number of shares required to recover the secret (must be ≥ 2 and ≤ 255)
/// - `total_shares`: Total number of shares to generate (must be ≥ threshold)
///
/// # Returns
/// - Success: Vec<Share> containing the shares
/// - Failure: SecretSharingError
pub fn generate_shares(
    secret: &[u8],
    threshold: usize,
    total_shares: usize,
) -> Result<Vec<Share>, SecretSharingError> {
    // Validate input parameters
    if secret.is_empty() {
        return Err(SecretSharingError::EmptySecret);
    }
    if threshold < 2 {
        return Err(SecretSharingError::InvalidThreshold(threshold)); // Provide parameter
    }
    if threshold > 255 {
        return Err(SecretSharingError::ThresholdTooLarge(threshold)); // Provide parameter
    }
    if total_shares < threshold {
        return Err(SecretSharingError::InsufficientShares(
            threshold as u8,
            total_shares,
        ));
    }

    let polys: Vec<Vec<GF256>> = secret
        .iter()
        .map(|&byte| poly::random_polynomial(GF256(byte), threshold as u8))
        .collect();

    let shares: Vec<Share> = poly::get_evaluator(polys).take(total_shares).collect();

    Ok(shares)
}

/// Recover secret from shares
///
/// # Parameters
/// - `shares`: Collection of shares (must contain at least threshold number of shares)
/// - `threshold`: Originally set threshold (must be ≥ 2 and ≤ 255)
///
/// # Returns
/// - Success: Recovered secret data Vec<u8>
/// - Failure: SecretSharingError
pub fn recover_secret(shares: &[Share], threshold: usize) -> Result<Vec<u8>, SecretSharingError> {
    // Parameter validation
    if threshold < 2 {
        return Err(SecretSharingError::InvalidThreshold(threshold)); // Provide parameter
    }
    if threshold > 255 {
        return Err(SecretSharingError::ThresholdTooLarge(threshold)); // Provide parameter
    }

    // Check number of shares
    if shares.len() < threshold {
        return Err(SecretSharingError::InsufficientShares(
            threshold as u8,
            shares.len(),
        ));
    }

    let recovered_bytes = poly::interpolate(shares);
    Ok(recovered_bytes)
}

pub fn u64_to_bytes(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

pub fn bytes_to_u64(bytes: &[u8]) -> Result<u64, &'static str> {
    bytes
        .try_into()
        .map(u64::from_be_bytes)
        .map_err(|_| "Input slice must be exactly 8 bytes")
}

/// Split u64 secret
///
pub fn generate_shares_u64(
    secret: u64,
    threshold: usize,
    total_shares: usize,
) -> Result<Vec<Share>, SecretSharingError> {
    // Parameter validation
    if threshold < 2 {
        return Err(SecretSharingError::InvalidThreshold(threshold));
    }
    if threshold > 255 {
        return Err(SecretSharingError::ThresholdTooLarge(threshold));
    }
    if total_shares < threshold {
        return Err(SecretSharingError::InsufficientShares(
            threshold as u8,
            total_shares,
        ));
    }

    let secret_bytes = u64_to_bytes(secret);
    let shares = generate_shares(&secret_bytes, threshold, total_shares)?;

    Ok(shares)
}

/// Recover u64 secret
pub fn recover_secret_u64(shares: &[Share], threshold: usize) -> Result<u64, SecretSharingError> {
    // Parameter validation
    if threshold < 2 {
        return Err(SecretSharingError::InvalidThreshold(threshold));
    }
    if threshold > 255 {
        return Err(SecretSharingError::ThresholdTooLarge(threshold));
    }
    if shares.len() < threshold {
        return Err(SecretSharingError::InsufficientShares(
            threshold as u8,
            shares.len(),
        ));
    }

    let bytes = recover_secret(shares, threshold)?;

    bytes_to_u64(&bytes).map_err(|e| SecretSharingError::RecoveryFailed(e.to_string()))
}

/// Shamir secret sharing for U64 with XOR obfuscation
///
/// Returns: (share collection, XOR mask)
pub fn generate_shares_with_xor(
    secret: u64,
    threshold: usize,
    total_shares: usize,
    mask: u64,
) -> Result<Vec<Share>, SecretSharingError> {
    // 1. Apply XOR obfuscation
    let masked_secret = secret ^ mask;

    // 2. Generate Shamir shares (return result directly)
    generate_shares_u64(masked_secret, threshold, total_shares)
}

/// Recover secret u64 with XOR obfuscation
pub fn recover_secret_with_xor(
    shares: &[Share],
    threshold: usize,
    mask: u64,
) -> Result<u64, SecretSharingError> {
    // 1. Recover obfuscated value
    let masked_secret = recover_secret_u64(shares, threshold)?;

    // 2. Use XOR to recover original value
    Ok(masked_secret ^ mask)
}

/// Get mask secret from configuration file
///
/// Reads the anonymous private key from the BFC configuration file and converts it to a u64 value.
/// The private key should be a hexadecimal string that can be parsed as a u64.
///
/// # Returns
/// - `Ok(u64)`: The parsed mask secret value
/// - `Err`: If the config file doesn't exist, is invalid, or the private key cannot be parsed
pub fn get_mask_secret_from_config() -> Result<u64, Box<dyn std::error::Error>> {
    let path = get_sui_config_directory().join("bfc_anonymous_config.yaml");

    // Load config file, return error if it doesn't exist or is invalid
    let config = AnonymousPrivateKeyConfig::from_yaml_file(&path)?;

    // Get the private key string from config
    let private_key_str = config.anonymous_privatekey
        .ok_or_else(|| anyhow!("Anonymous private key not found in configuration"))?;

    // Parse the private key string to u64
    // Handle both hex format (0x...) and decimal format
    let mask_secret = if private_key_str.starts_with("0x") || private_key_str.starts_with("0X") {
        // Parse as hexadecimal
        u64::from_str_radix(&private_key_str[2..], 16)
            .map_err(|e| anyhow!("Failed to parse private key as hex: {}", e))?
    } else {
        // Parse as decimal
        private_key_str.parse::<u64>()
            .map_err(|e| anyhow!("Failed to parse private key as decimal: {}", e))?
    };

    Ok(mask_secret)
}

pub fn get_sui_config_directory() -> PathBuf {
    match dirs::home_dir() {
        Some(v) => v.join(".bfc").join("bfc_config"),
        None => panic!("Cannot obtain home directory path"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_secret_sharing() {
        let secret = b"my secret data";
        let threshold = 3;
        let total_shares = 5;

        // Generate shares
        let shares = generate_shares(secret, threshold, total_shares).unwrap();
        assert_eq!(shares.len(), total_shares);

        // Recover secret using partial shares
        let recovered = recover_secret(&shares[0..threshold], threshold).unwrap();
        assert_eq!(recovered, secret);

        // Recover secret using all shares
        let recovered = recover_secret(&shares, threshold).unwrap();
        assert_eq!(recovered, secret);

        // Test insufficient shares case
        assert!(matches!(
            recover_secret(&shares[0..threshold - 1], threshold),
            Err(SecretSharingError::InsufficientShares(_, _))
        ));
    }

    #[test]
    fn test_error_handling() {
        // Test empty secret
        assert!(matches!(
            generate_shares(&[], 3, 5),
            Err(SecretSharingError::EmptySecret)
        ));

        // Test invalid threshold (1)
        assert!(matches!(
            generate_shares(b"data", 1, 3),
            Err(SecretSharingError::InvalidThreshold(1)) // Provide specific threshold value
        ));

        // Test invalid threshold (256)
        assert!(matches!(
            generate_shares(b"data", 256, 300),
            Err(SecretSharingError::ThresholdTooLarge(256)) // Provide specific threshold value
        ));

        // Test total shares less than threshold
        assert!(matches!(
            generate_shares(b"data", 4, 3),
            Err(SecretSharingError::InsufficientShares(4, 3)) // Provide specific threshold and share count
        ));
    }

    #[test]
    // Test u64 and u8 conversions
    fn test_conversions() {
        let num = 0x123456789ABCDEF0u64;
        let bytes = u64_to_bytes(num);
        assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0]);
        assert_eq!(bytes_to_u64(&bytes).unwrap(), num);
    }
    #[test]
    fn test_u64_secret_sharing() {
        let secret = 0xDEADBEEFCAFEBABEu64;
        let threshold = 3;
        let total_shares = 5;

        let shares = generate_shares_u64(secret, threshold, total_shares).unwrap();
        assert_eq!(shares.len(), total_shares);

        // Test consecutive shares
        assert_eq!(
            recover_secret_u64(&shares[0..threshold], threshold).unwrap(),
            secret
        );
        assert_eq!(
            recover_secret_u64(&shares[1..threshold + 1], threshold).unwrap(),
            secret
        );

        // Test scattered shares - create new Vec<Share> instead of Vec<&Share>
        let scattered = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
        assert_eq!(recover_secret_u64(&scattered, threshold).unwrap(), secret);

        // Test insufficient threshold case
        assert!(recover_secret_u64(&shares[0..threshold - 1], threshold).is_err());
    }
    #[test]
    fn test_xor_secret_sharing_basic() {
        let secret = 0xDEADBEEFu64;
        let threshold = 2;
        let total_shares = 4;
        let mask = 0x12345678ABCDEF00;

        let shares = generate_shares_with_xor(secret, threshold, total_shares, mask)
            .expect("Failed to generate shares");

        // Recover using sufficient shares
        let recovered = recover_secret_with_xor(&shares[..threshold], threshold, mask)
            .expect("Failed to recover secret");

        assert_eq!(secret, recovered, "Basic recovery failed");
    }

    #[test]
    fn test_xor_secret_sharing_randomized() {
        for _ in 0..100 {
            // Multiple random tests
            let secret = random::<u64>();
            let threshold = 3;
            let total_shares = 5;
            let mask = 0x12345678ABCDEF00;

            let shares = generate_shares_with_xor(secret, threshold, total_shares, mask)
                .expect("Failed to generate shares");

            // Recover using sufficient shares
            let recovered = recover_secret_with_xor(&shares[..threshold], threshold, mask)
                .expect("Failed to recover secret");

            assert_eq!(
                secret, recovered,
                "Randomized test failed for secret: {}",
                secret
            );
        }
    }

    #[test]
    fn test_xor_secret_sharing_insufficient_shares() {
        let secret = 0x12345678u64;
        let threshold = 3;
        let total_shares = 5;
        let mask = 0x12345678ABCDEF00;

        let shares = generate_shares_with_xor(secret, threshold, total_shares, mask)
            .expect("Failed to generate shares");

        // Should fail with insufficient shares
        assert!(
            recover_secret_with_xor(&shares[..threshold - 1], threshold, mask).is_err(),
            "Should fail with insufficient shares"
        );
    }

    #[test]
    fn test_xor_secret_sharing_wrong_mask() {
        let secret = 0xABCDEF01u64;
        let threshold = 2;
        let total_shares = 3;
        let mask = 0x12345678ABCDEF00;

        let shares = generate_shares_with_xor(secret, threshold, total_shares, mask)
            .expect("Failed to generate shares");

        // Should recover wrong value with wrong mask
        let wrong_mask = random::<u64>();
        let recovered = recover_secret_with_xor(&shares[..threshold], threshold, wrong_mask)
            .expect("Failed to recover (but should get wrong value)");

        assert_ne!(
            secret, recovered,
            "XOR protection failed - recovered correct secret with wrong mask"
        );
    }
}

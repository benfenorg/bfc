// XOR encryption module - providing an additional security layer for secret sharing
use crate::{
    error::SSSError, field::gf64_sss::FieldElement, field::FieldElement as FieldElementTrait,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

/// Simplified mask entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskEntry {
    pub key: String,
    pub mask: String,
}

/// Simplified encryption configuration structure
///
/// Load XOR mask configuration from simplified JSON file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    #[serde(flatten)]
    pub masks: HashMap<String, MaskEntry>,
}

/// Encrypted secret share
///
/// x-coordinate remains plaintext (for homomorphic operations), only y-value is encrypted
#[derive(Debug, Clone)]
pub struct EncryptedShare {
    pub x_coordinate: FieldElement, // x-coordinate remains plaintext
    pub y_encrypted: FieldElement,  // only y-value is encrypted
    pub y_mask_key: String,         // only need mask key for y-value
}

/// XOR encryption manager
///
/// Responsible for loading configuration, managing masks, executing encryption/decryption operations
#[derive(Clone)]
pub struct XorEncryptionManager {
    config: EncryptionConfig,
    masks: HashMap<String, u64>,
}

impl XorEncryptionManager {
    /// Load encryption configuration from JSON file
    ///
    /// # Arguments
    /// * `config_path` - Path to JSON configuration file
    ///
    /// # Returns
    /// * `Ok(XorEncryptionManager)` - Successfully loaded manager
    /// * `Err(SSSError)` - File reading or parsing error
    pub fn load_from_file(config_path: &str) -> Result<Self, SSSError> {
        // Read JSON configuration file
        let config_content = fs::read_to_string(config_path).map_err(|e| {
            SSSError::InvalidParameters(format!("Failed to read config file {config_path}: {e}"))
        })?;

        // Parse JSON content
        let config: EncryptionConfig = serde_json::from_str(&config_content).map_err(|e| {
            SSSError::InvalidParameters(format!("Failed to parse config JSON: {e}"))
        })?;

        // Extract mask mapping and parse hexadecimal masks
        let mut masks = HashMap::new();
        for entry in config.masks.values() {
            // Parse mask string
            let mask_u64 = if entry.mask.starts_with("0x") || entry.mask.starts_with("0X") {
                // Explicit hexadecimal prefix
                u64::from_str_radix(&entry.mask[2..], 16).map_err(|e| {
                    SSSError::InvalidParameters(format!(
                        "Failed to parse hex mask {}: {}",
                        entry.mask, e
                    ))
                })?
            } else if entry.mask.len() > 10 && entry.mask.chars().all(|c| c.is_ascii_hexdigit()) {
                // Length exceeds 10 digits and all are hexadecimal characters, likely hexadecimal (avoid confusion with decimal)
                u64::from_str_radix(&entry.mask, 16).map_err(|e| {
                    SSSError::InvalidParameters(format!(
                        "Failed to parse hex mask {}: {}",
                        entry.mask, e
                    ))
                })?
            } else {
                // Default to decimal parsing
                entry.mask.parse::<u64>().map_err(|e| {
                    SSSError::InvalidParameters(format!(
                        "Failed to parse decimal mask {}: {}",
                        entry.mask, e
                    ))
                })?
            };
            masks.insert(entry.key.clone(), mask_u64);
        }

        println!("🔐 XOR encryption manager loaded:");
        println!("   Number of masks: {}", masks.len());

        Ok(Self { config, masks })
    }

    /// Perform XOR encryption on a single secret share
    ///
    /// # Arguments
    /// * `share` - Original share (x, y)
    /// * `_x_mask_key` - x-coordinate mask key name (unused, x-coordinate remains plaintext)
    /// * `y_mask_key` - Mask key name used for y-value
    ///
    /// # Algorithm Principle
    /// To support homomorphic operations, x-coordinates must remain consistent, therefore:
    /// - x-coordinate remains plaintext, no encryption
    /// - Only y-value undergoes XOR encryption: y_encrypted = y ⊕ mask
    ///
    /// # Returns
    /// * `Ok(EncryptedShare)` - Encrypted share
    /// * `Err(SSSError)` - Errors such as mask not found
    pub fn encrypt_share(
        &self,
        share: &(FieldElement, FieldElement),
        _x_mask_key: &str,
        y_mask_key: &str,
    ) -> Result<EncryptedShare, SSSError> {
        let (x, y) = share;

        // Get y-value mask
        let y_mask = self.masks.get(y_mask_key).ok_or_else(|| {
            SSSError::InvalidParameters(format!("Y mask key '{y_mask_key}' not found"))
        })?;

        // Only perform XOR encryption on y-value, x-coordinate remains plaintext
        let y_u64 = FieldElementTrait::to_u64(y);
        let y_encrypted_u64 = y_u64 ^ y_mask;
        let y_encrypted = FieldElementTrait::from_u64(y_encrypted_u64);

        Ok(EncryptedShare {
            x_coordinate: *x, // x-coordinate remains plaintext
            y_encrypted,
            y_mask_key: y_mask_key.to_string(),
        })
    }

    /// Perform XOR decryption on encrypted secret share
    ///
    /// # Arguments
    /// * `encrypted_share` - Encrypted share
    ///
    /// # Returns
    /// * `Ok((FieldElement, FieldElement))` - Decrypted original share
    /// * `Err(SSSError)` - Errors such as mask not found
    pub fn decrypt_share(
        &self,
        encrypted_share: &EncryptedShare,
    ) -> Result<(FieldElement, FieldElement), SSSError> {
        // Get y-value mask
        let y_mask = self.masks.get(&encrypted_share.y_mask_key).ok_or_else(|| {
            SSSError::InvalidParameters(format!(
                "Y mask key '{}' not found",
                encrypted_share.y_mask_key
            ))
        })?;

        // Perform XOR decryption (only need to decrypt y-value, x-coordinate is already plaintext)
        let y_encrypted_u64 = FieldElementTrait::to_u64(&encrypted_share.y_encrypted);
        let y_decrypted_u64 = y_encrypted_u64 ^ y_mask;
        let y_decrypted = FieldElementTrait::from_u64(y_decrypted_u64);

        Ok((encrypted_share.x_coordinate, y_decrypted))
    }

    /// Batch encrypt secret shares
    ///
    /// # Arguments
    /// * `shares` - List of original shares
    /// * `x_mask_key` - Mask key name used for x-coordinate
    /// * `y_mask_key` - Mask key name used for y-value
    ///
    /// # Returns
    /// * `Ok(Vec<EncryptedShare>)` - List of encrypted shares
    /// * `Err(SSSError)` - Encryption error
    pub fn encrypt_shares(
        &self,
        shares: &[(FieldElement, FieldElement)],
        x_mask_key: &str,
        y_mask_key: &str,
    ) -> Result<Vec<EncryptedShare>, SSSError> {
        shares
            .iter()
            .map(|share| self.encrypt_share(share, x_mask_key, y_mask_key))
            .collect()
    }

    /// Batch decrypt secret shares
    ///
    /// # Arguments
    /// * `encrypted_shares` - List of encrypted shares
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - List of decrypted shares
    /// * `Err(SSSError)` - Decryption error
    pub fn decrypt_shares(
        &self,
        encrypted_shares: &[EncryptedShare],
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        encrypted_shares
            .iter()
            .map(|encrypted_share| self.decrypt_share(encrypted_share))
            .collect()
    }

    /// Get list of available mask keys
    ///
    /// # Returns
    /// List of all available mask keys
    pub fn get_available_mask_keys(&self) -> Vec<&String> {
        self.masks.keys().collect()
    }

    /// Display encryption configuration information
    pub fn display_config(&self) {
        println!("🔧 XOR encryption configuration:");
        println!("   Algorithm: XOR");
        println!("   Description: Simplified XOR mask configuration");
        println!("   \n🔑 Available masks:");

        for (name, entry) in &self.config.masks {
            println!("   - {}: {}", name, entry.key);
        }
    }
}

/// Encryption-aware homomorphic operations
///
/// Automatically handle XOR decryption before performing homomorphic operations
pub struct EncryptedHomomorphicOperations;

impl EncryptedHomomorphicOperations {
    /// Perform homomorphic addition on two encrypted secret shares
    ///
    /// # Arguments
    /// * `manager` - XOR encryption manager
    /// * `encrypted_shares_a` - First encrypted secret share
    /// * `encrypted_shares_b` - Second encrypted secret share
    ///
    /// # Algorithm Flow
    /// 1. Decrypt both share sets
    /// 2. Perform homomorphic addition
    /// 3. Return result (optionally re-encrypt)
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Addition result
    /// * `Err(SSSError)` - Operation error
    pub fn add_encrypted_shares(
        manager: &XorEncryptionManager,
        encrypted_shares_a: &[EncryptedShare],
        encrypted_shares_b: &[EncryptedShare],
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Step 1: Decrypt shares
        let decrypted_a = manager.decrypt_shares(encrypted_shares_a)?;
        let decrypted_b = manager.decrypt_shares(encrypted_shares_b)?;

        // Step 2: Perform homomorphic addition
        use crate::math::HomomorphicOperations;
        let result = HomomorphicOperations::add_shares(&decrypted_a, &decrypted_b)?;

        Ok(result)
    }

    /// Perform homomorphic subtraction on two encrypted secret shares
    ///
    /// # Arguments
    /// * `manager` - XOR encryption manager  
    /// * `encrypted_shares_a` - Encrypted shares of minuend
    /// * `encrypted_shares_b` - Encrypted shares of subtrahend
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Subtraction result
    /// * `Err(SSSError)` - Operation error
    pub fn subtract_encrypted_shares(
        manager: &XorEncryptionManager,
        encrypted_shares_a: &[EncryptedShare],
        encrypted_shares_b: &[EncryptedShare],
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Step 1: Decrypt shares
        let decrypted_a = manager.decrypt_shares(encrypted_shares_a)?;
        let decrypted_b = manager.decrypt_shares(encrypted_shares_b)?;

        // Step 2: Perform homomorphic subtraction
        use crate::math::HomomorphicOperations;
        let result = HomomorphicOperations::subtract_shares(&decrypted_a, &decrypted_b)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use std::fs;

    /// Create deterministic random number generator
    fn create_test_rng() -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(42)
    }

    /// Create test configuration file
    fn create_test_config() -> Result<std::path::PathBuf, std::io::Error> {
        let config_content = r#"{
  "test_x_mask": {
    "key": "TEST_X_MASK",
    "mask": "0x1234567890ABCDEF"
  },
  "test_y_mask": {
    "key": "TEST_Y_MASK", 
    "mask": "0xFEDCBA0987654321"
  }
}"#;

        use std::env;
        use std::time::{SystemTime, UNIX_EPOCH};
        let temp_dir = env::temp_dir();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let config_path = temp_dir.join(format!("test_encryption_config_{timestamp}.json"));
        fs::write(&config_path, config_content)?;
        Ok(config_path)
    }

    #[test]
    fn test_load_config_from_file() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        assert_eq!(manager.masks.len(), 2);
        assert!(manager.masks.contains_key("TEST_X_MASK"));
        assert!(manager.masks.contains_key("TEST_Y_MASK"));

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_encrypt_decrypt_single_share() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        // Create test share
        let original_share = (
            FieldElementTrait::from_u64(12345),
            FieldElementTrait::from_u64(67890),
        );

        // Encrypt
        let encrypted_share = manager
            .encrypt_share(&original_share, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption should succeed");

        // Verify encrypted values are different (x-coordinate remains plaintext, only y-value is encrypted)
        assert_eq!(encrypted_share.x_coordinate, original_share.0); // x-coordinate remains unchanged
        assert_ne!(encrypted_share.y_encrypted, original_share.1); // y-value is encrypted

        // Decrypt
        let decrypted_share = manager
            .decrypt_share(&encrypted_share)
            .expect("Decryption should succeed");

        // Verify decryption recovers original values
        assert_eq!(decrypted_share.0, original_share.0);
        assert_eq!(decrypted_share.1, original_share.1);

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }


    #[test]
    fn test_encrypted_homomorphic_addition() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        let mut rng = create_test_rng();

        // Create shares for two secrets
        let secret_a = 100u64;
        let secret_b = 200u64;
        let expected_sum = secret_a + secret_b;

        let threshold = 3;
        let total_shares = 5;

        // Create compatible shares (same x-coordinates)
        use crate::field::gf64_sss::random_element;
        use crate::poly::Polynomial;

        let mut x_coords = Vec::new();
        for _ in 0..total_shares {
            x_coords.push(random_element(&mut rng));
        }

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
        let encrypted_shares_a = manager
            .encrypt_shares(&shares_a, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption A should succeed");
        let encrypted_shares_b = manager
            .encrypt_shares(&shares_b, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption B should succeed");

        // Perform encrypted homomorphic addition
        let sum_shares = EncryptedHomomorphicOperations::add_encrypted_shares(
            &manager,
            &encrypted_shares_a,
            &encrypted_shares_b,
        )
        .expect("Encrypted homomorphic addition should succeed");

        // Verify result
        use crate::secret::SecretSharing;
        let recovered_sum =
            SecretSharing::recover(&sum_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_sum, expected_sum,
            "Encrypted homomorphic addition should produce correct result"
        );

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_invalid_mask_keys() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        let test_share = (
            FieldElementTrait::from_u64(123),
            FieldElementTrait::from_u64(456),
        );

        // Test non-existent x mask key (now x-coordinate remains plaintext, so invalid x mask key won't cause error)
        let result = manager.encrypt_share(&test_share, "INVALID_X_MASK", "TEST_Y_MASK");
        assert!(
            result.is_ok(),
            "X mask key is ignored since x coordinate remains plaintext"
        );

        // Test non-existent y mask key
        let result = manager.encrypt_share(&test_share, "TEST_X_MASK", "INVALID_Y_MASK");
        assert!(result.is_err());

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_get_available_mask_keys() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        let available_keys = manager.get_available_mask_keys();

        // Verify number and content of returned keys
        assert_eq!(available_keys.len(), 2);
        assert!(available_keys.contains(&&"TEST_X_MASK".to_string()));
        assert!(available_keys.contains(&&"TEST_Y_MASK".to_string()));

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_display_config() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        // This function outputs to stdout, we can only ensure it doesn't crash
        manager.display_config();

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_encrypted_homomorphic_subtraction() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        let mut rng = create_test_rng();

        // Create two secret shares
        let secret_a = 1000u64;
        let secret_b = 300u64;
        let expected_diff = secret_a - secret_b;

        let threshold = 3;
        let total_shares = 5;

        // Create compatible shares (same x-coordinates)
        use crate::field::gf64_sss::random_element;
        use crate::poly::Polynomial;

        let mut x_coords = Vec::new();
        for _ in 0..total_shares {
            x_coords.push(random_element(&mut rng));
        }

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
        let encrypted_shares_a = manager
            .encrypt_shares(&shares_a, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption A should succeed");
        let encrypted_shares_b = manager
            .encrypt_shares(&shares_b, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption B should succeed");

        // Perform encrypted homomorphic subtraction
        let diff_shares = EncryptedHomomorphicOperations::subtract_encrypted_shares(
            &manager,
            &encrypted_shares_a,
            &encrypted_shares_b,
        )
        .expect("Encrypted homomorphic subtraction should succeed");

        // Verify result
        use crate::secret::SecretSharing;
        let recovered_diff =
            SecretSharing::recover(&diff_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_diff, expected_diff,
            "Encrypted homomorphic subtraction should produce correct result"
        );

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_encrypt_decrypt_consistency() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        // Test multiple different share values
        let test_shares = [
            (
                FieldElementTrait::from_u64(0),
                FieldElementTrait::from_u64(0),
            ),
            (
                FieldElementTrait::from_u64(1),
                FieldElementTrait::from_u64(u64::MAX),
            ),
            (
                FieldElementTrait::from_u64(u64::MAX),
                FieldElementTrait::from_u64(1),
            ),
            (
                FieldElementTrait::from_u64(12345),
                FieldElementTrait::from_u64(67890),
            ),
        ];

        for (i, test_share) in test_shares.iter().enumerate() {
            // Encrypt
            let encrypted_share = manager
                .encrypt_share(test_share, "TEST_X_MASK", "TEST_Y_MASK")
                .expect("Encryption should succeed");

            // Verify encrypted values (x-coordinate remains plaintext, y-value is encrypted)
            assert_eq!(
                encrypted_share.x_coordinate, test_share.0,
                "X coordinate should remain plaintext for test case {i}"
            );
            assert_ne!(
                encrypted_share.y_encrypted, test_share.1,
                "Y coordinate should be encrypted for test case {i}"
            );

            // Decrypt
            let decrypted_share = manager
                .decrypt_share(&encrypted_share)
                .expect("Decryption should succeed");

            // Verify decryption recovers original values
            assert_eq!(
                decrypted_share.0, test_share.0,
                "X coordinate should match after decrypt for test case {i}"
            );
            assert_eq!(
                decrypted_share.1, test_share.1,
                "Y coordinate should match after decrypt for test case {i}"
            );
        }

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_mask_parsing() {
        // Test hexadecimal mask parsing
        let hex_config_content = r#"{
            "hex_mask_1": {"key": "HEX_MASK_1", "mask": "0x123456789ABCDEF0"},
            "hex_mask_2": {"key": "HEX_MASK_2", "mask": "0xFEDCBA0987654321"},
            "decimal_mask": {"key": "DEC_MASK", "mask": "1234567890"}
        }"#;

        use std::env;
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test_hex_config.json");
        fs::write(&config_path, hex_config_content).expect("Write hex config should succeed");

        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Load hex config should succeed");

        // Verify number of masks
        assert_eq!(manager.masks.len(), 3);

        // Verify hexadecimal masks are correctly parsed
        assert!(manager.masks.contains_key("HEX_MASK_1"));
        assert!(manager.masks.contains_key("HEX_MASK_2"));
        assert!(manager.masks.contains_key("DEC_MASK"));

        // Verify mask values
        assert_eq!(
            manager.masks.get("HEX_MASK_1"),
            Some(&0x123456789ABCDEF0u64)
        );
        assert_eq!(
            manager.masks.get("HEX_MASK_2"),
            Some(&0xFEDCBA0987654321u64)
        );
        assert_eq!(manager.masks.get("DEC_MASK"), Some(&1234567890u64));

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_invalid_mask_format() {
        // Test invalid mask format
        let invalid_config_content = r#"{
            "invalid_hex": {"key": "INVALID_HEX", "mask": "0xGHIJKLMN"},
            "valid_mask": {"key": "VALID_MASK", "mask": "0x1234567890ABCDEF"}
        }"#;

        use std::env;
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test_invalid_config.json");
        fs::write(&config_path, invalid_config_content)
            .expect("Write invalid config should succeed");

        let result = XorEncryptionManager::load_from_file(config_path.to_str().unwrap());

        // Should fail due to invalid hexadecimal mask
        assert!(result.is_err(), "Invalid hex mask should cause error");

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }


    #[test]
    fn test_encrypted_share_structure() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        let test_share = (
            FieldElementTrait::from_u64(999),
            FieldElementTrait::from_u64(888),
        );

        let encrypted_share = manager
            .encrypt_share(&test_share, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption should succeed");

        // Verify EncryptedShare structure fields
        assert_eq!(encrypted_share.y_mask_key, "TEST_Y_MASK");

        // Verify encrypted values (x-coordinate remains plaintext, y-value is encrypted)
        assert_eq!(encrypted_share.x_coordinate, test_share.0); // x-coordinate remains plaintext
        assert_ne!(encrypted_share.y_encrypted, test_share.1); // y-value is encrypted

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_different_mask_combinations() {
        let config_path = create_test_config().expect("Failed to create test config");
        let manager = XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
            .expect("Failed to load config");

        let test_share = (
            FieldElementTrait::from_u64(777),
            FieldElementTrait::from_u64(555),
        );

        // Test different mask key combinations
        let mask_combinations = [
            ("TEST_X_MASK", "TEST_X_MASK"), // Same mask
            ("TEST_X_MASK", "TEST_Y_MASK"), // Different masks
            ("TEST_Y_MASK", "TEST_X_MASK"), // Swapped masks
            ("TEST_Y_MASK", "TEST_Y_MASK"), // Same mask
        ];

        for (i, (x_mask, y_mask)) in mask_combinations.iter().enumerate() {
            let encrypted_share = manager
                .encrypt_share(&test_share, x_mask, y_mask)
                .expect("Encryption should succeed");

            let decrypted_share = manager
                .decrypt_share(&encrypted_share)
                .expect("Decryption should succeed");

            assert_eq!(
                decrypted_share, test_share,
                "Decryption should recover original for combination {i}"
            );
        }

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

}

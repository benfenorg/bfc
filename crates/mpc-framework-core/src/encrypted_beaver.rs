/*!
# Encrypted Share Beaver Multiplication API

This module provides a unified API for handling mask-encrypted secret shares, decrypting them and performing secure multiplication using Beaver triples.

## Core Features

### 1. Encrypted Share Processing
- Accept XOR mask-encrypted secret shares as input
- Automatically decrypt shares to restore plaintext secret shares
- Maintain coordinate system consistency to ensure homomorphic operation correctness

### 2. Beaver Triple Multiplication
- Use pre-generated or dynamically generated Beaver triples
- Execute secure multiplication protocol without revealing original secret values

### 3. Unified API Design
- Simplified one-stop interface hiding complex internal implementation
- Automatically handle coordinate matching, decryption, multiplication and all steps
- Complete error handling and validation mechanisms

## Use Cases

- **Secure Database Queries**: Perform computations on encrypted data
- **Privacy-Preserving ML**: Secure multiplication during training processes
- **Multi-Party Secure Computation**: Distributed computing scenarios

## Performance Optimizations

- **Cache Reuse**: Reuse Beaver triple distributors
- **Memory Optimization**: Minimize intermediate data structures

*/

use crate::{
    beaver::{BeaverMultiplication, BeaverTriple, BeaverTripleDistributor},
    encryption::{EncryptedShare, XorEncryptionManager},
    error::SSSError,
    field::gf64_sss::FieldElement,
};
use rand_core::RngCore;

/// Encrypted Share Beaver Multiplication Processor
///
/// Provides unified interface for decryption of encrypted shares and Beaver multiplication
pub struct EncryptedBeaverProcessor {
    /// XOR encryption manager
    encryption_manager: XorEncryptionManager,

    /// Beaver triple distributor
    beaver_distributor: Option<BeaverTripleDistributor>,

    /// Threshold parameter
    threshold: usize,

    /// Total number of shares
    total_shares: usize,
}

impl EncryptedBeaverProcessor {
    /// Create a new encrypted share Beaver processor
    ///
    /// # Parameters
    /// * `encryption_manager` - XOR encryption manager
    /// * `threshold` - Secret reconstruction threshold
    /// * `total_shares` - Total number of shares
    /// * `beaver_cache_size` - Beaver triple cache size
    /// * `rng` - Random number generator
    ///
    /// # Returns
    /// * `Ok(EncryptedBeaverProcessor)` - Successfully created processor
    /// * `Err(SSSError)` - Error information for creation failure
    pub fn new<R: RngCore>(
        encryption_manager: XorEncryptionManager,
        threshold: usize,
        total_shares: usize,
        beaver_cache_size: usize,
        rng: &mut R,
    ) -> Result<Self, SSSError> {
        // Validate parameter validity
        if threshold == 0 || threshold > total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Invalid threshold: {threshold} must be between 1 and {total_shares}"
            )));
        }

        // Create Beaver triple distributor
        let beaver_distributor = Some(BeaverTripleDistributor::new(
            threshold,
            total_shares,
            beaver_cache_size,
            rng,
        )?);

        Ok(Self {
            encryption_manager,
            beaver_distributor,
            threshold,
            total_shares,
        })
    }

    /// Create processor with specified coordinates
    ///
    /// # Parameters
    /// * `encryption_manager` - XOR encryption manager
    /// * `threshold` - Secret reconstruction threshold
    /// * `total_shares` - Total number of shares
    /// * `coordinates` - Predefined x coordinates
    /// * `beaver_cache_size` - Beaver triple cache size
    /// * `rng` - Random number generator
    pub fn new_with_coordinates<R: RngCore>(
        encryption_manager: XorEncryptionManager,
        threshold: usize,
        total_shares: usize,
        coordinates: &[FieldElement],
        beaver_cache_size: usize,
        rng: &mut R,
    ) -> Result<Self, SSSError> {
        // Validate coordinate count
        if coordinates.len() != total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Coordinates length {} must match total_shares {}",
                coordinates.len(),
                total_shares
            )));
        }

        // Create Beaver triple distributor with coordinates
        let beaver_distributor = Some(BeaverTripleDistributor::new_with_coordinates(
            threshold,
            total_shares,
            beaver_cache_size,
            coordinates,
            rng,
        )?);

        Ok(Self {
            encryption_manager,
            beaver_distributor,
            threshold,
            total_shares,
        })
    }

    /// Perform Beaver multiplication on encrypted shares (main API method)
    ///
    /// # Parameters
    /// * `encrypted_x_shares` - Encrypted shares of the first operand
    /// * `encrypted_y_shares` - Encrypted shares of the second operand
    /// * `rng` - Random number generator
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Secret shares of multiplication result
    /// * `Err(SSSError)` - Error information for operation failure
    ///
    /// # Algorithm Flow
    /// 1. Decrypt input encrypted shares
    /// 2. Obtain or generate Beaver triples
    /// 3. Execute Beaver multiplication protocol
    /// 4. Return multiplication result
    pub fn multiply_encrypted_shares<R: RngCore>(
        &mut self,
        encrypted_x_shares: &[EncryptedShare],
        encrypted_y_shares: &[EncryptedShare],
        rng: &mut R,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Step 1: Validate input
        if encrypted_x_shares.len() != encrypted_y_shares.len() {
            return Err(SSSError::InvalidParameters(
                "Encrypted share arrays must have the same length".into(),
            ));
        }

        if encrypted_x_shares.len() != self.total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Number of shares {} does not match expected total_shares {}",
                encrypted_x_shares.len(),
                self.total_shares
            )));
        }

        // Step 2: Decrypt shares
        let x_shares = self.decrypt_shares(encrypted_x_shares)?;
        let y_shares = self.decrypt_shares(encrypted_y_shares)?;

        // Step 3: Obtain Beaver triple
        let beaver_triple = match &mut self.beaver_distributor {
            Some(distributor) => distributor.get_triple(rng)?,
            None => {
                return Err(SSSError::InvalidParameters(
                    "Beaver distributor not initialized".into(),
                ));
            }
        };

        // Step 4: Execute Beaver multiplication
        BeaverMultiplication::multiply_with_beaver(&x_shares, &y_shares, &beaver_triple)
    }


    /// Perform Beaver multiplication and return encrypted result
    ///
    /// # Parameters
    /// * `encrypted_x_shares` - Encrypted shares of the first operand
    /// * `encrypted_y_shares` - Encrypted shares of the second operand
    /// * `result_x_mask_key` - Mask key for result x coordinate
    /// * `result_y_mask_key` - Mask key for result y coordinate
    /// * `rng` - Random number generator
    ///
    /// # Returns
    /// * `Ok(Vec<EncryptedShare>)` - Encrypted multiplication result
    /// * `Err(SSSError)` - Error information for operation failure
    pub fn multiply_and_encrypt<R: RngCore>(
        &mut self,
        encrypted_x_shares: &[EncryptedShare],
        encrypted_y_shares: &[EncryptedShare],
        result_x_mask_key: &str,
        result_y_mask_key: &str,
        rng: &mut R,
    ) -> Result<Vec<EncryptedShare>, SSSError> {
        // Execute Beaver multiplication
        let multiplication_result =
            self.multiply_encrypted_shares(encrypted_x_shares, encrypted_y_shares, rng)?;

        // Encrypt result
        self.encryption_manager.encrypt_shares(
            &multiplication_result,
            result_x_mask_key,
            result_y_mask_key,
        )
    }

    /// Perform multiplication with specified Beaver triple
    ///
    /// # Parameters
    /// * `encrypted_x_shares` - Encrypted shares of the first operand
    /// * `encrypted_y_shares` - Encrypted shares of the second operand
    /// * `beaver_triple` - Specified Beaver triple
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Multiplication result
    /// * `Err(SSSError)` - Error information for operation failure
    pub fn multiply_with_specific_triple(
        &self,
        encrypted_x_shares: &[EncryptedShare],
        encrypted_y_shares: &[EncryptedShare],
        beaver_triple: &BeaverTriple,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Decrypt shares
        let x_shares = self.decrypt_shares(encrypted_x_shares)?;
        let y_shares = self.decrypt_shares(encrypted_y_shares)?;

        // Execute Beaver multiplication
        BeaverMultiplication::multiply_with_beaver(&x_shares, &y_shares, beaver_triple)
    }

    /// Chain multiplication: perform multiple consecutive multiplication operations
    ///
    /// # Parameters
    /// * `encrypted_operands` - List of encrypted operands
    /// * `rng` - Random number generator
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Final result of chain multiplication
    /// * `Err(SSSError)` - Error information for operation failure
    ///
    /// # Description
    /// Computes operands[0] * operands[1] * operands[2] * ... * operands[n-1]
    pub fn chain_multiply_encrypted_shares<R: RngCore>(
        &mut self,
        encrypted_operands: &[Vec<EncryptedShare>],
        rng: &mut R,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        if encrypted_operands.len() < 2 {
            return Err(SSSError::InvalidParameters(
                "At least two operands required for multiplication".into(),
            ));
        }

        // Initialize result with the first operand
        let mut result_shares = self.decrypt_shares(&encrypted_operands[0])?;

        // Multiply with subsequent operands sequentially
        for encrypted_operand in &encrypted_operands[1..] {
            // Obtain Beaver triple
            let beaver_triple = match &mut self.beaver_distributor {
                Some(distributor) => distributor.get_triple(rng)?,
                None => {
                    return Err(SSSError::InvalidParameters(
                        "Beaver distributor not initialized".into(),
                    ));
                }
            };

            // Decrypt current operand
            let operand_shares = self.decrypt_shares(encrypted_operand)?;

            // Execute multiplication
            result_shares = BeaverMultiplication::multiply_with_beaver(
                &result_shares,
                &operand_shares,
                &beaver_triple,
            )?;
        }

        Ok(result_shares)
    }

    /// Get processor statistics
    ///
    /// # Returns
    /// * `ProcessorStats` - Struct containing performance statistics
    pub fn get_statistics(&self) -> ProcessorStats {
        ProcessorStats {
            threshold: self.threshold,
            total_shares: self.total_shares,
            beaver_distributor_initialized: self.beaver_distributor.is_some(),
            available_mask_keys: self
                .encryption_manager
                .get_available_mask_keys()
                .into_iter()
                .cloned()
                .collect(),
        }
    }

    // === Internal Helper Methods ===

    /// Decrypt share array
    ///
    /// # Parameters
    /// * `encrypted_shares` - Encrypted share array
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Decrypted shares
    /// * `Err(SSSError)` - Error information for decryption failure
    fn decrypt_shares(
        &self,
        encrypted_shares: &[EncryptedShare],
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        let mut decrypted_shares = Vec::with_capacity(encrypted_shares.len());

        for encrypted_share in encrypted_shares {
            let decrypted_share = self.encryption_manager.decrypt_share(encrypted_share)?;
            decrypted_shares.push(decrypted_share);
        }

        Ok(decrypted_shares)
    }
}

/// Processor statistics
#[derive(Debug, Clone)]
pub struct ProcessorStats {
    /// Threshold parameter
    pub threshold: usize,

    /// Total number of shares
    pub total_shares: usize,

    /// Whether Beaver distributor is initialized
    pub beaver_distributor_initialized: bool,

    /// Available mask keys
    pub available_mask_keys: Vec<String>,
}

impl ProcessorStats {
    /// Get statistics summary
    ///
    /// # Returns
    /// * `String` - Formatted statistics string
    pub fn get_summary(&self) -> String {
        format!(
            "EncryptedBeaverProcessor Stats:\n\
             - Threshold: {}/{}\n\
             - Beaver Distributor: {}\n\
             - Available Mask Keys: {}\n\
             - Mask Keys: {:?}",
            self.threshold,
            self.total_shares,
            if self.beaver_distributor_initialized {
                "Initialized"
            } else {
                "Not Initialized"
            },
            self.available_mask_keys.len(),
            self.available_mask_keys
        )
    }
}

/// Convenience function: simplified encrypted share Beaver multiplication API
///
/// # Parameters
/// * `encrypted_x_shares` - Encrypted shares of the first operand
/// * `encrypted_y_shares` - Encrypted shares of the second operand
/// * `encryption_manager` - XOR encryption manager
/// * `threshold` - Secret reconstruction threshold
/// * `rng` - Random number generator
///
/// # Returns
/// * `Ok(Vec<(FieldElement, FieldElement)>)` - Secret shares of multiplication result
/// * `Err(SSSError)` - Error information for operation failure
///
/// # Usage
/// This is a simplified one-time API suitable for scenarios that don't require repeated operations.
pub fn multiply_encrypted_shares_simple<R: RngCore>(
    encrypted_x_shares: &[EncryptedShare],
    encrypted_y_shares: &[EncryptedShare],
    encryption_manager: XorEncryptionManager,
    threshold: usize,
    rng: &mut R,
) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
    let total_shares = encrypted_x_shares.len();

    // Create unified coordinate system
    let x_coords: Vec<FieldElement> = (1..=total_shares)
        .map(|i| FieldElement::from(i as u64))
        .collect();

    let mut processor = EncryptedBeaverProcessor::new_with_coordinates(
        encryption_manager,
        threshold,
        total_shares,
        &x_coords,
        32, // Default cache size
        rng,
    )?;

    processor.multiply_encrypted_shares(encrypted_x_shares, encrypted_y_shares, rng)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{field::FieldElement as FieldElementTrait, secret::SecretSharing};
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    /// Create test random number generator
    fn create_test_rng() -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(42)
    }

    /// Create test encryption manager
    fn create_test_encryption_manager() -> XorEncryptionManager {
        use std::fs;
        use tempfile::NamedTempFile;

        // Create temporary configuration file
        let config_content = r#"{
            "mask1": {
                "key": "mask1",
                "mask": "1234567890ABCDEF"
            },
            "mask2": {
                "key": "mask2", 
                "mask": "FEDCBA0987654321"
            },
            "mask3": {
                "key": "mask3",
                "mask": "1111222233334444"
            },
            "mask4": {
                "key": "mask4",
                "mask": "5555666677778888"
            }
        }"#;

        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        fs::write(temp_file.path(), config_content).expect("Failed to write config");

        XorEncryptionManager::load_from_file(temp_file.path().to_str().unwrap())
            .expect("Failed to load encryption manager")
    }

    /// Create unified x-coordinate system
    fn create_x_coordinates(total_shares: usize) -> Vec<FieldElement> {
        (1..=total_shares)
            .map(|i| FieldElement::from(i as u64))
            .collect()
    }

    /// Create secret sharing with specified coordinates
    fn create_shares_with_coordinates<R: RngCore>(
        secret: u64,
        threshold: usize,
        x_coords: &[FieldElement],
        rng: &mut R,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        use crate::poly::Polynomial;

        let polynomial = Polynomial::new(threshold - 1, FieldElement::from(secret), rng);
        let shares = x_coords
            .iter()
            .map(|&x| (x, polynomial.evaluate(&x)))
            .collect();

        Ok(shares)
    }

    #[test]
    fn test_encrypted_beaver_processor_creation() {
        let mut rng = create_test_rng();
        let encryption_manager = create_test_encryption_manager();
        let (threshold, total_shares) = (3, 5);

        // Create unified coordinate system
        let x_coords = create_x_coordinates(total_shares);

        let processor = EncryptedBeaverProcessor::new_with_coordinates(
            encryption_manager,
            threshold,
            total_shares,
            &x_coords,
            32,
            &mut rng,
        );

        assert!(processor.is_ok(), "Processor creation should succeed");

        let processor = processor.unwrap();
        let stats = processor.get_statistics();
        assert_eq!(stats.threshold, threshold);
        assert_eq!(stats.total_shares, total_shares);
        assert!(stats.beaver_distributor_initialized);
        assert!(stats.available_mask_keys.len() >= 4);
    }

    #[test]
    fn test_basic_encrypted_multiplication() {
        let mut rng = create_test_rng();
        let encryption_manager = create_test_encryption_manager();
        let (threshold, total_shares) = (3, 5);

        // Create unified coordinate system
        let x_coords = create_x_coordinates(total_shares);

        let mut processor = EncryptedBeaverProcessor::new_with_coordinates(
            encryption_manager.clone(),
            threshold,
            total_shares,
            &x_coords,
            32,
            &mut rng,
        )
        .unwrap();

        // Create test secrets
        let secret_x = 15u64;
        let secret_y = 7u64;

        // Calculate correct expected value using finite field arithmetic
        let x_field = FieldElement::from(secret_x);
        let y_field = FieldElement::from(secret_y);
        let expected_field: FieldElement = x_field * y_field;
        let expected = expected_field.to_u64();

        // Create secret shares using unified coordinates
        let x_shares =
            create_shares_with_coordinates(secret_x, threshold, &x_coords, &mut rng).unwrap();
        let y_shares =
            create_shares_with_coordinates(secret_y, threshold, &x_coords, &mut rng).unwrap();

        // Encrypt shares
        let encrypted_x_shares = encryption_manager
            .encrypt_shares(&x_shares, "mask1", "mask2")
            .unwrap();
        let encrypted_y_shares = encryption_manager
            .encrypt_shares(&y_shares, "mask3", "mask4")
            .unwrap();

        // Execute encrypted multiplication
        let result_shares = processor
            .multiply_encrypted_shares(&encrypted_x_shares, &encrypted_y_shares, &mut rng)
            .unwrap();

        // Verify result
        let recovered_result = SecretSharing::recover(&result_shares[0..threshold]).unwrap();
        assert_eq!(
            recovered_result, expected,
            "Multiplication result should be correct"
        );
    }

    #[test]
    fn test_simple_api_function() {
        let mut rng = create_test_rng();
        let encryption_manager = create_test_encryption_manager();
        let (threshold, total_shares) = (3, 5);

        // Create unified coordinate system
        let x_coords = create_x_coordinates(total_shares);

        // Create test data
        let secret_x = 25u64;
        let secret_y = 4u64;

        let x_shares =
            create_shares_with_coordinates(secret_x, threshold, &x_coords, &mut rng).unwrap();
        let y_shares =
            create_shares_with_coordinates(secret_y, threshold, &x_coords, &mut rng).unwrap();

        let encrypted_x_shares = encryption_manager
            .encrypt_shares(&x_shares, "mask1", "mask2")
            .unwrap();
        let encrypted_y_shares = encryption_manager
            .encrypt_shares(&y_shares, "mask3", "mask4")
            .unwrap();

        // Use simplified API
        let result_shares = multiply_encrypted_shares_simple(
            &encrypted_x_shares,
            &encrypted_y_shares,
            create_test_encryption_manager(),
            threshold,
            &mut rng,
        )
        .unwrap();

        // Verify result
        let recovered_result = SecretSharing::recover(&result_shares[0..threshold]).unwrap();

        let x_field = FieldElement::from(secret_x);
        let y_field = FieldElement::from(secret_y);
        let expected: FieldElement = x_field * y_field;
        let expected_u64 = expected.to_u64();

        assert_eq!(
            recovered_result, expected_u64,
            "Simple API result should be correct"
        );
    }


    #[test]
    fn test_multiply_and_encrypt() {
        let mut rng = create_test_rng();
        let encryption_manager = create_test_encryption_manager();
        let (threshold, total_shares) = (3, 5);

        // Create unified coordinate system
        let x_coords = create_x_coordinates(total_shares);

        let mut processor = EncryptedBeaverProcessor::new_with_coordinates(
            encryption_manager.clone(),
            threshold,
            total_shares,
            &x_coords,
            32,
            &mut rng,
        )
        .unwrap();

        // Create test data
        let secret_x = 12u64;
        let secret_y = 8u64;

        let x_shares =
            create_shares_with_coordinates(secret_x, threshold, &x_coords, &mut rng).unwrap();
        let y_shares =
            create_shares_with_coordinates(secret_y, threshold, &x_coords, &mut rng).unwrap();

        let encrypted_x_shares = encryption_manager
            .encrypt_shares(&x_shares, "mask1", "mask2")
            .unwrap();
        let encrypted_y_shares = encryption_manager
            .encrypt_shares(&y_shares, "mask3", "mask4")
            .unwrap();

        // Execute multiplication and encrypt result
        let encrypted_result = processor
            .multiply_and_encrypt(
                &encrypted_x_shares,
                &encrypted_y_shares,
                "mask1", // Reuse mask key
                "mask3",
                &mut rng,
            )
            .unwrap();

        // Decrypt and verify result
        let decrypted_result_shares: Result<Vec<_>, _> = encrypted_result
            .iter()
            .map(|encrypted_share| encryption_manager.decrypt_share(encrypted_share))
            .collect();

        let decrypted_shares = decrypted_result_shares.unwrap();
        let recovered_result = SecretSharing::recover(&decrypted_shares[0..threshold]).unwrap();

        // Calculate expected result
        let x_field = FieldElement::from_u64(secret_x);
        let y_field = FieldElement::from_u64(secret_y);
        let expected: FieldElement = x_field * y_field;
        let expected_u64 = expected.to_u64();

        assert_eq!(
            recovered_result, expected_u64,
            "Multiply and encrypt result should be correct"
        );
    }

    #[test]
    fn test_chain_multiplication() {
        let mut rng = create_test_rng();
        let encryption_manager = create_test_encryption_manager();
        let (threshold, total_shares) = (3, 5);

        // Create unified coordinate system
        let x_coords = create_x_coordinates(total_shares);

        let mut processor = EncryptedBeaverProcessor::new_with_coordinates(
            encryption_manager.clone(),
            threshold,
            total_shares,
            &x_coords,
            64,
            &mut rng,
        )
        .unwrap();

        // Create test data: calculate (a * b) * c
        let secret_a = 3u64;
        let secret_b = 4u64;
        let secret_c = 5u64;

        // Step 1: calculate a * b
        let a_shares =
            create_shares_with_coordinates(secret_a, threshold, &x_coords, &mut rng).unwrap();
        let b_shares =
            create_shares_with_coordinates(secret_b, threshold, &x_coords, &mut rng).unwrap();

        let encrypted_a_shares = encryption_manager
            .encrypt_shares(&a_shares, "mask1", "mask2")
            .unwrap();
        let encrypted_b_shares = encryption_manager
            .encrypt_shares(&b_shares, "mask3", "mask4")
            .unwrap();

        let ab_result_shares = processor
            .multiply_encrypted_shares(&encrypted_a_shares, &encrypted_b_shares, &mut rng)
            .unwrap();

        // Step 2: calculate (a * b) * c
        let c_shares =
            create_shares_with_coordinates(secret_c, threshold, &x_coords, &mut rng).unwrap();
        let encrypted_c_shares = encryption_manager
            .encrypt_shares(&c_shares, "mask1", "mask2")
            .unwrap();
        let encrypted_ab_shares = encryption_manager
            .encrypt_shares(&ab_result_shares, "mask3", "mask4")
            .unwrap();

        let final_result_shares = processor
            .multiply_encrypted_shares(&encrypted_ab_shares, &encrypted_c_shares, &mut rng)
            .unwrap();

        // Verify result
        let recovered_result = SecretSharing::recover(&final_result_shares[0..threshold]).unwrap();

        // Calculate expected result: (3 * 4) * 5 = 60
        let a_field = FieldElement::from_u64(secret_a);
        let b_field = FieldElement::from_u64(secret_b);
        let c_field = FieldElement::from_u64(secret_c);
        let expected: FieldElement = (a_field * b_field) * c_field;
        let expected_u64 = expected.to_u64();

        assert_eq!(
            recovered_result, expected_u64,
            "Chain multiplication result should be correct"
        );
    }

    // Removed duplicate test_simple_api_function

    #[test]
    fn test_error_handling() {
        let mut rng = create_test_rng();
        let encryption_manager = create_test_encryption_manager();

        // Test invalid threshold
        let result = EncryptedBeaverProcessor::new(
            encryption_manager.clone(),
            0, // Invalid threshold
            5,
            32,
            &mut rng,
        );
        assert!(result.is_err(), "Should fail with invalid threshold");

        // Test threshold greater than total shares
        let result = EncryptedBeaverProcessor::new(
            encryption_manager.clone(),
            6, // Threshold greater than total shares
            5,
            32,
            &mut rng,
        );
        assert!(result.is_err(), "Should fail when threshold > total_shares");

        // Test mismatched share lengths
        let x_coords = create_x_coordinates(5);
        let mut processor = EncryptedBeaverProcessor::new_with_coordinates(
            encryption_manager.clone(),
            3,
            5,
            &x_coords,
            32,
            &mut rng,
        )
        .unwrap();

        let x_shares = SecretSharing::split(10, 3, 5, &mut rng).unwrap();
        let y_shares = SecretSharing::split(20, 3, 3, &mut rng).unwrap(); // Different length

        let encrypted_x_shares = encryption_manager
            .encrypt_shares(x_shares.get_shares(), "mask1", "mask2")
            .unwrap();
        let encrypted_y_shares = encryption_manager
            .encrypt_shares(y_shares.get_shares(), "mask3", "mask4")
            .unwrap();

        let result =
            processor.multiply_encrypted_shares(&encrypted_x_shares, &encrypted_y_shares, &mut rng);
        assert!(result.is_err(), "Should fail with mismatched share lengths");
    }
}

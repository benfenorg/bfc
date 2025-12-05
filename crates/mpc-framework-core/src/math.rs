// Import necessary modules and types
use crate::{
    error::SSSError,
    field::gf64_sss::FieldElement,
    field::FieldElement as FieldElementTrait,
    secret::SecretSharing,
    types::ShareList,
};
use rand_core::RngCore;

/// Homomorphic operations module for secret sharing schemes
///
/// Homomorphic operations allow direct mathematical computations on secret shares
/// without reconstructing the original secrets. This is an important concept in
/// modern cryptography, widely used in privacy computing and secure multi-party computation.
///
/// # Mathematical Principles
///
/// For Shamir's secret sharing scheme, if we have shares of two secrets s1 and s2:
/// - s1 shares: (x₁, f₁(x₁)), (x₂, f₁(x₂)), ..., (xₙ, f₁(xₙ))  
/// - s2 shares: (x₁, f₂(x₁)), (x₂, f₂(x₂)), ..., (xₙ, f₂(xₙ))
///
/// Where f₁(0) = s1, f₂(0) = s2
///
/// ## Additive Homomorphism
/// If we compute: (x₁, f₁(x₁) + f₂(x₁)), (x₂, f₁(x₂) + f₂(x₂)), ...
/// The polynomial f₃(x) = f₁(x) + f₂(x) defined by these points satisfies: f₃(0) = f₁(0) + f₂(0) = s1 + s2
///
/// ## Subtractive Homomorphism  
/// Similarly: f₄(x) = f₁(x) - f₂(x) satisfies: f₄(0) = s1 - s2
///
/// ## Constant Multiplication Homomorphism
/// f₅(x) = c × f₁(x) satisfies: f₅(0) = c × s1
///
/// This enables linear computations in the ciphertext domain, which is the foundation of many privacy-preserving protocols.
///
/// Homomorphic operations struct for secret sharing
///
/// Provides functionality for homomorphic operations on secret shares including
/// addition, subtraction, constant multiplication, etc.
pub struct HomomorphicOperations;

impl HomomorphicOperations {
    /// Perform homomorphic addition on two secret shares
    ///
    /// # Parameters
    /// * `shares_a` - Shares of the first secret
    /// * `shares_b` - Shares of the second secret
    ///
    /// # Algorithm Principle
    /// For each pair of share points with the same x coordinate (x, y₁) and (x, y₂),
    /// compute new share points (x, y₁ + y₂).
    ///
    /// The secret reconstructed from the result shares equals the sum of the original two secrets.
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - The added shares
    /// * `Err(SSSError)` - Share mismatch or other errors
    ///
    /// # Example
    /// ```
    /// // secret_a = 100, secret_b = 200
    /// // After addition, shares reconstruct to secret_c = 300
    /// ```
    pub fn add_shares(shares_a: &ShareList, shares_b: &ShareList) -> Result<ShareList, SSSError> {
        // Validate input parameters
        if shares_a.is_empty() || shares_b.is_empty() {
            return Err(SSSError::InvalidParameters(
                "Both share sets must be non-empty".into(),
            ));
        }

        if shares_a.len() != shares_b.len() {
            return Err(SSSError::InvalidParameters(
                "Share sets must have the same length".into(),
            ));
        }

        // Verify x coordinate matching
        for (i, ((x_a, _), (x_b, _))) in shares_a.iter().zip(shares_b.iter()).enumerate() {
            if FieldElementTrait::to_u64(x_a) != FieldElementTrait::to_u64(x_b) {
                return Err(SSSError::InvalidParameters(format!(
                    "X coordinates must match at position {i}"
                )));
            }
        }

        // Perform homomorphic addition: add y coordinates of each share point
        let result_shares: Vec<(FieldElement, FieldElement)> = shares_a
            .iter()
            .zip(shares_b.iter())
            .map(|((x_a, y_a), (_, y_b))| {
                // Add y coordinates: y_result = y_a + y_b
                let y_result = *y_a + *y_b;
                (*x_a, y_result)
            })
            .collect();

        Ok(result_shares)
    }

    /// Perform homomorphic subtraction on two secret shares
    ///
    /// # Parameters
    /// * `shares_a` - Shares of the minuend
    /// * `shares_b` - Shares of the subtrahend
    ///
    /// # Algorithm Principle
    /// For each pair of share points with the same x coordinate (x, y₁) and (x, y₂),
    /// compute new share points (x, y₁ - y₂).
    ///
    /// The secret reconstructed from the result shares equals the difference of the original two secrets.
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - The subtracted shares
    /// * `Err(SSSError)` - Share mismatch or other errors
    pub fn subtract_shares(
        shares_a: &[(FieldElement, FieldElement)],
        shares_b: &[(FieldElement, FieldElement)],
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Validate input parameters
        if shares_a.is_empty() || shares_b.is_empty() {
            return Err(SSSError::InvalidParameters(
                "Both share sets must be non-empty".into(),
            ));
        }

        if shares_a.len() != shares_b.len() {
            return Err(SSSError::InvalidParameters(
                "Share sets must have the same length".into(),
            ));
        }

        // Verify x coordinate matching
        for (i, ((x_a, _), (x_b, _))) in shares_a.iter().zip(shares_b.iter()).enumerate() {
            if FieldElementTrait::to_u64(x_a) != FieldElementTrait::to_u64(x_b) {
                return Err(SSSError::InvalidParameters(format!(
                    "X coordinates must match at position {i}"
                )));
            }
        }

        // Perform homomorphic subtraction: subtract y coordinates of each share point
        let result_shares: Vec<(FieldElement, FieldElement)> = shares_a
            .iter()
            .zip(shares_b.iter())
            .map(|((x_a, y_a), (_, y_b))| {
                // Subtract y coordinates: y_result = y_a - y_b
                let y_result = *y_a - *y_b;
                (*x_a, y_result)
            })
            .collect();

        Ok(result_shares)
    }

    /// Perform constant multiplication on secret shares
    ///
    /// # Parameters
    /// * `shares` - Original shares
    /// * `constant` - Multiplier constant
    ///
    /// # Algorithm Principle
    /// For each share point (x, y), compute new share point (x, c × y).
    /// The secret reconstructed from the result shares equals the original secret multiplied by the constant.
    ///
    /// This operation preserves the polynomial degree, so the threshold remains unchanged.
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Shares after multiplication
    /// * `Err(SSSError)` - Error when input is invalid
    pub fn multiply_by_constant(
        shares: &[(FieldElement, FieldElement)],
        constant: u64,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        if shares.is_empty() {
            return Err(SSSError::InvalidParameters(
                "Share set must be non-empty".into(),
            ));
        }

        let constant_fe: FieldElement = FieldElementTrait::from_u64(constant);

        // Perform constant multiplication: multiply y coordinate of each share point by constant
        let result_shares: Vec<(FieldElement, FieldElement)> = shares
            .iter()
            .map(|(x, y)| {
                // Multiply y coordinate by constant: y_result = c × y
                let y_result = constant_fe * *y;
                (*x, y_result)
            })
            .collect();

        Ok(result_shares)
    }

    /// Perform constant addition on secret shares
    ///
    /// # Parameters
    /// * `shares` - Original shares
    /// * `constant` - Addend constant
    /// * `threshold` - Threshold value (used to generate new polynomial)
    /// * `rng` - Random number generator
    ///
    /// # Algorithm Principle
    /// Constant addition is more complex because we cannot simply add the constant to each share's y coordinate.
    /// We need to generate a new constant polynomial with the constant term being the number to add, then perform share addition.
    ///
    /// Steps:
    /// 1. Create a (1, n) secret sharing of the constant (threshold 1, so it's a constant polynomial)
    /// 2. Perform homomorphic addition between this sharing and the original shares
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Shares after addition
    /// * `Err(SSSError)` - Error when operation fails
    pub fn add_constant<R: RngCore>(
        shares: &ShareList,
        constant: u64,
        _threshold: usize,
        rng: &mut R,
    ) -> Result<ShareList, SSSError> {
        if shares.is_empty() {
            return Err(SSSError::InvalidParameters(
                "Share set must be non-empty".into(),
            ));
        }

        // Create secret sharing of the constant
        // Use threshold 1, so the polynomial is a constant polynomial f(x) = constant
        let constant_sharing = SecretSharing::split(constant, 1, shares.len(), rng)?;
        let _constant_shares = constant_sharing.get_shares();

        // We need to evaluate the constant polynomial at the same x coordinates
        let matched_constant_shares: Vec<(FieldElement, FieldElement)> = shares
            .iter()
            .map(|(x, _)| {
                // For constant polynomial f(x) = c, the value at any point is c
                let constant_fe: FieldElement = FieldElementTrait::from_u64(constant);
                (*x, constant_fe)
            })
            .collect();

        // Perform homomorphic addition
        Self::add_shares(shares, &matched_constant_shares)
    }

    /// Verify the correctness of homomorphic operations
    ///
    /// # Parameters
    /// * `original_secret_a` - First original secret
    /// * `original_secret_b` - Second original secret
    /// * `result_shares` - Shares after operation
    /// * `expected_result` - Expected operation result
    /// * `threshold` - Threshold value required for reconstruction
    ///
    /// # Returns
    /// * `Ok(bool)` - Whether verification passed
    /// * `Err(SSSError)` - Error during verification process
    pub fn verify_homomorphic_operation(
        _original_secret_a: u64,
        _original_secret_b: u64,
        result_shares: &[(FieldElement, FieldElement)],
        expected_result: u64,
        threshold: usize,
    ) -> Result<bool, SSSError> {
        // Recover secret from result shares
        let recovered_result = crate::secret::SecretSharing::recover(&result_shares[0..threshold])?;

        // Verify if the recovered result matches the expected result
        let verification_passed = recovered_result == expected_result;

        Ok(verification_passed)
    }

    /// Perform secure multiplication using Beaver triples
    ///
    /// # Parameters
    /// * `x_shares` - Shares of the first secret
    /// * `y_shares` - Shares of the second secret
    /// * `beaver_triple` - Beaver triple
    ///
    /// # Algorithm Principle
    /// Use Beaver triple (a, b, c) where c = a * b to compute x * y:
    /// 1. Calculate d = x - a and e = y - b
    /// 2. Recover values of d and e (randomized, can be safely revealed)
    /// 3. Calculate x * y = c + d * b + e * a + d * e
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Shares of multiplication result
    /// * `Err(SSSError)` - Error when computation fails
    pub fn multiply_with_beaver_triple(
        x_shares: &[(FieldElement, FieldElement)],
        y_shares: &[(FieldElement, FieldElement)],
        beaver_triple: &crate::beaver::BeaverTriple,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        crate::beaver::BeaverMultiplication::multiply_with_beaver(x_shares, y_shares, beaver_triple)
    }
}

/// Enhanced homomorphic operations supporting XOR encryption
///
/// Provides direct homomorphic operation support for encrypted secret shares
pub struct EncryptedAwareHomomorphicOperations;

impl EncryptedAwareHomomorphicOperations {
    /// Perform homomorphic addition on encrypted secret shares (with automatic decryption)
///
/// # Arguments
/// * `encrypted_shares_a` - First encrypted secret share list
/// * `encrypted_shares_b` - Second encrypted secret share list
/// * `encryption_manager` - XOR encryption manager
///
/// # Algorithm Flow
/// 1. Automatically decrypt input shares
/// 2. Execute standard homomorphic addition
/// 3. Return computation result (unencrypted)
///
/// # Returns
/// * `Ok(Vec<(FieldElement, FieldElement)>)` - Addition result
/// * `Err(SSSError)` - Operation failure
    pub fn add_encrypted_shares_with_manager(
        encrypted_shares_a: &[crate::encryption::EncryptedShare],
        encrypted_shares_b: &[crate::encryption::EncryptedShare],
        encryption_manager: &crate::encryption::XorEncryptionManager,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Decrypt shares
        let decrypted_a = encryption_manager.decrypt_shares(encrypted_shares_a)?;
        let decrypted_b = encryption_manager.decrypt_shares(encrypted_shares_b)?;

        // Execute homomorphic addition
        HomomorphicOperations::add_shares(&decrypted_a, &decrypted_b)
    }

    /// Perform homomorphic subtraction on encrypted secret shares (with automatic decryption)
///
/// # Arguments
/// * `encrypted_shares_a` - Minuend encrypted shares
/// * `encrypted_shares_b` - Subtrahend encrypted shares
/// * `encryption_manager` - XOR encryption manager
///
/// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Subtraction result
    /// * `Err(SSSError)` - Operation failure
    pub fn subtract_encrypted_shares_with_manager(
        encrypted_shares_a: &[crate::encryption::EncryptedShare],
        encrypted_shares_b: &[crate::encryption::EncryptedShare],
        encryption_manager: &crate::encryption::XorEncryptionManager,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Decrypt shares
        let decrypted_a = encryption_manager.decrypt_shares(encrypted_shares_a)?;
        let decrypted_b = encryption_manager.decrypt_shares(encrypted_shares_b)?;

        // Execute homomorphic subtraction
        HomomorphicOperations::subtract_shares(&decrypted_a, &decrypted_b)
    }

    /// Perform constant multiplication on encrypted secret shares (with automatic decryption)
    ///
    /// # Parameters
    /// * `encrypted_shares` - Encrypted shares
    /// * `constant` - Multiplier constant
    /// * `encryption_manager` - XOR encryption manager
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Multiplication result
    /// * `Err(SSSError)` - Operation failure
    pub fn multiply_encrypted_shares_by_constant(
        encrypted_shares: &[crate::encryption::EncryptedShare],
        constant: u64,
        encryption_manager: &crate::encryption::XorEncryptionManager,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        // Decrypt shares
        let decrypted_shares = encryption_manager.decrypt_shares(encrypted_shares)?;

        // Execute constant multiplication
        HomomorphicOperations::multiply_by_constant(&decrypted_shares, constant)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    /// Create deterministic random number generator for repeatable tests
    fn create_test_rng() -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(42)
    }

    /// Test helper function: create shares for two secrets on the same x coordinates
    fn create_compatible_shares_for_test<R: RngCore>(
        secret_a: u64,
        secret_b: u64,
        threshold: usize,
        total_shares: usize,
        rng: &mut R,
    ) -> crate::types::EncryptedAwareSharePairResult {
        use crate::field::gf64_sss::random_element;
        use crate::poly::Polynomial;

        // Generate same x coordinates
        let mut x_coords = Vec::new();
        for _ in 0..total_shares {
            x_coords.push(random_element(rng));
        }

        // Create polynomial for first secret and evaluate
        let poly_a = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(secret_a), rng);
        let shares_a: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_a.evaluate(&x))).collect();

        // Create polynomial for second secret and evaluate at same x coordinates
        let poly_b = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(secret_b), rng);
        let shares_b: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_b.evaluate(&x))).collect();

        Ok((shares_a, shares_b))
    }

    /// Create compatible test shares with predefined coordinates
///
/// # Error Cause:
/// The original create_compatible_shares_for_test generates new random coordinates each time, causing coordinate mismatch with Beaver triples
    /// The original create_compatible_shares_for_test generates new random coordinates each time,
    /// causing coordinate mismatch with Beaver triples
    fn create_compatible_shares_with_coords<R: RngCore>(
        secret_a: u64,
        secret_b: u64,
        threshold: usize,
        x_coords: &[FieldElement],
        rng: &mut R,
    ) -> crate::types::EncryptedAwareSharePairResult {
        use crate::poly::Polynomial;

        // Create polynomial for first secret and evaluate
        let poly_a = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(secret_a), rng);
        let shares_a: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_a.evaluate(&x))).collect();

        // Create polynomial for second secret and evaluate at same x coordinates
        let poly_b = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(secret_b), rng);
        let shares_b: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_b.evaluate(&x))).collect();

        Ok((shares_a, shares_b))
    }

    #[test]
    fn test_homomorphic_addition_basic() {
        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 5;

        let secret_a = 100u64;
        let secret_b = 200u64;
        let expected_sum = secret_a + secret_b;

        // Create shares for two secrets on same x coordinates
        let (shares_a, shares_b) = create_compatible_shares_for_test(
            secret_a,
            secret_b,
            threshold,
            total_shares,
            &mut rng,
        )
        .expect("Creating compatible shares should succeed");

        // Perform homomorphic addition
        let sum_shares = HomomorphicOperations::add_shares(&shares_a, &shares_b)
            .expect("Homomorphic addition should succeed");

        // Verify result
        let recovered_sum =
            SecretSharing::recover(&sum_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_sum, expected_sum,
            "Homomorphic addition should produce correct result"
        );

        // Use verification function
        let verification = HomomorphicOperations::verify_homomorphic_operation(
            secret_a,
            secret_b,
            &sum_shares,
            expected_sum,
            threshold,
        )
        .expect("Verification should succeed");

        assert!(verification, "Verification should pass");
    }

    #[test]
    fn test_homomorphic_subtraction_basic() {
        let mut rng = create_test_rng();
        let threshold = 2;
        let total_shares = 4;

        let secret_a = 1000u64;
        let secret_b = 300u64;
        let expected_diff = secret_a - secret_b; // 700

        // Create shares for two secrets on same x coordinates
        let (shares_a, shares_b) = create_compatible_shares_for_test(
            secret_a,
            secret_b,
            threshold,
            total_shares,
            &mut rng,
        )
        .expect("Creating compatible shares should succeed");

        // Perform homomorphic subtraction
        let diff_shares = HomomorphicOperations::subtract_shares(&shares_a, &shares_b)
            .expect("Homomorphic subtraction should succeed");

        // Verify result
        let recovered_diff =
            SecretSharing::recover(&diff_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_diff, expected_diff,
            "Homomorphic subtraction should produce correct result"
        );

        // Use verification function
        let verification = HomomorphicOperations::verify_homomorphic_operation(
            secret_a,
            secret_b,
            &diff_shares,
            expected_diff,
            threshold,
        )
        .expect("Verification should succeed");

        assert!(verification, "Verification should pass");
    }

    #[test]
    fn test_constant_multiplication() {
        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 6;

        let secret = 50u64;
        let constant = 7u64;
        let expected_product = secret * constant; // 350

        // Create shares for the secret
        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");
        let shares = sharing.get_shares();

        // Perform constant multiplication
        let product_shares = HomomorphicOperations::multiply_by_constant(shares, constant)
            .expect("Constant multiplication should succeed");

        // Verify result
        let recovered_product =
            SecretSharing::recover(&product_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_product, expected_product,
            "Constant multiplication should produce correct result"
        );
    }

    #[test]
    fn test_constant_addition() {
        let mut rng = create_test_rng();
        let threshold = 2;
        let total_shares = 3;

        let secret = 123u64;
        let constant = 77u64;
        let expected_sum = secret + constant; // 200

        // Create shares for the secret
        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");
        let shares = sharing.get_shares();

        // Perform constant addition
        let sum_shares =
            HomomorphicOperations::add_constant(&shares.to_vec(), constant, threshold, &mut rng)
                .expect("Constant addition should succeed");

        // Verify result
        let recovered_sum =
            SecretSharing::recover(&sum_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_sum, expected_sum,
            "Constant addition should produce correct result"
        );
    }


    #[test]
    fn test_complex_arithmetic_expression() {
        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 5;

        // Calculate expression: (a + b) - (c * 2) + 100
        let secret_a = 500u64;
        let secret_b = 300u64;
        let secret_c = 150u64;
        let expected_result = (secret_a + secret_b) - (secret_c * 2) + 100; // 500

        // Generate same x coordinates for all shares
        use crate::field::gf64_sss::random_element;
        use crate::poly::Polynomial;
        let mut x_coords = Vec::new();
        for _ in 0..total_shares {
            x_coords.push(random_element(&mut rng));
        }

        // Create shares for three secrets on same x coordinates
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

        let poly_c = Polynomial::new(
            threshold - 1,
            FieldElementTrait::from_u64(secret_c),
            &mut rng,
        );
        let shares_c: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_c.evaluate(&x))).collect();

        // Step 1: a + b
        let ab_sum_shares =
            HomomorphicOperations::add_shares(&shares_a, &shares_b).expect("A + B should succeed");

        // Step 2: c * 2
        let c_doubled_shares = HomomorphicOperations::multiply_by_constant(&shares_c, 2)
            .expect("C * 2 should succeed");

        // Step 3: (a + b) - (c * 2)
        let intermediate_shares =
            HomomorphicOperations::subtract_shares(&ab_sum_shares, &c_doubled_shares)
                .expect("(A + B) - (C * 2) should succeed");

        // Step 4: + 100
        let final_shares =
            HomomorphicOperations::add_constant(&intermediate_shares, 100, threshold, &mut rng)
                .expect("+ 100 should succeed");

        // Verify final result
        let recovered_result = SecretSharing::recover(&final_shares[0..threshold])
            .expect("Final recovery should succeed");

        assert_eq!(
            recovered_result, expected_result,
            "Complex expression should produce correct result: {recovered_result} != {expected_result}"
        );
    }

    #[test]
    fn test_error_handling_mismatched_shares() {
        let mut rng = create_test_rng();

        // Create shares with different lengths
        let sharing_a = SecretSharing::split(100, 2, 3, &mut rng).expect("Split A should succeed");
        let sharing_b = SecretSharing::split(200, 2, 5, &mut rng).expect("Split B should succeed");

        let shares_a = sharing_a.get_shares();
        let shares_b = sharing_b.get_shares();

        // Addition attempt should fail
        let result = HomomorphicOperations::add_shares(&shares_a.to_vec(), &shares_b.to_vec());
        assert!(
            result.is_err(),
            "Mismatched share lengths should cause error"
        );
    }

    #[test]
    fn test_error_handling_empty_shares() {
        let empty_shares: Vec<(FieldElement, FieldElement)> = vec![];
        let some_shares = vec![(
            FieldElementTrait::from_u64(1),
            FieldElementTrait::from_u64(100),
        )];

        // Empty share addition should fail
        let result = HomomorphicOperations::add_shares(&empty_shares, &some_shares);
        assert!(result.is_err(), "Empty shares should cause error");

        // Constant multiplication on empty shares should fail
        let result = HomomorphicOperations::multiply_by_constant(&empty_shares, 5);
        assert!(result.is_err(), "Empty shares should cause error");
    }

    #[test]
    fn test_zero_operations() {
        let mut rng = create_test_rng();
        let threshold = 2;
        let total_shares = 3;

        let secret = 456u64;

        // Create secret shares
        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");
        let shares = sharing.get_shares();

        // Multiply by 0 should give 0
        let zero_shares = HomomorphicOperations::multiply_by_constant(shares, 0)
            .expect("Multiply by zero should succeed");
        let recovered_zero =
            SecretSharing::recover(&zero_shares[0..threshold]).expect("Recovery should succeed");
        assert_eq!(recovered_zero, 0, "Multiply by zero should give zero");

        // Add 0 should remain unchanged
        let unchanged_shares =
            HomomorphicOperations::add_constant(&shares.to_vec(), 0, threshold, &mut rng)
                .expect("Add zero should succeed");
        let recovered_unchanged = SecretSharing::recover(&unchanged_shares[0..threshold])
            .expect("Recovery should succeed");
        assert_eq!(
            recovered_unchanged, secret,
            "Add zero should not change value"
        );
    }

    // Test encrypted-aware homomorphic operations
    #[test]
    fn test_encrypted_aware_homomorphic_addition() {
        use std::env;
        use std::fs;

        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 5;
        let secret_a = 150u64;
        let secret_b = 250u64;
        let expected_sum = secret_a + secret_b;

        // Create temporary test configuration file
        let config_content = r#"{
            "test_x_mask": {"key": "TEST_X_MASK", "mask": "0x1234567890ABCDEF"},
            "test_y_mask": {"key": "TEST_Y_MASK", "mask": "0xFEDCBA0987654321"}
        }"#;

        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test_encrypted_config.json");
        fs::write(&config_path, config_content).expect("Write config should succeed");

        // Load encryption manager
        let encryption_manager =
            crate::encryption::XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
                .expect("Load encryption manager should succeed");

        // Create compatible shares
        let (shares_a, shares_b) = create_compatible_shares_for_test(
            secret_a,
            secret_b,
            threshold,
            total_shares,
            &mut rng,
        )
        .expect("Creating compatible shares should succeed");

        // Encrypt shares
        let encrypted_shares_a = encryption_manager
            .encrypt_shares(&shares_a, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption A should succeed");

        let encrypted_shares_b = encryption_manager
            .encrypt_shares(&shares_b, "TEST_X_MASK", "TEST_Y_MASK")
            .expect("Encryption B should succeed");

        // Execute encrypted-aware homomorphic addition
        let result_shares = EncryptedAwareHomomorphicOperations::add_encrypted_shares_with_manager(
            &encrypted_shares_a,
            &encrypted_shares_b,
            &encryption_manager,
        )
        .expect("Encrypted homomorphic addition should succeed");

        // Verify result
        let recovered_sum =
            SecretSharing::recover(&result_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_sum, expected_sum,
            "Encrypted homomorphic addition should produce correct result"
        );

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_encrypted_aware_homomorphic_subtraction() {
        use std::env;
        use std::fs;

        let mut rng = create_test_rng();
        let threshold = 2;
        let total_shares = 4;
        let secret_a = 800u64;
        let secret_b = 300u64;
        let expected_diff = secret_a - secret_b;

        // Create temporary test configuration file
        let config_content = r#"{
            "coord_mask": {"key": "COORD_MASK", "mask": "0xAAAABBBBCCCCDDDD"},
            "val_mask": {"key": "VALUE_MASK", "mask": "0x1111222233334444"}
        }"#;

        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test_sub_config.json");
        fs::write(&config_path, config_content).expect("Write config should succeed");

        // Load encryption manager
        let encryption_manager =
            crate::encryption::XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
                .expect("Load encryption manager should succeed");

        // Create compatible shares
        let (shares_a, shares_b) = create_compatible_shares_for_test(
            secret_a,
            secret_b,
            threshold,
            total_shares,
            &mut rng,
        )
        .expect("Creating compatible shares should succeed");

        // Encrypt shares
        let encrypted_shares_a = encryption_manager
            .encrypt_shares(&shares_a, "COORD_MASK", "VALUE_MASK")
            .expect("Encryption A should succeed");

        let encrypted_shares_b = encryption_manager
            .encrypt_shares(&shares_b, "COORD_MASK", "VALUE_MASK")
            .expect("Encryption B should succeed");

        // Execute encrypted-aware homomorphic subtraction
        let result_shares =
            EncryptedAwareHomomorphicOperations::subtract_encrypted_shares_with_manager(
                &encrypted_shares_a,
                &encrypted_shares_b,
                &encryption_manager,
            )
            .expect("Encrypted homomorphic subtraction should succeed");

        // Verify result
        let recovered_diff =
            SecretSharing::recover(&result_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_diff, expected_diff,
            "Encrypted homomorphic subtraction should produce correct result"
        );

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_encrypted_aware_constant_multiplication() {
        use std::env;
        use std::fs;

        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 5;
        let secret = 75u64;
        let constant = 4u64;
        let expected_product = secret * constant;

        // Create temporary test configuration file
        let config_content = r#"{
            "mul_x_mask": {"key": "MUL_X_MASK", "mask": "0x5555666677778888"},
            "mul_y_mask": {"key": "MUL_Y_MASK", "mask": "0x9999AAAABBBBCCCC"}
        }"#;

        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test_mul_config.json");
        fs::write(&config_path, config_content).expect("Write config should succeed");

        // Load encryption manager
        let encryption_manager =
            crate::encryption::XorEncryptionManager::load_from_file(config_path.to_str().unwrap())
                .expect("Load encryption manager should succeed");

        // Create secret shares
        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");
        let shares = sharing.get_shares();

        // Encrypt shares
        let encrypted_shares = encryption_manager
            .encrypt_shares(shares, "MUL_X_MASK", "MUL_Y_MASK")
            .expect("Encryption should succeed");

        // Execute encrypted-aware constant multiplication
        let result_shares =
            EncryptedAwareHomomorphicOperations::multiply_encrypted_shares_by_constant(
                &encrypted_shares,
                constant,
                &encryption_manager,
            )
            .expect("Encrypted constant multiplication should succeed");

        // Verify result
        let recovered_product =
            SecretSharing::recover(&result_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_product, expected_product,
            "Encrypted constant multiplication should produce correct result"
        );

        // Clean up test file
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_verify_homomorphic_operation_comprehensive() {
        let mut rng = create_test_rng();
        let threshold = 4;
        let total_shares = 7;

        // Test verification of multiple operations
        let test_cases = [
            (100u64, 50u64, 150u64), // Addition
            (200u64, 75u64, 125u64), // Subtraction
            (25u64, 12u64, 37u64),   // Simple addition
        ];

        for (i, (a, b, expected)) in test_cases.iter().enumerate() {
            let (shares_a, shares_b) =
                create_compatible_shares_for_test(*a, *b, threshold, total_shares, &mut rng)
                    .expect("Creating compatible shares should succeed");

            let result_shares = if expected == &(a + b) {
                HomomorphicOperations::add_shares(&shares_a, &shares_b)
            } else {
                HomomorphicOperations::subtract_shares(&shares_a, &shares_b)
            }
            .expect("Operation should succeed");

            // Test verification function
            let verification = HomomorphicOperations::verify_homomorphic_operation(
                *a,
                *b,
                &result_shares,
                *expected,
                threshold,
            )
            .expect("Verification should succeed");

            assert!(verification, "Test case {i} should pass verification");

            // Test wrong expected value
            let wrong_expected = expected + 1;
            let wrong_verification = HomomorphicOperations::verify_homomorphic_operation(
                *a,
                *b,
                &result_shares,
                wrong_expected,
                threshold,
            )
            .expect("Verification should succeed");

            assert!(
                !wrong_verification,
                "Wrong expected value should fail verification for test case {i}"
            );
        }
    }


    #[test]
    fn test_homomorphic_operations_stress_test() {
        let mut rng = create_test_rng();
        let threshold = 5;
        let total_shares = 10;

        // Test large number operations
        let large_secret_a = u32::MAX as u64;
        let large_secret_b = u16::MAX as u64;

        let (shares_a, shares_b) = create_compatible_shares_for_test(
            large_secret_a,
            large_secret_b,
            threshold,
            total_shares,
            &mut rng,
        )
        .expect("Creating large compatible shares should succeed");

        // Addition
        let add_result = HomomorphicOperations::add_shares(&shares_a, &shares_b)
            .expect("Large number addition should succeed");
        let recovered_add =
            SecretSharing::recover(&add_result[0..threshold]).expect("Recovery should succeed");
        assert_eq!(recovered_add, large_secret_a + large_secret_b);

        // Subtraction
        let sub_result = HomomorphicOperations::subtract_shares(&shares_a, &shares_b)
            .expect("Large number subtraction should succeed");
        let recovered_sub =
            SecretSharing::recover(&sub_result[0..threshold]).expect("Recovery should succeed");
        assert_eq!(recovered_sub, large_secret_a - large_secret_b);

        // Large constant multiplication
        let large_constant = 1000u64;
        let mul_result = HomomorphicOperations::multiply_by_constant(&shares_a, large_constant)
            .expect("Large constant multiplication should succeed");
        let recovered_mul =
            SecretSharing::recover(&mul_result[0..threshold]).expect("Recovery should succeed");

        // Note: There may be finite field modular arithmetic effects here
        let expected_mul = (large_secret_a as u128 * large_constant as u128) as u64;
        // Verify in finite field
        assert_eq!(recovered_mul, expected_mul);
    }

    #[test]
    fn test_x_coordinate_mismatch_error() {
        let mut rng = create_test_rng();

        // Create two share groups with different x coordinates
        let sharing_a = SecretSharing::split(100, 2, 3, &mut rng).expect("Split A should succeed");
        let sharing_b = SecretSharing::split(200, 2, 3, &mut rng).expect("Split B should succeed");

        let shares_a = sharing_a.get_shares();
        let shares_b = sharing_b.get_shares();

        // Addition with mismatched x coordinates should fail
        let result = HomomorphicOperations::add_shares(&shares_a.to_vec(), &shares_b.to_vec());
        assert!(
            result.is_err(),
            "Mismatched x coordinates should cause error"
        );

        // Check error message
        match result.unwrap_err() {
            SSSError::InvalidParameters(msg) => {
                assert!(
                    msg.contains("X coordinates must match"),
                    "Error should mention coordinate mismatch"
                );
            }
            _ => panic!("Should return InvalidParameters error"),
        }
    }

    #[test]
    fn test_beaver_triple_multiplication_integration() {
        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 5;

        let x_secret = 23u64;
        let y_secret = 29u64;
        // Calculate expected value using finite field multiplication
        let x_field = FieldElement::from_u64(x_secret);
        let y_field = FieldElement::from_u64(y_secret);
        let expected_product = (x_field * y_field).to_u64();

        // Create compatible shares
        let (x_shares, y_shares) = create_compatible_shares_for_test(
            x_secret,
            y_secret,
            threshold,
            total_shares,
            &mut rng,
        )
        .expect("Creating compatible shares should succeed");

        // Create Beaver triple
        let beaver_triple = crate::beaver::BeaverTriple::new_with_coordinates(
            rng.next_u64(),
            rng.next_u64(),
            &x_shares.iter().map(|(x, _)| *x).collect::<Vec<_>>(),
            threshold,
            &mut rng,
        )
        .expect("Beaver triple creation should succeed");

        // Use integrated method of HomomorphicOperations
        let product_shares = HomomorphicOperations::multiply_with_beaver_triple(
            &x_shares,
            &y_shares,
            &beaver_triple,
        )
        .expect("Beaver multiplication should succeed");

        // Verify result
        let recovered_product =
            SecretSharing::recover(&product_shares[0..threshold]).expect("Recovery should succeed");

        assert_eq!(
            recovered_product, expected_product,
            "Integrated Beaver multiplication should produce correct result"
        );
    }


    #[test]
    fn test_beaver_vs_direct_multiplication_consistency() {
        let mut rng = create_test_rng();
        let threshold = 2;
        let total_shares = 3;

        let x = 7u64;
        let y = 11u64;
        // Calculate expected value using finite field multiplication
        let expected = (FieldElement::from_u64(x) * FieldElement::from_u64(y)).to_u64();

        // Generate unified x coordinates for both methods
        use crate::field::gf64_sss::random_element;
        let mut x_coords = Vec::new();
        for _ in 0..total_shares {
            x_coords.push(random_element(&mut rng));
        }

        // Method 1: Direct multiplication with constants
        // Error cause: Different coordinate systems caused inconsistent results
        use crate::poly::Polynomial;
        let poly_x = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(x), &mut rng);
        let x_shares_direct: Vec<(FieldElement, FieldElement)> = x_coords
            .iter()
            .map(|&coord| (coord, poly_x.evaluate(&coord)))
            .collect();

        let direct_result = HomomorphicOperations::multiply_by_constant(&x_shares_direct, y)
            .expect("Direct multiplication should succeed");

        let direct_recovered = SecretSharing::recover(&direct_result[0..threshold])
            .expect("Direct recovery should succeed");

        // Method 2: Beaver triple with same coordinates
        let (x_shares, y_shares) =
            create_compatible_shares_with_coords(x, y, threshold, &x_coords, &mut rng)
                .expect("Creating compatible shares should succeed");

        let triple = crate::beaver::BeaverTriple::new_with_coordinates(
            rng.next_u64(),
            rng.next_u64(),
            &x_coords,
            threshold,
            &mut rng,
        )
        .expect("Triple creation should succeed");

        let beaver_result =
            HomomorphicOperations::multiply_with_beaver_triple(&x_shares, &y_shares, &triple)
                .expect("Beaver multiplication should succeed");

        let beaver_recovered = SecretSharing::recover(&beaver_result[0..threshold])
            .expect("Beaver recovery should succeed");

        // Verify both methods produce same result
        assert_eq!(direct_recovered, expected);
        assert_eq!(beaver_recovered, expected);
        assert_eq!(
            direct_recovered, beaver_recovered,
            "Direct and Beaver methods should produce same result"
        );
    }
}

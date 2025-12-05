// Import necessary modules and types
use crate::{
    error::SSSError,                                 // Error type definitions
    field::gf64_sss::{random_element, FieldElement}, // Finite field elements and random number generation
    field::FieldElement as FieldElementTrait,        // Finite field trait
    poly::Polynomial,                                // Polynomial operations
    types::ShareList,                                // Type aliases
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use rand_core::RngCore; // Random number generator trait

/// Main structure for Shamir's Secret Sharing scheme
///
/// Stores threshold parameters and generated secret shares
pub struct SecretSharing {
    threshold: usize,  // Minimum number of shares required to reconstruct the secret
    shares: ShareList, // Secret share points: (x-coordinate, y-coordinate)
}

impl SecretSharing {
    /// Split a secret into multiple shares
    ///
    /// # Parameters
    /// * `secret` - The 64-bit secret value to be shared
    /// * `threshold` - Minimum number of shares required to reconstruct the secret (t)
    /// * `total_shares` - Total number of shares (n)
    /// * `rng` - Cryptographically secure random number generator
    ///
    /// # Algorithm Principle
    /// 1. Use the secret as the constant term (y-intercept) of a (t-1)-degree polynomial
    /// 2. Randomly generate (t-1) coefficients to construct polynomial f(x) = secret + a₁x + a₂x² + ... + aₜ₋₁x^(t-1)
    /// 3. Select n different x values on the finite field, compute corresponding y values as shares
    /// 4. Each share is a point (x, f(x))
    ///
    /// # Returns
    /// * `Ok(SecretSharing)` - Successfully created secret sharing scheme
    /// * `Err(SSSError)` - Error when parameters are invalid
    pub fn split<R: RngCore>(
        secret: u64,
        threshold: usize,
        total_shares: usize,
        rng: &mut R,
    ) -> Result<Self, SSSError> {
        // Validate parameter legality: threshold must be within reasonable range
        if threshold == 0 || threshold > total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Threshold must be between 1 and {total_shares}, got {threshold}"
            )));
        }

        // Convert secret to finite field element
        let secret_fe = FieldElementTrait::from_u64(secret);

        // Construct (t-1)-degree polynomial with secret as constant term (value at x=0)
        let poly = Polynomial::new(threshold - 1, secret_fe, rng);

        // Generate a total of n share points
        let mut shares = Vec::with_capacity(total_shares);
        for _ in 0..total_shares {
            // Randomly select x-coordinate in finite field (ensure not 0 to avoid direct secret exposure)
            let x = random_element(rng);
            // Calculate polynomial value at x as y-coordinate
            let y = poly.evaluate(&x);
            // Store share point (x, y)
            shares.push((x, y));
        }

        Ok(Self { threshold, shares })
    }

    /// Split a secret into multiple shares using a seed
    ///
    /// # Parameters
    /// * `secret` - The 64-bit secret value to be shared
    /// * `threshold` - Minimum number of shares required to reconstruct the secret (t)
    /// * `total_shares` - Total number of shares (n)
    /// * `seed` - 64-bit seed for random number generation
    ///
    /// # Returns
    /// * `Ok(SecretSharing)` - Successfully created secret sharing scheme
    /// * `Err(SSSError)` - Error when parameters are invalid
    pub fn split_with_seed(
        secret: u64,
        threshold: usize,
        total_shares: usize,
        seed: u64,
    ) -> Result<Self, SSSError> {
        if threshold == 0 || threshold > total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Threshold must be between 1 and {total_shares}, got {threshold}"
            )));
        }

        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let secret_fe = FieldElementTrait::from_u64(secret);

        // Use new constructor to ensure coefficients are within u64 range
        let poly = Polynomial::new_with_u64_coeffs(threshold - 1, secret_fe, &mut rng);

        let mut shares = Vec::with_capacity(total_shares);
        for _ in 0..total_shares {
            // Ensure x-coordinate is also within u64 range
            let x_u64 = rng.next_u64();
            let x = FieldElementTrait::from_u64(x_u64);
            let y = poly.evaluate(&x);
            shares.push((x, y));
        }

        Ok(Self { threshold, shares })
    }

    /// Recover the original secret from shares
    ///
    /// # Parameters
    /// * `shares` - At least t secret share points
    ///
    /// # Algorithm Principle
    /// Using Lagrange interpolation:
    /// 1. Given t points (x₁,y₁), (x₂,y₂), ..., (xₜ,yₜ)
    /// 2. Reconstruct the unique (t-1)-degree polynomial f(x)
    /// 3. Calculate f(0) to get the original secret
    ///
    /// Lagrange interpolation formula:
    /// f(0) = Σᵢ yᵢ * Πⱼ≠ᵢ (0-xⱼ)/(xᵢ-xⱼ) = Σᵢ yᵢ * Πⱼ≠ᵢ (-xⱼ)/(xᵢ-xⱼ)
    ///
    /// # Returns
    /// * `Ok(u64)` - Recovered original secret
    /// * `Err(SSSError)` - Error when insufficient shares or interpolation fails
    pub fn recover(shares: &[(FieldElement, FieldElement)]) -> Result<u64, SSSError> {
        // Check if there are sufficient shares
        if shares.is_empty() {
            return Err(SSSError::InsufficientShares(
                "At least one share is required".into(),
            ));
        }

        // Use Lagrange interpolation to reconstruct polynomial value at x=0 (i.e., original secret)
        Polynomial::interpolate(shares).map(|s| FieldElementTrait::to_u64(&s))
    }

    /// Get reference to all share points
    ///
    /// # Returns
    /// Slice of share points, each element is an (x, y) coordinate pair
    pub fn get_shares(&self) -> &ShareList {
        &self.shares
    }

    /// Get threshold value
    ///
    /// # Returns
    /// Minimum number of shares required to reconstruct the secret
    pub fn get_threshold(&self) -> usize {
        self.threshold
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

    #[test]
    fn test_split_basic_functionality() {
        let mut rng = create_test_rng();
        let secret = 123456789u64;
        let threshold = 3;
        let total_shares = 5;

        let result = SecretSharing::split(secret, threshold, total_shares, &mut rng);

        assert!(result.is_ok(), "Split should succeed with valid parameters");

        let sharing = result.unwrap();
        assert_eq!(sharing.get_threshold(), threshold, "Threshold should match");
        assert_eq!(
            sharing.get_shares().len(),
            total_shares,
            "Should generate correct number of shares"
        );
    }

    #[test]
    fn test_split_invalid_parameters() {
        let mut rng = create_test_rng();
        let secret = 123u64;

        // Test threshold = 0
        let result = SecretSharing::split(secret, 0, 5, &mut rng);
        assert!(result.is_err(), "Should reject threshold = 0");

        // Test threshold > total_shares
        let result = SecretSharing::split(secret, 6, 5, &mut rng);
        assert!(result.is_err(), "Should reject threshold > total_shares");

        // Test boundary condition: threshold = total_shares (should succeed)
        let result = SecretSharing::split(secret, 5, 5, &mut rng);
        assert!(result.is_ok(), "Should accept threshold = total_shares");
    }

    #[test]
    fn test_split_with_seed_deterministic() {
        let secret = 12345u64;
        let threshold = 2;
        let total_shares = 3;
        let seed = 99;

        // First split
        let result1 = SecretSharing::split_with_seed(secret, threshold, total_shares, seed)
            .expect("Split with seed should succeed");
        let shares1 = result1.get_shares();

        // Second split using the same seed
        let result2 = SecretSharing::split_with_seed(secret, threshold, total_shares, seed)
            .expect("Split with seed should succeed again");
        let shares2 = result2.get_shares();

        // Verify that shares generated twice are identical
        assert_eq!(
            shares1, shares2,
            "Shares generated with the same seed should be identical"
        );

        // Use different seed
        let result3 = SecretSharing::split_with_seed(secret, threshold, total_shares, seed + 1)
            .expect("Split with different seed should also succeed");
        let shares3 = result3.get_shares();

        // Verify that shares generated with different seeds are different
        assert_ne!(
            shares1, shares3,
            "Shares generated with different seeds should be different"
        );
    }

    #[test]
    fn test_recover_basic_functionality() {
        let mut rng = create_test_rng();
        let original_secret = 987654321u64;
        let threshold = 3;
        let total_shares = 5;

        // Split secret
        let sharing = SecretSharing::split(original_secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");

        let shares = sharing.get_shares();

        // Recover secret using exactly threshold shares
        let recovered = SecretSharing::recover(&shares[0..threshold])
            .expect("Recovery should succeed with sufficient shares");

        assert_eq!(
            recovered, original_secret,
            "Recovered secret should match original"
        );

        // Recover secret using more than threshold shares
        let recovered = SecretSharing::recover(&shares[0..total_shares])
            .expect("Recovery should succeed with more than threshold shares");

        assert_eq!(
            recovered, original_secret,
            "Recovered secret should match with extra shares"
        );
    }

    #[test]
    fn test_recover_insufficient_shares() {
        let result = SecretSharing::recover(&[]);
        assert!(result.is_err(), "Should fail with empty shares");

        match result.unwrap_err() {
            SSSError::InsufficientShares(_) => {}
            _ => panic!("Should return InsufficientShares error"),
        }
    }

    #[test]
    fn test_recover_with_different_share_combinations() {
        let mut rng = create_test_rng();
        let original_secret = 555666777u64;
        let threshold = 4;
        let total_shares = 7;

        let sharing = SecretSharing::split(original_secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");

        let all_shares = sharing.get_shares();

        // Test different share combinations
        let test_combinations = [
            &all_shares[0..4], // First 4
            &all_shares[1..5], // Middle 4
            &all_shares[3..7], // Last 4
            &all_shares[0..6], // First 6
        ];

        for (i, shares) in test_combinations.iter().enumerate() {
            let recovered = SecretSharing::recover(shares)
                .unwrap_or_else(|_| panic!("Recovery should succeed for combination {i}"));

            assert_eq!(
                recovered, original_secret,
                "All combinations should recover the same secret (combination {i})"
            );
        }
    }

    #[test]
    fn test_get_shares() {
        let mut rng = create_test_rng();
        let secret = 111222333u64;
        let threshold = 2;
        let total_shares = 4;

        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");

        let shares = sharing.get_shares();

        assert_eq!(
            shares.len(),
            total_shares,
            "Should return correct number of shares"
        );

        // Verify that x-coordinates of all share points are not zero
        for (x, _y) in shares {
            assert_ne!(
                FieldElementTrait::to_u64(x),
                0,
                "x coordinate should not be zero"
            );
        }
    }

    #[test]
    fn test_get_threshold() {
        let mut rng = create_test_rng();
        let secret = 444555666u64;

        let test_thresholds = [1, 3, 5, 10];

        for &threshold in &test_thresholds {
            let sharing = SecretSharing::split(secret, threshold, threshold + 2, &mut rng)
                .expect("Split should succeed");

            assert_eq!(
                sharing.get_threshold(),
                threshold,
                "get_threshold should return correct value for threshold {threshold}"
            );
        }
    }

    #[test]
    fn test_edge_case_threshold_one() {
        let mut rng = create_test_rng();
        let secret = 777888999u64;

        // threshold = 1 means any single share can recover the secret
        let sharing = SecretSharing::split(secret, 1, 3, &mut rng)
            .expect("Split should succeed with threshold = 1");

        let shares = sharing.get_shares();

        // Test recovering secret with single share
        for (i, share) in shares.iter().enumerate() {
            let recovered = SecretSharing::recover(&[*share])
                .unwrap_or_else(|_| panic!("Should recover with single share {i}"));

            assert_eq!(
                recovered, secret,
                "Single share {i} should recover the secret"
            );
        }
    }

    #[test]
    fn test_different_secrets() {
        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 5;

        // Use test values within finite field range
        let test_secrets = [0, 1, 100, 999999, 4294967293u64];

        for &secret in &test_secrets {
            let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
                .expect("Split should succeed");

            let shares = sharing.get_shares();
            let recovered =
                SecretSharing::recover(&shares[0..threshold]).expect("Recovery should succeed");

            assert_eq!(
                recovered, secret,
                "Should correctly handle secret value {secret}"
            );
        }
    }

    #[test]
    fn test_share_uniqueness() {
        let mut rng = create_test_rng();
        let secret = 123456u64;
        let threshold = 3;
        let total_shares = 6;

        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed");

        let shares = sharing.get_shares();

        // Verify that x-coordinates of all share points are different
        for i in 0..shares.len() {
            for j in (i + 1)..shares.len() {
                let (x1, _) = shares[i];
                let (x2, _) = shares[j];
                assert_ne!(
                    FieldElementTrait::to_u64(&x1),
                    FieldElementTrait::to_u64(&x2),
                    "Share x-coordinates should be unique (shares {i} and {j})"
                );
            }
        }
    }

    #[test]
    fn test_large_threshold_and_shares() {
        let mut rng = create_test_rng();
        let secret = 987654321u64;
        let threshold = 50;
        let total_shares = 100;

        let sharing = SecretSharing::split(secret, threshold, total_shares, &mut rng)
            .expect("Split should succeed with large parameters");

        assert_eq!(sharing.get_threshold(), threshold);
        assert_eq!(sharing.get_shares().len(), total_shares);

        // Verify recovery functionality
        let shares = sharing.get_shares();
        let recovered = SecretSharing::recover(&shares[0..threshold])
            .expect("Recovery should succeed with large threshold");

        assert_eq!(recovered, secret, "Should handle large threshold correctly");
    }

}

/*!
# Beaver Triple Implementation - Secure Multi-Party Computation

This module implements Beaver triples, a core cryptographic primitive for efficient secure multiplication in Secure Multi-Party Computation (SMPC).

## Core Concepts

### Beaver Triples
Each Beaver triple contains three secret values (a, b, c) satisfying the relation c = a * b. These values are distributed using Shamir secret sharing.

### Secure Multiplication Protocol
To securely compute the product of two secret-shared values x and y without revealing the original values:

1. **Masking**: Compute d = x - a and e = y - b
2. **Reveal Differences**: Safely reveal d and e values (this does not leak x and y)
3. **Reconstruct Product**: Use formula x * y = c + d*b + e*a + d*e

## Mathematical Foundation

### Finite Field Arithmetic
All operations are performed in the finite field GF(p) where p = 18446744069414584321. This ensures:

- **Perfect Security**: Information-theoretic security guarantees
- **Efficient Computation**: Optimized modular arithmetic
- **Numerical Stability**: Prevents integer overflow

### Shamir Secret Sharing
Uses (threshold, n) threshold scheme:
- n parties each hold one share
- Any threshold shares can reconstruct the secret
- Fewer than threshold shares reveal no information

## Performance Optimizations

### Caching Mechanism
- **Pre-computation**: Generate triples in advance to reduce latency
- **Memory Management**: Automatically manage cache size and lifecycle

### Coordinate System
- **Unified Coordinates**: All shares use the same x-coordinates
- **Numerical Stability**: Use simple coordinates to avoid interpolation errors
- **Compatibility Guarantee**: Ensure homomorphic operations execute correctly

## Security Analysis

### Information-Theoretic Security
- d = x - a and e = y - b are randomly masked values that reveal no information about x or y

### Privacy Protection
- Original secrets x and y are never directly exposed
- Only legitimate computation parties can obtain correct results

### Collusion Resistance
- At least threshold shares are required to reconstruct any secret
- Fewer than threshold malicious parties cannot obtain useful information

## Module Structure

- `BeaverTriple`: Triple data structure and basic operations
- `BeaverTripleDistributor`: Triple generation and distribution management
- `BeaverMultiplication`: Secure multiplication protocol implementation
- `tests`: Comprehensive test suite verifying correctness and performance

*/

use crate::{
    error::SSSError, field::gf64_sss::FieldElement, field::FieldElement as FieldElementTrait,
    secret::SecretSharing,
};
use rand_core::RngCore;
use std::collections::VecDeque;

/// Beaver Triple Structure
///
/// Represents a Beaver triple (a, b, c) where c = a * b
/// Used for secure multiplication protocols
///
/// Structure Details:
/// - Each value (a, b, c) is split into multiple secret shares
/// - Uses Shamir secret sharing where each share is in (x-coordinate, y-coordinate) form  
/// - Only threshold shares are needed to reconstruct the original secret
#[derive(Debug, Clone)]
pub struct BeaverTriple {
    /// Shares of secret value a
    /// Format: Vec<(x-coordinate, y-coordinate)>
    pub a_shares: Vec<(FieldElement, FieldElement)>,

    /// Shares of secret value b  
    /// Format: Vec<(x-coordinate, y-coordinate)>
    pub b_shares: Vec<(FieldElement, FieldElement)>,

    /// Shares of secret value c where c = a * b
    /// Format: Vec<(x-coordinate, y-coordinate)>
    pub c_shares: Vec<(FieldElement, FieldElement)>,

    /// Minimum number of shares needed for reconstruction
    /// Must satisfy: 1 ≤ threshold ≤ total shares
    pub threshold: usize,
}

impl BeaverTriple {
    /// Create a new Beaver triple
    ///
    /// Creates secret shares for (a, b, c) triple using randomly generated x-coordinates
    ///
    /// # Arguments
    /// * `a` - First secret value  
    /// * `b` - Second secret value
    /// * `threshold` - Minimum shares needed for reconstruction
    /// * `total_shares` - Total number of shares  
    /// * `rng` - Random number generator
    ///
    /// # Returns
    /// * `Ok(BeaverTriple)` - Successfully created triple
    /// * `Err(SSSError)` - Error during creation
    ///
    /// # Algorithm Steps:
    /// 1. Calculate c = a × b (finite field multiplication)
    /// 2. Create Shamir secret shares for a, b, c
    /// 3. All shares use same x-coordinate system for consistency
    pub fn new<R: RngCore>(
        a: u64,
        b: u64,
        threshold: usize,
        total_shares: usize,
        rng: &mut R,
    ) -> Result<Self, SSSError> {
        // Step 1: Calculate c = a * b (finite field multiplication)
        // Bug fix: Use finite field multiplication instead of regular integer multiplication
        let a_field = FieldElement::from_u64(a);
        let b_field = FieldElement::from_u64(b);
        let c_field = a_field * b_field;
        let c = c_field.to_u64();

        // Step 2: Create secret shares for a, b, c respectively
        // SecretSharing::split automatically generates random x-coordinates and creates polynomials
        let a_sharing = SecretSharing::split(a, threshold, total_shares, rng)?;
        let b_sharing = SecretSharing::split(b, threshold, total_shares, rng)?;
        let c_sharing = SecretSharing::split(c, threshold, total_shares, rng)?;

        // Step 3: Construct BeaverTriple instance
        Ok(Self {
            a_shares: a_sharing.get_shares().to_vec(), // Extract a's share vector
            b_shares: b_sharing.get_shares().to_vec(), // Extract b's share vector
            c_shares: c_sharing.get_shares().to_vec(), // Extract c's share vector
            threshold,                                 // Save threshold parameter
        })
    }

    /// Create Beaver triple with specified coordinates
    ///
    /// Unlike new(), this method allows specifying x-coordinates to ensure multiple triples use the same coordinate system
    ///
    /// # Arguments  
    /// * `a` - First secret value
    /// * `b` - Second secret value
    /// * `x_coords` - Specified array of x-coordinates
    /// * `threshold` - Threshold value
    /// * `rng` - Random number generator (for polynomial coefficients)
    ///
    /// # Importance:
    /// All related secret shares must use the same x-coordinate system,
    /// otherwise homomorphic operations (like addition, subtraction) will fail
    pub fn new_with_coordinates<R: RngCore>(
        a: u64,
        b: u64,
        x_coords: &[FieldElement],
        threshold: usize,
        rng: &mut R,
    ) -> Result<Self, SSSError> {
        use crate::poly::Polynomial;

        // Step 1: Calculate c = a * b (finite field multiplication)
        // Bug fix: Previously used wrapping_mul which computed outside finite field, now using correct field multiplication
        let a_field = FieldElement::from_u64(a);
        let b_field = FieldElement::from_u64(b);
        let c_field = a_field * b_field;
        let c = c_field.to_u64();

        // Step 2: Create polynomial for a and evaluate at specified coordinates
        // Create random polynomial of degree (threshold-1) with constant term a
        let poly_a = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(a), rng);
        let a_shares: Vec<(FieldElement, FieldElement)> = x_coords
            .iter()
            .map(|&x| (x, poly_a.evaluate(&x))) // (x, poly_a(x))
            .collect();

        // Step 3: Create polynomial for b and evaluate at same coordinates
        let poly_b = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(b), rng);
        let b_shares: Vec<(FieldElement, FieldElement)> = x_coords
            .iter()
            .map(|&x| (x, poly_b.evaluate(&x))) // (x, poly_b(x))
            .collect();

        // Step 4: Create polynomial for c and evaluate at same coordinates
        let poly_c = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(c), rng);
        let c_shares: Vec<(FieldElement, FieldElement)> = x_coords
            .iter()
            .map(|&x| (x, poly_c.evaluate(&x))) // (x, poly_c(x))
            .collect();

        // Step 5: Return triple instance
        // Key: All shares use same x-coordinates, ensuring compatibility for subsequent homomorphic operations
        Ok(Self {
            a_shares,
            b_shares,
            c_shares,
            threshold,
        })
    }

    /// Verify the Beaver triple correctness
    ///
    /// Ensures triple validity by reconstructing secret values and verifying c = a * b
    ///
    /// # Returns
    /// * `Ok(true)` - Verification passes, triple is valid
    /// * `Ok(false)` - Verification fails, triple is invalid  
    /// * `Err(SSSError)` - Error during verification
    ///
    /// # Algorithm Steps:
    /// 1. Use first threshold shares to reconstruct original values of a, b, c
    /// 2. Verify if reconstructed c equals a * b
    /// 3. Return verification result
    pub fn verify(&self) -> Result<bool, SSSError> {
        // Step 1: Recover secret values a, b, c
        // Use Lagrange interpolation to reconstruct original secrets from shares
        let a = SecretSharing::recover(&self.a_shares[0..self.threshold])?;
        let b = SecretSharing::recover(&self.b_shares[0..self.threshold])?;
        let c = SecretSharing::recover(&self.c_shares[0..self.threshold])?;

        // Step 2: Calculate expected c value and compare (finite field multiplication)
        // Bug fix: Use finite field multiplication for verification
        let a_field = FieldElement::from_u64(a);
        let b_field = FieldElement::from_u64(b);
        let expected_c_field = a_field * b_field;
        let expected_c = expected_c_field.to_u64();

        // Step 3: Return verification result
        Ok(c == expected_c) // Verify if c equals a * b
    }

    /// Get threshold value
    /// Returns minimum number of shares needed to reconstruct secrets
    pub fn get_threshold(&self) -> usize {
        self.threshold
    }

    /// Get share count
    /// Returns total number of shares for each secret value
    pub fn get_share_count(&self) -> usize {
        self.a_shares.len()
    }
}

/// Beaver Triple Distributor
///
/// Manages generation and distribution of Beaver triples with caching for performance
///
/// # Design Goals:
/// 1. Pre-generate triples to reduce real-time computation overhead
/// 2. Use unified coordinate system to ensure compatibility
/// 3. Automatically manage cache to balance memory and performance
pub struct BeaverTripleDistributor {
    /// Secret reconstruction threshold
    threshold: usize,

    /// Total shares per secret
    total_shares: usize,

    /// Triple cache queue (FIFO)
    /// Uses VecDeque for efficient front removal and back insertion
    triple_cache: VecDeque<BeaverTriple>,

    /// Cache capacity limit
    cache_size: usize,

    /// Pre-computed x-coordinates
    /// All triples share these coordinates to ensure compatibility
    precomputed_x_coords: Vec<FieldElement>,
}

impl BeaverTripleDistributor {
    /// Create a new Beaver triple distributor
    ///
    /// Initializes distributor and pre-fills cache
    ///
    /// # Arguments
    /// * `threshold` - Threshold value
    /// * `total_shares` - Total number of shares
    /// * `cache_size` - Size of triple cache
    /// * `rng` - Random number generator
    ///
    /// # Initialization Steps:
    /// 1. Validate parameter validity (threshold must be in reasonable range)
    /// 2. Generate unified x-coordinate system for all triples
    /// 3. Pre-fill cache to provide immediately available triples
    /// 4. Return configured distributor instance
    pub fn new<R: RngCore>(
        threshold: usize,
        total_shares: usize,
        cache_size: usize,
        rng: &mut R,
    ) -> Result<Self, SSSError> {
        // Step 1: Validate parameter validity
        if threshold == 0 || threshold > total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Threshold must be between 1 and {total_shares}, got {threshold}"
            )));
        }

        // Step 2: Generate pre-computed x-coordinates
        // These coordinates will be shared by all triples, ensuring compatibility
        use crate::field::gf64_sss::random_element;
        let mut precomputed_x_coords = Vec::with_capacity(total_shares);
        for _ in 0..total_shares {
            precomputed_x_coords.push(random_element(rng));
        }

        // Step 3: Initialize distributor structure
        let mut distributor = Self {
            threshold,
            total_shares,
            triple_cache: VecDeque::with_capacity(cache_size), // Create cache queue with specified capacity
            cache_size,
            precomputed_x_coords,
        };

        // Step 4: Pre-fill cache
        // Generate triples immediately to reduce latency of first requests
        distributor.refill_cache(rng)?;

        // Step 5: Return configured distributor
        Ok(distributor)
    }

    /// Create Beaver triple distributor with specified coordinates
    ///
    /// # Arguments
    /// * `threshold` - Threshold value
    /// * `total_shares` - Total number of shares
    /// * `cache_size` - Size of triple cache
    /// * `x_coords` - Pre-specified x coordinates
    /// * `rng` - Random number generator
    pub fn new_with_coordinates<R: RngCore>(
        threshold: usize,
        total_shares: usize,
        cache_size: usize,
        x_coords: &[FieldElement],
        rng: &mut R,
    ) -> Result<Self, SSSError> {
        // Validate parameter validity
        if threshold == 0 || threshold > total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Threshold must be between 1 and {total_shares}, got {threshold}"
            )));
        }

        if x_coords.len() != total_shares {
            return Err(SSSError::InvalidParameters(format!(
                "Coordinates length {} must match total_shares {}",
                x_coords.len(),
                total_shares
            )));
        }

        // Use provided coordinates
        let precomputed_x_coords = x_coords.to_vec();

        // Initialize distributor structure
        let mut distributor = Self {
            threshold,
            total_shares,
            triple_cache: VecDeque::with_capacity(cache_size),
            cache_size,
            precomputed_x_coords,
        };

        // Pre-fill cache
        distributor.refill_cache(rng)?;

        Ok(distributor)
    }

    /// Get a single Beaver triple
    ///
    /// # Arguments
    /// * `rng` - Random number generator (for cache refill if needed)
    ///
    /// # Returns
    /// * `Ok(BeaverTriple)` - Retrieved triple
    /// * `Err(SSSError)` - Error during generation
    pub fn get_triple<R: RngCore>(&mut self, rng: &mut R) -> Result<BeaverTriple, SSSError> {
        // Refill cache if empty
        if self.triple_cache.is_empty() {
            self.refill_cache(rng)?;
        }

        self.triple_cache
            .pop_front()
            .ok_or_else(|| SSSError::InvalidParameters("Failed to generate beaver triple".into()))
    }

    /// Refill the cache
    fn refill_cache<R: RngCore>(&mut self, rng: &mut R) -> Result<(), SSSError> {
        self.triple_cache.clear();

        for _ in 0..self.cache_size {
            // Generate random a and b
            let a = rng.next_u64();
            let b = rng.next_u64();

            // Create triple with precomputed coordinates
            let triple = BeaverTriple::new_with_coordinates(
                a,
                b,
                &self.precomputed_x_coords,
                self.threshold,
                rng,
            )?;

            self.triple_cache.push_back(triple);
        }

        Ok(())
    }

    /// Get cache information
    pub fn get_cache_info(&self) -> (usize, usize) {
        (self.triple_cache.len(), self.cache_size)
    }

    /// Get configuration
    pub fn get_config(&self) -> (usize, usize) {
        (self.threshold, self.total_shares)
    }
}

/// Beaver Triple Multiplication Protocol
///
/// Implements core algorithms for secure multiplication protocol using Beaver triples
pub struct BeaverMultiplication;

impl BeaverMultiplication {
    /// Multiply secret-shared values using Beaver triple
    ///
    /// This is a core operation in secure multi-party computation, enabling multiplication without revealing original values
    ///
    /// # Protocol Steps:
    /// To securely compute product of secret values x and y using Beaver triple (a, b, c) where c = a * b:
    ///
    /// 1. Compute differences d = x - a and e = y - b
    /// 2. Reveal d and e to all parties (these values are safe)
    /// 3. Compute product x * y = c + d * b + e * a + d * e
    ///
    /// # Security Analysis:
    /// - d and e are random values that don't leak info about x, y
    /// - Only parties with Beaver triple can compute correct result
    /// - Original secrets x, y are never used directly
    ///
    /// # Arguments
    /// * `x_shares` - Shares of first operand
    /// * `y_shares` - Shares of second operand  
    /// * `beaver_triple` - Beaver triple
    ///
    /// # Returns
    /// * `Ok(Vec<(FieldElement, FieldElement)>)` - Multiplication result shares
    /// * `Err(SSSError)` - Error during computation
    ///
    /// # Important Notes:
    /// All input shares must use the same x-coordinate system, otherwise homomorphic operations will fail
    pub fn multiply_with_beaver(
        x_shares: &[(FieldElement, FieldElement)],
        y_shares: &[(FieldElement, FieldElement)],
        beaver_triple: &BeaverTriple,
    ) -> Result<Vec<(FieldElement, FieldElement)>, SSSError> {
        use crate::math::HomomorphicOperations;

        // Pre-check: Validate share count consistency
        if x_shares.len() != y_shares.len() || x_shares.len() != beaver_triple.a_shares.len() {
            return Err(SSSError::InvalidParameters(
                "Share counts must match".into(),
            ));
        }

        // === Three core steps of Beaver multiplication protocol ===

        // Step 1: Compute mask differences d = x - a and e = y - b
        // Use homomorphic subtraction, result is still in secret-shared form
        let d_shares = HomomorphicOperations::subtract_shares(x_shares, &beaver_triple.a_shares)?;
        let e_shares = HomomorphicOperations::subtract_shares(y_shares, &beaver_triple.b_shares)?;

        // Step 2: Reconstruct and reveal d and e values
        // This step is secure because d and e are randomly masked values that don't reveal original secrets
        let d_u64 = SecretSharing::recover(&d_shares[0..beaver_triple.threshold])?;
        let e_u64 = SecretSharing::recover(&e_shares[0..beaver_triple.threshold])?;

        // Convert to field elements for arithmetic consistency
        let d = FieldElement::from_u64(d_u64);
        let e = FieldElement::from_u64(e_u64);

        // Step 3: Compute x * y using Beaver formula: x * y = c + d * b + e * a + d * e

        // Sub-step 3a: Compute d * b term (constant times shares)
        let db_shares =
            HomomorphicOperations::multiply_by_constant(&beaver_triple.b_shares, d_u64)?;

        // Sub-step 3b: Compute e * a term (constant times shares)
        let ea_shares =
            HomomorphicOperations::multiply_by_constant(&beaver_triple.a_shares, e_u64)?;

        // Sub-step 3c: Compute d * e term (product of two public constants)
        // This is a public constant, all shares have the same y-value
        let de = d * e; // Finite field multiplication
        let de_shares = beaver_triple
            .c_shares
            .iter()
            .map(|(x, _)| (*x, de)) // Use same x-coordinate, y-value is de constant
            .collect::<Vec<_>>();

        // Sub-step 3d: Combine all terms c + d*b + e*a + d*e
        // Use homomorphic addition to add terms sequentially
        let mut result = beaver_triple.c_shares.clone(); // Start with c term
        result = HomomorphicOperations::add_shares(&result, &db_shares)?; // + d*b
        result = HomomorphicOperations::add_shares(&result, &ea_shares)?; // + e*a
        result = HomomorphicOperations::add_shares(&result, &de_shares)?; // + d*e

        // Return final multiplication result shares
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;

    /// Create test random number generator
    /// Use fixed seed to ensure test reproducibility
    fn create_test_rng() -> ChaCha20Rng {
        ChaCha20Rng::seed_from_u64(42) // Fixed seed 42 ensures consistent random sequence
    }

    /// Test basic Beaver triple creation functionality
    ///
    /// Verifies correctness of triple creation and basic properties
    #[test]
    fn test_beaver_triple_creation() {
        let mut rng = create_test_rng();
        let threshold = 3; // Need 3 shares to reconstruct secret
        let total_shares = 5; // Generate 5 shares total
        let a = 7u64; // First secret value
        let b = 11u64; // Second secret value

        // Create Beaver triple
        let triple = BeaverTriple::new(a, b, threshold, total_shares, &mut rng)
            .expect("Beaver triple creation should succeed");

        // Verify basic properties
        assert_eq!(triple.get_threshold(), threshold);
        assert_eq!(triple.get_share_count(), total_shares);

        // Verify mathematical correctness: c = a * b
        assert!(triple.verify().expect("Verification should succeed"));
    }

    #[test]
    fn test_beaver_triple_verification() {
        let mut rng = create_test_rng();
        let threshold = 2;
        let total_shares = 4;

        let test_cases = [(5u64, 7u64), (0u64, 100u64), (u32::MAX as u64, 2u64)];

        for (a, b) in test_cases {
            let triple = BeaverTriple::new(a, b, threshold, total_shares, &mut rng)
                .expect("Triple creation should succeed");

            assert!(
                triple.verify().expect("Verification should succeed"),
                "Triple ({a}, {b}) should be valid"
            );
        }
    }

    #[test]
    fn test_beaver_triple_distributor() {
        let mut rng = create_test_rng();
        let threshold = 3;
        let total_shares = 5;
        let cache_size = 10;

        let mut distributor =
            BeaverTripleDistributor::new(threshold, total_shares, cache_size, &mut rng)
                .expect("Distributor creation should succeed");

        // Test getting single triple
        let triple = distributor
            .get_triple(&mut rng)
            .expect("Getting triple should succeed");
        assert!(triple.verify().expect("Triple should be valid"));
    }

    /// Test core Beaver multiplication protocol
    ///
    /// This is the most important test of the module, verifying Beaver multiplication correctness
    ///
    /// Test scenario: Securely compute 5 × 7 = 35 using Beaver triple
    #[test]
    fn test_beaver_multiplication() {
        let mut rng = create_test_rng();
        let threshold = 3; // Threshold parameter for Shamir secret sharing
        let total_shares = 5; // Total number of shares generated

        // Define two secret values to multiply
        let x_secret = 5u64; // First secret: x = 5
        let y_secret = 7u64; // Second secret: y = 7
        let _expected_product = x_secret.wrapping_mul(y_secret); // Expected result: 35

        // Use known Beaver triple values for debugging
        let a_secret = 3u64; // Value a in Beaver triple
        let b_secret = 11u64; // Value b in Beaver triple
                              // Value c will be automatically calculated as a × b = 3 × 11 = 33

        use crate::poly::Polynomial;

        // Use simple x coordinates for consistency
        let x_coords: Vec<FieldElement> = (1..=total_shares)
            .map(|i| FieldElement::from_u64(i as u64))
            .collect();

        let poly_x = Polynomial::new(
            threshold - 1,
            FieldElementTrait::from_u64(x_secret),
            &mut rng,
        );
        let x_shares: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_x.evaluate(&x))).collect();

        let poly_y = Polynomial::new(
            threshold - 1,
            FieldElementTrait::from_u64(y_secret),
            &mut rng,
        );
        let y_shares: Vec<(FieldElement, FieldElement)> =
            x_coords.iter().map(|&x| (x, poly_y.evaluate(&x))).collect();

        // Create Beaver triple with known a, b values for debugging
        let triple =
            BeaverTriple::new_with_coordinates(a_secret, b_secret, &x_coords, threshold, &mut rng)
                .expect("Triple creation should succeed");

        // Verify the triple is correct
        assert!(triple.verify().expect("Triple should be valid"));

        // Test the manual calculation to debug the algorithm
        // d = x - a, e = y - b
        let _d = x_secret.wrapping_sub(a_secret); // 5 - 3 = 2
        let _e = y_secret.wrapping_sub(b_secret); // 7 - 11 = -4 (mod p)
        let _c = a_secret.wrapping_mul(b_secret); // 3 * 11 = 33

        // x * y should equal: c + d*b + e*a + d*e
        // = 33 + 2*11 + (-4)*3 + 2*(-4)
        // = 33 + 22 - 12 - 8 = 35
        // But we expect x*y = 5*7 = 35, so the math checks out

        // Perform Beaver multiplication
        let product_shares =
            BeaverMultiplication::multiply_with_beaver(&x_shares, &y_shares, &triple)
                .expect("Beaver multiplication should succeed");

        // Verify result by checking that it matches direct multiplication in the field
        let recovered_product = SecretSharing::recover(&product_shares[0..threshold])
            .expect("Product recovery should succeed");

        // For comparison, do direct multiplication in the field
        let x_field = FieldElement::from_u64(x_secret);
        let y_field = FieldElement::from_u64(y_secret);
        let expected_product_field = (x_field * y_field).to_u64();

        assert_eq!(
            recovered_product, expected_product_field,
            "Beaver multiplication should produce same result as direct field multiplication"
        );
    }

    #[test]
    fn test_beaver_with_zero_values() {
        let mut rng = create_test_rng();
        let threshold = 2;
        let _total_shares = 3;

        // Use simple x coordinates for debugging
        let x_coords = vec![
            FieldElement::from_u64(1),
            FieldElement::from_u64(2),
            FieldElement::from_u64(3),
        ];

        // Test a simple case: 0 * 42
        let x = 0u64;
        let y = 42u64;

        // Calculate expected result using field multiplication
        let x_field = FieldElement::from_u64(x);
        let y_field = FieldElement::from_u64(y);
        let expected = (x_field * y_field).to_u64();

        println!("Testing {x} * {y} = {expected} in field");

        // Create shares with consistent coordinates
        use crate::poly::Polynomial;
        let poly_x = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(x), &mut rng);
        let x_shares: Vec<(FieldElement, FieldElement)> = x_coords
            .iter()
            .map(|&coord| (coord, poly_x.evaluate(&coord)))
            .collect();

        let poly_y = Polynomial::new(threshold - 1, FieldElementTrait::from_u64(y), &mut rng);
        let y_shares: Vec<(FieldElement, FieldElement)> = x_coords
            .iter()
            .map(|&coord| (coord, poly_y.evaluate(&coord)))
            .collect();

        // Verify that reconstructing x and y gives the right values
        let reconstructed_x = SecretSharing::recover(&x_shares[0..threshold])
            .expect("Should be able to reconstruct x");
        let reconstructed_y = SecretSharing::recover(&y_shares[0..threshold])
            .expect("Should be able to reconstruct y");

        println!("Reconstructed x = {reconstructed_x}, y = {reconstructed_y}");
        assert_eq!(reconstructed_x, x);
        assert_eq!(reconstructed_y, y);

        // Create Beaver triple with known values for debugging
        let a = 7u64;
        let b = 13u64;
        let triple = BeaverTriple::new_with_coordinates(a, b, &x_coords, threshold, &mut rng)
            .expect("Triple creation should succeed");

        // Verify triple
        assert!(triple.verify().expect("Triple should be valid"));

        let result_shares =
            BeaverMultiplication::multiply_with_beaver(&x_shares, &y_shares, &triple)
                .expect("Multiplication should succeed");

        let result =
            SecretSharing::recover(&result_shares[0..threshold]).expect("Recovery should succeed");

        println!("Got result: {result}, expected: {expected}");

        assert_eq!(
            result, expected,
            "Zero multiplication case ({x}, {y}) should work"
        );
    }

    #[test]
    fn test_distributor_cache_management() {
        let mut rng = create_test_rng();
        let threshold = 2;
        let total_shares = 3;
        let cache_size = 3;

        let mut distributor =
            BeaverTripleDistributor::new(threshold, total_shares, cache_size, &mut rng)
                .expect("Distributor creation should succeed");

        let (initial_cache, max_cache) = distributor.get_cache_info();
        assert_eq!(initial_cache, cache_size);
        assert_eq!(max_cache, cache_size);

        // Consume all cached triples
        for _ in 0..cache_size {
            distributor
                .get_triple(&mut rng)
                .expect("Getting triple should succeed");
        }

        let (empty_cache, _) = distributor.get_cache_info();
        assert_eq!(empty_cache, 0);

        // Getting another triple should trigger cache refill
        let triple = distributor
            .get_triple(&mut rng)
            .expect("Getting triple should succeed");
        assert!(triple.verify().expect("Triple should be valid"));

        let (refilled_cache, _) = distributor.get_cache_info();
        assert_eq!(refilled_cache, cache_size - 1); // One consumed
    }

    #[test]
    fn test_error_handling() {
        let mut rng = create_test_rng();

        // Test invalid threshold parameters
        let result = BeaverTripleDistributor::new(0, 5, 10, &mut rng);
        assert!(result.is_err());

        let result = BeaverTripleDistributor::new(6, 5, 10, &mut rng);
        assert!(result.is_err());

        // Test mismatched share counts
        let triple = BeaverTriple::new(5, 7, 2, 3, &mut rng).unwrap();
        let x_shares = vec![(
            FieldElementTrait::from_u64(1),
            FieldElementTrait::from_u64(2),
        )];
        let y_shares = vec![
            (
                FieldElementTrait::from_u64(3),
                FieldElementTrait::from_u64(4),
            ),
            (
                FieldElementTrait::from_u64(5),
                FieldElementTrait::from_u64(6),
            ),
        ];

        let result = BeaverMultiplication::multiply_with_beaver(&x_shares, &y_shares, &triple);
        assert!(result.is_err());
    }
}

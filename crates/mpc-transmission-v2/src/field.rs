// Import finite field related libraries
use ff::{Field, PrimeField}; // ff library provides finite field implementation
use rand_core::RngCore; // Random number generator trait
use subtle::CtOption; // Constant time option type

/// Abstract trait for finite field elements
///
/// Defines basic finite field operations needed in Shamir secret sharing
/// A finite field is a mathematical structure containing finite elements, supporting addition and multiplication
/// and every non-zero element has a multiplicative inverse
pub trait FieldElement: Copy + Clone {
    /// Zero element of the finite field (additive identity)
    fn zero() -> Self;

    /// Unit element of the finite field (multiplicative identity)
    fn one() -> Self;

    /// Finite field addition operation
    /// Modifies current element to self + rhs
    fn add(&mut self, rhs: &Self);

    /// Finite field multiplication operation
    /// Modifies current element to self * rhs
    fn mul(&mut self, rhs: &Self);

    /// Calculate multiplicative inverse
    /// Returns element inv that satisfies self * inv = 1
    /// Uses CtOption to ensure constant time operations, preventing side-channel attacks
    fn inv(&self) -> CtOption<Self>;

    /// Convert u64 to finite field element
    fn from_u64(value: u64) -> Self;

    /// Convert finite field element to u64
    fn to_u64(&self) -> u64;

    /// Generate random finite field element
    fn random<R: RngCore>(rng: &mut R) -> Self;
}

/// Concrete implementation of 64-bit finite field GF(p)
///
/// # Mathematical Background
/// - Choose large prime p = 18446744069414584321 as modulus
/// - All operations are performed modulo p
/// - This ensures sufficient security and computational efficiency
///
/// # Implementation Details
/// - Uses ff library's PrimeField derive macro to automatically generate finite field operations
/// - Internal representation as [u64; 2] array, supporting big integer arithmetic
/// - Uses little-endian byte order for storage
#[derive(PrimeField)]
#[PrimeFieldModulus = "18446744069414584321"] // Modulus p: must be a large prime
#[PrimeFieldGenerator = "7"] // Generator: must be a primitive root of p
#[PrimeFieldReprEndianness = "little"] // Little-endian byte order
pub struct GF64([u64; 2]); // Internally uses 2 u64s for storage, supporting big number arithmetic

/// Implement FieldElement trait for GF64
///
/// Provides unified finite field operation interface for convenient use in SSS
impl FieldElement for GF64 {
    /// Return zero element of the finite field
    /// In GF(p), the zero element is integer 0
    fn zero() -> Self {
        Self::ZERO
    }

    /// Return unit element of the finite field
    /// In GF(p), the unit element is integer 1
    fn one() -> Self {
        Self::ONE
    }

    /// Finite field addition: (self + rhs) mod p
    ///
    /// # Algorithm Principle
    /// In finite field GF(p), addition is defined as integer addition modulo p
    /// Result is always in range [0, p-1]
    fn add(&mut self, rhs: &Self) {
        *self += *rhs;
    }

    /// Finite field multiplication: (self * rhs) mod p
    ///
    /// # Algorithm Principle  
    /// In finite field GF(p), multiplication is defined as integer multiplication modulo p
    /// Result is always in range [0, p-1]
    fn mul(&mut self, rhs: &Self) {
        *self *= *rhs;
    }

    /// Calculate multiplicative inverse: find inv that satisfies self * inv ≡ 1 (mod p)
    ///
    /// # Algorithm Principle
    /// Uses extended Euclidean algorithm to calculate modular inverse
    /// For prime modulus p, every non-zero element has a unique multiplicative inverse
    ///
    /// # Security Considerations
    /// - Uses CtOption to prevent side-channel attacks
    /// - Computation time is independent of input values, enhancing attack resistance
    ///
    /// # Return Value
    /// * `Some(inv)` - Successfully calculated inverse
    /// * `None` - Input is zero (zero element has no inverse)
    fn inv(&self) -> CtOption<Self> {
        self.invert()
    }

    /// Convert u64 to finite field element
    fn from_u64(value: u64) -> Self {
        Self::from(value)
    }

    /// Convert finite field element to u64
    /// Note: This conversion may lose information, for demonstration only
    fn to_u64(&self) -> u64 {
        // Use standard conversion method, convert finite field element to byte array then to u64
        let bytes = self.to_repr();
        u64::from_le_bytes([
            bytes.as_ref()[0],
            bytes.as_ref()[1],
            bytes.as_ref()[2],
            bytes.as_ref()[3],
            bytes.as_ref()[4],
            bytes.as_ref()[5],
            bytes.as_ref()[6],
            bytes.as_ref()[7],
        ])
    }

    /// Generate random finite field element
    fn random<R: RngCore>(rng: &mut R) -> Self {
        <Self as Field>::random(&mut *rng)
    }
}

/// Provide additional convenience functions for GF64
pub mod gf64_sss {
    pub use super::GF64 as FieldElement;
    use super::*;

    /// Generate random finite field element
    ///
    /// # Parameters
    /// * `rng` - Cryptographically secure random number generator
    ///
    /// # Usage
    /// Used in SSS for:
    /// - Generating random coefficients for polynomials
    /// - Selecting random x-coordinates for share points
    ///
    /// # Return Value
    /// Uniformly distributed random finite field element
    pub fn random_element<R: RngCore>(rng: &mut R) -> FieldElement {
        <FieldElement as Field>::random(rng)
    }
}

/// Unit test module
///
/// Verify correctness and consistency of finite field operations
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
    fn test_field_element_zero_and_one() {
        let zero = GF64::zero();
        let one = GF64::one();

        assert_eq!(zero, GF64::ZERO, "zero() should return ZERO constant");
        assert_eq!(one, GF64::ONE, "one() should return ONE constant");
        assert_ne!(zero, one, "Zero and one should be different");

        // Verify special properties of zero element
        assert_eq!(zero.to_u64(), 0u64, "Zero should convert to 0");
        assert_eq!(one.to_u64(), 1u64, "One should convert to 1");
    }

    #[test]
    fn test_field_addition_basic() {
        let a = GF64::from(5u64);
        let b = GF64::from(3u64);

        // Test basic addition
        let mut sum = a;
        sum.add(&b);
        assert_eq!(sum, GF64::from(8u64), "5 + 3 should equal 8");

        // Verify addition does not change original values
        assert_eq!(a, GF64::from(5u64), "Original value a should not change");
        assert_eq!(b, GF64::from(3u64), "Original value b should not change");

        // Test larger numbers
        let large_a = GF64::from(1000000u64);
        let large_b = GF64::from(999999u64);
        let mut large_sum = large_a;
        large_sum.add(&large_b);
        assert_eq!(
            large_sum,
            GF64::from(1999999u64),
            "Large number addition should work"
        );
    }

    #[test]
    fn test_field_addition_properties() {
        let a = GF64::from(123u64);
        let b = GF64::from(456u64);
        let c = GF64::from(789u64);
        let zero = GF64::zero();

        // Addition commutativity: a + b = b + a
        let mut a_plus_b = a;
        a_plus_b.add(&b);
        let mut b_plus_a = b;
        b_plus_a.add(&a);
        assert_eq!(a_plus_b, b_plus_a, "Addition should be commutative");

        // Addition associativity: (a + b) + c = a + (b + c)
        let mut left_assoc = a;
        left_assoc.add(&b);
        left_assoc.add(&c);

        let mut b_plus_c = b;
        b_plus_c.add(&c);
        let mut right_assoc = a;
        right_assoc.add(&b_plus_c);

        assert_eq!(left_assoc, right_assoc, "Addition should be associative");

        // Addition identity: a + 0 = a
        let mut a_plus_zero = a;
        a_plus_zero.add(&zero);
        assert_eq!(a_plus_zero, a, "Adding zero should not change value");
    }

    #[test]
    fn test_field_multiplication_basic() {
        let a = GF64::from(5u64);
        let b = GF64::from(3u64);

        // Test basic multiplication
        let mut product = a;
        product.mul(&b);
        assert_eq!(product, GF64::from(15u64), "5 * 3 should equal 15");

        // Verify multiplication does not change original values
        assert_eq!(a, GF64::from(5u64), "Original value a should not change");
        assert_eq!(b, GF64::from(3u64), "Original value b should not change");

        // Test multiplication by zero
        let mut zero_product = a;
        zero_product.mul(&GF64::zero());
        assert_eq!(
            zero_product,
            GF64::zero(),
            "Multiplication by zero should give zero"
        );
    }

    #[test]
    fn test_field_multiplication_properties() {
        let a = GF64::from(7u64);
        let b = GF64::from(11u64);
        let c = GF64::from(13u64);
        let one = GF64::one();

        // Multiplication commutativity: a * b = b * a
        let mut a_times_b = a;
        a_times_b.mul(&b);
        let mut b_times_a = b;
        b_times_a.mul(&a);
        assert_eq!(a_times_b, b_times_a, "Multiplication should be commutative");

        // Multiplication associativity: (a * b) * c = a * (b * c)
        let mut left_assoc = a;
        left_assoc.mul(&b);
        left_assoc.mul(&c);

        let mut b_times_c = b;
        b_times_c.mul(&c);
        let mut right_assoc = a;
        right_assoc.mul(&b_times_c);

        assert_eq!(
            left_assoc, right_assoc,
            "Multiplication should be associative"
        );

        // Multiplication identity: a * 1 = a
        let mut a_times_one = a;
        a_times_one.mul(&one);
        assert_eq!(a_times_one, a, "Multiplying by one should not change value");
    }

    #[test]
    fn test_field_distributive_property() {
        let a = GF64::from(17u64);
        let b = GF64::from(23u64);
        let c = GF64::from(29u64);

        // Distributive law: a * (b + c) = (a * b) + (a * c)

        // Left side: a * (b + c)
        let mut b_plus_c = b;
        b_plus_c.add(&c);
        let mut left_side = a;
        left_side.mul(&b_plus_c);

        // Right side: (a * b) + (a * c)
        let mut a_times_b = a;
        a_times_b.mul(&b);
        let mut a_times_c = a;
        a_times_c.mul(&c);
        let mut right_side = a_times_b;
        right_side.add(&a_times_c);

        assert_eq!(left_side, right_side, "Distributive property should hold");
    }

    #[test]
    fn test_multiplicative_inverse_basic() {
        let test_values = [1u64, 2, 3, 7, 17, 101, 1009, 999983];

        for &value in &test_values {
            let a = GF64::from(value);
            let inv_result = a.inv();

            assert!(
                bool::from(inv_result.is_some()),
                "Non-zero element {value} should have inverse"
            );

            let inv_a = inv_result.unwrap();

            // Verify a * a^(-1) = 1
            let mut product = a;
            product.mul(&inv_a);
            assert_eq!(
                product,
                GF64::one(),
                "Element {value} times its inverse should equal one"
            );

            // Verify a^(-1) * a = 1 (commutativity)
            let mut product2 = inv_a;
            product2.mul(&a);
            assert_eq!(
                product2,
                GF64::one(),
                "Inverse times element {value} should equal one"
            );
        }
    }

    #[test]
    fn test_zero_has_no_inverse() {
        let zero = GF64::zero();
        let inv_result = zero.inv();

        assert!(
            !bool::from(inv_result.is_some()),
            "Zero should not have multiplicative inverse"
        );
    }

    #[test]
    fn test_inverse_of_inverse() {
        let a = GF64::from(42u64);
        let inv_a = a.inv().unwrap();
        let inv_inv_a = inv_a.inv().unwrap();

        assert_eq!(
            a, inv_inv_a,
            "Inverse of inverse should equal original element"
        );
    }

    #[test]
    fn test_from_u64_conversion() {
        let test_values = [0u64, 1, 100, 65535, 1000000, u32::MAX as u64];

        for &value in &test_values {
            let field_elem = GF64::from_u64(value);
            let converted_back = field_elem.to_u64();

            // For small values, should be completely reversible
            if value < 1000000 {
                assert_eq!(
                    converted_back, value,
                    "Small value {value} should convert back exactly"
                );
            }
        }
    }

    #[test]
    fn test_to_u64_consistency() {
        let a = GF64::from(12345u64);
        let b = GF64::from(54321u64);

        // If two finite field elements are equal, their u64 representations should also be equal
        assert_eq!(a.to_u64(), a.to_u64(), "Same element should give same u64");

        // Different elements should give different u64 (within reasonable range)
        if a != b {
            assert_ne!(
                a.to_u64(),
                b.to_u64(),
                "Different elements should give different u64"
            );
        }
    }

    #[test]
    fn test_random_element_generation() {
        let mut rng = create_test_rng();

        // Generate multiple random elements
        let random_elements: Vec<GF64> = (0..10)
            .map(|_| <GF64 as FieldElement>::random(&mut rng))
            .collect();

        // Verify diversity of random elements (check if at least some elements are different)
        let mut has_different_elements = false;
        for i in 1..random_elements.len() {
            if random_elements[i] != random_elements[0] {
                has_different_elements = true;
                break;
            }
        }
        assert!(has_different_elements, "Random elements should be diverse");

        // Verify all random elements are valid finite field elements
        for elem in &random_elements {
            // Try basic operations
            let mut test = *elem;
            test.add(&GF64::one());
            test.mul(&GF64::from(2u64));
            // If we can execute to here, the element is valid
        }
    }

    #[test]
    fn test_gf64_sss_random_element() {
        let mut rng = create_test_rng();

        // Test convenience function in module
        let random1 = gf64_sss::random_element(&mut rng);
        let random2 = gf64_sss::random_element(&mut rng);

        // Random elements should be different (extremely high probability)
        assert_ne!(random1, random2, "Random elements should be different");

        // Verify returned type is indeed FieldElement
        let _: gf64_sss::FieldElement = random1;
    }

    #[test]
    fn test_field_arithmetic_consistency() {
        let a = GF64::from(137u64);
        let b = GF64::from(239u64);

        // Using trait methods
        let mut trait_sum = a;
        trait_sum.add(&b);

        let mut trait_product = a;
        trait_product.mul(&b);

        // Using ff library's native methods
        let native_sum = a + b;
        let native_product = a * b;

        assert_eq!(
            trait_sum, native_sum,
            "Trait addition should match native addition"
        );
        assert_eq!(
            trait_product, native_product,
            "Trait multiplication should match native multiplication"
        );
    }

    #[test]
    fn test_large_numbers() {
        // Test large numbers close to modulus
        let large_values = [
            18446744069414584320u64, // modulus - 1
            18446744069414584319u64, // modulus - 2
            9223372034707292160u64,  // modulus / 2
        ];

        for &value in &large_values {
            let field_elem = GF64::from_u64(value);

            // Verify basic operations still work
            let mut test = field_elem;
            test.add(&GF64::one());
            test.mul(&GF64::from(2u64));

            // Verify inverse exists (if not zero)
            if field_elem != GF64::zero() {
                let inv = field_elem.inv();
                assert!(
                    bool::from(inv.is_some()),
                    "Large non-zero value should have inverse"
                );
            }
        }
    }

    #[test]
    fn test_field_element_equality() {
        let a = GF64::from(42u64);
        let b = GF64::from(42u64);
        let c = GF64::from(43u64);

        assert_eq!(a, b, "Same values should be equal");
        assert_ne!(a, c, "Different values should not be equal");

        // Verify Copy trait works correctly
        let a_copy = a;
        assert_eq!(a, a_copy, "Copy should preserve equality");
    }

    #[test]
    fn test_additive_inverse() {
        let test_values = [1u64, 17, 255, 65535];

        for &value in &test_values {
            let a = GF64::from(value);
            let neg_a = GF64::zero() - a; // calculate additive inverse

            // Verify a + (-a) = 0
            let mut sum = a;
            sum.add(&neg_a);
            assert_eq!(
                sum,
                GF64::zero(),
                "Element plus its additive inverse should equal zero"
            );
        }
    }

    #[test]
    fn test_power_operations() {
        let a = GF64::from(3u64);

        // Test square
        let a_squared = a * a;
        let mut manual_square = a;
        manual_square.mul(&a);
        assert_eq!(a_squared, manual_square, "Square operations should match");

        // Test cube
        let a_cubed = a_squared * a;
        let mut manual_cube = manual_square;
        manual_cube.mul(&a);
        assert_eq!(a_cubed, manual_cube, "Cube operations should match");
    }

    #[test]
    fn test_modular_properties() {
        // Verify properties of modular arithmetic
        let a = GF64::from(18446744069414584320u64); // Value close to modulus
        let b = GF64::from(5u64);

        // Even for large numbers, operation results should be within finite field
        let mut sum = a;
        sum.add(&b);

        let mut product = a;
        product.mul(&b);

        // These operations should not overflow or crash
        assert_ne!(sum, a, "Large addition should change value");
        assert_ne!(product, a, "Large multiplication should change value");
    }
}

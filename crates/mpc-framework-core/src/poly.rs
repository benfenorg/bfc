// Import error handling and finite field types
use crate::error::SSSError;
use crate::field::gf64_sss::FieldElement;
use crate::field::FieldElement as FieldElementTrait;
use ff::Field;
use rand_core::RngCore;

/// Polynomial structure
///
/// Used for polynomial construction and interpolation operations in Shamir secret sharing
/// Polynomial form: f(x) = a₀ + a₁x + a₂x² + ... + aₙxⁿ
/// Where coefficients[i] = aᵢ
#[derive(Debug, Clone)]
pub struct Polynomial {
    coefficients: Vec<FieldElement>, // Polynomial coefficients, starting from constant term
}

impl Polynomial {
    /// Create a new random polynomial
    ///
    /// # Arguments
    /// * `degree` - Degree of the polynomial (exponent of highest term)
    /// * `intercept` - Y-intercept (constant term a₀), this is the secret to be shared in SSS
    /// * `rng` - Cryptographically secure random number generator
    ///
    /// # Algorithm Description
    /// 1. Set the secret as constant term a₀
    /// 2. Randomly generate the remaining degree coefficients a₁, a₂, ..., aₑ
    /// 3. Construct polynomial f(x) = secret + a₁x + a₂x² + ... + aₑxᵉ
    ///
    /// # Returns
    /// Newly constructed polynomial object
    pub fn new<R: RngCore>(degree: usize, intercept: FieldElement, rng: &mut R) -> Self {
        // Initialize coefficient vector, constant term is the secret value
        let mut coefficients = vec![intercept];

        // Randomly generate coefficients for higher-order terms
        for _ in 0..degree {
            coefficients.push(FieldElementTrait::random(rng));
        }

        Self { coefficients }
    }

    /// Create new random polynomial, ensuring coefficients are within u64 range
    pub fn new_with_u64_coeffs<R: RngCore>(
        degree: usize,
        intercept: FieldElement,
        rng: &mut R,
    ) -> Self {
        let mut coefficients = vec![intercept];
        for _ in 0..degree {
            // Get a u64 value from RNG, then convert to FieldElement
            let random_u64 = rng.next_u64();
            coefficients.push(FieldElementTrait::from_u64(random_u64));
        }
        Self { coefficients }
    }

    /// Create a new polynomial with fixed seed (compatible with mpc-transmission)
    ///
    /// Uses a fixed seed [0x42; 32] for RNG, same as mpc-transmission.
    /// This ensures deterministic polynomial coefficient generation across all users.
    ///
    /// # Arguments
    /// * `degree` - Degree of the polynomial
    /// * `intercept` - Y-intercept (constant term, the secret)
    ///
    /// # Returns
    /// Polynomial with deterministic random coefficients
    pub fn new_with_fixed_seed(degree: usize, intercept: FieldElement) -> Self {
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        let mut coefficients = vec![intercept];

        // Use fixed seed [0x42; 32] - same as mpc-transmission
        let mut rng = ChaCha8Rng::from_seed([0x42; 32]);

        for _ in 0..degree {
            coefficients.push(FieldElementTrait::random(&mut rng));
        }

        Self { coefficients }
    }

    /// Evaluate polynomial at given point (Horner's method)
    ///
    /// # Arguments
    /// * `x` - x-coordinate of evaluation point
    ///
    /// # Algorithm
    /// Use Horner's method to efficiently evaluate polynomial:
    /// f(x) = a₀ + x(a₁ + x(a₂ + x(a₃ + ...)))
    /// This reduces multiplication operations from O(n²) to O(n)
    ///
    /// # Returns
    /// Value of polynomial at point x
    pub fn evaluate(&self, x: &FieldElement) -> FieldElement {
        if self.coefficients.is_empty() {
            return FieldElement::ZERO;
        }

        // True Horner method: start from highest degree term
        let mut result = self.coefficients[self.coefficients.len() - 1];

        // Start from second-to-last term, process sequentially
        for i in (0..self.coefficients.len() - 1).rev() {
            result = result * (*x) + self.coefficients[i];
        }

        result
    }

    /// Optimized version: fast evaluation using Horner method
    /// This is an explicit optimization implementation for performance-critical scenarios
    pub fn evaluate_horner_optimized(&self, x: &FieldElement) -> FieldElement {
        self.evaluate(x) // Now evaluate already uses Horner method
    }

    /// Lagrange interpolation to recover polynomial value at x=0
    ///
    /// # Arguments  
    /// * `points` - Known points on the polynomial [(x₁,y₁), (x₂,y₂), ..., (xₜ,yₜ)]
    ///
    /// # Algorithm
    /// Lagrange interpolation formula reconstructs a t-1 degree polynomial from t points:
    ///
    /// f(x) = Σᵢ₌₁ᵗ yᵢ * Lᵢ(x)
    ///
    /// Where basis function Lᵢ(x) = Πⱼ≠ᵢ (x - xⱼ) / (xᵢ - xⱼ)
    ///
    /// For the special case x=0:
    /// f(0) = Σᵢ₌₁ᵗ yᵢ * Πⱼ≠ᵢ (-xⱼ) / (xᵢ - xⱼ)
    ///
    /// # Mathematical Background
    /// - t distinct points uniquely determine a t-1 degree polynomial
    /// - In SSS, f(0) is the original secret
    /// - All operations are performed in finite field for cryptographic security
    ///
    /// # Returns
    /// * `Ok(FieldElement)` - Polynomial value at x=0 (original secret)
    /// * `Err(SSSError)` - Error when interpolation fails
    pub fn interpolate(points: &[(FieldElement, FieldElement)]) -> Result<FieldElement, SSSError> {
        // Check input validity
        if points.is_empty() {
            return Err(SSSError::InterpolationError("No points provided".into()));
        }

        let mut result = FieldElement::ZERO;

        // Calculate contribution of each given point (xᵢ, yᵢ) in interpolation
        for (i, (x_i, y_i)) in points.iter().enumerate() {
            // Calculate numerator and denominator of Lagrange basis function Lᵢ(0)
            let mut numerator = FieldElement::ONE; // Πⱼ≠ᵢ (-xⱼ)
            let mut denominator = FieldElement::ONE; // Πⱼ≠ᵢ (xᵢ - xⱼ)

            // Iterate through all other points to build basis function
            for (j, (x_j, _)) in points.iter().enumerate() {
                if i == j {
                    continue; // Skip current point
                }

                // Numerator: accumulate product (-xⱼ) = Πⱼ≠ᵢ (-xⱼ)
                numerator *= -*x_j;

                // Denominator: accumulate product (xᵢ - xⱼ) = Πⱼ≠ᵢ (xᵢ - xⱼ)
                denominator *= *x_i - *x_j;
            }

            // Calculate contribution of this term: yᵢ * Lᵢ(0) = yᵢ * (numerator/denominator)
            // In finite field, division is implemented by multiplying the inverse
            let inv = denominator.invert();
            let inv = if bool::from(inv.is_some()) {
                inv.unwrap()
            } else {
                return Err(SSSError::InterpolationError(
                    "Division by zero in interpolation".into(),
                ));
            };
            let term = *y_i * numerator * inv;

            // Accumulate to final result
            result += term;
        }

        Ok(result)
    }


    /// Get the degree of the polynomial
    ///
    /// # Returns
    /// Degree of the polynomial (exponent of highest term)
    pub fn degree(&self) -> usize {
        if self.coefficients.is_empty() {
            0
        } else {
            self.coefficients.len() - 1
        }
    }

    /// Get reference to polynomial coefficients (read-only)
    ///
    /// # Returns
    /// Reference to coefficient vector
    pub fn coefficients(&self) -> &[FieldElement] {
        &self.coefficients
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
    fn test_polynomial_new() {
        let mut rng = create_test_rng();
        let degree = 3;
        let intercept = FieldElementTrait::from_u64(100);

        let poly = Polynomial::new(degree, intercept, &mut rng);

        assert_eq!(poly.degree(), degree, "Polynomial degree should match");
        assert_eq!(
            poly.coefficients.len(),
            degree + 1,
            "Should have correct number of coefficients"
        );
        assert_eq!(
            poly.coefficients[0], intercept,
            "Constant term should equal intercept"
        );
    }

    #[test]
    fn test_polynomial_evaluate_constant() {
        let mut rng = create_test_rng();
        let intercept = FieldElementTrait::from_u64(42);

        // Create 0-degree polynomial (constant polynomial)
        let poly = Polynomial::new(0, intercept, &mut rng);

        let x = FieldElementTrait::from_u64(5);
        let result = poly.evaluate(&x);

        assert_eq!(
            result, intercept,
            "Constant polynomial should always return the constant"
        );
    }

    #[test]
    fn test_polynomial_evaluate_linear() {
        let intercept = FieldElementTrait::from_u64(10);
        let slope = FieldElementTrait::from_u64(3);

        // Manually create linear polynomial f(x) = 10 + 3x
        let poly = Polynomial {
            coefficients: vec![intercept, slope],
        };

        // Test f(0) = 10
        let x_zero = FieldElement::ZERO;
        let result_zero = poly.evaluate(&x_zero);
        assert_eq!(result_zero, intercept, "f(0) should equal intercept");

        // Test f(1) = 10 + 3 = 13
        let x_one = FieldElementTrait::from_u64(1);
        let result_one = poly.evaluate(&x_one);
        let expected = intercept + slope;
        assert_eq!(result_one, expected, "f(1) should equal intercept + slope");

        // Test f(2) = 10 + 6 = 16
        let x_two = FieldElementTrait::from_u64(2);
        let result_two = poly.evaluate(&x_two);
        let two: FieldElement = FieldElementTrait::from_u64(2);
        let expected: FieldElement = intercept + slope * two;
        assert_eq!(
            result_two, expected,
            "f(2) should equal intercept + 2*slope"
        );
    }

    #[test]
    fn test_polynomial_evaluate_quadratic() {
        // Create quadratic polynomial f(x) = 1 + 2x + 3x²
        let poly = Polynomial {
            coefficients: vec![
                FieldElementTrait::from_u64(1), // Constant term
                FieldElementTrait::from_u64(2), // Linear coefficient
                FieldElementTrait::from_u64(3), // Quadratic coefficient
            ],
        };

        // Test f(0) = 1
        let result = poly.evaluate(&FieldElement::ZERO);
        assert_eq!(result, FieldElementTrait::from_u64(1));

        // Test f(1) = 1 + 2 + 3 = 6
        let result = poly.evaluate(&FieldElementTrait::from_u64(1));
        assert_eq!(result, FieldElementTrait::from_u64(6));

        // Test f(2) = 1 + 4 + 12 = 17
        let result = poly.evaluate(&FieldElementTrait::from_u64(2));
        assert_eq!(result, FieldElementTrait::from_u64(17));
    }

    #[test]
    fn test_polynomial_degree() {
        let mut rng = create_test_rng();
        let intercept = FieldElementTrait::from_u64(1);

        let test_degrees = [0, 1, 5, 10];

        for &degree in &test_degrees {
            let poly = Polynomial::new(degree, intercept, &mut rng);
            assert_eq!(
                poly.degree(),
                degree,
                "Degree should be correct for degree {degree}"
            );
        }
    }

    #[test]
    fn test_interpolate_single_point() {
        let point = (
            FieldElementTrait::from_u64(5),
            FieldElementTrait::from_u64(100),
        );
        let points = vec![point];

        let result = Polynomial::interpolate(&points);
        assert!(result.is_ok(), "Single point interpolation should succeed");

        // Single point interpolation should return the y-value of that point (constant polynomial)
        let interpolated = result.unwrap();
        assert_eq!(interpolated, FieldElementTrait::from_u64(100));
    }

    #[test]
    fn test_interpolate_two_points() {
        // Create two points to define a line
        let points = vec![
            (
                FieldElementTrait::from_u64(1),
                FieldElementTrait::from_u64(3),
            ), // (1, 3)
            (
                FieldElementTrait::from_u64(2),
                FieldElementTrait::from_u64(5),
            ), // (2, 5)
        ];

        // This line is f(x) = 1 + 2x, so f(0) = 1
        let result = Polynomial::interpolate(&points);
        assert!(result.is_ok(), "Two point interpolation should succeed");

        let interpolated = result.unwrap();
        assert_eq!(
            interpolated,
            FieldElementTrait::from_u64(1),
            "f(0) should be 1"
        );
    }

    #[test]
    fn test_interpolate_three_points_quadratic() {
        // Create three points to define quadratic polynomial f(x) = 2 + 3x + x²
        // f(1) = 2 + 3 + 1 = 6
        // f(2) = 2 + 6 + 4 = 12
        // f(3) = 2 + 9 + 9 = 20
        let points = vec![
            (
                FieldElementTrait::from_u64(1),
                FieldElementTrait::from_u64(6),
            ),
            (
                FieldElementTrait::from_u64(2),
                FieldElementTrait::from_u64(12),
            ),
            (
                FieldElementTrait::from_u64(3),
                FieldElementTrait::from_u64(20),
            ),
        ];

        let result = Polynomial::interpolate(&points);
        assert!(result.is_ok(), "Three point interpolation should succeed");

        // f(0) = 2
        let interpolated = result.unwrap();
        assert_eq!(
            interpolated,
            FieldElementTrait::from_u64(2),
            "f(0) should be 2"
        );
    }

    #[test]
    fn test_interpolate_empty_points() {
        let points = vec![];
        let result = Polynomial::interpolate(&points);

        assert!(result.is_err(), "Empty points should cause error");
        match result.unwrap_err() {
            SSSError::InterpolationError(_) => {}
            _ => panic!("Should return InterpolationError"),
        }
    }

    #[test]
    fn test_interpolate_consistency_with_evaluation() {
        let mut rng = create_test_rng();
        let degree = 3;
        let intercept = FieldElementTrait::from_u64(42);

        // Create polynomial
        let poly = Polynomial::new(degree, intercept, &mut rng);

        // Evaluate at multiple points
        let x_values = [1, 2, 3, 4, 5];
        let mut points = Vec::new();

        for &x_val in &x_values {
            let x = FieldElementTrait::from_u64(x_val);
            let y = poly.evaluate(&x);
            points.push((x, y));
        }

        // Use any 4 points (degree+1 points) for interpolation
        let result = Polynomial::interpolate(&points[0..=degree]);
        assert!(result.is_ok(), "Interpolation should succeed");

        // Interpolation result should equal original intercept (f(0) value)
        let interpolated = result.unwrap();
        assert_eq!(
            interpolated, intercept,
            "Interpolated f(0) should match original intercept"
        );
    }

    #[test]
    fn test_interpolate_different_point_combinations() {
        let mut rng = create_test_rng();
        let degree = 2;
        let intercept = FieldElementTrait::from_u64(123);

        let poly = Polynomial::new(degree, intercept, &mut rng);

        // Generate sufficient points
        let mut all_points = Vec::new();
        for i in 1..=6 {
            let x = FieldElementTrait::from_u64(i);
            let y = poly.evaluate(&x);
            all_points.push((x, y));
        }

        // Test different 3-point combinations (degree + 1 = 3)
        let combinations = [
            &all_points[0..3], // Points 1,2,3
            &all_points[1..4], // Points 2,3,4
            &all_points[2..5], // Points 3,4,5
            &all_points[3..6], // Points 4,5,6
        ];

        for (i, points) in combinations.iter().enumerate() {
            let result = Polynomial::interpolate(points);
            assert!(result.is_ok(), "Combination {i} should succeed");

            let interpolated = result.unwrap();
            assert_eq!(
                interpolated, intercept,
                "All combinations should give same result (combination {i})"
            );
        }
    }

    #[test]
    fn test_polynomial_random_coefficients_uniqueness() {
        let mut rng = create_test_rng();
        let degree = 3;
        let intercept = FieldElementTrait::from_u64(100);

        // Create multiple polynomials, verify random coefficients are different
        let poly1 = Polynomial::new(degree, intercept, &mut rng);
        let poly2 = Polynomial::new(degree, intercept, &mut rng);

        // Constant terms should be the same
        assert_eq!(poly1.coefficients[0], poly2.coefficients[0]);

        // At least one higher-order coefficient should be different (extremely high probability)
        let mut has_different_coeff = false;
        for i in 1..=degree {
            if poly1.coefficients[i] != poly2.coefficients[i] {
                has_different_coeff = true;
                break;
            }
        }
        assert!(
            has_different_coeff,
            "Random coefficients should be different"
        );
    }

    #[test]
    fn test_edge_case_zero_degree() {
        let mut rng = create_test_rng();
        let intercept = FieldElementTrait::from_u64(99);

        // 0-degree polynomial is just a constant
        let poly = Polynomial::new(0, intercept, &mut rng);

        assert_eq!(poly.degree(), 0);
        assert_eq!(poly.coefficients.len(), 1);
        assert_eq!(poly.coefficients[0], intercept);

        // Evaluation at any point should be the constant
        let test_points = [0, 1, 100, 999];
        for &x_val in &test_points {
            let x = FieldElementTrait::from_u64(x_val);
            let result = poly.evaluate(&x);
            assert_eq!(
                result, intercept,
                "Constant polynomial should return same value for x={x_val}"
            );
        }
    }

    #[test]
    fn test_new_with_u64_coeffs() {
        let mut rng = create_test_rng();
        let degree = 3;
        let intercept = FieldElementTrait::from_u64(100);

        let poly = Polynomial::new_with_u64_coeffs(degree, intercept, &mut rng);

        assert_eq!(poly.degree(), degree);
        assert_eq!(poly.coefficients.len(), degree + 1);
        assert_eq!(poly.coefficients[0], intercept);

        // Verify all coefficients can be converted to u64 (within finite field)
        for (i, coeff) in poly.coefficients.iter().enumerate() {
            let as_u64 = FieldElementTrait::to_u64(coeff);
            let back_to_fe = <FieldElement as FieldElementTrait>::from_u64(as_u64);
            assert_eq!(
                back_to_fe, *coeff,
                "Coefficient {i} should round-trip through u64"
            );
        }
    }

    #[test]
    fn test_evaluate_horner_optimized() {
        let coeffs = vec![
            FieldElementTrait::from_u64(5), // Constant term
            FieldElementTrait::from_u64(3), // x term
            FieldElementTrait::from_u64(2), // x² term
        ];
        let poly = Polynomial {
            coefficients: coeffs,
        };

        let x = FieldElementTrait::from_u64(4);

        // Standard and optimized methods should give same result
        let result1 = poly.evaluate(&x);
        let result2 = poly.evaluate_horner_optimized(&x);

        assert_eq!(
            result1, result2,
            "Standard and optimized evaluation should match"
        );

        // Manual verification: f(4) = 5 + 3*4 + 2*16 = 5 + 12 + 32 = 49
        let expected = FieldElementTrait::from_u64(49);
        assert_eq!(result1, expected, "f(4) should equal 49");
    }



    #[test]
    fn test_coefficients_getter() {
        let mut rng = create_test_rng();
        let degree = 2;
        let intercept = FieldElementTrait::from_u64(123);

        let poly = Polynomial::new(degree, intercept, &mut rng);
        let coeffs = poly.coefficients();

        assert_eq!(coeffs.len(), degree + 1);
        assert_eq!(coeffs[0], intercept);

        // Verify that an immutable reference is returned
        // coeffs[0] = FieldElement::ZERO; // This line should fail to compile if uncommented
    }

    #[test]
    fn test_polynomial_with_zero_coefficients() {
        // Test case where all higher-order coefficients are zero
        let poly = Polynomial {
            coefficients: vec![
                FieldElementTrait::from_u64(5),
                FieldElement::ZERO,
                FieldElement::ZERO,
            ],
        };

        // This should behave like a constant polynomial
        let test_values = [0, 1, 5, 100];
        for &x_val in &test_values {
            let x = FieldElementTrait::from_u64(x_val);
            let result = poly.evaluate(&x);
            assert_eq!(
                result,
                FieldElementTrait::from_u64(5),
                "Polynomial with zero higher coefficients should be constant at x={x_val}"
            );
        }
    }

    #[test]
    fn test_evaluate_with_large_values() {
        let poly = Polynomial {
            coefficients: vec![
                FieldElementTrait::from_u64(1),
                FieldElementTrait::from_u64(1),
            ],
        };

        // Test large values
        let large_x = FieldElementTrait::from_u64(u64::MAX);
        let result = poly.evaluate(&large_x);

        // Result should be within finite field
        let _result_u64 = FieldElementTrait::to_u64(&result);
        // u64 values are always within u64 range, no need to check
    }

    #[test]
    fn test_interpolate_robustness() {
        // Test numerical stability of interpolation
        let points = vec![
            (
                FieldElementTrait::from_u64(1),
                FieldElementTrait::from_u64(1000000),
            ),
            (
                FieldElementTrait::from_u64(2),
                FieldElementTrait::from_u64(2000000),
            ),
            (
                FieldElementTrait::from_u64(3),
                FieldElementTrait::from_u64(3000000),
            ),
        ];

        let result = Polynomial::interpolate(&points);
        assert!(
            result.is_ok(),
            "Interpolation with large values should succeed"
        );

        // This represents a line through origin f(x) = 1000000x
        // So f(0) = 0
        let interpolated = result.unwrap();
        assert_eq!(
            interpolated,
            FieldElement::ZERO,
            "Linear interpolation should give f(0)=0"
        );
    }

    #[test]
    fn test_polynomial_comparison_and_equality() {
        let mut rng = create_test_rng();
        let intercept = FieldElementTrait::from_u64(42);

        // Create two polynomials with same parameters
        let poly1 = Polynomial::new(2, intercept, &mut rng);
        let poly2 = Polynomial::new(2, intercept, &mut rng);

        // Their constant terms should be the same
        assert_eq!(poly1.coefficients[0], poly2.coefficients[0]);

        // But other coefficients should be different (randomly generated)
        let mut coeffs_different = false;
        for i in 1..poly1.coefficients.len() {
            if poly1.coefficients[i] != poly2.coefficients[i] {
                coeffs_different = true;
                break;
            }
        }
        assert!(coeffs_different, "Random coefficients should be different");
    }
}

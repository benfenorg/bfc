// Import thiserror library for automatic Error trait implementation
use thiserror::Error;

/// Error type definitions for Shamir's Secret Sharing scheme
///
/// Uses thiserror library to automatically generate error handling code, providing clear error messages
/// and appropriate error propagation mechanisms for easier debugging and error handling
#[derive(Error, Debug)]
pub enum SSSError {
    /// Finite field arithmetic error
    ///
    /// # Possible causes
    /// - Division by zero operation (computing inverse of 0)
    /// - Finite field arithmetic overflow or underflow
    /// - Arithmetic exceptions caused by non-prime modulus
    ///
    /// # Example scenarios
    /// - Encountering zero elements when computing denominator inverse in Lagrange interpolation
    /// - Finite field element conversion failure
    #[error("Field operation error: {0}")]
    FieldError(String),

    /// Invalid parameter error
    ///
    /// # Possible causes
    /// - Threshold value t not in valid range [1, n]
    /// - Total share count n is 0 or too large
    /// - Parameter type mismatch or format error
    ///
    /// # Example scenarios
    /// - Attempting to create invalid threshold schemes like (0, 5) or (6, 5)
    /// - Passing negative numbers or values exceeding system limits
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    /// Insufficient shares error
    ///
    /// # Possible causes
    /// - Number of provided shares is less than threshold t
    /// - Share array is empty
    /// - Shares contain duplicate x-coordinates (violating uniqueness requirement)
    ///
    /// # Example scenarios
    /// - Providing only 2 shares for reconstruction in a (3, 5) scheme
    /// - Attempting to recover secret with an empty share list
    #[error("Insufficient shares: {0}")]
    InsufficientShares(String),

    /// Interpolation calculation error
    ///
    /// # Possible causes
    /// - Numerical calculation exceptions during Lagrange interpolation
    /// - Share points not on the same polynomial (data tampering)
    /// - Duplicate x-coordinates in share points, making interpolation non-unique
    ///
    /// # Mathematical background
    /// Lagrange interpolation requires all share points to have distinct x-coordinates,
    /// otherwise the polynomial cannot be uniquely determined, usually indicating data integrity issues
    ///
    /// # Example scenarios
    /// - Malicious attackers modified some share data
    /// - Share data corruption during network transmission
    #[error("Interpolation error: {0}")]
    InterpolationError(String),

    /// Invalid share format error
    ///
    /// # Possible causes
    /// - Share serialization/deserialization failure
    /// - Share data structure doesn't match expected format
    /// - Share encoding method not supported
    ///
    /// # Example scenarios
    /// - Incorrect share data format read from file or network
    /// - Compatibility issues between different SSS implementation versions
    #[error("Invalid share format: {0}")]
    InvalidShareFormat(String),

    /// Verification failure error
    ///
    /// # Possible causes
    /// - Reconstructed secret doesn't match expected value
    /// - Share integrity check failure
    /// - Digital signature verification failure
    ///
    /// # Security considerations
    /// This type of error usually indicates security threats such as:
    /// - Malicious share providers
    /// - Man-in-the-middle attacks
    /// - Share data tampering
    ///
    /// # Example scenarios
    /// - Failure when reconstructing secret using tampered shares
    /// - Digital signature verification failure for shares
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
}

/// Convenience implementations for error handling
impl SSSError {
    /// Create finite field operation error
    ///
    /// # Parameters
    /// * `msg` - Specific error description message
    ///
    /// # Returns
    /// SSSError::FieldError variant
    pub fn field_error(msg: &str) -> Self {
        SSSError::FieldError(msg.to_string())
    }

    /// Create invalid parameter error
    ///
    /// # Parameters
    /// * `msg` - Specific error description message
    ///
    /// # Returns
    /// SSSError::InvalidParameters variant
    pub fn invalid_parameters(msg: &str) -> Self {
        SSSError::InvalidParameters(msg.to_string())
    }

    /// Create insufficient shares error
    ///
    /// # Parameters
    /// * `msg` - Specific error description message
    ///
    /// # Returns
    /// SSSError::InsufficientShares variant
    pub fn insufficient_shares(msg: &str) -> Self {
        SSSError::InsufficientShares(msg.to_string())
    }
}

/// Convert finite field arithmetic errors to SSSError
///
/// This implementation allows the ? operator to automatically handle error propagation in finite field operations
/// When CtOption is None (e.g., computing inverse of 0), automatically converts to SSSError
impl From<subtle::CtOption<crate::field::GF64>> for SSSError {
    fn from(_option: subtle::CtOption<crate::field::GF64>) -> Self {
        SSSError::FieldError(
            // This case should not happen, as From is only called on error
            "Finite field operation failed - likely division by zero or invalid operation"
                .to_string(),
        )
    }
}

/// Convert mpc-transmission SecretSharingError to SSSError
///
/// This implementation allows automatic error conversion when using mpc-transmission functions
impl From<mpc_transmission::error::SecretSharingError> for SSSError {
    fn from(err: mpc_transmission::error::SecretSharingError) -> Self {
        match err {
            mpc_transmission::error::SecretSharingError::InvalidShare(msg) => {
                SSSError::InvalidShareFormat(msg)
            }
            mpc_transmission::error::SecretSharingError::InsufficientShares(required, provided) => {
                SSSError::InsufficientShares(format!(
                    "Required {} shares, but only {} provided",
                    required, provided
                ))
            }
            mpc_transmission::error::SecretSharingError::RecoveryFailed(msg) => {
                SSSError::InterpolationError(msg)
            }
            mpc_transmission::error::SecretSharingError::InvalidThreshold(threshold) => {
                SSSError::InvalidParameters(format!("Invalid threshold: {}", threshold))
            }
            mpc_transmission::error::SecretSharingError::ThresholdTooLarge(threshold) => {
                SSSError::InvalidParameters(format!("Threshold too large: {}", threshold))
            }
            mpc_transmission::error::SecretSharingError::EmptySecret => {
                SSSError::InvalidParameters("Empty secret data".to_string())
            }
            mpc_transmission::error::SecretSharingError::InvalidShareCount(threshold, shares) => {
                SSSError::InvalidParameters(format!(
                    "Share count must be ≥ threshold (threshold: {}, shares: {})",
                    threshold, shares
                ))
            }
            mpc_transmission::error::SecretSharingError::InvalidRandomLength(len) => {
                SSSError::InvalidParameters(format!("Invalid random length: {}", len))
            }
            mpc_transmission::error::SecretSharingError::Io(e) => {
                SSSError::InvalidShareFormat(format!("IO error: {}", e))
            }
            mpc_transmission::error::SecretSharingError::Json(e) => {
                SSSError::InvalidShareFormat(format!("JSON parse error: {}", e))
            }
            mpc_transmission::error::SecretSharingError::KeyParse(msg) => {
                SSSError::InvalidParameters(format!("Key parse error: {}", msg))
            }
            mpc_transmission::error::SecretSharingError::InvalidKey(key) => {
                SSSError::InvalidParameters(format!("Invalid key: {}", key))
            }
            mpc_transmission::error::SecretSharingError::FileNotFound(path) => {
                SSSError::InvalidShareFormat(format!("File not found: {}", path))
            }
            mpc_transmission::error::SecretSharingError::ValidationFailed(msg) => {
                SSSError::VerificationFailed(msg)
            }
            mpc_transmission::error::SecretSharingError::ArithmeticOverflow { operation } => {
                SSSError::FieldError(format!("Arithmetic overflow: {}", operation))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_variants() {
        let field_error = SSSError::field_error("Division by zero");
        assert!(matches!(field_error, SSSError::FieldError(_)));

        let param_error = SSSError::invalid_parameters("Invalid threshold");
        assert!(matches!(param_error, SSSError::InvalidParameters(_)));

        let share_error = SSSError::insufficient_shares("Not enough shares");
        assert!(matches!(share_error, SSSError::InsufficientShares(_)));
    }

    #[test]
    fn test_error_creation_methods() {
        let error1 = SSSError::field_error("test message");
        let error2 = SSSError::invalid_parameters("test message");
        let error3 = SSSError::insufficient_shares("test message");

        assert!(error1.to_string().contains("test message"));
        assert!(error2.to_string().contains("test message"));
        assert!(error3.to_string().contains("test message"));
    }

    #[test]
    fn test_error_debug_format() {
        let error = SSSError::field_error("debug test");
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("FieldError"));
        assert!(debug_str.contains("debug test"));
    }
}

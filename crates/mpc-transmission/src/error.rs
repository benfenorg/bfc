use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecretSharingError {
    #[error("Invalid share data: {0}")]
    InvalidShare(String),

    #[error("Insufficient shares: required {0}, provided {1}")]
    InsufficientShares(u8, usize),

    #[error("Invalid threshold: must be ≥ 2 and ≤ 255 (got {0})")]
    InvalidThreshold(usize),

    #[error("Threshold too large: maximum 255 (got {0})")]
    ThresholdTooLarge(usize),

    #[error("Empty secret data")]
    EmptySecret,

    #[error("Recovery failed: {0}")]
    RecoveryFailed(String),

    #[error("Share count must be ≥ threshold (threshold: {0}, shares: {1})")]
    InvalidShareCount(u8, usize),

    #[error("Random data length must be ≥ 1 (got {0})")]
    InvalidRandomLength(usize),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Key parse error: {0}")]
    KeyParse(String),
    #[error("Invalid key: {0}")]
    InvalidKey(u64),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Data validation failed: {0}")]
    ValidationFailed(String),

    #[error("Arithmetic overflow: {operation} overflow occurred")]
    ArithmeticOverflow { operation: String },
}

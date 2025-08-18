#[allow(unused_imports)]
// 标准库导入
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::io::{self};

// 外部crate导入
#[allow(unused_imports)]
use rand::random;
use thiserror::Error;

// sharks crate的重新导出
pub use sharks::{Share, Sharks};

// 本地模块声明
pub mod field;
pub mod math;
pub mod read;

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
}

/// 生成秘密分片
///
/// # 参数
/// - `secret`: 要保护的秘密数据(字节切片)
/// - `threshold`: 恢复秘密所需的最小分片数(必须 ≥ 2且 ≤ 255)
/// - `total_shares`: 要生成的总分片数(必须 ≥ threshold)
///
/// # 返回
/// - 成功: 包含分片的 Vec<Share>
/// - 失败: SecretSharingError
pub fn generate_shares(
    secret: &[u8],
    threshold: usize,
    total_shares: usize,
) -> Result<Vec<Share>, SecretSharingError> {
    // 验证输入参数
    if secret.is_empty() {
        return Err(SecretSharingError::EmptySecret);
    }
    if threshold < 2 {
        return Err(SecretSharingError::InvalidThreshold(threshold)); // 提供参数
    }
    if threshold > 255 {
        return Err(SecretSharingError::ThresholdTooLarge(threshold)); // 提供参数
    }
    if total_shares < threshold {
        return Err(SecretSharingError::InsufficientShares(
            threshold as u8,
            total_shares,
        ));
    }

    // 创建Shamir秘密共享实例
    let sharks = Sharks(threshold as u8);

    // 生成分片
    let dealer = sharks.dealer(secret);
    let shares: Vec<Share> = dealer.take(total_shares).collect();

    Ok(shares)
}

/// 从分片恢复秘密
///
/// # 参数
/// - `shares`: 分片集合(必须包含至少阈值数量的分片)
/// - `threshold`: 最初设置的阈值(必须 ≥ 2且 ≤ 255)
///
/// # 返回
/// - 成功: 恢复出的秘密数据 Vec<u8>
/// - 失败: SecretSharingError
pub fn recover_secret(shares: &[Share], threshold: usize) -> Result<Vec<u8>, SecretSharingError> {
    // 参数验证
    if threshold < 2 {
        return Err(SecretSharingError::InvalidThreshold(threshold)); // 提供参数
    }
    if threshold > 255 {
        return Err(SecretSharingError::ThresholdTooLarge(threshold)); // 提供参数
    }

    // 检查分片数量
    if shares.len() < threshold {
        return Err(SecretSharingError::InsufficientShares(
            threshold as u8,
            shares.len(),
        ));
    }

    // 创建恢复器并尝试恢复
    let sharks = Sharks(threshold as u8);
    sharks
        .recover(shares)
        .map_err(|e| SecretSharingError::RecoveryFailed(e.to_string()))
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

/// 分割u64秘密
///
pub fn generate_shares_u64(
    secret: u64,
    threshold: usize,
    total_shares: usize,
) -> Result<Vec<Share>, SecretSharingError> {
    // 参数验证
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
    let sharks = Sharks(threshold as u8);
    let dealer = sharks.dealer(&secret_bytes);
    let shares = dealer.take(total_shares).collect();

    Ok(shares)
}

/// 恢复u64秘密
pub fn recover_secret_u64(shares: &[Share], threshold: usize) -> Result<u64, SecretSharingError> {
    // 参数验证
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

    let sharks = Sharks(threshold as u8);
    let bytes = sharks
        .recover(shares)
        .map_err(|e| SecretSharingError::RecoveryFailed(e.to_string()))?;

    bytes_to_u64(&bytes).map_err(|e| SecretSharingError::RecoveryFailed(e.to_string()))
}

/// 使用 XOR 混淆的 Shamir 秘密共享 U64
///
/// 返回: (分片集合, XOR掩码)
pub fn generate_shares_with_xor(
    secret: u64,
    threshold: usize,
    total_shares: usize,
    mask: u64,
) -> Result<Vec<Share>, SecretSharingError> {
    // 1. 应用 XOR 混淆
    let masked_secret = secret ^ mask;

    // 2. 生成 Shamir 分片（直接返回结果）
    generate_shares_u64(masked_secret, threshold, total_shares)
}

/// 恢复带 XOR 混淆的秘密 u64
pub fn recover_secret_with_xor(
    shares: &[Share],
    threshold: usize,
    mask: u64,
) -> Result<u64, SecretSharingError> {
    // 1. 恢复混淆后的值
    let masked_secret = recover_secret_u64(shares, threshold)?;

    // 2. 使用 XOR 恢复原始值
    Ok(masked_secret ^ mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_secret_sharing() {
        let secret = b"my secret data";
        let threshold = 3;
        let total_shares = 5;

        // 生成分片
        let shares = generate_shares(secret, threshold, total_shares).unwrap();
        assert_eq!(shares.len(), total_shares);

        // 使用部分分片恢复秘密
        let recovered = recover_secret(&shares[0..threshold], threshold).unwrap();
        assert_eq!(recovered, secret);

        // 使用全部分片恢复秘密
        let recovered = recover_secret(&shares, threshold).unwrap();
        assert_eq!(recovered, secret);

        // 测试分片不足情况
        assert!(matches!(
            recover_secret(&shares[0..threshold - 1], threshold),
            Err(SecretSharingError::InsufficientShares(_, _))
        ));
    }

    #[test]
    fn test_error_handling() {
        // 测试空秘密
        assert!(matches!(
            generate_shares(&[], 3, 5),
            Err(SecretSharingError::EmptySecret)
        ));

        // 测试无效阈值(1)
        assert!(matches!(
            generate_shares(b"data", 1, 3),
            Err(SecretSharingError::InvalidThreshold(1)) // 提供具体的阈值值
        ));

        // 测试无效阈值(256)
        assert!(matches!(
            generate_shares(b"data", 256, 300),
            Err(SecretSharingError::ThresholdTooLarge(256)) // 提供具体的阈值值
        ));

        // 测试总分片数少于阈值
        assert!(matches!(
            generate_shares(b"data", 4, 3),
            Err(SecretSharingError::InsufficientShares(4, 3)) // 提供具体的阈值和分片数
        ));
    }

    #[test]
    // 测试u64与u8转换
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

        // 测试连续分片
        assert_eq!(
            recover_secret_u64(&shares[0..threshold], threshold).unwrap(),
            secret
        );
        assert_eq!(
            recover_secret_u64(&shares[1..threshold + 1], threshold).unwrap(),
            secret
        );

        // 测试离散分片 - 创建新的Vec<Share>而不是Vec<&Share>
        let scattered = vec![shares[0].clone(), shares[2].clone(), shares[4].clone()];
        assert_eq!(recover_secret_u64(&scattered, threshold).unwrap(), secret);

        // 测试不足阈值的情况
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

        // 使用足够的分片恢复
        let recovered = recover_secret_with_xor(&shares[..threshold], threshold, mask)
            .expect("Failed to recover secret");

        assert_eq!(secret, recovered, "Basic recovery failed");
    }

    #[test]
    fn test_xor_secret_sharing_randomized() {
        for _ in 0..100 {
            // 多次随机测试
            let secret = random::<u64>();
            let threshold = 3;
            let total_shares = 5;
            let mask = 0x12345678ABCDEF00;

            let shares = generate_shares_with_xor(secret, threshold, total_shares, mask)
                .expect("Failed to generate shares");

            // 使用足够的分片恢复
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

        // 使用不足的分片应该失败
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

        // 使用错误的掩码应该恢复出错误的值
        let wrong_mask = random::<u64>();
        let recovered = recover_secret_with_xor(&shares[..threshold], threshold, wrong_mask)
            .expect("Failed to recover (but should get wrong value)");

        assert_ne!(
            secret, recovered,
            "XOR protection failed - recovered correct secret with wrong mask"
        );
    }
}

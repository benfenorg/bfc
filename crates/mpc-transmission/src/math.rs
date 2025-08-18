#[allow(unused_imports)]
use crate::{SecretSharingError, Share, generate_shares_with_xor, recover_secret_with_xor};

// 采用方案一 在计算过程中先恢复原始秘密, 再进行计算 输出u64计算结果
//
// 加法运算
pub fn add_shared_secrets(
    shares1: &[Share],
    shares2: &[Share],
    threshold: usize, // 新增参数
    mask1: u64,
    mask2: u64,
) -> Result<u64, SecretSharingError> {
    let secret1 = recover_secret_with_xor(shares1, threshold, mask1)?;
    let secret2 = recover_secret_with_xor(shares2, threshold, mask2)?;
    Ok(secret1.wrapping_add(secret2))
}

/// 减法运算
pub fn sub_shared_secrets(
    shares1: &[Share],
    shares2: &[Share],
    threshold: usize,
    mask1: u64,
    mask2: u64,
) -> Result<u64, SecretSharingError> {
    let secret1 = recover_secret_with_xor(shares1, threshold, mask1)?;
    let secret2 = recover_secret_with_xor(shares2, threshold, mask2)?;
    Ok(secret1.wrapping_sub(secret2))
}

/// 乘法运算
pub fn mul_shared_secrets(
    shares1: &[Share],
    shares2: &[Share],
    threshold: usize,
    mask1: u64,
    mask2: u64,
) -> Result<u64, SecretSharingError> {
    let secret1 = recover_secret_with_xor(shares1, threshold, mask1)?;
    let secret2 = recover_secret_with_xor(shares2, threshold, mask2)?;
    Ok(secret1.wrapping_mul(secret2))
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用固定 mask（确保所有测试用例使用相同的 mask）
    const TEST_MASK: u64 = 0x12345678ABCDEF00;

    // 生成测试数据：两个秘密的分片 + 阈值
    fn setup_test_secrets() -> (Vec<Share>, Vec<Share>, usize) {
        let threshold = 3;
        let total_shares = 5;
        let secret1 = 12345;
        let secret2 = 67890;

        let shares1 = generate_shares_with_xor(secret1, threshold, total_shares, TEST_MASK)
            .expect("Failed to generate shares1");
        let shares2 = generate_shares_with_xor(secret2, threshold, total_shares, TEST_MASK)
            .expect("Failed to generate shares2");

        (shares1, shares2, threshold)
    }

    // 测试加法：secret1 + secret2
    #[test]
    fn test_addition() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        let result = add_shared_secrets(
            &shares1[..threshold], // 取前 threshold 个分片
            &shares2[..threshold],
            threshold,
            TEST_MASK, // shares1 的 mask
            TEST_MASK, // shares2 的 mask
        )
        .unwrap();

        assert_eq!(result, 12345 + 67890);
    }

    // 测试减法：secret1 - secret2（使用 wrapping_sub 处理无符号数）
    #[test]
    fn test_subtraction() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        let result = sub_shared_secrets(
            &shares1[..threshold],
            &shares2[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();

        assert_eq!(result, 12345u64.wrapping_sub(67890));
    }

    // 测试乘法：secret1 * secret2
    #[test]
    fn test_multiplication() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        let result = mul_shared_secrets(
            &shares1[..threshold],
            &shares2[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();

        assert_eq!(result, 12345 * 67890);
    }

    // 测试边界条件（零、最大值等）
    #[test]
    fn test_edge_cases() {
        let threshold = 2;
        let total_shares = 3;
        let secret_zero = 0;
        let secret_one = 1;
        let secret_max = u64::MAX;

        // 生成分片
        let shares_zero =
            generate_shares_with_xor(secret_zero, threshold, total_shares, TEST_MASK).unwrap();
        let shares_one =
            generate_shares_with_xor(secret_one, threshold, total_shares, TEST_MASK).unwrap();
        let shares_max =
            generate_shares_with_xor(secret_max, threshold, total_shares, TEST_MASK).unwrap();

        // 0 * MAX = 0
        let mul_result = mul_shared_secrets(
            &shares_zero[..threshold],
            &shares_max[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();
        assert_eq!(mul_result, 0);

        // 1 - MAX = 2（wrapping_sub 结果）
        let sub_result = sub_shared_secrets(
            &shares_one[..threshold],
            &shares_max[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();
        assert_eq!(sub_result, 1u64.wrapping_sub(u64::MAX));

        // MAX + 1 = 0（wrapping_add 结果）
        let add_result = add_shared_secrets(
            &shares_max[..threshold],
            &shares_one[..threshold],
            threshold,
            TEST_MASK,
            TEST_MASK,
        )
        .unwrap();
        assert_eq!(add_result, 0);
    }

    // 测试分片不足时返回错误
    #[test]
    fn test_insufficient_shares() {
        let (shares1, shares2, threshold) = setup_test_secrets();

        // 提供的分片数 < threshold
        assert!(
            add_shared_secrets(
                &shares1[..threshold - 1], // 少一个分片
                &shares2[..threshold],
                threshold,
                TEST_MASK,
                TEST_MASK,
            )
            .is_err()
        );

        assert!(
            sub_shared_secrets(
                &shares1[..threshold],
                &shares2[..threshold - 1],
                threshold,
                TEST_MASK,
                TEST_MASK,
            )
            .is_err()
        );
    }
}

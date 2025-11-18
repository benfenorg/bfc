// 全面测试文档：验证本地秘密分享实现的所有功能和边界情况
// 这个测试套件涵盖所有可能的测试场景，确保实现的正确性和健壮性

use mpc_transmission::{
    bytes_to_u64, error::SecretSharingError, generate_shares, generate_shares_u64,
    generate_shares_with_xor, recover_secret, recover_secret_u64, recover_secret_with_xor,
    u64_to_bytes, Share,
};
use std::collections::HashSet;

#[cfg(test)]
mod comprehensive_tests {
    use super::*;

    // ============================================================================
    // 基本功能测试
    // ============================================================================

    #[test]
    fn test_basic_secret_sharing_all_sizes() {
        println!("=== 基本功能测试：所有数据大小 ===");

        // 测试不同大小的秘密数据
        let zero_data = vec![0u8; 100];
        let full_data = vec![255u8; 200];
        let range_data = (0..=255).collect::<Vec<u8>>();

        let test_cases = vec![
            (b"a".as_slice(), "单字符"),
            (b"hello".as_slice(), "短字符串"),
            (
                b"Hello, World! This is a test secret with moderate length.".as_slice(),
                "中等长度",
            ),
            (zero_data.as_slice(), "100字节零数据"),
            (full_data.as_slice(), "200字节满数据"),
            (range_data.as_slice(), "完整字节范围"),
        ];

        for (secret, description) in test_cases {
            println!("测试案例: {}", description);

            for threshold in 2u8..=5 {
                for total_shares in threshold as usize..=10 {
                    let shares =
                        generate_shares(secret, threshold as usize, total_shares).expect(&format!(
                            "生成分片失败: {}, threshold={}, total={}",
                            description, threshold, total_shares
                        ));

                    assert_eq!(shares.len(), total_shares);

                    // 使用最小阈值恢复
                    let recovered =
                        recover_secret(&shares[0..threshold as usize], threshold as usize)
                            .expect(&format!("恢复失败: {}", description));

                    assert_eq!(recovered, secret, "恢复的数据不匹配: {}", description);
                }
            }
            println!("✅ {} 测试通过", description);
        }
    }

    #[test]
    fn test_u64_comprehensive() {
        println!("=== U64功能全面测试 ===");

        let test_values = vec![
            (0u64, "零值"),
            (1u64, "最小正值"),
            (u64::MAX, "最大值"),
            (u64::MAX / 2, "中间值"),
            (0x123456789ABCDEFu64, "典型十六进制值"),
            (0xDEADBEEFCAFEBABEu64, "特殊模式值"),
        ];

        for (secret, description) in test_values {
            println!("测试U64值: {} ({})", secret, description);

            for threshold in 2..=7 {
                for total_shares in threshold..=12 {
                    let shares = generate_shares_u64(secret, threshold, total_shares)
                        .expect(&format!("U64分片生成失败: {}", description));

                    assert_eq!(shares.len(), total_shares);

                    // 测试所有可能的阈值组合
                    for start in 0..=(total_shares - threshold) {
                        let end = start + threshold;
                        let recovered = recover_secret_u64(&shares[start..end], threshold).expect(
                            &format!("U64恢复失败: {}, 组合[{}:{}]", description, start, end),
                        );

                        assert_eq!(recovered, secret, "U64恢复不匹配: {}", description);
                    }
                }
            }
            println!("✅ U64 {} 测试通过", description);
        }
    }

    #[test]
    fn test_xor_comprehensive() {
        println!("=== XOR混淆功能全面测试 ===");

        let secret_mask_pairs = vec![
            (0u64, 0u64, "零值零掩码"),
            (u64::MAX, u64::MAX, "最大值最大掩码"),
            (0x123456789ABCDEFu64, 0xFEDCBA9876543210u64, "互补模式"),
            (0xAAAAAAAAAAAAAAAAu64, 0x5555555555555555u64, "交替位模式"),
            (0x0F0F0F0F0F0F0F0Fu64, 0xF0F0F0F0F0F0F0F0u64, "半字节模式"),
        ];

        for (secret, mask, description) in secret_mask_pairs {
            println!(
                "测试XOR: secret=0x{:016X}, mask=0x{:016X} ({})",
                secret, mask, description
            );

            for threshold in 2..=5 {
                for total_shares in threshold..=8 {
                    let shares = generate_shares_with_xor(secret, threshold, total_shares, mask)
                        .expect(&format!("XOR分片生成失败: {}", description));

                    // 测试正确掩码恢复
                    let recovered = recover_secret_with_xor(&shares[0..threshold], threshold, mask)
                        .expect(&format!("XOR恢复失败: {}", description));
                    assert_eq!(recovered, secret, "XOR恢复不匹配: {}", description);

                    // 测试错误掩码（应该产生不同结果）
                    let wrong_mask = mask ^ 0x1234567890ABCDEFu64;
                    if wrong_mask != mask {
                        // 确保掩码确实不同
                        let wrong_recovered =
                            recover_secret_with_xor(&shares[0..threshold], threshold, wrong_mask)
                                .expect("错误掩码恢复应该成功但结果错误");
                        assert_ne!(
                            wrong_recovered, secret,
                            "错误掩码不应该恢复正确值: {}",
                            description
                        );
                    }
                }
            }
            println!("✅ XOR {} 测试通过", description);
        }
    }

    // ============================================================================
    // 边界条件和极端情况测试
    // ============================================================================

    #[test]
    fn test_boundary_conditions() {
        println!("=== 边界条件测试 ===");

        // 最小阈值测试
        let secret = b"boundary test";
        let shares = generate_shares(secret, 2, 2).expect("最小配置分片生成失败");
        let recovered = recover_secret(&shares, 2).expect("最小配置恢复失败");
        assert_eq!(recovered, secret);
        println!("✅ 最小阈值(2,2)测试通过");

        // 最大阈值测试
        let shares = generate_shares(secret, 255, 255).expect("最大配置分片生成失败");
        assert_eq!(shares.len(), 255);
        let recovered = recover_secret(&shares, 255).expect("最大配置恢复失败");
        assert_eq!(recovered, secret);
        println!("✅ 最大阈值(255,255)测试通过");

        // 大数据量测试
        let large_secret = vec![42u8; 10000]; // 10KB数据
        let shares = generate_shares(&large_secret, 3, 5).expect("大数据分片生成失败");
        let recovered = recover_secret(&shares[0..3], 3).expect("大数据恢复失败");
        assert_eq!(recovered, large_secret);
        println!("✅ 大数据量(10KB)测试通过");

        // 单字节所有可能值测试
        for byte_val in 0u8..=255 {
            let secret = vec![byte_val];
            let shares =
                generate_shares(&secret, 2, 3).expect(&format!("字节值{}分片失败", byte_val));
            let recovered =
                recover_secret(&shares[0..2], 2).expect(&format!("字节值{}恢复失败", byte_val));
            assert_eq!(recovered, secret);
        }
        println!("✅ 所有单字节值(0-255)测试通过");
    }

    #[test]
    fn test_extreme_share_distributions() {
        println!("=== 极端分片分布测试 ===");

        let secret = b"distribution test";
        let threshold = 5;
        let total_shares = 20;

        let shares =
            generate_shares(secret, threshold, total_shares).expect("极端分布分片生成失败");

        // 测试各种分片选择模式
        let test_patterns = vec![
            (vec![0, 1, 2, 3, 4], "连续前5个"),
            (vec![15, 16, 17, 18, 19], "连续后5个"),
            (vec![0, 5, 10, 15, 19], "等间隔分布"),
            (vec![1, 3, 7, 13, 17], "质数索引"),
            (vec![0, 2, 4, 6, 8], "偶数索引"),
            (vec![1, 3, 5, 7, 9], "奇数索引"),
        ];

        for (indices, description) in test_patterns {
            let selected_shares: Vec<_> = indices.iter().map(|&i| shares[i].clone()).collect();
            let recovered = recover_secret(&selected_shares, threshold)
                .expect(&format!("极端分布恢复失败: {}", description));
            assert_eq!(recovered, secret, "极端分布恢复不匹配: {}", description);
            println!("✅ {} 测试通过", description);
        }
    }

    // ============================================================================
    // 错误处理测试
    // ============================================================================

    #[test]
    fn test_comprehensive_error_handling() {
        println!("=== 全面错误处理测试 ===");

        // 空秘密错误
        match generate_shares(&[], 3, 5) {
            Err(SecretSharingError::EmptySecret) => println!("✅ 空秘密错误正确"),
            _ => panic!("空秘密应该返回EmptySecret错误"),
        }

        // 无效阈值错误
        let invalid_thresholds = vec![0, 1, 256, 300];
        for threshold in invalid_thresholds {
            let result = generate_shares(b"test", threshold, 5);
            assert!(result.is_err(), "阈值{}应该返回错误", threshold);
            println!("✅ 无效阈值{}错误正确", threshold);
        }

        // 分片数少于阈值错误
        match generate_shares(b"test", 5, 3) {
            Err(SecretSharingError::InsufficientShares(5, 3)) => println!("✅ 分片数不足错误正确"),
            _ => panic!("分片数不足应该返回InsufficientShares错误"),
        }

        // 恢复时分片不足错误
        let shares = generate_shares(b"test", 5, 10).unwrap();
        match recover_secret(&shares[0..3], 5) {
            Err(SecretSharingError::InsufficientShares(5, 3)) => {
                println!("✅ 恢复分片不足错误正确")
            }
            _ => panic!("恢复分片不足应该返回InsufficientShares错误"),
        }

        // U64转换错误
        let short_bytes = vec![1, 2, 3]; // 少于8字节
        match bytes_to_u64(&short_bytes) {
            Err("Input slice must be exactly 8 bytes") => println!("✅ U64转换错误正确"),
            _ => panic!("短字节数组应该返回转换错误"),
        }
    }

    #[test]
    fn test_malformed_share_handling() {
        println!("=== 畸形分片处理测试 ===");

        // 空分片
        let empty_bytes = vec![];
        match Share::try_from(empty_bytes.as_slice()) {
            Err("A Share must be at least 2 bytes long") => println!("✅ 空分片错误正确"),
            _ => panic!("空分片应该返回错误"),
        }

        // 单字节分片
        let single_byte = vec![1];
        match Share::try_from(single_byte.as_slice()) {
            Err("A Share must be at least 2 bytes long") => println!("✅ 单字节分片错误正确"),
            _ => panic!("单字节分片应该返回错误"),
        }

        // 正常分片验证
        let normal_bytes = vec![1, 2, 3, 4];
        let share = Share::try_from(normal_bytes.as_slice()).expect("正常分片应该成功");
        assert_eq!(share.x.0, 1);
        assert_eq!(share.y.len(), 3);
        println!("✅ 正常分片解析正确");
    }

    // ============================================================================
    // 性能和压力测试
    // ============================================================================

    #[test]
    fn test_performance_stress() {
        println!("=== 性能压力测试 ===");

        use std::time::Instant;

        // 大量小分片测试
        let start = Instant::now();
        for i in 0..1000 {
            let secret = format!("test secret {}", i);
            let shares = generate_shares(secret.as_bytes(), 3, 5).expect("大量小分片失败");
            let recovered = recover_secret(&shares[0..3], 3).expect("大量小分片恢复失败");
            assert_eq!(recovered, secret.as_bytes());
        }
        let duration = start.elapsed();
        println!("✅ 1000个小分片测试用时: {:?}", duration);

        // 少量大分片测试
        let start = Instant::now();
        for size in vec![1024, 2048, 4096, 8192] {
            let secret = vec![42u8; size];
            let shares = generate_shares(&secret, 5, 8).expect(&format!("{}字节大分片失败", size));
            let recovered =
                recover_secret(&shares[0..5], 5).expect(&format!("{}字节恢复失败", size));
            assert_eq!(recovered, secret);
            println!("✅ {}字节大分片测试通过", size);
        }
        let duration = start.elapsed();
        println!("✅ 大分片测试用时: {:?}", duration);

        // 高阈值测试
        let start = Instant::now();
        let secret = b"high threshold test";
        for threshold in vec![10, 20, 50, 100] {
            let shares = generate_shares(secret, threshold, threshold + 10)
                .expect(&format!("阈值{}分片失败", threshold));
            let recovered = recover_secret(&shares[0..threshold], threshold)
                .expect(&format!("阈值{}恢复失败", threshold));
            assert_eq!(recovered, secret);
            println!("✅ 阈值{}测试通过", threshold);
        }
        let duration = start.elapsed();
        println!("✅ 高阈值测试用时: {:?}", duration);
    }

    // ============================================================================
    // 确定性行为测试
    // ============================================================================

    #[test]
    fn test_deterministic_behavior_comprehensive() {
        println!("=== 全面确定性行为测试 ===");

        // 多次运行相同输入应该产生相同结果
        let zero_test_data = vec![0u8; 100];
        let full_test_data = vec![255u8; 50];

        let test_secrets = vec![
            b"deterministic test 1".as_slice(),
            b"a".as_slice(),
            b"".as_slice(), // 这应该失败，但我们测试错误的确定性
            zero_test_data.as_slice(),
            full_test_data.as_slice(),
        ];

        for secret in test_secrets {
            if secret.is_empty() {
                // 测试错误的确定性
                for _ in 0..10 {
                    let result = generate_shares(secret, 3, 5);
                    assert!(result.is_err(), "空秘密应该始终失败");
                }
                println!("✅ 空秘密错误确定性正确");
                continue;
            }

            let mut all_shares = Vec::new();

            // 生成10次相同的分片
            for run in 0..10 {
                let shares =
                    generate_shares(secret, 3, 5).expect(&format!("确定性测试运行{}失败", run));
                all_shares.push(shares);
            }

            // 验证所有运行结果相同
            for run in 1..10 {
                assert_eq!(
                    all_shares[0].len(),
                    all_shares[run].len(),
                    "运行{}分片数量不一致",
                    run
                );

                for share_idx in 0..all_shares[0].len() {
                    let share_0 = &all_shares[0][share_idx];
                    let share_run = &all_shares[run][share_idx];

                    assert_eq!(
                        share_0.x.0, share_run.x.0,
                        "运行{}分片{}的x不一致",
                        run, share_idx
                    );
                    assert_eq!(
                        share_0.y.len(),
                        share_run.y.len(),
                        "运行{}分片{}的y长度不一致",
                        run,
                        share_idx
                    );

                    for y_idx in 0..share_0.y.len() {
                        assert_eq!(
                            share_0.y[y_idx].0, share_run.y[y_idx].0,
                            "运行{}分片{}的y[{}]不一致",
                            run, share_idx, y_idx
                        );
                    }
                }
            }

            println!("✅ 秘密长度{}的确定性测试通过", secret.len());
        }
    }

    // ============================================================================
    // 序列化和兼容性测试
    // ============================================================================

    #[test]
    fn test_serialization_comprehensive() {
        println!("=== 全面序列化测试 ===");

        let zero_ser_data = vec![0u8; 100];
        let range_ser_data = (0u8..=255).collect::<Vec<u8>>();

        let secrets = vec![
            b"ser test".as_slice(),
            zero_ser_data.as_slice(),
            range_ser_data.as_slice(),
        ];

        for secret in secrets {
            let shares = generate_shares(secret, 3, 5).expect("序列化测试分片生成失败");

            // 测试每个分片的序列化和反序列化
            for (i, share) in shares.iter().enumerate() {
                // 序列化
                let bytes: Vec<u8> = Vec::from(share);
                println!("分片{}序列化: {} bytes", i, bytes.len());

                // 反序列化
                let recovered_share =
                    Share::try_from(bytes.as_slice()).expect(&format!("分片{}反序列化失败", i));

                // 验证内容一致
                assert_eq!(share.x.0, recovered_share.x.0, "分片{}的x值不一致", i);
                assert_eq!(
                    share.y.len(),
                    recovered_share.y.len(),
                    "分片{}的y长度不一致",
                    i
                );

                for (j, (original, recovered)) in
                    share.y.iter().zip(recovered_share.y.iter()).enumerate()
                {
                    assert_eq!(original.0, recovered.0, "分片{}的y[{}]不一致", i, j);
                }
            }

            // 测试序列化后的分片能否正常恢复秘密
            let serialized_shares: Vec<Share> = shares
                .iter()
                .map(|s| Share::try_from(Vec::from(s).as_slice()).unwrap())
                .collect();

            let recovered =
                recover_secret(&serialized_shares[0..3], 3).expect("序列化分片恢复失败");
            assert_eq!(recovered, secret, "序列化分片恢复的秘密不匹配");

            println!("✅ 秘密长度{}的序列化测试通过", secret.len());
        }
    }

    // ============================================================================
    // 数学正确性测试
    // ============================================================================

    #[test]
    fn test_mathematical_correctness() {
        println!("=== 数学正确性测试 ===");

        // 测试拉格朗日插值的数学性质
        let secret = b"math test";
        let threshold = 4;
        let total_shares = 10;

        let shares =
            generate_shares(secret, threshold, total_shares).expect("数学测试分片生成失败");

        // 测试任意threshold个分片都能恢复相同结果
        let mut recovered_secrets = Vec::new();

        // 生成所有可能的threshold大小的组合（选择前几个进行测试以避免组合爆炸）
        for start in 0..=(total_shares - threshold).min(6) {
            let selected_shares = &shares[start..start + threshold];
            let recovered = recover_secret(selected_shares, threshold).expect(&format!(
                "数学测试组合[{}:{}]恢复失败",
                start,
                start + threshold
            ));
            recovered_secrets.push(recovered);
        }

        // 验证所有恢复的秘密都相同
        for (i, recovered) in recovered_secrets.iter().enumerate() {
            assert_eq!(*recovered, secret, "数学测试组合{}恢复结果不匹配", i);
        }
        println!("✅ 拉格朗日插值数学正确性验证通过");

        // 测试阈值-1个分片无法恢复（理论上应该随机）
        let insufficient_shares = &shares[0..threshold - 1];
        // 注意：我们的实现可能会返回错误而不是随机值，这是更安全的行为
        let result = recover_secret(insufficient_shares, threshold);
        assert!(result.is_err(), "阈值不足应该失败");
        println!("✅ 阈值不足保护验证通过");
    }

    #[test]
    fn test_field_operations_correctness() {
        println!("=== 有限域运算正确性测试 ===");

        // 通过生成大量不同的分片来间接测试GF(256)运算
        let test_cases = (0u8..=255).step_by(17).collect::<Vec<_>>(); // 采样测试

        for &byte_val in &test_cases {
            let secret = vec![byte_val];

            // 测试不同阈值下的一致性
            for threshold in 2u8..=5 {
                let shares1 =
                    generate_shares(&secret, threshold as usize, (threshold + 2) as usize)
                        .expect(&format!("域运算测试值{}阈值{}失败", byte_val, threshold));

                let recovered = recover_secret(&shares1[0..threshold as usize], threshold as usize)
                    .expect(&format!("域运算恢复值{}阈值{}失败", byte_val, threshold));

                assert_eq!(
                    recovered, secret,
                    "域运算值{}阈值{}恢复不匹配",
                    byte_val, threshold
                );
            }
        }
        println!("✅ 有限域运算正确性验证通过");
    }

    // ============================================================================
    // 安全性和随机性测试
    // ============================================================================

    #[test]
    fn test_share_uniqueness_and_distribution() {
        println!("=== 分片唯一性和分布测试 ===");

        let secret = b"uniqueness test";
        let threshold = 3;
        let total_shares = 20;

        let shares =
            generate_shares(secret, threshold, total_shares).expect("唯一性测试分片生成失败");

        // 测试所有分片的x值唯一
        let x_values: HashSet<u8> = shares.iter().map(|s| s.x.0).collect();
        assert_eq!(x_values.len(), total_shares, "分片x值应该全部唯一");
        println!("✅ 分片x值唯一性验证通过");

        // 测试x值在合理范围内（1-255）
        for (i, share) in shares.iter().enumerate() {
            assert!(share.x.0 >= 1, "分片{}的x值{}应该大于0", i, share.x.0);
            assert_eq!(
                share.y.len(),
                secret.len(),
                "分片{}的y长度{}不匹配秘密长度{}",
                i,
                share.y.len(),
                secret.len()
            );
        }
        println!("✅ 分片x值范围验证通过");

        // 测试不同秘密产生不同分片（在相同x值下）
        let secret2 = b"different secret";
        let shares2 =
            generate_shares(secret2, threshold, total_shares).expect("对比秘密分片生成失败");

        // 比较相同位置的分片应该不同（至少y值不同）
        for i in 0..total_shares {
            if shares[i].x.0 == shares2[i].x.0 {
                // x值相同时
                let y_different = shares[i]
                    .y
                    .iter()
                    .zip(shares2[i].y.iter())
                    .any(|(y1, y2)| y1.0 != y2.0);
                assert!(
                    y_different,
                    "不同秘密在相同x值{}下产生了相同的y值",
                    shares[i].x.0
                );
            }
        }
        println!("✅ 不同秘密分片差异性验证通过");
    }

    // ============================================================================
    // 兼容性和回归测试
    // ============================================================================

    #[test]
    fn test_api_compatibility() {
        println!("=== API兼容性测试 ===");

        // 测试所有公共API都能正常调用
        let secret = b"compatibility test";

        // generate_shares API
        let shares = generate_shares(secret, 3, 5).expect("generate_shares API失败");
        assert_eq!(shares.len(), 5);

        // recover_secret API
        let recovered = recover_secret(&shares[0..3], 3).expect("recover_secret API失败");
        assert_eq!(recovered, secret);

        // U64 APIs
        let secret_u64 = 0x123456789ABCDEFu64;
        let shares_u64 =
            generate_shares_u64(secret_u64, 2, 4).expect("generate_shares_u64 API失败");
        let recovered_u64 =
            recover_secret_u64(&shares_u64[0..2], 2).expect("recover_secret_u64 API失败");
        assert_eq!(recovered_u64, secret_u64);

        // XOR APIs
        let mask = 0xFEDCBA9876543210u64;
        let shares_xor = generate_shares_with_xor(secret_u64, 2, 3, mask)
            .expect("generate_shares_with_xor API失败");
        let recovered_xor = recover_secret_with_xor(&shares_xor[0..2], 2, mask)
            .expect("recover_secret_with_xor API失败");
        assert_eq!(recovered_xor, secret_u64);

        // 工具函数APIs
        let bytes = u64_to_bytes(secret_u64);
        assert_eq!(bytes.len(), 8);
        let converted_back = bytes_to_u64(&bytes).expect("bytes_to_u64 API失败");
        assert_eq!(converted_back, secret_u64);

        // Share序列化APIs
        let share_bytes: Vec<u8> = Vec::from(&shares[0]);
        let share_recovered =
            Share::try_from(share_bytes.as_slice()).expect("Share::try_from API失败");
        assert_eq!(shares[0].x.0, share_recovered.x.0);

        println!("✅ 所有公共API兼容性验证通过");
    }

    #[test]
    fn test_regression_scenarios() {
        println!("=== 回归测试场景 ===");

        // 测试之前可能出现问题的特定场景

        // 场景1: 边界阈值值
        let secret = b"regression test 1";
        let shares = generate_shares(secret, 2, 2).expect("回归测试1失败");
        let recovered = recover_secret(&shares, 2).expect("回归测试1恢复失败");
        assert_eq!(recovered, secret);
        println!("✅ 回归测试1(最小阈值)通过");

        // 场景2: 大阈值
        let shares = generate_shares(secret, 100, 120).expect("回归测试2失败");
        let recovered = recover_secret(&shares[0..100], 100).expect("回归测试2恢复失败");
        assert_eq!(recovered, secret);
        println!("✅ 回归测试2(大阈值)通过");

        // 场景3: 特殊字节值
        let special_bytes = vec![0, 1, 127, 128, 254, 255];
        let shares = generate_shares(&special_bytes, 3, 6).expect("回归测试3失败");
        let recovered = recover_secret(&shares[0..3], 3).expect("回归测试3恢复失败");
        assert_eq!(recovered, special_bytes);
        println!("✅ 回归测试3(特殊字节值)通过");

        // 场景4: U64边界值
        for &value in &[0u64, 1u64, u64::MAX - 1, u64::MAX] {
            let shares =
                generate_shares_u64(value, 2, 3).expect(&format!("回归测试4值{}失败", value));
            let recovered = recover_secret_u64(&shares[0..2], 2)
                .expect(&format!("回归测试4值{}恢复失败", value));
            assert_eq!(recovered, value);
        }
        println!("✅ 回归测试4(U64边界值)通过");

        println!("✅ 所有回归测试场景通过");
    }

    // ============================================================================
    // 综合端到端测试
    // ============================================================================

    #[test]
    fn test_end_to_end_comprehensive() {
        println!("=== 综合端到端测试 ===");

        // 模拟真实使用场景的完整流程
        let original_data = b"This is a comprehensive end-to-end test of the secret sharing system. \
                             It includes various characters: !@#$%^&*()_+-=[]{}|;:,.<>? and numbers 0123456789.";

        println!("原始数据长度: {} bytes", original_data.len());

        // 第1步: 生成分片
        let threshold = 5;
        let total_shares = 10;
        let shares = generate_shares(original_data, threshold, total_shares)
            .expect("端到端测试分片生成失败");

        println!("生成{}个分片，阈值为{}", shares.len(), threshold);

        // 第2步: 序列化分片（模拟存储/传输）
        let serialized_shares: Vec<Vec<u8>> = shares.iter().map(|s| Vec::from(s)).collect();

        println!(
            "序列化完成，每个分片平均大小: {:.1} bytes",
            serialized_shares.iter().map(|s| s.len()).sum::<usize>() as f64
                / serialized_shares.len() as f64
        );

        // 第3步: 反序列化分片（模拟从存储/传输中恢复）
        let deserialized_shares: Vec<Share> = serialized_shares
            .iter()
            .map(|bytes| Share::try_from(bytes.as_slice()).expect("端到端反序列化失败"))
            .collect();

        // 第4步: 使用不同的分片组合恢复数据
        let test_combinations = vec![
            ((0..threshold).collect::<Vec<_>>(), "前N个分片"),
            (
                (total_shares - threshold..total_shares).collect::<Vec<_>>(),
                "后N个分片",
            ),
            (
                (0..total_shares)
                    .step_by(2)
                    .take(threshold)
                    .collect::<Vec<_>>(),
                "间隔分片",
            ),
            (vec![1, 3, 5, 7, 9], "奇数索引分片"),
        ];

        for (indices, description) in test_combinations {
            let selected_shares: Vec<_> = indices
                .iter()
                .map(|&i| deserialized_shares[i].clone())
                .collect();

            if selected_shares.len() >= threshold {
                let recovered = recover_secret(&selected_shares[0..threshold], threshold)
                    .expect(&format!("端到端{}恢复失败", description));

                assert_eq!(
                    recovered, original_data,
                    "端到端{}恢复数据不匹配",
                    description
                );
                println!("✅ {} 恢复成功", description);
            }
        }

        // 第5步: 测试U64端到端流程
        let u64_secret = 0xDEADBEEFCAFEBABEu64;
        let u64_shares = generate_shares_u64(u64_secret, 3, 7).expect("U64端到端分片失败");
        let u64_recovered = recover_secret_u64(&u64_shares[2..5], 3).expect("U64端到端恢复失败");
        assert_eq!(u64_recovered, u64_secret);
        println!("✅ U64端到端测试通过");

        // 第6步: 测试XOR端到端流程
        let xor_mask = 0x123456789ABCDEFu64;
        let xor_shares =
            generate_shares_with_xor(u64_secret, 3, 6, xor_mask).expect("XOR端到端分片失败");
        let xor_recovered =
            recover_secret_with_xor(&xor_shares[1..4], 3, xor_mask).expect("XOR端到端恢复失败");
        assert_eq!(xor_recovered, u64_secret);
        println!("✅ XOR端到端测试通过");

        println!("✅ 综合端到端测试全部通过");
    }

    // ============================================================================
    // 数字分片一致性专项测试
    // ============================================================================

    #[test]
    fn test_numerical_share_consistency() {
        println!("=== 数字分片一致性专项测试 ===");

        // 测试不同数值的分片一致性
        let test_numbers = vec![
            0u64,                  // 零值
            1u64,                  // 最小正值
            42u64,                 // 小整数
            1000u64,               // 千位数
            65536u64,              // 2^16
            u64::MAX / 2,          // 中位值
            u64::MAX - 1,          // 接近最大值
            u64::MAX,              // 最大值
            0x123456789ABCDEFu64,  // 十六进制模式
            0xDEADBEEFCAFEBABEu64, // 特殊模式
            0xAAAAAAAAAAAAAAAAu64, // 交替位模式
            0x5555555555555555u64, // 反向交替位
        ];

        for &number in &test_numbers {
            println!("测试数字: 0x{:016X} ({})", number, number);

            // 使用不同阈值和分片数测试一致性
            for threshold in 2u8..=5 {
                for total_shares in (threshold as usize)..=8 {
                    // 第一次生成分片
                    let shares1 = generate_shares_u64(number, threshold as usize, total_shares)
                        .expect(&format!("数字{}第一次分片生成失败", number));

                    // 第二次生成分片 (应该相同，因为固定种子)
                    let shares2 = generate_shares_u64(number, threshold as usize, total_shares)
                        .expect(&format!("数字{}第二次分片生成失败", number));

                    // 验证两次生成的分片完全一致
                    assert_eq!(
                        shares1.len(),
                        shares2.len(),
                        "数字{}分片数量不一致: {} vs {}",
                        number,
                        shares1.len(),
                        shares2.len()
                    );

                    for (i, (share1, share2)) in shares1.iter().zip(shares2.iter()).enumerate() {
                        assert_eq!(
                            share1.x.0, share2.x.0,
                            "数字{}分片{}的x值不一致: {} vs {}",
                            number, i, share1.x.0, share2.x.0
                        );

                        assert_eq!(
                            share1.y.len(),
                            share2.y.len(),
                            "数字{}分片{}的y长度不一致: {} vs {}",
                            number,
                            i,
                            share1.y.len(),
                            share2.y.len()
                        );

                        for (j, (y1, y2)) in share1.y.iter().zip(share2.y.iter()).enumerate() {
                            assert_eq!(
                                y1.0, y2.0,
                                "数字{}分片{}的y[{}]不一致: {} vs {}",
                                number, i, j, y1.0, y2.0
                            );
                        }
                    }

                    // 验证所有可能的分片组合都能恢复相同结果
                    let mut recovered_values = Vec::new();

                    // 测试不同的分片组合
                    for start in 0..=(total_shares - threshold as usize) {
                        let selected_shares = &shares1[start..start + threshold as usize];
                        let recovered = recover_secret_u64(selected_shares, threshold as usize)
                            .expect(&format!(
                                "数字{}恢复失败，组合[{}:{}]",
                                number,
                                start,
                                start + threshold as usize
                            ));

                        recovered_values.push(recovered);
                        assert_eq!(
                            recovered, number,
                            "数字{}恢复值{}不匹配原值{}",
                            number, recovered, number
                        );
                    }

                    // 验证所有恢复值都相同
                    for (i, &recovered) in recovered_values.iter().enumerate() {
                        assert_eq!(
                            recovered, number,
                            "数字{}恢复组合{}结果{}不一致",
                            number, i, recovered
                        );
                    }

                    println!("  ✅ 阈值{}分片数{}一致性验证通过", threshold, total_shares);
                }
            }

            println!("✅ 数字0x{:016X}一致性测试全部通过", number);
        }

        println!("✅ 数字分片一致性专项测试全部通过");
    }

    #[test]
    fn test_numerical_cross_validation() {
        println!("=== 数字交叉验证测试 ===");

        // 测试相同数值用不同方式生成的分片交叉兼容性
        let test_number = 0x123456789ABCDEFu64;
        let threshold = 3usize;
        let total_shares = 6usize;

        println!("测试数字: 0x{:016X}", test_number);

        // 方式1: 直接使用U64接口
        let shares_u64 =
            generate_shares_u64(test_number, threshold, total_shares).expect("U64接口分片生成失败");

        // 方式2: 转换为字节后使用通用接口
        let number_bytes = u64_to_bytes(test_number);
        let shares_bytes =
            generate_shares(&number_bytes, threshold, total_shares).expect("字节接口分片生成失败");

        println!(
            "U64接口生成{}个分片，字节接口生成{}个分片",
            shares_u64.len(),
            shares_bytes.len()
        );

        // 验证两种方式生成的分片应该相同（因为都使用固定种子）
        assert_eq!(
            shares_u64.len(),
            shares_bytes.len(),
            "两种接口生成的分片数量不同"
        );

        for (i, (share_u64, share_bytes)) in shares_u64.iter().zip(shares_bytes.iter()).enumerate()
        {
            assert_eq!(
                share_u64.x.0, share_bytes.x.0,
                "分片{}的x值不同: U64={} vs 字节={}",
                i, share_u64.x.0, share_bytes.x.0
            );

            assert_eq!(
                share_u64.y.len(),
                share_bytes.y.len(),
                "分片{}的y长度不同: U64={} vs 字节={}",
                i,
                share_u64.y.len(),
                share_bytes.y.len()
            );

            for (j, (y_u64, y_bytes)) in share_u64.y.iter().zip(share_bytes.y.iter()).enumerate() {
                assert_eq!(
                    y_u64.0, y_bytes.0,
                    "分片{}的y[{}]不同: U64={} vs 字节={}",
                    i, j, y_u64.0, y_bytes.0
                );
            }
        }

        // 交叉验证恢复
        // 用U64分片通过字节接口恢复
        let recovered_bytes_from_u64 =
            recover_secret(&shares_u64[0..threshold], threshold).expect("从U64分片恢复字节失败");
        let recovered_number_from_bytes =
            bytes_to_u64(&recovered_bytes_from_u64).expect("字节转U64失败");

        // 用字节分片通过U64接口恢复
        let recovered_number_from_u64 = recover_secret_u64(&shares_bytes[0..threshold], threshold)
            .expect("从字节分片恢复U64失败");

        // 验证所有方式恢复的结果都相同
        assert_eq!(
            recovered_number_from_bytes, test_number,
            "U64分片->字节->U64路径恢复错误"
        );
        assert_eq!(
            recovered_number_from_u64, test_number,
            "字节分片->U64路径恢复错误"
        );
        assert_eq!(
            recovered_number_from_bytes, recovered_number_from_u64,
            "两种恢复路径结果不一致"
        );

        println!("✅ 原始数字: 0x{:016X}", test_number);
        println!(
            "✅ U64分片->字节->U64恢复: 0x{:016X}",
            recovered_number_from_bytes
        );
        println!("✅ 字节分片->U64恢复: 0x{:016X}", recovered_number_from_u64);
        println!("✅ 数字交叉验证测试通过");
    }

    #[test]
    fn test_numerical_bit_pattern_consistency() {
        println!("=== 数字位模式一致性测试 ===");

        // 测试特定位模式的数字分片一致性
        let bit_patterns = vec![
            (0x0000000000000000u64, "全零位"),
            (0xFFFFFFFFFFFFFFFFu64, "全一位"),
            (0x0000000000000001u64, "最低位"),
            (0x8000000000000000u64, "最高位"),
            (0x0000000000000000u64 | (1u64 << 31), "第32位"),
            (0xAAAAAAAAAAAAAAAAu64, "交替位(1010...)"),
            (0x5555555555555555u64, "交替位(0101...)"),
            (0x0F0F0F0F0F0F0F0Fu64, "半字节模式"),
            (0xF0F0F0F0F0F0F0F0u64, "反半字节模式"),
            (0x00FF00FF00FF00FFu64, "字节模式"),
            (0xFF00FF00FF00FF00u64, "反字节模式"),
            (0x0000FFFF0000FFFFu64, "双字节模式"),
            (0xFFFF0000FFFF0000u64, "反双字节模式"),
        ];

        let threshold = 3usize;
        let total_shares = 5usize;

        for (pattern, description) in bit_patterns {
            println!("测试位模式: 0x{:016X} ({})", pattern, description);

            // 多次生成和恢复，验证一致性
            let mut all_recoveries = Vec::new();

            for iteration in 0..5 {
                // 生成分片
                let shares = generate_shares_u64(pattern, threshold, total_shares).expect(
                    &format!("位模式{}第{}次分片生成失败", description, iteration),
                );

                // 验证分片结构合理性
                assert_eq!(shares.len(), total_shares);
                for (i, share) in shares.iter().enumerate() {
                    assert!(share.x.0 >= 1, "分片{}的x值{}应该大于0", i, share.x.0);
                    assert_eq!(
                        share.y.len(),
                        8,
                        "U64分片的y长度应该是8，实际{}",
                        share.y.len()
                    );
                }

                // 多种组合恢复
                let combinations: Vec<(Vec<usize>, &str)> = vec![
                    ((0..threshold).collect(), "前N个"),
                    (
                        ((total_shares - threshold)..total_shares).collect(),
                        "后N个",
                    ),
                    (vec![0, 2, 4], "间隔分片"),
                ];

                for (selected_indices, combo_desc) in combinations {
                    if selected_indices.len() >= threshold {
                        let selected_shares: Vec<_> = selected_indices
                            .iter()
                            .take(threshold)
                            .map(|&i| shares[i].clone())
                            .collect();

                        let recovered =
                            recover_secret_u64(&selected_shares, threshold).expect(&format!(
                                "位模式{}{}第{}次恢复失败",
                                description, combo_desc, iteration
                            ));

                        all_recoveries.push((iteration, combo_desc, recovered));
                        assert_eq!(
                            recovered, pattern,
                            "位模式{}{}第{}次恢复值0x{:016X}不匹配原值0x{:016X}",
                            description, combo_desc, iteration, recovered, pattern
                        );
                    }
                }
            }

            // 验证所有恢复结果都完全一致
            for (iteration, combo_desc, recovered) in all_recoveries {
                assert_eq!(
                    recovered, pattern,
                    "位模式{}在第{}次{}恢复中结果0x{:016X}不一致",
                    description, iteration, combo_desc, recovered
                );
            }

            println!("  ✅ 位模式 {} 一致性验证通过", description);
        }

        println!("✅ 数字位模式一致性测试全部通过");
    }

    #[test]
    fn test_numerical_precision_consistency() {
        println!("=== 数字精度一致性测试 ===");

        // 测试数字在分片过程中是否保持精确精度
        let precision_tests = vec![
            // 边界值测试
            (0u64, "零值精度"),
            (1u64, "单位精度"),
            (u64::MAX, "最大值精度"),
            // 幂次测试
            (2u64.pow(0), "2^0精度"),
            (2u64.pow(1), "2^1精度"),
            (2u64.pow(8), "2^8精度"),
            (2u64.pow(16), "2^16精度"),
            (2u64.pow(32), "2^32精度"),
            (2u64.pow(63), "2^63精度"),
            // 十进制边界
            (10u64.pow(0), "10^0精度"),
            (10u64.pow(1), "10^1精度"),
            (10u64.pow(9), "10^9精度"),
            (10u64.pow(18), "10^18精度"),
            // 特殊数值
            (u64::MAX / 3, "最大值/3精度"),
            (u64::MAX / 7, "最大值/7精度"),
            (u64::MAX - 42, "最大值-42精度"),
        ];

        for (original_value, description) in precision_tests {
            println!("测试精度: {} ({})", original_value, description);

            // 使用不同的阈值配置测试
            for threshold in 2u8..=4 {
                for total_shares in (threshold as usize)..=7 {
                    // 生成分片
                    let shares =
                        generate_shares_u64(original_value, threshold as usize, total_shares)
                            .expect(&format!("{}分片生成失败", description));

                    // 测试每一种可能的恢复组合
                    let total_combinations = (0..=(total_shares - threshold as usize))
                        .map(|start| (start, start + threshold as usize))
                        .collect::<Vec<_>>();

                    for (start, end) in total_combinations {
                        let recovered = recover_secret_u64(&shares[start..end], threshold as usize)
                            .expect(&format!("{}恢复失败[{}:{}]", description, start, end));

                        // 验证精确匹配 - 不允许任何精度损失
                        assert_eq!(
                            recovered, original_value,
                            "{}: 原值={}, 恢复值={}, 阈值={}, 组合[{}:{}] - 精度丢失!",
                            description, original_value, recovered, threshold, start, end
                        );

                        // 额外验证：位级别的完全一致性
                        let original_bits = format!("{:064b}", original_value);
                        let recovered_bits = format!("{:064b}", recovered);
                        assert_eq!(
                            original_bits, recovered_bits,
                            "{}: 位级别不匹配\n原值: {}\n恢复: {}",
                            description, original_bits, recovered_bits
                        );
                    }
                }
            }

            println!("  ✅ {} 精度测试通过", description);
        }

        println!("✅ 数字精度一致性测试全部通过 - 无任何精度损失");
    }
}

// 运行所有测试的便利函数
pub fn run_all_comprehensive_tests() {
    println!("🚀 开始运行全面测试套件...\n");
    println!("建议运行: cargo test comprehensive_tests -- --nocapture");
    println!("这将运行所有{}个comprehensive测试模块\n", 20);

    println!("测试覆盖范围:");
    println!("- 基本功能测试 (3个测试)");
    println!("- 边界条件测试 (2个测试)");
    println!("- 错误处理测试 (2个测试)");
    println!("- 性能压力测试 (1个测试)");
    println!("- 确定性测试 (1个测试)");
    println!("- 序列化测试 (1个测试)");
    println!("- 数学正确性测试 (2个测试)");
    println!("- 安全性测试 (1个测试)");
    println!("- 兼容性测试 (1个测试)");
    println!("- 回归测试 (1个测试)");
    println!("- 端到端测试 (1个测试)");
    println!("- 数字分片一致性测试 (4个测试) ⭐新增⭐");
}

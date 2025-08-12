use std::env::args;

#[allow(unused_imports)]
use mpc_transmission::{
    SecretSharingError, generate_shares_with_xor,
    math::{add_shared_secrets, mul_shared_secrets, sub_shared_secrets},
    read::{get_mask_by_key, read_numeric_json},
};

fn main() -> Result<(), SecretSharingError> {
    // 基本配置
    let threshold = 5; // 恢复阈值
    let total_shares = 10; // 总分片数

    // 1. 获取文件路径参数或使用默认值
    let file_path = args()
        .nth(1)
        .unwrap_or_else(|| "data.json".to_string());

    // 2. 读取JSON文件
    println!("正在加载mask文件: {}", file_path);
    let mask_data = read_numeric_json(&file_path)?;

    // 3. 显示所有可用key
    println!("可用mask keys: {:?}", mask_data.keys().collect::<Vec<_>>());

    // 4. 获取用户输入的key（这里简化为硬编码，实际可用std::io读取用户输入）
    let selected_key = 2; // 示例key
    println!("获取key {} 对应的mask值", selected_key);

    // 5. 获取并验证mask值
    let mask = get_mask_by_key(&mask_data, selected_key)?;
    println!("成功获取mask值: {}", mask);

    // 测试案例1：普通数值
    println!("=== 测试普通数值 ===");
    println!("使用的 mask: {}", mask);
    test_operations(1213, 425, threshold, total_shares, mask)?;

    // 测试案例2：边界值
    println!("\n=== 测试边界值 ===");
    println!("使用的 mask: {}", mask);
    test_operations(u64::MAX, 1, threshold, total_shares, mask)?;

    println!("使用的 mask: {}", mask);
    test_operations(0, u64::MAX, threshold, total_shares, mask)?;

    // 测试案例3：零值
    println!("\n=== 测试零值 ===");
    println!("使用的 mask: {}", mask);
    test_operations(0, 0, threshold, total_shares, mask)?;

    Ok(())
}

/// 测试三种运算的辅助函数
fn test_operations(
    a: u64,
    b: u64,
    threshold: usize,
    total_shares: usize,
    mask: u64,
) -> Result<(), SecretSharingError> {
    // 生成分片 (使用相同的 mask)
    let shares_a = generate_shares_with_xor(a, threshold, total_shares, mask)?;
    let shares_b = generate_shares_with_xor(b, threshold, total_shares, mask)?;

    // 使用前threshold个分片进行计算 (传递相同的 mask)
    let add_result = add_shared_secrets(
        &shares_a[..threshold],
        &shares_b[..threshold],
        threshold,
        mask, // 使用相同的 mask
        mask, // 使用相同的 mask
    )?;

    let sub_result = sub_shared_secrets(
        &shares_a[..threshold],
        &shares_b[..threshold],
        threshold,
        mask,
        mask,
    )?;

    let mul_result = mul_shared_secrets(
        &shares_a[..threshold],
        &shares_b[..threshold],
        threshold,
        mask,
        mask,
    )?;

    // 验证结果
    println!("原始值: a = {}, b = {}", a, b);
    println!("加法结果: {} (期望: {})", add_result, a.wrapping_add(b));
    println!("减法结果: {} (期望: {})", sub_result, a.wrapping_sub(b));
    println!("乘法结果: {} (期望: {})", mul_result, a.wrapping_mul(b));

    assert_eq!(add_result, a.wrapping_add(b));
    assert_eq!(sub_result, a.wrapping_sub(b));
    assert_eq!(mul_result, a.wrapping_mul(b));

    Ok(())
}

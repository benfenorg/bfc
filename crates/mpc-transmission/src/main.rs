use std::env::args;

#[allow(unused_imports)]
use mpc_transmission::{
    generate_shares_with_xor,
    math::{add_shared_secrets, mul_shared_secrets, sub_shared_secrets},
    read::{get_mask_by_key, read_numeric_json},
    error::SecretSharingError,
};

fn main() -> Result<(), SecretSharingError> {
    // Basic configuration
    let threshold = 5; // Recovery threshold
    let total_shares = 10; // Total number of shares

    // 1. Get file path parameter or use default value
    let file_path = args().nth(1).unwrap_or_else(|| "data.json".to_string());

    // 2. Read JSON file
    println!("Loading mask file: {}", file_path);
    let mask_data = read_numeric_json(&file_path)?;

    // 3. Display all available keys
    println!(
        "Available mask keys: {:?}",
        mask_data.keys().collect::<Vec<_>>()
    );

    // 4. Get user input key (simplified as hardcoded here, can use std::io to read user input in practice)
    let selected_key = 2; // Example key
    println!("Getting mask value for key {}", selected_key);

    // 5. Get and validate mask value
    let mask = get_mask_by_key(&mask_data, selected_key)?;
    println!("Successfully obtained mask value: {}", mask);

    // Test case 1: Normal values
    println!("=== Testing Normal Values ===");
    println!("Using mask: {}", mask);
    test_operations(1213, 425, threshold, total_shares, mask)?;

    // Test case 2: Boundary values
    println!("\n=== Testing Boundary Values ===");
    println!("Using mask: {}", mask);
    test_operations(u64::MAX, 1, threshold, total_shares, mask)?;

    println!("Using mask: {}", mask);
    test_operations(0, u64::MAX, threshold, total_shares, mask)?;

    // Test case 3: Zero values
    println!("\n=== Testing Zero Values ===");
    println!("Using mask: {}", mask);
    test_operations(0, 0, threshold, total_shares, mask)?;

    Ok(())
}

/// Helper function to test three operations
fn test_operations(
    a: u64,
    b: u64,
    threshold: usize,
    total_shares: usize,
    mask: u64,
) -> Result<(), SecretSharingError> {
    // Generate shares (using the same mask)
    let shares_a = generate_shares_with_xor(a, threshold, total_shares, mask)?;
    let shares_b = generate_shares_with_xor(b, threshold, total_shares, mask)?;

    // Use the first threshold shares for computation (pass the same mask)
    let add_result = add_shared_secrets(
        &shares_a[..threshold],
        &shares_b[..threshold],
        threshold,
        mask, // Use the same mask
        mask, // Use the same mask
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

    // Verify results
    println!("Original values: a = {}, b = {}", a, b);
    println!(
        "Addition result: {} (expected: {})",
        add_result,
        a.wrapping_add(b)
    );
    println!(
        "Subtraction result: {} (expected: {})",
        sub_result,
        a.wrapping_sub(b)
    );
    println!(
        "Multiplication result: {} (expected: {})",
        mul_result,
        a.wrapping_mul(b)
    );

    assert_eq!(add_result, a.wrapping_add(b));
    assert_eq!(sub_result, a.wrapping_sub(b));
    assert_eq!(mul_result, a.wrapping_mul(b));

    Ok(())
}

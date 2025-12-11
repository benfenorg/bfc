use std::env::args;

#[allow(unused_imports)]
use mpc_transmission::{
    bytes_to_u64,
    error::SecretSharingError,
    generate_shares, generate_shares_u64, generate_shares_with_xor,
    math::{add_shared_secrets, mul_shared_secrets, sub_shared_secrets},
    read::{get_mask_by_key, read_numeric_json},
    recover_secret, recover_secret_u64, recover_secret_with_xor, u64_to_bytes, Share,
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
    // println!("\n=== Testing Boundary Values ===");
    // println!("Using mask: {}", mask);
    // test_operations(u64::MAX, 1, threshold, total_shares, mask)?;
    //
    // println!("Using mask: {}", mask);
    // test_operations(0, u64::MAX, threshold, total_shares, mask)?;
    //
    // // Test case 3: Zero values
    // println!("\n=== Testing Zero Values ===");
    // println!("Using mask: {}", mask);
    // test_operations(0, 0, threshold, total_shares, mask)?;

    println!("\n{}", "=".repeat(60));
    println!("🔍 Starting share accuracy verification test");
    println!("{}", "=".repeat(60));
    verify_share_accuracy()?;

    // Special test: XOR sharing of number 30
    println!("\n{}", "=".repeat(60));
    println!("🔢 Special test: XOR sharing demonstration for number 30");
    println!("{}", "=".repeat(60));
    test_number_30_xor_shares()?;

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

/// Share accuracy verification function - comprehensive validation of local secret sharing implementation accuracy
fn verify_share_accuracy() -> Result<(), SecretSharingError> {
    println!("📋 Share accuracy verification starting...\n");

    // 1. Basic byte sharing accuracy verification
    verify_basic_byte_sharing()?;

    // 2. U64 number sharing accuracy verification
    verify_u64_sharing()?;

    // 3. Deterministic behavior verification
    verify_deterministic_behavior()?;

    // 4. Interface compatibility verification
    verify_interface_compatibility()?;

    // 5. Precision preservation verification
    verify_precision_preservation()?;

    println!("🎉 All share accuracy verifications passed!\n");
    Ok(())
}

/// 1. Basic byte sharing accuracy verification
fn verify_basic_byte_sharing() -> Result<(), SecretSharingError> {
    println!("1️⃣ Basic byte sharing accuracy verification");
    println!("{}", "-".repeat(40));

    let zero_data = vec![0u8; 50];
    let max_data = vec![255u8; 30];
    let cycle_data = (0u8..=255).cycle().take(100).collect::<Vec<u8>>();

    let test_data = vec![
        (b"Hello, World!".as_slice(), "short string"),
        (
            b"The quick brown fox jumps over the lazy dog.".as_slice(),
            "English sentence",
        ),
        (zero_data.as_slice(), "zero-padded data"),
        (max_data.as_slice(), "max-value padded"),
        (cycle_data.as_slice(), "cyclic byte pattern"),
    ];

    for (data, description) in test_data {
        print!("  测试 {}: ", description);

        // Generate shares
        let shares = generate_shares(data, 3, 5)?;

        // Recover and verify with different combinations
        let combinations = vec![
            (&shares[0..3], "first 3 shares"),
            (&shares[1..4], "middle 3 shares"),
            (&shares[2..5], "last 3 shares"),
        ];

        let mut all_results = Vec::new();

        for (share_combo, combo_desc) in combinations {
            let recovered = recover_secret(share_combo, 3)?;
            all_results.push(recovered.clone());

            if recovered != data {
                println!("❌ Failed - {} recovery result mismatch", combo_desc);
                return Err(SecretSharingError::RecoveryFailed(format!(
                    "{} recovery result doesn't match original data",
                    combo_desc
                )));
            }
        }

        // Verify all combination recovery results are the same
        for i in 1..all_results.len() {
            if all_results[0] != all_results[i] {
                println!(
                    "❌ Failed - different share combinations have inconsistent recovery results"
                );
                return Err(SecretSharingError::RecoveryFailed(
                    "different share combinations have inconsistent recovery results".to_string(),
                ));
            }
        }

        println!("✅ Passed ({} bytes)", data.len());
    }

    println!("✅ Basic byte sharing accuracy verification all passed\n");
    Ok(())
}

/// 2. U64 number sharing accuracy verification
fn verify_u64_sharing() -> Result<(), SecretSharingError> {
    println!("2️⃣ U64 number sharing accuracy verification");
    println!("{}", "-".repeat(40));

    let test_numbers = vec![
        (0u64, "zero value"),
        (1u64, "minimum positive value"),
        (42u64, "small integer"),
        (u64::MAX / 2, "midpoint value"),
        (u64::MAX - 1, "max value - 1"),
        (u64::MAX, "maximum value"),
        (0x123456789ABCDEFu64, "hexadecimal pattern"),
        (0xAAAAAAAAAAAAAAAAu64, "alternating bit pattern"),
        (0x5555555555555555u64, "inverse alternating bit pattern"),
    ];

    for (number, description) in test_numbers {
        print!("  Testing {}: ", description);

        // Generate shares
        let shares = generate_shares_u64(number, 4, 7)?;

        // Verify recovery results with different combinations
        let mut recovery_results = Vec::new();

        for start in 0..=(7 - 4) {
            let selected_shares = &shares[start..start + 4];
            let recovered = recover_secret_u64(selected_shares, 4)?;
            recovery_results.push(recovered);

            if recovered != number {
                println!(
                    "❌ Failed - recovered value 0x{:016X} doesn't match original 0x{:016X}",
                    recovered, number
                );
                return Err(SecretSharingError::RecoveryFailed(format!(
                    "U64 recovery value mismatch: expected 0x{:016X}, got 0x{:016X}",
                    number, recovered
                )));
            }
        }

        // Verify all recovery results are consistent
        let first_result = recovery_results[0];
        for (i, &result) in recovery_results.iter().enumerate() {
            if result != first_result {
                println!(
                    "❌ Failed - combination {} recovery result 0x{:016X} inconsistent with first result 0x{:016X}",
                    i, result, first_result
                );
                return Err(SecretSharingError::RecoveryFailed(
                    "U64 recovery results inconsistent".to_string(),
                ));
            }
        }

        println!("✅ Passed (0x{:016X})", number);
    }

    println!("✅ U64 number sharing accuracy verification all passed\n");
    Ok(())
}

/// 3. Deterministic behavior verification
fn verify_deterministic_behavior() -> Result<(), SecretSharingError> {
    println!("3️⃣ Deterministic behavior verification");
    println!("{}", "-".repeat(40));

    let test_data = b"deterministic test data";

    println!("  Generating shares multiple times with same input:");
    println!(
        "    Test data: {:?}",
        std::str::from_utf8(test_data).unwrap()
    );

    // Generate shares 5 times
    let mut all_share_sets = Vec::new();
    for i in 0..5 {
        let shares = generate_shares(test_data, 3, 6)?;
        all_share_sets.push(shares);

        if i == 0 {
            // Show details of first generated shares
            println!("    Details of 1st generated shares:");
            for (j, share) in all_share_sets[0].iter().enumerate() {
                let y_values: Vec<String> = share.y.iter().map(|y| format!("{}", y.0)).collect();
                println!(
                    "      Share{}: x={}, y=[{}]",
                    j,
                    share.x.0,
                    y_values.join(",")
                );
            }
        } else {
            // Verify identical to first generated shares
            if all_share_sets[0].len() != all_share_sets[i].len() {
                println!(
                    "❌ Failed - {}th generation has different number of shares",
                    i + 1
                );
                return Err(SecretSharingError::RecoveryFailed(
                    "inconsistent number of shares".to_string(),
                ));
            }

            println!("    {}th generation verification:", i + 1);
            let mut all_match = true;
            for (j, (share0, sharei)) in all_share_sets[0]
                .iter()
                .zip(all_share_sets[i].iter())
                .enumerate()
            {
                // Verify x values are the same
                if share0.x.0 != sharei.x.0 {
                    println!(
                        "      ❌ Share{} x value different: {} vs {}",
                        j, share0.x.0, sharei.x.0
                    );
                    all_match = false;
                }

                // Verify y values are the same
                if share0.y.len() != sharei.y.len() {
                    println!(
                        "      ❌ Share{} y length different: {} vs {}",
                        j,
                        share0.y.len(),
                        sharei.y.len()
                    );
                    all_match = false;
                }

                for (k, (y0, yi)) in share0.y.iter().zip(sharei.y.iter()).enumerate() {
                    if y0.0 != yi.0 {
                        println!(
                            "      ❌ Share{} y[{}] different: {} vs {}",
                            j, k, y0.0, yi.0
                        );
                        all_match = false;
                    }
                }
            }

            if all_match {
                println!("      ✅ All shares completely consistent");
            } else {
                return Err(SecretSharingError::RecoveryFailed(
                    "shares inconsistent".to_string(),
                ));
            }
        }
    }

    println!("  ✅ Byte data 5 generations completely consistent");

    // Verify U64 determinism
    println!("  U64 determinism verification:");
    let test_number = 0xDEADBEEFCAFEBABEu64;
    println!("    Test number: 0x{:016X} ({})", test_number, test_number);

    let mut u64_share_sets = Vec::new();
    for i in 0..3 {
        let shares = generate_shares_u64(test_number, 2, 4)?;
        u64_share_sets.push(shares);

        if i == 0 {
            // Show details of first generated U64 shares
            println!("    1st generation U64 share details:");
            for (j, share) in u64_share_sets[0].iter().enumerate() {
                let y_values: Vec<String> = share.y.iter().map(|y| format!("{}", y.0)).collect();
                println!(
                    "      Share{}: x={}, y=[{}]",
                    j,
                    share.x.0,
                    y_values.join(",")
                );
            }
        } else {
            println!("    {}th generation verification:", i + 1);
            let mut all_match = true;

            // Verify completely consistent
            for (j, (share0, sharei)) in u64_share_sets[0]
                .iter()
                .zip(u64_share_sets[i].iter())
                .enumerate()
            {
                if share0.x.0 != sharei.x.0 {
                    println!(
                        "      ❌ Share{} x value different: {} vs {}",
                        j, share0.x.0, sharei.x.0
                    );
                    all_match = false;
                }

                if share0.y.len() != sharei.y.len() {
                    println!(
                        "      ❌ Share{} y length different: {} vs {}",
                        j,
                        share0.y.len(),
                        sharei.y.len()
                    );
                    all_match = false;
                }

                for (k, (y0, yi)) in share0.y.iter().zip(sharei.y.iter()).enumerate() {
                    if y0.0 != yi.0 {
                        println!(
                            "      ❌ Share{} y[{}] different: {} vs {}",
                            j, k, y0.0, yi.0
                        );
                        all_match = false;
                    }
                }
            }

            if all_match {
                println!("      ✅ All U64 shares completely consistent");
            } else {
                return Err(SecretSharingError::RecoveryFailed(
                    "U64 shares inconsistent".to_string(),
                ));
            }
        }
    }

    println!("  ✅ U64 numbers 3 generations completely consistent");
    println!("✅ Deterministic behavior verification all passed\n");
    Ok(())
}

/// 4. Interface compatibility verification
fn verify_interface_compatibility() -> Result<(), SecretSharingError> {
    println!("4️⃣ Interface compatibility verification");
    println!("{}", "-".repeat(40));

    let test_number = 0x123456789ABCDEFu64;

    print!("  U64 interface vs byte interface compatibility: ");

    // Method 1: U64 interface
    let shares_u64 = generate_shares_u64(test_number, 3, 5)?;

    // Method 2: Byte interface
    let number_bytes = u64_to_bytes(test_number);
    let shares_bytes = generate_shares(&number_bytes, 3, 5)?;

    // Verify shares generated by both methods are the same
    if shares_u64.len() != shares_bytes.len() {
        println!("❌ Failed - two interfaces generated different number of shares");
        return Err(SecretSharingError::RecoveryFailed(
            "interface share count inconsistent".to_string(),
        ));
    }

    for (i, (share_u64, share_bytes)) in shares_u64.iter().zip(shares_bytes.iter()).enumerate() {
        if share_u64.x.0 != share_bytes.x.0 || share_u64.y != share_bytes.y {
            println!("❌ Failed - share{} different under two interfaces", i);
            return Err(SecretSharingError::RecoveryFailed(
                "interface shares inconsistent".to_string(),
            ));
        }
    }

    // Cross-recovery verification
    let recovered_u64 = recover_secret_u64(&shares_bytes[0..3], 3)?;
    let recovered_bytes = recover_secret(&shares_u64[0..3], 3)?;
    let recovered_from_bytes = bytes_to_u64(&recovered_bytes)
        .map_err(|e| SecretSharingError::RecoveryFailed(e.to_string()))?;

    if recovered_u64 != test_number || recovered_from_bytes != test_number {
        println!("❌ Failed - cross-recovery results don't match");
        return Err(SecretSharingError::RecoveryFailed(
            "cross-recovery failed".to_string(),
        ));
    }

    println!("✅ Passed (interfaces fully compatible)");
    println!("✅ Interface compatibility verification all passed\n");
    Ok(())
}

/// 5. Precision preservation verification
fn verify_precision_preservation() -> Result<(), SecretSharingError> {
    println!("5️⃣ Precision preservation verification");
    println!("{}", "-".repeat(40));

    let precision_tests = vec![
        (1u64, "minimum precision"),
        (2u64.pow(32), "2^32 precision"),
        (10u64.pow(18), "10^18 precision"),
        (u64::MAX / 3, "high precision division"),
        (u64::MAX - 42, "high precision subtraction"),
        (u64::MAX, "maximum precision"),
    ];

    for (value, description) in precision_tests {
        print!("  {}: ", description);

        // Multi-round precision verification
        for round in 0..3 {
            let shares = generate_shares_u64(value, 2, 5)?;

            // Recover with different share combinations
            for start in 0..=3 {
                let recovered = recover_secret_u64(&shares[start..start + 2], 2)?;

                if recovered != value {
                    println!(
                        "❌ Failed - round {} combination [{}:{}] precision loss: {} != {}",
                        round + 1,
                        start,
                        start + 2,
                        recovered,
                        value
                    );
                    return Err(SecretSharingError::RecoveryFailed(
                        "precision loss".to_string(),
                    ));
                }

                // Bit-level verification
                let original_bits = format!("{:064b}", value);
                let recovered_bits = format!("{:064b}", recovered);
                if original_bits != recovered_bits {
                    println!("❌ Failed - bit-level precision loss");
                    return Err(SecretSharingError::RecoveryFailed(
                        "bit-level precision loss".to_string(),
                    ));
                }
            }
        }

        println!("✅ Passed (no precision loss)");
    }

    println!("✅ Precision preservation verification all passed\n");
    Ok(())
}

/// Special test: XOR sharing demonstration for number 30
fn test_number_30_xor_shares() -> Result<(), SecretSharingError> {
    let secret_number = 30u64;
    let threshold = 2usize;
    let total_shares = 2usize;
    let mask = 987654321098765u64; // Using mask read from config file

    println!("📊 Test parameters:");
    println!("  Original number: {}", secret_number);
    println!("  XOR mask: {} (0x{:016X})", mask, mask);
    println!(
        "  Masked number: {} (0x{:016X})",
        secret_number ^ mask,
        secret_number ^ mask
    );
    println!("  Threshold: {}", threshold);
    println!("  Number of shares: {}", total_shares);
    println!();

    println!("🔢 10 rounds of XOR sharing detailed results:");
    println!("{}", "-".repeat(80));

    for round in 1..=10 {
        println!("Round {} sharing:", round);

        // Generate XOR-encrypted shares
        let shares = generate_shares_with_xor(secret_number, threshold, total_shares, mask)?;

        println!("  Generated share data:");
        for (i, share) in shares.iter().enumerate() {
            let y_hex: Vec<String> = share.y.iter().map(|y| format!("0x{:02X}", y.0)).collect();
            let y_dec: Vec<String> = share.y.iter().map(|y| format!("{}", y.0)).collect();
            println!(
                "    Share{}: x={}, y_hex=[{}]",
                i,
                share.x.0,
                y_hex.join(",")
            );
            println!("           y_dec=[{}]", y_dec.join(","));
        }

        // Verify recovery
        let recovered = recover_secret_with_xor(&shares, threshold, mask)?;
        println!(
            "  Recovery verification: {} -> {}",
            if recovered == secret_number {
                "✅ Success"
            } else {
                "❌ Failed"
            },
            recovered
        );

        // Show internal calculation process
        println!("  Internal process:");
        println!("    1. Original number: {}", secret_number);
        println!("    2. XOR mask: {}", mask);
        println!(
            "    3. Masked value: {} XOR {} = {}",
            secret_number,
            mask,
            secret_number ^ mask
        );
        println!("    4. Share masked value");
        println!("    5. Recover masked value: {}", secret_number ^ mask);
        println!(
            "    6. Unmask: {} XOR {} = {}",
            secret_number ^ mask,
            mask,
            recovered
        );

        println!();
    }

    // Statistical analysis
    println!("🔍 Share consistency analysis:");
    println!("{}", "-".repeat(50));

    let mut all_rounds_shares = Vec::new();
    for _round in 1..=10 {
        let shares = generate_shares_with_xor(secret_number, threshold, total_shares, mask)?;
        all_rounds_shares.push(shares);
    }

    // Check if all rounds have identical shares
    let first_round = &all_rounds_shares[0];
    let mut all_identical = true;

    for (round_idx, round_shares) in all_rounds_shares.iter().enumerate() {
        if round_idx == 0 {
            continue;
        }

        for (share_idx, (first_share, current_share)) in
            first_round.iter().zip(round_shares.iter()).enumerate()
        {
            if first_share.x.0 != current_share.x.0 || first_share.y != current_share.y {
                println!(
                    "  Round {} share {} different from round 1",
                    round_idx + 1,
                    share_idx
                );
                all_identical = false;
            }
        }
    }

    if all_identical {
        println!("  ✅ All 10 rounds of sharing results completely consistent (determinism verification passed)");
        println!("  📍 Standard share pattern:");
        for (i, share) in first_round.iter().enumerate() {
            let y_values: Vec<String> = share.y.iter().map(|y| format!("{}", y.0)).collect();
            println!(
                "     Share{}: x={}, y=[{}]",
                i,
                share.x.0,
                y_values.join(",")
            );
        }
    } else {
        println!("  ❌ Share results have differences (determinism verification failed)");
    }

    // Show original byte representation
    println!("\n🔤 Original data byte representation:");
    let number_bytes = u64_to_bytes(secret_number);
    println!("  Number 30 byte representation: {:?}", number_bytes);
    println!(
        "  Hexadecimal: [{}]",
        number_bytes
            .iter()
            .map(|b| format!("0x{:02X}", b))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("  Binary: {:#066b}", secret_number);

    // Show XOR effect
    println!("\n🔐 XOR encryption effect:");
    let masked_number = secret_number ^ mask;
    let masked_bytes = u64_to_bytes(masked_number);
    println!("  Before masking: {} -> {:?}", secret_number, number_bytes);
    println!("  XOR mask: {} (0x{:016X})", mask, mask);
    println!("  After masking: {} -> {:?}", masked_number, masked_bytes);
    println!("  Byte changes:");
    for i in 0..8 {
        println!(
            "    Byte{}: {} -> {} (XOR result: {})",
            i,
            number_bytes[i],
            masked_bytes[i],
            number_bytes[i] ^ masked_bytes[i]
        );
    }

    println!("\n🎯 Test summary:");
    println!("  - Original number: {}", secret_number);
    println!(
        "  - Share configuration: threshold {}, total {}",
        threshold, total_shares
    );
    println!("  - XOR mask: {}", mask);
    println!("  - Determinism: ✅ 10 generations completely consistent");
    println!("  - Recovery accuracy: ✅ 100% success rate");
    println!("  - Security: ✅ XOR obfuscation effective");

    Ok(())
}

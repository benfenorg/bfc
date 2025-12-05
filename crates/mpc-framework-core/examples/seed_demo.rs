//! Seed management and test case generation example
//!
//! Usage:
//! 1. Use default seed: cargo run --example seed_demo
//! 2. Use custom seed: TEST_SEED=12345 cargo run --example seed_demo  
//! 3. Use random seed: USE_RANDOM_SEED=1 cargo run --example seed_demo

// Note: The structs in main need to be moved to lib.rs for proper import
// Using a simplified version for demonstration

use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;
use std::env;

// Copy necessary struct definitions (should be defined in lib.rs in actual project)
#[derive(Debug, Clone)]
pub struct SeedConfig {
    pub base_seed: u64,
    pub use_random_seed: bool,
    pub custom_seed: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌱 Seed Management Demo");
    println!();

    // Demonstrate seed management functionality
    let seed = if let Ok(seed_str) = env::var("TEST_SEED") {
        if let Ok(seed) = seed_str.parse::<u64>() {
            println!("🌱 Using seed specified by environment variable: {seed}");
            seed
        } else {
            42
        }
    } else if env::var("USE_RANDOM_SEED").is_ok() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        println!("🎲 Generated random seed: {seed}");
        seed
    } else {
        println!("🔧 Using default seed: 42");
        42
    };

    // Generate some example data using deterministic seed
    let mut rng = ChaCha20Rng::seed_from_u64(seed);

    println!("📊 Test case examples generated using seed {seed}:");
    println!();

    // Random addition test cases
    println!("🔢 Random addition test cases:");
    for i in 0..5 {
        let a = rng.next_u64() % 100000;
        let b = rng.next_u64() % 100000;
        println!("   {}: {} + {} = {}", i + 1, a, b, a + b);
    }

    println!();

    // Boundary value test cases
    println!("🎯 Boundary value test cases:");
    let boundary_values = [0u64, 1, 2, u8::MAX as u64, u16::MAX as u64, u32::MAX as u64];
    for (i, &val) in boundary_values.iter().take(3).enumerate() {
        println!("   {}: {}", i + 1, val);
    }

    println!();
    println!("✅ Seed demo completed!");
    println!();
    println!("💡 Usage:");
    println!("   - Default run: cargo run --example seed_demo");
    println!("   - Custom seed: TEST_SEED=12345 cargo run --example seed_demo");
    println!("   - Random seed: USE_RANDOM_SEED=1 cargo run --example seed_demo");
    println!("   - Same seed produces the same random sequence, facilitating test result reproduction");

    Ok(())
}

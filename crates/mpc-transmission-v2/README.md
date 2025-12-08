# MPC Framework Core



 **High-Performance Multi-Party Computation (MPC) Framework** - An enterprise-grade secure multi-party computation solution built with Rust

## Project Overview

MPC Framework Core is a production-ready multi-party computation framework that implements industry-leading cryptographic protocols and optimization algorithms. The framework enables multiple parties to perform collaborative computations without revealing private data, widely applicable in financial risk control, data analysis, and machine learning scenarios.

### Core Features

- **High Performance**: Built on Rust's zero-cost abstractions with batch operations and parallel computing support
- **Network MPC**: Full distributed multi-party computation support with real server connectivity
- **Security**: Implements core protocols including Shamir Secret Sharing, Beaver Triples, and Oblivious Transfer
- **Coordinate Compatibility**: Advanced secret sharing with automatic coordinate matching for reliable computations
- **Observability**: Complete performance monitoring and debugging tools
- **Ease of Use**: Rich example code and comprehensive API documentation
- **Batch Processing**: Support for large-scale batch secure computations

### Architecture Highlights

- **Modular Design**: Decoupled core components supporting flexible composition
- **Type Safety**: Leverages Rust's type system to ensure cryptographic operation correctness
- **Memory Safety**: Zero-copy design preventing sensitive data leakage
- **Protocol Compatibility**: Supports standard MPC protocols with interoperability

##  Quick Start



### Basic Usage Example

```rust
use mpc_framework_core::{
    secret::OptimizedSecretSharing,
    math::HomomorphicOperations,
    SecretSharing,
};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = ChaCha20Rng::from_entropy();
    
    // 1. Create optimized secret sharing instance
    let mut secret_sharing = OptimizedSecretSharing::new(3, 5, &mut rng)?; // 3-threshold, 5-shares
    
    // 2. Create compatible shares for secure computation
    let (shares_a, shares_b) = secret_sharing.create_compatible_shares(42, 58, &mut rng);
    
    // 3. Perform homomorphic addition
    let result_shares = HomomorphicOperations::add_shares(&shares_a, &shares_b)?;
    
    // 4. Reconstruct the result
    let result = SecretSharing::recover(&result_shares[..3])?;
    assert_eq!(result, 100); // 42 + 58 = 100
    
    println!("✅ MPC computation successful: {} + {} = {}", 42, 58, result);
    Ok(())
}
```

### Running Examples

```bash
# Basic secret sharing
cargo run --example basic_secret_sharing

# Secure multiplication protocol
cargo run --example beaver_multiplication

# Comprehensive performance test (includes network MPC)
cargo run --example comprehensive_performance_test --release

# Performance benchmark
cargo run --example performance_benchmark
```






### Testing

```bash
# Run all tests
cargo test

# Run benchmarks
cargo bench

# Run specific module tests
cargo test secret_sharing

# Performance analysis with local MPC
cargo run --example performance_benchmark --release

# Comprehensive network MPC performance test
cargo run --example comprehensive_performance_test --release

# Test network connectivity (requires remote servers)
cargo run --example two_server_connection
```



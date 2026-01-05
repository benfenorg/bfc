# Two-Party Secret Sharing API Usage Guide

## Overview

This module implements a (2,2)-threshold secret sharing scheme, supporting:
- Secret splitting and recovery
- Homomorphic addition and subtraction operations
- Secure multiplication based on Beaver triples

## Core Concepts

### Parameter Description

| Parameter | Type | Description |
|-----------|------|-------------|
| `value` | `u64` | Secret value to share |
| `user_id` | `u64` | User identifier for data interleaving encryption |
| `mask_secret` | `u64` | Mask secret key for XOR encryption and shuffle permutation |
| `coord_seed` | `u64` | Coordinate seed for generating share x-coordinates |

### Important Constraints

1. **Secret Value Range**: `value` must be less than the finite field modulus `18446744069414584321`
2. **Homomorphic Operation Requirements**: When performing homomorphic addition/subtraction, both secrets must use **the same `coord_seed`**
3. **Beaver Multiplication**: Multiplication operations require pre-generated Beaver triples

---

## API Details

### 1. Secret Splitting - `split_to_two_value_v2`

Splits a secret value into two encrypted hexadecimal shares.

```rust
pub fn split_to_two_value_v2(
    value: u64,        // Secret to split
    user_id: u64,      // User ID
    mask_secret: u64,  // Mask secret key
    coord_seed: u64,   // Coordinate seed
) -> (String, String, u64)
// Returns: (hex1, hex2, seed)
```

**Usage Example:**
```rust
let secret = 12345u64;
let user_id = 1u64;
let mask_secret = 0x1234567890ABCDEFu64;
let coord_seed = 116540450355;

let (hex1, hex2, seed) = split_to_two_value_v2(secret, user_id, mask_secret, coord_seed);
// hex1 and hex2 are encrypted shares (hexadecimal strings)
// seed equals the input coord_seed
```

---

### 2. Secret Recovery - `recover_value`

Recovers the original secret value from two encrypted shares.

```rust
pub fn recover_value(
    value1: String,    // First share (hex encoded)
    value2: String,    // Second share (hex encoded)
    mask_secret: u64,  // Mask secret key (must be same as during splitting)
) -> Result<u64, SSSError>
```

**Usage Example:**
```rust
let recovered = recover_value(hex1, hex2, mask_secret)?;
assert_eq!(recovered, secret);  // Recovered value equals original secret
```

---

### 3. Recover Share Objects - `recover_two_shares`

Recovers internal Share objects from hexadecimal strings (for subsequent operations).

```rust
pub fn recover_two_shares(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<Vec<Share>, SSSError>
// Returns: [Share0, Share1], each Share is an (x, y) coordinate pair
```

---

### 4. Homomorphic Addition - `add_two_shared_secrets_v2`

Performs addition on two secrets in encrypted state.

```rust
pub fn add_two_shared_secrets_v2(
    hex_a: String,      // One share of secret A
    hex_b: String,      // Corresponding share of secret B
    mask_secret: u64,   // Mask secret key
    index: u8,          // Share index (0 or 1)
    coord_seed_a: u64,  // Coordinate seed for A
    coord_seed_b: u64,  // Coordinate seed for B (must be same as A)
) -> Result<Vec<u8>, SSSError>
// Returns: Raw bytes of result share
```

**Complete Usage Example:**
```rust
let a = 100u64;
let b = 50u64;
let coord_seed = 116540450355;

// 1. Split both secrets using the same coord_seed
let (hex1_a, hex2_a, seed) = split_to_two_value_v2(a, user_id, mask_secret, coord_seed);
let (hex1_b, hex2_b, _) = split_to_two_value_v2(b, user_id, mask_secret, coord_seed);

// 2. Perform homomorphic addition for both positions separately
let result_bytes1 = add_two_shared_secrets_v2(
    hex1_a, hex1_b, mask_secret, 0, seed, seed
)?;
let result_bytes2 = add_two_shared_secrets_v2(
    hex2_a, hex2_b, mask_secret, 1, seed, seed
)?;

// 3. Convert bytes to Share and recover result
let share1 = bytes_to_share(&result_bytes1)?;
let share2 = bytes_to_share(&result_bytes2)?;
let result = recover_from_shares_internal(&[share1, share2])?;

assert_eq!(result, a + b);  // 150
```

---

### 5. Homomorphic Subtraction - `sub_two_shared_secrets_v2`

Performs subtraction on two secrets in encrypted state.

```rust
pub fn sub_two_shared_secrets_v2(
    hex_a: String,      // Share of minuend
    hex_b: String,      // Share of subtrahend
    mask_secret: u64,
    index: u8,
    coord_seed_a: u64,
    coord_seed_b: u64,
) -> Result<Vec<u8>, SSSError>
```

**Usage is the same as addition**, result is `A - B`.

---

### 6. Beaver Triple Multiplication

Multiplication operations are based on the Beaver triple protocol, divided into three steps:

#### Step Overview

```
Beaver triple: (a, b, c) where c = a × b

Steps to compute x × y:
1. Compute d = x - a, e = y - b (local)
2. Reveal d and e (requires communication)
3. Compute xy = c + d×b + e×a + d×e (local)
```

#### 6.1 Generate Beaver Triple

```rust
// Random generation
let triple = generate_beaver_triple()?;

// Or specify a, b values
let triple = generate_beaver_triple_with_values(a, b, mask_secret)?;
```

#### 6.2 Step 1 - Compute Masked Differences (Local)

**Version 1: Direct use of Share objects**

```rust
pub fn mul_step1_compute_masked_diff(
    share_val: &Share,    // Share of input value [x]_i or [y]_i
    beaver_share: &Share, // Share of Beaver triple [a]_i or [b]_i
) -> Share
// Returns: [d]_i = [x]_i - [a]_i or [e]_i = [y]_i - [b]_i
```

**Version 2: Use hex-encoded encrypted shares (auto-decrypt)**

```rust
pub fn mul_step1_compute_masked_diff_from_hex(
    hex_share_val: String,      // Hex-encoded share of input value
    hex_beaver_share: String,   // Hex-encoded share of Beaver triple
    mask_secret: u64,           // Mask secret key
    index: u8,                  // Share index (0 or 1)
    coord_seed_val: u64,        // Coordinate seed for input value
    coord_seed_beaver: u64,     // Coordinate seed for Beaver triple
) -> Result<Share, SSSError>
// Returns: [d]_i or [e]_i share
```

**Note**: If the input is a hex-encoded share (e.g., from `split_to_two_value_v2`), use the `from_hex` version to auto-decrypt.

#### 6.3 Step 2 - Reconstruct Public Values (Requires Communication)

```rust
pub fn mul_step2_reconstruct_masked_values(
    d_shares: &[Share],  // All parties' [d]_i
    e_shares: &[Share],  // All parties' [e]_i
) -> Result<(u64, u64), SSSError>
// Returns: (d, e) public values
```

#### 6.4 Step 3 - Compute Final Result (Local)

```rust
pub fn mul_step3_compute_result(
    beaver_a: &Share,  // [a]_i
    beaver_b: &Share,  // [b]_i
    beaver_c: &Share,  // [c]_i
    d_open: u64,       // Public d
    e_open: u64,       // Public e
) -> Vec<u8>
// Returns: Unencrypted share bytes of [xy]_i (16 bytes)
```

**Important**: Returns **unencrypted share bytes**, not the original value. To get the original value:
1. Both nodes get their respective share bytes
2. Convert to Share objects
3. Call `recover_from_shares_internal` to recover the original value

#### 6.5 Combined Steps 2 and 3 (Convenience Function)

If all shares are locally available, you can use the combined function:

```rust
pub fn mul_step2_and_3_combined(
    d_shares: &[Share],      // All parties' [d]_i shares
    e_shares: &[Share],      // All parties' [e]_i shares
    beaver_a: &Share,        // This party's [a]_i
    beaver_b: &Share,        // This party's [b]_i
    beaver_c: &Share,        // This party's [c]_i
) -> Result<Vec<u8>, SSSError>
// Returns: Unencrypted share bytes of [xy]_i
```

This function internally:
1. Reconstructs public values d and e (step 2)
2. Computes final result share (step 3)

#### Complete Multiplication Example

```rust
let x = 7u64;
let y = 11u64;

// 1. Generate Beaver triple
let triple = generate_beaver_triple()?;

// 2. Create shares for x and y (using Beaver triple coordinates)
let x_field: FieldElement = FieldElementTrait::from_u64(x);
let y_field: FieldElement = FieldElementTrait::from_u64(y);
let poly_x = Polynomial::new_with_fixed_seed(THRESHOLD - 1, x_field);
let poly_y = Polynomial::new_with_fixed_seed(THRESHOLD - 1, y_field);

let x_share_0 = (triple.a_shares[0].0, poly_x.evaluate(&triple.a_shares[0].0));
let x_share_1 = (triple.a_shares[1].0, poly_x.evaluate(&triple.a_shares[1].0));
let y_share_0 = (triple.b_shares[0].0, poly_y.evaluate(&triple.b_shares[0].0));
let y_share_1 = (triple.b_shares[1].0, poly_y.evaluate(&triple.b_shares[1].0));

// 3. Step 1: Compute masked differences
let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

// 4. Step 2: Reconstruct public values d and e
let (d_open, e_open) = mul_step2_reconstruct_masked_values(
    &[d_share_0, d_share_1],
    &[e_share_0, e_share_1],
)?;

// 5. Step 3: Compute final result
let result_bytes_0 = mul_step3_compute_result(
    &triple.a_shares[0], &triple.b_shares[0], &triple.c_shares[0],
    d_open, e_open,
);
let result_bytes_1 = mul_step3_compute_result(
    &triple.a_shares[1], &triple.b_shares[1], &triple.c_shares[1],
    d_open, e_open,
);

// 6. Recover final result (recover original value from shares)
let share1 = bytes_to_share(&result_bytes_0)?;
let share2 = bytes_to_share(&result_bytes_1)?;
let result = recover_from_shares_internal(&[share1, share2])?;

assert_eq!(result, x * y);  // 77
```

#### Simplified Example Using Combined Function

```rust
// Step 1: Compute masked differences (same as above)
let d_share_0 = mul_step1_compute_masked_diff(&x_share_0, &triple.a_shares[0]);
let d_share_1 = mul_step1_compute_masked_diff(&x_share_1, &triple.a_shares[1]);
let e_share_0 = mul_step1_compute_masked_diff(&y_share_0, &triple.b_shares[0]);
let e_share_1 = mul_step1_compute_masked_diff(&y_share_1, &triple.b_shares[1]);

// Steps 2 & 3: Combined execution (when all shares are local)
let result_bytes_0 = mul_step2_and_3_combined(
    &[d_share_0, d_share_1],
    &[e_share_0, e_share_1],
    &triple.a_shares[0], &triple.b_shares[0], &triple.c_shares[0],
)?;
let result_bytes_1 = mul_step2_and_3_combined(
    &[d_share_0, d_share_1],
    &[e_share_0, e_share_1],
    &triple.a_shares[1], &triple.b_shares[1], &triple.c_shares[1],
)?;

// Recover final result
let share1 = bytes_to_share(&result_bytes_0)?;
let share2 = bytes_to_share(&result_bytes_1)?;
let result = recover_from_shares_internal(&[share1, share2])?;
assert_eq!(result, x * y);
```

#### Result Format Description

**Important**: `mul_step3_compute_result` and `mul_step2_and_3_combined` return:
- ✅ **Unencrypted share bytes** (`Vec<u8>`, 16 bytes)
- ❌ **Not the original value** (requires two shares together to recover)
- ❌ **Not encrypted shares** (no user_id interleaving and shuffle/XOR)

To get the original value, you must:
1. Both nodes get their respective share bytes
2. Convert to Share objects: `bytes_to_share(&result_bytes)?`
3. Recover original value: `recover_from_shares_internal(&[share1, share2])?`

---

## Beaver Triple Operation Statistics

### Operations for Two Nodes

| Phase | Node 0 Operations | Node 1 Operations | Communication Count | Description |
|-------|------------------|------------------|---------------------|-------------|
| **Step 1** | 2 subtractions | 2 subtractions | 0 | Local computation |
| **Step 2** | 1 reconstruction | 1 reconstruction | 2 sends + 2 receives | Requires communication |
| **Step 3** | 1 composite computation | 1 composite computation | 0 | Local computation |
| **Total** | 4 computations | 4 computations | 4 messages | 4 operations per node |

### Detailed Operation Breakdown

**Step 1 (Local)**:
- Each node: 2 finite field subtractions
  - `[d]_i = [x]_i - [a]_i`
  - `[e]_i = [y]_i - [b]_i`

**Step 2 (Requires Communication)**:
- Each node: 2 sends + 2 receives + 2 Lagrange interpolations
  - Send `[d]_i` and `[e]_i` to the other party
  - Receive the other party's shares
  - Reconstruct public values `d` and `e`

**Step 3 (Local)**:
- Each node: 3 multiplications + 3 additions
  - `[xy]_i = [c]_i + d * [b]_i + e * [a]_i + d * e`

**Total Operations** (per node):
- Finite field operations: 8 (2 subtractions + 3 multiplications + 3 additions)
- Lagrange interpolations: 2
- Network operations: 4 (2 sends + 2 receives)
- **Total: 14 operations**

**Communication Statistics**:
- Message count: 4 messages (each node sends 2, receives 2)
- Communication rounds: 1 round (parallel send/receive)

---

## Helper Functions

### Byte Conversion

```rust
// Share to bytes
pub fn share_to_bytes(share: &Share) -> Vec<u8>

// Bytes to Share
pub fn bytes_to_share(data: &[u8]) -> Result<Share, SSSError>

// Recover secret from Share array
pub fn recover_from_shares_internal(shares: &[Share]) -> Result<u64, SSSError>
```

---

## Typical Usage Workflows

### Scenario 1: Simple Secret Sharing and Recovery

```rust
use mpc_framework_core::two_party_share::*;

// Configuration parameters
let user_id = 1u64;
let mask_secret = 0x1234567890ABCDEFu64;
let coord_seed = 116540450355;

// Split secret
let secret = 42u64;
let (hex1, hex2, _) = split_to_two_value_v2(secret, user_id, mask_secret, coord_seed);

// ... Distribute hex1 and hex2 to two parties ...

// Recover secret
let recovered = recover_value(hex1, hex2, mask_secret)?;
assert_eq!(recovered, secret);
```

### Scenario 2: Homomorphic Operations

```rust
// Both secrets use the same coord_seed
let a = 100u64;
let b = 50u64;

let (hex1_a, hex2_a, seed) = split_to_two_value_v2(a, user_id, mask_secret, coord_seed);
let (hex1_b, hex2_b, _) = split_to_two_value_v2(b, user_id, mask_secret, coord_seed);

// Homomorphic addition
let sum_bytes1 = add_two_shared_secrets_v2(hex1_a, hex1_b, mask_secret, 0, seed, seed)?;
let sum_bytes2 = add_two_shared_secrets_v2(hex2_a, hex2_b, mask_secret, 1, seed, seed)?;

// Recover result
let s1 = bytes_to_share(&sum_bytes1)?;
let s2 = bytes_to_share(&sum_bytes2)?;
let sum = recover_from_shares_internal(&[s1, s2])?;  // 150
```

---

## Security Considerations

1. **mask_secret must be kept secret**: Leakage would allow shares to be decrypted
2. **coord_seed consistency**: Secrets participating in homomorphic operations must use the same coord_seed
3. **Beaver triple single use**: Each triple can only be used for one multiplication operation
4. **Finite field range**: All operations are performed in finite field GF(18446744069414584321)
5. **Beaver multiplication result format**: Returns unencrypted share bytes, a single node cannot recover the original value, requires two shares together to recover

---

## Error Handling

```rust
pub enum SSSError {
    InvalidShareFormat(String),  // Share format error
    InvalidParameters(String),   // Invalid parameters (e.g., coord_seed mismatch)
    InsufficientShares(String),  // Insufficient number of shares
    // ...
}
```

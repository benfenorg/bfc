# Two-Party Share API Comparison Document

## mpc-framework-core vs mpc-transmission

This document compares the API implementation differences of the `two_party_share` module in both projects.

---

## Overview Comparison

| Feature | mpc-framework-core | mpc-transmission |
|---------|-------------------|------------------|
| **Finite Field** | GF(2^64 - 2^32 + 1) | GF(256) |
| **Secret Value Range** | `u64` (< modulus) | `u64` (full range) |
| **Share Size** | 16 bytes | 9 bytes |
| **Multiplication Implementation** | Beaver triple protocol | Direct GF256 operations |
| **Coordinate Generation** | Requires explicit `coord_seed` | Internally random generation |

---

## API Signature Comparison

### 1. Secret Splitting - `split_to_two_value_v2`

#### mpc-framework-core
```rust
pub fn split_to_two_value_v2(
    value: u64,
    user_id: u64,
    mask_secret: u64,
    coord_seed: u64,        // ⚠️ Additional parameter
) -> (String, String, u64)  // ⚠️ Returns seed
```

#### mpc-transmission
```rust
pub fn split_to_two_value_v2(
    value: u64,
    user_id: u64,
    mask_secret: u64,
) -> (String, String)       // Does not return seed
```

#### Differences
| Difference | mpc-framework-core | mpc-transmission |
|-----------|-------------------|------------------|
| `coord_seed` parameter | ✅ Requires explicit input | ❌ Internally auto-generated |
| Return value | `(hex1, hex2, seed)` | `(hex1, hex2)` |
| Homomorphic operation support | Requires same `coord_seed` | Auto-coordinated |

---

### 2. Secret Recovery - `recover_value`

#### mpc-framework-core
```rust
pub fn recover_value(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<u64, SSSError>
```

#### mpc-transmission
```rust
pub fn recover_value(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<u64, SecretSharingError>
```

#### Differences
✅ **Signatures are identical**, only error type names differ

---

### 3. Recover Share Objects - `recover_two_shares`

#### mpc-framework-core
```rust
pub fn recover_two_shares(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<Vec<Share>, SSSError>

// Share = (FieldElement, FieldElement)  // (x, y) coordinate pair
```

#### mpc-transmission
```rust
pub fn recover_two_shares(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<Vec<Share>, SecretSharingError>

// Share = { x: GF256, y: Vec<GF256> }  // GF256 structure
```

#### Differences
| Difference | mpc-framework-core | mpc-transmission |
|-----------|-------------------|------------------|
| Share type | `(FieldElement, FieldElement)` | `{ x: GF256, y: Vec<GF256> }` |
| Field element | 64-bit finite field | 8-bit GF256 |

---

### 4. Homomorphic Addition - `add_two_shared_secrets_v2`

#### mpc-framework-core
```rust
pub fn add_two_shared_secrets_v2(
    hex_a: String,          // Single share
    hex_b: String,          // Single share
    mask_secret: u64,
    index: u8,              // Share index (0 or 1)
    coord_seed_a: u64,      // Coordinate seed for A
    coord_seed_b: u64,      // Coordinate seed for B
) -> Result<Vec<u8>, SSSError>  // Returns raw bytes
```

#### mpc-transmission
```rust
pub fn add_two_shared_secrets_v2(
    shares1: Vec<Share>,    // Complete share array
    shares2: Vec<Share>,    // Complete share array
    mask_secret: u64,
) -> Result<u64, SecretSharingError>  // Directly returns result value
```

#### Differences
| Difference | mpc-framework-core | mpc-transmission |
|-----------|-------------------|------------------|
| Input format | Hex-encoded single share | Share object array |
| Output format | Raw bytes `Vec<u8>` | Directly returns `u64` result |
| Number of calls | Requires 2 calls (index=0,1) | Single call completes |
| Coordinate validation | Requires `coord_seed` matching | Internally auto-validated |
| Workflow | Share-level operations | Complete recovery then compute |

---

### 5. Homomorphic Subtraction - `sub_two_shared_secrets_v2`

#### mpc-framework-core
```rust
pub fn sub_two_shared_secrets_v2(
    hex_a: String,
    hex_b: String,
    mask_secret: u64,
    index: u8,
    coord_seed_a: u64,
    coord_seed_b: u64,
) -> Result<Vec<u8>, SSSError>
```

#### mpc-transmission
```rust
pub fn sub_two_shared_secrets_v2(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError>
```

#### Differences
Same difference pattern as addition

---

### 6. Multiplication - `mul_two_shared_secrets_v2`

#### mpc-framework-core (Beaver Triple Protocol)
```rust
// Step-by-step API
pub fn mul_step1_compute_masked_diff(
    share_val: &Share,
    beaver_share: &Share,
) -> Share

// Hex input version (auto-decrypt)
pub fn mul_step1_compute_masked_diff_from_hex(
    hex_share_val: String,
    hex_beaver_share: String,
    mask_secret: u64,
    index: u8,
    coord_seed_val: u64,
    coord_seed_beaver: u64,
) -> Result<Share, SSSError>

pub fn mul_step2_reconstruct_masked_values(
    d_shares: &[Share],
    e_shares: &[Share],
) -> Result<(u64, u64), SSSError>

pub fn mul_step3_compute_result(
    beaver_a: &Share,
    beaver_b: &Share,
    beaver_c: &Share,
    d_open: u64,
    e_open: u64,
) -> Vec<u8>  // Returns unencrypted share bytes

// Combined steps 2 and 3 (convenience function)
pub fn mul_step2_and_3_combined(
    d_shares: &[Share],
    e_shares: &[Share],
    beaver_a: &Share,
    beaver_b: &Share,
    beaver_c: &Share,
) -> Result<Vec<u8>, SSSError>

// Beaver triple generation
pub fn generate_beaver_triple(mask_secret: u64) -> Result<BeaverTriple, SSSError>
pub fn generate_beaver_triple_with_values(a: u64, b: u64, mask_secret: u64) -> Result<BeaverTriple, SSSError>
```

#### mpc-transmission (Direct Computation)
```rust
pub fn mul_two_shared_secrets_v2(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError>
```

#### Differences
| Difference | mpc-framework-core | mpc-transmission |
|-----------|-------------------|------------------|
| Protocol | Beaver triple (MPC secure) | Direct GF256 multiplication |
| Complexity | High (requires pre-computed triples) | Low (direct computation) |
| Security | Supports two-party without revealing inputs | Requires recovery of original values |
| API complexity | 3-step API + combined function | Single function call |
| Return value | Unencrypted share bytes (`Vec<u8>`) | Directly returns original value (`u64`) |
| Recover original value | Requires two shares together to recover | Directly returns result |

---

## Usage Comparison

### Secret Splitting and Recovery

#### mpc-framework-core
```rust
// Split - requires specifying coord_seed
let (hex1, hex2, seed) = split_to_two_value_v2(12345, user_id, mask_secret, coord_seed);

// Recover
let value = recover_value(hex1, hex2, mask_secret)?;
```

#### mpc-transmission
```rust
// Split - does not require coord_seed
let (hex1, hex2) = split_to_two_value_v2(12345, user_id, mask_secret);

// Recover
let value = recover_value(hex1, hex2, mask_secret)?;
```

---

### Homomorphic Addition

#### mpc-framework-core
```rust
// Use the same coord_seed
let (hex1_a, hex2_a, seed) = split_to_two_value_v2(100, user_id, mask_secret, coord_seed);
let (hex1_b, hex2_b, _) = split_to_two_value_v2(50, user_id, mask_secret, coord_seed);

// Process both share positions separately
let result_bytes1 = add_two_shared_secrets_v2(hex1_a, hex1_b, mask_secret, 0, seed, seed)?;
let result_bytes2 = add_two_shared_secrets_v2(hex2_a, hex2_b, mask_secret, 1, seed, seed)?;

// Convert and recover
let share1 = bytes_to_share(&result_bytes1)?;
let share2 = bytes_to_share(&result_bytes2)?;
let result = recover_from_shares_internal(&[share1, share2])?;  // 150
```

#### mpc-transmission
```rust
let (hex1_a, hex2_a) = split_to_two_value_v2(100, user_id, mask_secret);
let (hex1_b, hex2_b) = split_to_two_value_v2(50, user_id, mask_secret);

// Recover Share objects
let shares1 = recover_two_shares(hex1_a, hex2_a, mask_secret)?;
let shares2 = recover_two_shares(hex1_b, hex2_b, mask_secret)?;

// Single call completes addition
let result = add_two_shared_secrets_v2(shares1, shares2, mask_secret)?;  // 150
```

---

### Multiplication Operations

#### mpc-framework-core (Beaver Protocol)
```rust
// 1. Generate Beaver triple
let triple = generate_beaver_triple(mask_secret)?;

// 2. Create shares for x and y (using triple coordinates)
let x_shares = create_shares_with_triple_coords(x, &triple);
let y_shares = create_shares_with_triple_coords(y, &triple);

// 3. Step 1: Compute masked differences (local)
let d_shares = [
    mul_step1_compute_masked_diff(&x_shares[0], &triple.a_shares[0]),
    mul_step1_compute_masked_diff(&x_shares[1], &triple.a_shares[1]),
];
let e_shares = [
    mul_step1_compute_masked_diff(&y_shares[0], &triple.b_shares[0]),
    mul_step1_compute_masked_diff(&y_shares[1], &triple.b_shares[1]),
];

// 4. Step 2: Reveal d and e (requires communication)
let (d, e) = mul_step2_reconstruct_masked_values(&d_shares, &e_shares)?;

// 5. Step 3: Compute result (local)
let result_bytes_0 = mul_step3_compute_result(
    &triple.a_shares[0], &triple.b_shares[0], &triple.c_shares[0], d, e,
);
let result_bytes_1 = mul_step3_compute_result(
    &triple.a_shares[1], &triple.b_shares[1], &triple.c_shares[1], d, e,
);

// 6. Recover result (recover original value from shares)
let share1 = bytes_to_share(&result_bytes_0)?;
let share2 = bytes_to_share(&result_bytes_1)?;
let result = recover_from_shares_internal(&[share1, share2])?;  // x * y
```

**Or use combined function** (when all shares are local):

```rust
// Step 1: Compute masked differences (same as above)
let d_shares = [...];
let e_shares = [...];

// Steps 2 & 3: Combined execution
let result_bytes_0 = mul_step2_and_3_combined(
    &d_shares, &e_shares,
    &triple.a_shares[0], &triple.b_shares[0], &triple.c_shares[0],
)?;
let result_bytes_1 = mul_step2_and_3_combined(
    &d_shares, &e_shares,
    &triple.a_shares[1], &triple.b_shares[1], &triple.c_shares[1],
)?;

// Recover result
let share1 = bytes_to_share(&result_bytes_0)?;
let share2 = bytes_to_share(&result_bytes_1)?;
let result = recover_from_shares_internal(&[share1, share2])?;  // x * y
```

**Note**: `mul_step3_compute_result` and `mul_step2_and_3_combined` return **unencrypted share bytes**, not the original value. Two shares are required together to recover the original value.

#### mpc-transmission (Direct Computation)
```rust
let (hex1_a, hex2_a) = split_to_two_value_v2(x, user_id, mask_secret);
let (hex1_b, hex2_b) = split_to_two_value_v2(y, user_id, mask_secret);

let shares1 = recover_two_shares(hex1_a, hex2_a, mask_secret)?;
let shares2 = recover_two_shares(hex1_b, hex2_b, mask_secret)?;

let result = mul_two_shared_secrets_v2(shares1, shares2, mask_secret)?;  // x * y
```

---

## Internal Helper Function Comparison

| Function Name | mpc-framework-core | mpc-transmission |
|--------------|-------------------|------------------|
| `get_num_encoded` | ✅ Same | ✅ Same |
| `get_xor_mask` | ✅ Same | ✅ Same |
| `u64_to_bytes` | ✅ Same | ✅ Same |
| `generate_shuffle_permutations` | ✅ Same | ✅ Same |
| `shuffle_data` | ✅ Same | ✅ Same |
| `unshuffle_data` | ✅ Same | ✅ Same |
| `encode_share_data` | `encode_share_data_with_user_id` | `encode_share_data` |
| `decode_share_data` | `decode_share_data_with_user_id` | `decode_share_data` |

---

## Key Differences Summary

### 1. Finite Field Selection
- **mpc-framework-core**: Uses 64-bit prime field GF(2^64 - 2^32 + 1)
  - Advantages: Direct support for large integer operations
  - Disadvantages: Cannot handle values >= modulus
  
- **mpc-transmission**: Uses GF(256)
  - Advantages: Supports full `u64` range
  - Disadvantages: Requires splitting u64 into 8 GF256 elements

### 2. Coordinate Management
- **mpc-framework-core**: Explicit `coord_seed` parameter
  - Advantages: Clear control over coordinate consistency for homomorphic operations
  - Disadvantages: More complex API
  
- **mpc-transmission**: Internally auto-managed
  - Advantages: Concise API
  - Disadvantages: Lower flexibility for homomorphic operations

### 3. Multiplication Protocol
- **mpc-framework-core**: Beaver triple
  - Advantages: MPC secure, supports two-party multiplication without revealing inputs
  - Disadvantages: Requires pre-generated triples, complex API
  - API: Step-by-step API (steps 1/2/3) + combined function (steps 2&3)
  - Return value: Unencrypted share bytes, requires two shares together to recover original value
  
- **mpc-transmission**: Direct recovery then compute
  - Advantages: Simple and direct, directly returns original value
  - Disadvantages: Requires recovery of original values first, not true MPC multiplication

### 4. Return Value Format
- **mpc-framework-core**: Share-level returns `Vec<u8>`
  - Homomorphic addition/subtraction: Returns unencrypted share bytes, requires two shares together to recover original value
  - Beaver multiplication: Returns unencrypted share bytes, requires two shares together to recover original value
  - Suitable for distributed MPC scenarios, parties compute independently, single node cannot recover original value
  
- **mpc-transmission**: Directly returns `u64` result
  - All operations directly return original values
  - Suitable for single-point computation scenarios

---

## Interoperability

⚠️ **Shares from the two projects cannot directly interoperate**

Reasons:
1. Different finite fields (GF64 vs GF256)
2. Different share byte sizes (16 bytes vs 9 bytes)
3. Different polynomial coefficient generation methods

For interoperability, you must:
1. Completely recover the original secret value on one side
2. Re-split using the other side's API

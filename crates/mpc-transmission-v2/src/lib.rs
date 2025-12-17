// Module declarations
pub mod beaver;
pub mod beaver_cache;
pub mod encrypted_beaver;
pub mod encryption;
pub mod error;
pub mod field;
pub mod math;
pub mod poly;
pub mod secret;
pub mod two_party_helper;
pub mod two_party_share;

// Re-export main public APIs

use serde::{Deserialize, Serialize};
// Export from error module
pub use error::SSSError;


// Export from field module
pub use field::{gf64_sss, FieldElement};

// Export from math module
pub use math::HomomorphicOperations;

// Export from poly module
pub use poly::Polynomial;

// Export from secret module
pub use secret::SecretSharing;

// Export from encryption module
pub use encryption::{EncryptedShare, EncryptionConfig, MaskEntry, XorEncryptionManager};

// Export from beaver module
pub use beaver::{BeaverMultiplication, BeaverTriple, BeaverTripleDistributor};

// Export from beaver_cache module
pub use beaver_cache::{BeaverTripleCache, CacheStats, CachedTriple};

// Export from encrypted_beaver module
pub use encrypted_beaver::EncryptedBeaverProcessor;
use log::info;
// Export from two_party_share module - Two-party secret sharing operations
pub use two_party_share::{
    add_two_shared_secrets_v2,
    // Beaver triple generation
    generate_beaver_triple,
    generate_beaver_triple_with_values,
    // Beaver multiplication - step by step API
    mul_step1_compute_masked_diff,
    mul_step1_compute_masked_diff_from_hex,
    mul_step2_and_3_combined,
    mul_step2_reconstruct_masked_values,
    mul_step3_compute_result,
    // High-level multiplication API
    mul_two_shared_secrets_v2,
    recover_two_shares_v2,
    recover_value_v2,
    split_to_two_value_v2,
    sub_two_shared_secrets_v2,
    split_to_two_bytes_value_v2,
};

// Share converter functions - conversion between mpc-transmission and mpc-transmission-v2 formats

/// mpc-transmission-v2 finite field modulus
pub const FIELD_MODULUS: u64 = 18446744069414584321;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConvertedData {
   pub value1: String,
   pub value2: String,
   pub coord_seed: u64,
}


/// Recover the original secret value from mpc-transmission format shares
///
/// # Arguments
/// * `transmission_hex1` - First mpc-transmission format hex share
/// * `transmission_hex2` - Second mpc-transmission format hex share
/// * `mask_secret` - Mask secret key
///
/// # Returns
/// * `Ok(u64)` - Recovered original secret value
/// * `Err(SSSError)` - If decoding or recovery fails
fn recover_from_transmission_shares(
    transmission_hex1: &str,
    transmission_hex2: &str,
    mask_secret: u64,
) -> Result<u64, SSSError> {
    Ok(mpc_transmission::two_party_share::recover_value(
        transmission_hex1.to_string(),
        transmission_hex2.to_string(),
        mask_secret,
    )?)
}

/// Check if the given shares are in valid mpc-transmission format
///
/// This function validates whether two hex-encoded shares and a mask secret can be used
/// to successfully recover a secret value using the mpc-transmission format.
///
/// # Arguments
/// * `transmission_hex1` - First mpc-transmission format hex share
/// * `transmission_hex2` - Second mpc-transmission format hex share
/// * `mask_secret` - Mask secret key used for encoding
///
/// # Returns
/// * `true` - If the shares are valid and can successfully recover a value
/// * `false` - If the shares are invalid, malformed, or cannot recover a value
pub fn is_v1_transmission_shares_format(
    transmission_hex1: &str,
    transmission_hex2: &str,
    mask_secret: u64,
) -> bool {
    if recover_from_transmission_shares(transmission_hex1, transmission_hex2, mask_secret).is_err()
    {
        false
    } else {
        true
    }
}

pub fn process_shares_data_convert(transmission_hex1: &str, transmission_hex2: &str, mask_secret: u64, coord_seed :u64, user_id: u64) -> Result<ConvertedData, SSSError> {
    let (value1, value2, coord_seed_a) =  if is_v1_transmission_shares_format(transmission_hex1, transmission_hex2, mask_secret){
        match convert_from_v1_transmission_shares(transmission_hex1, transmission_hex2, mask_secret, user_id, coord_seed, 1) {
            Ok((core_hex1, core_hex2, coord_seed_a)) => (core_hex1, core_hex2, coord_seed_a),
            Err(e) => {
                info!("Failed to convert transmission shares to core shares: {}", e);
                Err(e)
            }?
        }
    } else {
        (transmission_hex1.to_string(), transmission_hex2.to_string(), coord_seed)
    };

    Ok(ConvertedData {
        value1,
        value2,
        coord_seed: coord_seed_a,
    })
}

/// Convert shares from mpc-transmission format to mpc-transmission-v2 format
///
/// # Arguments
/// * `transmission_hex1` - First mpc-transmission format hex share
/// * `transmission_hex2` - Second mpc-transmission format hex share
/// * `mask_secret` - Mask secret key
/// * `user_id` - User ID (for mpc-transmission-v2 encoding)
/// * `coord_seed` - Coordinate seed (for mpc-transmission-v2)
/// * `version` - Version number, only version 1 is supported
///
/// # Returns
/// * `Ok((String, String, u64))` - mpc-transmission-v2 format (hex1, hex2, seed)
/// * `Err(SSSError)` - If conversion fails
///
/// # Errors
/// - Returns error if version number is not 1
/// - Returns error if mpc-transmission value >= FIELD_MODULUS
///
/// # Important Notes
/// - The conversion process requires complete recovery of the original secret value, which temporarily exposes the secret
/// - The finite field modulus of mpc-transmission-v2 is 18446744069414584321
/// - If the mpc-transmission value >= modulus, conversion will fail
pub fn convert_from_v1_transmission_shares(
    transmission_hex1: &str,
    transmission_hex2: &str,
    mask_secret: u64,
    user_id: u64,
    coord_seed: u64,
    version: u8,
) -> Result<(String, String, u64), SSSError> {
    // Step 0: Check version number, only convert version 1
    if version != 1 {
        return Err(SSSError::InvalidParameters(format!(
            "Unsupported version: {}. Only version 1 is supported for conversion.",
            version
        )));
    }

    // Step 1: Recover original value from mpc-transmission format
    let value =
        recover_from_transmission_shares(transmission_hex1, transmission_hex2, mask_secret)?;

    // Step 2: Check if value is within mpc-transmission-v2 finite field range
    if value >= FIELD_MODULUS {
        return Err(SSSError::InvalidParameters(format!(
            "Value {} exceeds field modulus {}. Cannot convert to mpc-transmission-v2 format.",
            value, FIELD_MODULUS
        )));
    }

    // Step 3: Re-split using mpc-transmission-v2
    let (hex1, hex2, seed) = split_to_two_value_v2(value, user_id, mask_secret, coord_seed);

    Ok((hex1, hex2, seed))
}

/// Recover a value from two shares, automatically detecting the format (transmission or core)
/// Returns Ok(u64) if successful, Err(String) with error message if failed
pub fn recover_value_from_shares_v2(value1: String, value2: String, mask_secret: u64) -> Result<u64, String> {
    if is_v1_transmission_shares_format(&value1, &value2, mask_secret) {
        mpc_transmission::two_party_share::recover_value(value1, value2, mask_secret)
            .map_err(|e| {
                e.to_string()
            })
    } else {
        recover_value_v2(value1, value2, mask_secret)
            .map_err(|e| {
                e.to_string()
            })
    }
}

/// Recover secret value from Share objects
///
/// This is a public wrapper around `recover_from_shares_internal` that provides
/// a convenient interface for recovering secrets from Share objects.
///
/// # Arguments
/// * `shares` - A slice of Share objects (at least one share is required)
///
/// # Returns
/// * `Ok(u64)` - Recovered secret value
/// * `Err(SSSError)` - If recovery fails (e.g., insufficient shares)
pub fn recover_from_shares(shares: &[types::Share]) -> Result<u64, SSSError> {
    use crate::two_party_helper::recover_from_shares_internal;
    recover_from_shares_internal(shares)
}

/// Generate multiple secret shares from a u64 secret
///
/// Similar to `split_to_two_value_internal`, but supports generating any number of shares
/// with a configurable threshold. Uses deterministic coordinate generation based on coord_seed
/// and fixed-seed polynomial generation for compatibility.
///
/// # Arguments
/// * `secret` - The secret value to be shared (u64)
/// * `threshold` - Minimum number of shares required to recover the secret (must be ≥ 2)
/// * `total_shares` - Total number of shares to generate (must be ≥ threshold)
/// * `coord_seed` - Seed for deterministic coordinate generation
///
/// # Returns
/// * `Ok(Vec<Share>)` - Vector of Share objects (x-coordinate, y-value pairs)
/// * `Err(SSSError)` - If parameters are invalid
///
/// # Errors
/// - Returns error if threshold < 2
/// - Returns error if total_shares < threshold
pub fn generate_shares_u64(
    secret: u64,
    threshold: usize,
    total_shares: usize,
    coord_seed: u64,
) -> Result<Vec<types::Share>, SSSError> {
    // Parameter validation
    if threshold < 2 {
        return Err(SSSError::InvalidParameters(format!(
            "Threshold must be at least 2, got {}",
            threshold
        )));
    }
    if total_shares < threshold {
        return Err(SSSError::InvalidParameters(format!(
            "Total shares ({}) must be at least threshold ({})",
            total_shares, threshold
        )));
    }

    // Generate deterministic coordinates based on coord_seed
    use crate::field::FieldElement as FieldElementTrait;
    use crate::poly::Polynomial;
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use rand_core::RngCore;

    let mut rng = ChaCha20Rng::seed_from_u64(coord_seed);
    let coords: Vec<crate::field::gf64_sss::FieldElement> = (0..total_shares)
        .map(|_| FieldElementTrait::from_u64(rng.next_u64()))
        .collect();

    // Create polynomial with fixed seed (same as split_to_two_value_internal)
    let secret_fe = FieldElementTrait::from_u64(secret);
    let poly = Polynomial::new_with_fixed_seed(threshold - 1, secret_fe);

    // Generate shares by evaluating polynomial at each coordinate
    let shares: Vec<types::Share> = coords
        .iter()
        .map(|x| (*x, poly.evaluate(x)))
        .collect();

    Ok(shares)
}

// Type aliases - simplify complex type definitions
pub mod types {
    use crate::field::gf64_sss::FieldElement;

    /// Secret share pair: (x coordinate, y value)
    pub type Share = (FieldElement, FieldElement);

    /// Share list
    pub type ShareList = Vec<Share>;

    /// Share pair: two related share lists
    pub type SharePair = (ShareList, ShareList);

    /// Function return double share result
    pub type SharePairResult = (ShareList, ShareList);

    /// Encryption-aware double share result type
    pub type EncryptedAwareSharePairResult = Result<SharePairResult, crate::SSSError>;
}

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
pub mod share_converter;
pub mod two_party_helper;
pub mod two_party_share;

// Re-export main public APIs

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

// Export from two_party_share module - Two-party secret sharing operations
pub use two_party_share::{
    split_to_two_value,
    recover_value,
    recover_two_shares,
    add_two_shared_secrets,
    sub_two_shared_secrets,
    // Beaver multiplication - step by step API
    mul_step1_compute_masked_diff,
    mul_step1_compute_masked_diff_from_hex,
    mul_step2_reconstruct_masked_values,
    mul_step3_compute_result,
    mul_step2_and_3_combined,
    // Beaver triple generation
    generate_beaver_triple,
    generate_beaver_triple_with_values,
};

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

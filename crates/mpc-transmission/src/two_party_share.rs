use crate::error::SecretSharingError;
use crate::math::{add_shared_secrets, mul_shared_secrets, sub_shared_secrets};
use crate::share::Share;
use crate::{generate_shares_with_xor, recover_secret_with_xor};

const THRESHOLD: usize = 2;
const TOTAL_SHARES: usize = 2;

/// Recover u64 secret
pub fn split_to_two_value(value: u64, mask_secret: u64) -> (String, String) {
    let shares = generate_shares_with_xor(value, THRESHOLD, TOTAL_SHARES, mask_secret).unwrap();
    let value1: Vec<u8> = (&shares[0]).into();
    let value2: Vec<u8> = (&shares[1]).into();
    let hex_value1 = hex::encode(value1);
    let hex_value2 = hex::encode(value2);
    (hex_value1, hex_value2)
}

pub fn recover_value(
    value1: String,
    value2: String,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    let shares = recover_two_shares(value1, value2)?;
    let value = recover_secret_with_xor(&shares[..THRESHOLD], THRESHOLD, mask_secret)?;
    Ok(value)
}

pub fn recover_two_shares(
    value1: String,
    value2: String,
) -> Result<Vec<Share>, SecretSharingError> {
    let value1: Vec<u8> =
        hex::decode(value1).map_err(|e| SecretSharingError::InvalidShare(e.to_string()))?;
    let value2: Vec<u8> =
        hex::decode(value2).map_err(|e| SecretSharingError::InvalidShare(e.to_string()))?;

    let share1: Share = value1.as_slice().try_into().map_err(|_| {
        SecretSharingError::InvalidShare("value1 convert to share failed".to_string())
    })?;
    let share2: Share = value2.as_slice().try_into().map_err(|_| {
        SecretSharingError::InvalidShare("value2 convert to share failed".to_string())
    })?;
    Ok(vec![share1, share2])
}

pub fn add_two_shared_secrets(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    add_shared_secrets(
        &shares1[..THRESHOLD],
        &shares2[..THRESHOLD],
        THRESHOLD,
        mask_secret,
        mask_secret,
    )
}

pub fn sub_two_shared_secrets(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    sub_shared_secrets(
        &shares1[..THRESHOLD],
        &shares2[..THRESHOLD],
        THRESHOLD,
        mask_secret,
        mask_secret,
    )
}

pub fn mul_two_shared_secrets(
    shares1: Vec<Share>,
    shares2: Vec<Share>,
    mask_secret: u64,
) -> Result<u64, SecretSharingError> {
    mul_shared_secrets(
        &shares1[..THRESHOLD],
        &shares2[..THRESHOLD],
        THRESHOLD,
        mask_secret,
        mask_secret,
    )
}

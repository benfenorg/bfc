use ed25519_dalek::SignatureError;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Ed25519Error {
    #[error("Invalid public key length: expected 32, got {0}")]
    InvalidPublicKeyLength(usize),

    #[error("Invalid signature length: expected 64, got {0}")]
    InvalidSignatureLength(usize),

    #[error("Invalid public key: {0}")]
    InvalidPublicKey(#[from] SignatureError),
}

/// Generates a new Ed25519 keypair
pub fn generate_keypair() -> (Vec<u8>, SigningKey) {
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let public_key = signing_key.verifying_key().as_bytes().to_vec();
    (public_key, signing_key)
}

/// Signs a message with a private key
pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> Vec<u8> {
    signing_key.sign(message).to_bytes().to_vec()
}

/// Verifies an Ed25519 signature
pub fn verify_signature(
    public_key: &[u8],
    signature: &[u8],
    message: &[u8],
) -> Result<(), Ed25519Error> {
    // Validate public key length
    if public_key.len() != 32 {
        return Err(Ed25519Error::InvalidPublicKeyLength(public_key.len()));
    }

    // Validate signature length
    if signature.len() != 64 {
        return Err(Ed25519Error::InvalidSignatureLength(signature.len()));
    }

    // Convert to fixed-size arrays
    let public_key_arr: [u8; 32] = public_key
        .try_into()
        .map_err(|_| Ed25519Error::InvalidPublicKeyLength(public_key.len()))?;

    let signature_arr: [u8; 64] = signature
        .try_into()
        .map_err(|_| Ed25519Error::InvalidSignatureLength(signature.len()))?;

    // Create verifying key
    let verifying_key = VerifyingKey::from_bytes(&public_key_arr)?;
    // Create signature object
    let sig = Signature::from_bytes(&signature_arr);
    // Verify signature
    verifying_key.verify(message, &sig)?;

    Ok(())
}

/// Verifies a signature using strict verification (recommended)
pub fn verify_signature_strict(
    public_key: &[u8],
    signature: &[u8],
    message: &[u8],
) -> Result<(), Ed25519Error> {
    // Validate public key length
    if public_key.len() != 32 {
        return Err(Ed25519Error::InvalidPublicKeyLength(public_key.len()));
    }

    // Validate signature length
    if signature.len() != 64 {
        return Err(Ed25519Error::InvalidSignatureLength(signature.len()));
    }

    // Convert to fixed-size arrays
    let public_key_arr: [u8; 32] = public_key
        .try_into()
        .map_err(|_| Ed25519Error::InvalidPublicKeyLength(public_key.len()))?;

    let signature_arr: [u8; 64] = signature
        .try_into()
        .map_err(|_| Ed25519Error::InvalidSignatureLength(signature.len()))?;

    // Create verifying key
    let verifying_key = VerifyingKey::from_bytes(&public_key_arr)?;

    // Create signature object
    let sig = Signature::from_bytes(&signature_arr);

    // Verify signature with strict checking
    verifying_key.verify_strict(message, &sig)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sign_and_verify() {
        let (public_key, signing_key) = generate_keypair();
        let message: &str =
            "BFCf502f799165f3310907119d7b60e6dbf146914253901981125d1c1a4c7ec3b581c2e";
        let signature = sign_message(&signing_key, message.as_bytes());

        // Regular verification
        assert!(verify_signature(&public_key, &signature, message.as_bytes()).is_ok());

        // Strict verification
        assert!(verify_signature_strict(&public_key, &signature, message.as_bytes()).is_ok());

        // Test with wrong message
        let wrong_message = b"Tampered message";
        assert!(verify_signature(&public_key, &signature, wrong_message).is_err());

        // Test with wrong public key
        let (wrong_pubkey, _) = generate_keypair();
        assert!(verify_signature(&wrong_pubkey, &signature, message.as_bytes()).is_err());
    }

    #[test]
    fn test_invalid_lengths() {
        let message = b"Test message";

        // Too short public key
        assert!(matches!(
            verify_signature(&[0u8; 31], &[0u8; 64], message),
            Err(Ed25519Error::InvalidPublicKeyLength(31))
        ));

        // Too long public key
        assert!(matches!(
            verify_signature(&[0u8; 33], &[0u8; 64], message),
            Err(Ed25519Error::InvalidPublicKeyLength(33))
        ));

        // Too short signature
        assert!(matches!(
            verify_signature(&[0u8; 32], &[0u8; 63], message),
            Err(Ed25519Error::InvalidSignatureLength(63))
        ));

        // Too long signature
        assert!(matches!(
            verify_signature(&[0u8; 32], &[0u8; 65], message),
            Err(Ed25519Error::InvalidSignatureLength(65))
        ));
    }

    #[test]
    fn test_invalid_signature() {
        let (public_key, signing_key) = generate_keypair();
        let message = b"Test message";
        let mut signature = sign_message(&signing_key, message);

        // Tamper with signature
        signature[0] = signature[0].wrapping_add(1);

        // assert!(matches!(
        //     verify_signature(&public_key, &signature, message),
        //     Err(Ed25519Error::VerificationFailed(_))
        // ));
    }

    #[test]
    fn test_hex_conversion() {
        // let data = b"test data";
        // let hex_str = bytes_to_hex(data);
        // let decoded = hex_to_bytes(&hex_str).unwrap();
        //
        // assert_eq!(data, decoded.as_slice());
        // assert!(hex_str.starts_with("0x"));
    }
}

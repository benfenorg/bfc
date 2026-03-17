use anchor_lang::prelude::*;
use crate::errors::BridgeError;

pub const PROCESSED_TRANSFER_SEED: &str= "processed_transfer";

/// Tracks processed transfer messages to prevent replay attacks
#[account]
#[derive(Debug,Default)]
pub struct ProcessTransfer {
    /// Bump seed for PDA
    pub bump: [u8; 1],
    /// The nonce of the processed transfer
    pub nonce: u64,
    /// Whether this transfer has been processed
    pub is_processed: bool,

    //padding upgrade
    pub padding: [u64; 8],
}




impl ProcessTransfer {
    pub const SPACE: usize = 8 + 1 + 8 + 1 + (8 * 8);


    pub fn new(nonce: u64, bump: u8) -> Self {
        Self {
            nonce,
            is_processed: false,
            bump: [bump],
            padding: [0; 8],
        }
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn is_processed(&self) -> bool {
        self.is_processed
    }

    // pub fn seeds(&self) -> [&[u8]; 3] {
    //     [
    //         PROCESSED_TRANSFER_SEED.as_bytes(),
    //         self.nonce.to_be_bytes().as_ref(),
    //         self.bump.as_ref(),
    //     ]
    // }


    pub fn mark_processed(&mut self) -> Result<()> {
        require!(!self.is_processed, BridgeError::TransferAlreadyProcessed);
        self.is_processed = true;
        Ok(())
    }
}

#[cfg(test)]
pub mod process_transfer_tests{
    use super::*;
   

   #[test]
   fn test_mark_processed(){
    let mut process_transfer = ProcessTransfer::new(1, 1);
     assert!(!process_transfer.is_processed);
    process_transfer.mark_processed().unwrap();
    assert!(process_transfer.is_processed);
   }

   #[test]
   fn test_mark_processed_twice(){
    let mut process_transfer = ProcessTransfer::new(1, 1);
    process_transfer.mark_processed().unwrap();
    assert!(process_transfer.is_processed);
    let result = process_transfer.mark_processed();
    assert!(result.is_err());
   }

   #[test]
   fn test_mark_processed_with_different_nonce(){
    let mut process_transfer = ProcessTransfer::new(1, 1);
    process_transfer.mark_processed().unwrap();
    assert!(process_transfer.is_processed);
    let mut process_transfer = ProcessTransfer::new(2, 1);
    let result = process_transfer.mark_processed();
    assert!(result.is_ok());
    assert!(process_transfer.is_processed);
   }

}

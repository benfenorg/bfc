use anchor_lang::prelude::*;
use crate::errors::BridgeError;

/// Constants
pub const SUI_ADDRESS_LENGTH: usize = 32;

pub const BENFEN_BRIDGE_SEED: &str = "benfen_bridge";
pub const VAULT_SEED: &str = "vault";


/// BenfenBridge equivalent in Anchor framework
/// This contract implements a token bridge that enables users to deposit and withdraw
/// supported tokens to and from other chains. The bridge supports the transfer of SOL and SPL
/// tokens. Bridge operations are managed by a committee of validators that are responsible
/// for verifying and processing bridge messages.
#[account]
#[derive(Debug,Default)]
pub struct BenfenBridge {
    /// Bump seed for PDA
    pub bump: [u8;1],
    /// Whether the bridge is currently paused
    pub is_paused: bool,
    /// Committee contract address for signature verification
    pub committee: Pubkey,
    /// Bridge config contract address
    pub config: Pubkey,
    /// Padding for future upgrades
    pub padding: [u64; 32],
}


impl BenfenBridge {
    pub const SPACE: usize = 8 + 1 + 1 + 32 + 32 + (32 * 8);


    /// Initialize the bridge with required parameters
    pub fn initialize(
        &mut self,
        committee: Pubkey,
        config: Pubkey,
        bump: u8,
    ) -> Result<()> {
        self.is_paused = false;
        self.committee = committee;
        self.config=config;
        self.bump = [bump];
        self.padding = [0; 32];
        Ok(())
    }

    /// Check if the bridge is paused
    pub fn require_not_paused(&self) -> Result<()> {
        require!(!self.is_paused, BridgeError::BridgePaused);
        Ok(())
    }

    /// Pause the bridge (only authority)
    pub fn pause(&mut self) -> Result<()> {
        self.is_paused = true;
        Ok(())
    }

    /// Unpause the bridge (only authority)
    pub fn unpause(&mut self) -> Result<()> {
        self.is_paused = false;
        Ok(())
    } 

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            &BENFEN_BRIDGE_SEED.as_bytes(),
            self.committee.as_ref(),
            self.bump.as_ref(),
        ]
    }
}


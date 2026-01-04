
use anchor_lang::prelude::*;
use crate::errors::BridgeUpgradeError;

pub const UPGRADE_AUTHORITY_SEED: &str = "upgrade_authority";
pub const UPGRADE_COOLDOWN_PERIOD: i64 = 24 * 60 * 60; // 24小时（以秒为单位）

#[account]
pub struct UpgradeAuthority {
    pub enabled: bool,
    pub bump: [u8;1],
    pub current_version: u8,
    pub last_upgrade_timestamp: i64,
    pub committee: Pubkey,
}

impl UpgradeAuthority {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn initialize(&mut self, enabled: bool, bump: [u8;1], committee: Pubkey) {
        self.enabled = enabled;
        self.bump = bump;
        self.committee = committee;
        self.current_version = 1;
        self.last_upgrade_timestamp = 0;
    }
    
    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            &UPGRADE_AUTHORITY_SEED.as_bytes(),
            self.committee.as_ref(),
            self.bump.as_ref(),
        ]
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn can_upgrade(&self, clock: &Sysvar<Clock>) -> Result<()> {
        require!(
            clock.unix_timestamp - self.last_upgrade_timestamp >= UPGRADE_COOLDOWN_PERIOD,
            BridgeUpgradeError::UpgradeCooldownNotElapsed
        );
        Ok(())
    }

    pub fn update_version(&mut self, new_version: u8, clock: &Sysvar<Clock>) {
        self.current_version = new_version;
        self.last_upgrade_timestamp = clock.unix_timestamp;
    }
}

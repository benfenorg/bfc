use anchor_lang::prelude::*;



pub const MESSAGE_CONFIG_SEED: &str = "message_config";
use crate::errors::MessageError;


#[account]
#[derive(Default,Debug)]
pub struct MessageConfig {
    pub verifier: Pubkey, 
    pub message_type: u8,      
    pub nonce: u64,           
    //padding upgrade
    pub padding: [u8; 32],
}


impl MessageConfig {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn initialize(
        &mut self,
        verifier: Pubkey,
        message_type: u8,
    ) -> Result<()> {
        require!(verifier != Pubkey::default(), MessageError::InvalidMessageVerifier);
        self.verifier = verifier;
        self.message_type = message_type;
        self.nonce = 0;
        Ok(())
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn message_type(&self) -> u8 {
        self.message_type
    }

    pub fn increment_nonce(&mut self) {
        self.nonce = self.nonce.checked_add(1).unwrap();
    }
}



#[cfg(test)]
pub mod test_message_config{
    use super::*;

    pub fn build_message_config(verifier: Pubkey, message_type: u8) -> MessageConfig {
        let mut config = MessageConfig::default();
        config.initialize(verifier, message_type).unwrap();
        config
    }
    #[test]
    fn test_message_config(){
        let mut config = MessageConfig::default();
        config.initialize(Pubkey::new_unique(), 1).unwrap();
        assert_eq!(config.message_type(), 1);
        assert_eq!(config.nonce(), 0);
    }

    #[test]
    fn test_increment_nonce(){
        let mut config = MessageConfig::default();
        config.initialize(Pubkey::new_unique(), 1).unwrap();
        assert_eq!(config.nonce(), 0);
        config.increment_nonce();
        assert_eq!(config.nonce(), 1);
    }

   
}
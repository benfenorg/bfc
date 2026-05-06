use anchor_lang::prelude::*;
use crate::errors::MessageError;
pub const MESSAGE_VERIFIER_SEED: &str = "message_verifier";


#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Default)]
pub struct MessageVerifier {
    /// Bump seed for PDA
    pub bump: [u8;1],

    pub message_number: u64,    //有多少message 创建
    pub committee: Pubkey,
}



impl MessageVerifier {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();


    pub fn initialize(
        &mut self,
        bump: u8,
        committee: Pubkey,
    ) -> Result<()> {
        self.bump = [bump];
        self.committee = committee;
        self.message_number = 0;
        Ok(())
    }

    pub fn increment_message_number(&mut self) -> Result<()> {
        self.message_number = self
            .message_number
            .checked_add(1)
            .ok_or(MessageError::MessageNumberOverflow)?;
        Ok(())
    }

    pub fn message_number(&self) -> u64 {
        self.message_number
    }

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            MESSAGE_VERIFIER_SEED.as_bytes(),
            self.committee.as_ref(),
            self.bump.as_ref(),
        ]
    }

    pub fn key(&self) -> Pubkey {
        Pubkey::create_program_address(
            &self.seeds(),
            &crate::ID,
        ).unwrap()
    }
}


#[cfg(test)]
pub mod test_message_verifier{
    use super::*;
    use std::cell::RefCell;
    use crate::states::committee::committee_test::build_default_committee_with_config;


    pub fn build_message_verifier()->RefCell<MessageVerifier>{
        let mut message_verifier=MessageVerifier::default();
        let committee = build_default_committee_with_config().0.borrow().key();
        let bump = 255;
        message_verifier.initialize(bump, committee).unwrap();
        RefCell::new(message_verifier)
    }

    #[test]
    fn test_initialize(){
        let mut verifier = MessageVerifier::default();
        let committee = Pubkey::new_unique();
        let bump = 255;
        verifier.initialize(bump, committee).unwrap();
        assert_eq!(verifier.committee, committee);
    }

    #[test]
    fn test_increment_message_number(){
        let mut verifier = MessageVerifier::default();
        let committee = Pubkey::new_unique();
        let bump = 255;
        verifier.initialize(bump, committee).unwrap();
        assert_eq!(verifier.message_number(), 0);
        verifier.increment_message_number().unwrap();
        assert_eq!(verifier.message_number(), 1);
    }


}


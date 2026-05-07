use std::cell::RefMut;

use anchor_lang::prelude::*;
use crate::states::message_verifier::MessageVerifier;
use crate::states::message_config::MessageConfig;

use crate::states::message::*;
use crate::states::committee::Committee;
use crate::states::bridge_config::BridgeConfig;
use crate::errors::MessageError;

pub fn verify_bridge_signature<'info>(
    message_config:  &mut  MessageConfig,
    bridge_config:  &mut RefMut<'info, BridgeConfig>,
    verifier: &mut  RefMut<'info,MessageVerifier>,
    committee:  &mut RefMut<'info, Committee>,
    message: &Message,
    signatures: Vec<Vec<u8>>,
) -> Result<()> {
    if message_config.verifier==Pubkey::default() {
       message_config.initialize(verifier.key(), message.message_type)?;
       verifier.increment_message_number()?;
    }
    require!(message.message_type == message_config.message_type, MessageError::InvalidMessageType);
    committee.verify_signatures(signatures, &message)?;
    if message.message_type != TOKEN_TRANSFER {
        require!(bridge_config.chain_id==message.chain_id, MessageError::InvalidMessageChainId);
        require!(message_config.nonce==message.nonce, MessageError::InvalidMessageNonce);
        message_config.increment_nonce()?;
    }
    Ok(())
}


#[cfg(test)]
pub mod test_verify_message {
    // use super::*;
    // use std::cell::RefCell;
    // use ethers::utils::keccak256;

    // use crate::states::bridge_config::test_bridge_config::build_bridge_config;
    // use crate::states::committee::committee_test::{build_default_committee_with_config,get_signature,get_default_addresses};
    // use crate::states::message_verifier::test_message_verifier::build_message_verifier;
    // use crate::states::message_config::test_message_config::build_message_config;

    // fn build_bridge_verifier() -> (RefCell<BridgeConfig>, RefCell<Committee>, RefCell<MessageVerifier>) {
    //     let bump=255;
    //     let solana_chain_id=66;
    //    // let benfen_chain_id=2;
    //     let bridge_config=build_bridge_config(bump, solana_chain_id);
    //     let committee=build_default_committee_with_config().0;
    //     let message_verifier=build_message_verifier();

    //     (bridge_config,committee,message_verifier)
    // }


    // #[test]
    // fn test_verify_bridge_signature(){
    //     let (bridge_config,committee,message_verifier)=build_bridge_verifier();
    //     let mut message_config=build_message_config(message_verifier.borrow().key(), EMERGENCY_OP);
    //     let message = Message {
    //         message_type: EMERGENCY_OP, 
    //         version: 1,
    //         nonce: 2,
    //         chain_id: 1,
    //         payload: vec![0], // true for freezing
    //     };
    //     let (_, wallets) = get_default_addresses();

    //     let encoded = encode_message(&message);
    //     let hash = keccak256(&encoded);
    //     let sig0 = get_signature(&hash, &wallets[0]);
    //     let sig1 = get_signature(&hash, &wallets[1]);
    //     let sig2 = get_signature(&hash, &wallets[2]);
    //     let sig3 = get_signature(&hash, &wallets[3]);
    //     verify_bridge_signature(
    //         &mut message_config,
    //         &mut bridge_config.borrow_mut(),
    //         &mut message_verifier.borrow_mut(),
    //         &mut committee.borrow_mut(),
    //         &message,
    //         vec![sig0,sig1,sig2,sig3],
    //     ).unwrap();
    // }
}
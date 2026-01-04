use anchor_lang::prelude::*;
use crate::errors::BridgeCommitteeError;
use crate::errors::BridgeConfigError;
use std::collections::HashSet;
use crate::states::message::*;
use solana_program::secp256k1_recover;

/// Seed used for committee account
pub const COMMITTEE_SEED: &str = "committee";

// Benfen chain currently has 18 nodes, this number will not change
// And reserve space for 64 nodes on Solana chain
pub const MAX_COMMITTEE_MEMBERS: usize = 64;

/// Recovery ID used for Ethereum signature recovery
const RECOVERY_ID: u8 = 27;

/// Length of an Ethereum address in bytes
const ETH_ADDRESS_LENGTH: usize = 20;

/// Length of an Ethereum signature in bytes
const SIGNATURE_LENGTH: usize = 65;

/// Maximum index of a committee member
const MAX_MEMBER_INDEX: u8 = 128;


#[account(zero_copy)]
#[repr(C, packed)]
pub struct Committee {
    pub bump: [u8;1],
    pub member_count: u8,
    pub members: [CommitteeMember; MAX_COMMITTEE_MEMBERS],
    pub min_stake_required: u16,
    pub config: Pubkey,
    //upgrade padding
    pub padding: [u8; 16],
}

#[zero_copy]
#[repr(C, packed)]
#[derive(Default)]
pub struct CommitteeMember {
    pub address: [u8; ETH_ADDRESS_LENGTH], // ethereum address (ETH_ADDRESS_LENGTH bytes)
    pub stake: u16,
    pub index: u8,
    pub is_blocklisted: u8,
}

impl Default for Committee {
    fn default() -> Self {
        let mut members = [CommitteeMember::default(); MAX_COMMITTEE_MEMBERS];
        for i in 0..MAX_COMMITTEE_MEMBERS {
            members[i] = CommitteeMember {
                address: [0; 20],
                stake: 0,
                index: i as u8,
                //is_blocklisted keep consistent with EVM contract
                // 0 is false 1 is true
                is_blocklisted: 1,
            };
        }
        Self {
            bump: [0],
            member_count: 0,
            members,
            min_stake_required: 0,
            config: Pubkey::default(),
            padding: [0; 16],
        }
    }
}

impl Committee {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn initialize(
        &mut self,
        bump: u8,
        members: &[CommitteeMember],
        min_stake_required: u16,
    ) -> Result<()> {
        self.bump[0] = bump;
        require!(members.len() > 0, BridgeCommitteeError::CommitteeSizeExceeded);
        require!(members.len() <= MAX_COMMITTEE_MEMBERS, BridgeCommitteeError::CommitteeSizeExceeded);
        let mut total_stake = 0u16;
        let mut seen_addresses = HashSet::new();
        for (i, member) in members.iter().enumerate() {
            require!(member.index < MAX_MEMBER_INDEX, BridgeCommitteeError::InvalidMemberIndex);
            require!(member.index ==i as u8, BridgeCommitteeError::InvalidMemberIndex);    
            require!(
                seen_addresses.insert(member.address),
                BridgeCommitteeError::DuplicateMemberAddress
            );    
            total_stake = total_stake
                .checked_add(member.stake)
                .ok_or(BridgeCommitteeError::StakeOverflow)?;
            self.members[i] = *member;
            // set is_blocklisted to 0 (false)
            self.members[i].is_blocklisted = 0; //0 = false, 1 = true
        }
        
        require!(total_stake >= min_stake_required, BridgeCommitteeError::InsufficientStake);
        
        self.member_count = members.len() as u8;
        self.min_stake_required = min_stake_required;
        
        Ok(())
    }



    pub fn initialize_config(
        &mut self,
        config: Pubkey,
    ) -> Result<()> {
        require!(config != Pubkey::default(), BridgeConfigError::InvalidConfigPubkey);
        self.config = config;
        Ok(())
    }

    pub fn verify_signatures(
        &self,
        signatures: Vec<Vec<u8>>,
        message: &Message,
    ) -> Result<()> {
        let required_stake = compute_required_stake(message);
        let message_hash = compute_message_hash(message);
        let mut approval_stake = 0u16;
        let mut bitmap = 0u128;
        let members = &self.members[..self.member_count as usize];

        for signature in signatures {
            let signer = Self::recover_signer(&message_hash, &signature)?;

            let member = members
                .iter()
                .find(|m| m.address == signer)
                .ok_or(BridgeCommitteeError::InvalidSigner)?;

            require!(member.is_blocklisted == 0, BridgeCommitteeError::SignerBlocklisted);
            require!(member.stake > 0, BridgeCommitteeError::InsufficientStake);
            require!(member.index < MAX_MEMBER_INDEX, BridgeCommitteeError::InvalidMemberIndex);


            let mask = 1u128 << member.index;
            require!((bitmap & mask) == 0u128, BridgeCommitteeError::DuplicateSignature);
            bitmap |= mask;

            approval_stake = approval_stake
                .checked_add(member.stake)
                .ok_or(BridgeCommitteeError::StakeOverflow)?;
        }
        
        require!(approval_stake as u32 >= required_stake, BridgeCommitteeError::InsufficientStake);
        Ok(())
    }

    pub fn update_blocklist(
        &mut self,
        addresses: &[[u8; ETH_ADDRESS_LENGTH]],
        is_blocklisted: u8,
    ) {
       let members = &mut self.members[..self.member_count as usize];
        for address in addresses {
            if let Some(member) = members.iter_mut().find(|m| m.address == *address) {
                member.is_blocklisted = is_blocklisted;
            }
        }
    }


    pub fn is_blocklisted(&self, address: &[u8; ETH_ADDRESS_LENGTH]) -> bool {
        self.members.iter().any(|m| m.address == *address && m.is_blocklisted == 1)
    }

    pub fn is_member(&self, address: &[u8; ETH_ADDRESS_LENGTH]) -> bool {
        self.members.iter().any(|m| m.address == *address)
    }


    /// Splits the provided signature into its r, s, and v components
    ///
    /// # Arguments
    /// * `signature` - The signature to be split
    ///
    /// # Returns
    /// A tuple containing:
    /// * r - The r component of the signature
    /// * s - The s component of the signature
    /// * v - The v component of the signature (adjusted for Ethereum verification)
    ///
    /// # Errors
    /// * Returns error if signature length is not 65 bytes
    pub fn split_signature(signature: &[u8]) -> Result<([u8; 32], [u8; 32], u8)> {
        require!(signature.len() == SIGNATURE_LENGTH, BridgeCommitteeError::InvalidSignature);

        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        
        r.copy_from_slice(&signature[0..32]);
        s.copy_from_slice(&signature[32..64]);
        let mut v = signature[64];

        // Adjust for ethereum signature verification
        if v < RECOVERY_ID {
            v += RECOVERY_ID;
        }

        Ok((r, s, v))
    }


    /// Recovers the signer's Ethereum address from a message hash and signature
    ///
    /// # Arguments
    /// * `hash` - The message hash that was signed
    /// * `signature` - The signature bytes
    ///
    /// # Returns
    /// The recovered Ethereum address if successful
    pub fn recover_signer(hash: &[u8; 32], signature: &[u8]) -> Result<[u8; 20]> {
        require!(signature.len() == 65, BridgeCommitteeError::InvalidSignature);

        let (r, s, v) = Self::split_signature(signature)?;
        
        // Combine r and s into signature bytes
        let mut sig_bytes = [0u8; 64];
        sig_bytes[..32].copy_from_slice(&r);
        sig_bytes[32..].copy_from_slice(&s);

        // Convert v to recovery_id by subtracting RECOVERY_ID
        let recovery_id = v.checked_sub(RECOVERY_ID).ok_or(BridgeCommitteeError::InvalidSignature)?;

        let pubkey = secp256k1_recover::secp256k1_recover(
            hash,
            recovery_id,
            &sig_bytes
        ).map_err(|_| BridgeCommitteeError::InvalidSignature)?;
       

        Ok(pubkey_to_eth_address(pubkey.0))
    }

    pub fn seeds(&self) -> [&[u8]; 3] {
        [
            COMMITTEE_SEED.as_bytes(),
            self.config.as_ref(),
            self.bump.as_ref(),
        ]
    }

    pub fn key(&self) -> Pubkey {
        Pubkey::create_program_address(&self.seeds(), &crate::id()).unwrap()
        //  Pubkey::create_program_address(&self.seeds(), &crate::id()).unwrap()

    }
} 

#[cfg(test)]
pub mod committee_test {
    use super::*;
    use std::cell::RefCell;
    use ethers::signers::{LocalWallet, Signer};
    use ethers::utils::keccak256;
    use ethers::types::{Address, H256};


    use crate::states::bridge_config::test_bridge_config;
    use crate::states::bridge_config::BridgeConfig;

    pub fn create_committee(
        address: [u8; 20],
        stake: u16,
        index: u8,
        is_blocklisted: u8,
    )->CommitteeMember{
        CommitteeMember{
            address,
            stake,
            index,
            is_blocklisted,
        }
    }

    pub fn get_default_addresses() -> ([[u8;20];4], [LocalWallet; 4]) {
        let private1_key = "3e3ab7b01d3b9333af72e5094faef36caa316ae5a18860af06c318279a0d526c";
        let private2_key = "422f5467c01a3ac9937fcfc92d3fbb3785bd589edea658a5fe62e802ba1d8a7b";
        let private3_key = "b7ca7b617b59b663435ffd0d40b8b85bda613613e058e879cc1f9d3ac88904ff";
        let private4_key = "6832e63b5460136f4ceb04d20bbc3aa3865ca755deb7cb1d11f6d6f63ff817da";
        
        let wallet1: LocalWallet = private1_key.parse().unwrap();
        let wallet2: LocalWallet = private2_key.parse().unwrap();
        let wallet3: LocalWallet = private3_key.parse().unwrap();
        let wallet4: LocalWallet = private4_key.parse().unwrap();

        let address1: [u8; 20] = wallet1.address().as_bytes().try_into().unwrap();
        let address2: [u8; 20] = wallet2.address().as_bytes().try_into().unwrap();
        let address3: [u8; 20] = wallet3.address().as_bytes().try_into().unwrap();
        let address4: [u8; 20] = wallet4.address().as_bytes().try_into().unwrap();
        
        let mut addresses = [[0u8; 20]; 4];
        let  wallets = [wallet1, wallet2, wallet3, wallet4];
        addresses[0] = address1;
        addresses[1] = address2;
        addresses[2] = address3;
        addresses[3] = address4;
        
        (addresses,wallets)
    }


    pub fn create_default_member() -> [CommitteeMember; 4] {
        let addresses = get_default_addresses().0;
        let mut members = [CommitteeMember::default(); 4];
        
        for (index, address) in addresses.iter().enumerate() {
            members[index] = create_committee(*address, 2500, index as u8, 0);
        }
        
        members
    }

    pub fn build_committee_with_config(
        members: &[CommitteeMember],
        min_stake_required: u16,
        config: Pubkey,
    )->RefCell<Committee>{
        let mut committee = Committee::default();
        let bump=255;
        committee.initialize(bump, members, min_stake_required).unwrap();
        committee.initialize_config(config).unwrap();
        RefCell::new(committee)
    }


    pub fn build_default_committee_with_config() -> (RefCell<Committee>,RefCell<BridgeConfig>){
        let solana_chain_id=1;
        let bump=255;
       // let supported_chains=[2,3];
        let config=test_bridge_config::build_bridge_config(bump,solana_chain_id);
        let config_key = config.borrow().key();
        let members = create_default_member();
       (build_committee_with_config(&members, 7500, config_key),config)
    }

    /// Helper function to get the signature components from a message hash and private key
    /// 
    /// # Arguments
    /// * `digest` - The message hash to be signed (32 bytes)
    /// * `wallet` - The wallet containing the private key for signing
    /// 
    /// # Returns
    /// A 65-byte signature in the format: r (32 bytes) + s (32 bytes) + v (1 byte)
    /// 
    /// # Note
    /// This function is designed for testing purposes and mimics the Solidity version
    /// The signature format is compatible with Ethereum's ECDSA signature standard
    pub fn get_signature(digest: &[u8; 32], wallet: &LocalWallet) -> Vec<u8> {
        use ethers::types::H256;
        
        // Sign the digest using the wallet
        let signature = wallet.sign_hash(H256::from_slice(digest)).unwrap();
        
        // Convert to bytes and return
        // The signature is already in the correct format: r + s + v (65 bytes total)
        signature.to_vec()
    }



    #[test]
    fn test_bitmap_verification() {
        let mut bitmap = 0u128;
        // Case 1 (index = 2)
        let index1 = 2u8;
        let mask1 = 1u128 << index1;
        // Verify bit is not set
        assert_eq!(bitmap & mask1, 0u128); 
        bitmap |= mask1;
        // Verify bit is set
        assert_eq!(bitmap & mask1, mask1); 

        // Case 2 (index = 5)
        let index2 = 5u8;
        let mask2 = 1u128 << index2;
        // Verify bit is not set
        assert_eq!(bitmap & mask2, 0u128);
        bitmap |= mask2;
        // Verify bit is set
        assert_eq!(bitmap & mask2, mask2); 

        // Case 3 Test duplicate signature (try to set index = 2 again)
        let duplicate_mask = 1u128 << index1;
        // Verify bit is already set
        assert_ne!(bitmap & duplicate_mask, 0u128); 

        // Verify final bitmap state
        let expected = (1u128 << index1) | (1u128 << index2);
        assert_eq!(bitmap, expected);
    }

    #[test]
    fn test_recover_signer_success() {
        let private_key = "005b4d436063f6fcec371a687cbed889f61eb8621ca6baf9a10c96ba2dfad114";
        let wallet: LocalWallet = private_key.parse().unwrap();

        // 获取地址
        let address: Address = wallet.address();
        println!("address: {:?}", address);
        // 签名消息
        let message = Message{
              message_type: 1,
              version: 1,
              nonce: 1,
              chain_id: 1,
              payload: vec![1u8;0],
        };

        let encoded = encode_message(&message);
        let hash = keccak256(&encoded);
        let sig = wallet.sign_hash(H256::from_slice(&hash)).unwrap();
        let sig_bytes = sig.to_vec();
        let sig_bytes = &sig_bytes[..];
        let signer = Committee::recover_signer(&hash, sig_bytes).unwrap();
        assert_eq!(signer, address.as_bytes());
    }

    #[test]
    fn test_update_blocklist(){
        let committee = build_default_committee_with_config().0;
        let address = get_default_addresses().0[0];
        assert_eq!(committee.borrow().is_blocklisted(&address), false);
        committee.borrow_mut().update_blocklist(&[address], 1);
        assert_eq!(committee.borrow().is_blocklisted(&address), true);
    }

    #[test]
    fn test_verify_signature(){
        let committee = build_default_committee_with_config().0;
        let (_,wallets) = get_default_addresses();
        let message = Message{
            message_type: 1,
            version: 1,
            nonce: 1,
            chain_id: 1,
            payload: vec![1u8;0],
        };


        let encoded = encode_message(&message);
        let hash = keccak256(&encoded);

        let mut signatures=Vec::new();

        for wallet in wallets{
            let sig = get_signature(&hash, &wallet);
            signatures.push(sig);
        }

        committee.borrow_mut().verify_signatures(signatures, &message).unwrap();
    }

    #[test]
    fn test_committee_initialization_edge_cases() {
        // Test empty members
        let mut committee = Committee::default();
        let empty_members: Vec<CommitteeMember> = vec![];
        let bump = 255;
        let result = committee.initialize(bump, &empty_members, 10);
        assert!(result.is_err());
        
        // Test oversized committee
        let oversized_members: Vec<CommitteeMember> = (0..=MAX_COMMITTEE_MEMBERS)
            .map(|i| CommitteeMember {
                address: [i as u8; 20],
                stake: 100,
                index: i as u8,
                is_blocklisted: 0,
            })
            .collect();

        let bump = 255;
        let result = committee.initialize(bump,&oversized_members, 100);
        assert!(result.is_err());
        
        // Test invalid member index
        let invalid_index_member = CommitteeMember {
            address: [1; 20],
            stake: 100,
            index: MAX_MEMBER_INDEX, // Invalid index
            is_blocklisted: 0,
        };
        let result = committee.initialize(bump,&[invalid_index_member], 100);
        assert!(result.is_err());
        
        // Test stake overflow
        let high_stake_members: Vec<CommitteeMember> = (0..4)
            .map(|i| CommitteeMember {
                address: [i as u8; 20],
                stake: u16::MAX / 2 + 1,
                index: i as u8,
                is_blocklisted: 0,
            })
            .collect();
        let result = committee.initialize(bump,&high_stake_members, 100);
        assert!(result.is_err());
        
        // Test insufficient total stake
        let low_stake_members: Vec<CommitteeMember> = (0..2)
            .map(|i| CommitteeMember {
                address: [i as u8; 20],
                stake: 50,
                index: i as u8,
                is_blocklisted: 0,
            })
            .collect();
        let result = committee.initialize(bump,&low_stake_members, 200);
        assert!(result.is_err());
    }

    #[test]
    fn test_blocklist_operations() {
        let committee = build_default_committee_with_config().0;
        let (addresses, _) = get_default_addresses();
        
        // Test normal blocklist update
        assert_eq!(committee.borrow().is_blocklisted(&addresses[0]), false);
        committee.borrow_mut().update_blocklist(&[addresses[0]], 1);
        assert_eq!(committee.borrow().is_blocklisted(&addresses[0]), true);
        
        // Test blocklist removal
        committee.borrow_mut().update_blocklist(&[addresses[0]], 0);
        assert_eq!(committee.borrow().is_blocklisted(&addresses[0]), false);
        
        // Test multiple addresses blocklist
        committee.borrow_mut().update_blocklist(&[addresses[0], addresses[1]], 1);
        assert_eq!(committee.borrow().is_blocklisted(&addresses[0]), true);
        assert_eq!(committee.borrow().is_blocklisted(&addresses[1]), true);
        assert_eq!(committee.borrow().is_blocklisted(&addresses[2]), false);
        
        // Test non-existent address
        let non_existent_address = [255u8; 20];
        committee.borrow_mut().update_blocklist(&[non_existent_address], 1);
        assert_eq!(committee.borrow().is_blocklisted(&non_existent_address), false);
    }

    #[test]
    fn test_signature_splitting_edge_cases() {
        // Test invalid signature length
        let short_sig = vec![0u8; 64]; // Too short
        let result = Committee::split_signature(&short_sig);
        assert!(result.is_err());
        
        let long_sig = vec![0u8; 66]; // Too long
        let result = Committee::split_signature(&long_sig);
        assert!(result.is_err());
        
        // Test valid signature with v adjustment
        let mut valid_sig = vec![0u8; 65];
        valid_sig[64] = 0; // v < RECOVERY_ID
        let result = Committee::split_signature(&valid_sig);
        assert!(result.is_ok());
        let (_, _, v) = result.unwrap();
        assert_eq!(v, RECOVERY_ID);
        
        // Test signature with v >= RECOVERY_ID
        valid_sig[64] = RECOVERY_ID + 1;
        let result = Committee::split_signature(&valid_sig);
        assert!(result.is_ok());
        let (_, _, v) = result.unwrap();
        assert_eq!(v, RECOVERY_ID + 1);
    }

    #[test]
    fn test_signature_recovery_edge_cases() {
        // Test invalid signature length
         let hash = [0u8; 32];
        let short_sig = vec![0u8; 64];
        let result = Committee::recover_signer(&hash, &short_sig);
        assert!(result.is_err());
        
        // Test zero signature
        let zero_sig = vec![0u8; 65];
        let result = Committee::recover_signer(&hash, &zero_sig);
        assert!(result.is_err());
        
    }

    #[test]
    fn test_verify_signatures_edge_cases() {
        let committee = build_default_committee_with_config().0;
        let (addresses, wallets) = get_default_addresses();
        
        // Test with blocklisted signer
        committee.borrow_mut().update_blocklist(&[addresses[0]], 1);
        
        let message = Message {
            message_type: 1,
            version: 1,
            nonce: 1,
            chain_id: 1,
            payload: vec![],
        };
        
        let encoded = encode_message(&message);
        let hash = keccak256(&encoded);
        let sig = get_signature(&hash, &wallets[0]);
        
        let result = committee.borrow().verify_signatures(vec![sig], &message);
        assert!(result.is_err());
        
        // Reset blocklist
        committee.borrow_mut().update_blocklist(&[addresses[0]], 0);
        
        // Test duplicate signatures
        let sig1 = get_signature(&hash, &wallets[0]);
        let sig2 = get_signature(&hash, &wallets[0]); // Same signer
        
        let result = committee.borrow().verify_signatures(vec![sig1, sig2], &message);
        assert!(result.is_err());
        
        // Test insufficient stake
        let low_stake_message = Message {
            message_type: 0, // TOKEN_TRANSFER requires 3334 stake
            version: 1,
            nonce: 1,
            chain_id: 1,
            payload: vec![],
        };
        
        let encoded = encode_message(&low_stake_message);
        let hash = keccak256(&encoded);
        let sig = get_signature(&hash, &wallets[0]); // Only 2500 stake
        
        let result = committee.borrow().verify_signatures(vec![sig], &low_stake_message);
        assert!(result.is_err());


        //Test stake success
        let sig = get_signature(&hash, &wallets[0]); //  2500 stake
        let sig1: Vec<u8> = get_signature(&hash, &wallets[1]); //  2500 stake
        
        let result = committee.borrow().verify_signatures(vec![sig, sig1], &low_stake_message);
        assert!(result.is_ok());


    }

    #[test]
    fn test_committee_size_calculation() {
        // Verify the SIZE constant matches actual struct size
        let expected_size = 8 + // discriminator
            1 + // bump
            1 + // member_count
            (20 + 2 + 1 + 1) * MAX_COMMITTEE_MEMBERS + // members
            2 + // min_stake_required
            32 + // config
            16; // padding
        
        assert_eq!(Committee::SPACE, expected_size);
    }

    #[test]
    fn test_committee_member_structure() {
        let member = CommitteeMember {
            address: [1u8; 20],
            stake: 1000,
            index: 5,
            is_blocklisted: 1,
        };
        
        // Copy packed fields to local variables to avoid unaligned references
        let address = member.address;
        let stake = member.stake;
        let index = member.index;
        let is_blocklisted = member.is_blocklisted;
        
        assert_eq!(address.len(), ETH_ADDRESS_LENGTH);
        assert_eq!(stake, 1000);
        assert_eq!(index, 5);
        assert_eq!(is_blocklisted, 1);
    }

    #[test]
    fn test_initialize_config_edge_cases() {
        let mut committee = Committee::default();
        
        // Test invalid config (default pubkey)
        let result = committee.initialize_config(Pubkey::default());
        assert!(result.is_err());
        
        // Test valid config
        let valid_config = Pubkey::new_unique();
        let result = committee.initialize_config(valid_config);
        assert!(result.is_ok());
        assert_eq!(committee.config, valid_config);
    }

    #[test]
    fn test_signature_verification_with_different_message_types() {
        let committee = build_default_committee_with_config().0;
        let (_, wallets) = get_default_addresses();
        
        // Test EMERGENCY_OP message (requires different stake)
        let emergency_message = Message {
            message_type: 2, // EMERGENCY_OP
            version: 1,
            nonce: 1,
            chain_id: 1,
            payload: vec![0], // true for freezing
        };
        
        let encoded = encode_message(&emergency_message);
        let hash = keccak256(&encoded);
        
        // Use only first wallet (2500 stake > 450 required for freezing)
        let sig = get_signature(&hash, &wallets[0]);
        let result = committee.borrow().verify_signatures(vec![sig], &emergency_message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_signatures_with_duplicate_signature() {
        let committee = build_default_committee_with_config().0;
        let (_, wallets) = get_default_addresses();
        
        let invalid_message = Message {
            message_type: 2, // EMERGENCY_OP
            version: 1,
            nonce: 1,
            chain_id: 1,
            payload: vec![0], // true for freezing
        };
        
        let encoded = encode_message(&invalid_message);
        let hash = keccak256(&encoded);
        let sig0 = get_signature(&hash, &wallets[0]);
        let sig1 = get_signature(&hash, &wallets[1]);
        let sig2 = get_signature(&hash, &wallets[2]);
        let sig3 = get_signature(&hash, &wallets[2]);
        
        let result = committee.borrow().verify_signatures(vec![sig0,sig1,sig2,sig3], &invalid_message);
        assert_eq!(result.unwrap_err(), BridgeCommitteeError::DuplicateSignature.into())
    }

    // #[test]
    // fn test_verify_signatures_with_invalid_nonce() {
    //     let committee = build_default_committee_with_config().0;

    //     let (_, wallets) = get_default_addresses();

    //     let invalid_message = Message {
    //         message_type: 2, // EMERGENCY_OP
    //         version: 1,
    //         nonce: 2,
    //         chain_id: 1,
    //         payload: vec![0], // true for freezing
    //     };

    //     let encoded = encode_message(&invalid_message);
    //     let hash = keccak256(&encoded);
    //     let sig0 = get_signature(&hash, &wallets[0]);
    //     let sig1 = get_signature(&hash, &wallets[1]);
    //     let sig2 = get_signature(&hash, &wallets[2]);
    //     let sig3 = get_signature(&hash, &wallets[3]);


    //     let result = committee.borrow().verify_signatures(vec![sig0,sig1,sig2,sig3], &invalid_message);
    //    // assert_eq!(result.unwrap_err(), BridgeError::InvalidNonce.into())
    // }


    #[test]
    fn test_get_address(){
        let (_, addresses) = get_default_addresses();
        //println!("{:?}",addresses[0].address());

        for address in addresses.iter() {
            println!("{:?}",address.address());
        }
    }

   
}


pub mod token;

use anchor_lang::prelude::Pubkey;

pub const BPF_LOADER_UPGRADEABLE_ID: Pubkey =
    anchor_lang::prelude::pubkey!("BPFLoaderUpgradeab1e11111111111111111111111");

pub fn bridge_config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[crate::states::bridge_config::CONFIG_SEED.as_bytes()],
        &crate::id(),
    )
}

pub fn committee_pda(bridge_config: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            crate::states::committee::COMMITTEE_SEED.as_bytes(),
            bridge_config.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn message_verifier_pda(committee: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            crate::states::message_verifier::MESSAGE_VERIFIER_SEED.as_bytes(),
            committee.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn benfen_bridge_pda(committee: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            crate::states::benfen_bridge::BENFEN_BRIDGE_SEED.as_bytes(),
            committee.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn upgrade_authority_pda(committee: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            crate::states::upgrade_authority::UPGRADE_AUTHORITY_SEED.as_bytes(),
            committee.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn chain_limit_pda(bridge_config: &Pubkey, chain_id: u8) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            crate::states::chain_limit::CHAIN_LIMIT_SEED.as_bytes(),
            &[chain_id],
            bridge_config.as_ref(),
        ],
        &crate::id(),
    )
}

pub fn token_config_pda(token_id: u64) -> (Pubkey, u8) {
    let token_id_bytes = token_id.to_be_bytes();
    Pubkey::find_program_address(
        &[
            crate::states::token_config::TOKEN_CONFIG_SEED.as_bytes(),
            token_id_bytes.as_ref(),
        ],
        &crate::id(),
    )
}


//测试生成的dpubkey是否和主网匹配
#[cfg(test)]
mod test_util_pdas {
    use std::str::FromStr;

    use super::*;
    use anchor_lang::prelude::Pubkey;

    #[test]
    fn test_bridge_config_pda_matches_create_program_address() {
        let pda = bridge_config_pda().0;
        let bridge_config_pda = Pubkey::from_str("2Yxi26CXpfAus6KXHqZ1cAVUN3xSK8ZPks7a5sTPSQ8M").expect("bridge config not match");
        assert_eq!(pda, bridge_config_pda);
    }

    #[test]
    fn test_committee_pda_matches_create_program_address() {
        let bridge_config = bridge_config_pda().0;
        let pda = committee_pda(&bridge_config).0;
        let expected=Pubkey::from_str("63nBxQUZxfd3QLLBiJg2JP5yEqjQrsEXorJCn7i44JQU").unwrap();
       
        assert_eq!(pda, expected);
    }

    #[test]
    fn test_message_verifier_pda_matches_create_program_address() {
        let bridge_config = bridge_config_pda().0;
        let committee = committee_pda(&bridge_config).0;
        let pda= message_verifier_pda(&committee).0;
        let expected =Pubkey::from_str("AN52vrqyjbqgWofPfZ2AMVTXzZWC72hrWsxwGRYLciK4").unwrap();
        assert_eq!(pda, expected);
    }

    #[test]
    fn test_benfen_bridge_pda_matches_create_program_address() {
        let bridge_config = bridge_config_pda().0;
        let committee = committee_pda(&bridge_config).0;
        let pda = benfen_bridge_pda(&committee).0;
        let expected =Pubkey::from_str("9Feuu3z1Zt7e49AW8u1itu7ht29RNWDETdhp4ZuNKu6S").unwrap();
        assert_eq!(pda, expected);
    }

    // #[test]
    // fn test_upgrade_authority_pda_matches_create_program_address() {
    //     let bridge_config = bridge_config_pda().0;
    //     let committee = committee_pda(&bridge_config).0;
    //     let (pda, bump) = upgrade_authority_pda(&committee);
    //     let expected = Pubkey::create_program_address(
    //         &[
    //             crate::states::upgrade_authority::UPGRADE_AUTHORITY_SEED.as_bytes(),
    //             committee.as_ref(),
    //             &[bump],
    //         ],
    //         &crate::id(),
    //     )
    //     .unwrap();
    //     assert_eq!(pda, expected);
    // }

    #[test]
    fn test_chain_limit_pda_matches_create_program_address() {
        let bridge_config = bridge_config_pda().0;
        let chain_id = 0u8;
        let pda= chain_limit_pda(&bridge_config, chain_id).0;
        let expected =Pubkey::from_str("DbbXNCdxKEcgE8L7cMUpY5xE1qPAU3XFyQj2NEMbyTGG").unwrap();
        assert_eq!(pda, expected);
    }

    #[test]
    fn test_token_config_pda_matches_create_program_address() {
        let token_id = 3u64;
        let pda = token_config_pda(token_id).0;
        let expected =Pubkey::from_str("7ZWQeHHBM6SU38tDNaydpyte7rPcKcj3wpu3gsUQmENr").unwrap();
        assert_eq!(pda, expected);
    }
}

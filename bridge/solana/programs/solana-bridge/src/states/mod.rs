


pub mod bridge_config;
pub mod token_config;
pub mod committee;
pub mod message_verifier;
pub mod message_config;
pub mod message;
pub mod upgrade_authority;
pub mod chain_limit;

pub mod benfen_bridge;
pub mod process_transfer;

#[cfg(test)]
mod space_tests {
    use super::{
        benfen_bridge::BenfenBridge,
        bridge_config::BridgeConfig,
        chain_limit::ChainLimit,
        committee::Committee,
        message_config::MessageConfig,
        message_verifier::MessageVerifier,
        process_transfer::ProcessTransfer,
        token_config::TokenConfigAccount,
        upgrade_authority::UpgradeAuthority,
    };
    use anchor_lang::prelude::AnchorSerialize;

    #[test]
    fn test_non_zero_copy_space_matches_anchor_serialize() {
        let bridge = BenfenBridge::default();
        assert_eq!(BenfenBridge::SPACE, 8 + bridge.try_to_vec().unwrap().len());
        assert!(BenfenBridge::SPACE <= 8 + std::mem::size_of::<BenfenBridge>());

        let msg = MessageConfig::default();
        assert_eq!(MessageConfig::SPACE, 8 + msg.try_to_vec().unwrap().len());
        assert!(MessageConfig::SPACE <= 8 + std::mem::size_of::<MessageConfig>());

        let p = ProcessTransfer::default();
        assert_eq!(ProcessTransfer::SPACE, 8 + p.try_to_vec().unwrap().len());
        assert!(ProcessTransfer::SPACE <= 8 + std::mem::size_of::<ProcessTransfer>());

        let ua = UpgradeAuthority {
            enabled: true,
            bump: [1],
            current_version: 1,
            last_upgrade_timestamp: 0,
            committee: Default::default(),
        };
        assert_eq!(UpgradeAuthority::SPACE, 8 + ua.try_to_vec().unwrap().len());
        assert!(UpgradeAuthority::SPACE <= 8 + std::mem::size_of::<UpgradeAuthority>());
    }

    #[test]
    fn test_zero_copy_space_matches_sizeof() {
        assert_eq!(BridgeConfig::LEN, 8 + std::mem::size_of::<BridgeConfig>());
        assert_eq!(ChainLimit::SPACE, 8 + std::mem::size_of::<ChainLimit>());
        assert_eq!(Committee::SPACE, 8 + std::mem::size_of::<Committee>());
        assert_eq!(
            MessageVerifier::SPACE,
            8 + std::mem::size_of::<MessageVerifier>()
        );
        assert_eq!(
            TokenConfigAccount::SPACE,
            8 + std::mem::size_of::<TokenConfigAccount>()
        );
    }
}




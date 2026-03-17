use anchor_lang::prelude::*;

pub const CONFIG_SEED: &str = "bridge_config";

use crate::errors::BridgeConfigError;


#[account(zero_copy)]
#[repr(C, packed)]
pub struct BridgeConfig {
    /// Bump seed for PDA
    pub bump: [u8;1],
    // solana chain id in bridge node
    pub chain_id: u8, 
    // 记录当前bridge支持的token数量
    pub token_count: u64, 
    // 记录当前支持跨到的链数(预留字段，暂时设计只支持一个链(To Benfen))，
    // 每增加一个链，就会多创建一个 ChainLimiter 
    pub chain_count: u8, 
    // 同上,每个链对应一个ChainLimiter 
    pub supported_chains: [u8; 256], 

    // upgrade  padding
    pub padding: [u64; 10], 
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            bump: [0],
            chain_id: 0,
            token_count: 0,
            chain_count: 0,
            supported_chains: [255; 256],
            padding: [0; 10],
        }
    }
}


impl BridgeConfig {
    pub const LEN: usize = 8
        + 1
        + 1
        + 8
        + 1
        + 256
        + (10 * 8);

    pub fn initialize(
        &mut self,
        bump: u8,
        chain_id: u8,
    ) -> Result<()> {
        self.bump = [bump];
        self.chain_id = chain_id;
        self.supported_chains = [255; 256];

        Ok(())
    }

    pub fn benfen_chain_id(&self) -> u8 {
        (self.padding[0] & 0xff) as u8
    }

    pub fn set_benfen_chain_id(&mut self, chain_id: u8) {
        self.padding[0] = (self.padding[0] & !0xff) | (chain_id as u64);
    }

    pub fn is_chain_supported(&self, chain_id: u8) -> bool {
        self.supported_chains.contains(&chain_id) && self.chain_count > 0 && chain_id != 255
    }

    pub fn increment_token_count(&mut self) -> Result<()> {
        self.token_count = self
            .token_count
            .checked_add(1)
            .ok_or(BridgeConfigError::TokenCountOverflow)?;
        Ok(())
    }

    pub fn add_target_chain(&mut self, chain_id: u8) -> Result<()> {
        require!(self.chain_id != chain_id, BridgeConfigError::CannotSupportSelf);
        require!(chain_id != 255, BridgeConfigError::InvalidChainId);
        
        require!(!self.is_chain_supported(chain_id), BridgeConfigError::ChainAlreadySupported);
        require!(self.chain_count < u8::MAX, BridgeConfigError::ChainCountOverflow);
        self.supported_chains[self.chain_count as usize] = chain_id;
        self.chain_count = self
            .chain_count
            .checked_add(1)
            .ok_or(BridgeConfigError::ChainCountOverflow)?;
        Ok(())
    }

    pub fn seeds(&self) -> [&[u8]; 2] {
        [
            CONFIG_SEED.as_bytes(),
            self.bump.as_ref(),
        ]
    }

    pub fn key(&self) -> Pubkey {
        Pubkey::create_program_address(&self.seeds(), &crate::id()).unwrap()
    }
}
#[cfg(test)]
pub mod test_bridge_config {
    use super::*;
    use std::cell::RefCell;

    pub fn build_bridge_config(
        bump: u8,
        chain_id: u8,
    ) -> RefCell<BridgeConfig> {
        let _ = bump;
        let (_, derived_bump) =
            Pubkey::find_program_address(&[CONFIG_SEED.as_bytes()], &crate::id());
        let mut bridge_config = BridgeConfig::default();
        bridge_config.initialize(derived_bump, chain_id).unwrap();
        RefCell::new(bridge_config)
    }


    #[test]
    fn test_add_target_chain_with_benfen_mainnet() {
        let solana_mainnet_chain_id = 66;
        let benfen_mainnet_chain_id = 0;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        assert!(!bridge_config.borrow().is_chain_supported(benfen_mainnet_chain_id));
        bridge_config.borrow_mut().add_target_chain(benfen_mainnet_chain_id).unwrap();
        assert!(bridge_config.borrow().is_chain_supported(benfen_mainnet_chain_id));
    }


    #[test]
    fn test_add_target_chain_with_benfen_testnet() {
        let solana_testnet_chain_id = 67;
        let benfen_testnet_chain_id = 1;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_testnet_chain_id);
        assert!(!bridge_config.borrow().is_chain_supported(benfen_testnet_chain_id));
        bridge_config.borrow_mut().add_target_chain(benfen_testnet_chain_id).unwrap();
        assert!(bridge_config.borrow().is_chain_supported(benfen_testnet_chain_id));
    }

    #[test]
    fn test_is_chain_supported() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        assert!(!bridge_config.borrow().is_chain_supported(1));
    }



    #[test]
    fn test_add_token_in_bridge_config() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        let mut token_count = bridge_config.borrow().token_count;
        assert_eq!(token_count, 0);
        bridge_config.borrow_mut().increment_token_count().unwrap();
        token_count = bridge_config.borrow().token_count;
        assert_eq!(token_count, 1);
    }

    #[test]
    fn test_multiple_token_increments() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        
        // 测试多次增加token计数
        for i in 1..=5 {
            bridge_config.borrow_mut().increment_token_count().unwrap();
            let token_count=bridge_config.borrow().token_count;
            assert_eq!(token_count, i);
        }
    }

    #[test]
    fn test_add_target_chain_error_same_chain() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        
        // 测试添加相同链ID应该失败
        let result = bridge_config.borrow_mut().add_target_chain(solana_mainnet_chain_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_target_chain_error_duplicate() {
        let solana_mainnet_chain_id = 66;
        let benfen_mainnet_chain_id = 0;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        
        // 第一次添加应该成功
        bridge_config.borrow_mut().add_target_chain(benfen_mainnet_chain_id).unwrap();
        assert!(bridge_config.borrow().is_chain_supported(benfen_mainnet_chain_id));
        
        // 第二次添加相同链应该失败
        let result = bridge_config.borrow_mut().add_target_chain(benfen_mainnet_chain_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_target_chain_error_reserved_chain_id() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);

        let result = bridge_config.borrow_mut().add_target_chain(255);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_target_chains() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        
        let chains_to_add = [0, 1, 2, 3, 4]; // 添加多个不同的链
        
        for (index, &chain_id) in chains_to_add.iter().enumerate() {
            bridge_config.borrow_mut().add_target_chain(chain_id).unwrap();
            assert!(bridge_config.borrow().is_chain_supported(chain_id));
            assert_eq!(bridge_config.borrow().chain_count, (index + 1) as u8);
        }
        
        // 验证所有链都被支持
        for &chain_id in &chains_to_add {
            assert!(bridge_config.borrow().is_chain_supported(chain_id));
        }
    }

    #[test]
    fn test_is_chain_supported_empty_config() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        
        // 测试空配置时不支持任何链
        assert!(!bridge_config.borrow().is_chain_supported(0));
        assert!(!bridge_config.borrow().is_chain_supported(1));
        assert!(!bridge_config.borrow().is_chain_supported(255));
    }

    #[test]
    fn test_bridge_config_default() {
        let config = BridgeConfig::default();
        
        assert_eq!(config.chain_id, 0);
        let token_count=config.token_count;
        assert_eq!(token_count, 0);
        //assert_eq!(config.token_count, 0);
        assert_eq!(config.chain_count, 0);
        // 对于packed结构体，需要复制字段内容以避免对齐问题
        let supported_chains_copy = config.supported_chains;
        let padding_copy = config.padding;
        assert_eq!(supported_chains_copy, [255; 256]);
        assert_eq!(padding_copy, [0; 10]);
    }

    #[test]
    fn test_benfen_chain_id_default() {
        let config = BridgeConfig::default();
        assert_eq!(config.benfen_chain_id(), 0);
    }

    #[test]
    fn test_benfen_chain_id_roundtrip() {
        let mut config = BridgeConfig::default();
        config.set_benfen_chain_id(1);
        assert_eq!(config.benfen_chain_id(), 1);

        config.set_benfen_chain_id(255);
        assert_eq!(config.benfen_chain_id(), 255);
    }

    #[test]
    fn test_benfen_chain_id_preserves_padding_upper_bits() {
        let mut config = BridgeConfig::default();
        config.padding = [0x1122_3344_5566_7700, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        config.set_benfen_chain_id(0xaa);
        let padding_copy = config.padding;
        assert_eq!(padding_copy[0], 0x1122_3344_5566_77aa);
        assert_eq!(config.benfen_chain_id(), 0xaa);
    }

    #[test]
    fn test_bridge_config_size() {
        // 验证LEN常量计算正确
        assert_eq!(BridgeConfig::LEN, 8 + 1 + 1 + 8 + 1 + 256 + 80);
        assert_eq!(BridgeConfig::LEN, 355);
    }

    #[test]
    fn test_initialize_different_chain_ids() {
        let chain_ids = [0, 1, 66, 67, 255];
        let bump=255;
        
        for &chain_id in &chain_ids {
            let mut config = BridgeConfig::default();
            config.initialize(bump,chain_id).unwrap();
            assert_eq!(config.chain_id, chain_id);
        }
    }

    #[test]
    fn test_seeds_method() {
        let config = BridgeConfig::default();
        let seeds = config.seeds();
        
        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0], CONFIG_SEED.as_bytes());
        assert_eq!(seeds[0], b"bridge_config");
    }


    #[test]
    fn test_chain_count_increments_correctly() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        
        assert_eq!(bridge_config.borrow().chain_count, 0);
        
        bridge_config.borrow_mut().add_target_chain(0).unwrap();
        assert_eq!(bridge_config.borrow().chain_count, 1);
        
        bridge_config.borrow_mut().add_target_chain(1).unwrap();
        assert_eq!(bridge_config.borrow().chain_count, 2);
        
        bridge_config.borrow_mut().add_target_chain(2).unwrap();
        assert_eq!(bridge_config.borrow().chain_count, 3);
    }

    #[test]
    fn test_supported_chains_array_population() {
        let solana_mainnet_chain_id = 66;
        let bump=255;
        let bridge_config = build_bridge_config(bump,solana_mainnet_chain_id);
        
        let chains_to_add = [10, 20, 30];
        
        for (index, &chain_id) in chains_to_add.iter().enumerate() {
            bridge_config.borrow_mut().add_target_chain(chain_id).unwrap();
            
            // 验证链ID被正确存储在数组中
            assert_eq!(bridge_config.borrow().supported_chains[index], chain_id);
        }
    }


}

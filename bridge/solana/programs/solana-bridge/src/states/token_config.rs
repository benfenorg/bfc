use anchor_lang::prelude::*;
use crate::errors::BridgeTokenError;
use crate::errors::BridgeConfigError;
use crate::errors::BridgeLimiterError;

pub const TOKEN_CONFIG_SEED: &str = "token_config";

#[account(zero_copy)]
#[repr(C, packed)]
#[derive(Default)]
pub struct TokenConfigAccount {
    pub config: Pubkey, // bridge config in solana
    pub chain: Pubkey, //  cross to chain from solana 
    pub mint: Pubkey, // token mint in solana
    pub token_id: u64, // token id in chain
    pub price: u64, // 8 decimal precision
    pub decimal: u8, //solana decimal
    pub benfen_decimal: u8, // benfen decimal
    pub is_native: u8, //0: false, 1: true
    pub padding: [u8; 24], // upgrade padding
}
 

impl TokenConfigAccount {
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn initialize(
        &mut self,
        config: Pubkey,
        chain: Pubkey,
        mint: Pubkey,
        token_id: u64,
        price: u64,
        decimal: u8,
        benfen_decimal: u8,
        is_native: u8,
    ) -> Result<()> {
        require!(config != Pubkey::default(), BridgeConfigError::InvalidConfigPubkey);
        require!(chain != Pubkey::default(), BridgeLimiterError::InvalidLimiterPubkey);
        require!(mint != Pubkey::default(), BridgeTokenError::InvalidTokenMint);
        require!(token_id > 0, BridgeTokenError::InvalidTokenId);
        require!(price > 0, BridgeTokenError::InvalidTokenPrice);
        require!(decimal > 0, BridgeTokenError::InvalidFungibleTokenDecimals);
        require!(benfen_decimal > 0, BridgeTokenError::InvalidTokenBenfenDecimal);
        self.config = config;
        self.chain = chain;
        self.mint = mint;
        self.token_id = token_id;
        self.price = price;
        self.decimal = decimal;
        self.benfen_decimal = benfen_decimal;
        self.is_native = is_native;
        Ok(())
    }

    pub fn update_price(&mut self, price: u64) -> Result<()> {
        require!(price > 0, BridgeTokenError::InvalidTokenPrice);
        self.price = price;
        Ok(())
    }

    pub fn update_mint(&mut self, mint: Pubkey) -> Result<()> {
        require!(mint != Pubkey::default(), BridgeTokenError::InvalidTokenMint);
        self.mint = mint;
        Ok(())
    }

    pub fn update_benfen_decimal(&mut self, benfen_decimal: u8) -> Result<()> {
        require!(benfen_decimal > 0, BridgeTokenError::InvalidTokenBenfenDecimal);
        self.benfen_decimal = benfen_decimal;
        Ok(())
    }

    pub fn update_is_native(&mut self, is_native: u8) {
       // require!(self.is_native != is_native, BridgeError::SameTokenNative);
        self.is_native = is_native;
    }

    pub fn token_id(&self) -> u64 {
        self.token_id
    }

    pub fn is_native(&self) -> bool {
        self.is_native == 1
    }

    pub fn price(&self) -> u64 {
        self.price
    }
    pub fn decimal(&self) -> u8 {
        self.decimal
    }
    pub fn benfen_decimal(&self) -> u8 {
        self.benfen_decimal
    }

    pub fn mint(&self) -> Pubkey {
        self.mint
    }
    
}

#[cfg(test)]
pub mod token_config_test{
    use super::*;
    // use std::cell::RefCell;

    // pub fn build_token_config(
    //     token_id: u64,
    //     price: u64,
    //     mint: Pubkey,
    //     benfen_decimal: u8,
    //     is_native: u8,
    // ) -> RefCell<TokenConfigAccount> {
    //     let mut token_config = TokenConfigAccount::default();
    //     token_config.initialize(token_id, price, mint, benfen_decimal, is_native).unwrap();
    //     RefCell::new(token_config)
    // }


    #[test]
    fn test_update_price(){
        let mut token_config = TokenConfigAccount::default();
        token_config.initialize(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            1,
            1,
            1,
            1,
            1,
        ).unwrap();
        assert_eq!(token_config.price(), 1);
        token_config.update_price(100).unwrap();
        assert_eq!(token_config.price(), 100);
    }

    #[test]
    fn test_update_price_is_zero(){
        let mut token_config = TokenConfigAccount::default();
        token_config.initialize(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            1,
            1,
            1,
            1,
            1,
        ).unwrap();
        assert_eq!(token_config.price(), 1);
        let result=token_config.update_price(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_mint(){
        let mut token_config = TokenConfigAccount::default();
        let mint=Pubkey::new_unique();
        token_config.initialize(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            mint,
            1,
            1,
            1,
            1,
            1,
        ).unwrap();
        assert_eq!(token_config.mint(), mint);
        let twice_mint=Pubkey::new_unique();
        token_config.update_mint(twice_mint).unwrap();
        assert_eq!(token_config.mint(), twice_mint);
    }

    #[test]
    fn test_update_mint_is_default(){
        let mut token_config = TokenConfigAccount::default();
        let mint=Pubkey::new_unique();
        token_config.initialize(
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            mint,
            1,
            1,
            1,
            1,
            1,
        ).unwrap();
        assert_eq!(token_config.mint(), mint);
        let result=token_config.update_mint(Pubkey::default());
        assert!(result.is_err());
    }

    
}
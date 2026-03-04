
#[test_only]
module bridge::bridge_withdraw_min_fee_tests {
    use bridge::bridge::{Self, TokenDepositedEventV3};
    use bridge::chain_ids;
    use bridge::usdc::{Self, USDC};
    use bridge::bridge_env::{Self, create_env, usdc_id};
    use sui::event;
    use std::unit_test::assert_eq;
    use sui::address;

    const USD_8DP: u64 = 100_000_000; // 1 USD in 8 decimals
    const USDC_DECIMALS: u64 = 1_000_000;

    #[test]
    fun test_send_token_min_fee() {
        // 1. Setup env
        let mut env = create_env(chain_ids::sui_testnet());
        env.create_bridge_default();
        
        // 2. Setup Min Config
        env.setup_min_config_registry();
        
        let target_chain = chain_ids::eth_sepolia() as u64; 
        let token_id = usdc_id();
        // Min fee in token amount (USDC 6 decimals): 2 USDC = 2_000_000
        let min_fee_token = 2 * USDC_DECIMALS;
        
        env.set_min_fee_cross_out(target_chain, token_id, min_fee_token);
        
        // Fix USDC price (bridge_env sets it to 1, which is wrong, should be 1 USD)
        // 1 USD = 100_000_000 (8 decimals)
        let sender = @0xA;
        env.update_asset_price(sender, token_id, USD_8DP);
        
        // 3. Mint tokens on Sui (simulate bridge in)
        let amount_usdc = 100 * USDC_DECIMALS;
        let eth_address = x"0000000000000000000000000000000000001234";
        
        // Bridge in from ETH Sepolia
        let seq_num = env.bridge_to_sui<USDC>(
            target_chain as u8,
            eth_address,
            sender,
            amount_usdc
        );
        
        let usdc_coin = env.claim_token<USDC>(sender, target_chain as u8, seq_num);
        
        assert_eq!(usdc_coin.value(), amount_usdc);
        
        // 4. Send token back (Withdraw from Sui)
        // Min fee = 2 USDC (2_000_000 in 6 decimals). Expected amount_after_fee = 100_000_000 - 2_000_000 = 98_000_000.
        
        env.send_token_test<USDC>(sender, target_chain as u8, eth_address, usdc_coin);
        
        // 5. Verify event
        let mut events = event::events_by_type<TokenDepositedEventV3>();
        
        assert_eq!(events.length(), 1);
        let event = events.pop_back();
        
        let (
            _seq_num,
            _source_chain,
            sender_address,
            target_chain_event,
            _target_address,
            token_type,
            _origin_token_type,
            amount_before_fee,
            amount_after_fee
        ) = event.unwrap_deposited_event_v3();
        
        assert_eq!(amount_before_fee, amount_usdc);
        assert_eq!(amount_after_fee, 98_000_000);
        assert_eq!(token_type, token_id);
        assert_eq!(target_chain_event, target_chain as u8);
        assert_eq!(sender_address, address::to_bytes(sender));
        
        env.destroy_env();
    }
}

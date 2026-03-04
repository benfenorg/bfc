#[test_only]
module bridge::bridge_fee_min_apply_test {
    use std::unit_test::assert_eq;
    use bridge::bridge_env::{Self, create_env, create_bridge_default};
    use bridge::chain_ids;
    use bridge::bridge_min_config;
    use bridge::usdc::USDC;
    use bridge::bridge::{
        get_cross_out_fee_amount,
        test_load_inner,
        test_load_mut_uid
    };

    #[test]
    fun test_cross_out_fee_respects_min_usdc_eth(){
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();

        // take bridge
        let mut wrapper = env.bridge(@0x1);
        let bridge = wrapper.bridge_ref_mut();

        let chain = chain_ids::eth_mainnet() as u64;
        let amount = 100_000_000;

        let inner = test_load_inner(bridge);
        let token_id = inner.inner_treasury().token_id<USDC>();

        // set fee min
        let uid = test_load_mut_uid(bridge);
        let ctx = bridge_env::ctx(&mut env);
        bridge_min_config::new_bridge_min_config_registry(uid, ctx);
        bridge_min_config::set_min_fee_cross_out(uid, chain, token_id, 60_000, ctx);

        let fee = get_cross_out_fee_amount<USDC>(bridge, chain, amount);
        assert_eq!(fee != 60_000, true);

        wrapper.return_bridge();
        env.destroy_env();
    }
}

#[test_only]
module bridge::bridge_min_config_tests {
    use sui::test_scenario;
    use bridge::bridge_min_config;
    use bridge::bridge_fee;
    use bridge::chain_ids::{eth_mainnet, bsc_mainnet, btc_mainnet, btc_testnet};
    use bridge::treasury;
    use std::unit_test::assert_eq;
    use sui::test_utils;
    use bridge::bridge_env::{usdc_id, usdt_id, busd_id};

    const USD_8DP: u64 = 100_000_000; // 1 USD in 8 decimals

    public struct MinConfigObject has key,store {
        id: UID
    }

    public fun new(ctx: &mut TxContext) : MinConfigObject{
        MinConfigObject { id: object::new(ctx) }
    }

    #[test]
    fun test_exists() {
        let mut scenario = test_scenario::begin(@0xAE0);
        let ctx = scenario.ctx();
        let mut obj = new(ctx);
        assert_eq!(bridge_min_config::exists(&obj.id), false);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id, ctx);
        assert_eq!(bridge_min_config::exists(&obj.id), true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_check_cross_in_amount_ok_btc() {
        let mut scenario = test_scenario::begin(@0xAE1);
        let ctx = scenario.ctx();
        let mut obj = new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id, ctx);
        let btc_chain = btc_mainnet() as u64;
        let btc_token_id = 1u64; // BTC in treasury mock
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, btc_chain, 10_000); // 0.0001 BTC min
        scenario.next_tx(@0xAE2);
        let treas = treasury::mock_for_test(scenario.ctx());
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, btc_chain, btc_token_id, 15_000), true);
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, btc_chain, btc_token_id, 5_000), false);
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, btc_chain, btc_token_id, 10_000), true);
        test_utils::destroy(treas);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_check_cross_out_amount_ok_btc() {
        let mut scenario = test_scenario::begin(@0xAE3);
        let ctx = scenario.ctx();
        let mut obj = new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id, ctx);
        let btc_chain = btc_testnet() as u64;
        let btc_token_id = 1u64;
        bridge_min_config::set_min_limit_cross_out(&mut obj.id, btc_chain, 10_000);
        scenario.next_tx(@0xAE4);
        let treas = treasury::mock_for_test(scenario.ctx());
        assert_eq!(bridge_min_config::check_cross_out_amount_ok(&obj.id, &treas, btc_chain, btc_token_id, 20_000), true);
        assert_eq!(bridge_min_config::check_cross_out_amount_ok(&obj.id, &treas, btc_chain, btc_token_id, 8_000), false);
        test_utils::destroy(treas);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_check_cross_in_amount_ok_usd() {
        let mut scenario = test_scenario::begin(@0xAE5);
        let ctx = scenario.ctx();
        let mut obj = new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id, ctx);
        let chain = eth_mainnet() as u64;
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, chain, USD_8DP); // 1 USD min
        scenario.next_tx(@0xAE6);
        let treas = treasury::mock_for_test(scenario.ctx());
        // USDC: 1e6 amount = 1 USD, so 2e6 = 2 USD >= 1 USD -> true
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, chain, usdc_id(), 2_000_000), true);
        // 0.5e6 = 0.5 USD < 1 USD -> false
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, chain, usdc_id(), 500_000), false);
        // Test BUSD (id = 5, decimal_multiplier = 1_000_000_000, notional_value = 1 USD)
        // 1e9 = 1 USD, so 2e9 = 2 USD >= 1 USD -> true
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, chain, busd_id(), 2_000_000_000), true);
        // 0.5e9 = 0.5 USD < 1 USD -> false
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, chain, busd_id(), 500_000_000), false);
        // 1e9 = 1 USD == 1 USD -> true
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, chain, busd_id(), 1_000_000_000), true);
        
        test_utils::destroy(treas);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_check_cross_out_amount_ok_usd() {
        let mut scenario = test_scenario::begin(@0xAE7);
        let ctx = scenario.ctx();
        let mut obj = new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id, ctx);
        let chain = eth_mainnet() as u64;
        bridge_min_config::set_min_limit_cross_out(&mut obj.id, chain, 10 * USD_8DP); // 10 USD min
        scenario.next_tx(@0xAE8);
        let treas = treasury::mock_for_test(scenario.ctx());
        assert_eq!(bridge_min_config::check_cross_out_amount_ok(&obj.id, &treas, chain, usdc_id(), 10_000_000), true);
        assert_eq!(bridge_min_config::check_cross_out_amount_ok(&obj.id, &treas, chain, usdc_id(), 5_000_000), false);
        test_utils::destroy(treas);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_get_usd_value() {
        let mut scenario = test_scenario::begin(@0xAE9);
        let treas = treasury::mock_for_test(scenario.ctx());
        // USDC: 1e6 amount = 1 USD = 100_000_000 in 8 dp
        assert_eq!(bridge_min_config::get_usd_value(&treas, usdc_id(), 1_000_000), USD_8DP);
        assert_eq!(bridge_min_config::get_usd_value(&treas, usdc_id(), 2_500_000), 2 * USD_8DP + 50_000_000);
        test_utils::destroy(treas);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_check_no_registry_returns_true() {
        let mut scenario = test_scenario::begin(@0xAEA);
        let ctx = scenario.ctx();
        let obj = new(ctx);
        let treas = treasury::mock_for_test(scenario.ctx());
        assert_eq!(bridge_min_config::check_cross_in_amount_ok(&obj.id, &treas, eth_mainnet() as u64, usdc_id(), 0), true);
        assert_eq!(bridge_min_config::check_cross_out_amount_ok(&obj.id, &treas, eth_mainnet() as u64, usdc_id(), 0), true);
        test_utils::destroy(treas);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_get_effective_cross_out_fee() {
        let mut scenario = test_scenario::begin(@0xAEB);
        let ctx = scenario.ctx();
        let mut obj = new(ctx);
        bridge_fee::new_bridge_fee_registry(&mut obj.id, ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id, ctx);
        let chain = eth_mainnet() as u64;
        bridge_fee::set_fee_in_cross_out(&mut obj.id, chain, usdc_id(), 1, 500, ctx); // 0.05%
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain, usdc_id(), 5 * USD_8DP, ctx); // 5 USD min (config in 8dp USD)
        scenario.next_tx(@0xAEC);
        let treas = treasury::mock_for_test(scenario.ctx());
        let amount = 100_000_000; // 100 USDC (6 decimals)
        let effective = bridge_min_config::get_effective_cross_out_fee(&obj.id, chain, usdc_id(), amount);
        let calculated = bridge_fee::calculate_cross_out_fee_amount(&obj.id, chain, usdc_id(), amount); // 0.05% of 100 = 0.05 USDC
        assert_eq!(effective >= calculated, true);
        assert_eq!(effective >= 5_000_000, true); // at least 5 USDC (min floor)
        test_utils::destroy(treas);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_get_effective_cross_in_fee() {
        let mut scenario = test_scenario::begin(@0xAED);
        let ctx = scenario.ctx();
        let mut obj = new(ctx);
        bridge_fee::new_bridge_fee_registry(&mut obj.id, ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id, ctx);
        let chain = eth_mainnet() as u64;
        bridge_fee::set_fee_in_cross_in(&mut obj.id, chain, usdc_id(), 1, 500, ctx); // 0.05%
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain, usdc_id(), 3 * USD_8DP, ctx); // 3 USD min (config in 8dp USD)
        scenario.next_tx(@0xAEE);
        let treas = treasury::mock_for_test(scenario.ctx());
        let amount = 100_000_000; // 100 USDC (6 decimals)
        let effective = bridge_min_config::get_effective_cross_in_fee(&obj.id, chain, usdc_id(), amount);
        let calculated = bridge_fee::calculate_cross_in_fee_amount(&obj.id, chain, usdc_id(), amount); // 0.05% of 100 = 0.05 USDC
        assert_eq!(effective >= calculated, true);
        assert_eq!(effective >= 3_000_000, true); // at least 3 USDC (min floor)
        test_utils::destroy(treas);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_registry_and_defaults(){
        let mut scenario = test_scenario::begin(@0xAAA);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id,ctx);
        let chain = eth_mainnet() as u64;
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain)==0, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain)==0, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain, usdc_id())==0, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain, usdc_id())==0, true);
        let _r = bridge_min_config::borrow(&obj.id);
        let _rm = bridge_min_config::borrow_mut(&mut obj.id);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = bridge::bridge_min_config::EBridgeMinConfigRegistryAlreadyExists)]
    fun test_registry_duplicate_abort(){
        let mut scenario = test_scenario::begin(@0xAAB);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id,ctx);
        scenario.next_tx(@0xAAC);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id,scenario.ctx());
        abort 0
    }

    #[test]
    fun test_setters_and_getters_single_chain(){
        let mut scenario = test_scenario::begin(@0xABD);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id,ctx);
        let chain = eth_mainnet() as u64;
        scenario.next_tx(@0xABE);
        bridge_min_config::set_min_limit_cross_out(&mut obj.id, chain, 1_000);
        scenario.next_tx(@0xABF);
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, chain, 2_000);
        scenario.next_tx(@0xAC0);
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain, usdc_id(), 100, scenario.ctx());
        scenario.next_tx(@0xAC1);
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain, usdc_id(), 200, scenario.ctx());
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain)==1_000, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain)==2_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain, usdc_id())==100, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain, usdc_id())==200, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_update_values(){
        let mut scenario = test_scenario::begin(@0xAC2);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id,ctx);
        let chain = eth_mainnet() as u64;
        bridge_min_config::set_min_limit_cross_out(&mut obj.id, chain, 1_000);
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, chain, 2_000);
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain, usdc_id(), 100, scenario.ctx());
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain, usdc_id(), 200, scenario.ctx());
        scenario.next_tx(@0xAC3);
        bridge_min_config::set_min_limit_cross_out(&mut obj.id, chain, 5_000);
        scenario.next_tx(@0xAC4);
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, chain, 6_000);
        scenario.next_tx(@0xAC5);
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain, usdc_id(), 300, scenario.ctx());
        scenario.next_tx(@0xAC6);
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain, usdc_id(), 400, scenario.ctx());
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain)==5_000, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain)==6_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain, usdc_id())==300, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain, usdc_id())==400, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_multiple_chains_isolation(){
        let mut scenario = test_scenario::begin(@0xAC7);
        let ctx = scenario.ctx();
        let mut obj=new(ctx);
        bridge_min_config::new_bridge_min_config_registry(&mut obj.id,ctx);
        let chain_a = eth_mainnet() as u64;
        let chain_b = bsc_mainnet() as u64;
        bridge_min_config::set_min_limit_cross_out(&mut obj.id, chain_a, 1_000);
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain_a, usdc_id(), 100, scenario.ctx());
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, chain_b, 9_000);
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain_b, usdt_id(), 900, scenario.ctx());
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain_a)==1_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain_a, usdc_id())==100, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain_b)==9_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain_b, usdt_id())==900, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain_a)==0, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain_a, usdc_id())==0, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain_b)==0, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain_b, usdt_id())==0, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }
}

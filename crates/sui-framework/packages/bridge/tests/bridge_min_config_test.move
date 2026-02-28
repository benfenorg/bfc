#[test_only]
module bridge::bridge_min_config_tests {
    use sui::test_scenario;
    use bridge::bridge_min_config;
    use bridge::chain_ids::{eth_mainnet, bsc_mainnet};
    use std::unit_test::assert_eq;
    use sui::test_utils;

    public struct MinConfigObject has key,store {
        id: UID
    }

    public fun new(ctx: &mut TxContext) : MinConfigObject{
        MinConfigObject { id: object::new(ctx) }
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
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain)==0, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain)==0, true);
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
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain, 100);
        scenario.next_tx(@0xAC1);
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain, 200);
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain)==1_000, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain)==2_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain)==100, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain)==200, true);
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
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain, 100);
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain, 200);
        scenario.next_tx(@0xAC3);
        bridge_min_config::set_min_limit_cross_out(&mut obj.id, chain, 5_000);
        scenario.next_tx(@0xAC4);
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, chain, 6_000);
        scenario.next_tx(@0xAC5);
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain, 300);
        scenario.next_tx(@0xAC6);
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain, 400);
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain)==5_000, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain)==6_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain)==300, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain)==400, true);
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
        bridge_min_config::set_min_fee_cross_out(&mut obj.id, chain_a, 100);
        bridge_min_config::set_min_limit_cross_in(&mut obj.id, chain_b, 9_000);
        bridge_min_config::set_min_fee_cross_in(&mut obj.id, chain_b, 900);
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain_a)==1_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain_a)==100, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain_b)==9_000, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain_b)==900, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_in(&obj.id, chain_a)==0, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_in(&obj.id, chain_a)==0, true);
        assert_eq!(bridge_min_config::get_min_limit_cross_out(&obj.id, chain_b)==0, true);
        assert_eq!(bridge_min_config::get_min_fee_cross_out(&obj.id, chain_b)==0, true);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }
}

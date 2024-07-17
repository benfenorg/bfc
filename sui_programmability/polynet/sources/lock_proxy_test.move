#[test_only]
module polynet::lock_proxy_test {
    use std::ascii::string;
    use polynet::controller::{update_lock_proxy_manager_start_time, bind_proxy, unbind_proxy, bind_asset, unbind_asset};
    use polynet::config::{init_cc_config, CrossChainGlobalConfig, borrow_mut_lp_manager};
    use sui::clock;
    use polynet::bf_usdc::BF_USDC;
    use sui::test_scenario;
    use polynet::acl::{ Self};
    use polynet::lock_proxy::{convert_to_short_key, check_amount_result};

    #[test]
    fun test_init_lock_manager(){

        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            update_lock_proxy_manager_start_time(&mut ccConfig,&clock, ctx);
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(ccConfig);
        };

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_check_amount_result() {
        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;

        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let clock = clock::create_for_testing(ctx);
            update_lock_proxy_manager_start_time(&mut ccConfig,&clock, ctx);
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let lpmanager = borrow_mut_lp_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            let result = check_amount_result<BF_USDC>(10000000000000, lpmanager, &b"BF_USDC", false, &clock);
            assert!(result, 4018);
            let result = check_amount_result<BF_USDC>(10000000000000000000, lpmanager, &b"BF_USDC", false, &clock);
            assert!(result == false, 4018);
            test_scenario::return_shared(ccConfig);
            clock::destroy_for_testing(clock);
        };
        test_scenario::end(scenario_val);
    }


    #[test]
    fun test_bind_proxy() {
        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let clock = clock::create_for_testing(ctx);
            update_lock_proxy_manager_start_time(&mut ccConfig,&clock, ctx);
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            //let lpmanager = borrow_mut_lp_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let hash = x"0123";
            bind_proxy(&mut ccConfig, 10, hash, ctx);
            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            unbind_proxy(&mut ccConfig, 10,  ctx);
            test_scenario::return_shared(ccConfig);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_convert_to_short_key(){
        let input = string(b"0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0000000000000000000000000000000000000000000000000000000000000000::bfc_eth::BFC_ETH>");
        assert!(convert_to_short_key(&input) == b"BFC_ETH", 1);

        input = string(b"0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0000000000000000000000000000000000000000000000000000000000000000::bf_usdt::BF_USDT>");
        assert!(convert_to_short_key(&input) == b"BF_USDT", 1);

        input = string(b"0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0000000000000000000000000000000000000000000000000000000000000000::bf_usdc::BF_USDC>");
        assert!(convert_to_short_key(&input) == b"BF_USDC", 1);

        input = string(b"0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0000000000000000000000000000000000000000000000000000000000000000::bfc_usdc::BFC_BTC>");
        assert!(convert_to_short_key(&input) == b"BFC_BTC", 1);
    }

    #[test]
    fun test_asset(){
        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;
        assert!(acl::is_admin(owner), 4001);


        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);


            let clock = clock::create_for_testing(ctx);
            update_lock_proxy_manager_start_time(&mut ccConfig,&clock, ctx);
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(ccConfig);
        };


        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let hash = x"0123";
            let decimal = 6;
            bind_asset<BF_USDC>(&mut ccConfig, 10,hash, decimal, ctx);

            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            unbind_asset<BF_USDC>(&mut ccConfig, 10, ctx);

            test_scenario::return_shared(ccConfig);
        };
        test_scenario::end(scenario_val);
    }

}

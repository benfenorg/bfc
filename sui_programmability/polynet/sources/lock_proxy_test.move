
#[test_only]
#[allow(unused_use)]
module polynet::lock_proxy_test {

    use std::ascii::string;
    use polynet::controller::{update_lock_proxy_manager_start_time, bind_proxy, unbind_proxy, bind_asset, unbind_asset};
    use polynet::config::{init_cc_config, CrossChainGlobalConfig, borrow_mut_lp_manager};
    use sui::clock;
    use polynet::bfc_usdc::BFC_USDC;
    use sui::test_scenario;
    use polynet::utils;
    use polynet::lock_proxy::{ paused, LockProxyManager, unpause, pause, transferOwnerShip, convert_to_short_key, checkAmountResult
    };

    #[test]
    fun test_init_lock_manager(){

        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

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
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let lpmanager = borrow_mut_lp_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);



            pause(lpmanager, ctx);

            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let lpmanager = borrow_mut_lp_manager(&mut ccConfig);

            let ctx = test_scenario::ctx(&mut scenario_val);



            unpause(lpmanager, ctx);

            test_scenario::return_shared(ccConfig);
        };

        let new_owner = @0xfd8669e7e9ecb8d9b893dc6b0ad6727aa28c80dd1c5a34809d20910c5ffa7525;
        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let lpmanager = borrow_mut_lp_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);


            transferOwnerShip(lpmanager, new_owner,ctx);

            test_scenario::return_shared(ccConfig);
        };

        //change back to the original owner
        test_scenario::next_tx(&mut scenario_val, new_owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let lpmanager = borrow_mut_lp_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);


            transferOwnerShip(lpmanager, owner,ctx);

            test_scenario::return_shared(ccConfig);
        };


        test_scenario::end(scenario_val);

    }

    #[test]
    fun test_check_amount_result() {
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;

        assert!(utils::is_admin(owner), 4001);

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
        };


        test_scenario::next_tx(&mut scenario_val, owner);
        {


            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let lpmanager = borrow_mut_lp_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);


            let clock = clock::create_for_testing(ctx);


            let result = checkAmountResult(10000000000000, lpmanager, &b"BFC_USDT", false, &clock);
            assert!(result, 4018);

            let result = checkAmountResult(100000000000000, lpmanager, &b"BFC_USDT", false, &clock);
            assert!(result == false, 4018);

            test_scenario::return_shared(ccConfig);
            clock::destroy_for_testing(clock);
        };
        test_scenario::end(scenario_val);
    }


    #[test]
    fun test_bind_proxy() {
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

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
        };


        test_scenario::next_tx(&mut scenario_val, owner);
        {


            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            //let lpmanager = borrow_mut_lp_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let hash = x"0123";
            bind_proxy(&mut ccConfig, 10, hash, ctx);

            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
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

        input = string(b"0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0000000000000000000000000000000000000000000000000000000000000000::bfc_usdt::BFC_USDT>");
        assert!(convert_to_short_key(&input) == b"BFC_USDT", 1);

        input = string(b"0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0000000000000000000000000000000000000000000000000000000000000000::bfc_usdc::BFC_USDC>");
        assert!(convert_to_short_key(&input) == b"BFC_USDC", 1);

        input = string(b"0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0000000000000000000000000000000000000000000000000000000000000000::bfc_usdc::BFC_BTC>");
        assert!(convert_to_short_key(&input) == b"BFC_BTC", 1);
    }

    #[test]
    fun test_asset(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);


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
        };



        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let hash = x"0123";
            let decimal = 6;
            bind_asset<BFC_USDC>(&mut ccConfig, 10,hash, decimal, ctx);

            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);


            unbind_asset<BFC_USDC>(&mut ccConfig, 10,  ctx);

            test_scenario::return_shared(ccConfig);
        };


        test_scenario::end(scenario_val);
    }

}

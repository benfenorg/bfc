#[test_only]
module polynet::wrapper_v1_test {
    use std::debug::print;
    use std::vector;
    use polynet::controller::{update_lock_proxy_manager_start_time, bind_proxy, bind_asset};
    use polynet::config::{init_cc_config, CrossChainGlobalConfig, borrow_mut_wrapper_store, borrow_mut_all};
    use sui::bfc::BFC;
    use sui::clock;
    use sui::coin;
    use polynet::lock_proxy::{Treasury};
    use polynet::bf_usdc::{new_for_test, BF_USDC};
    use polynet::tools::{init_mainnet_ccm, init_as_testnet};
    use polynet::wrapper_v1::{fee_collector, set_fee_collector, lock_and_pay_fee_with_fund};
    use polynet::acl::{ Self};
    use sui::test_scenario;

    #[test]
    fun test_wrapper_init(){
        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);

        };

        //set fee collector
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let wStore = borrow_mut_wrapper_store(&mut ccConfig);
            set_fee_collector(wStore, owner, ctx);
            test_scenario::return_shared( ccConfig);
        };

        let new_fee_collecotr = @0x01;
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let wStore = borrow_mut_wrapper_store(&mut ccConfig);
            set_fee_collector(wStore, new_fee_collecotr, ctx);
            test_scenario::return_shared( ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let wStore = borrow_mut_wrapper_store(&mut ccConfig);
            let fee_collector =  fee_collector(wStore);
            assert!(fee_collector==new_fee_collecotr, 4002);
            test_scenario::return_shared(ccConfig);
        };

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_lock_and_pay_fee(){
        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;
        let contract = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899");
        vector::push_back(&mut keepers, x"09c6475ce07577ab72a1f96c263e5030cb53a843b00ca1238a093d9dcb183e2fec837e621b7ec6db7658c9b9808da304aed599043de1b433d490ff74f577c53d");
        vector::push_back(&mut keepers, x"e68a6e54bdfa0af47bd18465f4352f5151dc729c61a7399909f1cd1c6d816c0241800e782bb05f6f803b9f958930ebcee0b67d3af27845b4fbfa09e926cf17ae");
        vector::push_back(&mut keepers, x"29e0d1c5b2ae838930ae1ad861ddd3d0745d1c7f142492cabd02b291d2c95c1dda6633dc7be5dd4f9597f32f1e45721959d0902a8e56a58b2db79ada7c3ce932");

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
            new_for_test(ctx, owner);
        };

         test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);

            init_mainnet_ccm(&mut ccConfig,owner,&clock, ctx);
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(ccConfig);
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
            bind_proxy(&mut ccConfig, 10, hash, ctx);
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
            let clock = clock::create_for_testing(ctx);

            init_as_testnet(&mut ccConfig, &clock, contract, ctx);
            test_scenario::return_shared(ccConfig);
            clock::destroy_for_testing(clock);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let (lock_proxy, wrapper_store,manager) = borrow_mut_all(&mut ccConfig);
            let treasury =  test_scenario::take_shared<Treasury<BF_USDC>>(&mut scenario_val );

            print(&treasury);
            let coin =  coin::mint_for_testing<BF_USDC>(10000000000, test_scenario::ctx(&mut scenario_val));
            let fee =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));
            
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            let toAddress = x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899";
            lock_and_pay_fee_with_fund<BF_USDC>(manager, lock_proxy, &mut treasury, wrapper_store, @0x2, &mut coin, 7000000000, fee, 10, &toAddress, &clock, ctx);
            test_scenario::return_shared(ccConfig);
            test_scenario::return_shared( treasury);
            coin::burn_for_testing(coin);
            clock::destroy_for_testing(clock);
        };
        test_scenario::end(scenario_val);
    }
}

#[test_only]
module polynet::wrapper_v1_test {
    use std::debug::print;
    use std::vector;
    use sui::bfc::BFC;
    use sui::clock;
    use sui::coin;
    use polynet::bfc_eth::{new_for_test, BFC_ETH};
    use polynet::cross_chain_manager::CrossChainManager;
    use polynet::lock_proxy::{init_lock_proxy_manager, LockProxyManager, Treasury, bindProxy, bindAsset};
    use polynet::tools::{init_mainnet_ccm, init_as_mainnet};
    use polynet::wrapper_v1::{init_wrapper, feeCollector, WrapperStore, setFeeCollector, lock_and_pay_fee_with_fund};
    use polynet::utils;
    use sui::test_scenario;
    use sui::test_scenario::return_to_sender;

    #[test]
    fun test_wrapper_init(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_wrapper(ctx);

        };
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let wrapper_store = test_scenario::take_shared<WrapperStore>(&mut scenario_val );

            let fee_collector =  feeCollector(&mut wrapper_store);
            assert!(fee_collector==owner, 4002);

            test_scenario::return_shared( wrapper_store);
        };

        let new_fee_collecotr = @0x01;
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let wrapper_store = test_scenario::take_shared<WrapperStore>(&mut scenario_val );


            let ctx = test_scenario::ctx(&mut scenario_val);
            setFeeCollector(&mut wrapper_store, new_fee_collecotr,ctx);
            test_scenario::return_shared( wrapper_store);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let wrapper_store = test_scenario::take_shared<WrapperStore>(&mut scenario_val );

            let fee_collector =  feeCollector(&mut wrapper_store);
            assert!(fee_collector==new_fee_collecotr, 4002);

            test_scenario::return_shared( wrapper_store);
        };

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_lock_and_pay_fee(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899");
        vector::push_back(&mut keepers, x"09c6475ce07577ab72a1f96c263e5030cb53a843b00ca1238a093d9dcb183e2fec837e621b7ec6db7658c9b9808da304aed599043de1b433d490ff74f577c53d");
        vector::push_back(&mut keepers, x"e68a6e54bdfa0af47bd18465f4352f5151dc729c61a7399909f1cd1c6d816c0241800e782bb05f6f803b9f958930ebcee0b67d3af27845b4fbfa09e926cf17ae");
        vector::push_back(&mut keepers, x"29e0d1c5b2ae838930ae1ad861ddd3d0745d1c7f142492cabd02b291d2c95c1dda6633dc7be5dd4f9597f32f1e45721959d0902a8e56a58b2db79ada7c3ce932");


        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_mainnet_ccm(ctx);

            let clock = clock::create_for_testing(ctx);
            init_lock_proxy_manager(&clock, ctx);
            init_wrapper(ctx);
            new_for_test(ctx, owner);
            clock::destroy_for_testing(clock);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let hash = x"0123";
            bindProxy(&mut manager, 10, hash, ctx);

            test_scenario::return_shared(manager);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let hash = x"0123";
            let decimal = 6;
            bindAsset<BFC_ETH>(&mut manager, 10,hash, decimal, ctx);

            test_scenario::return_shared(manager);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let manager = test_scenario::take_shared<CrossChainManager>(&mut scenario_val);
            let lock_proxy = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let wrapper_store = test_scenario::take_shared<WrapperStore>(&mut scenario_val );
            let treasury =  test_scenario::take_from_sender<Treasury<BFC_ETH>>(&mut scenario_val );
           // let fund  = test_scenario::take_from_sender<Coin<BFC_BTC>>(&mut scenario_val );
            print(&treasury);
            let coin =  coin::mint_for_testing<BFC_ETH>(10000000000, test_scenario::ctx(&mut scenario_val));
            let fee =  coin::mint_for_testing<BFC>(10000000000, test_scenario::ctx(&mut scenario_val));

            let ctx = test_scenario::ctx(&mut scenario_val);

            let clock = clock::create_for_testing(ctx);
            init_as_mainnet(&mut manager, &mut lock_proxy, &clock, ctx);


            let toAddress = x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899";
            lock_and_pay_fee_with_fund<BFC_ETH>(&mut manager, &mut lock_proxy, &mut treasury, &mut wrapper_store, @0x2, coin, fee, 10, &toAddress, ctx);
            test_scenario::return_shared(manager);
            test_scenario::return_shared(lock_proxy);
            test_scenario::return_shared(wrapper_store);
            test_scenario::return_to_sender(&mut scenario_val, treasury);
            clock::destroy_for_testing(clock);
        };
        test_scenario::end(scenario_val);
    }
}

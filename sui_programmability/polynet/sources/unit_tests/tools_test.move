#[test_only]
module polynet::tools_test {
    use std::debug::print;
    use std::vector;
    use sui::clock;
    use polynet::lock_proxy::{ get_license_id};
    use polynet::tools::{init_testnet_ccm, init_mainnet_ccm, issue_license_to_lock_proxy, init_as_testnet,
        init_as_mainnet
    };
    use polynet::config::{init_cc_config, CrossChainGlobalConfig, Self};
    use polynet::cross_chain_manager::{get_poly_id,set_poly_id};
    use polynet::acl::{ Self};
    use sui::test_scenario;

    #[test]
    fun test_init_testnet_ccm(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(acl::is_admin(owner), 4001);

       

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
        };

        //  let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            init_testnet_ccm(&mut ccConfig,owner,&clock, ctx);
            clock::destroy_for_testing(clock);
            test_scenario::return_shared(ccConfig);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let global = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let manager = config::borrow_crosschain_manager(&mut global);
            let result = get_poly_id(manager);
            assert!(result == 1200, 4002);

            test_scenario::return_shared(global);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_init_mainnet_ccm(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);

        };

        // let scenario_val = test_scenario::begin(owner);
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
            let global = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let manager = config::borrow_crosschain_manager(&mut global);
            let result = get_poly_id(manager);
            assert!(result == 41, 4002);

            test_scenario::return_shared(global);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_issue_license_to_lock_proxy() {
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);

        };

        // let scenario_val = test_scenario::begin(owner);

        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899");
        vector::push_back(&mut keepers, x"09c6475ce07577ab72a1f96c263e5030cb53a843b00ca1238a093d9dcb183e2fec837e621b7ec6db7658c9b9808da304aed599043de1b433d490ff74f577c53d");
        vector::push_back(&mut keepers, x"e68a6e54bdfa0af47bd18465f4352f5151dc729c61a7399909f1cd1c6d816c0241800e782bb05f6f803b9f958930ebcee0b67d3af27845b4fbfa09e926cf17ae");
        vector::push_back(&mut keepers, x"29e0d1c5b2ae838930ae1ad861ddd3d0745d1c7f142492cabd02b291d2c95c1dda6633dc7be5dd4f9597f32f1e45721959d0902a8e56a58b2db79ada7c3ce932");


        let new_polyId: u64 = 42;
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let global = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            issue_license_to_lock_proxy(&mut global,owner);
            
            let (lock_proxy,manager) = config::borrow_mut_lp_and_cc_managers(&mut global);
            let ctx = test_scenario::ctx(&mut scenario_val);
            set_poly_id(manager, new_polyId, ctx);

            let (licenseInfoBytes, licenseInfo) = get_license_id(lock_proxy);
            print(&licenseInfo);
            assert!(vector::length(&licenseInfoBytes) != 0, 0);
            test_scenario::return_shared(global);
        };

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_init_as_testnet() {
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);

        };

        // let scenario_val = test_scenario::begin(owner);
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899");
        vector::push_back(&mut keepers, x"09c6475ce07577ab72a1f96c263e5030cb53a843b00ca1238a093d9dcb183e2fec837e621b7ec6db7658c9b9808da304aed599043de1b433d490ff74f577c53d");
        vector::push_back(&mut keepers, x"e68a6e54bdfa0af47bd18465f4352f5151dc729c61a7399909f1cd1c6d816c0241800e782bb05f6f803b9f958930ebcee0b67d3af27845b4fbfa09e926cf17ae");
        vector::push_back(&mut keepers, x"29e0d1c5b2ae838930ae1ad861ddd3d0745d1c7f142492cabd02b291d2c95c1dda6633dc7be5dd4f9597f32f1e45721959d0902a8e56a58b2db79ada7c3ce932");



        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            init_testnet_ccm(&mut ccConfig,owner,&clock, ctx);
            test_scenario::return_shared(ccConfig);
            clock::destroy_for_testing(clock);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let global = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
           
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            init_as_testnet(&mut global, &clock, owner , ctx);
             
            let (lock_proxy,manager) = config::borrow_mut_lp_and_cc_managers(&mut global);

            let (licenseInfoBytes, licenseInfo) = get_license_id(lock_proxy);
            print(&licenseInfo);
            assert!(vector::length(&licenseInfoBytes) != 0, 0);

            let result = get_poly_id(manager);
            print(&result);
            assert!(result == 1200, 4002);

            test_scenario::return_shared(global);
            clock::destroy_for_testing(clock);
        };

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_init_as_mainnet() {
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);

        };

        // let scenario_val = test_scenario::begin(owner);
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899");
        vector::push_back(&mut keepers, x"09c6475ce07577ab72a1f96c263e5030cb53a843b00ca1238a093d9dcb183e2fec837e621b7ec6db7658c9b9808da304aed599043de1b433d490ff74f577c53d");
        vector::push_back(&mut keepers, x"e68a6e54bdfa0af47bd18465f4352f5151dc729c61a7399909f1cd1c6d816c0241800e782bb05f6f803b9f958930ebcee0b67d3af27845b4fbfa09e926cf17ae");
        vector::push_back(&mut keepers, x"29e0d1c5b2ae838930ae1ad861ddd3d0745d1c7f142492cabd02b291d2c95c1dda6633dc7be5dd4f9597f32f1e45721959d0902a8e56a58b2db79ada7c3ce932");


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
            let global = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);

            let ctx = test_scenario::ctx(&mut scenario_val);

            let clock = clock::create_for_testing(ctx);
            init_as_mainnet(&mut global,&clock, owner, ctx);
            let (lock_proxy,manager) = config::borrow_mut_lp_and_cc_managers(&mut global);

            let (licenseInfoBytes, licenseInfo) = get_license_id(lock_proxy);
            print(&licenseInfo);
            assert!(vector::length(&licenseInfoBytes) != 0, 0);

            let result = get_poly_id(manager);
            print(&result);
            assert!(result == 41, 4002);

            test_scenario::return_shared(global);
            clock::destroy_for_testing(clock);
        };

        test_scenario::end(scenario_val);
    }
}

#[test_only]
#[allow(unused_use)]
module polynet::cross_chain_manager_test {

    use std::vector;
    use polynet::controller::{change_poly_id, grant_role, revoke_role, set_blacklist};
    use polynet::config::{init_cc_config, CrossChainGlobalConfig, borrow_mut_crosschain_manager, Self};
    use polynet::cross_chain_manager::{CrossChainManager, get_poly_id,
        update_cross_chain_manager_config, verify_header_and_execute_tx
    };
    use sui::test_scenario;
    use polynet::cross_chain_manager;
    use polynet::utils;
    use polynet::lock_proxy;
    use polynet::acl::{ Self};

    #[test]
    fun test_cross_chain_manager() {
        // sender address
        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);

        let polyId: u64 = 41;
        let startHeight: u64 = 0;
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899");
        vector::push_back(&mut keepers, x"09c6475ce07577ab72a1f96c263e5030cb53a843b00ca1238a093d9dcb183e2fec837e621b7ec6db7658c9b9808da304aed599043de1b433d490ff74f577c53d");
        vector::push_back(&mut keepers, x"e68a6e54bdfa0af47bd18465f4352f5151dc729c61a7399909f1cd1c6d816c0241800e782bb05f6f803b9f958930ebcee0b67d3af27845b4fbfa09e926cf17ae");
        vector::push_back(&mut keepers, x"29e0d1c5b2ae838930ae1ad861ddd3d0745d1c7f142492cabd02b291d2c95c1dda6633dc7be5dd4f9597f32f1e45721959d0902a8e56a58b2db79ada7c3ce932");


        //init cc config
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
        };

        //update cross chain config
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let manager = borrow_mut_crosschain_manager(&mut ccConfig);
            let ctx = test_scenario::ctx(&mut scenario_val);


            update_cross_chain_manager_config( manager, keepers, startHeight, polyId,  ctx);

            test_scenario::return_shared(ccConfig);

        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);


            change_poly_id(&mut  ccConfig,  polyId,  ctx);

            test_scenario::return_shared(ccConfig);
        };

        //get poly id
        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let manager = borrow_mut_crosschain_manager(&mut ccConfig);

            let result = get_poly_id(manager);

            assert!(result == polyId, 4002);
            test_scenario::return_shared(ccConfig);

        };

        //grant role
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            //let manager = borrow_mut_crosschain_manager(&mut ccConfig);

            let new_role_address = @0x01;
            grant_role(&mut  ccConfig, 1, new_role_address, ctx);


            test_scenario::return_shared(ccConfig);
        };


        //revoke role
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            //let manager = borrow_mut_crosschain_manager(&mut ccConfig);

            let new_role_address = @0x01;
            revoke_role(&mut ccConfig, 1, new_role_address, ctx);


            test_scenario::return_shared(ccConfig);
        };



        //set black list
        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            //let manager = borrow_mut_crosschain_manager(&mut ccConfig);

            // let new_role_address = @0x01;
            set_blacklist(&mut  ccConfig, x"01", 1, ctx);

            test_scenario::return_shared(ccConfig);
        };

        test_scenario::end(scenario_val);
    }
}

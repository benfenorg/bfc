#[test_only]
#[allow(unused_use)]
module polynet::cross_chain_manager_test {

    use std::vector;
    use polynet::cross_chain_manager::{setPolyId, CrossChainManager, getPolyId};
    use sui::test_scenario;
    use polynet::cross_chain_manager;
    use polynet::utils;

    #[test]
    fun test_cross_chain_manager() {
        // sender address
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);

        let polyId: u64 = 41;
        let startHeight: u64 = 0;
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"2bed55e8c4d9cbc50657ff5909ee51dc394a92aad911c36bace83c4d63540794bc68a65f1a54ec4f14a630043090bc29ee9cddf90f3ecb86e0973ffff3fd4899");
        vector::push_back(&mut keepers, x"09c6475ce07577ab72a1f96c263e5030cb53a843b00ca1238a093d9dcb183e2fec837e621b7ec6db7658c9b9808da304aed599043de1b433d490ff74f577c53d");
        vector::push_back(&mut keepers, x"e68a6e54bdfa0af47bd18465f4352f5151dc729c61a7399909f1cd1c6d816c0241800e782bb05f6f803b9f958930ebcee0b67d3af27845b4fbfa09e926cf17ae");
        vector::push_back(&mut keepers, x"29e0d1c5b2ae838930ae1ad861ddd3d0745d1c7f142492cabd02b291d2c95c1dda6633dc7be5dd4f9597f32f1e45721959d0902a8e56a58b2db79ada7c3ce932");


        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            cross_chain_manager::init_crosschain_manager(keepers, startHeight, polyId, ctx);
        };

        let new_polyId: u64 = 42;
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<CrossChainManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            setPolyId(&mut manager, new_polyId, ctx);

            test_scenario::return_shared(manager);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            //let ctx = test_scenario::ctx(&mut scenario_val);
            let manager = test_scenario::take_shared<CrossChainManager>(&mut scenario_val);
            let result = getPolyId(&mut manager);
            assert!(result == new_polyId, 4002);

            test_scenario::return_shared(manager);
        };



        test_scenario::end(scenario_val);
    }
}

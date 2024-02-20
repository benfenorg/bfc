#[test_only]
module polynet::tools_test {
    use polynet::tools::{init_testnet_ccm, init_mainnet_ccm};
    use polynet::cross_chain_manager::{CrossChainManager, getPolyId};
    use polynet::utils;
    use sui::test_scenario;

    #[test]
    fun test_init_testnet_ccm(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_testnet_ccm(ctx);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<CrossChainManager>(&mut scenario_val);
            let result = getPolyId(&mut manager);
            assert!(result == 998, 4002);

            test_scenario::return_shared(manager);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_init_mainnet_ccm(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_mainnet_ccm(ctx);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<CrossChainManager>(&mut scenario_val);
            let result = getPolyId(&mut manager);
            assert!(result == 41, 4002);

            test_scenario::return_shared(manager);
        };
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_init_as_mainnet() {

    }
}

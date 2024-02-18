
#[test_only]
module polynet::lock_proxy_test {

    use sui::test_scenario;
    use polynet::utils;
    use polynet::lock_proxy::init_lock_proxy_manager;

    #[test]
    fun test_init_lock_manager(){

        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_lock_proxy_manager(ctx);
        };
        test_scenario::end(scenario_val);

    }
}


#[test_only]
#[allow(unused_use)]
module polynet::lock_proxy_test {

    use polynet::bfc_usdc::BFC_USDC;
    use sui::test_scenario;
    use polynet::utils;
    use polynet::lock_proxy::{init_lock_proxy_manager, paused, LockProxyManager, unpause, pause, transferOwnerShip,
        bindProxy, unbindProxy, bindAsset, unbindAsset
    };

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

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            pause(&mut manager, ctx);

            test_scenario::return_shared(manager);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            unpause(&mut manager, ctx);

            test_scenario::return_shared(manager);
        };

        let new_owner = @0xfd8669e7e9ecb8d9b893dc6b0ad6727aa28c80dd1c5a34809d20910c5ffa7525;
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
              transferOwnerShip(&mut manager, new_owner,ctx);

            test_scenario::return_shared(manager);
        };

        //change back to the original owner
        test_scenario::next_tx(&mut scenario_val, new_owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            transferOwnerShip(&mut manager, owner,ctx);

            test_scenario::return_shared(manager);
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
            init_lock_proxy_manager(ctx);
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

            unbindProxy(&mut manager, 10,  ctx);

            test_scenario::return_shared(manager);
        };


        test_scenario::end(scenario_val);
    }


    #[test]
    fun test_asset(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(utils::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_lock_proxy_manager(ctx);
        };
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let hash = x"0123";
            let decimal = 6;
            bindAsset<BFC_USDC>(&mut manager, 10,hash, decimal, ctx);

            test_scenario::return_shared(manager);
        };

        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let manager = test_scenario::take_shared<LockProxyManager>(&mut scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            unbindAsset<BFC_USDC>(&mut manager, 10,  ctx);

            test_scenario::return_shared(manager);
        };


        test_scenario::end(scenario_val);
    }

}

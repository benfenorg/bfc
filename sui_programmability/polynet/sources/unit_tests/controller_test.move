#[test_only]
module polynet::controller_test {
    use std::vector;
    use polynet::controller::{update_fee_collector, update_fee_config,change_Book_keeper,
    change_start_height,update_crosschain_config,get_asset,output_license_id,pause_global,
    unpause_global,update_lock_min_amount,update_unlock_min_amount,reset_lock_amount,
    reset_unlock_amount,mint_treasury,deposit_treasury, bind_proxy, bind_asset};
    use polynet::config::{init_cc_config, CrossChainGlobalConfig,borrow_mut_wrapper_store,
    borrow_wrapper_store};
    use sui::clock;
    use polynet::wrapper_v1::{fee_collector,need_fee};
    use polynet::bfc_usdc::{ BFC_USDC, new_for_test};
    use polynet::bfc_usdt::{ BFC_USDT};
    use polynet::tools::{ init_as_testnet};
    use polynet::acl::{ Self};
    use sui::test_scenario;

   

    #[test]
    fun test_unlock_tx(){
        let owner = @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590;
        assert!(acl::is_admin(owner), 4001);

        let scenario_val = test_scenario::begin(owner);
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ctx = test_scenario::ctx(&mut scenario_val);
            init_cc_config(ctx);
            new_for_test(ctx, owner);
        };

        // init and issue license 
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            // let global_config_id = object::id(&ccConfig);

            let config_address = @0xf441b258a5a3f1d5a6284682ae670f4ea569b7f0f8b49458d8e991e7ce554333;

            init_as_testnet(&mut ccConfig, &clock,config_address , ctx);
            test_scenario::return_shared(ccConfig);
            clock::destroy_for_testing(clock);
        };

         // update_fee_collector
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
          

            let new_fee_collector = @0x01;

            update_fee_collector(&mut ccConfig, new_fee_collector , ctx);
           
            let wStore = borrow_mut_wrapper_store(&mut ccConfig);
            let fee_collector =  fee_collector(wStore);
            assert!(fee_collector==new_fee_collector, 8002);
            test_scenario::return_shared(ccConfig);
        };

         // update_fee_config
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
          

            let update_need_fee = true;

            update_fee_config(&mut ccConfig, update_need_fee , ctx);
           
            let wStore = borrow_wrapper_store(&mut ccConfig);
            let need =  need_fee(wStore);
            assert!(need==update_need_fee, 8003);
            test_scenario::return_shared(ccConfig);
        };

         // change_Book_keeper
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
          
            let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
            vector::push_back(&mut keepers, x"26f22a620ab00e3c5832a12d6e91406bc67ea7b1e9582e800abd921c371074daa6dae5ba6aa9737460758fd17a590e79097ef519421894c7492ffded22983684");
            vector::push_back(&mut keepers, x"b73a7c698594c7e1e1e57746bedc99693130e801500996af39a62ea929d0797dda10be75ede8791bc97e311c26a7028d035c0fd55e61fe8a13836ef892861159");
            vector::push_back(&mut keepers, x"e3b9e57f97515aa8818b071637f5b42c8c24f864cb6826297f4e0ad78bbf1802fc18054796af0e2395ac41f36f43514fdca42c22b6e4cc1d1b22b07d0beceb44");
            vector::push_back(&mut keepers, x"4e552e00b6a7457d6b79298b449922de987561fe02d420398c862f1447e9231f39346373619d6dbdb830a00e0e0d35e0116c74129d0dfa5d8184c2eb5a6dcfbe");
            let start_height = 1000;

            change_Book_keeper(&mut ccConfig, keepers, start_height, ctx);
           
            test_scenario::return_shared(ccConfig);
        };

         // change_start_height
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
          
            let start_height = 2000;

            change_start_height(&mut ccConfig, start_height, ctx);
           
            test_scenario::return_shared(ccConfig);
        };

          // update_crosschain_config
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
          
            let start_height = 3000;
            let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
            vector::push_back(&mut keepers, x"26f22a620ab00e3c5832a12d6e91406bc67ea7b1e9582e800abd921c371074daa6dae5ba6aa9737460758fd17a590e79097ef519421894c7492ffded22983684");
            vector::push_back(&mut keepers, x"b73a7c698594c7e1e1e57746bedc99693130e801500996af39a62ea929d0797dda10be75ede8791bc97e311c26a7028d035c0fd55e61fe8a13836ef892861159");
            vector::push_back(&mut keepers, x"e3b9e57f97515aa8818b071637f5b42c8c24f864cb6826297f4e0ad78bbf1802fc18054796af0e2395ac41f36f43514fdca42c22b6e4cc1d1b22b07d0beceb44");
            vector::push_back(&mut keepers, x"4e552e00b6a7457d6b79298b449922de987561fe02d420398c862f1447e9231f39346373619d6dbdb830a00e0e0d35e0116c74129d0dfa5d8184c2eb5a6dcfbe");
            let poly_id = 1300;

            update_crosschain_config(&mut ccConfig, keepers, start_height, poly_id, ctx);
           
            test_scenario::return_shared(ccConfig);
        };



        // output_license_id
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            output_license_id(&mut ccConfig); 
            test_scenario::return_shared(ccConfig);
        };

        // get_asset
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let proxy_hash = x"8b94AE7F1f59F6F8B20Bf71944D5cB1f1c7439c4";
            bind_proxy(&mut ccConfig, 602, proxy_hash, ctx);

            let hash = x"b646D14eFA51D516cBc8DF5d1D8aB5Fd6DAD9ddB";
            let decimal = 18;
            bind_asset<BFC_USDC>(&mut ccConfig, 602,hash, decimal, ctx);

            get_asset<BFC_USDC>(&mut ccConfig, 602, ctx);
            test_scenario::return_shared(ccConfig);
        };

        
        test_scenario::end(scenario_val);
    }
}

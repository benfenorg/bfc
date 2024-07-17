#[test_only]
module polynet::unlock_test {
    use std::vector;
    use polynet::controller::{bind_proxy, bind_asset,test_relay_unlock_tx};
    use polynet::config::{init_cc_config, CrossChainGlobalConfig};
    use sui::clock;
    use polynet::lock_proxy::{Treasury};
    use polynet::bf_usdc::{ BF_USDC, new_for_test};
    use polynet::bf_usdt::{ BF_USDT};
    use polynet::tools::{ init_as_testnet};
    use polynet::acl::{ Self};
    use sui::test_scenario;

   

    #[test]
    fun test_unlock_tx(){
        let owner = @0xfbc4e44802b47459c0dbb03d123b2561ae86b8a559848d185ccd4fca6116e346;
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

        // bind_proxy
        test_scenario::next_tx(&mut scenario_val, owner);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);
            let hash = x"8b94AE7F1f59F6F8B20Bf71944D5cB1f1c7439c4";
            bind_proxy(&mut ccConfig, 602, hash, ctx);
            test_scenario::return_shared(ccConfig);
        };

        // bind_asset usdc
        test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let hash = x"b646D14eFA51D516cBc8DF5d1D8aB5Fd6DAD9ddB";
            let decimal = 18;
            bind_asset<BF_USDC>(&mut ccConfig, 602,hash, decimal, ctx);
            test_scenario::return_shared(ccConfig);
        };

         // bind_asset usdt
         test_scenario::next_tx(&mut scenario_val, owner);
        {

            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let ctx = test_scenario::ctx(&mut scenario_val);

            let hash = x"9f3a4eb9871181e5fcb4cf1016e20fac8536f766";
            let decimal = 6;
            bind_asset<BF_USDT>(&mut ccConfig, 602,hash, decimal, ctx);
            test_scenario::return_shared(ccConfig);
        };

       
        //relay_unlock_tx
        test_scenario::next_tx(&mut scenario_val, @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590);
        {
            let ccConfig = test_scenario::take_shared<CrossChainGlobalConfig>(&scenario_val);
            let treasury =  test_scenario::take_shared<Treasury<BF_USDC>>(&mut scenario_val );
            let ctx = test_scenario::ctx(&mut scenario_val);
            let clock = clock::create_for_testing(ctx);
            let proof = x"fda1012096c990f8ba227a52dde8c6d6d36a16585bb93453eb5a3e14bb84fe63433b82d95a0200000000000020000000000000000000000000000000000000000000000000000000000000008620418a9310916a2ba4bcfa687bd22782b49143bda0e535c5af338c10cdb6b62919148b94ae7f1f59f6f8b20bf71944d5cb1f1c7439c4b0040000000000002c20f441b258a5a3f1d5a6284682ae670f4ea569b7f0f8b49458d8e991e7ce5543330a6c6f636b5f70726f787906756e6c6f636be4a2303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030323a3a636f696e3a3a436f696e3c313561343735356232353633306638626630616338306437306333653464366662643634386137373338376565623963373137663733656365653136373563373a3a6266635f757364633a3a4246435f555344433e20520eef03f83d2fd08316762b60601a88cdbc956912f3ee22f804c521bc352c8f000010632d5ec76b050000000000000000000000000000000000000000000000";
            let raw_header = x"00000000ffffffffffffff7fd21214c41cbd5f677dcd61bb182559d7b55f84491cd84d421231e4390b99f8370000000000000000000000000000000000000000000000000000000000000000d73ae4e6efacff0905ca82300f3a91585fc7466947c9e91edb53a23d463edc2073e544c50f327aa315a814253d3cad67ff9f039a14b28fcbc5ea687822baf9eb7ceffb659bf58f0321a53356dd9a87e0fd14017b226c6561646572223a31312c227672665f76616c7565223a2242454b59424547765a49484f324f6b72455465355046395a63426c4674685456564d66566d4435754a774c666230374a734a383136564431385451437179424751325649764c7441327734365857616b2b7371686d4e773d222c227672665f70726f6f66223a226a4958504b6d536c667367757751414a2f35745758652f57735a53506c676d4c6950776d59453165674b427a585a7143706b48654730596d79645746317057703043556973523459765771673752565a6831477a52673d3d222c226c6173745f636f6e6669675f626c6f636b5f6e756d223a35393733393233332c226e65775f636861696e5f636f6e666967223a6e756c6c7d0000000000000000000000000000000000000000";
            let header_proof = vector::empty<u8>();
            let cur_raw_header = x"00";
            let header_sig = x"8b4c9fb32b017f2aa312592e75924462eb98354eb69fcd9b8c965514b7027f4044ef46da8f61e989b1031201df874fef563c8a764df0e4c2f5e86ea6e0be196f0124ee04d0a2cf683e3e5e57689046a5c05f4694b22bab200e13c88e8697114dc43496f8d39c66638b632849d76b1355305a1a6ca192197b6563b1e42338364f0200813e2bd830990b30232bb9ed785b341fc778ae55a0f9ce582b26c9ff00e0c50f13d5a1d53558c9e854dd973f9a5390c97a3e1606a602c0ec8e4bea5c027b273201";
            test_relay_unlock_tx<BF_USDC>(
                                &mut ccConfig, 
                                &mut treasury,
                                proof,
                                raw_header,
                                header_proof,
                                cur_raw_header,
                                header_sig,
                                &clock, 
                                ctx
                            );
            test_scenario::return_shared(ccConfig);
            test_scenario::return_shared( treasury);
            clock::destroy_for_testing(clock);
        };

        test_scenario::end(scenario_val);
    }
}

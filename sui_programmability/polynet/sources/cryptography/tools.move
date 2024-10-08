module polynet::tools {
    use polynet::cross_chain_manager;
    use polynet::lock_proxy;
    use std::vector;
    use polynet::controller::{update_lock_proxy_manager_start_time};
    use polynet::wrapper_v1::set_fee_collector;
    use polynet::config::{CrossChainGlobalConfig, borrow_mut_all};
    use sui::clock::Clock;
    use polynet::config;
    use sui::tx_context;
    use sui::tx_context::TxContext;


    const EINVALID_ADMIN: u64 = 2006;

    // mainnet
    public  fun init_as_mainnet(
        _config: &mut CrossChainGlobalConfig,
        _clock: &Clock,
        _contract: address,
        _ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_admin_role(_config, sender);

        init_mainnet_ccm(_config,sender,_clock, _ctx);
        issue_license_to_lock_proxy(_config, _contract);
    }

    public entry fun init_mainnet_ccm(
        _global: &mut CrossChainGlobalConfig,
        _feeAddress: address,
        _clock: &Clock,
        _ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_admin_role(_global, sender);
        let (_, wrapper, cc_manager) = config::borrow_mut_all(_global);

        let polyId: u64 = 48;
        let startHeight: u64 = 0;
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();

        vector::push_back(&mut keepers, x"288bdebfab545852b31638298c7100a9c26ad325f246b0939c661b9b112722c188f1611de3c1c4ed0c68bedaa0c2e6f771e306ad397c8fa571d44b2856fbaece");
        vector::push_back(&mut keepers, x"09d013d6adf0e56e439bde646ea9eff68467fa0a03a16d227d79f98c5a6725f64011909c67b64362ceaaf7032d177b453c0bf57a8ed8b07c17b00dc6525ec751");
        vector::push_back(&mut keepers, x"cf256247dda995ab1ec68ef90865b5d78f6d805339b40e736e913493bb25446d159039a2a07c9b4ac01a9d1201defbd444226afe45a122265b14bd0c91b83c0f");
        vector::push_back(&mut keepers, x"ecb8c0737073522c6f80dd8c7f1d8f0ec19c320010a27f9568bcdebefe8196d4898a17e9aeba611bfce20dc542c1152cea665fd426d3cbb5048bd9bbca23b1b3");
        cross_chain_manager::update_cross_chain_manager_config(cc_manager, keepers, startHeight, polyId, _ctx);

        set_fee_collector(wrapper, _feeAddress, _ctx);
        update_lock_proxy_manager_start_time(_global, _clock, _ctx);

    }

    public fun issue_license_to_lock_proxy(
        _config: &mut CrossChainGlobalConfig,
        _contract: address
    ) {
        let (lpManager,_, _) = borrow_mut_all(_config);
        let license = cross_chain_manager::issue_license(b"lock_proxy", _contract);
        lock_proxy::receive_license(lpManager,license);
    }

    // testnet
    public  fun init_as_testnet(
        _config: &mut CrossChainGlobalConfig,
        _clock: &Clock,
        _contract: address,
        _ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_admin_role(_config, sender);

        init_testnet_ccm(_config,sender,_clock, _ctx);
        issue_license_to_lock_proxy(_config, _contract);
    }

    public entry fun init_testnet_ccm(
        _global: &mut CrossChainGlobalConfig,
        _fee_address: address,
        _clock: &Clock,
        _ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_admin_role(_global, sender);

        let (_, wrapper, cc_manager) = config::borrow_mut_all(_global);
        let polyId: u64 = 1200;
        let startHeight: u64 = 0;
        let keepers: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut keepers, x"26f22a620ab00e3c5832a12d6e91406bc67ea7b1e9582e800abd921c371074daa6dae5ba6aa9737460758fd17a590e79097ef519421894c7492ffded22983684");
        vector::push_back(&mut keepers, x"b73a7c698594c7e1e1e57746bedc99693130e801500996af39a62ea929d0797dda10be75ede8791bc97e311c26a7028d035c0fd55e61fe8a13836ef892861159");
        vector::push_back(&mut keepers, x"e3b9e57f97515aa8818b071637f5b42c8c24f864cb6826297f4e0ad78bbf1802fc18054796af0e2395ac41f36f43514fdca42c22b6e4cc1d1b22b07d0beceb44");
        vector::push_back(&mut keepers, x"4e552e00b6a7457d6b79298b449922de987561fe02d420398c862f1447e9231f39346373619d6dbdb830a00e0e0d35e0116c74129d0dfa5d8184c2eb5a6dcfbe");
        cross_chain_manager::update_cross_chain_manager_config(cc_manager,keepers, startHeight, polyId, _ctx);

        set_fee_collector(wrapper, _fee_address, _ctx);
        update_lock_proxy_manager_start_time(_global, _clock, _ctx);
        // issue_license_to_lock_proxy(_global, _ctx);
    }
}
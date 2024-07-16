module polynet::controller {
    use sui::tx_context::{TxContext, Self};
    use sui::clock::Clock;
    use sui::coin::{TreasuryCap, Coin, Self};
    use sui::bfc::BFC;
    use polynet::bf_usdc;
    use polynet::config::{CrossChainGlobalConfig, Self};
    use polynet::wrapper_v1::{Self};
    use polynet::cross_chain_manager::{Self};
    use polynet::lock_proxy::{Treasury, Self};

    const ERR_VERSION_CHECK: u64 = 5000; 
    const EDEPOSIT_TREASURY_AMOUNT: u64 = 5001;

     // update crosschain_manager
    public entry fun migrate(  
        _global: &mut CrossChainGlobalConfig,
        _ctx: &mut TxContext
    )  {
        let sender = tx_context::sender(_ctx);
        config::check_admin_role(_global, sender);
        config::migrate(_global,_ctx);
    }

    public entry fun update_fee_collector(
        _global: &mut CrossChainGlobalConfig, 
        _new_fee_collector: address, 
        _ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        config::check_version(_global);

        wrapper_v1::set_fee_collector(
                        config::borrow_mut_wrapper_store(_global),
                        _new_fee_collector,
                        _ctx
                    ); 
    }
    // change lock need fee in eth or not
    public entry fun update_fee_config(
        _global: &mut CrossChainGlobalConfig, 
        _need_fee: bool, 
        _ctx: &mut TxContext
    ) {
     
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        config::check_version(_global);

        wrapper_v1::update_fee_config(
                        config::borrow_mut_wrapper_store(_global),
                        _need_fee
                    ); 
    }

      // change book keeper
    public entry fun change_Book_keeper(
        _global: &mut CrossChainGlobalConfig, 
        _keepers: vector<vector<u8>>, 
        _start_height: u64, 
        _ctx: &mut TxContext
    )  {

        config::check_version(_global);
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);

        let ccManager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::change_book_keeper(
                                ccManager,
                                _keepers,
                                _start_height,
                                _ctx
                            );
    }

       // change book keeper
    public entry fun change_start_height(
        _global: &mut CrossChainGlobalConfig, 
        _start_height: u64, 
        _ctx: &mut TxContext
    )  {

        config::check_version(_global);
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let cc_manager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::change_start_height(
                                cc_manager,
                                _start_height,
                                _ctx
                            );
    }


    public entry fun change_poly_id(
        _global: &mut CrossChainGlobalConfig, 
        _poly_id: u64, 
        _ctx: &mut TxContext
    )  {
        config::check_version(_global);
        // sender address
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let ccManager = config::borrow_mut_crosschain_manager(_global);
        
        cross_chain_manager::set_poly_id(
                                ccManager,
                                _poly_id,
                                _ctx
                            );
    }

     // update crosschain_config
    public entry fun update_crosschain_config(
        _global: &mut CrossChainGlobalConfig,
        _keepers: vector<vector<u8>>,
        _start_height: u64,
        _poly_id: u64,
        _ctx: &mut TxContext
    )  {

        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        config::check_version(_global);
        let ccManager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::update_cross_chain_manager_config(
                                ccManager,
                                _keepers,
                                _start_height,
                                _poly_id,
                                _ctx
                             );
    }

    public entry fun relay_unlock_tx<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _treasury_ref:&mut Treasury<CoinType>,
        _proof: vector<u8>, 
        _raw_header: vector<u8>, 
        _header_proof: vector<u8>, 
        _cur_raw_header: vector<u8>, 
        _header_sig: vector<u8>,
        _clock: &Clock,
        _ctx: &mut TxContext
    ) {
        //check system pause
      
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_assets_role(_global, sender);
        config::check_pause(_global);
        let (lp_manager,cc_manager) = config::borrow_mut_lp_and_cc_managers(_global);
        let license_ref = lock_proxy::get_license_ref(lp_manager);

        let certificate = cross_chain_manager::verify_header_and_execute_tx(
                                                    cc_manager,
                                                    license_ref, 
                                                    &_proof, 
                                                    &_raw_header, 
                                                    &_header_proof, 
                                                    &_cur_raw_header, 
                                                    &_header_sig, 
                                                    _ctx
                                                );
        lock_proxy::relay_unlock_tx<CoinType>(
                        &certificate,
                        lp_manager,
                        _treasury_ref,
                        _clock, 
                        _ctx
                    );
    }

    public entry fun relay_unlock_tx_read_certificate<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _treasury_ref:&mut Treasury<CoinType>,
        _proof: vector<u8>, 
        _raw_header: vector<u8>, 
        _header_proof: vector<u8>, 
        _cur_raw_header: vector<u8>, 
        _header_sig: vector<u8>,
        _clock: &Clock,
        _ctx: &mut TxContext
    ) {
        //check system pause
      
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_assets_role(_global, sender);
        config::check_pause(_global);
        let (lp_manager,cc_manager) = config::borrow_mut_lp_and_cc_managers(_global);
        let license_ref = lock_proxy::get_license_ref(lp_manager);

       cross_chain_manager::verify_header_and_execute_tx(
                                            cc_manager,
                                            license_ref, 
                                            &_proof, 
                                            &_raw_header, 
                                            &_header_proof, 
                                            &_cur_raw_header, 
                                            &_header_sig, 
                                            _ctx
                                        );
    }

    public entry fun get_asset<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _to_chain_id: u64,
        _ctx: &mut TxContext
    ) {
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let lp_manager = config::borrow_lp_manager(_global);
        lock_proxy::get_to_asset<CoinType>(lp_manager,_to_chain_id);
    }

    public entry fun lock_and_pay_fee<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _treasury_ref:&mut Treasury<CoinType>,
        _fund: &mut Coin<CoinType>,
        _amount: u64,
        _fee: Coin<BFC>,
        _to_chain_id: u64, 
        _to_address: vector<u8>,
        _clock:&Clock,
        _ctx: &mut TxContext
    )  {

        //check system pause
        config::check_pause(_global);
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        let (lp_manager,wrapper_store,cc_manager) = config::borrow_mut_all(_global);     
        wrapper_v1::lock_and_pay_fee_with_fund<CoinType>(
                        cc_manager,
                        lp_manager,
                        _treasury_ref,
                        wrapper_store,
                        sender, 
                        _fund, 
                        _amount,
                        _fee, 
                        _to_chain_id, 
                        &_to_address,
                        _clock, 
                        _ctx
                    );
    }

    public entry fun update_lock_proxy_manager_start_time(
        _global:&mut CrossChainGlobalConfig,
        _clock: &Clock, 
        _ctx: &mut TxContext
    ) {
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);

        let lpManager = config::borrow_mut_lp_manager(_global);

        lock_proxy::update_lock_proxy_manager_start_time(
                        lpManager,
                        _clock,
                        _ctx
                    );
    }

    public entry fun bind_proxy(
        _global: &mut CrossChainGlobalConfig,
        _to_chain_id: u64,
        _target_proxy_hash: vector<u8>,
        _ctx: &mut TxContext
    )  {
        // sender address
        config::check_version(_global);
        config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        
        lock_proxy::bind_proxy(
                        lp_manager,
                        _to_chain_id,
                        _target_proxy_hash
                    );
    }

    public entry fun unbind_proxy(
        _global: &mut CrossChainGlobalConfig, 
        _to_chain_id: u64, 
        _ctx: &mut TxContext
    ) {

        config::check_version(_global);
        config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);

        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::unbind_proxy(
                        lpManager,
                        _to_chain_id
                    );
    }

    public entry fun bind_asset<CoinType>(
        _global: &mut CrossChainGlobalConfig, 
        _to_chain_id: u64,
        _to_asset_hash: vector<u8>,
        _to_asset_decimals: u8,
        _ctx: &mut TxContext
    )  {

        config::check_version(_global);
        config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let lpManager = config::borrow_mut_lp_manager(_global);
      
        lock_proxy::bind_asset<CoinType>(
                        lpManager,
                        _to_chain_id,
                        _to_asset_hash,
                        _to_asset_decimals,
                        _ctx
                    );
    }

    public entry fun unbind_asset<CoinType>(
        _global: &mut CrossChainGlobalConfig, 
        _to_chain_id: u64, 
        _ctx: &mut TxContext
    ) {
        config::check_version(_global);
        config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);

        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::unbind_asset<CoinType>(
                        lpManager,
                        _to_chain_id
                    );
    }

    public entry fun output_license_id( _global: &mut CrossChainGlobalConfig) {
        let lpManager = config::borrow_lp_manager(_global);
        lock_proxy::output_license_id(lpManager);
    }

    public entry fun grant_role(
        _global: &mut CrossChainGlobalConfig,  
        _role: u64, 
        _account: address, 
        _ctx: &mut TxContext
    )  {
        config::check_version(_global);
        config::grant_role(
                    _global,
                    _role,
                    _account,
                    _ctx
                 );
}

    public entry fun revoke_role(
        _global: &mut CrossChainGlobalConfig,  
        _role: u64, 
        _account: address, 
        _ctx: &mut TxContext
     )  {
        config::check_version(_global);
        config::revoke_role(
                    _global,
                    _role,
                    _account,
                    _ctx
                 );

    }

    public entry fun set_blacklist(
        _global: &mut CrossChainGlobalConfig,  
        _license_id: vector<u8>,
        _access_level: u8, 
        _ctx: &mut TxContext
    )  {

        config::check_version(_global);
        config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global,sender);
        let ccManager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::set_blacklist(
                                ccManager,
                                _license_id,
                                _access_level,
                                _ctx
                            );
    }

    public entry fun issue_license_to_lock_proxy(
        _global: &mut CrossChainGlobalConfig, 
        _contract: address,//global_config 
        _ctx: &mut TxContext
    ) {
        config::check_version(_global);
        config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global,sender);
        let license = cross_chain_manager::issue_license( b"lock_proxy", _contract);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        lock_proxy::receive_license(lp_manager,license);
    }

    public entry fun pause_global(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){

        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global,sender);
        config::pause(_global);
    }

    public entry fun unpause_global(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){

        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global,sender);
        config::unpause(_global);
    }

     public entry fun update_lock_min_amount(
        _global:&mut CrossChainGlobalConfig,
        _min_amount: u64,
        _ctx: &mut TxContext
    )  {
        config::check_version(_global);
        // config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        lock_proxy::update_lock_min_amount(lp_manager,_min_amount);
    }

    public entry fun update_unlock_min_amount(
        _global:&mut CrossChainGlobalConfig,
        _min_amount: u64,
        _ctx: &mut TxContext
    )  {
        config::check_version(_global);
        // config::check_pause(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        lock_proxy::update_unlock_min_amount(lp_manager,_min_amount);
    }

    public entry fun reset_lock_amount(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){
        
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        let amount_manager = lock_proxy::borrow_lock_amount_limit_manager(lp_manager);
        lock_proxy::reset_amount(amount_manager);
    }

    public entry fun reset_unlock_amount(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){
        
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_operator_role(_global, sender);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        let amount_manager = lock_proxy::borrow_unlock_amount_limit_manager(lp_manager);
        lock_proxy::reset_amount(amount_manager);
    }

    public entry fun mint_treasury<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _cap: &mut TreasuryCap<CoinType>,
        _treasury: &mut Treasury<CoinType>,
        _amount: u64, 
        _ctx: &mut TxContext
    ){
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_treasury_role(_global, sender);
        let new_coin = bf_usdc::mint_treasury<CoinType>(
                                                        _cap,
                                                        _amount,
                                                        _ctx
                                                    );
        lock_proxy::deposit<CoinType>(_treasury, new_coin);     
    }

    public entry fun deposit_treasury<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _treasury: &mut Treasury<CoinType>,
        _coin:&mut Coin<CoinType>, 
        _amount: u64, 
        _ctx: &mut TxContext
    ){
        config::check_version(_global);
        assert!(_amount > 0 ,EDEPOSIT_TREASURY_AMOUNT );
        let deposit_coin = coin::split<CoinType>(_coin,_amount,_ctx);
        lock_proxy::deposit<CoinType>(_treasury, deposit_coin);    
    }

    #[test_only]
    public  fun test_relay_unlock_tx<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _treasury_ref:&mut Treasury<CoinType>,
        _proof: vector<u8>, 
        _raw_header: vector<u8>, 
        _header_proof: vector<u8>, 
        _cur_raw_header: vector<u8>, 
        _header_sig: vector<u8>,
        _clock: &Clock,
        _ctx: &mut TxContext
    ) {
        //check system pause
      
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        config::check_assets_role(_global, sender);
        config::check_pause(_global);
        let (lp_manager,cc_manager) = config::borrow_mut_lp_and_cc_managers(_global);
        let license_ref = lock_proxy::get_license_ref(lp_manager);

        let certificate = cross_chain_manager::verify_header_and_execute_tx(
                                                    cc_manager,
                                                    license_ref, 
                                                    &_proof, 
                                                    &_raw_header, 
                                                    &_header_proof, 
                                                    &_cur_raw_header, 
                                                    &_header_sig, 
                                                    _ctx
                                                );
        lock_proxy::test_relay_unlock_tx<CoinType>(
                        &certificate,
                        lp_manager,
                        _treasury_ref,
                        _clock, 
                        _ctx
                    );
    }
}
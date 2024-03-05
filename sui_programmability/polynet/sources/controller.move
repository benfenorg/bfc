module polynet::controller {
    use std::type_name::{Self};
    use sui::transfer;
    use sui::tx_context::{TxContext, Self};
    use sui::clock::Clock;
    use sui::coin::{Coin, Self};
    use sui::bfc::BFC;
    use polynet::events;
    use polynet::utils;
    use polynet::config::{CrossChainGlobalConfig, Self};
    use polynet::wrapper_v1::{feeCollector, WrapperStore, Self};
    use polynet::cross_chain_manager::{CrossChainManager, Self};
    use polynet::lock_proxy::{Treasury, LockProxyManager, Self};


    const EINVALID_SYSTEM_IS_PAUSED: u64 = 5000;
    const ELICENSE_NOT_EXIST: u64 = 5001;
    const ERR_VERSION_CHECK: u64 = 5002; 
    const ERR_PAUSE_ROLE: u64 = 5003;
    const EINVALID_ADMIN_SIGNER: u64 = 5004;
    const ENOT_CHANGE_KEEPER_ROLE: u64 = 5005;
    const EVERIFY_HEADER_FAILED: u64 = 5006;
    const EVERIFY_HEADER_PROOF_FAILED: u64 = 5007;
    const EVERIFIER_NOT_RECEIVER: u64 = 5008;
    const EBLACKLISTED_TO: u64 = 5009;

     // update crosschain_manager
    public entry fun migrate(  
        _global: &mut CrossChainGlobalConfig,
        _keepers: vector<vector<u8>>,
        _start_height: u64,
        _poly_id: u64,
        _ctx: &mut TxContext
    )  {

        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);

        let ccManager = config::borrow_mut_crosschain_manager(_global);
        cross_chain_manager::update_cross_chain_config(
            ccManager,
            _keepers,
            _start_height,
            _poly_id,
            _ctx
        );
        config::migrate(_global,_ctx);
       
        events::migrate_book_keeper_event(
                    _keepers,
                    _start_height,
                    _poly_id,
                    sender
                    );
    }


    public entry fun update_fee_collector(
        _global: &mut CrossChainGlobalConfig, 
        _new_fee_collector: address, 
        _ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);
        config::check_version(_global);

        wrapper_v1::setFeeCollector(
            config::borrow_mut_wrapper_store(_global),
            _new_fee_collector,
            _ctx
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
        let ccManager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::check_keeper_role(ccManager, sender);
        cross_chain_manager::change_book_keeper(
            ccManager,
            _keepers,
            _start_height,
            _ctx
        );
    }

    public entry fun set_poly_id(
        _global: &mut CrossChainGlobalConfig, 
        _poly_id: u64, 
        _ctx: &mut TxContext
    )  {
        config::check_version(_global);
        // sender address
        let sender = tx_context::sender(_ctx);
        let ccManager = config::borrow_mut_crosschain_manager(_global);
        cross_chain_manager::check_keeper_role(ccManager, sender);
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
        _poly_d: u64,
        _ctx: &mut TxContext
    )  {

        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);
        config::check_version(_global);
        let ccManager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::update_cross_chain_config(
                                ccManager,
                                _keepers,
                                _start_height,
                                _poly_d,
                                _ctx
                            );
        
        events::update_book_keeper_event(
                _keepers,
                _start_height,
                _poly_d,
                sender
            );
    }

  


    public entry fun relay_unlock_tx<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _treasury_ref:&mut Treasury<CoinType>,
        _proof: vector<u8>, 
        _rawHeader: vector<u8>, 
        _headerProof: vector<u8>, 
        _curRawHeader: vector<u8>, 
        _headerSig: vector<u8>,
        _clock: &Clock,
        _ctx: &mut TxContext
    ) {
        //check system pause
        config::check_pause(_global);
        config::check_version(_global);

        // let ccManager = config::borrow_mut_crosschain_manager(_global);

        // let poly_id = config::get_poly_id(_global);
        // let cur_epoch_start_height = config::get_cur_epoch_start_height(_global);
        let (lpManager,ccManager) = config::borrow_mut_lp_and_cc_managers(_global);
        
        lock_proxy::check_paused(lpManager);
        let license_ref = lock_proxy::get_license_ref(lpManager);

      

        let certificate = cross_chain_manager::verifyHeaderAndExecuteTx(
            // poly_id,
            // cur_epoch_start_height,
            ccManager,
            license_ref, 
            &_proof, 
            &_rawHeader, 
            &_headerProof, 
            &_curRawHeader, 
            &_headerSig, 
            _ctx
        );
        // let mut_lp_manager = config::borrow_mut_lp_manager(_global);
        lock_proxy::relay_unlock_tx<CoinType>(
            &certificate,
            lpManager,
            _treasury_ref,
            _clock, 
            _ctx
        );
    }

    public entry fun lock_and_pay_fee<CoinType>(
        _global:&mut CrossChainGlobalConfig,
        _treasury_ref:&mut Treasury<CoinType>,
        _account: address,
        _fund: Coin<CoinType>,
        _fee: Coin<BFC>,
        _toChainId: u64, 
        _toAddress: vector<u8>,
        _clock:&Clock,
        _ctx: &mut TxContext
    )  {

        //check system pause
        config::check_pause(_global);
        config::check_version(_global);
        let (lp_manager,wrapper_store,cc_manager) = config::borrow_mut_all(_global);
        lock_proxy::check_paused(lp_manager);
        // let wrapper_store = config::borrow_mut_wrapper_store(_global);
       
        //any user can lock bfc assets and transfer to evm
        // let ccManager = config::borrow_mut_crosschain_manager(_global);


        wrapper_v1::lock_and_pay_fee_with_fund<CoinType>(
                        cc_manager,
                        lp_manager,
                        _treasury_ref,
                        wrapper_store,
                        _account, 
                        _fund, 
                        _fee, 
                        _toChainId, 
                        &_toAddress,
                        _clock, 
                        _ctx
                    );
    }

    public fun lock_and_pay_fee_with_fund<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        wrapperstore:&mut WrapperStore,
        account: address,
        fund: Coin<CoinType>, 
        fee: Coin<BFC>,
        toChainId: u64, 
        toAddress: &vector<u8>,
        clock:&Clock,
        ctx: &mut TxContext
    )  {
        let amount = coin::value(&fund);
        let fee_amount = coin::value(&fee);
        //coin::deposit<BFC>(feeCollector(), fee);

        let feeCollector = feeCollector(wrapperstore);
        transfer::public_transfer(fee, feeCollector);

        lock_proxy::lock(ccManager,lpManager,treasury_ref, account, fund, toChainId, toAddress,clock, ctx);
        //let config_ref = borrow_global_mut<WrapperStore>(POLY_BRIDGE);
        events::lock_with_fee_event(
                    type_name::get<Coin<CoinType>>(),
                    account,
                    toChainId,
                    *toAddress,
                    amount,
                    fee_amount,
                );
    }

    public entry fun update_lock_proxy_manager_start_time(
        _global:&mut CrossChainGlobalConfig,
        _clock: &Clock, 
        _ctx: &mut TxContext
    ) {
        // sender address
        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);
        config::check_version(_global);

        let lpManager = config::borrow_mut_lp_manager(_global);

        lock_proxy::update_lock_proxy_manager_start_time(
            lpManager,
            _clock,
            _ctx
        );
    }

    public entry fun transfer_lp_manager_ownerShip(
        _global: &mut CrossChainGlobalConfig, 
        _new_owner: address, 
        _ctx:&mut TxContext
    ) {
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        let lpManager = config::borrow_mut_lp_manager(_global);

        lock_proxy::onlyOwner(lpManager, sender);
        lock_proxy::transferOwnerShip(lpManager,_new_owner,_ctx);
    }

    public entry fun pause_lp_manager(_global: &mut CrossChainGlobalConfig,  _ctx: &mut TxContext) {

        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::onlyOwner(lpManager, sender);
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        lock_proxy::pause(lpManager,_ctx);
    }

    public entry fun unpause_lp_manager(_global: &mut CrossChainGlobalConfig, _ctx: &mut TxContext) {

        config::check_version(_global);
        // sender address
        let sender = tx_context::sender(_ctx);
        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::onlyOwner(lpManager, sender);
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        lock_proxy::unpause(lpManager,_ctx);
    }

    public entry fun bind_proxy(
        _global: &mut CrossChainGlobalConfig,
        _to_chain_id: u64,
        _target_proxy_hash: vector<u8>,
        _ctx: &mut TxContext
    )  {
        // sender address
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::check_paused(lpManager);
        lock_proxy::onlyOwner(lpManager, sender);
        
        lock_proxy::bind_proxy(
            lpManager,
            _to_chain_id,
            _target_proxy_hash,
            _ctx
        );
    }

    public entry fun unbind_proxy(
        _global: &mut CrossChainGlobalConfig, 
        _to_chain_id: u64, 
        _ctx: &mut TxContext
    ) {

        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::onlyOwner(lpManager, sender);
        lock_proxy::check_paused(lpManager);
        lock_proxy::unbind_proxy(
            lpManager,
            _to_chain_id,
            _ctx
        )
    }

    public entry fun bind_asset<CoinType>(
        _global: &mut CrossChainGlobalConfig, 
        _to_chain_id: u64,
        _to_asset_hash: vector<u8>,
        _to_asset_decimals: u8,
        _ctx: &mut TxContext
    )  {

        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::onlyOwner(lpManager, sender);
        lock_proxy::check_paused(lpManager);
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
        let sender = tx_context::sender(_ctx);
        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::check_paused(lpManager);
        lock_proxy::onlyOwner(lpManager, sender);
        lock_proxy::unbind_asset<CoinType>(
            lpManager,
            _to_chain_id,
            _ctx
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
        let ccManager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::grant_role(
                                ccManager,
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
        let ccManager = config::borrow_mut_crosschain_manager(_global);

        cross_chain_manager::revoke_role(
                                ccManager,
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
        _ctx: &mut TxContext
    ) {
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);
        
        let ccManager = config::borrow_mut_crosschain_manager(_global);
        let license = cross_chain_manager::issueLicense(ccManager, b"lock_proxy", _ctx);

        let lpManager = config::borrow_mut_lp_manager(_global);
        lock_proxy::receiveLicense(lpManager,license);
    }


    public entry fun pause_global(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){

        config::check_version(_global);
        let ccManager = config::borrow_mut_crosschain_manager(_global);
        let sender = tx_context::sender(_ctx);
        cross_chain_manager::check_pause_role(ccManager,sender);
        config::pause(_global,_ctx);
    }

    public entry fun unpause_global(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){

        config::check_version(_global);
        let ccManager = config::borrow_mut_crosschain_manager(_global);
        let sender = tx_context::sender(_ctx);
        cross_chain_manager::check_pause_role(ccManager,sender);
        config::unpause(_global,_ctx);
    }

    public entry fun reset_lock_amount(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){
        
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        let amount_manager = lock_proxy::borrow_lock_amount_limit_manager(lp_manager);
        lock_proxy::reset_amount(amount_manager);
    }

    public entry fun reset_unlock_amount(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){
        
        config::check_version(_global);
        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);
        let lp_manager = config::borrow_mut_lp_manager(_global);
        let amount_manager = lock_proxy::borrow_unlock_amount_limit_manager(lp_manager);
        lock_proxy::reset_amount(amount_manager);
    }







}
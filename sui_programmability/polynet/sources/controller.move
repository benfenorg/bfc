module polynet::controller {
    use sui::event;
    use std::option;
    use std::type_name::{Self, TypeName};
    use sui::transfer;
    use sui::tx_context::{TxContext, Self};
    use sui::clock::Clock;
    use sui::coin::{Coin, Self};
    use sui::bfc::BFC;
    use polynet::events;
    use polynet::config::{CrossChainGlobalConfig, Self};
    use polynet::wrapper_v1::{init_wrapper, feeCollector, WrapperStore};
    use polynet::cross_chain_manager::{CrossChainManager, Self};
    use polynet::lock_proxy::{Treasury, paused, LockProxyManager, Self};


    const EINVALID_SYSTEM_IS_PAUSED: u64 = 5000;
    const ELICENSE_NOT_EXIST: u64 = 5001;
    const ERR_VERSION_CHECK: u64 = 5002; 

     // update crosschain_manager
    public entry fun migrate(  
        _global: &mut CrossChainGlobalConfig,
        _keepers: vector<vector<u8>>,
        _startHeight: u64,
        _polyId: u64,
        _ctx: &mut TxContext
    )  {

        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);

        config::update_config(
            &mut _manager.config,
            _keepers,
            _startHeight,
            _polyId,
            _ctx
        );
        config::migrate(_global,_ctx);
       
        events::migrate_book_keeper_event(
                    _startHeight,
                    _keepers,
                    _polyId,
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
        assert!(utils::is_admin(sender), EINVALID_ADMIN);
        config::check_version(_global);

        wrapper_v1::setFeeCollector(
            config::borrow_mut_wrapper_store(_global),
            _new_fee_collector,
            _ctx
        ); 
    }

     // update crosschain_config
    public entry fun update_crosschain_config(
        _global: &mut CrossChainGlobalConfig,
        _keepers: vector<vector<u8>>,
        _startHeight: u64,
        _polyId: u64,
        _ctx: &mut TxContext
    )  {

        let sender = tx_context::sender(_ctx);
        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);
        config::check_version(_global);

        config::update_config(
            _global,
            _keepers,
            _startHeight,
            _polyId,
            _ctx
        );
       
        event::update_book_keeper_event(
                _startHeight,
                _keepers,
                _polyId,
                sender
            );
    }

   fun relay_unlock_tx_internal<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        proof: vector<u8>, 
        rawHeader: vector<u8>, 
        headerProof: vector<u8>, 
        curRawHeader: vector<u8>, 
        headerSig: vector<u8>,
        clock:&Clock,
        ctx: &mut TxContext
    )  {

        // borrow license
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
        let license_ref = option::borrow(&lpManager.license_store.license);

        let certificate = cross_chain_manager::verifyHeaderAndExecuteTx(ccManager,license_ref, &proof, &rawHeader, &headerProof, &curRawHeader, &headerSig, ctx);
        lock_proxy::unlock<CoinType>(lpManager, treasury_ref, certificate, clock, ctx);
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
        let lpManager = config::borrow_mut_lp_manager(_global);
        let ccManager = config::borrow_mut_crosschain_manager(_global);
        lock_proxy::check_paused(lpManager);
        config::check_version(_global);

        relay_unlock_tx_internal<CoinType>(
            ccManager, 
            lpManager,
            _treasury_ref,
            _proof, 
            _rawHeader, 
            _headerProof, 
            _curRawHeader, 
            _headerSig, 
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
        let lpManager = config::borrow_mut_lp_manager(_global);
        let ccManager = config::borrow_mut_crosschain_manager(_global);
        let wrapperStore = config::borrow_mut_wrapper_store(_global);
        lock_proxy::check_paused(lpManager);
        config::check_version(_global);

        //any user can lock bfc assets and transfer to evm

        wrapper_v1::lock_and_pay_fee_with_fund<CoinType>(
                        ccManager,
                        lpManager,
                        _treasury_ref,
                        wrapperstore,
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
        let sender = tx_context::sender(ctx);
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
        let sender = tx_context::sender(ctx);
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
        let sender = tx_context::sender(ctx);
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
        let sender = tx_context::sender(ctx);
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




}
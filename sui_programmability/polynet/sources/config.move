module polynet::config {
    use sui::event;
    use std::vector;
    use sui::transfer;
    use polynet::events;
    use sui::object::{UID, Self};
    use sui::table::{Table, Self};
    use sui::tx_context::{TxContext, Self};
    use polynet::utils;
    use polynet::acl::{Access_control_list, Self};
    use polynet::wrapper_v1::{WrapperStore, Self};
    use polynet::cross_chain_manager::{CrossChainManager, Self,ACLStore};
    use polynet::lock_proxy::{Treasury, paused, LockProxyManager, Self};

    friend polynet::controller;

    const VERSION: u64 = 1;
    const ERR_CHECK_CONFIG_PAUSED: u64 = 6000;
    const ERR_VERSION_CHECK: u64 = 6001;
    const EALREADY_EXECUTED: u64 = 6002;


    struct CrossChainGlobalConfig has key,store{
        id: UID,
        // polyId: u64,
        paused: bool,
        // curBookKeepers: vector<vector<u8>>,
        // curEpochStartHeight: u64,
        // ethToPolyTxHashMap: Table<u128, vector<u8>>,
        // fromChainTxExist: Table<u64, Table<vector<u8>, bool>>,
        crossChainManager: CrossChainManager,
        lockProxyManager: LockProxyManager,
        wrapperStore: WrapperStore,
        version: u64
    }

  
     // initialize
   fun init(_ctx: &mut TxContext)  {

        // init global config
        let config = CrossChainGlobalConfig{
            id: object::new(_ctx),
            // polyId: 0,
            paused: false,
            // curBookKeepers: vector::empty<vector<u8>>(),
            // curEpochStartHeight: 0,
            // ethToPolyTxHashMap: table::new<u128, vector<u8>>(_ctx),
            // fromChainTxExist: table::new<u64, Table<vector<u8>, bool>>(_ctx),
            crossChainManager: cross_chain_manager::new(_ctx),
            lockProxyManager: lock_proxy::new(_ctx),
            wrapperStore: wrapper_v1::new(_ctx),
            version: VERSION
        };
       
        transfer::share_object(config);

        events::init_book_keeper_event(
                    vector::empty<vector<u8>>(),
                    0,
                    sender 
                );
    }

    public(friend) fun migrate(
        _global: &mut CrossChainGlobalConfig,
        _ctx: &mut TxContext 
    ) {
        assert!(_global.version < VERSION,ERR_VERSION_CHECK);
        _global.version = VERSION;
      
        events::migrate(tx_context::sender(_ctx));
    }


    public(friend) fun update_config(
        _global: &mut CrossChainGlobalConfig,
        _keepers: vector<vector<u8>>,
        _startHeight: u64,
        _polyId: u64,
        _ctx: &mut TxContext
    ) {
       
        _global.polyId = _polyId;
        _global.curEpochStartHeight = _startHeight;
        _global.curBookKeepers = _keepers;

    }

    public(friend) fun check_version(_global: &CrossChainGlobalConfig) {
        assert!(_global.version == VERSION,ERR_VERSION_CHECK);

    }

    public(friend) fun borrow_wrapper_store(_global: &CrossChainGlobalConfig): WrapperStore {
        _global.wrapperStore
    }

    public(friend) fun borrow_mut_wrapper_store(_global: &mut CrossChainGlobalConfig): &mut WrapperStore {
        &mut _global.wrapperStore
    }

    public(friend) fun borrow_mut_crosschain_manager(_global: &mut CrossChainGlobalConfig): &mut CrossChainManager {
        &mut _global.crossChainManager
    }

    public(friend) fun borrow_crosschain_manager(_global: &mut CrossChainGlobalConfig): CrossChainManager {
        _global.crossChainManager
    }


    public(friend) fun borrow_mut_lp_manager(_global: &mut CrossChainGlobalConfig): &mut LockProxyManager {
        &mut _global.lockProxyManager
    }

     public(friend) fun borrow_lp_manager(_global: &mut CrossChainGlobalConfig): LockProxyManager {
        _global.lockProxyManager
    }


       // pause/unpause
   fun paused(_global: &CrossChainGlobalConfig): bool  {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
       _global.paused
    }

    public(friend) fun check_pause(_global:&CrossChainGlobalConfig){

        assert!(!paused(_global),ERR_CHECK_CONFIG_PAUSED);
      
    }


    public(friend) fun pause(_global:&mut CrossChainGlobalConfig, _ctx: &mut TxContext){

        assert!(!paused(_global),ERR_CHECK_CONFIG_PAUSED);
        ccManager.config.paused = true;
    }

    public(friend) fun unpause(_global:&mut CrossChainGlobalConfig, c_ctxx: &mut TxContext)  {

        assert!(paused(_global),ERR_CHECK_CONFIG_PAUSED);
        ccManager.config.paused = false;
    }



   


    // public(friend) fun check_from_chain_tx_exist(
    //     _global: &CrossChainGlobalConfig, 
    //     _from_chain_id: u64, 
    //     _from_chain_tx: &vector<u8>
    // ) {
    //     //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
    //     let exist = false;
    //     if (table::contains(&_global.fromChainTxExist, _from_chain_id)) {
    //         if (table::contains(table::borrow(&_global.fromChainTxExist, _from_chain_id), *_from_chain_tx)) {
    //             exist = true;
    //         };
    //     };
    //     assert!(!exist, EALREADY_EXECUTED);

    // }

   

     
        // markFromChainTxExist(global, from_chain_id, &poly_tx_hash, ctx);




}

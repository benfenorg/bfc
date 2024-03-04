module polynet::config {
    use sui::event;
    use std::vector;
    use sui::object::{UID, Self};
    use sui::table::{Table, Self};
    use sui::tx_context::{TxContext, Self};
    use polynet::acl::{Access_control_list, Self};
    use polynet::wrapper_v1::{WrapperStore, Self};
    use polynet::cross_chain_manager::{CrossChainManager, Self,ACLStore};
    use polynet::lock_proxy::{Treasury, paused, LockProxyManager, Self};

    friend polynet::controller;

    const VERSION: u64 = 1;
    const ERR_CHECK_CONFIG_PAUSED: u64 = 5000;

    struct CrossChainGlobalConfig has key,store{
        id: UID,
        polyId: u64,
        paused: bool,
        ethToPolyTxHashIndex: u128,
        curBookKeepers: vector<vector<u8>>,
        curEpochStartHeight: u64,
        ethToPolyTxHashMap: Table<u128, vector<u8>>,
        fromChainTxExist: Table<u64, Table<vector<u8>, bool>>,
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
            polyId: 0,
            paused: false,
            ethToPolyTxHashIndex: 0,
            curBookKeepers: vector::empty<vector<u8>>(),
            curEpochStartHeight: 0,
            ethToPolyTxHashMap: table::new<u128, vector<u8>>(_ctx),
            fromChainTxExist: table::new<u64, Table<vector<u8>, bool>>(_ctx),
            crossChainManager: cross_chain_manager::new(_ctx),
            lockProxyManager: lock_proxy::new(_ctx),
            wrapperStore: wrapper_v1::new(_ctx),
            version: VERSION
        };
       
        transfer::share_object(config);

        event::emit(
            InitBookKeeperEvent{
                height: 0,
                sender,
                keepers: vector::empty<vector<u8>>(),
            },
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
        _global.startHeight = _startHeight;
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




}

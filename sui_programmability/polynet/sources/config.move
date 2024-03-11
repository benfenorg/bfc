#[allow(unused_const)]
module polynet::config {
    use sui::transfer;
    use polynet::events;
    use sui::object::{UID, Self};
    use sui::tx_context::{TxContext, Self};
    use polynet::wrapper_v1::{WrapperStore, Self};
    use polynet::cross_chain_manager::{CrossChainManager, Self};
    use polynet::lock_proxy::{LockProxyManager, Self};

    friend polynet::controller;
    #[test_only]
    friend polynet::cross_chain_manager_test;
    #[test_only]
    friend polynet::lock_proxy_test;
    #[test_only]
    friend polynet::wrapper_v1_test;
    friend polynet::tools;

    const VERSION: u64 = 2;
    const ERR_CHECK_CONFIG_PAUSED: u64 = 6000;
    const ERR_VERSION_CHECK: u64 = 6001;
    const EALREADY_EXECUTED: u64 = 6002;


    struct CrossChainGlobalConfig has key {
        id: UID,
        paused: bool,
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
            paused: false,
            crossChainManager: cross_chain_manager::new(_ctx),
            lockProxyManager: lock_proxy::new(_ctx),
            wrapperStore: wrapper_v1::new(_ctx),
            version: VERSION
        };
       
        transfer::share_object(config);

    }

    public(friend) fun init_cc_config(_ctx: &mut TxContext){
        // init global config
        let config = CrossChainGlobalConfig{
            id: object::new(_ctx),
            paused: false,
            crossChainManager: cross_chain_manager::new(_ctx),
            lockProxyManager: lock_proxy::new(_ctx),
            wrapperStore: wrapper_v1::new(_ctx),
            version: VERSION
        };

        transfer::share_object(config);

    }


    public(friend) fun migrate(
        _global: &mut CrossChainGlobalConfig,
        _ctx: &mut TxContext 
    ) {
        assert!(_global.version < VERSION,ERR_VERSION_CHECK);
        _global.version = VERSION;
      
        events::migrate(tx_context::sender(_ctx));
    }

    public(friend) fun check_version(_global: &CrossChainGlobalConfig) {
        assert!(_global.version == VERSION,ERR_VERSION_CHECK);

    }

    public(friend) fun borrow_wrapper_store(_global: &CrossChainGlobalConfig): &WrapperStore {
        &_global.wrapperStore
    }

    public(friend) fun borrow_mut_wrapper_store(_global: &mut CrossChainGlobalConfig): &mut WrapperStore {
        &mut _global.wrapperStore
    }

    public(friend) fun borrow_mut_crosschain_manager(_global: &mut CrossChainGlobalConfig): &mut CrossChainManager {
        &mut _global.crossChainManager
    }

    public(friend) fun borrow_crosschain_manager(_global: &mut CrossChainGlobalConfig): &CrossChainManager {
        &_global.crossChainManager
    }


    public(friend) fun borrow_mut_lp_manager(_global: &mut CrossChainGlobalConfig): &mut LockProxyManager {
        &mut _global.lockProxyManager
    }

     public(friend) fun borrow_lp_manager(_global: &mut CrossChainGlobalConfig): &LockProxyManager {
        &_global.lockProxyManager
    }

    public(friend) fun borrow_mut_all(
        _global: &mut CrossChainGlobalConfig
    ): (
        &mut LockProxyManager,
        &mut WrapperStore,
        &mut CrossChainManager
        ) {
        (&mut _global.lockProxyManager, &mut _global.wrapperStore,&mut _global.crossChainManager )
    }

    public(friend) fun borrow_mut_lp_and_cc_managers(
        _global: &mut CrossChainGlobalConfig
    ): (
        &mut LockProxyManager,
        &mut CrossChainManager
        ) {
        (&mut _global.lockProxyManager, &mut _global.crossChainManager )
    }

       // pause/unpause
   fun paused(_global: &CrossChainGlobalConfig): bool  {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
       _global.paused
    }

    public(friend) fun check_pause(_global:&CrossChainGlobalConfig){

        assert!(!paused(_global),ERR_CHECK_CONFIG_PAUSED);
      
    }


    public(friend) fun pause(_global:&mut CrossChainGlobalConfig){

        assert!(!paused(_global),ERR_CHECK_CONFIG_PAUSED);
        _global.paused = true;
    }

    public(friend) fun unpause(_global:&mut CrossChainGlobalConfig)  {

        assert!(paused(_global),ERR_CHECK_CONFIG_PAUSED);
        _global.paused = false;
    }




}

module polynet::config {
    use sui::transfer;
    use polynet::events;
    use sui::table::{Table, Self};
    use sui::object::{UID, Self};
    use polynet::acl::{AccessControlManager, Self};
    use sui::tx_context::{TxContext, Self};
    use polynet::wrapper_v1::{WrapperStore, Self};
    use polynet::cross_chain_manager::{CrossChainManager, Self};
    use polynet::lock_proxy::{LockProxyManager, Self};

    friend polynet::controller;
    friend polynet::tools;

    #[test_only]
    friend polynet::cross_chain_manager_test;
    #[test_only]
    friend polynet::lock_proxy_test;
    #[test_only]
    friend polynet::wrapper_v1_test;
    #[test_only]
    friend polynet::tools_test;
    #[test_only]
    friend polynet::unlock_test;
    #[test_only]
    friend polynet::controller_test;

    const VERSION: u64 = 1;
    const ERR_CHECK_CONFIG_PAUSED: u64 = 6000;
    const ERR_VERSION_CHECK: u64 = 6001;
    const EALREADY_EXECUTED: u64 = 6002;
    const ENOT_OPERATE_ROLE_ROLE: u64 = 6003;
    const EALREADY_HAS_ROLE: u64 = 6004;
    const ENOT_HAS_ROLE: u64 = 6005;
    const ENOT_ADMIN: u64 = 6006;
    const ENOT_ASSETS_ROLE: u64 = 6007;
    const ENOT_TREASURY_ROLE: u64 = 6008;

     //basic roles 
    const ADMIN_ROLE: u64 = 1;
    const OPERATE_ROLE: u64 = 2;
    const ASSETS_ROLE: u64 = 3;
    const TREASURY_ROLE: u64 = 4;

     // access control
    struct ACLStore has store {
        role_acls: Table<u64, AccessControlManager>
    }

    struct CrossChainGlobalConfig has key {
        id: UID,
        paused: bool,
        acl_store: ACLStore,
        cross_chain_manager: CrossChainManager,
        lock_proxy_manager: LockProxyManager,
        wrapper_store: WrapperStore,
        version: u64
    }

     //init package and initialize crossChainManager/ lockProxyManager/ wrapperStore/ 
    fun init(_ctx: &mut TxContext)  {

        let sender = tx_context::sender(_ctx);

        assert!(acl::is_admin(sender), ENOT_ADMIN);
       // init access control lists
        let acls = table::new<u64, AccessControlManager>(_ctx);

        let admin_acl = acl::empty();
        let operate_acl = acl::empty();
        let assets_acl = acl::empty();
        let treasury_acl = acl::empty();

        acl::add_all(&mut admin_acl, acl::get_default_admin_address());
        acl::add_all(&mut operate_acl, acl::get_default_admin_address());
        acl::add_all(&mut assets_acl, acl::get_default_assets_admin_address());
        acl::add_all(&mut treasury_acl, acl::get_default_treasury_admin_address());

        table::add(&mut acls, ADMIN_ROLE, admin_acl);
        table::add(&mut acls, OPERATE_ROLE, operate_acl);
        table::add(&mut acls, ASSETS_ROLE, assets_acl);
        table::add(&mut acls, TREASURY_ROLE, treasury_acl);

        let acl_store = ACLStore{
            role_acls: acls
        };

        // init global config
        let config = CrossChainGlobalConfig{
            id: object::new(_ctx),
            paused: false,
            acl_store: acl_store,
            cross_chain_manager: cross_chain_manager::new(_ctx),
            lock_proxy_manager: lock_proxy::new(_ctx),
            wrapper_store: wrapper_v1::new(_ctx),
            version: VERSION
        };
       
        transfer::share_object(config);
    }

    public(friend) fun init_cc_config(_ctx: &mut TxContext){

        let sender = tx_context::sender(_ctx);

        assert!(acl::is_admin(sender), ENOT_ADMIN);
       // init access control lists
        let acls = table::new<u64, AccessControlManager>(_ctx);

        let admin_acl = acl::empty();
        let operate_acl = acl::empty();
        let assets_acl = acl::empty();
        let treasury_acl = acl::empty();

        acl::add_all(&mut admin_acl, acl::get_default_admin_address());
        acl::add_all(&mut operate_acl, acl::get_default_admin_address());
        acl::add_all(&mut assets_acl, acl::get_default_assets_admin_address());
        acl::add_all(&mut treasury_acl, acl::get_default_treasury_admin_address());

        table::add(&mut acls, ADMIN_ROLE, admin_acl);
        table::add(&mut acls, OPERATE_ROLE, operate_acl);
        table::add(&mut acls, ASSETS_ROLE, assets_acl);
        table::add(&mut acls, TREASURY_ROLE, treasury_acl);

        let acl_store = ACLStore{
            role_acls: acls
        };

        // init global config
        let config = CrossChainGlobalConfig{
            id: object::new(_ctx),
            paused: false,
            acl_store: acl_store,
            cross_chain_manager: cross_chain_manager::new(_ctx),
            lock_proxy_manager: lock_proxy::new(_ctx),
            wrapper_store: wrapper_v1::new(_ctx),
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
        &_global.wrapper_store
    }

    public(friend) fun borrow_mut_wrapper_store(_global: &mut CrossChainGlobalConfig): &mut WrapperStore {
        &mut _global.wrapper_store
    }

    public(friend) fun borrow_mut_crosschain_manager(_global: &mut CrossChainGlobalConfig): &mut CrossChainManager {
        &mut _global.cross_chain_manager
    }

    public(friend) fun borrow_crosschain_manager(_global: &mut CrossChainGlobalConfig): &CrossChainManager {
        &_global.cross_chain_manager
    }

    public(friend) fun borrow_mut_lp_manager(_global: &mut CrossChainGlobalConfig): &mut LockProxyManager {
        &mut _global.lock_proxy_manager
    }

    public(friend) fun borrow_lp_manager(_global: &mut CrossChainGlobalConfig): &LockProxyManager {
        &_global.lock_proxy_manager
    }

    public(friend) fun borrow_mut_all(
        _global: &mut CrossChainGlobalConfig
    ): (
        &mut LockProxyManager,
        &mut WrapperStore,
        &mut CrossChainManager
        ) {
        (&mut _global.lock_proxy_manager, &mut _global.wrapper_store,&mut _global.cross_chain_manager )
    }

    public(friend) fun borrow_mut_lp_and_cc_managers(
        _global: &mut CrossChainGlobalConfig
    ): (
        &mut LockProxyManager,
        &mut CrossChainManager
        ) {
        (&mut _global.lock_proxy_manager, &mut _global.cross_chain_manager )
    }

       // pause/unpause
    fun paused(_global: &CrossChainGlobalConfig): bool  {
       _global.paused
    }

    public(friend) fun check_pause(_global:&CrossChainGlobalConfig){

        assert!(!paused(_global),ERR_CHECK_CONFIG_PAUSED);
      
    }

    public(friend) fun pause(_global:&mut CrossChainGlobalConfig){

        assert!(!paused(_global),ERR_CHECK_CONFIG_PAUSED);
        _global.paused = true;
        events::update_pause_status_event(_global.paused);
    }

    public(friend) fun unpause(_global:&mut CrossChainGlobalConfig)  {

        assert!(paused(_global),ERR_CHECK_CONFIG_PAUSED);
        _global.paused = false;
        events::update_pause_status_event(_global.paused);
    }

    public(friend) fun has_role(_config: &CrossChainGlobalConfig, _role: u64, _account: address): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);

        if (table::contains(&_config.acl_store.role_acls, _role)) {
            let role_acl = table::borrow(&_config.acl_store.role_acls, _role);
            return acl::contains(role_acl, _account)
        } else {
            return false
        }
    }

    public(friend) fun grant_role(
        _config:&mut CrossChainGlobalConfig, 
        _role: u64, 
        _account: address, 
        _ctx: &mut TxContext
    )  {
        // sender address
        let sender = tx_context::sender(_ctx);

        assert!(has_role(_config, ADMIN_ROLE, sender), ENOT_ADMIN);
        assert!(!has_role(_config, _role, _account), EALREADY_HAS_ROLE);

        if (table::contains(&_config.acl_store.role_acls, _role)) {
            let role_acl = table::borrow_mut(&mut _config.acl_store.role_acls, _role);
            acl::add(role_acl, _account);
        } else {
            let role_acl = acl::empty();
            acl::add(&mut role_acl, _account);
            table::add(&mut _config.acl_store.role_acls, _role, role_acl);
        };
        events::update_role_event(
            true,
            _role,
            _account
        );
    }

    public(friend) fun revoke_role(
        _config: &mut CrossChainGlobalConfig, 
        _role: u64, 
        _account: address, 
        _ctx: &mut TxContext
    )  {
        // sender address
        let sender = tx_context::sender(_ctx);

        assert!(has_role(_config, ADMIN_ROLE, sender), ENOT_ADMIN);
        assert!(has_role(_config, _role, _account), ENOT_HAS_ROLE);
        let role_acl = table::borrow_mut(&mut _config.acl_store.role_acls, _role);
        acl::remove(role_acl, _account);

        events::update_role_event(
            false,
            _role,
            _account
        );
    }

    public(friend) fun check_operator_role(
         _config: &CrossChainGlobalConfig,
         _sender: address
    ) {
        assert!(has_role(_config, OPERATE_ROLE, (_sender)), ENOT_OPERATE_ROLE_ROLE);
    }

    public(friend) fun check_admin_role (
         _config: &CrossChainGlobalConfig,
         _sender: address
    ) {
          assert!(has_role(_config, ADMIN_ROLE, _sender), ENOT_ADMIN);
    }

    public(friend) fun check_assets_role (
         _config: &CrossChainGlobalConfig,
         _sender: address

    ) {
          assert!(has_role(_config, ASSETS_ROLE, _sender), ENOT_ASSETS_ROLE);
    }

    public(friend) fun check_treasury_role (
         _config: &CrossChainGlobalConfig,
         _sender: address
    ) {
          assert!(has_role(_config, TREASURY_ROLE, _sender), ENOT_TREASURY_ROLE);
    }
}

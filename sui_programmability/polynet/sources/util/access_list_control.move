module polynet::acl {
    use std::vector;

    friend polynet::config;
    friend polynet::tools;
    friend polynet::controller;
    friend polynet::lock_proxy;

    #[test_only]
    friend polynet::utils_test;

    /// The ACL already contains the address.
    const ECONTAIN: u64 = 7000;
    /// The ACL does not contain the address.
    const ENOT_CONTAIN: u64 = 7001;
    const ERROR_EMPTY_ADDRESS: u64 = 7002;

    //operation admin, some of the operation need to be signed by admin
    const ADMINS: vector<address> =vector[
                                          @admin,
                                          @0xb5e92ec96decaa207a41ffa1ea04c9a01ddf049c3a0c06764230cd3be1fc735e //its alexx for test
                                         ];

    //added: add relayer's address to assets admin, only assets admin can unlock token..!!!!
    const ASSETS_ADMIN:vector<address> = vector[
                                                @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590,//its for unit test
                                                @relayer_admin
                                               ];
    //added:  ONLY admin can MINT TREASURY token..!!!!
    const TREASURY_ADMIN:vector<address> = vector[
                                                 @treasure_admin
                                                ];

    struct AccessControlManager has store, drop, copy {
        list: vector<address>
    }

    public fun empty(): AccessControlManager{

        AccessControlManager{ list: vector::empty<address>() }
    }


    public fun contains(acl: &AccessControlManager, addr: address): bool{
        vector::contains(&acl.list, &addr)
    }

    public(friend) fun add_all(_acl: &mut AccessControlManager, _addrs: vector<address>){
        let size = vector::length(&_addrs);
        assert!(size > 0,ERROR_EMPTY_ADDRESS) ;
        
        let idx = 0;
        while (idx < size) {
            let account = vector::borrow<address>(&_addrs, idx);
            add(_acl, *account);
            idx = idx + 1;
        };
    }
    
    public(friend) fun add(_acl: &mut AccessControlManager, _addr: address){

        if (vector::length(&_acl.list) > 0 ) {
             assert!(!vector::contains(&_acl.list, &_addr), ECONTAIN);
        } ;
       
        vector::push_back(&mut _acl.list, _addr);
    }


    public(friend) fun remove(acl: &mut AccessControlManager, addr: address){
        let (found, index) = vector::index_of(&mut acl.list, &addr);
        assert!(found, (ENOT_CONTAIN));
        vector::remove(&mut acl.list, index);
    }

    public fun is_admin(a: address): bool {
        let result = vector::contains(&ADMINS, &a);
        return result
    }

    public(friend) fun is_assets_admin(a: address): bool {
        let result = vector::contains(&ASSETS_ADMIN, &a);
        return result
    }

    public(friend) fun get_default_admin_address(): vector<address> {
        ADMINS
    }

    public(friend) fun get_default_assets_admin_address(): vector<address> {
        ASSETS_ADMIN
    }

    public(friend) fun get_default_treasury_admin_address(): vector<address> {
        TREASURY_ADMIN
    }


    public(friend) fun default_admin_address(): address {
        let account = vector::borrow<address>(&ADMINS, 0);
        *account
    }





}

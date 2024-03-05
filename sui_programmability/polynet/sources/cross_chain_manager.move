module polynet::cross_chain_manager {
    use std::vector;
    use std::hash;
    use std::bcs;
    use polynet::utils;
    use sui::event;
    use sui::object;
    use sui::object::UID;
    use sui::table::{Table, Self};
    use sui::transfer;
    use sui::tx_context;
    //use sui::transfer::transfer;
    use sui::tx_context::TxContext;

    use polynet::acl::Access_control_list;
    use polynet::acl;
    use polynet::zero_copy_sink;
    use polynet::cross_chain_utils;
    use polynet::wrapper_v1;

    friend polynet::lock_proxy;
    friend polynet::controller;
    friend polynet::config;

    const ADMIN_ROLE: u64 = 1;
    const PAUSE_ROLE: u64 = 2;
    const CA_ROLE: u64 = 3;
    const CHANGE_KEEPER_ROLE: u64 = 4;

    // Errors
    const EINVALID_ADMIN_SIGNER: u64 = 4001;
    const EPAUSED: u64 = 4002;
    const EVERIFY_HEADER_FAILED: u64 = 4003;
    const EVERIFY_HEADER_PROOF_FAILED: u64 = 4004;
    const EALREADY_EXECUTED: u64 = 4005;
    const ENOT_TARGET_CHAIN: u64 = 4006;
    const EALREADY_HAS_ROLE: u64 = 4007;
    const ENOT_HAS_ROLE: u64 = 4008;
    const ENOT_ADMIN: u64 = 4009;
    const ENOT_PAUSE_ROLE: u64 = 4010;
    const ENOT_CA_ROLE: u64 = 4011;
    const ENOT_CHANGE_KEEPER_ROLE: u64 = 4012;
    const EBLACKLISTED_FROM: u64 = 4013;
    const EBLACKLISTED_TO: u64 = 4014;
    const EVERIFIER_NOT_RECEIVER: u64 = 4015;
   
    struct CrossChainManager has key, store{
        id: UID,
        paused: bool,
        acl_store: ACLStore,
        poly_id: u64,
        book_keepers: vector<vector<u8>>,
        epoch_start_height: u64,
        tx_hash_index: u128,
        tx_hash_map: Table<u128, vector<u8>>,
        from_chain_tx_exist: Table<u64, Table<vector<u8>, bool>>
        
        

       
    }

    // access control
    struct ACLStore has key, store {
        id: UID,
        role_acls: Table<u64, Access_control_list>,
        license_black_list: Table<vector<u8>, u8>
    }

   
    public(friend) fun new(_ctx: &mut TxContext): CrossChainManager {

          // sender address
        let sender = tx_context::sender(_ctx);

        // init access control lists
        let acls = table::new<u64, Access_control_list>(_ctx);

        let admin_acl = acl::empty();
        let pause_acl = acl::empty();
        let ca_acl = acl::empty();
        let keeper_acl = acl::empty();

        acl::add(&mut admin_acl, utils::get_default_admin_address());
        acl::add(&mut pause_acl, utils::get_default_admin_address());
        acl::add(&mut ca_acl, utils::get_default_admin_address());
        acl::add(&mut keeper_acl, utils::get_default_admin_address());

        table::add(&mut acls, ADMIN_ROLE, admin_acl);
        table::add(&mut acls, PAUSE_ROLE, pause_acl);
        table::add(&mut acls, CA_ROLE, ca_acl);
        table::add(&mut acls, CHANGE_KEEPER_ROLE, keeper_acl);

        let acl_store = ACLStore{
            id: object::new(_ctx),
            role_acls: acls,
            license_black_list: table::new<vector<u8>, u8>(_ctx)
        };

        let manager = CrossChainManager{
            id: object::new(_ctx),
            paused: false,
            acl_store: acl_store,
            poly_id: 0,
            book_keepers: vector::empty<vector<u8>>(),
            epoch_start_height: 0,
            tx_hash_index: 0,
            tx_hash_map: table::new<u128, vector<u8>>(_ctx),
            from_chain_tx_exist: table::new<u64, Table<vector<u8>, bool>>(_ctx)
           
        };
        manager
    }

    public(friend) fun hasRole(ccManager:&mut CrossChainManager, role: u64, account: address): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);

        if (table::contains(&ccManager.acl_store.role_acls, role)) {
            let role_acl = table::borrow_mut(&mut ccManager.acl_store.role_acls, role);
            return acl::contains(role_acl, account)
        } else {
            return false
        }
    }

    public(friend) fun grant_role(
        ccManager:&mut CrossChainManager, 
        role: u64, 
        account: address, 
        ctx: &mut TxContext
    )  {
        // sender address
        let sender = tx_context::sender(ctx);

        assert!(hasRole(ccManager, ADMIN_ROLE, sender), ENOT_ADMIN);
        assert!(!hasRole(ccManager, role, account), EALREADY_HAS_ROLE);

        if (table::contains(&ccManager.acl_store.role_acls, role)) {
            let role_acl = table::borrow_mut(&mut ccManager.acl_store.role_acls, role);
            acl::add(role_acl, account);
        } else {
            let role_acl = acl::empty();
            acl::add(&mut role_acl, account);
            table::add(&mut ccManager.acl_store.role_acls, role, role_acl);
        }
    }

    public(friend) fun revoke_role(
        ccManager:&mut CrossChainManager, 
        role: u64, 
        account: address, 
        ctx: &mut TxContext
    )  {
        // sender address
        let sender = tx_context::sender(ctx);

        assert!(hasRole(ccManager, ADMIN_ROLE, sender), ENOT_ADMIN);
        assert!(hasRole(ccManager, role, account), ENOT_HAS_ROLE);
        //let acl_store_ref = borrow_global_mut<ACLStore>(@poly);
        let role_acl = table::borrow_mut(&mut ccManager.acl_store.role_acls, role);
        acl::remove(role_acl, account);
    }

    public(friend) fun check_keeper_role(
         ccManager:&mut CrossChainManager,
         sender: address
    ) {
        assert!(hasRole(ccManager, CHANGE_KEEPER_ROLE, (sender)), ENOT_CHANGE_KEEPER_ROLE);

    }


    // cross chain license
    struct License has store, copy, drop {
        account: address,
        module_name: vector<u8>
    }

    public fun issueLicense(ccManager:&mut CrossChainManager,
                            module_name: vector<u8>,
                            ctx: &mut TxContext ): License {

        // sender address
        let sender = tx_context::sender(ctx);
        assert!(hasRole(ccManager, CA_ROLE, sender), ENOT_CA_ROLE);
        License{
            account: sender,
            module_name: module_name,
        }
    }

    public fun destroyLicense(license: License) {
        //need admin
        let License{
            account: _, module_name: _ } = license;
    }

    public fun getLicenseId(license: &License): (vector<u8>, LicenseInfo) {
        let res = zero_copy_sink::write_var_bytes(&bcs::to_bytes(&license.account));
        vector::append(&mut res, zero_copy_sink::write_var_bytes(&license.module_name));
        let licenseInfo = LicenseInfo{
            account: license.account,
            module_name: license.module_name,
        };
        return (res, licenseInfo)
    }

    public fun getLicenseInfo(license: &License): (address, vector<u8>) {
        (license.account, license.module_name)
    }


    // black list
    // access level: 0b000000xy , x means blackListed as fromContract , y means blackListed as toContract
    public fun isBlackListedFrom(ccManager:&CrossChainManager, license_id: vector<u8>): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);
        if (table::contains(&ccManager.acl_store.license_black_list, license_id)) {
            let access_level = *table::borrow(&ccManager.acl_store.license_black_list, license_id);
            return (access_level & 0x02) != 0
        } else {
            return false
        }
    }

    public fun isBlackListedTo(ccManager:&CrossChainManager, license_id: vector<u8>): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);
        if (table::contains(&ccManager.acl_store.license_black_list, license_id)) {
            let access_level = *table::borrow(&ccManager.acl_store.license_black_list, license_id);
            return (access_level & 0x01) != 0
        } else {
            return false
        }
    }

    public(friend) fun set_blacklist(
        ccManager:&mut CrossChainManager,
        license_id: vector<u8>,
        access_level: u8, 
        ctx: &mut TxContext
    )  {

        // sender address
        let sender = tx_context::sender(ctx);

        assert!(hasRole(ccManager, CA_ROLE, sender), ENOT_CA_ROLE);
        //let acl_store_ref = borrow_global_mut<ACLStore>(@poly);
        let v_ref = utils::borrow_mut_with_default(&mut ccManager.acl_store.license_black_list, license_id, access_level);
        *v_ref = access_level;
    }

  
    struct UpdateBookKeeperEvent has store, drop, copy {
        height: u64,
        sender: address,
        keepers: vector<vector<u8>>
    }

   


    struct ChangeBookKeeperEvent has store, drop, copy {
        height: u64,
        keepers: vector<vector<u8>>
    }

    struct CrossChainEvent has store, drop, copy {
        sender: address,
        tx_id: vector<u8>,
        proxy_or_asset_contract: vector<u8>,
        to_chain_id: u64,
        to_contract: vector<u8>,
        raw_data: vector<u8>,
    }

   

   

    public fun check_from_chain_tx_exist(ccManager:&CrossChainManager, fromChainId: u64, fromChainTx: &vector<u8>): bool {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        if (table::contains(&ccManager.from_chain_tx_exist, fromChainId)) {
            if (table::contains(table::borrow(&ccManager.from_chain_tx_exist, fromChainId), *fromChainTx)) {
                return *table::borrow(table::borrow(&ccManager.from_chain_tx_exist, fromChainId), *fromChainTx)
            };
        };
        return false
    }

    public fun getEthTxHashIndex(ccManager:&CrossChainManager): u128 {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
       ccManager.tx_hash_index
    }

    fun putEthTxHash(ccManager:&mut CrossChainManager, hash: &vector<u8>) {
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        let index = ccManager.tx_hash_index;
        utils::upsert(&mut ccManager.tx_hash_map, index, *hash);
        ccManager.tx_hash_index = index + 1;
    }

    public fun getEthTxHash(ccManager:&CrossChainManager, ethHashIndex: u128): vector<u8>  {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        return *table::borrow(&ccManager.tx_hash_map, ethHashIndex)
    }

    // cross chain
    public fun crossChain(
        ccManager:&mut CrossChainManager,
        license: &License,
        toChainId: u64,
        toContract: &vector<u8>,
        method: &vector<u8>,
        txData: &vector<u8>,
        ctx: &mut TxContext
    )  {
        // sender address
        let sender = tx_context::sender(ctx);

        // check license
        let (license_id, _) = getLicenseId(license);
        assert!(!isBlackListedFrom(ccManager, license_id), EBLACKLISTED_FROM);

        // pack args
        let tx_hash_index = getEthTxHashIndex(ccManager);
        let param_tx_hash = bcs::to_bytes(&tx_hash_index);
        vector::reverse(&mut param_tx_hash);

        let cross_chain_id = b"AptosCrossChainManager";
        vector::append(&mut cross_chain_id, copy param_tx_hash);
        cross_chain_id = hash::sha2_256(cross_chain_id);

        let raw_param = zero_copy_sink::write_var_bytes(&param_tx_hash);
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(&cross_chain_id));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(&license_id));
        vector::append(&mut raw_param, zero_copy_sink::write_u64(toChainId));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(toContract));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(method));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(txData));

        // mark
        putEthTxHash(ccManager, &hash::sha2_256(copy raw_param));

        // emit event
        //let event_store = borrow_global_mut<EventStore>(@poly);
        event::emit(
            CrossChainEvent{
                sender: sender,
                tx_id: param_tx_hash,
                proxy_or_asset_contract: license_id,
                to_chain_id: toChainId,
                to_contract: *toContract,
                raw_data: raw_param,
            },
        );
    }

    struct LicenseInfo has store, drop, copy {
        account: address,
        module_name: vector<u8>,
    }

    public fun get_license_account(license: &LicenseInfo) :address{
        return license.account
    }
    public fun get_license_module_name(license: &LicenseInfo) :vector<u8>{
        return license.module_name
    }
    
    // certificate
    struct Certificate has drop {
        from_contract: vector<u8>,
        from_chain_id: u64,
        target_license_id: vector<u8>,
        method: vector<u8>,
        args: vector<u8>
    }

    public fun read_certificate(certificate: &Certificate): (
        vector<u8>,
        u64,
        vector<u8>,
        vector<u8>,
        vector<u8>) 
    {
        return (
            certificate.from_contract,
            certificate.from_chain_id,
            certificate.target_license_id,
            certificate.method,
            certificate.args
        )
    }


    // verify header and execute tx
    public fun verifyHeaderAndExecuteTx(
        polyId: u64,
        cur_epoch_start_height: u64,
        ccManager:&mut CrossChainManager,
        license: &License,
        proof: &vector<u8>,
        rawHeader: &vector<u8>,
        headerProof: &vector<u8>,
        curRawHeader: &vector<u8>,
        headerSig: &vector<u8>,
        ctx: &mut TxContext
    ): Certificate  {
        let (
            _,
            _,
            _,
            height,
            _,
            _,
            _,
            cross_states_root,
            _,
            _,
            _
        ) = cross_chain_utils::deserializeHeader(rawHeader);
        let keepers = get_cur_book_keeper(ccManager);
        let n = vector::length(&keepers);
        let threshold = n - ( n - 1) / 3;

        // verify header
        if (height >= cur_epoch_start_height) {
            assert!(cross_chain_utils::verifySig(rawHeader, headerSig, &keepers, threshold), EVERIFY_HEADER_FAILED);
        } else {
            assert!(cross_chain_utils::verifySig(curRawHeader, headerSig, &keepers, threshold), EVERIFY_HEADER_FAILED);
            let (
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                blockRoot,
                _,
                _
            ) = cross_chain_utils::deserializeHeader(curRawHeader);
            let prove_value = cross_chain_utils::merkleProve(headerProof, &blockRoot);
            assert!(cross_chain_utils::getHeaderHash(rawHeader) == prove_value, EVERIFY_HEADER_PROOF_FAILED);
        };

        // verify cross state proof
        let to_merkle_value_bytes = cross_chain_utils::merkleProve(proof, &cross_states_root);
        let (
            poly_tx_hash,
            from_chain_id,
            source_tx_hash,
            _,
            from_contract,
            to_chain_id,
            to_contract,
            method,
            args
        ) = cross_chain_utils::deserializeMerkleValue(&to_merkle_value_bytes);

        // double-spending check/mark
        assert!(!check_from_chain_tx_exist(ccManager, from_chain_id, &poly_tx_hash), EALREADY_EXECUTED);
        mark_from_Chain_tx_exist(ccManager, from_chain_id, &poly_tx_hash, ctx);

        // check to chain id
        assert!(to_chain_id == polyId, ENOT_TARGET_CHAIN);

        // check verifier
        let (license_id, _) = getLicenseId(license);
        assert!(license_id == to_contract, EVERIFIER_NOT_RECEIVER);

        // check black list
        assert!(!isBlackListedTo(ccManager, to_contract), EBLACKLISTED_TO);

        events::verify_header_and_execute_tx_event(
            from_chain_id,
            to_contract,
            poly_tx_hash,
            source_tx_hash
        );

        // return a certificate to prove the execution is certified
        return Certificate{
            from_contract: from_contract,
            from_chain_id: from_chain_id,
            target_license_id: to_contract,
            method: method,
            args: args
        }
    }

     fun  mark_from_Chain_tx_exist(
        _ccManager: &mut CrossChainManager,
        _fromChainId: u64, 
        _fromChainTx: &vector<u8>, 
        _ctx: &mut TxContext
    ) {
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        if (table::contains(_ccManager.from_chain_tx_exist, _fromChainId)) {
            utils::upsert(table::borrow_mut(&mut _global.from_chain_tx_exist, _fromChainId), *_fromChainTx, true);
            return
        } else {
            let subTable = table::new<vector<u8>, bool>(_ctx);
            table::add(&mut subTable, *_fromChainTx, true);
            table::add(&mut _ccManager.from_chain_tx_exist, _fromChainId, subTable);
            return
        }
    }

    public(friend) fun new_certificate(
        _from_contract: vector<u8>,
        _from_chain_id: u64,
        _target_license_id: vector<u8>,
        _method: vector<u8>,
        _args: vector<u8>
    ): Certificate {
          return Certificate{
            from_contract: _from_contract,
            from_chain_id: _from_chain_id,
            target_license_id: _target_license_id,
            method: _method,
            args: _args
        }

    }


    public(friend) fun get_poly_id(_cross_chain_manager: &CrossChainManager): u64 {
         _cross_chain_manager.poly_id
    }

    public(friend) fun get_cur_epoch_start_height(_cross_chain_manager: &CrossChainManager): u64 {
        _cross_chain_manager.epoch_start_height
    }

    public(friend) fun get_cur_book_keeper(_cross_chain_manager: &CrossChainManager): vector<vector<u8>> {
        _cross_chain_manager.book_keepers

    }
}
module polynet::cross_chain_manager {
    use std::vector;
    use std::hash;
    use std::bcs;
    use polynet::utils;
    use sui::event;
    use sui::table::{Table, Self};
    use sui::tx_context;
    use sui::tx_context::TxContext;
    use polynet::zero_copy_sink;
    use polynet::cross_chain_utils;
    use polynet::events;

    friend polynet::lock_proxy;
    friend polynet::controller;
    friend polynet::config;
    friend polynet::tools;
    #[test_only]
    friend polynet::cross_chain_manager_test;
   

    // Errors
    const EVERIFY_HEADER_FAILED: u64 = 4001;
    const EVERIFY_HEADER_PROOF_FAILED: u64 = 4002;
    const EALREADY_EXECUTED: u64 = 4003;
    const ENOT_TARGET_CHAIN: u64 = 4004;
    const EBLACKLISTED_FROM: u64 = 4005;
    const EBLACKLISTED_TO: u64 = 4006;
    const EVERIFIER_NOT_RECEIVER: u64 = 4007;


    struct CrossChainManager has store {
        paused: bool,
        poly_id: u64,
        book_keepers: vector<vector<u8>>, //special decode pointer
        epoch_start_height: u64,
        tx_hash_index: u128,
        tx_hash_map: Table<u128, vector<u8>>,
        from_chain_tx_exist: Table<u64, Table<vector<u8>, bool>>,
        license_black_list: Table<vector<u8>, u8> //cross chain manager control the black_list
    }

    public(friend) fun new(_ctx: &mut TxContext): CrossChainManager {
     
        let manager = CrossChainManager{
            paused: false,
            poly_id: 998,
            book_keepers: vector::empty<vector<u8>>(),
            epoch_start_height: 0,
            tx_hash_index: 0,
            tx_hash_map: table::new<u128, vector<u8>>(_ctx),
            from_chain_tx_exist: table::new<u64, Table<vector<u8>, bool>>(_ctx),
            license_black_list: table::new<vector<u8>, u8>(_ctx)
           
        };
        manager
    }

    // cross chain license
    struct License has store, copy, drop {
        account: address,
        module_name: vector<u8>
    }

    //TODO: make sure account is token contract address 
    public(friend) fun issue_license(
        _module_name: vector<u8>,
        _ctx: &mut TxContext 
    ): License {

        let sender = tx_context::sender(_ctx);
        License{
            account: sender,
            module_name: _module_name,
        }
    }

    public fun destroyLicense(license: License) {
        //need admin
        let License{
            account: _, module_name: _ } = license;
    }

    public fun get_license_id(license: &License): (vector<u8>, LicenseInfo) {
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
        let (license_id, _) = get_license_id(license);
        assert!(!isBlackListedFrom(ccManager, license_id), EBLACKLISTED_FROM);

        // pack args
        let tx_hash_index = getEthTxHashIndex(ccManager);
        let param_tx_hash = bcs::to_bytes(&tx_hash_index);
        vector::reverse(&mut param_tx_hash);

        let cross_chain_id = b"AptosCrossChainManager"; //TODO: maybe need to change
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
        // polyId: u64,
        // cur_epoch_start_height: u64,
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
        let poly_id = get_poly_id(ccManager);
        let cur_epoch_start_height = get_cur_epoch_start_height(ccManager);
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
        assert!(to_chain_id == poly_id, ENOT_TARGET_CHAIN);

        // check verifier
        // let (license_id, _) = get_license_id(license);
        // assert!(license_id == to_contract, EVERIFIER_NOT_RECEIVER);

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
        if (table::contains(&_ccManager.from_chain_tx_exist, _fromChainId)) {
            utils::upsert(table::borrow_mut(&mut _ccManager.from_chain_tx_exist, _fromChainId), *_fromChainTx, true);
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

    fun get_cur_epoch_start_height(_cross_chain_manager: &CrossChainManager): u64 {
        _cross_chain_manager.epoch_start_height
    }

    public(friend) fun get_cur_book_keeper(_cross_chain_manager: &CrossChainManager): vector<vector<u8>> {
        _cross_chain_manager.book_keepers

    }

    public(friend) fun update_cross_chain_manager_config(
        _cross_chain_manager: &mut CrossChainManager,
        _keepers: vector<vector<u8>>,
        _start_height: u64,
        _poly_id: u64,
        _ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(_ctx);

        _cross_chain_manager.poly_id = _poly_id;
        _cross_chain_manager.epoch_start_height = _start_height;
        _cross_chain_manager.book_keepers = _keepers;

        events::update_book_keeper_event(
            _start_height,
            sender,
            _keepers,
            _poly_id
        );

    }

    public(friend) fun change_start_height(
        _cross_chain_manager: &mut CrossChainManager,
        _start_height: u64,
        _ctx: &mut TxContext

    ) {
        let sender = tx_context::sender(_ctx);
        _cross_chain_manager.epoch_start_height = _start_height;
        events::update_start_height_event(
            _start_height,
            sender
        );
    }

    public(friend) fun change_book_keeper(
        _cross_chain_manager: &mut CrossChainManager,
        _keepers: vector<vector<u8>>,
        _start_height: u64,
        _ctx: &mut TxContext

    ) {
        _cross_chain_manager.epoch_start_height = _start_height;
        _cross_chain_manager.book_keepers = _keepers;

    }

    public(friend) fun set_poly_id(
        _cross_chain_manager: &mut CrossChainManager,
        _poly_id: u64,
        _ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(_ctx);
        _cross_chain_manager.poly_id = _poly_id;
        events::update_poly_id_event(
            _poly_id,
            sender
        );

    }

       // black list
    // access level: 0b000000xy , x means blackListed as fromContract , y means blackListed as toContract
    public fun isBlackListedFrom(ccManager:&CrossChainManager, license_id: vector<u8>): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);
        if (table::contains(&ccManager.license_black_list, license_id)) {
            let access_level = *table::borrow(&ccManager.license_black_list, license_id);
            return (access_level & 0x02) != 0
        } else {
            return false
        }
    }

    public fun isBlackListedTo(ccManager:&CrossChainManager, license_id: vector<u8>): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);
        if (table::contains(&ccManager.license_black_list, license_id)) {
            let access_level = *table::borrow(&ccManager.license_black_list, license_id);
            return (access_level & 0x01) != 0
        } else {
            return false
        }
    }

    public(friend) fun set_blacklist(
        _cc_manager:&mut CrossChainManager,
        _license_id: vector<u8>,
        _access_level: u8, 
        _ctx: &mut TxContext
    )  {

        // sender address
        let sender = tx_context::sender(_ctx);

        let v_ref = utils::borrow_mut_with_default(&mut _cc_manager.license_black_list, _license_id, _access_level);
        *v_ref = _access_level;

        events::set_blacklist_event(
            _license_id,
            _access_level,
            sender
        );
    }

    
}
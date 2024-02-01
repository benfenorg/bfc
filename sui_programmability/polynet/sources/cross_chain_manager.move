module poly::cross_chain_manager {
    use std::vector;
    use std::hash;
    use std::bcs;
    use poly::utils;
    use sui::event;
    use sui::table::{Table, Self};
    use sui::transfer::transfer;
    use sui::tx_context::TxContext;

    use polynet::acl::Access_control_list;
    use polynet::acl;
    use poly::zero_copy_sink;
    use poly::cross_chain_utils;


    // Errors
    const EINVALID_SIGNER: u64 = 1;
    const EPAUSED: u64 = 2;
    const EVERIFY_HEADER_FAILED: u64 = 3;
    const EVERIFY_HEADER_PROOF_FAILED: u64 = 4;
    const EALREADY_EXECUTED: u64 = 5;
    const ENOT_TARGET_CHAIN: u64 = 6;
    const EALREADY_HAS_ROLE: u64 = 7;
    const ENOT_HAS_ROLE: u64 = 8;
    const ENOT_ADMIN: u64 = 9;
    const ENOT_PAUSE_ROLE: u64 = 10;
    const ENOT_CA_ROLE: u64 = 11;
    const ENOT_CHANGE_KEEPER_ROLE: u64 = 12;
    const EBLACKLISTED_FROM: u64 = 13;
    const EBLACKLISTED_TO: u64 = 14;
    const EVERIFIER_NOT_RECEIVER: u64 = 15;


    // access control
    struct ACLStore has key, store{
        role_acls: Table<u64, Access_control_list>,
        license_black_list: Table<vector<u8>, u8>
    }

    const ADMIN_ROLE: u64 = 1;
    const PAUSE_ROLE: u64 = 2;
    const CA_ROLE: u64 = 3;
    const CHANGE_KEEPER_ROLE: u64 = 4;

    public fun hasRole(acl_store_ref: &mut ACLStore, role: u64, account: address): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);

        if (table::contains(&acl_store_ref.role_acls, role)) {
            let role_acl = table::borrow_mut(&mut acl_store_ref.role_acls, role);
            return acl::contains(role_acl, account)
        } else {
            return false
        }
    }

    public entry fun grantRole(acl_store_ref:&mut ACLStore,  admin: address, role: u64, account: address) {
        assert!(hasRole(acl_store_ref, ADMIN_ROLE, admin), ENOT_ADMIN);
        assert!(!hasRole(acl_store_ref, role, account), EALREADY_HAS_ROLE);
        //let acl_store_ref = borrow_global_mut<ACLStore>(@poly);
        if (table::contains(&acl_store_ref.role_acls, role)) {
            let role_acl = table::borrow_mut(&mut acl_store_ref.role_acls, role);
            acl::add(role_acl, account);
        } else {
            let role_acl = acl::empty();
            acl::add(&mut role_acl, account);
            table::add(&mut acl_store_ref.role_acls, role, role_acl);
        }
    }

    public entry fun revokeRole(acl_store_ref:&mut ACLStore, admin: address, role: u64, account: address)  {
        assert!(hasRole(acl_store_ref, ADMIN_ROLE, admin), ENOT_ADMIN);
        assert!(hasRole(acl_store_ref, role, account), ENOT_HAS_ROLE);
        //let acl_store_ref = borrow_global_mut<ACLStore>(@poly);
        let role_acl = table::borrow_mut(&mut acl_store_ref.role_acls, role);
        acl::remove(role_acl, account);
    }


    // cross chain license
    struct License has key, store {
        account: address,
        module_name: vector<u8>
    }

    public fun issueLicense(acl_store_ref: &mut ACLStore, ca: address, account: address, module_name: vector<u8>): License {
        assert!(hasRole(acl_store_ref, CA_ROLE, ca), ENOT_CA_ROLE);
        License{
            account: account,
            module_name: module_name,
        }
    }

    public fun destroyLicense(license: License) {
        let License{ account: _, module_name: _ } = license;
    }

    public fun getLicenseId(license: &License): vector<u8> {
        let res = zero_copy_sink::write_var_bytes(&bcs::to_bytes(&license.account));
        vector::append(&mut res, zero_copy_sink::write_var_bytes(&license.module_name));
        return res
    }

    public fun getLicenseInfo(license: &License): (address, vector<u8>) {
        (license.account, license.module_name)
    }


    // black list
    // access level: 0b000000xy , x means blackListed as fromContract , y means blackListed as toContract
    public fun isBlackListedFrom(acl_store_ref:&ACLStore, license_id: vector<u8>): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);
        if (table::contains(&acl_store_ref.license_black_list, license_id)) {
            let access_level = *table::borrow(&acl_store_ref.license_black_list, license_id);
            return (access_level & 0x02) != 0
        } else {
            return false
        }
    }

    public fun isBlackListedTo(acl_store_ref:&ACLStore, license_id: vector<u8>): bool  {
        //let acl_store_ref = borrow_global<ACLStore>(@poly);
        if (table::contains(&acl_store_ref.license_black_list, license_id)) {
            let access_level = *table::borrow(&acl_store_ref.license_black_list, license_id);
            return (access_level & 0x01) != 0
        } else {
            return false
        }
    }

    public entry fun setBlackList(acl_store_ref:&mut ACLStore,  ca: address, license_id: vector<u8>, access_level: u8)  {
        assert!(hasRole(acl_store_ref, CA_ROLE, ca), ENOT_CA_ROLE);
        //let acl_store_ref = borrow_global_mut<ACLStore>(@poly);
        let v_ref = utils::borrow_mut_with_default(&mut acl_store_ref.license_black_list, license_id, access_level);
        *v_ref = access_level;
    }
 

    // event 
    struct EventStore has key, store {
        //init_book_keeper_event: event::EventHandle<InitBookKeeperEvent>,
        //change_book_keeper_event: event::EventHandle<ChangeBookKeeperEvent>,
        //cross_chain_event: event::EventHandle<CrossChainEvent>,
        //verify_header_and_execute_tx_event: event::EventHandle<VerifyHeaderAndExecuteTxEvent>,
    }

    struct InitBookKeeperEvent has store, drop, copy {
        height: u64,
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

    struct VerifyHeaderAndExecuteTxEvent has store, drop, copy {
        from_chain_id: u64,
        to_contract: vector<u8>,
        cross_chain_tx_hash: vector<u8>,
        from_chain_tx_hash: vector<u8>,
    }

    
    // data store
    struct CrossChainGlobalConfig has key {
        polyId: u64,
        paused: bool,
        ethToPolyTxHashIndex: u128,
        curBookKeepers: vector<vector<u8>>,
        curEpochStartHeight: u64,
        ethToPolyTxHashMap: Table<u128, vector<u8>>,
        fromChainTxExist: Table<u64, Table<vector<u8>, bool>>,
    }

    fun putPolyId(config_ref:&mut CrossChainGlobalConfig, polyId: u64) {
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        config_ref.polyId = polyId;
    }

    public fun getPolyId(config_ref:& CrossChainGlobalConfig): u64 {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        return config_ref.polyId
    }

    fun putCurEpochStartHeight(config_ref:&mut CrossChainGlobalConfig,height: u64)  {
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        config_ref.curEpochStartHeight = height;
    }

    public fun getCurEpochStartHeight(config_ref:&CrossChainGlobalConfig): u64  {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        return config_ref.curEpochStartHeight
    }

    fun putCurBookKeepers(config_ref:&mut CrossChainGlobalConfig,keepers: &vector<vector<u8>>){
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        config_ref.curBookKeepers = *keepers;
    }

    public fun getCurBookKeepers(config_ref:&CrossChainGlobalConfig): vector<vector<u8>> {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        return config_ref.curBookKeepers
    }

    fun  markFromChainTxExist(config_ref:&mut CrossChainGlobalConfig, fromChainId: u64, fromChainTx: &vector<u8>, ctx: &mut TxContext) {
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        if (table::contains(&config_ref.fromChainTxExist, fromChainId)) {
            utils::upsert(table::borrow_mut(&mut config_ref.fromChainTxExist, fromChainId), *fromChainTx, true);
            return
        } else {
            let subTable = table::new<vector<u8>, bool>(ctx);
            table::add(&mut subTable, *fromChainTx, true);
            table::add(&mut config_ref.fromChainTxExist, fromChainId, subTable);
            return
        }
    }

    public fun checkIfFromChainTxExist(config_ref:&CrossChainGlobalConfig, fromChainId: u64, fromChainTx: &vector<u8>): bool {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        if (table::contains(&config_ref.fromChainTxExist, fromChainId)) {
            if (table::contains(table::borrow(&config_ref.fromChainTxExist, fromChainId), *fromChainTx)) {
                return *table::borrow(table::borrow(&config_ref.fromChainTxExist, fromChainId), *fromChainTx)
            };
        };
        return false
    }

    public fun getEthTxHashIndex(config_ref:&CrossChainGlobalConfig): u128 {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        return config_ref.ethToPolyTxHashIndex
    }

    fun putEthTxHash(config_ref:&mut CrossChainGlobalConfig, hash: &vector<u8>) {
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        let index = config_ref.ethToPolyTxHashIndex;
        utils::upsert(&mut config_ref.ethToPolyTxHashMap, index, *hash);
        config_ref.ethToPolyTxHashIndex = index + 1;
    }

    public fun getEthTxHash(config_ref:&CrossChainGlobalConfig, ethHashIndex: u128): vector<u8>  {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        return *table::borrow(&config_ref.ethToPolyTxHashMap, ethHashIndex)
    }


    // pause/unpause
    public fun paused(config_ref:&CrossChainGlobalConfig): bool  {
        //let config_ref = borrow_global<CrossChainGlobalConfig>(@poly);
        return config_ref.paused
    }

    public fun pause(config_ref:&mut CrossChainGlobalConfig, acl_store_ref:&mut ACLStore, account: address)  {
        assert!(hasRole(acl_store_ref, PAUSE_ROLE, account), ENOT_PAUSE_ROLE);
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        config_ref.paused = true;
    }

    public fun unpause(config_ref:&mut CrossChainGlobalConfig, acl_store_ref:&mut ACLStore, account: address)  {
        assert!(hasRole(acl_store_ref, PAUSE_ROLE, (account)), ENOT_PAUSE_ROLE);
        //let config_ref = borrow_global_mut<CrossChainGlobalConfig>(@poly);
        config_ref.paused = false;
    }


    // initialize
    public fun init_manager(account: address, keepers: vector<vector<u8>>, startHeight: u64, polyId: u64, ctx: &mut TxContext)  {
        assert!((account) == @poly, EINVALID_SIGNER);
        
        // init access control lists
        let acls = table::new<u64, Access_control_list>(ctx);
        let admin_acl = acl::empty();
        let pause_acl = acl::empty();
        let ca_acl = acl::empty();
        let keeper_acl = acl::empty();
        acl::add(&mut admin_acl, @poly);
        acl::add(&mut pause_acl, @poly);
        acl::add(&mut ca_acl, @poly);
        acl::add(&mut keeper_acl, @poly);
        table::add(&mut acls, ADMIN_ROLE, admin_acl);
        table::add(&mut acls, PAUSE_ROLE, pause_acl);
        table::add(&mut acls, CA_ROLE, ca_acl);
        table::add(&mut acls, CHANGE_KEEPER_ROLE, keeper_acl);

        transfer(ACLStore{
            role_acls: acls,
            license_black_list: table::new<vector<u8>, u8>(ctx)
        }, account);

        // init global config
        let config = CrossChainGlobalConfig{
            polyId: polyId,
            paused: false,
            ethToPolyTxHashIndex: 0,
            curBookKeepers: keepers,
            curEpochStartHeight: startHeight,
            ethToPolyTxHashMap: table::new<u128, vector<u8>>(ctx),
            fromChainTxExist: table::new<u64, Table<vector<u8>, bool>>(ctx)
        };
        
        transfer(config, account);

        // init event store
        transfer(EventStore{
            //init_book_keeper_event: account::new_event_handle<InitBookKeeperEvent>(account),
            //change_book_keeper_event: account::new_event_handle<ChangeBookKeeperEvent>(account),
            //cross_chain_event: account::new_event_handle<CrossChainEvent>(account),
            //verify_header_and_execute_tx_event: account::new_event_handle<VerifyHeaderAndExecuteTxEvent>(account),
        }, account);

        //let event_store = borrow_global_mut<EventStore>(@poly);
        event::emit(
            InitBookKeeperEvent{
                height: startHeight,
                keepers: keepers,
            },
        );
    }

    
    // set poly id
    public entry fun setPolyId(config_ref:&mut CrossChainGlobalConfig, acl_store_ref:&mut ACLStore, account: address, polyId: u64)  {
        assert!(hasRole(acl_store_ref, CHANGE_KEEPER_ROLE, (account)), ENOT_CHANGE_KEEPER_ROLE);
        putPolyId(config_ref, polyId);
    }


    // change book keeper
    public entry fun changeBookKeeper(config_ref:&mut CrossChainGlobalConfig, acl_store_ref:&mut ACLStore, account: address, keepers: vector<vector<u8>>, startHeight: u64)  {
        assert!(hasRole(acl_store_ref, CHANGE_KEEPER_ROLE, (account)), ENOT_CHANGE_KEEPER_ROLE);
        putCurBookKeepers(config_ref, &keepers);
        putCurEpochStartHeight(config_ref, startHeight);

        //let event_store = borrow_global_mut<EventStore>(@poly);
        event::emit(
            ChangeBookKeeperEvent{
                height: startHeight,
                keepers: keepers,
            },
        );
    }

    
    // cross chain
    public fun crossChain(config_ref:&mut CrossChainGlobalConfig, acl_store_ref:&ACLStore, account: address, license: &License, toChainId: u64, toContract: &vector<u8>, method: &vector<u8>, txData: &vector<u8>)  {
        assert!(!paused(config_ref), EPAUSED);

        // check license
        let msg_sender = getLicenseId(license);
        assert!(!isBlackListedFrom(acl_store_ref, msg_sender), EBLACKLISTED_FROM);

        // pack args
        let tx_hash_index = getEthTxHashIndex(config_ref);
        let param_tx_hash = bcs::to_bytes(&tx_hash_index);
        vector::reverse(&mut param_tx_hash);

        let cross_chain_id = b"AptosCrossChainManager";
        vector::append(&mut cross_chain_id, copy param_tx_hash);
        cross_chain_id = hash::sha2_256(cross_chain_id);

        let raw_param = zero_copy_sink::write_var_bytes(&param_tx_hash);
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(&cross_chain_id));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(&msg_sender));
        vector::append(&mut raw_param, zero_copy_sink::write_u64(toChainId));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(toContract));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(method));
        vector::append(&mut raw_param, zero_copy_sink::write_var_bytes(txData));

        // mark
        putEthTxHash(config_ref, &hash::sha2_256(copy raw_param));

        // emit event
        //let event_store = borrow_global_mut<EventStore>(@poly);
        event::emit(
            CrossChainEvent{
                sender: account,
                tx_id: param_tx_hash,
                proxy_or_asset_contract: msg_sender,
                to_chain_id: toChainId,
                to_contract: *toContract,
                raw_data: raw_param,
            },
        );
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
    public fun verifyHeaderAndExecuteTx(config_ref:&CrossChainGlobalConfig,
                                        acl_store_ref:&ACLStore,
                                        license: &License,
                                        proof: &vector<u8>, rawHeader: &vector<u8>, headerProof: &vector<u8>, curRawHeader: &vector<u8>, headerSig: &vector<u8>, ctx: &mut TxContext): Certificate  {
        assert!(!paused(config_ref), EPAUSED);

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
        let keepers = getCurBookKeepers(config_ref);
        let cur_epoch_start_height = getCurEpochStartHeight(config_ref);
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
        assert!(!checkIfFromChainTxExist(config_ref, from_chain_id, &poly_tx_hash), EALREADY_EXECUTED);
        markFromChainTxExist(config_ref, from_chain_id, &poly_tx_hash, ctx);

        // check to chain id
        assert!(to_chain_id == getPolyId(config_ref), ENOT_TARGET_CHAIN);

        // check verifier
        let msg_sender = getLicenseId(license);
        assert!(msg_sender == to_contract, EVERIFIER_NOT_RECEIVER);

        // check black list
        assert!(!isBlackListedTo(acl_store_ref, to_contract), EBLACKLISTED_TO);

        // emit event
        //let event_store = borrow_global_mut<EventStore>(@poly);
        event::emit(
            VerifyHeaderAndExecuteTxEvent{
                from_chain_id: from_chain_id,
                to_contract: to_contract,
                cross_chain_tx_hash: poly_tx_hash,
                from_chain_tx_hash: source_tx_hash,
            },
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
}
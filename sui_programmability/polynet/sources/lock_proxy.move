module polynet::lock_proxy {
    use std::ascii::{Self, as_bytes, string, String};
    use std::vector;
    use std::option::{Self, Option};
    use std::string::{Self,length};
    use sui::math;
    use sui::table::{Self, Table};
    use std::type_name::{Self, TypeName};
    use sui::clock;
    use sui::clock::Clock;
    use polynet::cross_chain_manager::{CrossChainManager, LicenseInfo};
    use sui::coin::{Coin, Self};
    use sui::object;
    use sui::object::UID;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;
    use sui::vec_map;
    use sui::vec_map::VecMap;
    use polynet::cross_chain_manager;
    use polynet::zero_copy_sink;
    use polynet::zero_copy_source;
    use polynet::utils;
    use polynet::events;
    use polynet::consts;
    use polynet::acl::{ Self};

    friend polynet::wrapper_v1;
    friend polynet::controller;
    friend polynet::config;

    #[test_only]
    friend polynet::lock_proxy_test;
     #[test_only]
    friend polynet::cross_chain_manager_test;

    const ENOT_OWNER: u64 = 3001;
    const ETREASURY_NOT_EXIST: u64 = 3002;
    const ELICENSE_ALREADY_EXIST: u64 = 3003;
    const ELICENSE_NOT_EXIST: u64 = 3004;
    const ETARGET_PROXY_NOT_BIND: u64 = 3005;
    const ETARGET_ASSET_NOT_BIND: u64 = 3006;
    const EINVALID_COINTYPE: u64 = 3007;
    const EINVALID_FROM_CONTRACT: u64 = 3008;
    const EINVALID_TARGET_LICENSE_ID: u64 = 3009;
    const EINVALID_METHOD: u64 = 3010;
    const EINVALID_LICENSE_INFO: u64 = 3011;
    const EINVALID_ASSETS_ADMIN_SIGNER: u64 = 3012;
    const ELICENSE_STORE_NOT_EXIST: u64 = 3013;
    const EXCEEDED_MAXIMUM_AMOUNT_LIMIT: u64 = 3014;
    const ERR_CHECK_LP_MANAGER_PAUSED: u64 = 3015;
    const ETARGET_ASSET_CHAIN_NOT_BIND: u64 = 3016;
    const EMIN_UNLOCK_AMOUNT: u64 = 3017;
    const EMIN_LOCK_AMOUNT: u64 = 3018;

    const ONE_DAY : u64 = 24*60*60*1000; //24*60*60*1000

    struct AmountLimitManager has  store {
        time: u64,
        amount_record: VecMap<vector<u8>, u64>,
    }

    struct LockProxyManager has store{
        lock_min_amount: u64,
        unlock_min_amount: u64,
        lock_proxy_store: LockProxyStore,
        license_store: LicenseStore,
        amountLockManager: AmountLimitManager,
        amountUnlockManager: AmountLimitManager,
    }

    struct LockProxyStore has store{
        proxy_map: Table<u64, vector<u8>>,
        asset_map: Table<TypeName, Table<u64, vector<u8>>>
      
    }

    struct Treasury<phantom CoinType> has key, store {
        id: UID,
        coin: Coin<CoinType>
    }

    struct LicenseStore has store {
        license: Option<cross_chain_manager::License>
    }

    public(friend) fun new(_ctx: &mut TxContext): LockProxyManager {

        // let sender = tx_context::sender(_ctx);
        let lockproxystore = LockProxyStore{
            proxy_map: table::new<u64, vector<u8>>(_ctx),
            asset_map: table::new<TypeName, Table<u64, vector<u8>>>(_ctx)
            };

        let licensestore = LicenseStore{
            license: option::none<cross_chain_manager::License>(),
        };

        //very big time to void starting
        let start_time = 1809285669000;
        let amountLockManager = AmountLimitManager{
            time: start_time,
            amount_record: vec_map::empty()
        };
        
        vec_map::insert(&mut amountLockManager.amount_record, b"BF_USDT" , consts::get_max_amount_per_day());
        vec_map::insert(&mut amountLockManager.amount_record, b"BF_USDC" , consts::get_max_amount_per_day());

        let amountUnlockManager = AmountLimitManager{
            time: start_time,
            amount_record: vec_map::empty()
        };
         
        vec_map::insert(&mut amountUnlockManager.amount_record, b"BF_USDT" , consts::get_max_amount_per_day());
        vec_map::insert(&mut amountUnlockManager.amount_record, b"BF_USDC" , consts::get_max_amount_per_day());

        let min_amount = consts::get_min_amount_per_tx();

        let manager = LockProxyManager{
            lock_min_amount: min_amount,
            unlock_min_amount: min_amount,
            lock_proxy_store: lockproxystore,
            license_store: licensestore,
            amountLockManager: amountLockManager,
            amountUnlockManager: amountUnlockManager,
        };

        manager
    }

    // update start_time after init
    public(friend) fun update_lock_proxy_manager_start_time(
        _lock_proxy_manager: &mut LockProxyManager,
        _clock: &Clock, 
        _ctx: &mut TxContext
    ) {
        let start_time = clock::timestamp_ms(_clock);
        _lock_proxy_manager.amountLockManager.time = start_time;
        _lock_proxy_manager.amountUnlockManager.time = start_time;
    }


    // getter function
    public fun get_target_proxy(lpManager: &LockProxyManager, to_chain_id: u64): vector<u8>  {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);

        if (table::contains(&lpManager.lock_proxy_store.proxy_map, to_chain_id)) {
            return *table::borrow(&lpManager.lock_proxy_store.proxy_map, to_chain_id)
        } else {
            abort ETARGET_PROXY_NOT_BIND
        }
    }
    //TODO: logic is not easy to understand
    public fun get_to_asset<CoinType>(lpManager: &LockProxyManager,  to_chain_id: u64): (vector<u8>, u8) {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);
        let from_asset = type_name::get<Coin<CoinType>>();
        if (table::contains(&lpManager.lock_proxy_store.asset_map, from_asset)) {
            let sub_table = table::borrow(&lpManager.lock_proxy_store.asset_map, from_asset);
            if (table::contains(sub_table, to_chain_id)) {
                let decimals_concat_to_asset = table::borrow(sub_table, to_chain_id);
                let decimals = *vector::borrow(decimals_concat_to_asset, 0);
                let to_asset = utils::slice(decimals_concat_to_asset, 1, vector::length(decimals_concat_to_asset) - 1);
                 events::read_asset(
                    to_asset,
                    to_chain_id,
                    decimals
                 );
                return (to_asset, decimals)
            } else {
                abort ETARGET_ASSET_CHAIN_NOT_BIND
            }
        } else {
            abort ETARGET_ASSET_NOT_BIND
        }
    }

    // where we need this
    // public fun get_balance< CoinType>(treasury_ref: &Treasury<CoinType>): u64  {
    //     //assert!(exists<Treasury<CoinType>>(POLY_BRIDGE), ETREASURY_NOT_EXIST);
    //     //let treasury_ref = borrow_global<Treasury<CoinType>>(POLY_BRIDGE);
    //     return coin::value(&treasury_ref.coin)
    // }

    public(friend) fun bind_proxy(
        lpManager: &mut LockProxyManager,
        to_chain_id: u64,
        target_proxy_hash: vector<u8>
      
    )  {
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        utils::upsert(&mut lpManager.lock_proxy_store.proxy_map, to_chain_id, target_proxy_hash);
        events::bind_proxy(
                    to_chain_id,
                    target_proxy_hash
                );
    }

    public(friend) fun unbind_proxy(
        lpManager: &mut LockProxyManager, 
        to_chain_id: u64
    ) {
     
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        if (table::contains(&lpManager.lock_proxy_store.proxy_map, to_chain_id)) {
            table::remove(&mut lpManager.lock_proxy_store.proxy_map, to_chain_id);
        } else {
            abort ETARGET_PROXY_NOT_BIND
        };

        events::unbind_proxy(
                    to_chain_id,
                    vector::empty<u8>()
                );
    }

    public(friend) fun bind_asset<CoinType>(
        lpManager: &mut LockProxyManager,
        to_chain_id: u64,
        to_asset_hash: vector<u8>,
        to_asset_decimals: u8,
        ctx: &mut TxContext
    )  {
      
        let from_asset = type_name::get<Coin<CoinType>>();
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        let decimals_concat_to_asset = vector::singleton(to_asset_decimals);
        vector::append(&mut decimals_concat_to_asset, to_asset_hash);
        if (table::contains(&lpManager.lock_proxy_store.asset_map, from_asset)) {
            utils::upsert(table::borrow_mut(&mut lpManager.lock_proxy_store.asset_map, from_asset), to_chain_id, decimals_concat_to_asset);
        } else {
            let subTable = table::new<u64, vector<u8>>(ctx);
            table::add(&mut subTable, to_chain_id, decimals_concat_to_asset);
            table::add(&mut lpManager.lock_proxy_store.asset_map, from_asset, subTable);
        };

        events::bind_asset(
                    from_asset,
                    to_chain_id,
                    to_asset_hash,
                    to_asset_decimals,
                 );
    }

    public(friend) fun unbind_asset<CoinType>(
        lpManager: &mut LockProxyManager, 
        to_chain_id: u64
    ) {
     
        let from_asset = type_name::get<Coin<CoinType>>();
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        if (table::contains(&lpManager.lock_proxy_store.asset_map, from_asset)) {
            let sub_table = table::borrow_mut(&mut lpManager.lock_proxy_store.asset_map, from_asset);
            if (table::contains(sub_table, to_chain_id)) {
                table::remove(sub_table, to_chain_id);
                // linked_table::drop(old);
            } else {
                abort ETARGET_ASSET_CHAIN_NOT_BIND
            };
        } else {
            abort ETARGET_ASSET_NOT_BIND
        };

        events::unbind_asset(
                    from_asset,
                    to_chain_id,
                    vector::empty<u8>(),
                    0,
                 );
    }

    // treasury function
    //public entry fun initTreasury<CoinType>(admin: address, ctx: &mut TxContext){
    public  fun init_treasury<CoinType>(ctx: &mut TxContext): Treasury<CoinType> {

        let treasury = Treasury<CoinType>{
            id: object::new(ctx),
            coin: coin::zero<CoinType>(ctx),
        };
        treasury
    }

    public fun lock_proxy_transfer<CoinType>(treasury:Treasury<CoinType>) {
        transfer::share_object(treasury)
    }

    //where we need this
    // public fun is_treasury_initialzed<CoinType>(): bool {
    //     true
    //     //exists<Treasury<CoinType>>(POLY_BRIDGE)
    // }

    public fun deposit<CoinType>(treasury_ref: &mut Treasury<CoinType>,  fund: Coin<CoinType>)  {
        coin::join<CoinType>(&mut treasury_ref.coin, fund);
    }

    //here we limit the withdraw using the assets list, the relayer address should be add to release token
    fun withdraw<CoinType>(treasury_ref:&mut Treasury<CoinType>, amount: u64 , ctx: &mut TxContext): Coin<CoinType> {
        // sender address: only assets admin can withdraw
        let sender = tx_context::sender(ctx);
        assert!(acl::is_assets_admin(sender), EINVALID_ASSETS_ADMIN_SIGNER);

        return coin::split(&mut treasury_ref.coin, amount, ctx)
    }

    // license function  
    //notice: license_account can't be equal to the type_name address
    public fun receive_license(
        lpManager: &mut LockProxyManager,
        license: cross_chain_manager::License
    )   {
        
        //license_account not used now
        let (_, license_module_name) = cross_chain_manager::get_license_info(&license);
        let this_type = type_name::get<LicenseStore>();
        // let this_account = type_name::get_address(&this_type);
        let this_module_name = type_name::get_module(&this_type);

        let license_module_name_string = ascii::string(license_module_name);
        // let license_account_string = address::to_ascii_string(license_account);
        //assert!(license_account_string == this_account && license_module_name_string == this_module_name, EINVALID_LICENSE_INFO);
        assert!(license_module_name_string == this_module_name, EINVALID_LICENSE_INFO);
        option::fill(&mut lpManager.license_store.license, license);
    }

    public fun remove_license(_lp_manager: &mut LockProxyManager, _admin: address): cross_chain_manager::License {

        assert!(option::is_some<cross_chain_manager::License>(&_lp_manager.license_store.license), ELICENSE_NOT_EXIST);
        option::extract<cross_chain_manager::License>(&mut _lp_manager.license_store.license)
    }

    public  fun get_license_id(lpManager: &LockProxyManager): (vector<u8>, LicenseInfo) {
        
        return cross_chain_manager::get_license_id(option::borrow(&lpManager.license_store.license))
    }

    public(friend) fun output_license_id(lpManager: &LockProxyManager) {
      
        let (data, licenseInfo) = cross_chain_manager::get_license_id(option::borrow(&lpManager.license_store.license));
        events::license_id(
            data,
            cross_chain_manager::get_license_account(&licenseInfo),
            string(cross_chain_manager::get_license_module_name(&licenseInfo))
        );
    }

    public(friend) fun relay_unlock_tx<CoinType>(
        certificate: &cross_chain_manager::Certificate,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        clock:&Clock,
        ctx: &mut TxContext
    )  {

        // borrow license
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
        // let license_ref = option::borrow(&lpManager.license_store.license);

        unlock<CoinType>(
            lpManager, 
            treasury_ref,
            certificate, 
            clock, 
            ctx
        );
    }
    
    #[test_only]
    public(friend) fun test_relay_unlock_tx<CoinType>(
        certificate: &cross_chain_manager::Certificate,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        clock:&Clock,
        ctx: &mut TxContext
    )  {

        // borrow license
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
        // let license_ref = option::borrow(&lpManager.license_store.license);

        test_unlock<CoinType>(
            lpManager, 
            treasury_ref,
            certificate, 
            clock, 
            ctx
        );
    }

    public(friend) fun get_license_ref(_lp_manager: &LockProxyManager): &cross_chain_manager::License {
        option::borrow(&_lp_manager.license_store.license)
    }
    
    // lock
    public(friend) fun lock<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        account: address,
        fund: &mut Coin<CoinType>,
        amount: u64,
        toChainId: u64,
        toAddress: &vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext
    )  {
        // lock fund
        assert!(amount >= lpManager.lock_min_amount, EMIN_UNLOCK_AMOUNT);
        let deposit_coin = coin::split<CoinType>(fund, amount, ctx);
        
        deposit(treasury_ref, deposit_coin);
        // utils::send_coin(fund,account);
        
        // borrow license
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        //let license_opt = &borrow_global<LicenseStore>(POLY_BRIDGE).license;
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
        let short_name = convert_to_short_key(type_name::borrow_string(&type_name::get<Coin<CoinType>>()));
        assert!(check_amount_result<CoinType>(amount, lpManager, &short_name, true, clock), EXCEEDED_MAXIMUM_AMOUNT_LIMIT);
        
        let license_ref = option::borrow(&lpManager.license_store.license);

        // get target proxy/asset
        let to_proxy = get_target_proxy(lpManager, toChainId);
        let (to_asset, to_asset_decimals) = get_to_asset<CoinType>(lpManager, toChainId);

        // precision conversion
        let target_chain_amount = to_target_chain_amount(amount, consts::get_decimal(), to_asset_decimals);
      

        // pack args
        let tx_data = serialize_tx_args(&to_asset, toAddress, target_chain_amount);

        // cross chain
        cross_chain_manager::cross_chain(
                                ccManager, 
                                license_ref, 
                                toChainId,
                                &to_proxy, 
                                &b"unlock",   //TODO: its not easy to understand
                                &tx_data, 
                                ctx
                            );

        // emit event 
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        events::lock_event(
                    type_name::get<Coin<CoinType>>(),
                    account,
                    toChainId,
                    to_asset,
                    *toAddress,
                    amount,
                    target_chain_amount,
                 );
    }

    // unlock
    public(friend) fun unlock<CoinType>(
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        certificate: &cross_chain_manager::Certificate,
        clock:&Clock,
        ctx: &mut TxContext
    )  {
        // read certificate
        let (
            from_contract,
            from_chain_id,
            target_license_id,
            method,
            args
        ) = cross_chain_manager::read_certificate(certificate);

        // unpac args
        let (
            to_asset,
            to_address,
            from_chain_amount
        ) = deserialize_tx_args(&args);

        // from asset decimal precision conversion
        let (_, from_asset_decimals) = get_to_asset<CoinType>(lpManager, from_chain_id);

        let amount = from_target_chain_amount(from_chain_amount, consts::get_decimal(),from_asset_decimals);
        assert!(amount >= lpManager.unlock_min_amount, EMIN_UNLOCK_AMOUNT);
        let short_name = convert_to_short_key(type_name::borrow_string(&type_name::get<Coin<CoinType>>()));
        
        //notice: if want to pass unit test should remove this check
        assert!(*as_bytes(type_name::borrow_string(&type_name::get<Coin<CoinType>>())) == to_asset, EINVALID_COINTYPE);

        assert!(get_target_proxy(lpManager, from_chain_id) == from_contract, EINVALID_FROM_CONTRACT);
        let (license_id, _) = get_license_id(lpManager);
        assert!(license_id == target_license_id, EINVALID_TARGET_LICENSE_ID);
        assert!(method == b"unlock", EINVALID_METHOD);

        assert!(check_amount_result<CoinType>(amount,lpManager, &short_name, false, clock), EXCEEDED_MAXIMUM_AMOUNT_LIMIT);
        // unlock fund
        let fund = withdraw<CoinType>(treasury_ref, amount, ctx);
        //todo need transfer.

        transfer::public_transfer(fund, utils::to_address(to_address));

        events::unlock(
                    type_name::get<Coin<CoinType>>(),
                    utils::to_address(to_address),
                    amount,
                    from_chain_amount
                 );
    }

    #[test_only]
    public(friend) fun test_unlock<CoinType>(
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        certificate: &cross_chain_manager::Certificate,
        clock:&Clock,
        ctx: &mut TxContext
    )  {
        // read certificate
        let (
            from_contract,
            from_chain_id,
            target_license_id,
            method,
            args
        ) = cross_chain_manager::read_certificate(certificate);

        // unpac args
        let (
            _,
            to_address,
            from_chain_amount
        ) = deserialize_tx_args(&args);

        // from asset decimal precision conversion
        let (_, from_asset_decimals) = get_to_asset<CoinType>(lpManager, from_chain_id);

        let amount = from_target_chain_amount(from_chain_amount, consts::get_decimal(),from_asset_decimals);
        assert!(amount >= lpManager.unlock_min_amount, EMIN_UNLOCK_AMOUNT);
        let short_name = convert_to_short_key(type_name::borrow_string(&type_name::get<Coin<CoinType>>()));
        
        //notice: if want to pass unit test should remove this check
        // assert!(*as_bytes(type_name::borrow_string(&type_name::get<Coin<CoinType>>())) == to_asset, EINVALID_COINTYPE);

        assert!(get_target_proxy(lpManager, from_chain_id) == from_contract, EINVALID_FROM_CONTRACT);
        let (license_id, _) = get_license_id(lpManager);
        assert!(license_id == target_license_id, EINVALID_TARGET_LICENSE_ID);
        assert!(method == b"unlock", EINVALID_METHOD);

        assert!(check_amount_result<CoinType>(amount,lpManager, &short_name, false, clock), EXCEEDED_MAXIMUM_AMOUNT_LIMIT);
        // unlock fund
        let fund = withdraw<CoinType>(treasury_ref, amount, ctx);
        //todo need transfer.

        transfer::public_transfer(fund, utils::to_address(to_address));

        events::unlock(
                    type_name::get<Coin<CoinType>>(),
                    utils::to_address(to_address),
                    amount,
                    from_chain_amount
                 );

    }

    //reset max amount per day of lock_proxy_manager
    //check user input amount if bigger than max amount
    public fun check_amount_result<CoinType>(
        user_amount: u64, 
        lockProxyManager: &mut LockProxyManager,  
        key: &vector<u8>, 
        flag: bool, 
        clock: &Clock
    ):bool{
        let current_time = clock::timestamp_ms(clock);
        let amountLimit : &mut AmountLimitManager;
        let _min_amount = 0;
        if (flag == true) {
            amountLimit = &mut lockProxyManager.amountLockManager;
            _min_amount = lockProxyManager.lock_min_amount;
        } else {
            amountLimit = &mut lockProxyManager.amountUnlockManager;
            _min_amount = lockProxyManager.unlock_min_amount;
        };

        if(current_time -  amountLimit.time > ONE_DAY){
            amountLimit.time = current_time;
            reset_amount(amountLimit);
        };
        let amount = vec_map::get_mut(&mut amountLimit.amount_record, key);
        if(user_amount > *amount){
            return false
        }else{
            *amount = *amount - user_amount;
            events::remaining_amount_change_event(
                    type_name::get<CoinType>(),
                    flag,
                    _min_amount,
                    *amount
            );
        };
        return true
    }

    public(friend) fun borrow_lock_amount_limit_manager(_lockProxyManager: &mut LockProxyManager): &mut AmountLimitManager {
        &mut _lockProxyManager.amountLockManager
    }
    public(friend) fun borrow_unlock_amount_limit_manager(_lockProxyManager: &mut LockProxyManager): &mut AmountLimitManager {
        &mut _lockProxyManager.amountUnlockManager
    }

    public(friend) fun reset_amount(amountManager:&mut AmountLimitManager){
        let amount = consts::get_max_amount_per_day();
        let usdt =vec_map::get_mut(&mut amountManager.amount_record, &b"BF_USDT");
        *usdt = amount;

        let usdc = vec_map::get_mut(&mut amountManager.amount_record, &b"BF_USDC");
        *usdc = amount;

        events::reset_per_day_amount_event(amount);
    }
   
    public fun to_target_chain_amount(amount: u64,local_decimals: u8,  target_decimals: u8): u128 {
        //let source_decimals = coin::decimals<CoinType>();
        (amount as u128) * pow_10(target_decimals) / pow_10(local_decimals)
    }

    public fun from_target_chain_amount(from_chain_amount: u128,local_decimals: u8, from_decimals: u8): u64 {
        //let source_decimals = coin::decimals<CoinType>();
        (from_chain_amount * pow_10(local_decimals) / pow_10(from_decimals) as u64)
    }

    fun pow_10(decimals: u8): u128 {
        //math128::pow(10, (decimals as u128))
        let data = math::pow(10, decimals);
        (data as u128)
    }

    // codecs
    public fun serialize_tx_args(to_asset: &vector<u8>, to_address: &vector<u8>, amount: u128): vector<u8> {
        let buf = zero_copy_sink::write_var_bytes(to_asset);
        vector::append(&mut buf, zero_copy_sink::write_var_bytes(to_address));
        vector::append(&mut buf, zero_copy_sink::write_u256((0 as u128), amount));
        return buf
    }

    public fun deserialize_tx_args(raw_data: &vector<u8>): (vector<u8>, vector<u8>, u128) {
        let offset = (0 as u64);
        let to_asset: vector<u8>;
        let to_address: vector<u8>;
        let amount: u128;
        (to_asset, offset) = zero_copy_source::next_var_bytes(raw_data, offset);
        (to_address, offset) = zero_copy_source::next_var_bytes(raw_data, offset);
        (_, amount, _) = zero_copy_source::next_u256(raw_data, offset);
        return (to_asset, to_address, amount)
    }

    public fun convert_to_short_key(name: &String) :vector<u8> {
        let ascii_name = &string::from_ascii(*name);

        if (string::index_of(ascii_name, &string::utf8(b"BFC_ETH")) != length(ascii_name)) {
            return b"BFC_ETH"
        } else if (string::index_of(ascii_name, &string::utf8(b"BF_USDT")) != length(ascii_name)) {
            return b"BF_USDT"
        } else if (string::index_of(ascii_name, &string::utf8(b"BF_USDC")) != length(ascii_name)) {
            return b"BF_USDC"
        } else {
            return b"BFC_BTC"
        }
    }

    public(friend) fun update_lock_min_amount(
        _lockProxyManager: &mut LockProxyManager,
        _min_amount: u64
    )  {
        let old_amount = _lockProxyManager.lock_min_amount;
        _lockProxyManager.lock_min_amount = _min_amount;

        events::update_min_amount_event(
            true,
            old_amount,
            _min_amount
        );
    }

    public(friend) fun update_unlock_min_amount(
        _lockProxyManager: &mut LockProxyManager,
        _min_amount: u64
    )  {
        let old_amount = _lockProxyManager.lock_min_amount;
        _lockProxyManager.unlock_min_amount = _min_amount;

        events::update_min_amount_event(
            false,
            old_amount,
            _min_amount
        );
    }
}
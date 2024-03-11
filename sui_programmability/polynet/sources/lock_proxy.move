#[allow(unused_field,unused_assignment,unused_type_parameter)]
module polynet::lock_proxy {
    use std::ascii;
    use std::ascii::{as_bytes, string, String};
    use std::vector;
    use std::option::{Self, Option};
    use std::string;
    use std::string::length;
    use sui::event;
    use sui::math;
    use sui::table::{Table, Self};
    use std::type_name::{Self, TypeName};
    use sui::address;
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


    friend polynet::wrapper_v1;
    friend polynet::controller;
    friend polynet::config;

    #[test_only]
    friend polynet::lock_proxy_test;


    const DEPRECATED: u64 = 4001;
    const ENOT_OWNER: u64 = 4002;
    const ETREASURY_ALREADY_EXIST: u64 = 4003;
    const ETREASURY_NOT_EXIST: u64 = 4004;
    const ELICENSE_ALREADY_EXIST: u64 = 4005;
    const ELICENSE_NOT_EXIST: u64 = 4006;
    const ETARGET_PROXY_NOT_BIND: u64 = 4007;
    const ETARGET_ASSET_NOT_BIND: u64 = 4008;
    const EINVALID_COINTYPE: u64 = 4009;
    const EINVALID_FROM_CONTRACT: u64 = 4010;
    const EINVALID_TARGET_LICENSE_ID: u64 = 4011;
    const EINVALID_METHOD: u64 = 4012;
    const ELICENSE_STORE_ALREADY_EXIST: u64 = 4013;
    const EINVALID_LICENSE_INFO: u64 = 4014;
    const EINVALID_ADMIN_SIGNER: u64 = 4015;
    const EINVALID_ASSETS_ADMIN_SIGNER: u64 = 4016;
    const ELICENSE_STORE_NOT_EXIST: u64 = 4017;

    const EXCEEDED_MAXIMUM_AMOUNT_LIMIT: u64 = 4018;
    const ERR_CHECK_LP_MANAGER_PAUSED: u64 = 4019;



    const MAX_AMOUNT: u64 = 100*10000*100000000; //1 million.

    const ONE_DAY : u64 = 24*60*60*1000; //24*60*60*1000

    struct AmountLimitManager has  store {
        // id: UID,
        time: u64,
        amount_record: VecMap<vector<u8>, u64>,
    }


    struct LockProxyManager has store{
        // id: UID,
        lock_proxy_store: LockProxyStore,
        license_store: LicenseStore,
        amountLockManager: AmountLimitManager,
        amountUnlockManager: AmountLimitManager,
    }

    struct LockProxyStore has store{
        // id: UID,
        proxy_map: Table<u64, vector<u8>>,
        asset_map: Table<TypeName, Table<u64, vector<u8>>>,
        paused: bool,
        owner: address,
    }

    struct Treasury<phantom CoinType> has key, store {
        id: UID,
        coin: Coin<CoinType>
    }

    struct LicenseStore has store {
        // id: UID,
        license: Option<cross_chain_manager::License>
    }


    // events
    struct BindProxyEvent has store, drop, copy {
        to_chain_id: u64,
        target_proxy_hash: vector<u8>
    }
    struct BindAssetEvent has store, drop, copy {
        from_asset: TypeName,
        to_chain_id: u64,
        to_asset_hash: vector<u8>,
        to_asset_decimals: u8,
    }
    struct UnlockEvent has store, drop, copy {
        to_asset: TypeName,
        to_address: address,
        amount: u64,
        from_chain_amount: u128,
    }

    struct LicenseIdEvent has store, drop, copy {
        license_id: vector<u8>,
        account: address,
        module_name: String,
    }

    public(friend) fun new(_ctx: &mut TxContext): LockProxyManager {

        let sender = tx_context::sender(_ctx);

        assert!(utils::is_admin(sender), EINVALID_ADMIN_SIGNER);

        let lockproxystore = LockProxyStore{
            // id: object::new(_ctx),
            proxy_map: table::new<u64, vector<u8>>(_ctx),
            asset_map: table::new<TypeName, Table<u64, vector<u8>>>(_ctx),
            paused: false,
            owner: (sender),
            };

        let licensestore = LicenseStore{
            // id: object::new(_ctx),
            license: option::none<cross_chain_manager::License>(),
        };

        //very big time to void starting
        let start_time = 1809285669000;
        let amountLockManager = AmountLimitManager{
            // id: object::new(_ctx),
            time: start_time,
            amount_record: vec_map::empty()
        };
        vec_map::insert(&mut amountLockManager.amount_record, b"BFC_USDT" , MAX_AMOUNT);
        vec_map::insert(&mut amountLockManager.amount_record, b"BFC_USDC" , MAX_AMOUNT);
        vec_map::insert(&mut amountLockManager.amount_record, b"BFC_BTC" , MAX_AMOUNT);
        vec_map::insert(&mut amountLockManager.amount_record, b"BFC_ETH" , MAX_AMOUNT);

        let amountUnlockManager = AmountLimitManager{
            // id: object::new(_ctx),
            time: start_time,
            amount_record: vec_map::empty()
        };
        vec_map::insert(&mut amountUnlockManager.amount_record, b"BFC_USDT" , MAX_AMOUNT);
        vec_map::insert(&mut amountUnlockManager.amount_record, b"BFC_USDC" , MAX_AMOUNT);
        vec_map::insert(&mut amountUnlockManager.amount_record, b"BFC_BTC" , MAX_AMOUNT);
        vec_map::insert(&mut amountUnlockManager.amount_record, b"BFC_ETH" , MAX_AMOUNT);

        let manager = LockProxyManager{
            // id: object::new(_ctx),
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
    public fun getTargetProxy(lpManager: &LockProxyManager, to_chain_id: u64): vector<u8>  {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);

        if (table::contains(&lpManager.lock_proxy_store.proxy_map, to_chain_id)) {
            return *table::borrow(&lpManager.lock_proxy_store.proxy_map, to_chain_id)
        } else {
            abort ETARGET_PROXY_NOT_BIND
        }
    }
    //TODO: logic is not easy to understand
    public fun getToAsset<CoinType>(lpManager: &LockProxyManager,  to_chain_id: u64): (vector<u8>, u8) {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);
        let from_asset = type_name::get<Coin<CoinType>>();
        if (table::contains(&lpManager.lock_proxy_store.asset_map, from_asset)) {
            let sub_table = table::borrow(&lpManager.lock_proxy_store.asset_map, from_asset);
            if (table::contains(sub_table, to_chain_id)) {
                let decimals_concat_to_asset = table::borrow(sub_table, to_chain_id);
                let decimals = *vector::borrow(decimals_concat_to_asset, 0);
                let to_asset = utils::slice(decimals_concat_to_asset, 1, vector::length(decimals_concat_to_asset) - 1);
                return (to_asset, decimals)
            } else {
                abort ETARGET_ASSET_NOT_BIND
            }
        } else {
            abort ETARGET_ASSET_NOT_BIND
        }
    }

    public fun paused(lpManager: &LockProxyManager): bool   {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);
        return lpManager.lock_proxy_store.paused
    }

    public fun owner(lpManager: &LockProxyManager): address {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);
        return lpManager.lock_proxy_store.owner
    }

    public fun getBalance< CoinType>(treasury_ref: &Treasury<CoinType>): u64  {
        //assert!(exists<Treasury<CoinType>>(POLY_BRIDGE), ETREASURY_NOT_EXIST);
        //let treasury_ref = borrow_global<Treasury<CoinType>>(POLY_BRIDGE);
        return coin::value(&treasury_ref.coin)
    }


    // owner function
    public(friend) fun onlyOwner(lpManager: &LockProxyManager, owner: address) {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);
        assert!((owner) == lpManager.lock_proxy_store.owner, ENOT_OWNER);
    }

    public(friend) fun transferOwnerShip(
        lpManager: &mut LockProxyManager, 
        new_owner: address
    ) {
      
        lpManager.lock_proxy_store.owner = new_owner;
    }

    public(friend) fun pause(lpManager: &mut LockProxyManager) {

        lpManager.lock_proxy_store.paused = true;
    }

    public(friend) fun unpause(lpManager: &mut LockProxyManager) {
        // assert!(paused(lpManager),ERR_CHECK_LP_MANAGER_PAUSED);
        lpManager.lock_proxy_store.paused = false;
    }

    public(friend) fun check_paused(lpManager: &LockProxyManager) {
         assert!(!paused(lpManager),ERR_CHECK_LP_MANAGER_PAUSED);
    }

    public(friend) fun bind_proxy(
        lpManager: &mut LockProxyManager,
        to_chain_id: u64,
        target_proxy_hash: vector<u8>
      
    )  {
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        utils::upsert(&mut lpManager.lock_proxy_store.proxy_map, to_chain_id, target_proxy_hash);

        event::emit(
            BindProxyEvent{
                to_chain_id: to_chain_id,
                target_proxy_hash,
            },
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

        event::emit(
            BindProxyEvent{
                to_chain_id: to_chain_id,
                target_proxy_hash: vector::empty<u8>(),
            },
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

        event::emit(
            BindAssetEvent{
                from_asset: from_asset,
                to_chain_id: to_chain_id,
                to_asset_hash,
                to_asset_decimals: to_asset_decimals,
            },
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
            } else {
                abort ETARGET_ASSET_NOT_BIND
            };
        } else {
            abort ETARGET_ASSET_NOT_BIND
        };

        event::emit(
            BindAssetEvent{
                from_asset: from_asset,
                to_chain_id: to_chain_id,
                to_asset_hash: vector::empty<u8>(),
                to_asset_decimals: 0,
            },
        );
    }

    // treasury function
    //public entry fun initTreasury<CoinType>(admin: address, ctx: &mut TxContext){
    public  fun initTreasury<CoinType>(ctx: &mut TxContext): Treasury<CoinType> {


        let treasury = Treasury<CoinType>{
            id: object::new(ctx),
            coin: coin::zero<CoinType>(ctx),
        };

        treasury

    }

    public fun lock_proxy_transfer<CoinType>(treasury:Treasury<CoinType>) {
        transfer::share_object(treasury)
    }

    public fun is_treasury_initialzed<CoinType>(): bool {
        true
        //exists<Treasury<CoinType>>(POLY_BRIDGE)
    }

    public fun is_admin(account: address): bool {
        utils::is_admin(account)
    }

    public fun deposit<CoinType>(treasury_ref: &mut Treasury<CoinType>,  fund: Coin<CoinType>)  {
        coin::join<CoinType>(&mut treasury_ref.coin, fund);
    }

    //todo. need more strick root right checking. admin has too many accounts.
    fun withdraw<CoinType>(treasury_ref:&mut Treasury<CoinType>, amount: u64 , ctx: &mut TxContext): Coin<CoinType> {
        // sender address: only assets admin can withdraw
        let sender = tx_context::sender(ctx);
        assert!(utils::is_assets_admin(sender), EINVALID_ASSETS_ADMIN_SIGNER);

        return coin::split(&mut treasury_ref.coin, amount, ctx)
    }




    // license function
    public fun receiveLicense(
        lpManager: &mut LockProxyManager,
        license: cross_chain_manager::License
    )   {
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_STORE_NOT_EXIST);
        //let license_opt = &mut borrow_global_mut<LicenseStore>(POLY_BRIDGE).license;
        //assert!(option::is_none<cross_chain_manager::License>(license_opt), ELICENSE_ALREADY_EXIST);

        let (license_account, license_module_name) = cross_chain_manager::getLicenseInfo(&license);
        let this_type = type_name::get<LicenseStore>();
        let this_account = type_name::get_address(&this_type);
        let this_module_name = type_name::get_module(&this_type);

        //todo
        let license_module_name_string = ascii::string(license_module_name);
        let license_account_string = address::to_ascii_string(license_account);
        //assert!(license_account_string == this_account && license_module_name_string == this_module_name, EINVALID_LICENSE_INFO);
        assert!(license_module_name_string == this_module_name, EINVALID_LICENSE_INFO);
        option::fill(&mut lpManager.license_store.license, license);
    }

    public fun removeLicense(lpManager: &mut LockProxyManager, admin: address): cross_chain_manager::License {
        assert!(utils::is_admin(admin), EINVALID_ADMIN_SIGNER);
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        //let license_opt = &mut borrow_global_mut<LicenseStore>(POLY_BRIDGE).license;
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
        option::extract<cross_chain_manager::License>(&mut lpManager.license_store.license)
    }

    public  fun getLicenseId(lpManager: &LockProxyManager): (vector<u8>, LicenseInfo) {
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        //let license_opt = &borrow_global<LicenseStore>(POLY_BRIDGE).license;
        //assert!(option::is_some<cross_chain_manager::License>(license_opt), ELICENSE_NOT_EXIST);
        return cross_chain_manager::getLicenseId(option::borrow(&lpManager.license_store.license))
    }

    public(friend) fun output_license_id(lpManager: &LockProxyManager) {
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        //let license_opt = &borrow_global<LicenseStore>(POLY_BRIDGE).license;
        //assert!(option::is_some<cross_chain_manager::License>(license_opt), ELICENSE_NOT_EXIST);
        let (data, licenseInfo) = cross_chain_manager::getLicenseId(option::borrow(&lpManager.license_store.license));
        event::emit(
            LicenseIdEvent{
                license_id: data,
                account: cross_chain_manager::get_license_account(&licenseInfo),
                module_name:  string(cross_chain_manager::get_license_module_name(&licenseInfo)),
            }
        )
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

    public(friend) fun get_license_ref(_lp_manager: &LockProxyManager): &cross_chain_manager::License {
        option::borrow(&_lp_manager.license_store.license)
    }
    

    // lock
    public(friend) fun lock<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        account: address,
        fund: Coin<CoinType>,
        toChainId: u64,
        toAddress: &vector<u8>,
        clock:&Clock,
        ctx: &mut TxContext
    )  {
        // lock fund
        let amount = coin::value(&fund);
        deposit(treasury_ref, fund);
        
        // borrow license
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        //let license_opt = &borrow_global<LicenseStore>(POLY_BRIDGE).license;
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
        let short_name = convert_to_short_key(type_name::borrow_string(&type_name::get<Coin<CoinType>>()));
        assert!(checkAmountResult(amount, lpManager, &short_name, true, clock), EXCEEDED_MAXIMUM_AMOUNT_LIMIT);
        
        let license_ref = option::borrow(&lpManager.license_store.license);

        // get target proxy/asset
        let to_proxy = getTargetProxy(lpManager, toChainId);
        let (to_asset, to_asset_decimals) = getToAsset<CoinType>(lpManager, toChainId);

        //todo,, decimals
        // precision conversion
        let target_chain_amount = to_target_chain_amount(amount, to_asset_decimals, to_asset_decimals);

        // pack args
        let tx_data = serializeTxArgs(&to_asset, toAddress, target_chain_amount);

        // cross chain
        cross_chain_manager::crossChain(
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
        ) = deserializeTxArgs(&args);

        // precision conversion
        let (_, decimals) = getToAsset<CoinType>(lpManager, from_chain_id);


        //todo, decimals
        let amount = from_target_chain_amount(from_chain_amount, decimals, decimals);
        let short_name = convert_to_short_key(type_name::borrow_string(&type_name::get<Coin<CoinType>>()));
        //todo, decimals
        //type_name::get<Coin<CoinType>>()

        assert!(*as_bytes(type_name::borrow_string(&type_name::get<Coin<CoinType>>())) == to_asset, EINVALID_COINTYPE);

        assert!(getTargetProxy(lpManager, from_chain_id) == from_contract, EINVALID_FROM_CONTRACT);
        let (license_id, _) = getLicenseId(lpManager);
        assert!(license_id == target_license_id, EINVALID_TARGET_LICENSE_ID);
        assert!(method == b"unlock", EINVALID_METHOD);

        assert!(checkAmountResult(amount,lpManager, &short_name, false, clock), EXCEEDED_MAXIMUM_AMOUNT_LIMIT);
        // unlock fund
        let fund = withdraw<CoinType>(treasury_ref, amount, ctx);
        //todo need transfer.

        transfer::public_transfer(fund, utils::to_address(to_address));

        // emit event
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        event::emit(
            UnlockEvent{
                to_asset: type_name::get<Coin<CoinType>>(),
                to_address: utils::to_address(to_address),
                amount,
                from_chain_amount
            },
        );
    }

   
    //reset max amount per day of lock_proxy_manager
    //check user input amount if bigger than max amount
    public fun checkAmountResult(user_amount: u64, lockProxyManager: &mut LockProxyManager,  key:&vector<u8>, flag: bool, clock:&Clock):bool{
        let current_time = clock::timestamp_ms(clock);
        let amountLimit : &mut AmountLimitManager;
        if (flag == true) {
            amountLimit = &mut lockProxyManager.amountLockManager;
        } else {
            amountLimit = &mut lockProxyManager.amountUnlockManager;
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
        let usdt =vec_map::get_mut(&mut amountManager.amount_record, &b"BFC_USDT");
        *usdt = MAX_AMOUNT;

        let usdc = vec_map::get_mut(&mut amountManager.amount_record, &b"BFC_USDC");
        *usdc = MAX_AMOUNT;

        let btc = vec_map::get_mut(&mut amountManager.amount_record, &b"BFC_BTC");
        *btc = MAX_AMOUNT;

        let eth = vec_map::get_mut(&mut amountManager.amount_record, &b"BFC_ETH");
        *eth = MAX_AMOUNT;
    }

  

    // entry fun resetAmountByAdmin(admin : address, amountManager : &mut AmountLimitManager){
    //     assert!(utils::is_admin(admin), EINVALID_ADMIN_SIGNER);
    //     resetAmount(amountManager);
    // }

   

    // decimals conversion
    public fun to_target_chain_amount(amount: u64,source_decimals: u8,  target_decimals: u8): u128 {
        //let source_decimals = coin::decimals<CoinType>();
        (amount as u128) * pow_10(target_decimals) / pow_10(source_decimals)
    }
    public fun from_target_chain_amount(target_chain_amount: u128,source_decimals: u8, target_decimals: u8): u64 {
        //let source_decimals = coin::decimals<CoinType>();
        (target_chain_amount * pow_10(source_decimals) / pow_10(target_decimals) as u64)
    }
    fun pow_10(decimals: u8): u128 {
        //math128::pow(10, (decimals as u128))
        let data = math::pow(10, decimals);
        (data as u128)
    }


    // codecs
    public fun serializeTxArgs(to_asset: &vector<u8>, to_address: &vector<u8>, amount: u128): vector<u8> {
        let buf = zero_copy_sink::write_var_bytes(to_asset);
        vector::append(&mut buf, zero_copy_sink::write_var_bytes(to_address));
        vector::append(&mut buf, zero_copy_sink::write_u256((0 as u128), amount));
        return buf
    }

    public fun deserializeTxArgs(raw_data: &vector<u8>): (vector<u8>, vector<u8>, u128) {
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
        } else if (string::index_of(ascii_name, &string::utf8(b"BFC_USDT")) != length(ascii_name)) {
            return b"BFC_USDT"
        } else if (string::index_of(ascii_name, &string::utf8(b"BFC_USDC")) != length(ascii_name)) {
            return b"BFC_USDC"
        } else {
            return b"BFC_BTC"
        }
    }
}
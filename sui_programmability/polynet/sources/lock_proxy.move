#[allow(unused_field,unused_assignment,unused_type_parameter)]
module polynet::lock_proxy {
    use std::ascii;
    use std::ascii::{as_bytes, String, string};
    use std::vector;
    use std::option::{Self, Option};
    use sui::event;
    use sui::math;
    use sui::table::{Table, Self};
    use std::type_name::{Self, TypeName};
    use sui::address;
    use polynet::cross_chain_manager::{CrossChainManager, LicenseInfo};
    use sui::coin::{Coin, Self};
    use sui::object;
    use sui::object::UID;
    use sui::transfer;
    use sui::tx_context;
    use sui::tx_context::TxContext;

    use polynet::cross_chain_manager;
    use polynet::zero_copy_sink;
    use polynet::zero_copy_source;
    use polynet::utils;

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
    const EINVALID_SIGNER: u64 = 4015;
    const ELICENSE_STORE_NOT_EXIST: u64 = 4016;


    struct LockProxyManager has key{
        id: UID,
        lock_proxy_store: LockProxyStore,
        license_store: LicenseStore,
    }

    struct LockProxyStore has key, store {
        id: UID,
        proxy_map: Table<u64, vector<u8>>,
        asset_map: Table<TypeName, Table<u64, vector<u8>>>,
        paused: bool,
        owner: address,
    }

    struct Treasury<phantom CoinType> has key, store {
        id: UID,
        coin: Coin<CoinType>
    }

    struct LicenseStore has key, store {
        id: UID,
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
    struct LockEvent has store, drop, copy {
        from_asset: TypeName,
        from_address: address,
        to_chain_id: u64,
        to_asset_hash: vector<u8>,
        to_address: vector<u8>,
        amount: u64,
        target_chain_amount: u128
    }

    struct LicenseIdEvent has store, drop, copy {
        license_id: vector<u8>,
        account: address,
        module_name: String,
    }

    // init
    public entry fun init_lock_proxy_manager(ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);

        assert!((sender) == utils::get_bridge_address(), EINVALID_SIGNER);

        let lockproxystore = LockProxyStore{
            id: object::new(ctx),
            proxy_map: table::new<u64, vector<u8>>(ctx),
            asset_map: table::new<TypeName, Table<u64, vector<u8>>>(ctx),
            paused: false,
            owner: (sender),
            };

        let licensestore = LicenseStore{
            id: object::new(ctx),
            license: option::none<cross_chain_manager::License>(),
        };

        let manager = LockProxyManager{
            id: object::new(ctx),
            lock_proxy_store: lockproxystore,
            license_store: licensestore,
        };



        transfer::share_object(manager)

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
    fun onlyOwner(lpManager: &LockProxyManager, owner: address) {
        //let config_ref = borrow_global<LockProxyStore>(POLY_BRIDGE);
        assert!((owner) == lpManager.lock_proxy_store.owner, ENOT_OWNER);
    }

    public entry fun transferOwnerShip(lpManager: &mut LockProxyManager, new_owner: address, ctx:&mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);

        onlyOwner(lpManager, sender);
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        lpManager.lock_proxy_store.owner = new_owner;
    }

    public entry fun pause(lpManager: &mut LockProxyManager,  ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        onlyOwner(lpManager, sender);
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        lpManager.lock_proxy_store.paused = true;
    }

    public entry fun unpause(lpManager: &mut LockProxyManager, ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        onlyOwner(lpManager, sender);
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        lpManager.lock_proxy_store.paused = false;
    }

    public entry fun bindProxy(lpManager: &mut LockProxyManager, to_chain_id: u64, target_proxy_hash: vector<u8>, ctx: &mut TxContext)  {
        // sender address
        let sender = tx_context::sender(ctx);
        onlyOwner(lpManager, sender);
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        utils::upsert(&mut lpManager.lock_proxy_store.proxy_map, to_chain_id, target_proxy_hash);

        event::emit(
            BindProxyEvent{
                to_chain_id: to_chain_id,
                target_proxy_hash,
            },
        );
    }

    public entry fun unbindProxy(lpManager: &mut LockProxyManager, to_chain_id: u64, ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        onlyOwner(lpManager, sender);
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

    public entry fun bindAsset<CoinType>(lpManager: &mut LockProxyManager,
                                         to_chain_id: u64,
                                         to_asset_hash: vector<u8>,
                                         to_asset_decimals: u8,
                                         ctx: &mut TxContext)  {
        // sender address
        let sender = tx_context::sender(ctx);
        onlyOwner(lpManager, sender);
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

    public entry fun unbindAsset<CoinType>(lpManager: &mut LockProxyManager, to_chain_id: u64, ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        onlyOwner(lpManager, sender);
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

        //assert!((admin) == utils::get_bridge_address(), EINVALID_SIGNER);
        //assert!(!exists<Treasury<CoinType>>(POLY_BRIDGE), ETREASURY_ALREADY_EXIST);


        let treasury = Treasury<CoinType>{
            id: object::new(ctx),
            coin: coin::zero<CoinType>(ctx),
        };

        treasury

    }

    public fun lock_proxy_transfer<CoinType>(treasury:Treasury<CoinType>,admin: address) {
        transfer::transfer(treasury, admin)
    }

    public fun is_treasury_initialzed<CoinType>(): bool {
        true
        //exists<Treasury<CoinType>>(POLY_BRIDGE)
    }

    public fun is_admin(account: address): bool {
        account == utils::get_bridge_address()
    }

    public fun deposit<CoinType>(treasury_ref: &mut Treasury<CoinType>,  fund: Coin<CoinType>)  {
        //assert!(exists<Treasury<CoinType>>(POLY_BRIDGE), ETREASURY_NOT_EXIST);
        //let treasury_ref = borrow_global_mut<Treasury<CoinType>>(POLY_BRIDGE);


        coin::join<CoinType>(&mut treasury_ref.coin, fund);
    }

    fun withdraw<CoinType>(treasury_ref:&mut Treasury<CoinType>, amount: u64 , ctx: &mut TxContext): Coin<CoinType> {
        //assert!(exists<Treasury<CoinType>>(POLY_BRIDGE), ETREASURY_NOT_EXIST);
        //let treasury_ref = borrow_global_mut<Treasury<CoinType>>(POLY_BRIDGE);

        return coin::split(&mut treasury_ref.coin, amount, ctx)
        //return coin::extract<CoinType>(&mut treasury_ref.coin, amount)
    }




    // license function
    public fun receiveLicense(lpManager: &mut LockProxyManager,
                              license: cross_chain_manager::License)   {
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
        assert!((admin) == utils::get_bridge_address(), EINVALID_SIGNER);
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

    public entry fun outputLicenseId(lpManager: &LockProxyManager) {
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
    

    // lock
    public fun lock<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        account: address,
                              fund: Coin<CoinType>,
                              toChainId: u64,
                              toAddress: &vector<u8>, ctx: &mut TxContext)  {
        // lock fund
        let amount = coin::value(&fund);
        deposit(treasury_ref, fund);
        
        // borrow license
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        //let license_opt = &borrow_global<LicenseStore>(POLY_BRIDGE).license;
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
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
        cross_chain_manager::crossChain(ccManager, license_ref, toChainId,
                            &to_proxy, &b"unlock", &tx_data, ctx);

        // emit event 
        //let config_ref = borrow_global_mut<LockProxyStore>(POLY_BRIDGE);
        event::emit(
            LockEvent{
                from_asset: type_name::get<Coin<CoinType>>(),
                from_address: (account),
                to_chain_id: toChainId,
                to_asset_hash: to_asset,
                to_address: *toAddress,
                amount,
                target_chain_amount,
            },
        );
    }


    // unlock
    public fun unlock<CoinType>(lpManager: &mut LockProxyManager,
                                treasury_ref:&mut Treasury<CoinType>,
                                certificate: cross_chain_manager::Certificate,
                                ctx: &mut TxContext
    )  {
        // read certificate
        let (
            from_contract,
            from_chain_id,
            target_license_id,
            method,
            args
        ) = cross_chain_manager::read_certificate(&certificate);

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

        // check
        //type_name::get<Coin<CoinType>>()
        assert!( *as_bytes(type_name::borrow_string(&type_name::get<Coin<CoinType>>())) == to_asset, EINVALID_COINTYPE);
        assert!(getTargetProxy(lpManager, from_chain_id) == from_contract, EINVALID_FROM_CONTRACT);
        let (license_id, _) = getLicenseId(lpManager);
        assert!(license_id == target_license_id, EINVALID_TARGET_LICENSE_ID);
        assert!(method == b"unlock", EINVALID_METHOD);

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

    public entry fun relay_unlock_tx<CoinType>(
        ccManager:&mut CrossChainManager,
        lpManager: &mut LockProxyManager,
        treasury_ref:&mut Treasury<CoinType>,
        proof: vector<u8>, 
        rawHeader: vector<u8>, 
        headerProof: vector<u8>, 
        curRawHeader: vector<u8>, 
        headerSig: vector<u8>,
        ctx: &mut TxContext
    )  {
        // borrow license
        //assert!(exists<LicenseStore>(POLY_BRIDGE), ELICENSE_NOT_EXIST);
        assert!(option::is_some<cross_chain_manager::License>(&lpManager.license_store.license), ELICENSE_NOT_EXIST);
        let license_ref = option::borrow(&lpManager.license_store.license);

        let certificate = cross_chain_manager::verifyHeaderAndExecuteTx(ccManager,license_ref, &proof, &rawHeader, &headerProof, &curRawHeader, &headerSig, ctx);
        unlock<CoinType>(lpManager, treasury_ref, certificate, ctx);
    }


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
}
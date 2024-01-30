module poly_bridge::lock_proxy {
    use std::ascii::as_bytes;
    use std::vector;
    use std::string;
    use std::option::{Self, Option};
    use sui::event;
    use sui::math;
    use sui::table::{Table, Self};
    use std::type_name::{Self, TypeName};
    use sui::coin::{Coin, Self};
    use sui::transfer::transfer;

    use poly::cross_chain_manager;
    use poly::zero_copy_sink;
    use poly::zero_copy_source;
    use poly::utils;

    const DEPRECATED: u64 = 1;
    const ENOT_OWNER: u64 = 2;
    const ETREASURY_ALREADY_EXIST: u64 = 3;
    const ETREASURY_NOT_EXIST: u64 = 4;
    const ELICENSE_ALREADY_EXIST: u64 = 5;
    const ELICENSE_NOT_EXIST: u64 = 6;
    const ETARGET_PROXY_NOT_BIND: u64 = 7;
    const ETARGET_ASSET_NOT_BIND: u64 = 8;
    const EINVALID_COINTYPE: u64 = 9;
    const EINVALID_FROM_CONTRACT: u64 = 10;
    const EINVALID_TARGET_LICENSE_ID: u64 = 11;
    const EINVALID_METHOD: u64 = 12;
    const ELICENSE_STORE_ALREADY_EXIST: u64 = 13;
    const EINVALID_LICENSE_INFO: u64 = 14;
    const EINVALID_SIGNER: u64 = 15;
    const ELICENSE_STORE_NOT_EXIST: u64 = 16;


    struct LockProxyStore has key, store {
        proxy_map: Table<u64, vector<u8>>,
        asset_map: Table<TypeName, Table<u64, vector<u8>>>,
        paused: bool,
        owner: address,
        //bind_proxy_event: event::EventHandle<BindProxyEvent>,
        //bind_asset_event: event::EventHandle<BindAssetEvent>,
        //lock_event: event::EventHandle<LockEvent>,
        //unlock_event: event::EventHandle<UnlockEvent>
    }

    struct Treasury<phantom CoinType> has key, store {
        coin: Coin<CoinType>
    }

    struct LicenseStore has key, store {
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


    // init
    public entry fun init(admin: address) {
        assert!((admin) == @poly_bridge, EINVALID_SIGNER);

        transfer(LockProxyStore{
            proxy_map: table::new<u64, vector<u8>>(),
            asset_map: table::new<TypeName, Table<u64, vector<u8>>>(),
            paused: false,
            owner: (admin),
            //bind_proxy_event: account::new_event_handle<BindProxyEvent>(admin),
            //bind_asset_event: account::new_event_handle<BindAssetEvent>(admin),
            //lock_event: account::new_event_handle<LockEvent>(admin),
            //unlock_event: account::new_event_handle<UnlockEvent>(admin),
            },
             admin);

        transfer(LicenseStore{
            license: option::none<cross_chain_manager::License>(),
        }, admin);
    }


    // getter function
    public fun getTargetProxy(to_chain_id: u64): vector<u8> acquires LockProxyStore {
        let config_ref = borrow_global<LockProxyStore>(@poly_bridge);
        if (table::contains(&config_ref.proxy_map, to_chain_id)) {
            return *table::borrow(&config_ref.proxy_map, to_chain_id)
        } else {
            abort ETARGET_PROXY_NOT_BIND
        }
    }

    public fun getToAsset<CoinType>(to_chain_id: u64): (vector<u8>, u8) acquires LockProxyStore {
        let config_ref = borrow_global<LockProxyStore>(@poly_bridge);
        let from_asset = type_name::get<Coin<CoinType>>();
        if (table::contains(&config_ref.asset_map, from_asset)) {
            let sub_table = table::borrow(&config_ref.asset_map, from_asset);
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

    public fun paused(): bool acquires LockProxyStore {
        let config_ref = borrow_global<LockProxyStore>(@poly_bridge);
        return config_ref.paused
    }

    public fun owner(): address acquires LockProxyStore {
        let config_ref = borrow_global<LockProxyStore>(@poly_bridge);
        return config_ref.owner
    }

    public fun getBalance<CoinType>(): u64 acquires Treasury {
        assert!(exists<Treasury<CoinType>>(@poly_bridge), ETREASURY_NOT_EXIST);
        let treasury_ref = borrow_global<Treasury<CoinType>>(@poly_bridge);
        return coin::value(&treasury_ref.coin)
    }


    // owner function
    fun onlyOwner(owner: address) acquires LockProxyStore {
        let config_ref = borrow_global<LockProxyStore>(@poly_bridge);
        assert!((owner) == config_ref.owner, ENOT_OWNER);
    }

    public entry fun transferOwnerShip(owner: address, new_owner: address) acquires LockProxyStore {
        onlyOwner(owner);
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
        config_ref.owner = new_owner;
    }

    public entry fun pause(owner: address) acquires LockProxyStore {
        onlyOwner(owner);
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
        config_ref.paused = true;
    }

    public entry fun unpause(owner: address) acquires LockProxyStore {
        onlyOwner(owner);
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
        config_ref.paused = false;
    }

    public entry fun bindProxy(owner: address, to_chain_id: u64, target_proxy_hash: vector<u8>) acquires LockProxyStore {
        onlyOwner(owner);
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
        table::upsert(&mut config_ref.proxy_map, to_chain_id, target_proxy_hash);

        event::emit(
            BindProxyEvent{
                to_chain_id: to_chain_id,
                target_proxy_hash,
            },
        );
    }

    public entry fun unbindProxy(owner: address, to_chain_id: u64) acquires LockProxyStore {
        onlyOwner(owner);
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
        if (table::contains(&config_ref.proxy_map, to_chain_id)) {
            table::remove(&mut config_ref.proxy_map, to_chain_id);
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

    public entry fun bindAsset<CoinType>(owner: address, to_chain_id: u64, to_asset_hash: vector<u8>, to_asset_decimals: u8) acquires LockProxyStore {
        onlyOwner(owner);
        let from_asset = type_name::get<Coin<CoinType>>();
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
        let decimals_concat_to_asset = vector::singleton(to_asset_decimals);
        vector::append(&mut decimals_concat_to_asset, to_asset_hash);
        if (table::contains(&config_ref.asset_map, from_asset)) {
            table::upsert(table::borrow_mut(&mut config_ref.asset_map, from_asset), to_chain_id, decimals_concat_to_asset);
        } else {
            let subTable = table::new<u64, vector<u8>>();
            table::add(&mut subTable, to_chain_id, decimals_concat_to_asset);
            table::add(&mut config_ref.asset_map, from_asset, subTable);
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

    public entry fun unbindAsset<CoinType>(owner: address, to_chain_id: u64) acquires LockProxyStore {
        onlyOwner(owner);
        let from_asset = type_name::get<Coin<CoinType>>();
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
        if (table::contains(&config_ref.asset_map, from_asset)) {
            let sub_table = table::borrow_mut(&mut config_ref.asset_map, from_asset);
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
    public entry fun initTreasury<CoinType>(admin: address) {
        assert!((admin) == @poly_bridge, EINVALID_SIGNER);
        assert!(!exists<Treasury<CoinType>>(@poly_bridge), ETREASURY_ALREADY_EXIST);


        transfer(Treasury<CoinType> { coin: coin::zero<CoinType>() }, admin);
    }

    public fun is_treasury_initialzed<CoinType>(): bool {
        exists<Treasury<CoinType>>(@poly_bridge)
    }

    public fun is_admin(account: address): bool {
        account == @poly_bridge
    }

    public fun deposit<CoinType>(fund: Coin<CoinType>) acquires Treasury {
        assert!(exists<Treasury<CoinType>>(@poly_bridge), ETREASURY_NOT_EXIST);
        let treasury_ref = borrow_global_mut<Treasury<CoinType>>(@poly_bridge);
        coin::merge<CoinType>(&mut treasury_ref.coin, fund);
    }

    fun withdraw<CoinType>(amount: u64): Coin<CoinType> acquires Treasury {
        assert!(exists<Treasury<CoinType>>(@poly_bridge), ETREASURY_NOT_EXIST);
        let treasury_ref = borrow_global_mut<Treasury<CoinType>>(@poly_bridge);
        return coin::extract<CoinType>(&mut treasury_ref.coin, amount)
    }


    // license function
    public fun receiveLicense(license: cross_chain_manager::License) acquires LicenseStore {
        assert!(exists<LicenseStore>(@poly_bridge), ELICENSE_STORE_NOT_EXIST);
        let license_opt = &mut borrow_global_mut<LicenseStore>(@poly_bridge).license;
        assert!(option::is_none<cross_chain_manager::License>(license_opt), ELICENSE_ALREADY_EXIST);
        let (license_account, license_module_name) = cross_chain_manager::getLicenseInfo(&license);
        let this_type = type_name::get<LicenseStore>();
        let this_account = type_name::get_address(&this_type);
        let this_module_name = type_name::get_module(&this_type);

        //todo
        //assert!(license_account == this_account && license_module_name == this_module_name, EINVALID_LICENSE_INFO);
        option::fill(license_opt, license);
    }

    public fun removeLicense(admin: address): cross_chain_manager::License acquires LicenseStore {
        assert!((admin) == @poly_bridge, EINVALID_SIGNER);
        assert!(exists<LicenseStore>(@poly_bridge), ELICENSE_NOT_EXIST);
        let license_opt = &mut borrow_global_mut<LicenseStore>(@poly_bridge).license;
        assert!(option::is_some<cross_chain_manager::License>(license_opt), ELICENSE_NOT_EXIST);
        option::extract<cross_chain_manager::License>(license_opt)
    }

    public fun getLicenseId(): vector<u8> acquires LicenseStore {
        assert!(exists<LicenseStore>(@poly_bridge), ELICENSE_NOT_EXIST);
        let license_opt = &borrow_global<LicenseStore>(@poly_bridge).license;
        assert!(option::is_some<cross_chain_manager::License>(license_opt), ELICENSE_NOT_EXIST);
        return cross_chain_manager::getLicenseId(option::borrow(license_opt))
    }
    

    // lock
    public fun lock<CoinType>(account: address, fund: Coin<CoinType>, toChainId: u64, toAddress: &vector<u8>) acquires Treasury, LicenseStore, LockProxyStore {
        // lock fund
        let amount = coin::value(&fund);
        deposit(fund);
        
        // borrow license
        assert!(exists<LicenseStore>(@poly_bridge), ELICENSE_NOT_EXIST);
        let license_opt = &borrow_global<LicenseStore>(@poly_bridge).license;
        assert!(option::is_some<cross_chain_manager::License>(license_opt), ELICENSE_NOT_EXIST);
        let license_ref = option::borrow(license_opt);

        // get target proxy/asset
        let to_proxy = getTargetProxy(toChainId);
        let (to_asset, to_asset_decimals) = getToAsset<CoinType>(toChainId);

        // precision conversion
        let target_chain_amount = to_target_chain_amount<CoinType>(amount, to_asset_decimals);

        // pack args
        let tx_data = serializeTxArgs(&to_asset, toAddress, target_chain_amount);

        // cross chain
        cross_chain_manager::crossChain(account, license_ref, toChainId, &to_proxy, &b"unlock", &tx_data);

        // emit event 
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
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
    public fun unlock<CoinType>(certificate: cross_chain_manager::Certificate) acquires Treasury, LicenseStore, LockProxyStore {
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
        let (_, decimals) = getToAsset<CoinType>(from_chain_id);
        let amount = from_target_chain_amount<CoinType>(from_chain_amount, decimals);

        // check
        assert!( *as_bytes(type_name::borrow_string<CoinType>()) == to_asset, EINVALID_COINTYPE);
        assert!(getTargetProxy(from_chain_id) == from_contract, EINVALID_FROM_CONTRACT);
        assert!(getLicenseId() == target_license_id, EINVALID_TARGET_LICENSE_ID);
        assert!(method == b"unlock", EINVALID_METHOD);

        // unlock fund
        let fund = withdraw<CoinType>(amount);
        coin::deposit(utils::to_address(to_address), fund);

        // emit event
        let config_ref = borrow_global_mut<LockProxyStore>(@poly_bridge);
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
        proof: vector<u8>, 
        rawHeader: vector<u8>, 
        headerProof: vector<u8>, 
        curRawHeader: vector<u8>, 
        headerSig: vector<u8>
    ) acquires Treasury, LicenseStore, LockProxyStore {
        // borrow license
        assert!(exists<LicenseStore>(@poly_bridge), ELICENSE_NOT_EXIST);
        let license_opt = &borrow_global<LicenseStore>(@poly_bridge).license;
        assert!(option::is_some<cross_chain_manager::License>(license_opt), ELICENSE_NOT_EXIST);
        let license_ref = option::borrow(license_opt);

        let certificate = cross_chain_manager::verifyHeaderAndExecuteTx(license_ref, &proof, &rawHeader, &headerProof, &curRawHeader, &headerSig);
        unlock<CoinType>(certificate);
    }


    // decimals conversion
    public fun to_target_chain_amount<CoinType>(amount: u64, target_decimals: u8): u128 {
        let source_decimals = coin::decimals<CoinType>();
        (amount as u128) * pow_10(target_decimals) / pow_10(source_decimals)
    }
    public fun from_target_chain_amount<CoinType>(target_chain_amount: u128, target_decimals: u8): u64 {
        let source_decimals = coin::decimals<CoinType>();
        (target_chain_amount * pow_10(source_decimals) / pow_10(target_decimals) as u64)
    }
    fun pow_10(decimals: u8): u128 {
        math128::pow(10, (decimals as u128))
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
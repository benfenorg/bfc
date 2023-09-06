module obc_system::treasury {
    use std::ascii::into_bytes;
    use std::type_name::{ get, into_string};
    use std::vector;

    use sui::obc::OBC;
    use sui::balance::{Self, Balance, Supply};
    use sui::object::{Self, ID, UID};
    use sui::tx_context::TxContext;
    use sui::dynamic_object_field;
    use sui::bag::{Self, Bag};

    use obc_system::event;
    use obc_system::vault::{Self, Vault};
    use obc_system::utils;

    friend obc_system::swap;
    friend obc_system::obc_system;

    // === Errors ===
    const ERR_THE_SAME_COIN: u64 = 100;
    const ERR_POOL_HAS_REGISTERED: u64 = 101;
    const ERR_INVALID_LIMIT: u64 = 102;
    const ERR_INVALID_VECTOR_LENGTH: u64 = 103;
    const ERR_MUST_BE_ORDER: u64 = 104;
    const ERR_POOL_NOT_EXISTS: u64 = 105;

    struct Treasury has key, store {
        id: UID,
        obc_balance: Balance<OBC>,
        /// stable coin supplies
        supplies: Bag,
        /// Vault index
        index: u64,
    }

    // call in obc_system
    public(friend) fun create_treasury(ctx: &mut TxContext): Treasury {
        let treasury = Treasury {
            id: object::new(ctx),
            obc_balance: balance::zero<OBC>(),
            supplies: bag::new(ctx),
            index: 0,
        };
        let treasury_id = object::id(&treasury);
        event::init_treasury(treasury_id);
        treasury
    }

    public fun index(_treasury: &Treasury): u64 {
        _treasury.index
    }

    fun check_vault<CoinTypeA, CoinTypeB>(_treasury: &Treasury, _vault_key: ID) {
        assert!(
            utils::cmp<CoinTypeA, CoinTypeB>() < 1,
            ERR_MUST_BE_ORDER
        );
        assert!(
            dynamic_object_field::exists_(
                &_treasury.id,
                _vault_key
            ),
            ERR_POOL_NOT_EXISTS
        );
    }

    public fun borrow_vault<CoinTypeA, CoinTypeB>(
        _treasury: &Treasury,
        _vault_key: ID
    ): &Vault<CoinTypeA, CoinTypeB> {
        check_vault<CoinTypeA, CoinTypeB>(_treasury, _vault_key);
        dynamic_object_field::borrow<ID, Vault<CoinTypeA, CoinTypeB>>(&_treasury.id, _vault_key)
    }

    public fun borrow_mut_vault<CoinTypeA, CoinTypeB>(
        _treasury: &mut Treasury,
        _vault_key: ID
    ): &mut Vault<CoinTypeA, CoinTypeB> {
        check_vault<CoinTypeA, CoinTypeB>(_treasury, _vault_key);
        dynamic_object_field::borrow_mut<ID, Vault<CoinTypeA, CoinTypeB>>(&mut _treasury.id, _vault_key)
    }

    public(friend) fun create_vault<CoinTypeA, CoinTypeB, SupplyCoinType>(
        _treasury: &mut Treasury,
        _supply: Supply<SupplyCoinType>,
        _position_number: u32,
        _tick_spacing: u32,
        _initialize_price: u128,
        _ts: u64,
        _ctx: &mut TxContext
    ) {
        if (utils::cmp<CoinTypeA, CoinTypeB>() < 1) {
            create_vault_internal<CoinTypeA, CoinTypeB, SupplyCoinType>(
                _treasury,
                _supply,
                _tick_spacing,
                _position_number,
                _initialize_price,
                _ts,
                _ctx,
            );
        } else {
            create_vault_internal<CoinTypeB, CoinTypeA, SupplyCoinType>(
                _treasury,
                _supply,
                _tick_spacing,
                _position_number,
                _initialize_price,
                _ts,
                _ctx,
            )
        };
    }

    public(friend) fun init_positions<CoinTypeA, CoinTypeB>(
        _treasury: &mut Treasury,
        _tick_spacing: u32,
        _spacing_times: u32,
        _ctx: &mut TxContext,
    ) {
        let vault_key = generate_vault_key<CoinTypeA, CoinTypeB>(_tick_spacing);
        if (utils::cmp<CoinTypeA, CoinTypeB>() < 1) {
            let vault = borrow_mut_vault<CoinTypeA, CoinTypeB>(_treasury, vault_key);
            vault::init_positions<CoinTypeA, CoinTypeB>(
                vault,
                _spacing_times,
                _ctx,
            );
        } else {
            let vault = borrow_mut_vault<CoinTypeB, CoinTypeA>(_treasury, vault_key);
            vault::init_positions<CoinTypeB, CoinTypeA>(
                vault,
                _spacing_times,
                _ctx,
            );
        }
    }

    /// creat vault for ordered A & B
    fun create_vault_internal<CoinTypeA, CoinTypeB, SupplyCoinType>(
        _treasury: &mut Treasury,
        _supply: Supply<SupplyCoinType>,
        _tick_spacing: u32,
        _position_number: u32,
        _initialize_price: u128,
        _ts: u64,
        _ctx: &mut TxContext
    ) {
        let vault_key = generate_vault_key<CoinTypeA, CoinTypeB>(_tick_spacing);
        assert!(!dynamic_object_field::exists_<ID>(&_treasury.id, vault_key), ERR_POOL_HAS_REGISTERED);

        // index increased
        _treasury.index = _treasury.index + 1;
        let new_vault = vault::create_vault<CoinTypeA, CoinTypeB>(
            _treasury.index,
            _tick_spacing,
            _position_number,
            _initialize_price,
            _ts,
            _ctx,
        );
        let vault_id = object::id(&new_vault);

        dynamic_object_field::add(
            &mut _treasury.id,
            vault_key,
            new_vault,
        );
        bag::add<ID, Supply<SupplyCoinType>>(&mut _treasury.supplies, vault_key, _supply);

        event::create_vault(
            vault_id,
            vault_key,
            into_string(get<CoinTypeA>()),
            into_string(get<CoinTypeB>()),
            _tick_spacing,
            _treasury.index,
        );
    }

    public fun generate_vault_key<CoinTypeA, CoinTypeB>(_tick_spacing: u32): ID {
        let comp = utils::cmp<CoinTypeA, CoinTypeB>();
        assert!(comp != 1, ERR_THE_SAME_COIN);
        let bytes = vector::empty<u8>();
        if (comp < 1) {
            // a_typename < b_typename
            vector::append(&mut bytes, into_bytes(into_string(get<CoinTypeA>())));
            vector::append(&mut bytes, b"-");
            vector::append(&mut bytes, into_bytes(into_string(get<CoinTypeB>())));
        } else {
            vector::append(&mut bytes, into_bytes(into_string(get<CoinTypeB>())));
            vector::append(&mut bytes, b"-");
            vector::append(&mut bytes, into_bytes(into_string(get<CoinTypeA>())));
        };
        vector::append(&mut bytes, b"-");
        vector::append(&mut bytes, into_bytes(utils::to_string((_tick_spacing as u128))));
        object::id_from_bytes(sui::hash::blake2b256(&bytes))
    }
}

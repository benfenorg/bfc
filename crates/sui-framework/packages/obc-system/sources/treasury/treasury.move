module obc_system::treasury {
    use std::ascii::String;
    use std::type_name;
    use std::type_name::{ get, into_string};

    use sui::transfer;
    use sui::coin::{Self, Coin};
    use sui::bag::{Self, Bag};
    use sui::balance::{Self, Balance, Supply};
    use sui::dynamic_object_field;
    use sui::obc::OBC;
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};

    use obc_system::event;
    use obc_system::vault::{Self, Vault};

    friend obc_system::obc_system_state_inner;

    // === Errors ===
    const ERR_THE_SAME_COIN: u64 = 100;
    const ERR_POOL_HAS_REGISTERED: u64 = 101;
    const ERR_INVALID_LIMIT: u64 = 102;
    const ERR_INVALID_VECTOR_LENGTH: u64 = 103;
    const ERR_MUST_BE_ORDER: u64 = 104;
    const ERR_POOL_NOT_EXISTS: u64 = 105;
    const ERR_ZERO_AMOUNT: u64 = 106;

    struct Treasury has key, store {
        id: UID,
        obc_balance: Balance<OBC>,
        /// stable coin supplies
        supplies: Bag,
        /// Vault index
        index: u64,
        time_interval: u32,
    }

    // call in obc_system
    public(friend) fun create_treasury(time_interval: u32, ctx: &mut TxContext): Treasury {
        let treasury = Treasury {
            id: object::new(ctx),
            obc_balance: balance::zero<OBC>(),
            supplies: bag::new(ctx),
            index: 0,
            time_interval: time_interval,
        };
        let treasury_id = object::id(&treasury);
        event::init_treasury(treasury_id);
        treasury
    }

    public fun index(_treasury: &Treasury): u64 {
        _treasury.index
    }

    fun check_vault<StableCoinType>(_treasury: &Treasury, _vault_key: String) {
        assert!(
            dynamic_object_field::exists_(
                &_treasury.id,
                _vault_key
            ),
            ERR_POOL_NOT_EXISTS
        );
    }

    public fun borrow_vault<StableCoinType>(
        _treasury: &Treasury,
        _vault_key: String
    ): &Vault<StableCoinType> {
        check_vault<StableCoinType>(_treasury, _vault_key);
        dynamic_object_field::borrow<String, Vault<StableCoinType>>(&_treasury.id, _vault_key)
    }

    public fun borrow_mut_vault<StableCoinType>(
        _treasury: &mut Treasury,
        _vault_key: String
    ): &mut Vault<StableCoinType> {
        check_vault<StableCoinType>(_treasury, _vault_key);
        dynamic_object_field::borrow_mut<String, Vault<StableCoinType>>(&mut _treasury.id, _vault_key)
    }

    public(friend) fun create_vault<StableCoinType>(
        _treasury: &mut Treasury,
        _supply: Supply<StableCoinType>,
        _position_number: u32,
        _tick_spacing: u32,
        _initialize_price: u128,
        _base_point: u64,
        _ts: u64,
        _ctx: &mut TxContext
    ) {
        create_vault_internal<StableCoinType>(
            _treasury,
            _supply,
            _tick_spacing,
            _position_number,
            _initialize_price,
            _base_point,
            _ts,
            _ctx,
        );
    }

    public(friend) fun init_vault_with_positions<StableCoinType>(
        _treasury: &mut Treasury,
        _supply: Supply<StableCoinType>,
        _initialize_price: u128,
        _base_point: u64,
        _position_number: u32,
        _tick_spacing: u32,
        _spacing_times: u32,
        _ts: u64,
        _ctx: &mut TxContext,
    ) {
        let vault_key = create_vault_internal<StableCoinType>(
            _treasury,
            _supply,
            _tick_spacing,
            _position_number,
            _initialize_price,
            _base_point,
            _ts,
            _ctx,
        );
        vault::init_positions<StableCoinType>(
            borrow_mut_vault<StableCoinType>(_treasury, vault_key),
            _spacing_times,
            _ctx,
        );
    }

    /// creat vault for ordered A & B
    fun create_vault_internal<StableCoinType>(
        _treasury: &mut Treasury,
        _supply: Supply<StableCoinType>,
        _tick_spacing: u32,
        _position_number: u32,
        _initialize_price: u128,
        _base_point: u64,
        _ts: u64,
        _ctx: &mut TxContext
    ): String {
        let vault_key = type_name::into_string(type_name::get<StableCoinType>());
        assert!(!dynamic_object_field::exists_<String>(&_treasury.id, vault_key), ERR_POOL_HAS_REGISTERED);

        // index increased
        _treasury.index = _treasury.index + 1;
        let new_vault = vault::create_vault<StableCoinType>(
            _treasury.index,
            _tick_spacing,
            _position_number,
            _initialize_price,
            _base_point,
            _ts,
            _ctx,
        );
        let vault_id = object::id(&new_vault);

        dynamic_object_field::add(
            &mut _treasury.id,
            vault_key,
            new_vault,
        );
        bag::add<String, Supply<StableCoinType>>(&mut _treasury.supplies, vault_key, _supply);

        event::create_vault(
            vault_id,
            vault_key,
            into_string(get<StableCoinType>()),
            into_string(get<OBC>()),
            _tick_spacing,
            _treasury.index,
        );
        vault_key
    }

    ///  ======= Swap
    /// Mint swap obc to stablecoin
    public(friend) entry fun mint<StableCoinType>(
        treasury: &mut Treasury,
        coin_obc: Coin<OBC>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        assert!(coin::value<OBC>(&coin_obc) > 0, ERR_ZERO_AMOUNT);
        swap_internal<StableCoinType>(
            treasury,
            false,
            coin::zero<StableCoinType>(ctx),
            coin_obc,
            amount,
            ctx,
        );
    }

    /// Burn swap stablecoin to obc
    public(friend) entry fun redeem<StableCoinType>(
        treasury: &mut Treasury,
        coin_sc: Coin<StableCoinType>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        assert!(coin::value<StableCoinType>(&coin_sc) > 0, ERR_ZERO_AMOUNT);
        swap_internal<StableCoinType>(
            treasury,
            true,
            coin_sc,
            coin::zero<OBC>(ctx),
            amount,
            ctx,
        );
    }

    fun transfer_or_delete<CoinType>(
        balance: Balance<CoinType>,
        ctx: &mut TxContext
    ) {
        if (balance::value(&balance) > 0) {
            transfer::public_transfer(coin::from_balance(balance, ctx), tx_context::sender(ctx));
        } else {
            balance::destroy_zero(balance);
        }
    }

    /// Internal swap
    fun swap_internal<StableCoinType>(
        treasury: &mut Treasury,
        a2b: bool, // true a->b , false b->a
        coin_a: Coin<StableCoinType>,
        coin_b: Coin<OBC>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        let vault_key = type_name::into_string(type_name::get<StableCoinType>());
        let mut_vault = borrow_mut_vault<StableCoinType>(treasury, vault_key);
        let current_sqrt_price = vault::vault_current_sqrt_price(mut_vault);
        let (balance_a, balance_b) = vault::swap<StableCoinType>(
            mut_vault,
            coin_a,
            coin_b,
            a2b,
            true,
            amount,
            0, // ? unuse
            current_sqrt_price,
            ctx
        );
        transfer_or_delete(balance_a, ctx);
        transfer_or_delete(balance_b, ctx);
    }
}

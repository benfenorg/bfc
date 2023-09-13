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
    use sui::clock::{Self, Clock};
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};

    use obc_system::usd::USD;
    use obc_system::event;
    use obc_system::vault::{Self, Vault};
    use obc_system::tick_math;

    friend obc_system::obc_system_state_inner;
    #[test_only]
    friend obc_system::treasury_test;
    #[test_only]
    friend obc_system::vault_test;

    // === Errors ===
    const ERR_POOL_HAS_REGISTERED: u64 = 100;
    const ERR_POOL_NOT_EXISTS: u64 = 101;
    const ERR_ZERO_AMOUNT: u64 = 102;
    const ERR_INSUFFICIENT: u64 = 103;

    struct Treasury has key, store {
        id: UID,
        obc_balance: Balance<OBC>,
        /// stable coin supplies
        supplies: Bag,
        /// Vault index
        index: u64,
        time_interval: u32,
        updated_at: u64,
        init: bool,
    }

    // call in obc_system
    public(friend) fun create_treasury(time_interval: u32, ctx: &mut TxContext): Treasury {
        let treasury = Treasury {
            id: object::new(ctx),
            obc_balance: balance::zero<OBC>(),
            supplies: bag::new(ctx),
            index: 0,
            time_interval,
            updated_at: 0,
            init: false,
        };
        let treasury_id = object::id(&treasury);
        event::init_treasury(treasury_id);
        treasury
    }

    public fun index(_treasury: &Treasury): u64 {
        _treasury.index
    }

    public fun get_balance(_treasury: &Treasury): u64 {
        balance::value(&_treasury.obc_balance)
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

    public fun get_vault_key<StableCoinType>(): String {
        type_name::into_string(type_name::get<StableCoinType>())
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
        _spacing_times: u32,
        _initialize_price: u128,
        _base_point: u64,
        _max_counter_times: u32,
        _ts: u64,
        _ctx: &mut TxContext
    ) {
        create_vault_internal<StableCoinType>(
            _treasury,
            _supply,
            _tick_spacing,
            _spacing_times,
            _position_number,
            _initialize_price,
            _base_point,
            _max_counter_times,
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
        _max_counter_times: u32,
        _ts: u64,
        _ctx: &mut TxContext,
    ) {
        let vault_key = create_vault_internal<StableCoinType>(
            _treasury,
            _supply,
            _tick_spacing,
            _spacing_times,
            _position_number,
            _initialize_price,
            _base_point,
            _max_counter_times,
            _ts,
            _ctx,
        );
        _ = vault::init_positions<StableCoinType>(
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
        _spacing_times: u32,
        _position_number: u32,
        _initialize_price: u128,
        _base_point: u64,
        _max_counter_times: u32,
        _ts: u64,
        _ctx: &mut TxContext
    ): String {
        let vault_key = get_vault_key<StableCoinType>();
        assert!(!dynamic_object_field::exists_<String>(&_treasury.id, vault_key), ERR_POOL_HAS_REGISTERED);

        // index increased
        _treasury.index = _treasury.index + 1;
        let new_vault = vault::create_vault<StableCoinType>(
            _treasury.index,
            _tick_spacing,
            _spacing_times,
            _position_number,
            _initialize_price,
            _base_point,
            _max_counter_times,
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
            _spacing_times,
            _treasury.index,
        );
        vault_key
    }

    ///  ======= Swap
    /// Mint swap obc to stablecoin
    public entry fun mint<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_obc: Coin<OBC>,
        _amount: u64,
        _ctx: &mut TxContext,
    ) {
        let balance_a = mint_internal<StableCoinType>(
            _treasury,
            _coin_obc,
            _amount,
            _ctx,
        );
        transfer_or_delete(balance_a, _ctx);
    }

    public(friend) fun mint_internal<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_obc: Coin<OBC>,
        _amount: u64,
        _ctx: &mut TxContext,
    ): Balance<StableCoinType> {
        assert!(coin::value<OBC>(&_coin_obc) > 0, ERR_ZERO_AMOUNT);
        let (balance_a, balance_b) = swap_internal<StableCoinType>(
            _treasury,
            false,
            coin::zero<StableCoinType>(_ctx),
            _coin_obc,
            _amount,
            true,
            _ctx,
        );
        transfer_or_delete(balance_b, _ctx);
        balance_a
    }

    /// Burn swap stablecoin to obc
    public entry fun redeem<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_sc: Coin<StableCoinType>,
        _amount: u64,
        _ctx: &mut TxContext,
    ) {
        assert!(coin::value<StableCoinType>(&_coin_sc) > 0, ERR_ZERO_AMOUNT);
        let balance_b = redeem_internal<StableCoinType>(
            _treasury,
            _coin_sc,
            _amount,
            _ctx,
        );
        transfer_or_delete(balance_b, _ctx);
    }

    public(friend) fun redeem_internal<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_sc: Coin<StableCoinType>,
        _amount: u64,
        _ctx: &mut TxContext,
    ): Balance<OBC> {
        assert!(coin::value<StableCoinType>(&_coin_sc) > 0, ERR_ZERO_AMOUNT);
        let (balance_a, balance_b)  =swap_internal<StableCoinType>(
            _treasury,
            true,
            _coin_sc,
            coin::zero<OBC>(_ctx),
            _amount,
            true,
            _ctx,
        );
        transfer_or_delete(balance_a, _ctx);
        balance_b
    }

    /// Burn swap stablecoin to obc
    public fun calculate_swap_result<StableCoinType>(
        _treasury: &Treasury,
        _a2b: bool,
        _amount: u64
    ): u64
    {
        let sc_vault = borrow_vault<StableCoinType>(_treasury, get_vault_key<StableCoinType>());
        vault::calculated_swap_result_amount_out(&vault::calculate_swap_result(sc_vault, _a2b, true, _amount))
    }

    fun transfer_or_delete<CoinType>(
        _balance: Balance<CoinType>,
        _ctx: &mut TxContext
    ) {
        if (balance::value(&_balance) > 0) {
            transfer::public_transfer(coin::from_balance(_balance, _ctx), tx_context::sender(_ctx));
        } else {
            balance::destroy_zero(_balance);
        }
    }

    /// Internal swap
    fun swap_internal<StableCoinType>(
        _treasury: &mut Treasury,
        _a2b: bool, // true a->b , false b->a
        _coin_a: Coin<StableCoinType>,
        _coin_b: Coin<OBC>,
        _amount: u64,
        _by_amount_in: bool,
        _ctx: &mut TxContext,
    ): (Balance<StableCoinType>, Balance<OBC>) {
        let vault_key = get_vault_key<StableCoinType>();
        let mut_vault = borrow_mut_vault<StableCoinType>(_treasury, vault_key);
        let sqrt_price_limit = tick_math::get_default_sqrt_price_limit(_a2b);
        vault::swap<StableCoinType>(
            mut_vault,
            _coin_a,
            _coin_b,
            _a2b,
            _by_amount_in,
            _amount,
            0, // ? unuse
            sqrt_price_limit,
            _ctx
        )
    }

    /// Rebalance
    public(friend) fun next_epoch_obc_required(_treasury: &Treasury): u64 {
        let total = 0;
        let times_per_day = (3600 * 24 / _treasury.time_interval as u64);

        // USD obc required
        let usd_v = borrow_vault<USD>(
            _treasury,
            get_vault_key<USD>(),
        );
        let obc_required_per_time = vault::obc_required(usd_v);
        total = total + obc_required_per_time * times_per_day;

        total - get_balance(_treasury)
    }

    public(friend) fun deposit(_treasury: &mut Treasury, _coin_obc: Coin<OBC>) {
        let min_amount = next_epoch_obc_required(_treasury);
        let input = coin::into_balance(_coin_obc);
        let input_amount = balance::value(&input);
        assert!(input_amount >= min_amount, ERR_INSUFFICIENT);
        balance::join(&mut _treasury.obc_balance, input);
        event::deposit(input_amount);

        if (!_treasury.init) {
            _treasury.init = true
        }
    }

    public(friend) fun rebalance(
        _treasury: &mut Treasury,
        clock: &Clock,
        _ctx: &mut TxContext,
    ) {
        // check init
        if (!_treasury.init) {
            return
        };

        // check time_interval
        let current_ts = clock::timestamp_ms(clock) / 1000;
        if ((current_ts - _treasury.updated_at) < (_treasury.time_interval as u64)) {
            return
        };

        // update updated_at
        _treasury.updated_at = current_ts;
        let usd_mut_v = dynamic_object_field::borrow_mut<String, Vault<USD>>(
            &mut _treasury.id,
            get_vault_key<USD>()
        );
        vault::update_state(usd_mut_v);

        vault::rebalance(
            usd_mut_v,
            &mut _treasury.obc_balance,
            bag::borrow_mut<String, Supply<USD>>(&mut _treasury.supplies, get_vault_key<USD>()),
            _ctx
        );
    }
}

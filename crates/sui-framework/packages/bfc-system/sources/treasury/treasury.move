#[allow(unused_const)]
module bfc_system::treasury {
    use std::ascii::String;
    use std::type_name;

    use sui::bag::{Self, Bag};
    use sui::balance::{Self, Balance, Supply};
    use sui::bfc::BFC;
    use sui::clock::{Self, Clock};
    use sui::coin::{Self, Coin};
    use sui::dynamic_field;
    use sui::vec_map::{Self, VecMap};

    use bfc_system::bars::BARS;
    use bfc_system::baud::BAUD;
    use bfc_system::bbrl::BBRL;
    use bfc_system::bcad::BCAD;
    use bfc_system::beur::BEUR;
    use bfc_system::bgbp::BGBP;
    use bfc_system::bidr::BIDR;
    use bfc_system::binr::BINR;
    use bfc_system::bjpy::BJPY;
    use bfc_system::bkrw::BKRW;
    use bfc_system::bmxn::BMXN;
    use bfc_system::brub::BRUB;
    use bfc_system::bsar::BSAR;
    use bfc_system::btry::BTRY;
    use bfc_system::busd::BUSD;
    use bfc_system::bzar::BZAR;
    use bfc_system::mgg::MGG;
    use bfc_system::event;
    use bfc_system::tick_math;
    use bfc_system::vault::{Self, Vault, VaultInfo};
    use bfc_system::position::Position;
    use bfc_system::tick::Tick;

    // friend bfc_system::bfc_system_state_inner;
    // #[test_only]
    // friend bfc_system::treasury_busd_test;
    // #[test_only]
    // friend bfc_system::treasury_beur_test;
    // #[test_only]
    // friend bfc_system::vault_test;
    // #[test_only]
    // friend bfc_system::test_utils;
    // #[test_only]
    // friend bfc_system::bfc_system_tests;

    // === Errors ===
    const ERR_POOL_HAS_REGISTERED: u64 = 100;
    const ERR_POOL_NOT_EXISTS: u64 = 101;
    const ERR_ZERO_AMOUNT: u64 = 102;
    const ERR_INSUFFICIENT: u64 = 103;
    const ERR_UNINITIALIZE_TREASURY: u64 = 104;
    const ERR_DEADLINE_EXCEED: u64 = 105;

    public struct TreasuryPauseCap has key, store {
        id: UID
    }

    public struct Treasury has key, store {
        id: UID,
        bfc_balance: Balance<BFC>,
        /// stable coin supplies
        supplies: Bag,
        /// Vault index
        index: u64,
        time_interval: u32,
        updated_at: u64,
        init: bool,
        total_bfc_supply: u64,
    }

    //spec module { pragma verify = false; }

    // call in bfc_system
    public(package) fun create_treasury(time_interval: u32, total_bfc_supply: u64, ctx: &mut TxContext): Treasury {
        let treasury = Treasury {
            id: object::new(ctx),
            bfc_balance: balance::zero<BFC>(),
            supplies: bag::new(ctx),
            index: 0,
            time_interval,
            updated_at: 0,
            init: false,
            total_bfc_supply: total_bfc_supply,
        };
        let treasury_id = object::id(&treasury);
        event::init_treasury(treasury_id);
        treasury
    }

    // call in bfc_system
    public(package) fun create_treasury_pause_cap(admin: address, ctx: &mut TxContext) {
        transfer::transfer(TreasuryPauseCap { id: object::new(ctx) }, admin);
    }

    public fun index(_treasury: &Treasury): u64 {
        _treasury.index
    }

    public fun get_balance(_treasury: &Treasury): u64 {
        balance::value(&_treasury.bfc_balance)
    }

    fun check_vault(_treasury: &Treasury, _vault_key: String) {
        assert!(
            dynamic_field::exists_(
                &_treasury.id,
                _vault_key
            ),
            ERR_POOL_NOT_EXISTS
        );
    }

    public fun get_vault_key<StableCoinType>(): String {
        type_name::into_string(type_name::get<StableCoinType>())
    }

    public(package) fun borrow_vault<StableCoinType>(
        _treasury: &Treasury,
        _vault_key: String
    ): &Vault<StableCoinType> {
        check_vault(_treasury, _vault_key);
        dynamic_field::borrow<String, Vault<StableCoinType>>(&_treasury.id, _vault_key)
    }

    public(package) fun borrow_mut_vault<StableCoinType>(
        _treasury: &mut Treasury,
        _vault_key: String
    ): &mut Vault<StableCoinType> {
        check_vault(_treasury, _vault_key);
        dynamic_field::borrow_mut<String, Vault<StableCoinType>>(&mut _treasury.id, _vault_key)
    }

    public fun vault_info<StableCoinType>(_treasury: &Treasury): VaultInfo {
        vault::vault_info(
            borrow_vault<StableCoinType>(_treasury, get_vault_key<StableCoinType>())
        )
    }

    public(package) fun vault_set_pause<StableCoinType>(_: &TreasuryPauseCap, _treasury: &mut Treasury, _pause: bool) {
        vault::set_pause(
            borrow_mut_vault<StableCoinType>(_treasury, get_vault_key<StableCoinType>()),
            _pause,
        );
    }

    public fun fetch_ticks<StableCoinType>(_treasury: &Treasury): vector<Tick> {
        vault::fetch_ticks(
            borrow_vault<StableCoinType>(_treasury, get_vault_key<StableCoinType>())
        )
    }

    public fun fetch_positions<StableCoinType>(_treasury: &Treasury): vector<Position> {
        vault::fetch_positions(
            borrow_vault<StableCoinType>(_treasury, get_vault_key<StableCoinType>())
        )
    }


    public(package) fun create_vault<StableCoinType>(
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

    public(package) fun init_vault_with_positions<StableCoinType>(
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
        assert!(!dynamic_field::exists_<String>(&_treasury.id, vault_key), ERR_POOL_HAS_REGISTERED);

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

        dynamic_field::add(
            &mut _treasury.id,
            vault_key,
            new_vault,
        );
        bag::add<String, Supply<StableCoinType>>(&mut _treasury.supplies, vault_key, _supply);
        vault_key
    }

    ///  ======= Swap
    /// Mint swap bfc to stablecoin
    public entry fun mint<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_bfc: Coin<BFC>,
        _clock: &Clock,
        _amount: u64,
        _min_amount: u64,
        _deadline: u64,
        _ctx: &mut TxContext,
    ) {
        let balance_a = mint_internal<StableCoinType>(
            _treasury,
            _coin_bfc,
            _amount,
            _ctx,
        );
        assert!(balance::value(&balance_a) >= _min_amount, ERR_INSUFFICIENT);
        assert!(clock::timestamp_ms(_clock) <= _deadline, ERR_DEADLINE_EXCEED);
        transfer_or_delete(balance_a, _ctx);
    }

    public(package) fun mint_internal<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_bfc: Coin<BFC>,
        _amount: u64,
        _ctx: &mut TxContext,
    ): Balance<StableCoinType> {
        assert!(coin::value<BFC>(&_coin_bfc) > 0, ERR_ZERO_AMOUNT);
        let (balance_a, balance_b) = swap_internal<StableCoinType>(
            _treasury,
            false,
            coin::zero<StableCoinType>(_ctx),
            _coin_bfc,
            _amount,
            true,
            _ctx,
        );
        transfer_or_delete(balance_b, _ctx);
        balance_a
    }

    /// Burn swap stablecoin to bfc
    public entry fun redeem<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_sc: Coin<StableCoinType>,
        _clock: &Clock,
        _amount: u64,
        _min_amount: u64,
        _deadline: u64,
        _ctx: &mut TxContext,
    ) {
        assert!(coin::value<StableCoinType>(&_coin_sc) > 0, ERR_ZERO_AMOUNT);
        let balance_b = redeem_internal<StableCoinType>(
            _treasury,
            _coin_sc,
            _amount,
            _ctx,
        );
        assert!(balance::value(&balance_b) >= _min_amount, ERR_INSUFFICIENT);
        assert!(clock::timestamp_ms(_clock) <= _deadline, ERR_DEADLINE_EXCEED);
        transfer_or_delete(balance_b, _ctx);
    }

    public(package) fun redeem_internal<StableCoinType>(
        _treasury: &mut Treasury,
        _coin_sc: Coin<StableCoinType>,
        _amount: u64,
        _ctx: &mut TxContext,
    ): Balance<BFC> {
        assert!(coin::value<StableCoinType>(&_coin_sc) > 0, ERR_ZERO_AMOUNT);
        let (balance_a, balance_b) = swap_internal<StableCoinType>(
            _treasury,
            true,
            _coin_sc,
            coin::zero<BFC>(_ctx),
            _amount,
            true,
            _ctx,
        );
        transfer_or_delete(balance_a, _ctx);
        balance_b
    }

    /// Burn swap stablecoin to bfc
    public fun calculate_swap_result<StableCoinType>(
        _treasury: &Treasury,
        _a2b: bool,
        _amount: u64
    ): vault::CalculatedSwapResult
    {
        let sc_vault = borrow_vault<StableCoinType>(_treasury, get_vault_key<StableCoinType>());
        vault::calculate_swap_result(sc_vault, _a2b, true, _amount)
    }
    #[allow(lint(self_transfer))]
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
        _coin_b: Coin<BFC>,
        _amount: u64,
        _by_amount_in: bool,
        _ctx: &mut TxContext,
    ): (Balance<StableCoinType>, Balance<BFC>) {
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

    public(package) fun deposit(_treasury: &mut Treasury, _coin_bfc: Coin<BFC>) {
        let min_amount = bfc_required(_treasury);
        let input = coin::into_balance(_coin_bfc);
        let input_amount = balance::value(&input);
        assert!(input_amount >= min_amount, ERR_INSUFFICIENT);
        balance::join(&mut _treasury.bfc_balance, input);

        if (!_treasury.init) {
            _treasury.init = true
        }
    }

    /// Rebalance
    public(package) fun bfc_required(_treasury: &Treasury): u64 {
        let treasury_total_bfc_supply = _treasury.total_bfc_supply;

        let total = one_coin_bfc_required<BUSD>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<MGG>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BJPY>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BAUD>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BKRW>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BBRL>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BCAD>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BEUR>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BGBP>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BIDR>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BINR>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BRUB>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BSAR>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BTRY>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BZAR>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BMXN>(_treasury, treasury_total_bfc_supply) +
            one_coin_bfc_required<BARS>(_treasury, treasury_total_bfc_supply);

        let get_treasury_balance = get_balance(_treasury);
        if (total > get_treasury_balance) {
            total - get_treasury_balance
        } else {
            0
        }
    }

    public(package) fun rebalance(
        _treasury: &mut Treasury,
        _pool_balance: u64,
        _update: bool,
        _clock: &Clock,
        _ctx: &mut TxContext,
    ) {
        // check init
        if (!_treasury.init) {
            return
        };

        let current_ts = clock::timestamp_ms(_clock) / 1000;

        if ((current_ts - _treasury.updated_at) < (_treasury.time_interval as u64)) {
            return
        };

        // update updated_at
        _treasury.updated_at = current_ts;
        let bfc_in_vault = rebalance_internal(_treasury, _update, _ctx);
        _treasury.total_bfc_supply = _pool_balance + bfc_in_vault + balance::value(&_treasury.bfc_balance);
    }

    public(package) fun rebalance_internal(
        _treasury: &mut Treasury,
        _update: bool,
        _ctx: &mut TxContext
    ): u64 {
        let mut bfc_in_vault = 0;
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BUSD>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<MGG>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BJPY>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BKRW>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BAUD>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BARS>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BBRL>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BCAD>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BEUR>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BGBP>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BIDR>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BINR>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BRUB>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BSAR>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BTRY>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BZAR>(_treasury, _update, _ctx);
        bfc_in_vault = bfc_in_vault + one_coin_rebalance_internal<BMXN>(_treasury, _update, _ctx);
        bfc_in_vault
    }


    public(package) fun get_exchange_rates(
        _treasury: &Treasury,
    ): VecMap<String, u64> {
        let mut rate_map = vec_map::empty<String, u64>();
        let amount = 1_000_000_000;

        one_coin_exchange_rate<BUSD>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<MGG>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BJPY>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BKRW>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BAUD>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BARS>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BBRL>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BCAD>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BEUR>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BGBP>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BIDR>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BINR>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BRUB>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BSAR>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BTRY>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BZAR>(_treasury, &mut rate_map, amount);
        one_coin_exchange_rate<BMXN>(_treasury, &mut rate_map, amount);

        rate_map
    }

    public(package) fun get_total_supply<StableCoinType>(
        _self: &Treasury
    ): u64
    {
        let key = get_vault_key<StableCoinType>();
        if (!dynamic_field::exists_(&_self.id, key)) {
            return 0
        };
        let supply = bag::borrow<String, Supply<StableCoinType>>(&_self.supplies, key);
        balance::supply_value(supply)
    }

    fun one_coin_rebalance_internal<StableCoinType>(
        _treasury: &mut Treasury,
        _update: bool,
        _ctx: &mut TxContext
    ): u64 {
        let key = get_vault_key<StableCoinType>();
        if (!dynamic_field::exists_(&_treasury.id, key)) {
            return 0
        };
        let mut_v = dynamic_field::borrow_mut<String, Vault<StableCoinType>>(
            &mut _treasury.id,
            key,
        );
        if (_update) {
            vault::update_state(mut_v);
        };

        // first rebalance just place liquidity not change vault state
        vault::rebalance(
            mut_v,
            &mut _treasury.bfc_balance,
            bag::borrow_mut<String, Supply<StableCoinType>>(&mut _treasury.supplies, key),
            _treasury.total_bfc_supply,
            _ctx
        )
    }

    fun one_coin_bfc_required<StableCoinType>(
        _treasury: &Treasury,
        _treasury_total_bfc_supply: u64
    ): u64 {
        let key = get_vault_key<StableCoinType>();
        if (dynamic_field::exists_(&_treasury.id, key)) {
            vault::bfc_required(borrow_vault<StableCoinType>(_treasury, key), _treasury_total_bfc_supply)
        } else {
            0
        }
    }

    fun one_coin_exchange_rate<StableCoinType>(
        _treasury: &Treasury,
        _rate_map: &mut VecMap<String, u64>,
        _amount: u64
    ) {
        let key = get_vault_key<StableCoinType>();
        if (!dynamic_field::exists_(&_treasury.id, key)) {
            return
        };
        vec_map::insert(
            _rate_map,
            key,
            vault::calculated_swap_result_amount_out(&calculate_swap_result<StableCoinType>(_treasury, true, _amount)),
        );
    }
}

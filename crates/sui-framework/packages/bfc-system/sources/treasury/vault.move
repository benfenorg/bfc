module bfc_system::vault {
use std::ascii::String;
use std::type_name;
    use std::vector;

    use sui::balance::{Self, Balance, Supply};
    use sui::coin::{Self, Coin};
    use sui::bfc::BFC;
    use sui::object;
    use sui::object::{ID, UID};
    use sui::tx_context::TxContext;

    use bfc_system::clmm_math;
    use bfc_system::event;
    use bfc_system::i32::{Self, I32};
    use bfc_system::math_u128;
    use bfc_system::math_u64;
    use bfc_system::option_u64;
    use bfc_system::position::{Self, Position, PositionManager};
    use bfc_system::tick::{Self, Tick, TickManager};
    use bfc_system::tick_math;

    friend bfc_system::treasury;
    friend bfc_system::bfc_system;
    friend bfc_system::bfc_system_state_inner;
    #[test_only]
    friend bfc_system::treasury_test;

    #[test_only]
    friend bfc_system::vault_test;

    const ERR_AMOUNT_INSUFFICIENT: u64 = 200;
    const ERR_MAX_AMOUNT: u64 = 201;
    const ERR_MAX_LIQUIDITY: u64 = 202;
    const ERR_POOL_INVALID: u64 = 203;
    const ERR_PAY_AMOUNT_INVALID: u64 = 204;
    const ERR_LIQUIDITY_DELTA_IS_ZERO: u64 = 205;
    const ERR_AMOUNT_IS_ZERO: u64 = 206;
    const ERR_POOL_IS_PAUSE: u64 = 207;
    const ERR_SQRT_PRICE_LIMIT_INVALID: u64 = 208;
    const ERR_TICK_INDEX_OPTION_IS_NONE: u64 = 209;
    const ERR_AMOUNT_MISMATCH: u64 = 210;
    const ERR_POSITIONS_IS_NOT_EMPTY: u64 = 211;
    const ERR_INVALID_SHAPE_KINDS: u64 = 212;

    const SHAPE_EQUAL_SIZE: u8 = 0;
    const SHAPE_DECREMENT_SIZE: u8 = 1;
    const SHAPE_INCREMENT_SIZE: u8 = 2;

    spec module { pragma verify = false; }

    struct Vault<phantom StableCoinType> has key, store {
        id: UID,

        position_number: u32,

        /// 0 -- init, equal, 1 -- down, 2 -- up
        state: u8,
        state_counter: u32,
        max_counter_times: u32,
        last_sqrt_price: u128,

        coin_a: Balance<StableCoinType>,
        coin_b: Balance<BFC>,

        /// The tick spacing
        tick_spacing: u32,
        /// The tick spacing times
        spacing_times: u32,

        /// The liquidity of current tick index
        liquidity: u128,

        /// The current sqrt price
        current_sqrt_price: u128,

        /// The current tick index
        current_tick_index: I32,

        /// The tick manager
        tick_manager: TickManager,

        /// The position manager
        position_manager: PositionManager,

        /// is the vault pause
        is_pause: bool,

        /// The vault index
        index: u64,

        base_point: u64,
    }

    struct VaultInfo has copy, drop {
        vault_id: ID,
        position_number: u32,
        state: u8,
        state_counter: u32,
        max_counter_times: u32,
        last_sqrt_price: u128,
        coin_a_balance: u64,
        coin_b_balance: u64,
        coin_a_type: String,
        coin_b_type: String,
        tick_spacing: u32,
        spacing_times: u32,
        liquidity: u128,
        current_sqrt_price: u128,
        current_tick_index: u32,
        is_pause: bool,
        index: u64,
        base_point: u64,
    }

    spec create_vault {
        pragma opaque;
    }
    // === Create vault ====
    public(friend) fun create_vault<StableCoinType>(
        _index: u64,
        _tick_spacing: u32,
        _spacing_times: u32,
        _position_number: u32,
        _initialize_price: u128,
        _base_point: u64,
        _max_counter_times: u32,
        _ts: u64,
        _ctx: &mut TxContext,
    ): Vault<StableCoinType> {
        let current_tick_index = tick_math::get_tick_at_sqrt_price(_initialize_price);
        let valid_index = tick_math::get_next_valid_tick_index(current_tick_index, _tick_spacing);
        let uid = object::new(_ctx);
        let pid = object::uid_to_inner(&uid);
        let current_sqrt_price = tick_math::get_sqrt_price_at_tick(valid_index);
        Vault {
            id: uid,
            position_number: _position_number,
            state: 0,
            state_counter: 0,
            last_sqrt_price: current_sqrt_price,
            coin_a: balance::zero<StableCoinType>(),
            coin_b: balance::zero<BFC>(),
            tick_spacing: _tick_spacing,
            spacing_times: _spacing_times,
            liquidity: 0,
            current_sqrt_price,
            current_tick_index: valid_index,
            tick_manager: tick::create_tick_manager(_tick_spacing, _ts, _ctx),
            position_manager: position::create_position_manager(pid, _tick_spacing, _ctx),
            is_pause: false,
            index: _index,
            base_point: _base_point,
            max_counter_times: _max_counter_times,
        }
    }

    /// open `position_number` positions
    public(friend) fun init_positions<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _spacing_times: u32,
        _ctx: &mut TxContext
    ): vector<vector<I32>> {
        assert!(position::get_total_positions(&_vault.position_manager) == 0, ERR_POSITIONS_IS_NOT_EMPTY);
        let ticks = tick::get_ticks(
            &_vault.tick_manager,
            _vault.current_tick_index,
            _spacing_times,
            _vault.position_number,
        );
        let index = 0;
        while (index < vector::length(&ticks)) {
            let current = vector::borrow(&ticks, index);
            open_position(
                _vault,
                *vector::borrow(current, 0),
                *vector::borrow(current, 1),
                _ctx,
            );
            index = index + 1;
        };
        ticks
    }

    public(friend) fun open_position<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _tick_lower: I32,
        _tick_upper: I32,
        _ctx: &mut TxContext
    ) {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let position_id = position::open_position<StableCoinType>(
            &mut _vault.position_manager,
            _vault.index,
            _tick_lower,
            _tick_upper,
            _ctx,
        );
        event::open_position(
            vault_id(_vault),
            position_id,
            _tick_lower,
            _tick_upper,
        )
    }

    public(friend) fun close_position<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _index: u64
    )
    {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        position::close_position(
            &mut _vault.position_manager,
            _index
        );
        event::close_position(vault_id(_vault), _index)
    }

    /// Flash loan resource for add_liquidity
    struct AddLiquidityReceipt<phantom StableCoinType> {
        vault_id: ID,
        amount_a: u64,
        amount_b: u64
    }

    spec add_liquidity_internal {
        pragma opaque;
    }
    fun add_liquidity_internal<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _index: u64,
        _use_amount: bool,
        _liquidity_delta: u128,
        _amount: u64,
        _fix_amount_a: bool
    ): AddLiquidityReceipt<StableCoinType> {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let expect_vault_id = vault_id(_vault);
        let mut_position = position::borrow_mut_position(
            &mut _vault.position_manager,
            _index
        );
        let (tick_lower, tick_upper) = position::get_tick_range(mut_position);
        let _vault_id = position::get_vault_id(mut_position);
        assert!(_vault_id == expect_vault_id, ERR_POOL_INVALID);
        let liquidity_delta: u128;
        let amount_a: u64;
        let amount_b: u64;
        if (_use_amount) {
            (liquidity_delta, amount_a, amount_b) = clmm_math::get_liquidity_by_amount(
                tick_lower,
                tick_upper,
                _vault.current_tick_index,
                _vault.current_sqrt_price,
                _amount,
                _fix_amount_a,
            );
        } else {
            liquidity_delta = _liquidity_delta;
            (amount_a, amount_b) = clmm_math::get_amount_by_liquidity(
                tick_lower,
                tick_upper,
                _vault.current_tick_index,
                _vault.current_sqrt_price,
                _liquidity_delta,
                true,
            );
        };
        let liquidity = position::increase_liquidity(mut_position, liquidity_delta);
        tick::increase_liquidity(
            &mut _vault.tick_manager,
            _vault.current_tick_index,
            tick_lower,
            tick_upper,
            liquidity_delta,
        );
        let is_in = false;
        if (i32::gte(_vault.current_tick_index, tick_lower)) {
            is_in = i32::lt(_vault.current_tick_index, tick_upper);
        };

        if (is_in) {
            assert!(math_u128::add_check(_vault.liquidity, liquidity), ERR_MAX_LIQUIDITY);
            _vault.liquidity = _vault.liquidity + liquidity;
        };
        event::add_liquidity(
            _vault_id,
            _index,
            tick_lower,
            tick_upper,
            liquidity_delta,
            _vault.liquidity,
            amount_a,
            amount_b
        );
        AddLiquidityReceipt<StableCoinType> {
            vault_id: _vault_id,
            amount_a,
            amount_b,
        }
    }

    public(friend) fun add_liquidity<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _index: u64,
        _delta_liquidity: u128
    ): AddLiquidityReceipt<StableCoinType> {
        assert!(_delta_liquidity > 0, ERR_LIQUIDITY_DELTA_IS_ZERO);
        add_liquidity_internal(
            _vault,
            _index,
            false,
            _delta_liquidity,
            0u64,
            false
        )
    }


    spec remove_liquidity {
        pragma opaque;
    }

    public(friend) fun remove_liquidity<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _index: u64,
        _delta_liquidity: u128
    ): (Balance<StableCoinType>, Balance<BFC>) {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let expect_vault_id = vault_id(_vault);
        let mut_position = position::borrow_mut_position(
            &mut _vault.position_manager,
            _index
        );
        let (tick_lower, tick_upper) = position::get_tick_range(mut_position);
        let _vault_id = position::get_vault_id(mut_position);
        assert!(_vault_id == expect_vault_id, ERR_POOL_INVALID);
        let liquidity = position::decrease_liquidity(mut_position, _delta_liquidity);
        tick::decrease_liquidity(
            &mut _vault.tick_manager,
            _vault.current_tick_index,
            tick_lower,
            tick_upper,
            _delta_liquidity,
        );
        let is_in = false;
        if (i32::lte(tick_lower, _vault.current_tick_index)) {
            is_in = i32::lt(_vault.current_tick_index, tick_upper);
        };

        if (is_in) {
            _vault.liquidity = _vault.liquidity - liquidity;
        };

        let (amount_a, amount_b) = clmm_math::get_amount_by_liquidity(
            tick_lower,
            tick_upper,
            _vault.current_tick_index,
            _vault.current_sqrt_price,
            _delta_liquidity,
            false,
        );
        let balance_a = balance::split(&mut _vault.coin_a, amount_a);
        let balance_b = balance::split(&mut _vault.coin_b, amount_b);
        event::remove_liquidity(
            _vault_id,
            _index,
            tick_lower,
            tick_upper,
            _delta_liquidity,
            _vault.liquidity,
            amount_a,
            amount_b
        );
        (balance_a, balance_b)
    }

    public(friend) fun add_liquidity_fix_coin<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _index: u64,
        _amount: u64,
        _fix_amount_a: bool
    ): AddLiquidityReceipt<StableCoinType> {
        assert!(_amount > 0, ERR_AMOUNT_IS_ZERO);
        add_liquidity_internal(
            _vault,
            _index,
            true,
            0u128,
            _amount,
            _fix_amount_a
        )
    }

    public(friend) fun repay_add_liquidity<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _balance_a: Balance<StableCoinType>,
        _balance_b: Balance<BFC>,
        _receipt: AddLiquidityReceipt<StableCoinType>
    )
    {
        let AddLiquidityReceipt { vault_id, amount_a, amount_b } = _receipt;
        assert!(balance::value(&_balance_a) == amount_a, ERR_AMOUNT_MISMATCH);
        assert!(balance::value(&_balance_b) == amount_b, ERR_AMOUNT_MISMATCH);
        assert!(vault_id == object::id(_vault), ERR_POOL_INVALID);
        balance::join(&mut _vault.coin_a, _balance_a);
        balance::join(&mut _vault.coin_b, _balance_b);
    }

    /// The step swap result
    struct SwapStepResult has copy, drop, store {
        current_sqrt_price: u128,
        target_sqrt_price: u128,
        current_liquidity: u128,
        current_tick_index: I32,
        amount_in: u64,
        amount_out: u64,
        remainer_amount: u64
    }

    /// The calculated swap result
    struct CalculatedSwapResult has copy, drop, store {
        amount_in: u64,
        amount_out: u64,
        after_sqrt_price: u128,
        is_exceed: bool,
        step_results: vector<SwapStepResult>,
        steps: u64
    }

    public fun calculated_swap_result_amount_out(_calculatedSwapResult: &CalculatedSwapResult): u64 {
        _calculatedSwapResult.amount_out
    }

    public fun calculated_swap_result_is_exceed(_calculatedSwapResult: &CalculatedSwapResult): bool {
        _calculatedSwapResult.is_exceed
    }

    public fun calculated_swap_result_amount_in(_calculatedSwapResult: &CalculatedSwapResult): u64 {
        _calculatedSwapResult.amount_in
    }

    public fun calculated_swap_result_after_sqrt_price(_calculatedSwapResult: &CalculatedSwapResult): u128 {
        _calculatedSwapResult.after_sqrt_price
    }

    public fun calculate_swap_result_step_results(
        _calculatedSwapResult: &CalculatedSwapResult
    ): &vector<SwapStepResult> {
        &_calculatedSwapResult.step_results
    }

    public(friend) fun default_calculated_swap_result(): CalculatedSwapResult {
        CalculatedSwapResult {
            amount_in: 0,
            amount_out: 0,
            steps: 0,
            step_results: vector::empty(),
            is_exceed: false,
            after_sqrt_price: 0
        }
    }

    fun check_remainer_amount_sub(amount: u64, amount_in: u64): u64 {
        assert!(amount >= amount_in, ERR_AMOUNT_INSUFFICIENT);
        amount - amount_in
    }

    fun update_swap_result(_swap_result: &mut CalculatedSwapResult, _in: u64, _out: u64) {
        assert!(math_u64::add_check(_swap_result.amount_in, _in), ERR_MAX_AMOUNT);
        assert!(math_u64::add_check(_swap_result.amount_out, _out), ERR_MAX_AMOUNT);
        _swap_result.amount_in = _swap_result.amount_in + _in;
        _swap_result.amount_out = _swap_result.amount_out + _out;
    }


    spec calculate_swap_result {
        pragma opaque;
    }
    // Calculate Swap Result
    public fun calculate_swap_result<StableCoinType>(
        _vault: &Vault<StableCoinType>,
        _a2b: bool,
        _by_amount_in: bool,
        _amount: u64,
    ): CalculatedSwapResult {
        let swap_result = default_calculated_swap_result();
        swap_result.after_sqrt_price = _vault.current_sqrt_price;
        let liquidity = _vault.liquidity;
        let current_sqrt_price = _vault.current_sqrt_price;
        let remainer_amount = _amount;
        let tick_index = _vault.current_tick_index;
        let start_score = tick::first_score_for_swap(
            &_vault.tick_manager,
            tick_index,
            _a2b,
        );
        while (remainer_amount > 0) {
            if (option_u64::is_none(&start_score)) {
                swap_result.is_exceed = true;
                break
            };
            // get next tick
            let (next_tick, next_score) = tick::borrow_tick_for_swap(
                &_vault.tick_manager,
                option_u64::borrow(&start_score),
                _a2b,
            );
            start_score = next_score;

            let target_sqrt_price = tick::sqrt_price(next_tick);
            let (amount_in, amount_out, next_sqrt_price) = clmm_math::compute_swap_step(
                current_sqrt_price,
                target_sqrt_price,
                liquidity,
                remainer_amount,
                _a2b,
                _by_amount_in,
            );

            if (amount_in != 0) {
                if (_by_amount_in) {
                    remainer_amount = check_remainer_amount_sub(remainer_amount, amount_in);
                } else {
                    remainer_amount = check_remainer_amount_sub(remainer_amount, amount_out);
                };
                update_swap_result(&mut swap_result, amount_in, amount_out);
            };
            vector::push_back(&mut swap_result.step_results, SwapStepResult {
                current_sqrt_price,
                target_sqrt_price,
                current_liquidity: liquidity,
                amount_in,
                amount_out,
                remainer_amount,
                current_tick_index: tick_index,
            });
            if (target_sqrt_price == next_sqrt_price) {
                current_sqrt_price = target_sqrt_price;
                liquidity = tick::cross_by_tick(next_tick, _a2b, liquidity);
                tick_index = tick::tick_index(next_tick);
            } else {
                current_sqrt_price = next_sqrt_price
            };
            swap_result.after_sqrt_price = current_sqrt_price;
        };
        swap_result
    }

    /// Flash loan resource for swap.
    /// There is no way in Move to pass calldata and make dynamic calls, but a resource can be used for this purpose.
    /// To make the execution into a single transaction, the flash loan function must return a resource
    /// that cannot be copied, cannot be saved, cannot be dropped, or cloned.
    struct FlashSwapReceipt<phantom StableCoinType> {
        vault_id: ID,
        a2b: bool,
        pay_amount: u64,
    }

    public(friend) fun swap<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _coin_a: Coin<StableCoinType>,
        _coin_b: Coin<BFC>,
        _a2b: bool,
        _by_amount_in: bool,
        _amount: u64,
        _amount_limit: u64,
        _sqrt_price_limit: u128,
        _ctx: &mut TxContext
    ): (Balance<StableCoinType>, Balance<BFC>) {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let (
            receive_a,
            receive_b,
            flash_receipt
        ) = flash_swap_internal<StableCoinType>(
            _vault,
            _a2b,
            _by_amount_in,
            _amount,
            _sqrt_price_limit
        );

        let pay_amount = flash_receipt.pay_amount;
        let pay_coin_a;
        let pay_coin_b;

        if (_a2b) {
            pay_coin_a = coin::into_balance(coin::split(&mut _coin_a, pay_amount, _ctx));
            pay_coin_b = balance::zero<BFC>();
        } else {
            pay_coin_a = balance::zero<StableCoinType>();
            pay_coin_b = coin::into_balance(coin::split(&mut _coin_b, pay_amount, _ctx));
        };

        coin::join(&mut _coin_a, coin::from_balance(receive_a, _ctx));
        coin::join(&mut _coin_b, coin::from_balance(receive_b, _ctx));

        repay_flash_swap<StableCoinType>(
            _vault,
            pay_coin_a,
            pay_coin_b,
            flash_receipt
        );
        (coin::into_balance(_coin_a), coin::into_balance(_coin_b))
    }

    fun repay_flash_swap<StableCoinType>(
        vault: &mut Vault<StableCoinType>,
        balance_a: Balance<StableCoinType>,
        balance_b: Balance<BFC>,
        receipt: FlashSwapReceipt<StableCoinType>
    ) {
        let FlashSwapReceipt<StableCoinType> {
            vault_id: _vault_id,
            a2b,
            pay_amount
        } = receipt;

        assert!(_vault_id == vault_id(vault), ERR_POOL_INVALID);
        assert!(pay_amount > 0, ERR_PAY_AMOUNT_INVALID);

        if (a2b) {
            assert!(balance::value(&balance_a) == pay_amount, ERR_PAY_AMOUNT_INVALID);
            balance::join(&mut vault.coin_a, balance_a);
            balance::destroy_zero(balance_b);
        } else {
            assert!(balance::value(&balance_b) == pay_amount, ERR_PAY_AMOUNT_INVALID);
            balance::join(&mut vault.coin_b, balance_b);
            balance::destroy_zero(balance_a);
        };
    }

    fun flash_swap_internal<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _a2b: bool,
        _by_amount_in: bool,
        _amount: u64,
        _sqrt_price_limit: u128
    ): (Balance<StableCoinType>, Balance<BFC>, FlashSwapReceipt<StableCoinType>)
    {
        let min_price = tick_math::min_sqrt_price();
        let max_price = tick_math::max_sqrt_price();
        if (_a2b) {
            assert!(
                min_price <= _sqrt_price_limit && _sqrt_price_limit < _vault.current_sqrt_price,
                ERR_SQRT_PRICE_LIMIT_INVALID
            );
        } else {
            assert!(
                _vault.current_sqrt_price < _sqrt_price_limit && _sqrt_price_limit <= max_price,
                ERR_SQRT_PRICE_LIMIT_INVALID
            );
        };
        let before_sqrt_price = _vault.current_sqrt_price;
        let swap_res = swap_in_vault(_vault, _a2b, _by_amount_in, _sqrt_price_limit, _amount);
        let balance_a_ret;
        let balance_b_ret;
        let coin_type_in: String;
        let coin_type_out: String;

        if (_a2b) {
            balance_b_ret = balance::split<BFC>(&mut _vault.coin_b, swap_res.amount_out);
            balance_a_ret = balance::zero<StableCoinType>();
            coin_type_in = type_name::into_string(type_name::get<StableCoinType>());
            coin_type_out = type_name::into_string(type_name::get<BFC>());
        } else {
            balance_a_ret = balance::split<StableCoinType>(&mut _vault.coin_a, swap_res.amount_out);
            balance_b_ret = balance::zero<BFC>();
            coin_type_in = type_name::into_string(type_name::get<BFC>());
            coin_type_out = type_name::into_string(type_name::get<StableCoinType>());
        };
        event::swap(
            vault_id(_vault),
            _a2b,
            coin_type_in,
            coin_type_out,
            swap_res.amount_in,
            swap_res.amount_out,
            balance::value(&balance_a_ret),
            balance::value(&balance_b_ret),
            before_sqrt_price,
            _vault.current_sqrt_price,
            swap_res.steps
        );
        (balance_a_ret, balance_b_ret, FlashSwapReceipt<StableCoinType> {
            vault_id: vault_id(_vault),
            a2b: _a2b,
            pay_amount: swap_res.amount_in
        })
    }

    fun swap_in_vault<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _a2b: bool,
        _by_amount_in: bool,
        _sqrt_price_limit: u128,
        _amount: u64,
    ): CalculatedSwapResult
    {
        let swap_result = default_calculated_swap_result();
        let next_score = tick::first_score_for_swap(&_vault.tick_manager, _vault.current_tick_index, _a2b);
        let remaining_amount = _amount;
        let current_sqrt_price = _vault.current_sqrt_price;
        while (remaining_amount > 0 && current_sqrt_price != _sqrt_price_limit) {
            assert!(!option_u64::is_none(&next_score), ERR_TICK_INDEX_OPTION_IS_NONE);
            let (tick, tick_score) = tick::borrow_tick_for_swap(
                &_vault.tick_manager,
                option_u64::borrow(&next_score),
                _a2b
            );
            next_score = tick_score;
            let next_tick_index = tick::tick_index(tick);
            let next_tick_sqrt_price = tick::sqrt_price(tick);
            let target_sqrt_price = if (_a2b) {
                math_u128::max(_sqrt_price_limit, next_tick_sqrt_price)
            } else {
                math_u128::min(_sqrt_price_limit, next_tick_sqrt_price)
            };
            let (amount_in, amount_out, next_sqrt_price) = clmm_math::compute_swap_step(
                _vault.current_sqrt_price,
                target_sqrt_price,
                _vault.liquidity,
                remaining_amount,
                _a2b,
                _by_amount_in
            );
            if (amount_in != 0) {
                if (_by_amount_in) {
                    remaining_amount = check_remainer_amount_sub(remaining_amount, amount_in);
                } else {
                    remaining_amount = check_remainer_amount_sub(remaining_amount, amount_out);
                };
                update_swap_result(&mut swap_result, amount_in, amount_out);
            };
            if (next_sqrt_price == next_tick_sqrt_price) {
                _vault.current_sqrt_price = target_sqrt_price;
                let next_tick = if (_a2b) {
                    i32::sub(next_tick_index, i32::from_u32(1))
                } else {
                    next_tick_index
                };
                _vault.current_tick_index = next_tick;
                _vault.liquidity = tick::cross_by_swap(
                    &mut _vault.tick_manager,
                    _vault.current_tick_index,
                    _a2b,
                    _vault.liquidity
                );
            } else {
                if (_vault.current_sqrt_price != next_tick_sqrt_price) {
                    _vault.current_sqrt_price = next_sqrt_price;
                    _vault.current_tick_index = tick_math::get_tick_at_sqrt_price(next_sqrt_price);
                }
            };
            current_sqrt_price = _vault.current_sqrt_price;
        };
        swap_result
    }

    /// Read Functions

    /// Calculate the position's amount_a/amount_b
    /// Params
    ///     - `vault` The clmm vault object.
    ///     - `_index` The index of position.
    /// Returns
    ///     - `amount_a` The amount of `StableCoinType`
    ///     - `amount_b` The amount of `BFC`
    public fun get_position_amounts<StableCoinType>(
        _vault: &Vault<StableCoinType>,
        _index: u64,
        _round_up: bool
    ): (u64, u64) {
        let position = position::borrow_position(
            &_vault.position_manager,
            _index
        );
        let (tick_lower, tick_upper) = position::get_tick_range(position);
        clmm_math::get_amount_by_liquidity(
            tick_lower,
            tick_upper,
            _vault.current_tick_index,
            _vault.current_sqrt_price,
            position::get_liquidity(position),
            _round_up
        )
    }

    public fun get_position_liquidity<StableCoinType>(
        _vault: &Vault<StableCoinType>,
        _index: u64
    ): u128
    {
        let position = position::borrow_position(
            &_vault.position_manager,
            _index
        );
        position::get_liquidity(position)
    }

    public fun get_position_tick_range_and_price<StableCoinType>(
        _vault: &Vault<StableCoinType>,
        _index: u64
    ): (I32, I32, u128, u128)
    {
        let position = position::borrow_position(
            &_vault.position_manager,
            _index
        );
        let (tick_lower_index, tick_upper_index) = position::get_tick_range(position);
        let price_lower = tick_math::get_sqrt_price_at_tick(tick_lower_index);
        let price_upper = tick_math::get_sqrt_price_at_tick(tick_upper_index);
        (tick_lower_index, tick_upper_index, price_lower, price_upper)
    }

    public fun fetch_ticks<StableCoinType>(_vault: &Vault<StableCoinType>): vector<Tick> {
        tick::fetch_ticks(&_vault.tick_manager)
    }

    public fun fetch_positions<StableCoinType>(_vault: &Vault<StableCoinType>): vector<Position> {
        position::fetch_positions(&_vault.position_manager, 1, (_vault.position_number as u64))
    }

    /// vault info
    public fun vault_info<StableCoinType>(_vault: &Vault<StableCoinType>): VaultInfo {
        VaultInfo {
            vault_id: vault_id(_vault),
            position_number: _vault.position_number,
            state: _vault.state,
            state_counter: _vault.state_counter,
            max_counter_times: _vault.max_counter_times,
            last_sqrt_price: _vault.last_sqrt_price,
            coin_a_balance: balance::value(&_vault.coin_a),
            coin_b_balance: balance::value(&_vault.coin_b),
            coin_a_type: type_name::into_string(type_name::get<StableCoinType>()),
            coin_b_type: type_name::into_string(type_name::get<OBC>()),
            tick_spacing: _vault.tick_spacing,
            spacing_times: _vault.spacing_times,
            liquidity: _vault.liquidity,
            current_sqrt_price: _vault.current_sqrt_price,
            current_tick_index: i32::abs_u32(_vault.current_tick_index),
            is_pause: _vault.is_pause,
            index: _vault.index,
            base_point: _vault.base_point,
        } }

    public fun vault_id<StableCoinType>(_vault: &Vault<StableCoinType>): ID {
        object::id(_vault)
    }

    public fun vault_current_sqrt_price<StableCoinType>(_vault: &Vault<StableCoinType>): u128 {
        _vault.current_sqrt_price
    }

    public fun vault_current_tick_index<StableCoinType>(_vault: &Vault<StableCoinType>): I32 {
        _vault.current_tick_index
    }

    public fun balances<StableCoinType>(_vault: &Vault<StableCoinType>): (u64, u64) {
        (
            balance::value<StableCoinType>(&_vault.coin_a),
            balance::value<BFC>(&_vault.coin_b)
        )
    }

    public fun get_liquidity<StableCoinType>(_vault: &Vault<StableCoinType>): u128 {
        _vault.liquidity
    }

    public fun get_vault_state<StableCoinType>(_vault: &Vault<StableCoinType>): u8
    {
        _vault.state
    }

    public fun bfc_required<StableCoinType>(_vault: &Vault<StableCoinType>): u64 {
        ((_vault.position_number as u64) + 1) / 2 * _vault.base_point
    }

    public fun min_liquidity_rate(): u128 {
        6
    }

    public fun max_liquidity_rate(): u128 {
        14
    }

    public fun base_liquidity_rate(): u128 {
        10
    }

    /// Rebalance
    /// State checker
    public(friend) fun update_state<StableCoinType>(_vault: &mut Vault<StableCoinType>) {
        let price = _vault.current_sqrt_price;
        let last_price = _vault.last_sqrt_price;
        if (price < last_price) {
            // down
            if (_vault.state == SHAPE_INCREMENT_SIZE) {
                _vault.state_counter = _vault.state_counter + 1;
            } else {
                // reset counter = 0  & set state = down
                _vault.state_counter = 0;
                _vault.state = SHAPE_INCREMENT_SIZE;
            }
        } else if (price > last_price) {
            // up
            if (_vault.state == SHAPE_DECREMENT_SIZE) {
                _vault.state_counter = _vault.state_counter + 1;
            } else {
                // reset counter = 0  & set state = up
                _vault.state_counter = 0;
                _vault.state = SHAPE_DECREMENT_SIZE;
            }
        } else {
            // equal
            _vault.state = SHAPE_EQUAL_SIZE;
            _vault.state_counter = 0;
        };

        _vault.last_sqrt_price = price;
        event::update_state(
            price,
            last_price,
            _vault.state,
            _vault.state_counter,
        );
    }

    fun rebuild_positions_after_clean_liquidities<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _ctx: &mut TxContext
    ): (Balance<StableCoinType>, Balance<BFC>, vector<vector<I32>>)
    {
        let position_index = 1u64;
        let position_number = (_vault.position_number as u64);
        let balance0 = balance::zero<StableCoinType>();
        let balance1 = balance::zero<BFC>();
        let spacing_times = _vault.spacing_times;
        while (position_index <= position_number) {
            let position = position::borrow_mut_position(&mut _vault.position_manager, position_index);
            let liquidity_delta = position::get_liquidity(position);
            if (liquidity_delta != 0) {
                let (_balance0, _balance1) = remove_liquidity(_vault, position_index, liquidity_delta);
                balance::join(&mut balance0, _balance0);
                balance::join(&mut balance1, _balance1);
            };
            position::close_position(&mut _vault.position_manager, position_index);
            position_index = position_index + 1;
        };
        let ticks = init_positions(_vault, spacing_times, _ctx);
        (balance0, balance1, ticks)
    }

    fun get_liquidity_from_base_point<StableCoinType>(
        _vault: &Vault<StableCoinType>,
        _ticks: &vector<vector<I32>>
    ): u128
    {
        let middle_tick = vector::borrow(_ticks, (_vault.position_number / 2 as u64));
        let (tick_lower_index, tick_upper_index) = (vector::borrow(middle_tick, 0), vector::borrow(middle_tick, 1));
        let (liquidity, _, _) = clmm_math::get_liquidity_by_amount(
            *tick_lower_index,
            *tick_upper_index,
            _vault.current_tick_index,
            _vault.current_sqrt_price,
            _vault.base_point,
            false
        );
        liquidity
    }

    public(friend) fun positions_liquidity_size_balance<StableCoinType>(
        _vault: &Vault<StableCoinType>,
        _ticks: &vector<vector<I32>>,
        _shape: u8
    ): vector<u128> {
        // base point position liquidity
        let liquidity = get_liquidity_from_base_point(_vault, _ticks);
        let liquidities = vector::empty<u128>();
        let index: u128 ;
        let length: u128;
        if (_shape == SHAPE_EQUAL_SIZE) {
            index = 0;
            length = (_vault.position_number as u128);
            while (index < length) {
                vector::push_back(&mut liquidities, liquidity);
                index = index + 1;
            };
        } else if (_shape == SHAPE_INCREMENT_SIZE) {
            index = min_liquidity_rate();
            length = max_liquidity_rate();
            while (index <= length) {
                vector::push_back(&mut liquidities, liquidity * index / base_liquidity_rate());
                index = index + 1;
            };
        } else {
            assert!(_shape == SHAPE_DECREMENT_SIZE, ERR_INVALID_SHAPE_KINDS);
            index = max_liquidity_rate();
            length = min_liquidity_rate();
            while (index >= length) {
                vector::push_back(&mut liquidities, liquidity * index / base_liquidity_rate());
                index = index - 1;
            };
        };
        liquidities
    }

    fun rebalance_internal<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _bfc_balance: &mut Balance<BFC>,
        _supply: &mut Supply<StableCoinType>,
        _balance0: Balance<StableCoinType>,
        _balance1: Balance<BFC>,
        _liquidities: vector<u128>
    )
    {
        let index = 0u64;
        let length = vector::length(&_liquidities);
        let position_length = position::get_total_positions(&_vault.position_manager);
        assert!(length == position_length, 1008611);
        let (total_a, total_b) = (0, 0);
        while (index < length) {
            let receipt = add_liquidity(
                _vault,
                index + 1, // position index
                *vector::borrow(&_liquidities, index)
            );
            let AddLiquidityReceipt {
                vault_id: _,
                amount_a,
                amount_b
            } = receipt;
            total_a = amount_a + total_a;
            total_b = amount_b + total_b;
            index = index + 1;
        };

        let balance0_value = balance::value(&_balance0);
        let balance1_value = balance::value(&_balance1);
        if (balance0_value < total_a) {
            balance::join(&mut _balance0, balance::increase_supply(_supply, total_a - balance0_value));
        }  else if (balance0_value > total_a) {
            balance::decrease_supply(_supply, balance::split(&mut _balance0, balance0_value - total_a));
        };

        if (balance1_value < total_b) {
            balance::join(&mut _balance1, balance::split(_bfc_balance, total_b - balance1_value));
        } else if (balance1_value > total_a) {
            balance::join(_bfc_balance, balance::split(&mut _balance1, balance1_value - total_b));
        };

        balance::join(&mut _vault.coin_a, _balance0);
        balance::join(&mut _vault.coin_b, _balance1);
    }

    public(friend) fun rebalance<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _bfc_balance: &mut Balance<BFC>,
        _supply: &mut Supply<StableCoinType>,
        _ctx: &mut TxContext
    ) {
        let (balance0, balance1, ticks) = rebuild_positions_after_clean_liquidities(_vault, _ctx);
        let shape = SHAPE_EQUAL_SIZE;
        if (_vault.state_counter >= _vault.max_counter_times) {
            shape = _vault.state;
            // reset state counter
            _vault.state_counter = 0;
        };
        let liquidities = positions_liquidity_size_balance(_vault, &ticks, shape);
        rebalance_internal(
            _vault,
            _bfc_balance,
            _supply,
            balance0,
            balance1,
            liquidities
        );
    }
}

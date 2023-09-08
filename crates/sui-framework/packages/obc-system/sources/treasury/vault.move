module obc_system::vault {
    use std::vector;

    use sui::balance::{Self, Balance};
    use sui::coin::{Self, Coin};
    use sui::obc::OBC;
    use sui::object;
    use sui::object::{ID, UID};
    use sui::tx_context::TxContext;

    use obc_system::clmm_math;
    use obc_system::event;
    use obc_system::i32::{Self, I32};
    use obc_system::math_u128;
    use obc_system::math_u64;
    use obc_system::option_u64;
    use obc_system::position::{Self, PositionManager};
    use obc_system::tick::{Self, TickManager};
    use obc_system::tick_math;

    friend obc_system::treasury;

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

    struct Vault<phantom StableCoinType> has key, store {
        id: UID,

        position_number: u32,
        /// 0 -- init, equal, 1 -- down, 2 -- up
        state: u8,
        state_counter: u32,

        coin_a: Balance<StableCoinType>,
        coin_b: Balance<OBC>,

        /// The tick spacing
        tick_spacing: u32,

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

    // === Create vault ====
    public(friend) fun create_vault<StableCoinType>(
        _index: u64,
        _tick_spacing: u32,
        _position_number: u32,
        _initialize_price: u128,
        _base_point: u64,
        _ts: u64,
        _ctx: &mut TxContext,
    ): Vault<StableCoinType> {
        let current_tick_index = tick_math::get_tick_at_sqrt_price(_initialize_price);
        let valid_index = tick_math::get_next_valid_tick_index(current_tick_index, _tick_spacing);
        let uid = object::new(_ctx);
        let pid = object::uid_to_inner(&uid);
        Vault {
            id: uid,
            position_number: _position_number,
            state: 0,
            state_counter: 0,
            coin_a: balance::zero<StableCoinType>(),
            coin_b: balance::zero<OBC>(),
            tick_spacing: _tick_spacing,
            liquidity: 0,
            current_sqrt_price: tick_math::get_sqrt_price_at_tick(valid_index),
            current_tick_index: valid_index,
            tick_manager: tick::create_tick_manager(_tick_spacing, _ts, _ctx),
            position_manager: position::create_position_manager(pid, _tick_spacing, _ctx),
            is_pause: false,
            index: _index,
            base_point: _base_point,
        }
    }

    /// open `position_number` positions
    public(friend) fun init_positions<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _spacing_times: u32,
        _ctx: &mut TxContext
    ) {
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
    }

    fun open_position<StableCoinType>(
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

    /// Calculate the position's amount_a/amount_b
    /// Params
    ///     - `vault` The clmm vault object.
    ///     - `position_id` The object id of position's NFT.
    /// Returns
    ///     - `amount_a` The amount of `StableCoinType`
    ///     - `amount_b` The amount of `OBC`
    public fun get_position_amounts<StableCoinType>(
        _vault: &Vault<StableCoinType>,
        _index: u64
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
            false
        )
    }

    /// Flash loan resource for add_liquidity
    struct AddLiquidityReceipt<phantom StableCoinType> {
        vault_id: ID,
        amount_a: u64,
        amount_b: u64
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

    public(friend) fun remove_liquidity<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _index: u64,
        _delta_liquidity: u128
    ): (Balance<StableCoinType>, Balance<OBC>) {
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
        tick::increase_liquidity(
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
        _balance_b: Balance<OBC>,
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

    fun default_calculated_swap_result(): CalculatedSwapResult {
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
                current_sqrt_price: current_sqrt_price,
                target_sqrt_price: target_sqrt_price,
                current_liquidity: liquidity,
                amount_in: amount_in,
                amount_out: amount_out,
                remainer_amount: remainer_amount,
                current_tick_index: tick_index,
            });
            if (target_sqrt_price == next_sqrt_price) {
                current_sqrt_price = next_sqrt_price;
                liquidity = tick::cross_by_swap(&_vault.tick_manager, tick_index, _a2b, liquidity);
                tick_index = tick::tick_index(next_tick);
                swap_result.after_sqrt_price = current_sqrt_price;
            };
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

    public fun swap<StableCoinType>(
        _vault: &mut Vault<StableCoinType>,
        _coin_a: Coin<StableCoinType>,
        _coin_b: Coin<OBC>,
        _a2b: bool,
        _by_amount_in: bool,
        _amount: u64,
        _amount_limit: u64,
        _sqrt_price_limit: u128,
        _ctx: &mut TxContext
    ): (Balance<StableCoinType>, Balance<OBC>) {
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
            pay_coin_b = balance::zero<OBC>();
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
        balance_b: Balance<OBC>,
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
    ): (Balance<StableCoinType>, Balance<OBC>, FlashSwapReceipt<StableCoinType>)
    {
        let min_price = tick_math::min_sqrt_price();
        let max_price = tick_math::max_sqrt_price();
        if (_a2b) {
            assert!(
                _sqrt_price_limit >= _vault.current_sqrt_price && _sqrt_price_limit >= min_price,
                ERR_SQRT_PRICE_LIMIT_INVALID
            );
        } else {
            assert!(
                _sqrt_price_limit <= _vault.current_sqrt_price && _sqrt_price_limit <= max_price,
                ERR_SQRT_PRICE_LIMIT_INVALID
            );
        };
        let before_sqrt_price = _vault.current_sqrt_price;
        let swap_res = swap_in_vault(_vault, _a2b, _by_amount_in, _sqrt_price_limit, _amount);
        let balance_a_ret;
        let balance_b_ret;
        if (_a2b) {
            balance_b_ret = balance::split<OBC>(&mut _vault.coin_b, swap_res.amount_out);
            balance_a_ret = balance::zero<StableCoinType>();
        } else {
            balance_a_ret = balance::split<StableCoinType>(&mut _vault.coin_a, swap_res.amount_out);
            balance_b_ret = balance::zero<OBC>();
        };
        event::swap(
            vault_id(_vault),
            _a2b,
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
            pay_amount: swap_res.amount_out + swap_res.amount_in
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
        while (remaining_amount > 0) {
            if (current_sqrt_price != _sqrt_price_limit) {
                assert!(option_u64::is_none(&next_score), ERR_TICK_INDEX_OPTION_IS_NONE);
                let (tick, tick_score) = tick::borrow_tick_for_swap(
                    &_vault.tick_manager,
                    option_u64::borrow(&next_score),
                    _a2b
                );
                next_score = tick_score;
                let tick_index = tick::tick_index(tick);
                let tick_sqrt_price = if (_a2b) {
                    math_u128::max(_sqrt_price_limit, tick::sqrt_price(tick))
                } else {
                    math_u128::min(_sqrt_price_limit, tick::sqrt_price(tick))
                };
                let (amount_in, amount_out, next_sqrt_price) = clmm_math::compute_swap_step(
                    _vault.current_sqrt_price,
                    tick_sqrt_price,
                    _vault.liquidity,
                    remaining_amount,
                    _a2b,
                    _by_amount_in
                );
                if (amount_in != 0 || amount_out != 0) {
                    if (_by_amount_in) {
                        remaining_amount = check_remainer_amount_sub(remaining_amount, amount_in);
                    } else {
                        remaining_amount = check_remainer_amount_sub(remaining_amount, amount_out);
                    };
                    update_swap_result(&mut swap_result, amount_in, amount_out);
                } else {
                    if (next_sqrt_price == tick::sqrt_price(tick)) {
                        _vault.current_sqrt_price = tick_sqrt_price;
                        let next_tick = if (_a2b) {
                            i32::sub(tick_index, i32::from_u32(1))
                        } else {
                            tick_index
                        };
                        _vault.current_tick_index = next_tick;
                        _vault.liquidity = tick::cross_by_swap(
                            &mut _vault.tick_manager,
                            _vault.current_tick_index,
                            _a2b,
                            _vault.liquidity
                        );
                    } else {
                        if (_vault.current_sqrt_price != tick::sqrt_price(tick)) {
                            _vault.current_sqrt_price = next_sqrt_price;
                            _vault.current_tick_index = tick_math::get_tick_at_sqrt_price(next_sqrt_price);
                        }
                    };
                };
            }
        };
        swap_result
    }

    /// Read Functions
    /// vault info
    public fun vault_id<StableCoinType>(_vault: &Vault<StableCoinType>): ID {
        object::id(_vault)
    }

    public fun vault_current_sqrt_price<StableCoinType>(_vault: &Vault<StableCoinType>): u128 {
        _vault.current_sqrt_price
    }

    public fun balances<StableCoinType>(_vault: &Vault<StableCoinType>): (u64, u64) {
        (
            balance::value<StableCoinType>(&_vault.coin_a),
            balance::value<OBC>(&_vault.coin_b)
        )
    }
}

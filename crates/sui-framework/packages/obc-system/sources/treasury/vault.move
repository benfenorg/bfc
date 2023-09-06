module obc_system::vault {
    use std::type_name::TypeName;
    use std::vector;

    use sui::balance::{Self, Balance};
    use sui::coin::{Self, Coin};
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

    // === Structs ===

    struct VaultSimpleInfo has store, copy, drop {
        vault_id: ID,
        vault_key: ID,
        coin_type_a: TypeName,
        coin_type_b: TypeName,
        tick_spacing: u32,
    }

    struct Vault<phantom CoinTypeA, phantom CoinTypeB> has key, store {
        id: UID,

        position_number: u32,
        state: u8, // 0 -- init, equal, 1 -- down, 2 -- up
        state_counter: u32,

        coin_a: Balance<CoinTypeA>,
        coin_b: Balance<CoinTypeB>,

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
    }

    // === Create vault ====
    public(friend) fun create_vault<CoinTypeA, CoinTypeB>(
        _index: u64,
        _tick_spacing: u32,
        _position_number: u32,
        _initialize_price: u128,
        _ts: u64,
        _ctx: &mut TxContext,
    ): Vault<CoinTypeA, CoinTypeB> {
        let current_tick_index = tick_math::get_tick_at_sqrt_price(_initialize_price);
        let uid = object::new(_ctx);
        let pid = object::uid_to_inner(&uid);
        Vault {
            id: uid,
            position_number: _position_number,
            state: 0,
            state_counter: 0,
            coin_a: balance::zero<CoinTypeA>(),
            coin_b: balance::zero<CoinTypeB>(),
            tick_spacing: _tick_spacing,
            liquidity: 0,
            current_sqrt_price: _initialize_price,
            current_tick_index,
            tick_manager: tick::create_tick_manager(_tick_spacing, _ts, _ctx),
            position_manager: position::create_position_manager(pid, _tick_spacing, _ctx),
            is_pause: false,
            index: _index,
        }
    }

    public(friend) fun create_vault_simple_info(
        _vault_id: ID,
        _vault_key: ID,
        _coin_type_a: TypeName,
        _coin_type_b: TypeName,
        _tick_spacing: u32
    ): VaultSimpleInfo {
        VaultSimpleInfo {
            vault_id: _vault_id,
            vault_key: _vault_key,
            coin_type_a: _coin_type_a,
            coin_type_b: _coin_type_b,
            tick_spacing: _tick_spacing
        }
    }

    // === Public Functions ===
    public fun open_position<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _tick_lower: u32,
        _tick_upper: u32,
        _ctx: &mut TxContext
    ) {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let tick_lower = i32::from_u32(_tick_lower);
        let tick_upper = i32::from_u32(_tick_upper);
        let position_id = position::open_position<CoinTypeA, CoinTypeB>(
            &mut _vault.position_manager,
            _vault.index,
            tick_lower,
            tick_upper,
            _ctx,
        );
        event::open_position(
            vault_id(_vault),
            position_id,
            tick_lower,
            tick_upper,
        )
    }

    public fun close_position<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _tick_lower: u32,
        _tick_upper: u32
    )
    {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let position_id = position::close_position(
            &mut _vault.position_manager,
            i32::from_u32(_tick_lower),
            i32::from_u32(_tick_upper)
        );
        event::close_position(vault_id(_vault), position_id)
    }

    /// Calculate the position's amount_a/amount_b
    /// Params
    ///     - `vault` The clmm vault object.
    ///     - `position_id` The object id of position's NFT.
    /// Returns
    ///     - `amount_a` The amount of `CoinTypeA`
    ///     - `amount_b` The amount of `CoinTypeB`
    public fun get_position_amounts<CoinTypeA, CoinTypeB>(
        _vault: &Vault<CoinTypeA, CoinTypeB>,
        _tick_lower: u32,
        _tick_upper: u32
    ): (u64, u64) {
        let tick_lower = i32::from_u32(_tick_lower);
        let tick_upper = i32::from_u32(_tick_upper);
        let position = position::borrow_position(
            &_vault.position_manager,
            tick_lower,
            tick_upper
        );
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
    struct AddLiquidityReceipt<phantom CoinTypeA, phantom CoinTypeB> {
        vault_id: ID,
        amount_a: u64,
        amount_b: u64
    }

    fun add_liquidity_internal<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _tick_lower: u32,
        _tick_upper: u32,
        _use_amount: bool,
        _liquidity_delta: u128,
        _amount: u64,
        _fix_amount_a: bool
    ): AddLiquidityReceipt<CoinTypeA, CoinTypeB> {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let tick_lower = i32::from_u32(_tick_lower);
        let tick_upper = i32::from_u32(_tick_upper);
        let expect_vault_id = vault_id(_vault);
        let mut_position = position::borrow_mut_position(
            &mut _vault.position_manager,
            tick_lower,
            tick_upper,
        );
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
            position::get_position_id(mut_position),
            tick_lower,
            tick_upper,
            liquidity_delta,
            _vault.liquidity,
            amount_a,
            amount_b
        );
        AddLiquidityReceipt<CoinTypeA, CoinTypeB> {
            vault_id: _vault_id,
            amount_a,
            amount_b,
        }
    }

    public fun add_liquidity<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _tick_lower: u32,
        _tick_upper: u32,
        _delta_liquidity: u128
    ): AddLiquidityReceipt<CoinTypeA, CoinTypeB> {
        assert!(_delta_liquidity > 0, ERR_LIQUIDITY_DELTA_IS_ZERO);
        add_liquidity_internal(
            _vault,
            _tick_lower,
            _tick_upper,
            false,
            _delta_liquidity,
            0u64,
            false
        )
    }

    public fun remove_liquidity<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _tick_lower: u32,
        _tick_upper: u32,
        _delta_liquidity: u128
    ): (Balance<CoinTypeA>, Balance<CoinTypeB>) {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let tick_lower = i32::from_u32(_tick_lower);
        let tick_upper = i32::from_u32(_tick_upper);
        let expect_vault_id = vault_id(_vault);
        let mut_position = position::borrow_mut_position(
            &mut _vault.position_manager,
            tick_lower,
            tick_upper,
        );
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
            position::get_position_id(mut_position),
            tick_lower,
            tick_upper,
            _delta_liquidity,
            _vault.liquidity,
            amount_a,
            amount_b
        );
        (balance_a, balance_b)
    }

    public fun add_liquidity_fix_coin<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _tick_lower: u32,
        _tick_upper: u32,
        _amount: u64,
        _fix_amount_a: bool
    ): AddLiquidityReceipt<CoinTypeA, CoinTypeB> {
        assert!(_amount > 0, ERR_AMOUNT_IS_ZERO);
        add_liquidity_internal(
            _vault,
            _tick_lower,
            _tick_upper,
            true,
            0u128,
            _amount,
            _fix_amount_a
        )
    }

    public fun repay_add_liquidity<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _balance_a: Balance<CoinTypeA>,
        _balance_b: Balance<CoinTypeB>,
        _receipt: AddLiquidityReceipt<CoinTypeA, CoinTypeB>
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
    public fun calculate_swap_result<CoinTypeA, CoinTypeB>(
        _vault: &Vault<CoinTypeA, CoinTypeB>,
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
    struct FlashSwapReceipt<phantom CoinTypeA, phantom CoinTypeB> {
        vault_id: ID,
        a2b: bool,
        pay_amount: u64,
    }

    public fun swap<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _coin_a: Coin<CoinTypeA>,
        _coin_b: Coin<CoinTypeB>,
        _a2b: bool,
        _by_amount_in: bool,
        _amount: u64,
        _amount_limit: u64,
        _sqrt_price_limit: u128,
        _ctx: &mut TxContext
    ): (Balance<CoinTypeA>, Balance<CoinTypeB>) {
        assert!(!_vault.is_pause, ERR_POOL_IS_PAUSE);
        let (
            receive_a,
            receive_b,
            flash_receipt
        ) = flash_swap_internal<CoinTypeA, CoinTypeB>(
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
            pay_coin_b = balance::zero<CoinTypeB>();
        } else {
            pay_coin_a = balance::zero<CoinTypeA>();
            pay_coin_b = coin::into_balance(coin::split(&mut _coin_b, pay_amount, _ctx));
        };

        coin::join(&mut _coin_a, coin::from_balance(receive_a, _ctx));
        coin::join(&mut _coin_b, coin::from_balance(receive_b, _ctx));

        repay_flash_swap<CoinTypeA, CoinTypeB>(
            _vault,
            pay_coin_a,
            pay_coin_b,
            flash_receipt
        );

        (coin::into_balance(_coin_a), coin::into_balance(_coin_b))
    }

    fun repay_flash_swap<CoinTypeA, CoinTypeB>(
        vault: &mut Vault<CoinTypeA, CoinTypeB>,
        balance_a: Balance<CoinTypeA>,
        balance_b: Balance<CoinTypeB>,
        receipt: FlashSwapReceipt<CoinTypeA, CoinTypeB>
    ) {
        let FlashSwapReceipt<CoinTypeA, CoinTypeB> {
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

    fun flash_swap_internal<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
        _a2b: bool,
        _by_amount_in: bool,
        _amount: u64,
        _sqrt_price_limit: u128
    ): (Balance<CoinTypeA>, Balance<CoinTypeB>, FlashSwapReceipt<CoinTypeA, CoinTypeB>)
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
            balance_b_ret = balance::split<CoinTypeB>(&mut _vault.coin_b, swap_res.amount_out);
            balance_a_ret = balance::zero<CoinTypeA>();
        } else {
            balance_a_ret = balance::split<CoinTypeA>(&mut _vault.coin_a, swap_res.amount_out);
            balance_b_ret = balance::zero<CoinTypeB>();
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
        (balance_a_ret, balance_b_ret, FlashSwapReceipt<CoinTypeA, CoinTypeB> {
            vault_id: vault_id(_vault),
            a2b: _a2b,
            pay_amount: swap_res.amount_out + swap_res.amount_in
        })
    }

    fun swap_in_vault<CoinTypeA, CoinTypeB>(
        _vault: &mut Vault<CoinTypeA, CoinTypeB>,
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
    public fun vault_id<CoinTypeA, CoinTypeB>(_vault: &Vault<CoinTypeA, CoinTypeB>): ID {
        object::id(_vault)
    }

    public fun vault_current_sqrt_price<CoinTypeA, CoinTypeB>(_vault: &Vault<CoinTypeA, CoinTypeB>): u128 {
        _vault.current_sqrt_price
    }

    public fun balances<CoinTypeA, CoinTypeB>(_vault: &Vault<CoinTypeA, CoinTypeB>): (u64, u64) {
        (
            balance::value<CoinTypeA>(&_vault.coin_a),
            balance::value<CoinTypeB>(&_vault.coin_b)
        )
    }
}

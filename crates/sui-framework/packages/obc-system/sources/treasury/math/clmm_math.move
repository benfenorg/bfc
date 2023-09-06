module obc_system::clmm_math {
    use obc_system::full_math_u128;
    use obc_system::i32::{Self, I32};
    use obc_system::math_u128;
    use obc_system::math_u256;
    use obc_system::tick_math;

    #[test_only]
    use std::debug;

    const EMATH_U256_CHECKED_SHLW_FAILED: u64 = 1000;
    const ETICK_MATH_EXCEED_MAX_SQRT_PRICE: u64 = 1001;
    const ETICK_MATH_LESS_MIN_SQRT_PRICE: u64 = 1002;
    const ETICK_EXCEED_MAXIMUM: u64 = 1003;
    const ETICK_LESS_MINIMUM: u64 = 1004;

    /// `liquidity = ( sqrt_price_upper * sqrt_price_lower * delta_a ) / delta_sqrt_price`
    public fun get_liquidity_from_a(
        sqrt_price_0: u128,
        sqrt_price_1: u128,
        amount_a: u64,
        round_up: bool
    ): u128
    {
        let sqrt_price_diff = if (sqrt_price_0 > sqrt_price_1) {
            sqrt_price_0 - sqrt_price_1
        } else {
            sqrt_price_1 - sqrt_price_0
        };
        let v1 = math_u256::shrw(full_math_u128::full_mul(sqrt_price_0, sqrt_price_1));
        let v2 = (amount_a as u256) * (sqrt_price_diff as u256);
        (math_u256::div_round(v1, v2, round_up) as u128)
    }

    /// `liquidity = delta_b / delta_sqrt_price`
    public fun get_liquidity_from_b(
        sqrt_price_0: u128,
        sqrt_price_1: u128,
        amount_b: u64,
        round_up: bool,
    ): u128
    {
        let sqrt_price_diff = if (sqrt_price_0 > sqrt_price_1) {
            sqrt_price_0 - sqrt_price_1
        } else {
            sqrt_price_1 - sqrt_price_0
        };
        (math_u256::div_round(
            math_u256::shlw((amount_b as u256)),
            math_u256::shlw((sqrt_price_diff as u256)),
            round_up
        ) as u128)
    }

    /// Gets the amount_a delta between two prices, for given amount of liquidity
    /// # Formula
    /// `delta_a = (liquidity * delta_sqrt_price) / (sqrt_price_upper * sqrt_price_lower)`
    /// # Params
    /// * `sqrt_price_0` - A sqrt price
    /// * `sqrt_price_1` - Another sqrt price
    /// * `liquidity` - The amount of usable liquidity
    /// * `round_up`- Whether to round the amount up or down
    public fun get_delta_a(
        sqrt_price_0: u128,
        sqrt_price_1: u128,
        liquidity: u128,
        round_up: bool,
    ): u64
    {
        let sqrt_price_diff = if (sqrt_price_0 > sqrt_price_1) {
            sqrt_price_0 - sqrt_price_1
        } else {
            sqrt_price_1 - sqrt_price_0
        };
        if (sqrt_price_diff == 0 || liquidity == 0) {
            return 0
        };
        let (numberator, is_ok) = math_u256::checked_shlw(full_math_u128::full_mul(liquidity, sqrt_price_diff));
        assert!(!is_ok, EMATH_U256_CHECKED_SHLW_FAILED);
        let denomminator = full_math_u128::full_mul(sqrt_price_0, sqrt_price_1);
        (math_u256::div_round(numberator, denomminator, round_up) as u64)
    }

    /// Gets the amount_b delta between two prices, for given amount of liquidity
    /// # Formula
    /// * `delta_b = delta_sqrt_price * liquidity`
    /// # Params
    /// * `sqrt_price_0` - A sqrt price
    /// * `sqrt_price_1` - Another sqrt price
    /// * `liquidity` - The amount of usable liquidity
    /// * `round_up`- Whether to round the amount up or down
    public fun get_delta_b(
        sqrt_price_0: u128,
        sqrt_price_1: u128,
        liquidity: u128,
        round_up: bool,
    ): u64
    {
        let sqrt_price_diff = if (sqrt_price_0 > sqrt_price_1) {
            sqrt_price_0 - sqrt_price_1
        } else {
            sqrt_price_1 - sqrt_price_0
        };
        if (sqrt_price_diff == 0 || liquidity == 0) {
            return 0
        };
        let product = full_math_u128::full_mul(liquidity, sqrt_price_diff);
        if (round_up) {
            if (product & (math_u256::shlw(1) - 1) > 0) {
                return (math_u256::shrw(product) + 1 as u64)
            };
        };
        (math_u256::shrw(product) as u64)
    }

    /// Gets the next sqrt price from given a delta of token_a
    /// # Formula
    /// `sqrt_price_new = (sqrt_price * liquidity) / (liquidity +- amount * sqrt_price)`
    /// # Arguments
    /// * `sqrt_price` - The starting price `sqrt(P)`
    /// * `liquidity` - The amount of usable liquidity L
    /// * `amount` - Delta of token a
    /// * `add` - Whether to add or remove the amount of token_a
    public fun get_next_sqrt_price_a_up(
        sqrt_price: u128,
        liquidity: u128,
        amount: u64,
        by_amount_input: bool,
    ): u128
    {
        if (amount == 0) {
            return sqrt_price
        };
        let (numberator, is_ok) = math_u256::checked_shlw(full_math_u128::full_mul(sqrt_price, liquidity));
        assert!(!is_ok, EMATH_U256_CHECKED_SHLW_FAILED);
        let liquidity_shl_64 = math_u256::shlw((liquidity as u256));
        let product = full_math_u128::full_mul(sqrt_price, (amount as u128));
        let quotient = if (by_amount_input) {
            math_u256::div_round(numberator, product + liquidity_shl_64, true)
        } else {
            math_u256::div_round(numberator, liquidity_shl_64 - product, true)
        };
        assert!((quotient as u128) <= tick_math::max_sqrt_price(), ETICK_MATH_EXCEED_MAX_SQRT_PRICE);
        assert!((quotient as u128) >= tick_math::min_sqrt_price(), ETICK_MATH_LESS_MIN_SQRT_PRICE);
        (quotient as u128)
    }

    /// Gets the next sqrt price given a delta of token_b
    /// # Formula
    /// * `new_sqrt_price = sqrt_price + (delta_b / liquidity)`
    /// # Arguments
    /// * `sqrt_price` - The starting price `sqrt(P)`, i.e., before accounting for the token_1 delta
    /// * `liquidity` - The amount of usable liquidity L
    /// * `amount` - Delta of token 1 (dy) to add or remove from virtual reserves
    /// * `add` - Whether to add or remove the amount of token_1
    public fun get_next_sqrt_price_b_down(
        sqrt_price: u128,
        liquidity: u128,
        amount: u64,
        by_amount_input: bool,
    ): u128
    {
        let delta_sqrt_price = math_u128::checked_div_round((amount as u128) << 64, liquidity, by_amount_input);
        let new_sqrt_price = if (by_amount_input) {
            sqrt_price + delta_sqrt_price
        } else {
            sqrt_price - delta_sqrt_price
        };
        assert!(new_sqrt_price <= tick_math::max_sqrt_price(), ETICK_MATH_EXCEED_MAX_SQRT_PRICE);
        assert!(new_sqrt_price >= tick_math::min_sqrt_price(), ETICK_MATH_LESS_MIN_SQRT_PRICE);
        new_sqrt_price
    }

    public fun get_next_sqrt_price_from_input(
        sqrt_price: u128,
        liquidity: u128,
        amount: u64,
        a2b: bool,
    ): u128
    {
        if (a2b) {
            get_next_sqrt_price_a_up(sqrt_price, liquidity, amount, true)
        } else {
            get_next_sqrt_price_b_down(sqrt_price, liquidity, amount, true)
        }
    }

    public fun get_next_sqrt_price_from_output(
        sqrt_price: u128,
        liquidity: u128,
        amount: u64,
        a2b: bool,
    ): u128
    {
        if (a2b) {
            get_next_sqrt_price_b_down(sqrt_price, liquidity, amount, false)
        } else {
            get_next_sqrt_price_a_up(sqrt_price, liquidity, amount, false)
        }
    }

    public fun get_delta_up_from_input(
        current_sqrt_price: u128,
        target_sqrt_price: u128,
        liquidity: u128,
        a2b: bool,
    ): u256
    {
        let sqrt_price_diff = if (current_sqrt_price > target_sqrt_price) {
            current_sqrt_price - target_sqrt_price
        } else {
            target_sqrt_price - current_sqrt_price
        };
        if (sqrt_price_diff == 0 || liquidity == 0) {
            0
        } else {
            if (a2b) {
                let (numberator, is_ok) = math_u256::checked_shlw(full_math_u128::full_mul(sqrt_price_diff, liquidity));
                assert!(!is_ok, EMATH_U256_CHECKED_SHLW_FAILED);
                let denomminator = full_math_u128::full_mul(current_sqrt_price, target_sqrt_price);
                math_u256::div_round(numberator, denomminator, true)
            } else {
                let product = full_math_u128::full_mul(liquidity, sqrt_price_diff);
                if (product & (math_u256::shlw(1) - 1) > 0) {
                    math_u256::shrw(product) + 1
                } else {
                    math_u256::shrw(product)
                }
            }
        }
    }

    public fun get_delta_down_from_output(
        current_sqrt_price: u128,
        target_sqrt_price: u128,
        liquidity: u128,
        a2b: bool,
    ): u256
    {
        let sqrt_price_diff = if (current_sqrt_price > target_sqrt_price) {
            current_sqrt_price - target_sqrt_price
        } else {
            target_sqrt_price - current_sqrt_price
        };
        if (sqrt_price_diff == 0 || liquidity == 0) {
            0
        } else {
            if (a2b) {
                math_u256::shrw(full_math_u128::full_mul(liquidity, sqrt_price_diff))
            } else {
                let (numberator, is_ok) = math_u256::checked_shlw(full_math_u128::full_mul(liquidity, sqrt_price_diff));
                assert!(!is_ok, EMATH_U256_CHECKED_SHLW_FAILED);
                let denomminator = full_math_u128::full_mul(current_sqrt_price, target_sqrt_price);
                math_u256::div_round(numberator, denomminator, false)
            }
        }
    }

    public fun compute_swap_step(
        current_sqrt_price: u128,
        target_sqrt_price: u128,
        liquidity: u128,
        amount: u64,
        a2b: bool,
        by_amount_input: bool,
    ): (u64, u64, u128) {
        if (liquidity == 0) {
            (0, 0, target_sqrt_price)
        } else {
            let next_sqrt_price;
            let amount_in: u64;
            let amount_out: u64;
            if (by_amount_input) {
                let max_amount_in =
                    get_delta_up_from_input(current_sqrt_price, target_sqrt_price, liquidity, a2b);
                if (max_amount_in > (amount as u256)) {
                    amount_in = amount;
                    next_sqrt_price = get_next_sqrt_price_from_input(
                        current_sqrt_price,
                        liquidity,
                        amount,
                        a2b,
                    );
                } else {
                    amount_in = (max_amount_in as u64);
                    next_sqrt_price = target_sqrt_price;
                };
                amount_out =
                    (get_delta_down_from_output(current_sqrt_price, next_sqrt_price, liquidity, a2b) as u64);
            } else {
                let max_amount_out = get_delta_down_from_output(
                    current_sqrt_price,
                    target_sqrt_price,
                    liquidity,
                    a2b,
                );
                if (max_amount_out > (amount as u256)) {
                    amount_out = amount;
                    next_sqrt_price =
                        get_next_sqrt_price_from_output(current_sqrt_price, liquidity, amount, a2b);
                } else {
                    amount_out = (max_amount_out as u64);
                    next_sqrt_price = target_sqrt_price;
                };
                amount_in =
                    (get_delta_up_from_input(current_sqrt_price, next_sqrt_price, liquidity, a2b) as u64);
            };
            (amount_in, amount_out, next_sqrt_price)
        }
    }

    public fun get_liquidity_by_amount(
        _tick_lower_index: I32,
        _tick_upper_index: I32,
        _current_tick_index: I32,
        _current_sqrt_price: u128,
        _amount: u64,
        _fix_amount_a: bool
    ): (u128, u64, u64)
    {
        let tick_lower_price = tick_math::get_sqrt_price_at_tick(_tick_lower_index);
        let tick_upper_price = tick_math::get_sqrt_price_at_tick(_tick_upper_index);
        let liquidity: u128;
        let amount_a: u64 = 0;
        let amount_b: u64 = 0;

        if (_fix_amount_a) {
            amount_a = _amount;
            if (i32::lt(_current_tick_index, _tick_lower_index)) {
                liquidity = get_liquidity_from_a(tick_lower_price, tick_upper_price, amount_a, false);
            } else {
                assert!(i32::lt(_current_tick_index, _tick_upper_index), ETICK_EXCEED_MAXIMUM);
                liquidity = get_liquidity_from_a(_current_sqrt_price, tick_upper_price, _amount, false);
                amount_b = get_delta_b(_current_sqrt_price, tick_lower_price, liquidity, true);
            };
        } else {
            amount_b = _amount;
            if (i32::gte(_tick_upper_index, _tick_lower_index)) {
                liquidity = get_liquidity_from_b(tick_lower_price, tick_upper_price, _amount, false)
            } else {
                assert!(i32::gte(_current_tick_index, _tick_lower_index), ETICK_LESS_MINIMUM);
                liquidity = get_liquidity_from_b(tick_lower_price, _current_sqrt_price, _amount, false);
                amount_a = get_delta_a(_current_sqrt_price, tick_upper_price, liquidity, true)
            };
        };
        (liquidity, amount_a, amount_b)
    }

    public fun get_amount_by_liquidity(
        _tick_lower_index: I32,
        _tick_upper_index: I32,
        _current_tick_index: I32,
        _current_sqrt_price: u128,
        _liquidity_delta: u128,
        _round_up: bool
    ): (u64, u64)
    {
        if (_liquidity_delta == 0) {
            return (0, 0)
        };
        let tick_lower_price = tick_math::get_sqrt_price_at_tick(_tick_lower_index);
        let tick_upper_price = tick_math::get_sqrt_price_at_tick(_tick_upper_index);
        let amount_a: u64;
        let amount_b: u64;
        if (i32::lt(_tick_lower_index, _current_tick_index)) {
            amount_a = get_delta_a(tick_lower_price, tick_upper_price, _liquidity_delta, _round_up);
            amount_b = 0;
        } else {
            if (i32::lt(_tick_upper_index, _current_tick_index)) {
                amount_a = get_delta_a(_current_sqrt_price, tick_upper_price, _liquidity_delta, _round_up);
                amount_b = get_delta_b(tick_lower_price, _current_sqrt_price, _liquidity_delta, _round_up);
            } else {
                amount_a = 0;
                amount_b = get_delta_b(tick_lower_price, tick_upper_price, _liquidity_delta, _round_up);
            }
        };
        (amount_a, amount_b)
    }

    #[test]
    public fun test_get_delta_up_from_input() {
        let current_sqrt_price: u128 = 4116333652252349704;
        let target_sqrt_price: u128 = 79084200890414257525634219231;
        let liquidity: u128 = 25403716994;
        let a2b = false;

        let max_amount_in =
            get_delta_up_from_input(current_sqrt_price, target_sqrt_price, liquidity, a2b);
        debug::print(&max_amount_in);
    }
}

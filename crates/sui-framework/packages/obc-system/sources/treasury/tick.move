module obc_system::tick {
    use std::option;
    use std::option::Option;

    use sui::tx_context::TxContext;

    use obc_system::i128::{Self, I128};
    use obc_system::i32::{Self, I32};
    use obc_system::math_u128;
    use obc_system::option_u64::OptionU64;
    use obc_system::skip_list::{Self, SkipList};
    use obc_system::tick_math;

    friend obc_system::treasury;
    friend obc_system::vault;

    const ERR_TICK_EXCEED_TWICE_MAXIMUM: u64 = 400;
    const ERR_TICK_EXCEED_U128_MAXIMUM: u64 = 401;
    const ERR_TICK_LIQUIDITY_INSUFFICIENT: u64 = 402;
    const ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY: u64 = 403;

    struct TickManager has store {
        tick_spacing: u32,
        ticks: SkipList<Tick>
    }

    struct Tick has store, copy, drop {
        index: I32,
        sqrt_price: u128,
        liquidity_net: I128,
        liquidity_gross: u128
    }

    public(friend) fun create_tick_manager(
        _tick_spacing: u32,
        _ts: u64,
        _ctx: &mut TxContext,
    ): TickManager {
        TickManager {
            tick_spacing: _tick_spacing,
            ticks: skip_list::new(16, 2, _ts, _ctx),
        }
    }

    /// tick info
    public fun sqrt_price(_tick: &Tick): u128 {
        _tick.sqrt_price
    }

    public fun liquidity_net(_tick: &Tick): I128 {
        _tick.liquidity_net
    }

    public fun tick_index(_tick: &Tick): I32 {
        _tick.index
    }

    public fun tick_spacing(_tick_manager: &TickManager): u32 {
        _tick_manager.tick_spacing
    }

    public fun fetch_ticks(_tick_manager: &TickManager) {
        abort 0
    }

    /// private fun
    fun default(_tick_index: I32): Tick {
        let sqrt_price = tick_math::get_sqrt_price_at_tick(_tick_index);
        Tick {
            sqrt_price,
            index: _tick_index,
            liquidity_net: i128::from(sqrt_price),
            liquidity_gross: 0
        }
    }

    fun tick_score(_tick_index: I32): u64 {
        let score = i32::as_u32(i32::add(_tick_index, tick_math::max_tick()));
        let bound = i32::as_u32(tick_math::max_tick()) * 2;
        assert!(score <= bound, ERR_TICK_EXCEED_TWICE_MAXIMUM);
        (score as u64)
    }

    fun update_by_liquidity(
        _tick: &mut Tick,
        _current_tick_index: I32,
        _liquidity_delta: u128,
        _is_add_liquidity: bool,
        _is_cross_net: bool
    )
    {
        if (_is_add_liquidity == true) {
            assert!(math_u128::add_check(_tick.liquidity_gross, _liquidity_delta), ERR_TICK_EXCEED_U128_MAXIMUM);
            _tick.liquidity_gross = _tick.liquidity_gross + _liquidity_delta;
        } else {
            assert!(_tick.liquidity_gross >= _liquidity_delta, ERR_TICK_LIQUIDITY_INSUFFICIENT);
            _tick.liquidity_gross = _tick.liquidity_gross - _liquidity_delta;
        };
        let is_overflowing: bool;
        let liquidity_net: I128;
        if (_is_add_liquidity) {
            if (_is_cross_net) {
                (liquidity_net, is_overflowing) = i128::overflowing_sub(
                    _tick.liquidity_net,
                    i128::from(_liquidity_delta)
                );
            } else {
                (liquidity_net, is_overflowing) = i128::overflowing_add(
                    _tick.liquidity_net,
                    i128::from(_liquidity_delta)
                );
            };
        } else {
            if (_is_cross_net) {
                (liquidity_net, is_overflowing) = i128::overflowing_add(
                    _tick.liquidity_net,
                    i128::from(_liquidity_delta)
                );
            } else {
                (liquidity_net, is_overflowing) = i128::overflowing_sub(
                    _tick.liquidity_net,
                    i128::from(_liquidity_delta)
                );
            };
        };
        assert!(!is_overflowing, ERR_TICK_LIQUIDITY_INSUFFICIENT);
        _tick.liquidity_net = liquidity_net;
    }

    /// add/remove liquidity
    public(friend) fun increase_liquidity(
        _tick_manager: &mut TickManager,
        _current_tick_index: I32,
        _tick_lower_index: I32,
        _tick_upper_index: I32,
        _liquidity_delta: u128
    )
    {
        if (_liquidity_delta == 0) {
            return
        };
        let tick_lower_score = tick_score(_tick_lower_index);
        let tick_upper_score = tick_score(_tick_upper_index);

        if (!skip_list::contains(&_tick_manager.ticks, tick_lower_score)) {
            skip_list::insert(&mut _tick_manager.ticks, tick_lower_score, default(_tick_lower_index));
        };
        if (!skip_list::contains(&_tick_manager.ticks, tick_upper_score)) {
            skip_list::insert(&mut _tick_manager.ticks, tick_upper_score, default(_tick_upper_index));
        };

        let lower_tick = skip_list::borrow_mut(&mut _tick_manager.ticks, tick_lower_score);
        update_by_liquidity(
            lower_tick,
            _current_tick_index,
            _liquidity_delta,
            true,
            false
        );
        let upper_tick = skip_list::borrow_mut(&mut _tick_manager.ticks, tick_upper_score);
        update_by_liquidity(
            upper_tick,
            _current_tick_index,
            _liquidity_delta,
            true,
            true
        );
    }

    public(friend) fun decrease_liquidity(
        _tick_manager: &mut TickManager,
        _current_tick_index: I32,
        _tick_lower_index: I32,
        _tick_upper_index: I32,
        _liquidity_delta: u128
    )
    {
        if (_liquidity_delta == 0) {
            return
        };
        let tick_lower_score = tick_score(_tick_lower_index);
        let tick_upper_score = tick_score(_tick_upper_index);
        assert!(
            !skip_list::contains(&_tick_manager.ticks, tick_lower_score) &&
                !skip_list::contains(&_tick_manager.ticks, tick_upper_score),
            ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY
        );
        let lower_tick = skip_list::borrow_mut(&mut _tick_manager.ticks, tick_lower_score);
        update_by_liquidity(
            lower_tick,
            _current_tick_index,
            _liquidity_delta,
            false,
            false
        );
        let is_liquidity_changed = lower_tick.liquidity_gross != _liquidity_delta;
        if (is_liquidity_changed && lower_tick.liquidity_gross == 0 && _current_tick_index != _tick_upper_index) {
            skip_list::remove(&mut _tick_manager.ticks, tick_lower_score);
        };
        let upper_tick = skip_list::borrow_mut(&mut _tick_manager.ticks, tick_upper_score);
        update_by_liquidity(
            upper_tick,
            _current_tick_index,
            _liquidity_delta,
            false,
            true
        );
        is_liquidity_changed = upper_tick.liquidity_gross != _liquidity_delta;
        if (is_liquidity_changed && upper_tick.liquidity_gross == 0 && _current_tick_index != _tick_lower_index) {
            skip_list::remove(&mut _tick_manager.ticks, tick_upper_score);
        };
    }

    public(friend) fun try_borrow_tick(_tick_manager: &TickManager, _tick_index: I32): Option<Tick> {
        let tick_score = tick_score(_tick_index);
        if (!skip_list::contains(&_tick_manager.ticks, tick_score)) {
            return option::none<Tick>()
        };
        let tick_borrow = skip_list::borrow(&_tick_manager.ticks, tick_score);
        return option::some(*tick_borrow)
    }

    public(friend) fun borrow_tick_for_swap(
        _tick_manager: &TickManager,
        _score: u64,
        _is_x2y: bool
    ): (&Tick, OptionU64) {
        let node = skip_list::borrow_node(&_tick_manager.ticks, _score);
        let score = if (_is_x2y) {
            skip_list::prev_score(node)
        } else {
            skip_list::next_score(node)
        };
        (skip_list::borrow_value(node), score)
    }

    public(friend) fun cross_by_swap(
        _tick_manager: &TickManager,
        _tick_index: I32,
        _is_x2y: bool,
        _liquidity: u128
    ): u128
    {
        let tick = skip_list::borrow(&_tick_manager.ticks, tick_score(_tick_index));
        let liquidity_net = if (_is_x2y) {
            i128::neg(tick.liquidity_net)
        } else {
            tick.liquidity_net
        };
        let liquidity_ret: u128;
        if (i128::is_neg(liquidity_net)) {
            assert!(i128::abs_u128(liquidity_net) <= _liquidity, ERR_TICK_LIQUIDITY_INSUFFICIENT);
            liquidity_ret = _liquidity - i128::abs_u128(liquidity_net);
        } else {
            assert!(math_u128::add_check(i128::abs_u128(liquidity_net), _liquidity), ERR_TICK_EXCEED_U128_MAXIMUM);
            liquidity_ret = i128::abs_u128(liquidity_net) + _liquidity;
        };
        liquidity_ret
    }

    public(friend) fun first_score_for_swap(
        _tick_manager: &TickManager,
        _tick_index: I32,
        _is_x2y: bool,
    ): OptionU64 {
        let score;
        if (_is_x2y) {
            score = tick_score(_tick_index);
            skip_list::find_prev(&_tick_manager.ticks, score, true)
        } else {
            if (i32::eq(
                _tick_index,
                i32::neg_from(tick_math::tick_bound() + 1),
            )) {
                score = tick_score(tick_math::min_tick());
                skip_list::find_next(&_tick_manager.ticks, score, true)
            } else {
                score = tick_score(_tick_index);
                skip_list::find_next(&_tick_manager.ticks, score, false)
            }
        }
    }
}
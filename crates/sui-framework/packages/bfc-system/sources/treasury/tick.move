#[allow(duplicate_alias)]
module bfc_system::tick {

    use bfc_system::i128::{Self, I128};
    use bfc_system::i32::{Self, I32};
    use bfc_system::math_u128;
    use bfc_system::option_u64::{Self, is_some, OptionU64};
    use bfc_system::skip_list::{Self, SkipList};
    use bfc_system::tick_math;

    #[test_only]
    use std::ascii::string;
    #[test_only]
    use std::debug;
    #[test_only]
    use bfc_system::position;
    #[test_only]
    use bfc_system::tick_math::get_sqrt_price_at_tick;
    #[test_only]
    use sui::object::{Self, UID};
    #[test_only]
    use sui::transfer;
    #[test_only]
    use sui::tx_context;

    //friend bfc_system::treasury;
    //friend bfc_system::vault;

    const ERR_TICK_EXCEED_TWICE_MAXIMUM: u64 = 400;
    const ERR_TICK_EXCEED_U128_MAXIMUM: u64 = 401;
    const ERR_TICK_LIQUIDITY_INSUFFICIENT: u64 = 402;
    const ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY: u64 = 403;
    const ERR_TICKS_REBUILD_NOT_EMPTY: u64 = 404;

    public struct TickManager has store {
        tick_spacing: u32,
        ticks: SkipList<Tick>
    }


    public struct Tick has store, copy, drop {
        index: I32,
        sqrt_price: u128,
        liquidity_net: I128,
        liquidity_gross: u128
    }

    //spec module { pragma verify = false; }

    public(package) fun create_tick_manager(
        _tick_spacing: u32,
        _ts: u64,
        _ctx: &mut TxContext,
    ): TickManager {
        TickManager {
            tick_spacing: _tick_spacing,
            ticks: skip_list::new(16, 2, _ts, _ctx),
        }
    }

    public(package) fun rebuild_ticks(_tick_manager: &mut TickManager, _ctx: &mut TxContext) {
        let _ticks = &_tick_manager.ticks;
        let mut scores = vector::empty<u64>();
        if (skip_list::length(_ticks) != 0) {
            let mut next_score = &skip_list::head(_ticks);
            while (is_some(next_score)) {
                let score = option_u64::borrow(next_score);
                vector::push_back(&mut scores, score);
                let node = skip_list::borrow_node(
                    _ticks,
                    score,
                );
                next_score = &skip_list::next_score(node);
            };
        };
        while (!vector::is_empty(&scores)) {
            let score = vector::pop_back(&mut scores);
            skip_list::remove<Tick>(&mut _tick_manager.ticks, score);
        };
        assert!(skip_list::is_empty(&_tick_manager.ticks), ERR_TICKS_REBUILD_NOT_EMPTY);
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

    public fun fetch_ticks(_tick_manager: &TickManager): vector<Tick> {
        let _ticks = &_tick_manager.ticks;
        let mut ticks = vector::empty<Tick>();
        if (skip_list::length(_ticks) != 0) {
            let mut next_score = &skip_list::head(_ticks);
            while (is_some(next_score)) {
                let score = option_u64::borrow(next_score);
                let node = skip_list::borrow_node(
                    _ticks,
                    score,
                );
                vector::push_back(&mut ticks, *skip_list::borrow<Tick>(_ticks, score));
                next_score = &skip_list::next_score(node);
            };
        };
        ticks
    }

    public(package) fun borrow_tick_for_swap(
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

    /// private fun
    fun default(_tick_index: I32): Tick {
        let sqrt_price = tick_math::get_sqrt_price_at_tick(_tick_index);
        Tick {
            sqrt_price,
            index: _tick_index,
            liquidity_net: i128::from(0),
            liquidity_gross: 0
        }
    }

    fun tick_score(_tick_index: I32): u64 {
        let score = i32::as_u32(i32::add(_tick_index, tick_math::max_tick()));
        assert!(
            score >= 0 && score <= i32::as_u32(i32::mul(tick_math::max_tick(), i32::from_u32(2))),
            ERR_TICK_EXCEED_TWICE_MAXIMUM
        );
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

    // spec update_by_liquidity {
    //     pragma opaque;
    // }

    /// add/remove liquidity
    public(package) fun increase_liquidity(
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

    public(package) fun decrease_liquidity(
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
            skip_list::contains(&_tick_manager.ticks, tick_lower_score),
            ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY
        );
        assert!(
            skip_list::contains(&_tick_manager.ticks, tick_upper_score),
            ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY
        );
        let lower_tick = skip_list::borrow_mut(&mut _tick_manager.ticks, tick_lower_score);
        update_by_liquidity(
            lower_tick,
            _current_tick_index,
            _liquidity_delta,
            false,
            true
        );
        let tick_bound = tick_math::tick_bound();
        let lower_tick_bound= i32::neg_from(tick_bound - tick_bound % _tick_manager.tick_spacing);
        if (lower_tick.liquidity_gross == 0 && !i32::eq(_tick_lower_index, lower_tick_bound)) {
            skip_list::remove(&mut _tick_manager.ticks, tick_lower_score);
        };
        let upper_tick = skip_list::borrow_mut(&mut _tick_manager.ticks, tick_upper_score);
        update_by_liquidity(
            upper_tick,
            _current_tick_index,
            _liquidity_delta,
            false,
            false
        );
        let upper_tick_bound = i32::from(tick_bound - tick_bound % _tick_manager.tick_spacing);
        if (upper_tick.liquidity_gross == 0 && !i32::eq(_tick_upper_index, upper_tick_bound)) {
            skip_list::remove(&mut _tick_manager.ticks, tick_upper_score);
        };
    }

    public(package) fun cross_by_tick(
        _tick: &Tick,
        _is_x2y: bool,
        _liquidity: u128
    ): u128
    {
        let liquidity_net = if (_is_x2y) {
            i128::neg(_tick.liquidity_net)
        } else {
            _tick.liquidity_net
        };
        let abs_liquidity_net = i128::abs_u128(liquidity_net);
        if (i128::is_neg(liquidity_net)) {
            assert!(abs_liquidity_net <= _liquidity, ERR_TICK_LIQUIDITY_INSUFFICIENT);
            _liquidity - abs_liquidity_net
        } else {
            assert!(math_u128::add_check(abs_liquidity_net, _liquidity), ERR_TICK_EXCEED_U128_MAXIMUM);
            _liquidity + abs_liquidity_net
        }
    }

    public(package) fun cross_by_swap(
        _tick_manager: &TickManager,
        _tick_index: I32,
        _is_x2y: bool,
        _liquidity: u128
    ): u128
    {
        let tick = skip_list::borrow(&_tick_manager.ticks, tick_score(_tick_index));
        cross_by_tick(tick, _is_x2y, _liquidity)
    }

    public(package) fun first_score_for_swap(
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

    public(package) fun get_ticks(
        _tick_manager: &TickManager,
        _tick_index: I32,
        _spacing_times: u32,
        _total_count: u32,
    ): vector<vector<I32>> {
        let gap = i32::from_u32(_spacing_times * _tick_manager.tick_spacing);
        let middle = tick_math::get_prev_valid_tick_index(_tick_index, _tick_manager.tick_spacing);
        let spacing_times = (_total_count - 1) / 2 * _spacing_times + (_spacing_times + 1) / 2;
        let mut lower = i32::sub(
            middle,
            i32::from_u32(_tick_manager.tick_spacing * spacing_times),
        );
        let mut count = _total_count;
        let mut ticks = vector::empty<vector<I32>>();
        while (count > 0) {
            let upper = i32::add(lower, gap);
            vector::push_back(&mut ticks, vector<I32>[lower, upper]);
            lower = upper;
            count = count - 1
        };
        ticks
    }

    #[test_only]
    public struct TestM has key, store {
        id: UID,
        m: TickManager,
    }

    #[test]
    fun test_get_ticks() {
        let is_debug = false;
        let mut ctx = tx_context::dummy();
        let tick_spacing: u32 = 60;
        let current_index = tick_math::get_tick_at_sqrt_price(100000000000);

        let m = create_tick_manager(tick_spacing, 123456, &mut ctx);
        let ticks = get_ticks(&m, current_index, 10, 9);
        transfer::public_transfer(TestM { id: object::new(&mut ctx), m }, tx_context::sender(&ctx));

        // check length
        assert!(vector::length(&ticks) == 9, 0);

        let middle_index = tick_math::get_prev_valid_tick_index(current_index, tick_spacing);
        let middle = i32::as_u32(middle_index);


        if (is_debug) {
            debug::print(&ticks);
            debug::print(&middle_index);
        };

        // check first
        let first = vector::borrow(&ticks, 0);
        position::check_position_tick_range(
            *vector::borrow(first, 0),
            *vector::borrow(first, 1),
            tick_spacing,
        );
        assert!(
            i32::eq(
                *vector::borrow(first, 0),
                i32::from_u32(middle - tick_spacing * 10 * 5 + 300)
            ),
            1,
        );

        // check last
        let last = vector::borrow(&ticks, 8);
        position::check_position_tick_range(
            *vector::borrow(last, 0),
            *vector::borrow(last, 1),
            tick_spacing,
        );
        assert!(
            i32::eq(
                *vector::borrow(last, 1),
                i32::from_u32(middle + tick_spacing * 10 * 5 - 300)
            ),
            2,
        );
    }

    #[test]
    fun test_fetch_ticks() {
        let is_debug = false;
        let mut ctx = tx_context::dummy();
        let tick_spacing: u32 = 30;
        let times = 10;
        let pnumber = 9;
        let current_index = tick_math::get_tick_at_sqrt_price(18446744073709551616);

        let m = create_tick_manager(tick_spacing, 123456, &mut ctx);
        let ticks = get_ticks(&m, current_index, times, pnumber);
        transfer::public_transfer(TestM { id: object::new(&mut ctx), m }, tx_context::sender(&ctx));
        if (is_debug) {
            debug::print(&ticks);
        };
        let mut i = 0;
        while (i < vector::length(&ticks)) {
            let current = vector::borrow(&ticks, i);

            let lower = vector::borrow(current, 0);
            let upper = vector::borrow(current, 1);
            if (is_debug) {
                debug::print(&string(b"\n ====== current index"));
                debug::print(current);

                debug::print(&string(b"\tlower price:"));
                debug::print(&get_sqrt_price_at_tick(*lower));
                debug::print(&string(b"\tuppper price:"));
                debug::print(&get_sqrt_price_at_tick(*upper));
            };
            i = i + 1
        };
    }
}
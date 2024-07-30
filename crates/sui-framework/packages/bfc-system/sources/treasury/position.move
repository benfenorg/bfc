module bfc_system::position {
    use std::ascii::into_bytes;
    use std::string::{Self, String};
    use std::type_name;
    use std::type_name::TypeName;

    use sui::bfc::BFC;

    use bfc_system::i32::{Self, I32};
    use bfc_system::linked_table;
    use bfc_system::math_u128;
    use bfc_system::tick_math;
    use bfc_system::utils::to_string;

    //friend bfc_system::treasury;
    //friend bfc_system::vault;

    const ERR_TICK_SPACING_INVALID_RANGE: u64 = 300;
    //const ERR_TICK_LOWER_TOO_SMALL: u64 = 301;
    //const ERR_TICK_UPPER_TOO_LARGE: u64 = 302;
    const ERR_TICK_INVALID_RANGE: u64 = 303;
    const ERR_TICK_INVALID_VALUE: u64 = 304;
    const ERR_INVALID_LIMIT: u64 = 305;
    //const ERR_INVALID_VECTOR_LENGTH: u64 = 306;
    const ERR_POSITION_INFO_NOT_EMPTY: u64 = 307;
    const ERR_POSITION_INFO_EMPTY: u64 = 308;
    const ERR_POSITION_INSUFFICIENT_LIQUIDITY: u64 = 309;
    const ERR_U128_ADD_CHECK_FAILED: u64 = 310;


    //spec module { pragma verify = false; }

    public struct PositionManager has store {
        vault_id: ID,
        tick_spacing: u32,
        position_index: u64,
        positions: linked_table::LinkedTable<u64, Position>,
    }

    public struct Position has copy, drop, store {
        vault_id: ID,
        index: u64,
        coin_type_a: TypeName,
        coin_type_b: TypeName,
        name: String,
        tick_lower_index: I32,
        tick_upper_index: I32,
        liquidity: u128,
    }

    /// create PositionManager
    public(package) fun create_position_manager(
        vault_id: ID,
        _tick_spacing: u32,
        _ctx: &mut TxContext,
    ): PositionManager {
        PositionManager {
            vault_id,
            tick_spacing: _tick_spacing,
            position_index: 0,
            positions: linked_table::new<u64, Position>(_ctx),
        }
    }

    /// position info
    public fun get_vault_id(_position: &Position): ID {
        _position.vault_id
    }

    public fun is_empty(_position: &Position): bool {
        _position.liquidity == 0
    }

    public fun get_liquidity(_position: &Position): u128 {
        _position.liquidity
    }

    public fun get_tick_range(_position: &Position): (I32, I32) {
        (_position.tick_lower_index, _position.tick_upper_index)
    }

    public fun is_position_exist(_manager: &PositionManager, _index: u64): bool {
        linked_table::contains(&_manager.positions, _index)
    }

    public fun get_total_positions(_manager: &PositionManager): u64 {
        linked_table::length(&_manager.positions)
    }

    public fun fetch_positions(
        _manager: &PositionManager,
        _start: u64,
        _limit: u64
    ): vector<Position> {
        assert!(_limit > 0 && _start > 0, ERR_INVALID_LIMIT);
        linked_table::fetch(
            &_manager.positions,
            _start,
            _limit
        )
    }

    public(package) fun borrow_mut_position(
        _manager: &mut PositionManager,
        _index: u64
    ): &mut Position {
        linked_table::borrow_mut(&mut _manager.positions, _index)
    }

    public(package) fun borrow_position(
        _manager: &PositionManager,
        _index: u64
    ): &Position {
        linked_table::borrow(&_manager.positions, _index)
    }

    /// check tick
    public fun check_position_tick_range(_lower: I32, _upper: I32, _tick_spacing: u32) {
        let tick_spacing = i32::from_u32(_tick_spacing);
        assert!(i32::gt(tick_spacing, tick_math::min_tick()), ERR_TICK_SPACING_INVALID_RANGE);
        assert!(i32::lt(tick_spacing, tick_math::max_tick()), ERR_TICK_SPACING_INVALID_RANGE);
        assert!(i32::lt(_lower, _upper), ERR_TICK_INVALID_RANGE);
        assert!(tick_math::is_valid_index(_lower, _tick_spacing), ERR_TICK_INVALID_VALUE);
        assert!(tick_math::is_valid_index(_upper, _tick_spacing), ERR_TICK_INVALID_VALUE);
    }

    /// open / close position
    public(package) fun open_position<StableCoinType>(
        _position_manager: &mut PositionManager,
        _vault_index: u64,
        _tick_lower: I32,
        _tick_upper: I32,
        _ctx: &mut TxContext
    ): u64
    {
        let tick_spacing = _position_manager.tick_spacing;
        check_position_tick_range(_tick_lower, _tick_upper, tick_spacing);
        _position_manager.position_index = _position_manager.position_index + 1;
        let position = Position {
            vault_id: _position_manager.vault_id,
            index: _position_manager.position_index,
            coin_type_a: type_name::get<StableCoinType>(),
            coin_type_b: type_name::get<BFC>(),
            name: new_position_name(_position_manager.position_index, _vault_index),
            tick_lower_index: _tick_lower,
            tick_upper_index: _tick_upper,
            liquidity: 0
        };
        linked_table::push_back(&mut _position_manager.positions, _position_manager.position_index, position);
        position.index
    }

    public(package) fun close_position(
        _manager: &mut PositionManager,
        _index: u64
    )
    {
        let position = linked_table::remove(&mut _manager.positions, _index);
        assert!(is_empty(&position), ERR_POSITION_INFO_NOT_EMPTY);
        destory(position);
        _manager.position_index = _manager.position_index - 1;
    }

    public(package) fun force_close_position(
        _manager: &mut PositionManager,
        _index: u64
    ) {
        let position = linked_table::remove(&mut _manager.positions, _index);
        destory(position);
        _manager.position_index = _manager.position_index - 1;
    }

    /// add/remove liquidity
    public(package) fun increase_liquidity(position: &mut Position, _liquidity_delta: u128): u128 {
        assert!(math_u128::add_check(_liquidity_delta, position.liquidity), ERR_U128_ADD_CHECK_FAILED);
        position.liquidity = position.liquidity + _liquidity_delta;
        position.liquidity
    }

    // spec increase_liquidity {
    //     pragma opaque;
    // }
    public(package) fun decrease_liquidity(position: &mut Position, _liquidity_delta: u128): u128 {
        assert!(!is_empty(position), ERR_POSITION_INFO_EMPTY);
        if (_liquidity_delta == 0) {
            return position.liquidity
        };
        assert!(position.liquidity >= _liquidity_delta, ERR_POSITION_INSUFFICIENT_LIQUIDITY);
        position.liquidity = position.liquidity - _liquidity_delta;
        position.liquidity
    }

    /// private fun
    fun destory(_position: Position) {}

    fun new_position_name(_position_index: u64, _vault_index: u64): String {
        let mut lp_name = string::utf8(b"");
        string::append_utf8(&mut lp_name, b"OpenBlock LP | Pool");
        string::append_utf8(&mut lp_name, b"-");
        string::append_utf8(&mut lp_name, into_bytes(to_string((_vault_index as u128))));
        string::append_utf8(&mut lp_name, b"-");
        string::append_utf8(&mut lp_name, into_bytes(to_string((_position_index as u128))));
        lp_name
    }
}

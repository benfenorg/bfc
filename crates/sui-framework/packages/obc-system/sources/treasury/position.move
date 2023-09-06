module obc_system::position {
    use std::ascii::into_bytes;
    use std::string::{Self, String};
    use std::type_name;
    use std::type_name::TypeName;
    use std::vector;

    use sui::object::{Self, ID};
    use sui::tx_context::TxContext;

    use obc_system::i32::{Self, I32};
    use obc_system::linked_table;
    use obc_system::math_u128;
    use obc_system::tick_math;
    use obc_system::utils::to_string;

    friend obc_system::treasury;
    friend obc_system::vault;

    const ERR_TICK_SPACING_INVALID_RANGE: u64 = 300;
    const ERR_TICK_LOWER_TOO_SMALL: u64 = 301;
    const ERR_TICK_UPPER_TOO_LARGE: u64 = 302;
    const ERR_TICK_INVALID_RANGE: u64 = 303;
    const ERR_TICK_INVALID_VALUE: u64 = 304;
    const ERR_INVALID_LIMIT: u64 = 305;
    const ERR_INVALID_VECTOR_LENGTH: u64 = 306;
    const ERR_POSITION_INFO_NOT_EMPTY: u64 = 307;
    const ERR_POSITION_INFO_EMPTY: u64 = 308;
    const ERR_POSITION_INSUFFICIENT_LIQUIDITY: u64 = 309;
    const ERR_U128_ADD_CHECK_FAILED: u64 = 310;

    struct PositionManager has store {
        vault_id: ID,
        tick_spacing: u32,
        position_index: u64,
        positions: linked_table::LinkedTable<ID, Position>,
    }

    struct Position has copy, drop, store {
        vault_id: ID,
        position_id: ID,
        index: u64,
        coin_type_a: TypeName,
        coin_type_b: TypeName,
        name: String,
        tick_lower_index: I32,
        tick_upper_index: I32,
        liquidity: u128,
    }

    /// create PositionManager
    public(friend) fun create_position_manager(
        vault_id: ID,
        _tick_spacing: u32,
        _ctx: &mut TxContext,
    ): PositionManager {
        PositionManager {
            vault_id,
            tick_spacing: _tick_spacing,
            position_index: 0,
            positions: linked_table::new<ID, Position>(_ctx),
        }
    }

    /// position info
    public fun get_vault_id(_position: &Position): ID {
        _position.vault_id
    }

    public fun get_position_id(_position: &Position): ID {
        _position.position_id
    }

    public fun is_empty(_position: &Position): bool {
        _position.liquidity == 0
    }

    public fun is_position_exist(_manager: &PositionManager, _position_id: ID): bool {
        linked_table::contains(&_manager.positions, _position_id)
    }

    public fun get_liquidity(_position: &Position): u128 {
        _position.liquidity
    }

    public fun fetch_positions(
        _manager: &PositionManager,
        _start: vector<ID>,
        _limit: u64
    ): vector<Position> {
        assert!(_limit > 0, ERR_INVALID_LIMIT);
        let len = vector::length(&_start);
        assert!(len > 0, ERR_INVALID_VECTOR_LENGTH);
        let ret = vector::empty<Position>();
        let idx = 0;
        while (idx < len) {
            let positions = linked_table::fetch(
                &_manager.positions,
                *vector::borrow<ID>(&_start, idx),
                _limit
            );
            if (vector::length(&positions) > 0) {
                vector::append(&mut ret, positions);
            };
            idx = idx + 1;
        };
        ret
    }

    public(friend) fun borrow_mut_position(
        _manager: &mut PositionManager,
        _tick_lower: I32,
        _tick_upper: I32
    ): &mut Position {
        let position_id = generate_position_key(_manager.vault_id, _tick_lower, _tick_upper);
        linked_table::borrow_mut(&mut _manager.positions, position_id)
    }

    public(friend) fun borrow_position(
        _manager: & PositionManager,
        _tick_lower: I32,
        _tick_upper: I32
    ): &Position {
        let position_id = generate_position_key(_manager.vault_id, _tick_lower, _tick_upper);
        linked_table::borrow(&_manager.positions, position_id)
    }

    /// check tick
    public fun check_position_tick_range(_lower: I32, _upper: I32, _tick_spacing: u32) {
        let tick_spacing = i32::from_u32(_tick_spacing);
        assert!(i32::gt(tick_spacing, tick_math::max_tick()), ERR_TICK_SPACING_INVALID_RANGE);
        assert!(i32::lt(tick_spacing, tick_math::min_tick()), ERR_TICK_SPACING_INVALID_RANGE);
        assert!(i32::lt(_lower, _upper), ERR_TICK_INVALID_RANGE);
        assert!(i32::gte(_lower, tick_math::min_tick()), ERR_TICK_LOWER_TOO_SMALL);
        assert!(i32::lte(_upper, tick_math::max_tick()), ERR_TICK_UPPER_TOO_LARGE);
        assert!(i32::eq(i32::zero(), i32::mod(_lower, tick_spacing)), ERR_TICK_INVALID_VALUE);
        assert!(i32::eq(i32::zero(), i32::mod(_upper, tick_spacing)), ERR_TICK_INVALID_VALUE);
    }

    /// open / close position
    public(friend) fun open_position<CoinTypeA, CoinTypeB>(
        _position_manager: &mut PositionManager,
        _vault_index: u64,
        _tick_lower: I32,
        _tick_upper: I32,
        _ctx: &mut TxContext
    ): ID
    {
        let tick_spacing = _position_manager.tick_spacing;
        check_position_tick_range(_tick_lower, _tick_upper, tick_spacing);
        _position_manager.position_index = _position_manager.position_index + 1;
        let position = Position {
            position_id: generate_position_key(_position_manager.vault_id, _tick_lower, _tick_lower),
            vault_id: _position_manager.vault_id,
            index: _position_manager.position_index,
            coin_type_a: type_name::get<CoinTypeA>(),
            coin_type_b: type_name::get<CoinTypeB>(),
            name: new_position_name(_position_manager.position_index, _vault_index),
            tick_lower_index: _tick_lower,
            tick_upper_index: _tick_upper,
            liquidity: 0
        };
        linked_table::push_back(&mut _position_manager.positions, position.position_id, position);
        position.position_id
    }

    public(friend) fun close_position(
        _manager: &mut PositionManager,
        _tick_lower: I32,
        _tick_upper: I32
    ): ID
    {
        let position_id = generate_position_key(_manager.vault_id, _tick_lower, _tick_upper);
        let position = linked_table::remove(&mut _manager.positions, position_id);
        assert!(!is_empty(&position), ERR_POSITION_INFO_NOT_EMPTY);
        destory(position);
        position_id
    }

    /// add/remove liquidity
    public(friend) fun increase_liquidity(position: &mut Position, _liquidity_delta: u128): u128 {
        assert!(!is_empty(position), ERR_POSITION_INFO_EMPTY);
        assert!(math_u128::add_check(_liquidity_delta, position.liquidity), ERR_U128_ADD_CHECK_FAILED);
        position.liquidity = position.liquidity + _liquidity_delta;
        position.liquidity
    }

    public(friend) fun decrease_liquidity(position: &mut Position, _liquidity_delta: u128): u128 {
        assert!(!is_empty(position), ERR_POSITION_INFO_EMPTY);
        if (_liquidity_delta == 0) {
            return position.liquidity
        };
        assert!(position.liquidity < _liquidity_delta, ERR_POSITION_INSUFFICIENT_LIQUIDITY);
        position.liquidity = position.liquidity - _liquidity_delta;
        position.liquidity
    }

    /// private fun
    fun destory(_position: Position) {}

    fun generate_position_key(
        _vault_id: ID,
        _tick_lower_index: I32,
        _tick_upper_index: I32
    ): ID
    {
        let bytes = vector::empty<u8>();
        vector::append(&mut bytes, object::id_to_bytes(&_vault_id));
        vector::append(&mut bytes, b"-");
        vector::append(&mut bytes, b"[");
        vector::append(&mut bytes, i32::get_bytes(_tick_lower_index));
        vector::append(&mut bytes, b",");
        vector::append(&mut bytes, i32::get_bytes(_tick_upper_index));
        vector::append(&mut bytes, b"]");
        object::id_from_bytes(sui::hash::blake2b256(&bytes))
    }

    fun new_position_name(_position_index: u64, _vault_index: u64): String {
        let lp_name = string::utf8(b"");
        string::append_utf8(&mut lp_name, b"OpenBlock LP | Pool");
        string::append_utf8(&mut lp_name, b"-");
        string::append_utf8(&mut lp_name, into_bytes(to_string((_vault_index as u128))));
        string::append_utf8(&mut lp_name, b"-");
        string::append_utf8(&mut lp_name, into_bytes(to_string((_position_index as u128))));
        lp_name
    }
}

#[test_only]
module obc_system::tick_test {
    use std::vector;
    use sui::transfer;
    use sui::tx_context;
    use sui::object::{Self, UID};
    use obc_system::tick;
    use obc_system::position;
    use obc_system::i32;
    use obc_system::tick_math;

    struct TestM has key, store {
        id: UID,
        m: tick::TickManager,
    }

    #[test]
    fun test_get_ticks() {
        let ctx = tx_context::dummy();
        let tick_spacing: u32 = 60;
        let current_index = tick_math::get_tick_at_sqrt_price(100000000000);

        let m = tick::create_tick_manager(tick_spacing, 123456, &mut ctx);
        let ticks = tick::get_ticks(&m, current_index, 10, 9);
        transfer::public_transfer(TestM { id: object::new(&mut ctx), m }, tx_context::sender(&ctx));

        // check length
        assert!(vector::length(&ticks) == 9, 0);

        let middle_index = tick_math::get_prev_valid_tick_index(current_index, tick_spacing);
        let middle = i32::as_u32(middle_index);

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
}

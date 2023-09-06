#[test_only]
module obc_system::tick_test {
    use std::vector;
    use std::debug;
    use sui::transfer;
    use sui::tx_context;
    use sui::object::{Self, UID};
    use obc_system::tick;
    use obc_system::i32;
    use obc_system::tick_math;

    struct TestM has key, store {
        id: UID,
        m: tick::TickManager,
    }

    #[test]
    fun test_get_ticks() {
        let start_index = tick_math::get_tick_at_sqrt_price(100000000000);
        debug::print(&start_index);
        let start_index_u32 = i32::as_u32(start_index);
        let ctx = tx_context::dummy();
        let m = tick::create_tick_manager(60, 123456, &mut ctx);
        let ticks = tick::get_ticks(&m, start_index, 10, 9);
        transfer::public_transfer(TestM { id: object::new(&mut ctx), m }, tx_context::sender(&ctx));

        debug::print(&ticks);
        assert!(vector::length(&ticks) == 9, 0);

        // check first
        let first = vector::borrow(&ticks, 0);
        assert!(
            i32::eq(
                *vector::borrow(first, 0),
                i32::from_u32(start_index_u32 - 600 * 5 + 300)
            ),
            1,
        );

        // check last
        let last = vector::borrow(&ticks, 8);
        assert!(
            i32::eq(
                *vector::borrow(last, 1),
                i32::from_u32(start_index_u32 + 600 * 5 - 300)
            ),
            2,
        );
    }
}

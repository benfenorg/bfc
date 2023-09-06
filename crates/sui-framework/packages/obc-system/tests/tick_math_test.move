#[test_only]
module obc_system::tick_math_test {
    use obc_system::i32;
    use obc_system::tick_math::{
        get_sqrt_price_at_tick, get_tick_at_sqrt_price,
        max_tick, min_tick
        };

    // The maximum tick that may be passed to #getSqrtRatioAtTick computed from log base 1.0001 of 2**64
    const MAX_TICK: u32 = 443636;
    const MIN_SQRT_PRICE_X64: u128 = 4295048016;
    const MAX_SQRT_PRICE_X64: u128 = 79226673515401279992447579055;

    /// Errors
    const EINVALID_TICK: u64 = 0;
    const EINVALID_SQRT_PRICE: u64 = 1;

    #[test]
    fun test_get_sqrt_price_at_tick() {
        // min tick
        assert!(get_sqrt_price_at_tick(i32::neg_from(MAX_TICK)) == 4295048016u128, 2);
        // max tick
        assert!(get_sqrt_price_at_tick(i32::from(MAX_TICK)) == 79226673515401279992447579055u128, 1);
        assert!(get_sqrt_price_at_tick(i32::neg_from(435444u32)) == 6469134034u128, 3);
        assert!(get_sqrt_price_at_tick(i32::from(408332u32)) == 13561044167458152057771544136u128, 4);
    }

    #[test]
    fun test_tick_swap_sqrt_price() {
        let t = i32::from(401098);
        while (i32::lte(t, i32::from(401200))) {
            let sqrt_price = get_sqrt_price_at_tick(t);
            let tick = get_tick_at_sqrt_price(sqrt_price);
            assert!(i32::eq(t, tick) == true, 0);
            t = i32::add(t, i32::from(1));
        }
    }

    #[test]
    fun test_get_tick_at_sqrt_price_1() {
        assert!(i32::eq(get_tick_at_sqrt_price(6469134034u128), i32::neg_from(435444)) == true, 0);
        assert!(i32::eq(get_tick_at_sqrt_price(13561044167458152057771544136u128), i32::from(408332u32)) == true, 0);
    }

    #[test]
    #[expected_failure]
    fun test_get_sqrt_price_at_invalid_upper_tick() {
        get_sqrt_price_at_tick(i32::add(max_tick(), i32::from(1)));
    }

    #[test]
    #[expected_failure]
    fun test_get_sqrt_price_at_invalid_lower_tick() {
        get_sqrt_price_at_tick(i32::sub(min_tick(), i32::from(1)));
    }

    #[test]
    #[expected_failure]
    fun test_get_tick_at_invalid_lower_sqrt_price() {
        get_tick_at_sqrt_price(MAX_SQRT_PRICE_X64 + 1);
    }

    #[test]
    #[expected_failure]
    fun test_get_tick_at_invalid_upper_sqrt_price() {
        get_tick_at_sqrt_price(MIN_SQRT_PRICE_X64 - 1);
    }
}
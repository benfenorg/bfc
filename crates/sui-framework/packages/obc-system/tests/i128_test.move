#[test_only]
module obc_system::i128_test {

    use obc_system::i128::{
        abs, abs_u128, add, as_u128, cmp,
        div, from, mul, neg_from, overflowing_add, overflowing_sub, shl, shr,
        sign, sub, wrapping_add, wrapping_sub
    };

    const EOverflow: u64 = 0;

    const MIN_AS_U128: u128 = 1 << 127;
    const MAX_AS_U128: u128 = 0x7fffffffffffffffffffffffffffffff;

    const LT: u8 = 0;
    const EQ: u8 = 1;
    const GT: u8 = 2;

    #[test]
    #[expected_failure]
    fun test_from_overflow() {
        as_u128(from(MIN_AS_U128));
        as_u128(from(0xffffffffffffffffffffffffffffffff));
    }

    #[test]
    fun test_neg_from() {
        assert!(as_u128(neg_from(0)) == 0, 0);
        assert!(as_u128(neg_from(1)) == 0xffffffffffffffffffffffffffffffff, 1);
        assert!(as_u128(neg_from(0x7fffffffffffffffffffffffffffffff)) == 0x80000000000000000000000000000001, 2);
        assert!(as_u128(neg_from(MIN_AS_U128)) == MIN_AS_U128, 2);
    }

    #[test]
    #[expected_failure]
    fun test_neg_from_overflow() {
        neg_from(0x80000000000000000000000000000001);
    }

    #[test]
    fun test_abs() {
        assert!(as_u128(from(10)) == 10u128, 0);
        assert!(as_u128(abs(neg_from(10))) == 10u128, 1);
        assert!(as_u128(abs(neg_from(0))) == 0u128, 2);
        assert!(as_u128(abs(neg_from(0x7fffffffffffffffffffffffffffffff))) == 0x7fffffffffffffffffffffffffffffff, 3);
        assert!(as_u128(neg_from(MIN_AS_U128)) == MIN_AS_U128, 4);
    }

    #[test]
    #[expected_failure]
    fun test_abs_overflow() {
        abs(neg_from(1 << 127));
    }

    #[test]
    fun test_wrapping_add() {
        assert!(as_u128(wrapping_add(from(0), from(1))) == 1, 0);
        assert!(as_u128(wrapping_add(from(1), from(0))) == 1, 0);
        assert!(as_u128(wrapping_add(from(10000), from(99999))) == 109999, 0);
        assert!(as_u128(wrapping_add(from(99999), from(10000))) == 109999, 0);
        assert!(as_u128(wrapping_add(from(MAX_AS_U128 - 1), from(1))) == MAX_AS_U128, 0);
        assert!(as_u128(wrapping_add(from(0), from(0))) == 0, 0);

        assert!(as_u128(wrapping_add(neg_from(0), neg_from(0))) == 0, 1);
        assert!(as_u128(wrapping_add(neg_from(1), neg_from(0))) == 0xffffffffffffffffffffffffffffffff, 1);
        assert!(as_u128(wrapping_add(neg_from(0), neg_from(1))) == 0xffffffffffffffffffffffffffffffff, 1);
        assert!(as_u128(wrapping_add(neg_from(10000), neg_from(99999))) == 0xfffffffffffffffffffffffffffe5251, 1);
        assert!(as_u128(wrapping_add(neg_from(99999), neg_from(10000))) == 0xfffffffffffffffffffffffffffe5251, 1);
        assert!(as_u128(wrapping_add(neg_from(MIN_AS_U128 - 1), neg_from(1))) == MIN_AS_U128, 1);

        assert!(as_u128(wrapping_add(from(0), neg_from(0))) == 0, 2);
        assert!(as_u128(wrapping_add(neg_from(0), from(0))) == 0, 2);
        assert!(as_u128(wrapping_add(neg_from(1), from(1))) == 0, 2);
        assert!(as_u128(wrapping_add(from(1), neg_from(1))) == 0, 2);
        assert!(as_u128(wrapping_add(from(10000), neg_from(99999))) == 0xfffffffffffffffffffffffffffea071, 2);
        assert!(as_u128(wrapping_add(from(99999), neg_from(10000))) == 89999, 2);
        assert!(as_u128(wrapping_add(neg_from(MIN_AS_U128), from(1))) == 0x80000000000000000000000000000001, 2);
        assert!(as_u128(wrapping_add(from(MAX_AS_U128), neg_from(1))) == MAX_AS_U128 - 1, 2);

        assert!(as_u128(wrapping_add(from(MAX_AS_U128), from(1))) == MIN_AS_U128, 2);
    }

    #[test]
    fun test_add() {
        assert!(as_u128(add(from(0), from(0))) == 0, 0);
        assert!(as_u128(add(from(0), from(1))) == 1, 0);
        assert!(as_u128(add(from(1), from(0))) == 1, 0);
        assert!(as_u128(add(from(10000), from(99999))) == 109999, 0);
        assert!(as_u128(add(from(99999), from(10000))) == 109999, 0);
        assert!(as_u128(add(from(MAX_AS_U128 - 1), from(1))) == MAX_AS_U128, 0);

        assert!(as_u128(add(neg_from(0), neg_from(0))) == 0, 1);
        assert!(as_u128(add(neg_from(1), neg_from(0))) == 0xffffffffffffffffffffffffffffffff, 1);
        assert!(as_u128(add(neg_from(0), neg_from(1))) == 0xffffffffffffffffffffffffffffffff, 1);
        assert!(as_u128(add(neg_from(10000), neg_from(99999))) == 0xfffffffffffffffffffffffffffe5251, 1);
        assert!(as_u128(add(neg_from(99999), neg_from(10000))) == 0xfffffffffffffffffffffffffffe5251, 1);
        assert!(as_u128(add(neg_from(MIN_AS_U128 - 1), neg_from(1))) == MIN_AS_U128, 1);

        assert!(as_u128(add(from(0), neg_from(0))) == 0, 2);
        assert!(as_u128(add(neg_from(0), from(0))) == 0, 2);
        assert!(as_u128(add(neg_from(1), from(1))) == 0, 2);
        assert!(as_u128(add(from(1), neg_from(1))) == 0, 2);
        assert!(as_u128(add(from(10000), neg_from(99999))) == 0xfffffffffffffffffffffffffffea071, 2);
        assert!(as_u128(add(from(99999), neg_from(10000))) == 89999, 2);
        assert!(as_u128(add(neg_from(MIN_AS_U128), from(1))) == 0x80000000000000000000000000000001, 2);
        assert!(as_u128(add(from(MAX_AS_U128), neg_from(1))) == MAX_AS_U128 - 1, 2);
    }

    #[test]
    fun test_overflowing_add() {
        let (result, overflow) = overflowing_add(from(MAX_AS_U128), neg_from(1));
        assert!(overflow == false && as_u128(result) == MAX_AS_U128 - 1, 1);
        let (_, overflow) = overflowing_add(from(MAX_AS_U128), from(1));
        assert!(overflow == true, 1);
        let (_, overflow) = overflowing_add(neg_from(MIN_AS_U128), neg_from(1));
        assert!(overflow == true, 1);
    }

    #[test]
    #[expected_failure]
    fun test_add_overflow() {
        add(from(MAX_AS_U128), from(1));
    }

    #[test]
    #[expected_failure]
    fun test_add_underflow() {
        add(neg_from(MIN_AS_U128), neg_from(1));
    }

    #[test]
    fun test_wrapping_sub() {
        assert!(as_u128(wrapping_sub(from(0), from(0))) == 0, 0);
        assert!(as_u128(wrapping_sub(from(1), from(0))) == 1, 0);
        assert!(as_u128(wrapping_sub(from(0), from(1))) == as_u128(neg_from(1)), 0);
        assert!(as_u128(wrapping_sub(from(1), from(1))) == as_u128(neg_from(0)), 0);
        assert!(as_u128(wrapping_sub(from(1), neg_from(1))) == as_u128(from(2)), 0);
        assert!(as_u128(wrapping_sub(neg_from(1), from(1))) == as_u128(neg_from(2)), 0);
        assert!(as_u128(wrapping_sub(from(1000000), from(1))) == 999999, 0);
        assert!(as_u128(wrapping_sub(neg_from(1000000), neg_from(1))) == as_u128(neg_from(999999)), 0);
        assert!(as_u128(wrapping_sub(from(1), from(1000000))) == as_u128(neg_from(999999)), 0);
        assert!(as_u128(wrapping_sub(from(MAX_AS_U128), from(MAX_AS_U128))) == as_u128(from(0)), 0);
        assert!(as_u128(wrapping_sub(from(MAX_AS_U128), from(1))) == as_u128(from(MAX_AS_U128 - 1)), 0);
        assert!(as_u128(wrapping_sub(from(MAX_AS_U128), neg_from(1))) == as_u128(neg_from(MIN_AS_U128)), 0);
        assert!(as_u128(wrapping_sub(neg_from(MIN_AS_U128), neg_from(1))) == as_u128(neg_from(MIN_AS_U128 - 1)), 0);
        assert!(as_u128(wrapping_sub(neg_from(MIN_AS_U128), from(1))) == as_u128(from(MAX_AS_U128)), 0);
    }

    #[test]
    fun test_sub() {
        assert!(as_u128(sub(from(0), from(0))) == 0, 0);
        assert!(as_u128(sub(from(1), from(0))) == 1, 0);
        assert!(as_u128(sub(from(0), from(1))) == as_u128(neg_from(1)), 0);
        assert!(as_u128(sub(from(1), from(1))) == as_u128(neg_from(0)), 0);
        assert!(as_u128(sub(from(1), neg_from(1))) == as_u128(from(2)), 0);
        assert!(as_u128(sub(neg_from(1), from(1))) == as_u128(neg_from(2)), 0);
        assert!(as_u128(sub(from(1000000), from(1))) == 999999, 0);
        assert!(as_u128(sub(neg_from(1000000), neg_from(1))) == as_u128(neg_from(999999)), 0);
        assert!(as_u128(sub(from(1), from(1000000))) == as_u128(neg_from(999999)), 0);
        assert!(as_u128(sub(from(MAX_AS_U128), from(MAX_AS_U128))) == as_u128(from(0)), 0);
        assert!(as_u128(sub(from(MAX_AS_U128), from(1))) == as_u128(from(MAX_AS_U128 - 1)), 0);
        assert!(as_u128(sub(neg_from(MIN_AS_U128), neg_from(1))) == as_u128(neg_from(MIN_AS_U128 - 1)), 0);
    }

    #[test]
    fun test_checked_sub() {
        let (result, overflowing) = overflowing_sub(from(MAX_AS_U128), from(1));
        assert!(overflowing == false && as_u128(result) == MAX_AS_U128 - 1, 1);

        let (_, overflowing) = overflowing_sub(neg_from(MIN_AS_U128), from(1));
        assert!(overflowing == true, 1);

        let (_, overflowing) = overflowing_sub(from(MAX_AS_U128), neg_from(1));
        assert!(overflowing == true, 1);
    }

    #[test]
    #[expected_failure]
    fun test_sub_overflow() {
        sub(from(MAX_AS_U128), neg_from(1));
    }

    #[test]
    #[expected_failure]
    fun test_sub_underflow() {
        sub(neg_from(MIN_AS_U128), from(1));
    }

    #[test]
    fun test_mul() {
        assert!(as_u128(mul(from(1), from(1))) == 1, 0);
        assert!(as_u128(mul(from(10), from(10))) == 100, 0);
        assert!(as_u128(mul(from(100), from(100))) == 10000, 0);
        assert!(as_u128(mul(from(10000), from(10000))) == 100000000, 0);

        assert!(as_u128(mul(neg_from(1), from(1))) == as_u128(neg_from(1)), 0);
        assert!(as_u128(mul(neg_from(10), from(10))) == as_u128(neg_from(100)), 0);
        assert!(as_u128(mul(neg_from(100), from(100))) == as_u128(neg_from(10000)), 0);
        assert!(as_u128(mul(neg_from(10000), from(10000))) == as_u128(neg_from(100000000)), 0);

        assert!(as_u128(mul(from(1), neg_from(1))) == as_u128(neg_from(1)), 0);
        assert!(as_u128(mul(from(10), neg_from(10))) == as_u128(neg_from(100)), 0);
        assert!(as_u128(mul(from(100), neg_from(100))) == as_u128(neg_from(10000)), 0);
        assert!(as_u128(mul(from(10000), neg_from(10000))) == as_u128(neg_from(100000000)), 0);
        assert!(as_u128(mul(from(MIN_AS_U128 / 2), neg_from(2))) == as_u128(neg_from(MIN_AS_U128)), 0);
    }

    #[test]
    #[expected_failure]
    fun test_mul_overflow() {
        mul(from(MIN_AS_U128 / 2), from(1));
        mul(neg_from(MIN_AS_U128 / 2), neg_from(2));
    }

    #[test]
    fun test_div() {
        assert!(as_u128(div(from(0), from(1))) == 0, 0);
        assert!(as_u128(div(from(10), from(1))) == 10, 0);
        assert!(as_u128(div(from(10), neg_from(1))) == as_u128(neg_from(10)), 0);
        assert!(as_u128(div(neg_from(10), neg_from(1))) == as_u128(from(10)), 0);

        assert!(abs_u128(neg_from(MIN_AS_U128)) == MIN_AS_U128, 0);
        assert!(as_u128(div(neg_from(MIN_AS_U128), from(1))) == MIN_AS_U128, 0);
    }

    #[test]
    #[expected_failure]
    fun test_div_overflow() {
        div(neg_from(MIN_AS_U128), neg_from(1));
    }

    #[test]
    fun test_shl() {
        assert!(as_u128(shl(from(10), 0)) == 10, 0);
        assert!(as_u128(shl(neg_from(10), 0)) == as_u128(neg_from(10)), 0);

        assert!(as_u128(shl(from(10), 1)) == 20, 0);
        assert!(as_u128(shl(neg_from(10), 1)) == as_u128(neg_from(20)), 0);

        assert!(as_u128(shl(from(10), 8)) == 2560, 0);
        assert!(as_u128(shl(neg_from(10), 8)) == as_u128(neg_from(2560)), 0);

        assert!(as_u128(shl(from(10), 32)) == 42949672960, 0);
        assert!(as_u128(shl(neg_from(10), 32)) == as_u128(neg_from(42949672960)), 0);

        assert!(as_u128(shl(from(10), 64)) == 184467440737095516160, 0);
        assert!(as_u128(shl(neg_from(10), 64)) == as_u128(neg_from(184467440737095516160)), 0);

        assert!(as_u128(shl(from(10), 127)) == 0, 0);
        assert!(as_u128(shl(neg_from(10), 127)) == 0, 0);
    }

    #[test]
    fun test_shr() {
        assert!(as_u128(shr(from(10), 0)) == 10, 0);
        assert!(as_u128(shr(neg_from(10), 0)) == as_u128(neg_from(10)), 0);

        assert!(as_u128(shr(from(10), 1)) == 5, 0);
        assert!(as_u128(shr(neg_from(10), 1)) == as_u128(neg_from(5)), 0);

        assert!(as_u128(shr(from(MAX_AS_U128), 8)) == 0x7fffffffffffffffffffffffffffff, 0);
        assert!(as_u128(shr(neg_from(MIN_AS_U128), 8)) == 0xff800000000000000000000000000000, 0);

        assert!(as_u128(shr(from(MAX_AS_U128), 96)) == 0x7fffffff, 0);
        assert!(as_u128(shr(neg_from(MIN_AS_U128), 96)) == 0xffffffffffffffffffffffff80000000, 0);

        assert!(as_u128(shr(from(MAX_AS_U128), 127)) == 0, 0);
        assert!(as_u128(shr(neg_from(MIN_AS_U128), 127)) == 0xffffffffffffffffffffffffffffffff, 0);
    }

    #[test]
    fun test_sign() {
        assert!(sign(neg_from(10)) == 1u8, 0);
        assert!(sign(from(10)) == 0u8, 0);
    }

    #[test]
    fun test_cmp() {
        assert!(cmp(from(1), from(0)) == GT, 0);
        assert!(cmp(from(0), from(1)) == LT, 0);

        assert!(cmp(from(0), neg_from(1)) == GT, 0);
        assert!(cmp(neg_from(0), neg_from(1)) == GT, 0);
        assert!(cmp(neg_from(1), neg_from(0)) == LT, 0);

        assert!(cmp(neg_from(MIN_AS_U128), from(MAX_AS_U128)) == LT, 0);
        assert!(cmp(from(MAX_AS_U128), neg_from(MIN_AS_U128)) == GT, 0);

        assert!(cmp(from(MAX_AS_U128), from(MAX_AS_U128 - 1)) == GT, 0);
        assert!(cmp(from(MAX_AS_U128 - 1), from(MAX_AS_U128)) == LT, 0);

        assert!(cmp(neg_from(MIN_AS_U128), neg_from(MIN_AS_U128 - 1)) == LT, 0);
        assert!(cmp(neg_from(MIN_AS_U128 - 1), neg_from(MIN_AS_U128)) == GT, 0);
    }

    #[test]
    fun test_castdown() {
        assert!((1u128 as u8) == 1u8, 0);
    }
}
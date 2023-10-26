module bfc_system::utils {
    use std::ascii::{Self, String};
    use std::type_name::get;
    use std::vector;

    use bfc_system::comparator;

    #[test_only]
    use std::ascii::into_bytes;

    public fun to_string(value: u128): String {
        if (value == 0) {
            return ascii::string(b"0")
        };
        let buffer = vector::empty<u8>();
        while (value != 0) {
            vector::push_back(&mut buffer, ((48 + value % 10) as u8));
            value = value / 10;
        };
        vector::reverse(&mut buffer);
        ascii::string(buffer)
    }

    /// 0: x < y  1: x = y  2: x > y
    public fun cmp<X, Y>(): u8 {
        let comp = comparator::compare(&get<X>(), &get<Y>());
        if (comparator::is_equal(&comp)) {
            1
        } else if (comparator::is_smaller_than(&comp)) {
            0
        } else {
            2
        }
    }

    #[test]
    fun test_to_string() {
        assert!(b"0" == into_bytes(to_string(0)), 0);
        assert!(b"1" == into_bytes(to_string(1)), 1);
        assert!(b"10" == into_bytes(to_string(10)), 2);
        assert!(b"257" == into_bytes(to_string(257)), 3);
        assert!(b"1111111111" == into_bytes(to_string(1111111111)), 4);
        assert!(
            b"340282366920938463463374607431768211455" == into_bytes(
                to_string(340282366920938463463374607431768211455)
            ),
            5
        );
    }
}

module obc_system::utils {
    use std::ascii::{Self, String};
    use std::type_name::get;
    use std::vector;

    use obc_system::comparator;

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
}

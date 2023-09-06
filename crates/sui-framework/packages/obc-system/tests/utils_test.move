#[test_only]
module obc_system::utils_test {
    use std::ascii::into_bytes;

    use obc_system::utils::to_string;

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

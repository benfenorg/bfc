#[test_only]
module obc_system::option_u64_test {
    use std::option;

    use obc_system::option_u64;

    #[test]
    fun test_opt() {
        let a = option_u64::some(10000u64);
        let n = 0;
        while (n < 10000) {
            _ = option_u64::borrow(&a);
            n = n + 1;
        };
    }

    #[test]
    fun test_option_contains() {
        let a = option::some(100000);
        let n = 0;
        while (n < 10000) {
            option::contains(&a, &100000);
            n = n + 1;
        }
    }
}
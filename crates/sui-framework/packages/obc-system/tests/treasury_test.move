#[test_only]
module obc_system::treasury_test {
    use std::debug;
    use obc_system::treasury;

    struct TestCoinA  {}
    struct TestCoinB  {}

    #[test]
    fun test_generate_vault_key() {
        let key = treasury::generate_vault_key<TestCoinA, TestCoinB>(60);
        debug::print(&key);

    }
}

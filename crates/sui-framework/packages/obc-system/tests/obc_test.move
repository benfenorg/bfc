#[test_only]
module obc_system::fang_tests {
    use obc_system::obc;

    #[test]
    fun test_length() {
        let i = obc::length();
        assert!(i == 32, 0);
    }
}
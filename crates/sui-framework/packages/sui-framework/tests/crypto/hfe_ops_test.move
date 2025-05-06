#[test_only]
module sui::hfe_ops_test {
    use sui::hfe_ops;
    #[test]
    fun test_hfe_ops_add() {
        let result = hfe_ops::hfe_ops_add(2, 3);
        assert!(result == 5);
    }

    #[test]
    fun test_split_data() {
        let pk1 = x"4f0adab8fe9f36875f6b7f28d9679c37ab2c96224e50224b5bda5add5b1ee7bb";
        let result = hfe_ops::split_data(&pk1, 3, 1);
        assert!(result.length() != 0);
    }
}


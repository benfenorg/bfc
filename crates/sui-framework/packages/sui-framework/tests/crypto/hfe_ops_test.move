#[test_only]
module sui::hfe_ops_test {
    use sui::hfe_ops;
    #[test]
    fun test_hfe_ops_add() {
        let pk1 = x"0227322b3a891a0a280d6bc1fb2cbb23d28f54906fd6407f5f741f6def5762609a";
        let pk2 = x"0227322b3a891a0a280d6bc1fb2cbb23d28f54906fd6407f5f741f6def5762609b";

        let result = hfe_ops::hfe_ops_add(&pk1, &pk2);
        assert!(result == false);

    }
}


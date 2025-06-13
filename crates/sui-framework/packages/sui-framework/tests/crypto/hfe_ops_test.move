#[test_only]
module sui::hfe_ops_test {
    use std::debug;
    use std::uq32_32::le;
    use sui::hex;
    use sui::hfe_ops;
    #[test]
    fun test_hfe_ops_add_call_rpc() {
        let result = hfe_ops::hfe_ops_add(2, 3, 4, 5);
        let data1 = result[0];
        let data2 = result[1];
        debug::print(&data1);
        debug::print(&data2);
        assert!(&data1 == 5);
        assert!(&data2 == 9);
    }

    #[test]
    fun test_hfe_ops_split_value() {
        let result = hfe_ops::hfe_ops_split_value(5);
        let data1 = result[0];
        let data2 = result[1];
        debug::print(&data1);
        debug::print(&data2);
        assert!(&data1 == 2);
        assert!(&data2 == 3);
       // assert!(&data1 == 5);
    }

    #[test]
    fun test_hfe_ops_restore_value() {
        let object_id : vector<u8> =
            x"00000000000000000000000000000000000000000000000000000011111111";
        let signature = x"4f0adab8fe9f36875f6b7f28d9679c37ab2c96224e50224b5bda5add5b1ee7bb";
        let result = hfe_ops::hfe_ops_restore_value(4, 5, signature, object_id);
        let data1 = result;
        debug::print(&data1);
        assert!(&data1 == 9);
    }

    #[test]
    fun test_split_data() {

        let pk1 = x"4f0adab8fe9f36875f6b7f28d9679c37ab2c96224e50224b5bda5add5b1ee7bb";
        let result = hfe_ops::split_data(&pk1, 3, 1);
        assert!(result.length() != 0);
    }
}


#[test_only]
module sui::hfe_ops_test {
    use std::debug;
    //use std::uq32_32::le;
    //use sui::hex;
    use sui::hfe_ops;
    use sui::anonymous_balance::{Self};
    use sui::test_utils::assert_eq;

    #[test]
    fun test_hfe_ops_add() {
        let owner : address = @0x1;
       let (value0, value1) = anonymous_balance::anoymous_coin_split_value(10);
       let (value2, value3) = anonymous_balance::anoymous_coin_split_value(11);

       let (result0, result1) = hfe_ops::hfe_ops_add(value0, value1, value2, value3, owner);
       assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 21) == 0);
       let (result2, result3) = hfe_ops::hfe_ops_add(value0, value1, value2, value3, owner);
       assert_eq(result0, result2);
       assert_eq(result1, result3);
    }

    #[test]
    fun test_hfe_ops_minus() {
        let owner : address = @0x1;
        let (value0, value1) = anonymous_balance::anoymous_coin_split_value(22);
        let (value2, value3) = anonymous_balance::anoymous_coin_split_value(11);

        let (result0, result1) = hfe_ops::hfe_ops_minus(value0, value1, value2, value3, owner);
        assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 11) == 0);
        let (result2, result3) = hfe_ops::hfe_ops_minus(value0, value1, value2, value3, owner);
        assert_eq(result0, result2);
        assert_eq(result1, result3);
    }

    #[test]
    fun test_hfe_ops_multiplied() {
         let owner : address = @0x1;
         let (value0, value1) = anonymous_balance::anoymous_coin_split_value(22);
         let (value2, value3) = anonymous_balance::anoymous_coin_split_value(10);

         let (result0, result1) = hfe_ops::hfe_ops_multiplied(value0, value1, value2, value3, owner);
         assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 220) == 0);

         let (result2, result3) = hfe_ops::hfe_ops_multiplied(value0, value1, value2, value3, owner);
         assert_eq(result0, result2);
         assert_eq(result1, result3);
    }

    #[test]
   fun test_hfe_ops_encode_data() {
        let owner : address = @0x1;
        let (result0, result1) = hfe_ops::hfe_ops_encode_data(5, owner);
        assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 5) == 0);
        assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 10) == 2);
        // repeat split value
        let (result2, result3) = hfe_ops::hfe_ops_encode_data(5, owner);
        assert_eq(result0, result2);
        assert_eq(result1, result3);
   }
}


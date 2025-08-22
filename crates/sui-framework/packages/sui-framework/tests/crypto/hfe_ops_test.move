#[test_only]
module sui::hfe_ops_test {
    use std::debug;
    //use std::uq32_32::le;
    //use sui::hex;
    use sui::hfe_ops;
    use sui:: anonymous_balance::{Self};

    #[test]
    fun test_hfe_ops_add() {
       let (value0, value1) = anonymous_balance::anoymous_coin_split_value(10);
       let (value2, value3) = anonymous_balance::anoymous_coin_split_value(11);

       let (result0, result1) = hfe_ops::hfe_ops_add(value0, value1, value2, value3);
       assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 21) == 0);
    }

    #[test]
    fun test_hfe_ops_minus() {
        let (value0, value1) = anonymous_balance::anoymous_coin_split_value(22);
        let (value2, value3) = anonymous_balance::anoymous_coin_split_value(11);

        let (result0, result1) = hfe_ops::hfe_ops_minus(value0, value1, value2, value3);
        assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 11) == 0);
    }

    #[test]
    fun test_hfe_ops_multiplied() {
         let (value0, value1) = anonymous_balance::anoymous_coin_split_value(22);
         let (value2, value3) = anonymous_balance::anoymous_coin_split_value(10);

         let (result0, result1) = hfe_ops::hfe_ops_multiplied(value0, value1, value2, value3);
         assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 220) == 0);
    }

    #[test]
   fun test_hfe_ops_split_value() {
        let (result0, result1) = hfe_ops::hfe_ops_split_value(5);
        assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 5) == 0);
        assert!(anonymous_balance::compare_anoymous_coin(result0, result1, 10) == 2);
   }

    #[test]
    fun test_hfe_ops_restore_value() {
        let (value0, value1) = anonymous_balance::anoymous_coin_split_value(22);
        let object_id : address = @0xd4c2360f11b1608f3be0b8d89bc97ff3047378dbc34d0b3976f40e6392496fd5;
        let signature = x"0f31177f8ece16b2cfb8c1ba0b71f73252acaa6cfbbe13d36c3320617f05bc7f9a860f16c8b10c787455a01ca7bcca3469858aae4e369bc994ab64967f1fd20f";
        let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
        let result = hfe_ops::hfe_ops_restore_value(value0, value1, signature, object_id, publickey);
        let data1 = result;
        debug::print(&data1);
        assert!(&data1 == 22);
    }
}


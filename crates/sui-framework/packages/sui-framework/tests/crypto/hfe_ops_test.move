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

  //  #[test]
  //  fun test_hfe_ops_restore_value() {
  //      let object_id : address = @0x33a2598b7c5e22d03967b42671926c5c18e82f0be9973c077041e9912696f910;
  //      let signature = x"cd5f94646b13eaa370a55fe9c084d6b266e1c3856c16e43fbc8b2e9a28076ffe7b73fec594974a0ff7a7ebac2cf9ad2196ff89fdd97c14c0883159ec0181730c";
  //      let publickey = x"8496d3d932986b43bb64b5d5c7548d5c97a73aebf4301447f3746680b2114ae1";
  //      let result = hfe_ops::hfe_ops_restore_value(4, 5, signature, object_id, publickey);
  //      let data1 = result;
  //      debug::print(&data1);
  //      assert!(&data1 == 9);
  //  }
}


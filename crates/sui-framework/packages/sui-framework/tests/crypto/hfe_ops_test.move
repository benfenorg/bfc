#[test_only]
module sui::hfe_ops_test {
    use std::debug;
    //use std::uq32_32::le;
    //use sui::hex;
    use sui::hfe_ops;
    #[test]
    fun test_hfe_ops_add_call_rpc() {
        let result = hfe_ops::hfe_ops_minus(b"2", b"3", b"4", b"5");
     //   let data1 = result[0];
     //   let data2 = result[1];
     //   debug::print(&data1);
     //   debug::print(&data2);
       //assert!(&data1 == 7);
       // assert!(&data2 == 7);
    }

   // #[test]
   // fun test_hfe_ops_split_value() {
   //     let result = hfe_ops::hfe_ops_split_value(5);
   //     let data1 = result[0];
   //     let data2 = result[1];
   //     debug::print(&data1);
   //     debug::print(&data2);
   //     assert!(&data1 == 2);
   //     assert!(&data2 == 3);
       // assert!(&data1 == 5);
    //}

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

    // #[test]
    // fun test_split_data() {
    //
    //     let pk1 = x"4f0adab8fe9f36875f6b7f28d9679c37ab2c96224e50224b5bda5add5b1ee7bb";
    //     let result = hfe_ops::split_data(&pk1, 3, 1);
    //     assert!(result.length() != 0);
    // }
}


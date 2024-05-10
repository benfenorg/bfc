module polynet::consts {

     const DECIMALS: u8 = 9;    //both for bfc_usdt and bfc_usdc decimal

     const HUGE_U64: u64 = 10000000000000000000;   // 100 0000 0000 * decimals = 9

     const LOCAL_AMOUNT: u64 = 10000000000000000;  // 1000 0000 * decimals = 9

     public fun get_decimal():u8 {
        DECIMALS
     }

     public fun get_huge():u64 {
        HUGE_U64
     }

     public fun get_local_amount():u64 {
        LOCAL_AMOUNT
     }



}
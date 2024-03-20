module polynet::consts {

     const DECIMALS: u8 = 9;

     const HUGE_U64: u64 = 10000000000000000000;

     const LOCAL_AMOUNT: u64 = 10000000000000000;

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
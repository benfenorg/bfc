module polynet::consts {
    use sui::math;

     const DECIMALS: u8 = 9;    //both for bfc_usdt and bfc_usdc decimal

     const HUGE_U64: u64 = 10000000000000000000;   // 100 0000 0000 * decimals = 9

     const LOCAL_AMOUNT: u64 = 10000000000000000;  // 1000 0000 * decimals = 9

     const MAX_AMOUNT_PER_DAY: u64 = 100*10000*1000000000; //1 million.

     const MIN_AMOUNT_PER_TX: u64 = 5; //5

     public fun get_decimal():u8 {
        DECIMALS
     }

     public fun get_huge():u64 {
        HUGE_U64
     }

     public fun get_local_amount():u64 {
        LOCAL_AMOUNT
     }

     public fun get_max_amount_per_day():u64 {
        MAX_AMOUNT_PER_DAY
     }

     public fun get_min_amount_per_tx():u64 {
        (MIN_AMOUNT_PER_TX * math::pow(10, get_decimal()) as u64)
     }
}
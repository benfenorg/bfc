module polynet::consts {
    use sui::math;

     const DECIMALS: u8 = 9;    //both for bfc_usdt and bfc_usdc decimal

     const HUGE_U64: u64 = 8000 * 10000;   // 8000 w * decimals 

     const LOCAL_AMOUNT: u64 = 1000 * 10000;  // 1000 w for local 7000 w for treasure admin

     const MAX_AMOUNT_PER_DAY: u64 = 200 * 10000; //2 million.

     const MIN_AMOUNT_PER_TX: u64 = 5; //5

     public fun get_decimal():u8 {
        DECIMALS
     }

     public fun get_huge():u64 {
        (HUGE_U64 * math::pow(10, get_decimal()) as u64)
     }

     public fun get_local_amount():u64 {
        (LOCAL_AMOUNT * math::pow(10, get_decimal()) as u64)
     }

     public fun get_max_amount_per_day():u64 {
        (MAX_AMOUNT_PER_DAY * math::pow(10, get_decimal()) as u64)
     }

     public fun get_min_amount_per_tx():u64 {
        (MIN_AMOUNT_PER_TX * math::pow(10, get_decimal()) as u64)
     }
}
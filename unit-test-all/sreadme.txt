
 this 2 tests should be

   fn fuzz_low_rgp_low_gas_price()
   fn fuzz_high_rgp_high_gas_price()

  UNIVERSE_SIZE=10 cargo nextest run --profile ci

 //run for 290 seconds
 //cargo test --color=always --test rgp_fuzz fuzz_high_rgp_high_gas_price --no-fail-fast
 // UNIVERSE_SIZE=10 cargo nextest run --profile ci
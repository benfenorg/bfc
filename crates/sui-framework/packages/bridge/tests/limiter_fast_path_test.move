// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::limiter_fast_path_tests {
    use sui::test_scenario;
    use sui::clock;
    use sui::test_utils;
    use std::unit_test::assert_eq;

    use bridge::limiter_fast_path::{Self};

    const ETH_MAINNET: u8 = 10;
    const BUSD_ID: u64 = 5;
    const USER_ADDRESS: vector<u8> = x"4DbAD3fFb2e932AC1d085d4fdFC46596e0cf5676";

    public struct LimiterFastPathObject has key,store {
        id: UID
    }


    public fun new(ctx: &mut TxContext) : LimiterFastPathObject{
        LimiterFastPathObject {
            id: object::new(ctx),
        }
    }

    #[test]
    fun test_basic_user_limit() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj=new(ctx);
        // Create new limiter
        limiter_fast_path::new_limiter_fast_path_for_testing(&mut obj.id,ctx);
        
        // Create test clock
        let mut clock = clock::create_for_testing(ctx);
        clock.set_for_testing(1706288001377);

        // Test initial state
        assert_eq!(limiter_fast_path::get_default_limit(&obj.id), 1000_000_000_000); // 10B default limit
        limiter_fast_path::set_default_limit(&mut obj.id, 2000_000_000_000);
        assert_eq!(limiter_fast_path::get_default_limit(&obj.id), 2000_000_000_000); // 10B default limit
        assert_eq!(limiter_fast_path::get_default_time_window(&obj.id), 24); // 24 hour window
        assert_eq!(limiter_fast_path::get_enabled(&obj.id), true); // Enabled by default
        limiter_fast_path::set_enabled(&mut obj.id, false);
        assert_eq!(limiter_fast_path::get_enabled(&obj.id), false); // Enabled by default
        limiter_fast_path::set_enabled(&mut obj.id, true);
        assert_eq!(limiter_fast_path::get_enabled(&obj.id), true); 
        // Test user limit check for new user
        let user = vector::empty();
        
        let amount = 1_000_000_000;
        assert!(limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            user,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        ), 0);

        // Check remaining limit
        let remaining = limiter_fast_path::get_user_remaining_limit(&mut obj.id, user, ETH_MAINNET, BUSD_ID, &clock);
        assert_eq!(remaining, 1999_000_000_000); // 10B - 1B = 9B

        // Try exceeding limit
        let exceed_amount = 2000_000_000_000; // 11B
        assert_eq!(limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            user,
            ETH_MAINNET,
            BUSD_ID,
            exceed_amount,
            &clock,
        ), false);

        // Cleanup
        clock::destroy_for_testing(clock);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_time_window() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        
        let mut obj=new(ctx);
        // Create new limiter
        limiter_fast_path::new_limiter_fast_path_for_testing(&mut obj.id,ctx);
        let mut clock = clock::create_for_testing(ctx);
        clock.set_for_testing(1706288001377);

        // let user = @0x42;
        let amount = 500_000_000_000; // 5B

        // First transfer
        let result = limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            USER_ADDRESS,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        );
        assert_eq!(result, true);
        // Advance clock 25 hours
        clock.increment_for_testing(23 * 60 * 60 * 1000);

        // First amount should be cleared from window
        let remaining = limiter_fast_path::get_user_remaining_limit(&mut obj.id, USER_ADDRESS, ETH_MAINNET, BUSD_ID, &clock);
        assert_eq!(remaining, 500_000_000_000); // Back to full limit
        clock.increment_for_testing(1 * 60 * 60 * 1000);
        let remaining = limiter_fast_path::get_user_remaining_limit(&mut obj.id, USER_ADDRESS, ETH_MAINNET, BUSD_ID, &clock);
        assert_eq!(remaining, 1000_000_000_000); // Back to full limit

        // Cleanup
        clock::destroy_for_testing(clock);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_limit_exceeded() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        
        let mut obj=new(ctx);
        // let user = @0x42;
        // Create new limiter
        limiter_fast_path::new_limiter_fast_path_for_testing(&mut obj.id,ctx);
        let clock = clock::create_for_testing(ctx);
        
        let remaining = limiter_fast_path::get_user_remaining_limit(&mut obj.id, USER_ADDRESS, ETH_MAINNET, BUSD_ID, &clock);
        assert_eq!(remaining, 1000_000_000_000); // Back to full limit

        
        let amount = 1001_000_000_000; 
        let result = limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            USER_ADDRESS,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        );
        assert_eq!(result, false);

        let remaining = limiter_fast_path::get_user_remaining_limit(&mut obj.id, USER_ADDRESS, ETH_MAINNET, BUSD_ID, &clock);
        assert_eq!(remaining, 1000_000_000_000); // Back to full limit

        let amount = 2000_000_000_000; 
        let result = limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            USER_ADDRESS,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        );
        assert_eq!(result, false);

        let amount = 1_000_000_000; 
        let result = limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            USER_ADDRESS,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        );
        assert_eq!(result, true);

        

        clock::destroy_for_testing(clock);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_product_config() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        
        let mut obj=new(ctx);
        // let user = @0x42;
        // Create new limiter
        limiter_fast_path::new_limiter_fast_path_for_testing(&mut obj.id,ctx);
        limiter_fast_path::registry_for_testing(&mut obj.id);
        let clock = clock::create_for_testing(ctx);
        
        let remaining = limiter_fast_path::get_user_remaining_limit(&mut obj.id, USER_ADDRESS, ETH_MAINNET, BUSD_ID, &clock);
        assert_eq!(remaining, 5000_000_000_000); // Back to full limit

        
        let amount = 5001_000_000_000; 
        let result = limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            USER_ADDRESS,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        );
        assert_eq!(result, false);

        let remaining = limiter_fast_path::get_user_remaining_limit(&mut obj.id, USER_ADDRESS, ETH_MAINNET, BUSD_ID, &clock);
        assert_eq!(remaining, 5000_000_000_000); // Back to full limit

        let amount = 2000_000_000_000; 
        let result = limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            USER_ADDRESS,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        );
        assert_eq!(result, true);

        let amount = 3000_000_000_000; 
        let result = limiter_fast_path::check_and_record_user_limit(
            &mut obj.id,
            USER_ADDRESS,
            ETH_MAINNET,
            BUSD_ID,
            amount,
            &clock,
        );
        assert_eq!(result, true);

        

        clock::destroy_for_testing(clock);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }
}

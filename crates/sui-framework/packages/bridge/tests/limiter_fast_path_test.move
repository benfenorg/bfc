// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::limiter_fast_path_tests {
    use sui::test_scenario;
    use sui::clock;
    use sui::test_utils::{assert_eq, destroy};

    use bridge::limiter_fast_path::{Self, UserLimiter};

    #[test]
    fun test_basic_user_limit() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        
        // Create new limiter
        let mut limiter = limiter_fast_path::new(ctx);
        
        // Create test clock
        let mut clock = clock::create_for_testing(ctx);
        clock.set_for_testing(1706288001377);

        // Test initial state
        limiter.set_default_limit(10_000_000_000);
        assert_eq(limiter.default_limit(), 10_000_000_000); // 10B default limit
        assert_eq(limiter.default_time_window(), 24); // 24 hour window
        assert_eq(limiter.enabled(), true); // Enabled by default

        // Test user limit check for new user
        let user = @0x42;
        let amount = 1_000_000_000; // 1B
        assert!(limiter_fast_path::check_and_record_user_limit(
            &mut limiter,
            user,
            amount,
            &clock,
            ctx
        ), 0);

        // Check remaining limit
        let remaining = limiter_fast_path::get_user_remaining_limit(&limiter, user, &clock);
        assert_eq(remaining, 9_000_000_000); // 10B - 1B = 9B

        // Try exceeding limit
        let exceed_amount = 11_000_000_000; // 11B
        assert!(!limiter_fast_path::check_and_record_user_limit(
            &mut limiter,
            user,
            exceed_amount,
            &clock,
            ctx
        ), 0);

        // Cleanup
        clock::destroy_for_testing(clock);
        destroy(limiter);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_time_window() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        
        let mut limiter = limiter_fast_path::new(ctx);
        let mut clock = clock::create_for_testing(ctx);
        clock.set_for_testing(1706288001377);

        let user = @0x42;
        let amount = 5_000_000_000; // 5B

        // First transfer
        assert!(limiter_fast_path::check_and_record_user_limit(
            &mut limiter,
            user,
            amount,
            &clock,
            ctx
        ), 0);

        // Advance clock 25 hours
        clock.increment_for_testing(25 * 60 * 60 * 1000);

        // First amount should be cleared from window
        let remaining = limiter_fast_path::get_user_remaining_limit(&limiter, user, &clock);
        assert_eq(remaining, 10_000_000_000); // Back to full limit

        // Cleanup
        clock::destroy_for_testing(clock);
        destroy(limiter);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = limiter_fast_path::EUserLimitExceeded)]
    fun test_limit_exceeded() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        
        let mut limiter = limiter_fast_path::new(ctx);
        let clock = clock::create_for_testing(ctx);
        
        let user = @0x42;
        let amount = 15_000_000_000; // 15B > 10B limit

        limiter_fast_path::check_and_record_user_limit(
            &mut limiter,
            user,
            amount,
            &clock,
            ctx
        );

        clock::destroy_for_testing(clock);
        destroy(limiter);
        test_scenario::end(scenario);
    }
}

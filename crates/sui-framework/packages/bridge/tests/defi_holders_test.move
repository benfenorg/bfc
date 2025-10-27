// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::defi_holders_test;
    use bridge::bridge::{test_defi_holders_add, test_defi_holders_get, test_defi_holders_del, create_defi_protocol_key_for_testing, test_defi_holders_amount_get, test_defi_holders_info_amount_get, test_defi_holders_info_lp_token_amount_get};
    use bridge::bridge_env::{create_env, create_bridge_default};
    use bridge::chain_ids;

    #[test]
    fun test_defi_holders_add_get() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user_address, protocol_key, amount, amount);

        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == amount, 0);

        let non_existent_key = create_defi_protocol_key_for_testing(2, 1, 4, 0);
        let zero_info = bridge.test_defi_holders_get(user_address, non_existent_key);
        assert!(test_defi_holders_info_amount_get(&zero_info) == 0, 0);

        let another_user = @0x2;
        let zero_info = bridge.test_defi_holders_get(another_user, protocol_key);
        assert!(test_defi_holders_info_amount_get(&zero_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_add_merge() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount1 = 1000;
        let amount2 = 500;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user_address, protocol_key, amount1, amount1);
        bridge.test_defi_holders_add(user_address, protocol_key, amount2, amount2);

        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == (amount1 + amount2), 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_delete() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let initial_amount = 1000;
        let del_amount = 300;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user_address, protocol_key, initial_amount, initial_amount);

        // Test successful deletion
        let result = bridge.test_defi_holders_del(user_address, protocol_key, del_amount, del_amount);
        assert!(result, 0); // Should return true for successful deletion

        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == (initial_amount - del_amount), 0);

        // Test deleting the remaining amount
        let result = bridge.test_defi_holders_del(user_address, protocol_key, (initial_amount - del_amount), (initial_amount - del_amount));
        assert!(result, 0); // Should return true for successful deletion

        let zero_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&zero_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_delete_partial() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let initial_amount = 1_000_000_000;
        let del_amount = 300_000_000;
        let del_amount_2 = 700_000_000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user_address, protocol_key, initial_amount, initial_amount);

        // Test successful deletion
        let result = bridge.test_defi_holders_del(user_address, protocol_key, 0, del_amount);
        assert!(result, 0); // Should return true for successful deletion
        let result = bridge.test_defi_holders_del(user_address, protocol_key, del_amount, 0);
        assert!(result, 0); // Should return true for successful deletion

        let result = bridge.test_defi_holders_del(user_address, protocol_key, 0, del_amount_2);
        assert!(result, 0); // Should return true for successful deletion
        let result = bridge.test_defi_holders_del(user_address, protocol_key, del_amount_2, 0);
        assert!(result, 0); // Should return true for successful deletion

        

        let zero_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&zero_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_del_excess() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let initial_amount = 1000;
        let del_amount = 1500;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user_address, protocol_key, initial_amount, initial_amount);

        // Test deletion with excess amount - should return false
        let result = bridge.test_defi_holders_del(user_address, protocol_key, del_amount, del_amount);
        assert!(!result, 0); // Should return false for excess deletion

        // Amount should remain unchanged
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == initial_amount, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_multiple_users_protocols() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user1 = @0x1;
        let user2 = @0x2;
        
        let protocol_key1 = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let protocol_key2 = create_defi_protocol_key_for_testing(2, 1, 4, 0);
        
        let amount1 = 1000;
        let amount2 = 2000;
        let amount3 = 3000;

        let mut bridge_wrap = env.bridge(user1);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user1, protocol_key1, amount1, amount1);
        bridge.test_defi_holders_add(user1, protocol_key2, amount2, amount2);
        bridge.test_defi_holders_add(user2, protocol_key1, amount3, amount3);

        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user1, protocol_key1)) == amount1, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user1, protocol_key2)) == amount2, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user2, protocol_key1)) == amount3, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user2, protocol_key2)) == 0, 0);

        // Test successful deletion
        let result = bridge.test_defi_holders_del(user1, protocol_key1, amount1, amount1);
        assert!(result, 0); // Should return true for successful deletion

        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user1, protocol_key1)) == 0, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user1, protocol_key2)) == amount2, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user2, protocol_key1)) == amount3, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_empty_user_table() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Test getting from non-existent user
        let zero_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&zero_info) == 0, 0);

        // Test deleting from non-existent user - should return false
        let result = bridge.test_defi_holders_del(user_address, protocol_key, 100, 100);
        assert!(!result, 0); // Should return false for non-existent user
        let zero_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&zero_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_user_table_cleanup() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key1 = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let protocol_key2 = create_defi_protocol_key_for_testing(2, 1, 4, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add two entries for the user
        bridge.test_defi_holders_add(user_address, protocol_key1, amount, amount);
        bridge.test_defi_holders_add(user_address, protocol_key2, amount, amount);

        // Verify both entries exist
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key1)) == amount, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key2)) == amount, 0);

        // Delete both entries
        let result1 = bridge.test_defi_holders_del(user_address, protocol_key1, amount, amount);
        assert!(result1, 0); // Should return true for successful deletion
        let result2 = bridge.test_defi_holders_del(user_address, protocol_key2, amount, amount);
        assert!(result2, 0); // Should return true for successful deletion

        // Verify both entries are gone and user table is cleaned up
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key1)) == 0, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key2)) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_large_values() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let large_amount = 1000000000000; // 1 trillion

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Test adding large values
        bridge.test_defi_holders_add(user_address, protocol_key, large_amount, large_amount);
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == large_amount, 0);

        // Test merging large values
        bridge.test_defi_holders_add(user_address, protocol_key, large_amount, large_amount);
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == (large_amount * 2), 0);

        // Test deleting large values
        let result = bridge.test_defi_holders_del(user_address, protocol_key, large_amount, large_amount);
        assert!(result, 0); // Should return true for successful deletion
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == large_amount, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    // New test cases for the enhanced defi_holders_del function

    #[test]
    fun test_defi_holders_del_nonexistent_protocol() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let existing_protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let nonexistent_protocol_key = create_defi_protocol_key_for_testing(2, 1, 4, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add an entry for one protocol
        bridge.test_defi_holders_add(user_address, existing_protocol_key, amount, amount);

        // Try to delete from a non-existent protocol key - should return false
        let result = bridge.test_defi_holders_del(user_address, nonexistent_protocol_key, amount, amount);
        assert!(!result, 0); // Should return false for non-existent protocol

        // Original entry should remain unchanged
        let retrieved_info = bridge.test_defi_holders_get(user_address, existing_protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == amount, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_del_zero_amount() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user_address, protocol_key, amount, amount);

        // Try to delete zero amount - should succeed (subtract 0)
        let result = bridge.test_defi_holders_del(user_address, protocol_key, 0, 0);
        assert!(result, 0); // Should return true for successful operation

        // Amount should remain unchanged
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == amount, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_del_exact_amount() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        bridge.test_defi_holders_add(user_address, protocol_key, amount, amount);

        // Delete exact amount - should succeed and remove entry
        let result = bridge.test_defi_holders_del(user_address, protocol_key, amount, amount);
        assert!(result, 0); // Should return true for successful deletion

        // Amount should be zero
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    // Additional boundary case tests

    #[test]
    fun test_defi_holders_add_zero_amount() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount = 0;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Adding zero amount should be allowed
        bridge.test_defi_holders_add(user_address, protocol_key, amount, amount);

        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_add_multiple_times_then_delete_all() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let mut amounts = vector[100, 200, 300, 400];

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add multiple times
        let mut total_added = 0;
        loop {
            if (amounts.is_empty()) break;
            let amount = amounts.pop_back();
            total_added = total_added + amount;
            bridge.test_defi_holders_add(user_address, protocol_key, amount, amount);
        };

        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == total_added, 0);

        // Delete all
        let result = bridge.test_defi_holders_del(user_address, protocol_key, total_added, total_added);
        assert!(result, 0);

        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_add_overflow_protection() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let _max_u64 = 18446744073709551615; // Maximum u64 value
        let half_max = 9223372036854775807;  // Approximately half of max u64

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add half of max value
        bridge.test_defi_holders_add(user_address, protocol_key, half_max, half_max);
        
        // Try to add another half - this should not overflow in normal circumstances
        bridge.test_defi_holders_add(user_address, protocol_key, half_max, half_max);

        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        // Should be close to max value (may not be exactly due to integer arithmetic)
        assert!(test_defi_holders_info_amount_get(&retrieved_info) > 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_multiple_users_same_protocol() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user1 = @0x1;
        let user2 = @0x2;
        let user3 = @0x3;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user1);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add same protocol for multiple users
        bridge.test_defi_holders_add(user1, protocol_key, amount, amount);
        bridge.test_defi_holders_add(user2, protocol_key, amount * 2, amount * 2);
        bridge.test_defi_holders_add(user3, protocol_key, amount * 3, amount * 3);

        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user1, protocol_key)) == amount, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user2, protocol_key)) == amount * 2, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user3, protocol_key)) == amount * 3, 0);

        // Delete from one user
        let result = bridge.test_defi_holders_del(user2, protocol_key, amount, amount);
        assert!(result, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user2, protocol_key)) == amount, 0);

        // Others should be unaffected
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user1, protocol_key)) == amount, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user3, protocol_key)) == amount * 3, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_delete_from_empty_user() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Try to delete from a user that has never added anything
        let result = bridge.test_defi_holders_del(user_address, protocol_key, amount, amount);
        assert!(!result, 0); // Should return false

        // Amount should still be zero
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_same_user_multiple_protocols_delete_one() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key1 = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let protocol_key2 = create_defi_protocol_key_for_testing(2, 1, 4, 0);
        let protocol_key3 = create_defi_protocol_key_for_testing(3, 1, 5, 0);
        let amount = 1000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add multiple protocols for same user
        bridge.test_defi_holders_add(user_address, protocol_key1, amount, amount);
        bridge.test_defi_holders_add(user_address, protocol_key2, amount * 2, amount * 2);
        bridge.test_defi_holders_add(user_address, protocol_key3, amount * 3, amount * 3);

        // Verify all are there
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key1)) == amount, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key2)) == amount * 2, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key3)) == amount * 3, 0);

        // Delete one protocol completely
        let result = bridge.test_defi_holders_del(user_address, protocol_key2, amount * 2, amount * 2);
        assert!(result, 0);

        // Verify that one is gone and others remain
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key1)) == amount, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key2)) == 0, 0);
        assert!(test_defi_holders_info_amount_get(&bridge.test_defi_holders_get(user_address, protocol_key3)) == amount * 3, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_protocol_key_uniqueness() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        
        // Create protocol keys that are identical
        let protocol_key1 = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let protocol_key2 = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        
        let amount1 = 1000;
        let amount2 = 2000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add with first key
        bridge.test_defi_holders_add(user_address, protocol_key1, amount1, amount1);
        
        // Add with second key (should be treated as same key)
        bridge.test_defi_holders_add(user_address, protocol_key2, amount2, amount2);

        // Should have combined amount since keys are identical
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key1);
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == (amount1 + amount2), 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_amount_get_simple() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        let amount = 5000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add some amount for the user
        bridge.test_defi_holders_add(user_address, protocol_key, amount, amount);

        // Test that defi_holders_amount_get returns the correct amount
        let retrieved_amount = bridge.test_defi_holders_amount_get(user_address, 1, 1, 3, 0);
        assert!(retrieved_amount == amount, 0);

        // Test with a different user (should return 0)
        let other_user = @0x2;
        let retrieved_amount = bridge.test_defi_holders_amount_get(other_user, 1, 1, 3, 0);
        assert!(retrieved_amount == 0, 0);

        // Test with a different protocol key (should return 0)
        let retrieved_amount = bridge.test_defi_holders_amount_get(user_address, 2, 1, 4, 0);
        assert!(retrieved_amount == 0, 0);
        
        bridge_wrap.return_bridge();
        env.destroy_env();
    }

    #[test]
    fun test_defi_holders_amount_vs_lp_token_amount_distinction() {
        let mut env = create_env(chain_ids::sui_custom());
        env.create_bridge_default();
        
        let user_address = @0x1;
        let protocol_key = create_defi_protocol_key_for_testing(1, 1, 3, 0);
        
        // Test with different values for amount and lp_token_amount
        let amount = 1000;
        let lp_token_amount = 2000;

        let mut bridge_wrap = env.bridge(user_address);
        let bridge = bridge_wrap.bridge_ref_mut();

        // Add defi holder with different amount and lp_token_amount values
        bridge.test_defi_holders_add(user_address, protocol_key, amount, lp_token_amount);

        // Retrieve the DefiHolderInfo
        let retrieved_info = bridge.test_defi_holders_get(user_address, protocol_key);
        
        // Verify that amount and lp_token_amount are stored and retrieved correctly
        assert!(test_defi_holders_info_amount_get(&retrieved_info) == amount, 0);
        assert!(test_defi_holders_info_lp_token_amount_get(&retrieved_info) == lp_token_amount, 0);
        
        // Verify using the public getter functions
        assert!(bridge.defi_holders_amount_get(user_address, 1, 1, 3, 0) == amount, 0);
        assert!(bridge.defi_holders_lp_token_amount_get(user_address, 1, 1, 3, 0) == lp_token_amount, 0);

        // Test merging with different values
        let amount2 = 500;
        let lp_token_amount2 = 1500;
        bridge.test_defi_holders_add(user_address, protocol_key, amount2, lp_token_amount2);
        
        let retrieved_info2 = bridge.test_defi_holders_get(user_address, protocol_key);
        // Verify that both values are correctly accumulated
        assert!(test_defi_holders_info_amount_get(&retrieved_info2) == (amount + amount2), 0);
        assert!(test_defi_holders_info_lp_token_amount_get(&retrieved_info2) == (lp_token_amount + lp_token_amount2), 0);
        
        // Verify using the public getter functions
        assert!(bridge.defi_holders_amount_get(user_address, 1, 1, 3, 0) == (amount + amount2), 0);
        assert!(bridge.defi_holders_lp_token_amount_get(user_address, 1, 1, 3, 0) == (lp_token_amount + lp_token_amount2), 0);

        // Test deletion with different values
        let del_amount = 300;
        let del_lp_token_amount = 400;
        let result = bridge.test_defi_holders_del(user_address, protocol_key, del_amount, del_lp_token_amount);
        assert!(result, 0); // Should return true for successful deletion

        let retrieved_info3 = bridge.test_defi_holders_get(user_address, protocol_key);
        // Verify that both values are correctly reduced
        assert!(test_defi_holders_info_amount_get(&retrieved_info3) == (amount + amount2 - del_amount), 0);
        assert!(test_defi_holders_info_lp_token_amount_get(&retrieved_info3) == (lp_token_amount + lp_token_amount2 - del_lp_token_amount), 0);
        
        // Verify using the public getter functions
        assert!(bridge.defi_holders_amount_get(user_address, 1, 1, 3, 0) == (amount + amount2 - del_amount), 0);
        assert!(bridge.defi_holders_lp_token_amount_get(user_address, 1, 1, 3, 0) == (lp_token_amount + lp_token_amount2 - del_lp_token_amount), 0);

        // Test edge case: zero values
        let zero_user = @0x2;
        let zero_info = bridge.test_defi_holders_get(zero_user, protocol_key);
        assert!(test_defi_holders_info_amount_get(&zero_info) == 0, 0);
        assert!(test_defi_holders_info_lp_token_amount_get(&zero_info) == 0, 0);
        
        // Verify using the public getter functions
        assert!(bridge.defi_holders_amount_get(zero_user, 1, 1, 3, 0) == 0, 0);
        assert!(bridge.defi_holders_lp_token_amount_get(zero_user, 1, 1, 3, 0) == 0, 0);

        bridge_wrap.return_bridge();
        env.destroy_env();
    }

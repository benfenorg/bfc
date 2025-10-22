// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::defi_protocol_validation_test {
    use sui::test_scenario;
    use bridge::defi_protocols;

    #[test]
    fun test_is_valid_protocol_function() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = scenario.ctx();
        
        // Create a dummy object to get a UID for testing
        let dummy_object = object::new(ctx);
        
        // Test when no DefiProtocolConfig is registered, should return false
        assert!(!defi_protocols::is_valid_protocol(&dummy_object, 1, 1, 1, 1), 0);
        
        object::delete(dummy_object);
        scenario.end();
    }
}
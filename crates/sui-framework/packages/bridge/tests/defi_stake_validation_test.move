// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::defi_stake_validation_test {
    use bridge::bridge_env::{Self, create_env, create_bridge};
    use bridge::bridge;
    use bridge::chain_ids;
    use bfc_system::bfc_system::BfcSystemState;
    use bfc_system::bfc_system;
    use bfc_system::bfc_system_tests::public_setup;
    use bfc_system::bfc_system_state_inner::BfcSystemModifyCap;
    use bfc_system::busd::BUSD;
    use sui::test_scenario;
    use sui::event;

    const MINT_BUSD_RIGHT_KEY: vector<u8> = b"MINT-BUSD-right_key";

    #[test]
    #[expected_failure(abort_code = 63)]
    fun test_defi_stake_without_protocol_config() {
        // This test verifies that defi_stake properly validates protocol configuration
        // and throws EDefiProtocolConfigNotFound when the protocol is not configured
        
        let mut env = create_env(chain_ids::sui_custom());
        
        // Setup validators and create bridge WITHOUT calling migrate() to setup defi protocols
        let validators = vector[
            bridge_env::create_validator(
                @0xAAAA,
                100,
                &b"1234567890_1234567890_1234567890",
            ),
            bridge_env::create_validator(
                @0xBBBB,
                100,
                &b"234567890_1234567890_1234567890_",
            ),
            bridge_env::create_validator(
                @0xCCCC,
                100,
                &b"34567890_1234567890_1234567890_1",
            ),
        ];
        env.setup_validators(validators);
        
        let sender = @0x0;
        env.create_bridge(sender);
        env.register_committee();
        env.init_committee(sender);
        // Do NOT call add_tokenlist or migrate - this sets up defi protocols
        // env.add_tokenlist(sender);
        env.setup_treasury(sender);
        
        
        // Get BUSD coin for testing
        let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
        let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
        let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
        let amount = 1000u64;

        scenario.next_tx(@0x0);
        let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

        // Call defi_stake function
        let mut bridge = env.bridge(@0x0);
        let ctx = env.ctx();

        let target_chain = chain_ids::eth_mainnet();
        let protocol_type = 1u64;
        let protocol_version = 3u64;
        let protocol_token_id = 3u64; // USDC

        // This should fail with EDefiProtocolConfigNotFound error
        bridge.bridge_ref_mut().defi_stake<BUSD>(
            &mut bfc_system_state,
            target_chain,
            coin,
            protocol_type,
            protocol_version,
            protocol_token_id,
            ctx,
        );

        // Cleanup - this code should not be reached due to expected failure
        bridge.return_bridge();
        sui::test_scenario::return_shared(bfc_system_state);
        sui::test_scenario::return_to_sender(&scenario, cap);
        sui::test_scenario::end(scenario);
        env.destroy_env();
    }

    // Helper function to set up tokenlist without calling migrate()
    fun setup_tokenlist_without_migrate(env: &mut bridge_env::BridgeEnv, sender: address) {
        env.scenario().next_tx(sender);
        let mut bridge = env.scenario().take_shared<bridge::Bridge>();
        let ctx = env.scenario().ctx();
        bridge.init_token_list(ctx);
        // We do NOT call bridge.migrate(ctx) here to avoid setting up defi protocols
        test_scenario::return_shared(bridge);
    }

    #[test]
    fun test_defi_stake_emits_event_with_positive_amounts() {
        let mut env = create_env(chain_ids::sui_custom());
        let validators = vector[
            bridge_env::create_validator(
                @0xAAAA,
                100,
                &b"1234567890_1234567890_1234567890",
            ),
            bridge_env::create_validator(
                @0xBBBB,
                100,
                &b"234567890_1234567890_1234567890_",
            ),
            bridge_env::create_validator(
                @0xCCCC,
                100,
                &b"34567890_1234567890_1234567890_1",
            ),
        ];
        env.setup_validators(validators);
        
        let sender = @0x0;
        env.create_bridge(sender);
        env.register_committee();
        env.init_committee(sender);
        env.add_tokenlist(sender);
        env.setup_treasury(sender);
        
        let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
        let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
        let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
        let amount = 1000u64;

        scenario.next_tx(@0x0);
        let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

        let mut bridge = env.bridge(@0x0);
        let ctx = env.ctx();

        let target_chain = chain_ids::eth_mainnet();
        let protocol_type = 1u64;
        let protocol_version = 3u64;
        let protocol_token_id = 3u64; 

        bridge.bridge_ref_mut().defi_stake<BUSD>(
            &mut bfc_system_state,
            target_chain,
            coin,
            protocol_type,
            protocol_version,
            protocol_token_id,
            ctx,
        );

        let events = event::events_by_type<bridge::DefiTransferOutEvent>();
        assert!(events.length() == 1, 0);
        let event = &events[0];
        assert!(bridge::get_defi_transfer_out_event_amount_before_fee(event) > 0, 0);
        assert!(bridge::get_defi_transfer_out_event_amount_after_fee(event) > 0, 0);

        bridge.return_bridge();
        sui::test_scenario::return_shared(bfc_system_state);
        sui::test_scenario::return_to_sender(&scenario, cap);
        sui::test_scenario::end(scenario);
        env.destroy_env();
    }
}
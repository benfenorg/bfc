// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(implicit_const_copy,unused_mut_ref)]
module sui_system::validator_tests {
    use sui::test_scenario;
    use sui::test_utils;
    use sui::url;
    use std::string::{Self};
    use sui_system::validator::{Self, Validator, rate_vec_map};
    use std::ascii;
    use sui_system::staking_pool::StakedBfc;
    use sui::coin::{Self, Coin};
    use sui::balance;
    use bfc_system::busd::BUSD;
    use sui::bag;
    use sui_system::stable_pool::StakedStable;
    use sui_system::test_runner;
    use sui_system::validator_builder;
    use std::unit_test::assert_eq;
    //use std::debug::print;

    const VALID_NET_PUBKEY: vector<u8> = vector[171, 2, 39, 3, 139, 105, 166, 171, 153, 151, 102, 197, 151, 186, 140, 116, 114, 90, 213, 225, 20, 167, 60, 69, 203, 12, 180, 198, 9, 217, 117, 38];

    const VALID_WORKER_PUBKEY: vector<u8> = vector[171, 3, 39, 3, 139, 105, 166, 171, 153, 151, 102, 197, 151, 186, 140, 116, 114, 90, 213, 225, 20, 167, 60, 69, 203, 12, 180, 198, 9, 217, 117, 38];

    // A valid proof of possession must be generated using the same account address and protocol public key.
    // If either VALID_ADDRESS or VALID_PUBKEY changed, PoP must be regenerated using [fn test_proof_of_possession].
    const VALID_ADDRESS: address = @0xaf76afe6f866d8426d2be85d6ef0b11f871a251d043b2f11e15563bf418f5a5a;
    const VALID_PUBKEY: vector<u8> = x"99f25ef61f8032b914636460982c5cc6f134ef1ddae76657f2cbfec1ebfc8d097374080df6fcf0dcb8bc4b0d8e0af5d80ebbff2b4c599f54f42d6312dfc314276078c1cc347ebbbec5198be258513f386b930d02c2749a803e2330955ebd1a10";
    const PROOF_OF_POSSESSION: vector<u8> = x"b01cc86f421beca7ab4cfca87c0799c4d038c199dd399fbec1924d4d4367866dba9e84d514710b91feb65316e4ceef43";

    const VALID_NET_ADDR: vector<u8> = b"/ip4/127.0.0.1/tcp/80";
    const VALID_P2P_ADDR: vector<u8> = b"/ip4/127.0.0.1/udp/80";
    const VALID_CONSENSUS_ADDR: vector<u8> = b"/ip4/127.0.0.1/udp/80";
    const VALID_WORKER_ADDR: vector<u8> = b"/ip4/127.0.0.1/udp/80";

    //const TOO_LONG_257_BYTES: vector<u8> = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test_only]
    fun get_test_validator(ctx: &mut TxContext): Validator {
        let init_stake = coin::into_balance(coin::mint_for_testing(1_000_000_000, ctx));
        let mut validator = validator::new(
        VALID_ADDRESS,
        VALID_PUBKEY,
        VALID_NET_PUBKEY,
        VALID_WORKER_PUBKEY,
        PROOF_OF_POSSESSION,
        b"Validator1",
        b"Validator1",
        b"Validator1",
        b"Validator1",
        VALID_NET_ADDR,
        VALID_P2P_ADDR,
        VALID_CONSENSUS_ADDR,
        VALID_WORKER_ADDR,
        1,
        0,
        ctx
        );

        validator::request_add_stake_at_genesis(
        &mut validator,
        init_stake,
        VALID_ADDRESS,
        ctx
        );

        validator.activate(0);
        validator::activate_stable(&mut validator, 0);

        validator
    }

    const DEFAULT_STAKE: u64 = 10;

    #[test]
    fun test_validator_with_stable() {
        let sender = VALID_ADDRESS;
        let mut scenario_val = test_scenario::begin(sender);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        let mut validator = get_test_validator(ctx);
        //add stable stake
        let new_stake = coin::into_balance(coin::mint_for_testing(30_000_000_000, ctx));
        let staked = validator::request_add_stable_stake<BUSD>(&mut validator, new_stake, sender, ctx);

        //process pending stake
        validator::process_pending_stable_stakes_and_withdraws<BUSD>(&mut validator, ctx);
        assert!( validator::stable_stake_amount<BUSD>(&validator) == 30_000_000_000, 0);
        let rate_map = rate_vec_map();
        assert!( validator::total_stake_with_all_stable(&validator, rate_map) == 31_000_000_000, 0);

        test_utils::destroy(staked);
        test_utils::destroy(validator);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun validator_owner_flow() {
        let initial_stake = DEFAULT_STAKE * 1_000_000_000;
        let mut runner = test_runner::new().build();
        runner.set_sender(@2);

        let validator = validator_builder::new()
        .sui_address(@2)
        .initial_stake(DEFAULT_STAKE)
        .build(runner.ctx());

        let pool_id = validator.staking_pool_id();

        assert_eq!(validator.total_stake(), initial_stake);
        assert_eq!(validator.sui_address(), @2);
        test_runner::destroy(validator);

        runner.owned_tx!<StakedBfc>(|stake| {
        assert_eq!(stake.amount(), initial_stake);
        assert_eq!(stake.pool_id(), pool_id);
        assert_eq!(stake.stake_activation_epoch(), 0);
        runner.keep(stake);
        });

        runner.finish();
    }

    #[test]
    // Scenario:
    // 1. Create a validator with initial stake
    // 2. Add extra stake, check pending stake amount
    // 3. Withdraw initial stake, check pending stake amount and pending withdraw amount
    // 4. Trigger state change and pending processing
    // 5. Check that pending values have been processed
    fun pending_validator_flow() {
        let initial_stake = DEFAULT_STAKE * 1_000_000_000;
        let added_stake = 30_000_000_000;
        let mut runner = test_runner::new().build();
        runner.set_sender(@2);

        let mut validator = validator_builder::new()
        .sui_address(@2)
        .initial_stake(DEFAULT_STAKE)
        .is_active_at_genesis(true)
        .build(runner.ctx());

        // add extra stake, but don't send it to the inventory just yet
        let extra_stake = validator.request_add_stake(test_runner::mint(30), @2, runner.ctx());

        assert_eq!(validator.total_stake(), initial_stake);
        assert_eq!(validator.pending_stake_amount(), added_stake);

        // take initial stake out of inventory
        runner.owned_tx!<StakedBfc>(|staked_sui| {
        let withdrawn_balance = validator
        .request_withdraw_stake(staked_sui, runner.ctx())
        .destroy_for_testing();

        assert_eq!(withdrawn_balance, initial_stake);
        assert_eq!(validator.total_stake(), initial_stake);
        assert_eq!(validator.pending_stake_amount(), added_stake);
        assert_eq!(validator.pending_stake_withdraw_amount(), initial_stake);

        // trigger the state change and pending processing
        validator.deposit_stake_rewards(balance::zero(), &rate_vec_map());
        validator.process_pending_stakes_and_withdraws(runner.ctx());

        assert_eq!(validator.total_stake(), added_stake);
        assert_eq!(validator.pending_stake_amount(), 0);
        assert_eq!(validator.pending_stake_withdraw_amount(), 0);
        });

        runner.finish();
        test_runner::destroy(validator);
        test_runner::destroy(extra_stake);
    }

    #[test]
    fun test_pending_validator_flow_with_stable() {
        let sender = VALID_ADDRESS;
        let mut scenario_val = test_scenario::begin(sender);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        let mut validator = get_test_validator(ctx);
        test_scenario::next_tx(scenario, sender);
        {
        let ctx = test_scenario::ctx(scenario);
        let new_stake = coin::into_balance<BUSD>(coin::mint_for_testing<BUSD>(30_000_000_000, ctx));
        let stake = validator::request_add_stable_stake<BUSD>(&mut validator, new_stake, sender, ctx);
        transfer::public_transfer(stake, sender);

        assert!(validator::total_stake(&validator) == 1_000_000_000, 0);
        assert_eq!(validator::pending_stake_stable_amount<BUSD>(&validator), 30_000_000_000);
        };

        test_scenario::next_tx(scenario, sender);
        {
        let coin_ids = test_scenario::ids_for_sender<StakedStable<BUSD>>(scenario);
        let stake = test_scenario::take_from_sender_by_id<StakedStable<BUSD>>(scenario, *vector::borrow(&coin_ids, 0));
        let ctx = test_scenario::ctx(scenario);
        let (withdrawn_balance, bfc) = validator::request_withdraw_stable_stake(&mut validator, stake, 1_000_000_000, ctx);
        transfer::public_transfer(coin::from_balance<BUSD>(withdrawn_balance, ctx), sender);
        transfer::public_transfer(coin::from_balance(bfc, ctx), sender);

        assert!(validator::total_stake(&validator) == 1_000_000_000, 0);
        assert_eq!(validator::pending_stake_stable_amount<BUSD>(&validator), 30_000_000_000);

        validator::deposit_stake_rewards(&mut validator, balance::zero(), &rate_vec_map());

        // Calling `process_pending_stakes_and_withdraws` will withdraw the coin and transfer to sender.
        validator::process_pending_stakes_and_withdraws(&mut validator, ctx);

        assert_eq!(validator::total_stake_with_all_stable(&validator, rate_vec_map()), 1_000_000_000);
        assert!(validator::pending_stake_amount(&validator) == 0, 0);
        assert!(validator::pending_stake_withdraw_amount(&validator) == 0, 0);
        };

        test_scenario::next_tx(scenario, sender);
        {
        let coin_ids = test_scenario::ids_for_sender<Coin<BUSD>>(scenario);
        let withdraw = test_scenario::take_from_sender_by_id<Coin<BUSD>>(scenario, *vector::borrow(&coin_ids, 0));
        assert_eq!(coin::value(&withdraw), 30_000_000_000);
        test_scenario::return_to_sender(scenario, withdraw);
        };

        test_utils::destroy(validator);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_metadata() {
        let mut scenario_val = test_scenario::begin(VALID_ADDRESS);
        let ctx = test_scenario::ctx(&mut scenario_val);
        let metadata = validator::new_metadata(
        VALID_ADDRESS,
        VALID_PUBKEY,
        VALID_NET_PUBKEY,
        VALID_WORKER_PUBKEY,
        PROOF_OF_POSSESSION,
        string::from_ascii(ascii::string(b"Validator1")),
        string::from_ascii(ascii::string(b"Validator1")),
        url::new_unsafe_from_bytes(b"image_url1"),
        url::new_unsafe_from_bytes(b"project_url1"),
        string::from_ascii(ascii::string(VALID_NET_ADDR)),
        string::from_ascii(ascii::string(VALID_P2P_ADDR)),
        string::from_ascii(ascii::string(VALID_CONSENSUS_ADDR)),
        string::from_ascii(ascii::string(VALID_WORKER_ADDR)),
        bag::new(ctx),
        );

        validator::validate_metadata(&metadata);
        test_utils::destroy(metadata);
        test_scenario::end(scenario_val);
    }

    #[test]
    #[expected_failure(abort_code = validator::EMetadataInvalidPubkey)]
    fun test_metadata_invalid_pubkey() {
        let mut scenario_val = test_scenario::begin(VALID_ADDRESS);
        let ctx = test_scenario::ctx(&mut scenario_val);
        let metadata = validator::new_metadata(
        VALID_ADDRESS,
        vector[42],
        VALID_NET_PUBKEY,
        VALID_WORKER_PUBKEY,
        PROOF_OF_POSSESSION,
        string::from_ascii(ascii::string(b"Validator1")),
        string::from_ascii(ascii::string(b"Validator1")),
        url::new_unsafe_from_bytes(b"image_url1"),
        url::new_unsafe_from_bytes(b"project_url1"),
        string::from_ascii(ascii::string(VALID_NET_ADDR)),
        string::from_ascii(ascii::string(VALID_P2P_ADDR)),
        string::from_ascii(ascii::string(VALID_CONSENSUS_ADDR)),
        string::from_ascii(ascii::string(VALID_WORKER_ADDR)),
        bag::new(ctx),
        );

        validator::validate_metadata(&metadata);
        test_utils::destroy(metadata);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun metadata() {
        let ctx = &mut tx_context::dummy();
        let metadata = validator_builder::preset().build_metadata(ctx);
        metadata.validate();
        test_utils::destroy(metadata);
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidPubkey)]
    fun metadata_invalid_pubkey() {
        let ctx = &mut tx_context::dummy();
        let metadata = validator_builder::preset()
            .protocol_pubkey_bytes(b"incorrect")
            .build_metadata(ctx);

        metadata.validate();

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidNetPubkey)]
    fun metadata_invalid_net_pubkey() {
        let ctx = &mut tx_context::dummy();
        let metadata = validator_builder::preset()
            .network_pubkey_bytes(b"incorrect")
            .build_metadata(ctx);

        metadata.validate();

        abort
    }

    #[test]
    #[expected_failure(abort_code = validator::EMetadataInvalidWorkerPubkey)]
    fun test_metadata_invalid_worker_pubkey() {
        let mut scenario_val = test_scenario::begin(VALID_ADDRESS);
        let ctx = scenario_val.ctx();
        let metadata = validator::new_metadata(
        VALID_ADDRESS,
        VALID_PUBKEY,
        VALID_NET_PUBKEY,
        vector[42],
        PROOF_OF_POSSESSION,
        b"Validator1".to_string(),
        b"Validator1".to_string(),
        url::new_unsafe_from_bytes(b"image_url1"),
        url::new_unsafe_from_bytes(b"project_url1"),
        VALID_NET_ADDR.to_string(),
        VALID_P2P_ADDR.to_string(),
        VALID_CONSENSUS_ADDR.to_string(),
        VALID_WORKER_ADDR.to_string(),
        bag::new(ctx),
        );

        validator::validate_metadata(&metadata);
        test_utils::destroy(metadata);
        scenario_val.end();
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidNetAddr)]
    fun metadata_invalid_net_addr() {
        let ctx = &mut tx_context::dummy();
        let metadata = validator_builder::preset().net_address(b"incorrect").build_metadata(ctx);

        metadata.validate();

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidP2pAddr)]
    fun metadata_invalid_p2p_addr() {
        let ctx = &mut tx_context::dummy();
        let metadata = validator_builder::preset().p2p_address(b"incorrect").build_metadata(ctx);

        metadata.validate();

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidPrimaryAddr)]
    fun metadata_invalid_consensus_addr() {
        let ctx = &mut tx_context::dummy();
        let metadata = validator_builder::preset().primary_address(b"incorrect").build_metadata(ctx);

        metadata.validate();

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidWorkerAddr)]
    fun metadata_invalid_worker_addr() {
        let ctx = &mut tx_context::dummy();
        let metadata = validator_builder::preset().worker_address(b"incorrect").build_metadata(ctx);

        metadata.validate();

        abort
    }

    #[test, allow(implicit_const_copy)]
    fun validator_update_metadata_ok() {
        let new_protocol_pub_key =
            x"96d19c53f1bee2158c3fcfb5bb2f06d3a8237667529d2d8f0fbb22fe5c3b3e64748420b4103674490476d98530d063271222d2a59b0f7932909cc455a30f00c69380e6885375e94243f7468e9563aad29330aca7ab431927540e9508888f0e1c";
        let new_pop =
            x"a8a0bcaf04e13565914eb22fa9f27a76f297db04446860ee2b923d10224cedb130b30783fb60b12556e7fc50e5b57a86";

        // prettier-ignore
        let new_worker_pub_key = vector[115, 220, 238, 151, 134, 159, 173, 41, 80, 2, 66, 196, 61, 17, 191, 76, 103, 39, 246, 127, 171, 85, 19, 235, 210, 106, 97, 97, 116, 48, 244, 191];
        // prettier-ignore
        let new_network_pub_key = vector[149, 128, 161, 13, 11, 183, 96, 45, 89, 20, 188, 205, 26, 127, 147, 254, 184, 229, 184, 102, 64, 170, 104, 29, 191, 171, 91, 99, 58, 178, 41, 156];

        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        // perform updates
        validator.update_next_epoch_network_address(b"/ip4/192.168.1.1/tcp/80");
        validator.update_next_epoch_p2p_address(b"/ip4/192.168.1.1/udp/80");
        validator.update_next_epoch_primary_address(b"/ip4/192.168.1.1/udp/80");
        validator.update_next_epoch_worker_address(b"/ip4/192.168.1.1/udp/80");
        validator.update_next_epoch_protocol_pubkey(new_protocol_pub_key, new_pop);
        validator.update_next_epoch_worker_pubkey(new_worker_pub_key);
        validator.update_next_epoch_network_pubkey(new_network_pub_key);

        validator.update_name(b"new_name");
        validator.update_description(b"new_desc");
        validator.update_image_url(b"new_image_url");
        validator.update_project_url(b"new_proj_url");

        // check updates
        assert_eq!(*validator.name(), b"new_name".to_string());
        assert_eq!(*validator.description(), b"new_desc".to_string());
        assert_eq!(*validator.image_url(), url::new_unsafe_from_bytes(b"new_image_url"));
        assert_eq!(*validator.project_url(), url::new_unsafe_from_bytes(b"new_proj_url"));
        assert_eq!(*validator.network_address(), validator_builder::valid_net_addr().to_string());
        assert_eq!(*validator.p2p_address(), validator_builder::valid_p2p_addr().to_string());
        assert_eq!(*validator.primary_address(), validator_builder::valid_consensus_addr().to_string());
        assert_eq!(*validator.worker_address(), validator_builder::valid_worker_addr().to_string());
        assert_eq!(*validator.protocol_pubkey_bytes(), validator_builder::valid_pubkey());
        assert_eq!(*validator.proof_of_possession(), validator_builder::valid_proof_of_possession());
        assert_eq!(*validator.network_pubkey_bytes(), validator_builder::valid_net_pubkey());
        assert_eq!(*validator.worker_pubkey_bytes(), validator_builder::valid_worker_pubkey());

        // next epoch
        assert!(
        validator
        .next_epoch_network_address()
        .is_some_and!(|addr| addr == b"/ip4/192.168.1.1/tcp/80".to_string()),
        );
        assert!(
        validator
        .next_epoch_p2p_address()
        .is_some_and!(|addr| addr == b"/ip4/192.168.1.1/udp/80".to_string()),
        );
        assert!(
        validator
        .next_epoch_primary_address()
        .is_some_and!(|addr| addr == b"/ip4/192.168.1.1/udp/80".to_string()),
        );
        assert!(
        validator
        .next_epoch_worker_address()
        .is_some_and!(|addr| addr == b"/ip4/192.168.1.1/udp/80".to_string()),
        );
        assert!(
        validator
        .next_epoch_protocol_pubkey_bytes()
        .is_some_and!(|key| key == new_protocol_pub_key),
        );
        assert!(validator.next_epoch_proof_of_possession().is_some_and!(|pop| pop == new_pop));
        assert!(
        validator.next_epoch_worker_pubkey_bytes().is_some_and!(|key| key == new_worker_pub_key),
        );
        assert!(
        validator.next_epoch_network_pubkey_bytes().is_some_and!(|key| key == new_network_pub_key),
        );

        test_utils::destroy(validator);
    }

    #[test, expected_failure(abort_code = sui_system::validator::EInvalidProofOfPossession)]
    fun validator_update_metadata_invalid_proof_of_possession() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_protocol_pubkey(
        x"96d19c53f1bee2158c3fcfb5bb2f06d3a8237667529d2d8f0fbb22fe5c3b3e64748420b4103674490476d98530d063271222d2a59b0f7932909cc455a30f00c69380e6885375e94243f7468e9563aad29330aca7ab431927540e9508888f0e1c",
        x"8b9794dfd11b88e16ba8f6a4a2c1e7580738dce2d6910ee594bebd88297b22ae8c34d1ee3f5a081159d68e076ef5d300",
        );

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidNetPubkey)]
    fun validator_update_metadata_invalid_network_key() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_network_pubkey(x"beef");

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidWorkerPubkey)]
    fun validator_update_metadata_invalid_worker_key() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_worker_pubkey(x"beef");

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidNetAddr)]
    fun validator_update_metadata_invalid_network_addr() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_network_address(b"beef");

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidPrimaryAddr)]
    fun validator_update_metadata_invalid_primary_addr() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_primary_address(b"beef");

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidWorkerAddr)]
    fun validator_update_metadata_invalid_worker_addr() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_worker_address(b"beef");

        abort
    }

    #[test, expected_failure(abort_code = sui_system::validator::EMetadataInvalidP2pAddr)]
    fun validator_update_metadata_invalid_p2p_address() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_p2p_address(b"beef");

        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_metadata_primary_address_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_primary_address(vector::tabulate!(257, |_| 0));
        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_metadata_net_address_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_network_address(vector::tabulate!(257, |_| 0));

        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_metadata_worker_address_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_worker_address(vector::tabulate!(257, |_| 0));

        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_metadata_p2p_address_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_next_epoch_p2p_address(vector::tabulate!(257, |_| 0));

        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_name_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_name(vector::tabulate!(257, |_| 0));

        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_description_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_description(vector::tabulate!(257, |_| 0));

        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_project_url_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_project_url(vector::tabulate!(257, |_| 0));

        abort
    }

    #[
    test,
    expected_failure(
        abort_code = sui_system::validator::EValidatorMetadataExceedingLengthLimit,
    ),
    ]
    fun validator_update_image_url_too_long() {
        let ctx = &mut tx_context::dummy();
        let mut validator = validator_builder::preset().build(ctx);

        validator.update_image_url(vector::tabulate!(257, |_| 0));

        abort
    }
}
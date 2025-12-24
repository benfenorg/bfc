// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_variable)]
module bridge::bridge_env {
    use bridge::bridge::{
        assert_not_paused,
        assert_paused,
        create_bridge_for_testing,
        inner_token_transfer_records,
        test_init_bridge_committee,
        test_get_external_token_transfer_action_status,
        get_available_claim_amount,
        test_load_inner_mut,
        test_load_mut_uid,
        get_cross_out_fee_amount,
        get_cross_in_fee_amount,
        Bridge,
        EmergencyOpEvent,
        TokenDepositedEventV2,
        TokenDepositedEventForSolanaV2,
        ExternalDepositedEventV2,
        TokenSendBackEventV2,
        TokenSendBackEventForSolanaV2,
        TokenTransferAlreadyApproved,
        TokenTransferAlreadyClaimed,
        TokenTransferApproved,
        TokenTransferClaimed,
        TokenTransferLimitExceed,
        // ExternalDepositedEvent,
        ExternalWithdrawEventV2,
        ExternalBridgeRecord,
        ExternalDepositedApprovedEvent,
    };
    use std::ascii;
    use std::debug;
    use bridge::btc::{Self, BTC};
    use bridge::chain_ids;
    use bridge::committee::BlocklistValidatorEvent;
    use bridge::eth::{Self, ETH};
    use bridge::limiter::UpdateRouteLimitEvent;
    use bridge::message::{
        Self,
        BridgeMessage,
        create_add_tokens_on_sui_message,
        create_add_external_coin_admin_message,
        create_remove_external_coin_admin_message,
        create_add_external_coin_witness_message,
        create_remove_external_coin_witness_message,
        create_add_external_coin_target_message,
        create_remove_external_coin_target_message,
        create_blocklist_message,
        create_add_token_on_token_list,
        create_remove_token_on_token_list,
        create_set_cross_in_bridge_fee,
        create_set_cross_out_bridge_fee,
        create_withdraw_fee_cap,
        create_fast_path_limit_message_v2,
        emergency_op_pause,
        emergency_op_unpause
    };
    use bridge::message_types;
    use bridge::test_token::{Self, TEST_TOKEN};
    use bridge::treasury::{
        TokenRegistrationEvent,
        NewTokenEvent,
        UpdateTokenPriceEvent
    };
    use bridge::bridge_fee::{Self,WithdrawBridgeFeeCap};
    use bridge::usdc::{Self, USDC};
    use bridge::usdt::{Self, USDT};
    use std::ascii::String;
    use std::type_name;
    use bridge::bnb::{Self, BNB};
    use bridge::busd::{Self, BUSD};

    use bridge::op::{ OP};
    use bridge::tokenlist;
    use sui::address;
    use sui::clock::Clock;
    use sui::coin::{Self, Coin, CoinMetadata, TreasuryCap};
    use sui::ecdsa_k1::{KeyPair, secp256k1_keypair_from_seed, secp256k1_sign};
    use sui::event;
    use sui::package::UpgradeCap;
    use sui::test_scenario::{Self, Scenario};
    use sui::test_utils::destroy;
    use sui_system::governance_test_utils::{
        advance_epoch_with_reward_amounts,
        create_sui_system_state_for_testing,
        create_validator_for_testing
    };
    use sui_system::sui_system::{
        validator_voting_powers_for_testing,
        SuiSystemState
    };
    use sui::hex;
    use bridge::limiter_fast_path;

    //
    // Token IDs
    //
    const BTC_ID: u64 = 1;
    const ETH_ID: u64 = 2;
    const USDC_ID: u64 = 3;
    const USDT_ID: u64 = 4;
    const BUSD_ID: u64 = 5;
    const BNB_ID: u64 = 6;
    const OP_ID: u64=7;


    public fun btc_id(): u64 {
        BTC_ID
    }

    public fun eth_id(): u64 {
        ETH_ID
    }

    public fun usdc_id(): u64 {
        USDC_ID
    }

    public fun usdt_id(): u64 {
        USDT_ID
    }

    public fun busd_id(): u64 {
        BUSD_ID
    }

    public fun  test_token_id(): u64 {
        100
    }

    public fun bnb_id(): u64 {
        BNB_ID
    }

    public fun op_id(): u64{
        OP_ID
    }

    //
    // Claim status
    //
    const CLAIMED: u8 = 1;
    const ALREADY_CLAIMED: u8 = 2;
    const LIMIT_EXCEEDED: u8 = 3;

    public fun claimed(): u8 {
        CLAIMED
    }

    public fun already_claimed(): u8 {
        ALREADY_CLAIMED
    }

    public fun limit_exceeded(): u8 {
        LIMIT_EXCEEDED
    }

    //
    // Approve status
    //
    const APPROVED: u8 = 1;
    const ALREADY_APPROVED: u8 = 2;

    public fun approved(): u8 {
        APPROVED
    }

    public fun already_approved(): u8 {
        ALREADY_APPROVED
    }

    //
    // Validators setup and info
    //

    // Validator info
    public struct ValidatorInfo has drop {
        validator: address,
        key_pair: KeyPair,
        stake_amount: u64,
    }

    public fun addr(validator: &ValidatorInfo): address {
        validator.validator
    }

    public fun public_key(validator: &ValidatorInfo): &vector<u8> {
        validator.key_pair.public_key()
    }

    public fun create_validator(
        validator: address,
        stake_amount: u64,
        seed: &vector<u8>,
    ): ValidatorInfo {
        ValidatorInfo {
            validator,
            key_pair: secp256k1_keypair_from_seed(seed),
            stake_amount,
        }
    }

    // Bridge environemnt
    public struct BridgeEnv {
        scenario: Scenario,
        validators: vector<ValidatorInfo>,
        chain_id: u8,
        vault: Vault,
        clock: Clock,
    }

    // Holds coins for different bridged tokens
    public struct Vault {
        btc_coins: Coin<BTC>,
        eth_coins: Coin<ETH>,
        usdc_coins: Coin<USDC>,
        usdt_coins: Coin<USDT>,
        test_coins: Coin<TEST_TOKEN>,
        bnb_coins: Coin<BNB>,
        op_coins: Coin<OP>,
    }

    // HotPotato to access shared state
    // TODO: if the bridge is the only shared state we could remvove this
    public struct BridgeWrapper {
        bridge: Bridge,
    }

    public fun bridge(env: &mut BridgeEnv, sender: address): BridgeWrapper {
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let bridge = scenario.take_shared<Bridge>();
        BridgeWrapper { bridge }
    }

    public fun bridge_ref(wrapper: &BridgeWrapper): &Bridge {
        &wrapper.bridge
    }

    public fun bridge_ref_mut(wrapper: &mut BridgeWrapper): &mut Bridge {
        &mut wrapper.bridge
    }

    public fun return_bridge(bridge: BridgeWrapper) {
        let BridgeWrapper { bridge } = bridge;
        test_scenario::return_shared(bridge);
    }

    //
    // Public functions
    //

    //
    // Environment creation and destruction
    //

    public fun create_env(chain_id: u8): BridgeEnv {
        let mut scenario = test_scenario::begin(@0x0);
        let ctx = scenario.ctx();
        let mut clock = sui::clock::create_for_testing(ctx);
        clock.set_for_testing(1_000_000_000);
        let btc_coins = coin::zero<BTC>(ctx);
        let eth_coins = coin::zero<ETH>(ctx);
        let usdc_coins = coin::zero<USDC>(ctx);
        let usdt_coins = coin::zero<USDT>(ctx);
        let test_coins = coin::zero<TEST_TOKEN>(ctx);
        let bnb_coins = coin::zero<BNB>(ctx);
        let op_coins=coin::zero<OP>(ctx);
        let vault = Vault {
            btc_coins,
            eth_coins,
            usdc_coins,
            usdt_coins,
            test_coins,
            bnb_coins,
            op_coins
        };
        BridgeEnv {
            scenario,
            chain_id,
            vault,
            validators: vector::empty(),
            clock,
        }
    }

    public fun destroy_env(env: BridgeEnv) {
        let BridgeEnv {
            scenario,
            chain_id: _,
            vault,
            validators: _,
            clock,
        } = env;
        destroy_valut(vault);
        clock.destroy_for_testing();
        scenario.end();
    }

    //
    // Add a set of validators to the chain.
    // Call only once in a test scenario.
    public fun setup_validators(
        env: &mut BridgeEnv,
        validators_info: vector<ValidatorInfo>,
    ) {
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let ctx = scenario.ctx();
        let validators = validators_info.map_ref!(
            |validator| {
                create_validator_for_testing(
                    validator.validator,
                    validator.stake_amount,
                    ctx,
                )
            },
        );
        env.validators = validators_info;
        create_sui_system_state_for_testing(validators, 0, 0, ctx);
        advance_epoch_with_reward_amounts(0, 0, scenario);
    }

    //
    // Bridge creation and setup
    //

    // Set up an environment with 3 validators, a bridge with
    // a treasury and a committee with all 3 validators.
    // The treasury will contain 5 tokens: ETH, BTC, USDT, USDC, BNB.
    // Save the Bridge as a shared object.
    public fun create_bridge_default(env: &mut BridgeEnv) {
        let validators = vector[
            create_validator(
                @0xAAAA,
                100,
                &b"1234567890_1234567890_1234567890",
            ),
            create_validator(
                @0xBBBB,
                100,
                &b"234567890_1234567890_1234567890_",
            ),
            create_validator(
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
        //env.init_external_limiter(sender);
        env.add_refund_admin(@0xABCD)
    }

    // Create a bridge and set up a treasury.
    // The treasury will contain 4 tokens: ETH, BTC, USDT, USDC, BNB.
    // Save the Bridge as a shared object.
    // No operation on the validators.
    public fun create_bridge(env: &mut BridgeEnv, sender: address) {
        env.scenario.next_tx(sender);
        let ctx = env.scenario.ctx();
        create_bridge_for_testing(object::new(ctx), env.chain_id, ctx);
    }

    public fun add_tokenlist(env: &mut BridgeEnv, sender: address){
        env.scenario.next_tx(sender);
        let mut bridge = env.scenario.take_shared<Bridge>();
        let ctx = env.scenario.ctx();
        bridge.init_token_list(ctx);
        //add center token list
        bridge.migrate(ctx);
        test_scenario::return_shared(bridge);
    }

    // Register 3 committee members (validators `@0xA`, `@0xB`, `@0xC`)
    public fun register_committee(env: &mut BridgeEnv) {
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let mut system_state = test_scenario::take_shared<SuiSystemState>(
            scenario,
        );

        env
            .validators
            .do_ref!(
                |validator| {
                    scenario.next_tx(validator.validator);
                    bridge.committee_registration(
                        &mut system_state,
                        *validator.key_pair.public_key(),
                        b"",
                        scenario.ctx(),
                    );
                },
            );

        test_scenario::return_shared(bridge);
        test_scenario::return_shared(system_state);
    }

    // Init the bridge committee
    public fun init_committee(env: &mut BridgeEnv, sender: address) {
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        let mut system_state = test_scenario::take_shared<SuiSystemState>(
            scenario,
        );
        let voting_powers = validator_voting_powers_for_testing(
            &mut system_state,
        );
        bridge.test_init_bridge_committee(
            voting_powers,
            7500,
            scenario.ctx(),
        );
        test_scenario::return_shared(bridge);
        test_scenario::return_shared(system_state);
    }

    // Set up a treasury with 4 tokens: ETH, BTC, USDT, USDC, BNB.
    public fun setup_treasury(env: &mut BridgeEnv, sender: address) {
        env.register_default_tokens(sender);
        env.add_default_tokens(sender);
        env.load_vault(sender);
    }


    // Register 4 tokens with the Bridge: ETH, BTC, USDT, USDC, BNB.
    fun register_default_tokens(env: &mut BridgeEnv, sender: address) {
        env.scenario.next_tx(sender);
        let mut bridge = env.scenario.take_shared<Bridge>();

        // BTC
        let (upgrade_cap, treasury_cap, metadata) = btc::create_bridge_token(env
            .scenario
            .ctx());
        bridge.register_foreign_token<BTC>(
            treasury_cap,
            upgrade_cap,
            &metadata,
        );
        destroy(metadata);
        // ETH
        let (upgrade_cap, treasury_cap, metadata) = eth::create_bridge_token(env
            .scenario
            .ctx());
        bridge.register_foreign_token<ETH>(
            treasury_cap,
            upgrade_cap,
            &metadata,
        );
        destroy(metadata);
        // USDC
        let (
            upgrade_cap,
            treasury_cap,
            metadata,
        ) = usdc::create_bridge_token(env.scenario.ctx());
        bridge.register_foreign_token<USDC>(
            treasury_cap,
            upgrade_cap,
            &metadata,
        );
        destroy(metadata);
        // USDT
        let (
            upgrade_cap,
            treasury_cap,
            metadata,
        ) = usdt::create_bridge_token(env.scenario.ctx());
        bridge.register_foreign_token<USDT>(
            treasury_cap,
            upgrade_cap,
            &metadata,
        );
        destroy(metadata);
        // BNB
        let (
            upgrade_cap,
            treasury_cap,
            metadata,
        ) = bnb::create_bridge_token(env.scenario.ctx());
        bridge.register_foreign_token<BNB>(
            treasury_cap,
            upgrade_cap,
            &metadata,
        );
        destroy(metadata);
        // BUSD
        let (
            upgrade_cap,
            treasury_cap,
            metadata,
        ) = busd::create_bridge_token(env.scenario.ctx());
        bridge.register_foreign_token<BUSD>(
            treasury_cap,
            upgrade_cap,
            &metadata,
        );
        destroy(metadata);

        test_scenario::return_shared(bridge);
    }

    fun add_refund_admin(env: &mut BridgeEnv, sender: address) {
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        bridge.setup_refund_admin_for_testing(sender.to_ascii_string());
        test_scenario::return_shared(bridge);
    }

    // Add the 4 tokens previously registered: ETH, BTC, USDT, USDC, BNB.
    fun add_default_tokens(env: &mut BridgeEnv, sender: address) {
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();

        let add_token_message = create_add_tokens_on_sui_message(
            env.chain_id,
            bridge.get_seq_num_for(message_types::add_tokens_on_sui()),
            false,
            vector[BTC_ID, ETH_ID, USDC_ID, USDT_ID, BUSD_ID,BNB_ID],
            vector[
                type_name::get<BTC>().into_string(),
                type_name::get<ETH>().into_string(),
                type_name::get<USDC>().into_string(),
                type_name::get<USDT>().into_string(),
                type_name::get<BUSD>().into_string(),
                type_name::get<BNB>().into_string(),
            ],
            vector[1000, 100, 1, 1, 1,60],
        );
        let signatures = env.sign_message(add_token_message);
        bridge.execute_system_message(add_token_message, signatures);

        test_scenario::return_shared(bridge);
    }

    public fun add_external_coin_admin(
        env: &mut BridgeEnv,
        coin_type_name: String,
        address: String,
    ) {
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();

        let add_message = create_add_external_coin_admin_message(
            env.chain_id,
            bridge.get_seq_num_for(message_types::add_external_coin_admin()),
            coin_type_name,
            address,
        );
        let signatures = env.sign_message(add_message);
        bridge.execute_system_message(add_message, signatures);

        // check
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let admin_cap = treasury.external_coin_admin_address();
        std::debug::print( admin_cap);
        let admins = admin_cap.try_get(&coin_type_name);
        assert!(admins.is_some());
        let admins = admins.destroy_some();
        assert!(admins.contains(&address));

        test_scenario::return_shared(bridge);
    }

    public fun add_external_coin_witness(
        env: &mut BridgeEnv,
        coin_type_name: String,
        addr: vector<u8>,
    ){
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let add_message = create_add_external_coin_witness_message(env.chain_id, bridge.get_seq_num_for(message_types::add_external_coin_witness()), coin_type_name, addr);
        debug::print(&add_message);
        let signatures = env.sign_message(add_message);
        bridge.execute_system_message(add_message, signatures);
          // check
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let admin_cap = treasury.external_coin_witness_address();
        std::debug::print( admin_cap);
        let admins = admin_cap.try_get(&coin_type_name);
        assert!(admins.is_some());
        let admins = admins.destroy_some();
        debug::print(&admins);
        assert!(admins.contains(&addr));

        test_scenario::return_shared(bridge);
    }


    public fun remove_external_coin_witness(
        env: &mut BridgeEnv,
        coin_type_name: String,
        addr: vector<u8>,
    )    {
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let remove_message = create_remove_external_coin_witness_message(
            env.chain_id,
            bridge.get_seq_num_for(message_types::remove_external_coin_witness()),
            coin_type_name,
            addr,
        );
        let signatures = env.sign_message(remove_message);
        bridge.execute_system_message(remove_message, signatures);

        // check
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let admin_cap = treasury.external_coin_witness_address();
        let admins = admin_cap.try_get(&coin_type_name);
        if (admins.is_some()) {
            let admins = admins.destroy_some();
            assert!(!admins.contains(&addr));
        };

        test_scenario::return_shared(bridge);
    }

    public fun add_token_on_token_list(
        env: &mut BridgeEnv,
        source_chain: u8,
        target_chain: u8,
        token_id: u64,
    ){
       //let scenario = &mut env.scenario;
        env.scenario.next_tx(@0x0);
        let mut bridge = env.scenario.take_shared<Bridge>();
        let add_message = create_add_token_on_token_list(env.chain_id, bridge.get_seq_num_for(message_types::add_token_on_token_list()),source_chain,target_chain , token_id);
        let signatures = env.sign_message(add_message);
        bridge.execute_system_message_with_ctx(add_message, signatures,env.scenario.ctx());
        //check
        env.scenario.next_tx(@0x1);
        let uid=test_load_mut_uid(&mut bridge);
        let exist= if (env.chain_id==source_chain){
            tokenlist::is_supported_from_benfen(uid,target_chain as u64,token_id)
        }else{
            tokenlist::is_supported_to_benfen(uid,source_chain as u64,token_id)
        };
        assert!(exist,0);
        test_scenario::return_shared(bridge);
    }
    public fun set_cross_in_bridge_fee<T>(
          env: &mut BridgeEnv,
          from_chain: u8,
          mode :u64,
          fee: u64,
          amount: u64,
    ){
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let token_id=1; //btc

        let add_message = create_set_cross_in_bridge_fee(env.chain_id, bridge.get_seq_num_for(message_types::set_cross_in_bridge_fee()),from_chain,token_id,mode ,fee);
        let signatures = env.sign_message(add_message);
        bridge.execute_system_message_with_ctx(add_message, signatures,env.scenario.ctx());
        env.scenario.next_tx(@0x1);
        //check
        if(mode==0){
            assert!(amount>fee);
            assert!(get_cross_in_fee_amount<T>(&bridge, from_chain as u64, amount)==fee,1);
            //after_fee=amount-get_cross_in_fee_amount<T>(&bridge, from_chain as u64, amount);
        }else if(mode==1){
            std::debug::print(&get_cross_in_fee_amount<T>(&bridge, from_chain as u64, amount));
            assert!(get_cross_in_fee_amount<T>(&bridge, from_chain as u64, amount)== (amount * fee) / 1_000_000);
        }else{
            abort 0
        };

        test_scenario::return_shared(bridge);
    }

    public fun set_cross_out_bridge_fee<T>(
          env: &mut BridgeEnv,
          to_chain: u8,
          mode :u64,
          fee: u64,
          amount: u64,
    ){
         let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let token_id=1; //btc

        let add_message = create_set_cross_out_bridge_fee(env.chain_id, bridge.get_seq_num_for(message_types::set_cross_out_bridge_fee()),to_chain,token_id,mode ,fee);
        let signatures = env.sign_message(add_message);
        bridge.execute_system_message_with_ctx(add_message, signatures,env.scenario.ctx());
        env.scenario.next_tx(@0x1);
        //check
        if(mode==0){
            assert!(amount>fee);
            assert!(get_cross_out_fee_amount<T>(&bridge, to_chain as u64, amount)==fee,1);
        }else if(mode==1){
            assert!(get_cross_out_fee_amount<T>(&bridge, to_chain as u64, amount)== (amount * fee) / 1_000_000);
        }else{
            abort 0
        };
        test_scenario::return_shared(bridge);
    }


    public fun withdraw_bridge_fee_cap<T>(
        env: &mut BridgeEnv,
        amount: u64
    ){
        // let scenario = &mut env.scenario;
        let coin_type=type_name::get<T>().into_string();
        env.scenario.next_tx(@0x0);
        let mut bridge =  env.scenario.take_shared<Bridge>();
        let sender=@0x5;

        let add_message = create_withdraw_fee_cap(env.chain_id, bridge.get_seq_num_for(message_types::withdraw_bridge_fee()),sender,coin_type ,amount);
        let signatures = env.sign_message(add_message);
        bridge.execute_system_message_with_ctx(add_message, signatures,env.scenario.ctx());

        env.scenario.next_tx(@0x5);
        //check
        {
            let cap =  env.scenario.take_from_sender<WithdrawBridgeFeeCap>();
            std::debug::print(&cap);
            assert!(bridge_fee::get_withdraw_cap_coin_type(&cap)==coin_type,1);
            assert!(bridge_fee::get_withdraw_cap_amount(&cap)==amount,1);
            env.scenario.return_to_sender(cap);
        };
        test_scenario::return_shared(bridge);
    }

    public fun fast_path_limit_update(
        env: &mut BridgeEnv,
        amount: u64
    ){
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge =  env.scenario.take_shared<Bridge>();
        // bridge.migrate(env.scenario.ctx());
        let update_message = create_fast_path_limit_message_v2(
            bridge.get_seq_num_for(message_types::fast_path_limit_update()),
            chain_ids::sui_testnet(),
            5,
            amount,
            chain_ids::arb_custom(),
        );
        let signatures = env.sign_message(update_message);
        bridge.execute_system_message(update_message, signatures);

        env.scenario.next_tx(@0x0);
        {
            let uid = bridge.test_load_mut_uid();
            let fast_path_limit_amount = limiter_fast_path::get_limit_config_info(uid,chain_ids::arb_custom(),5);
            assert!(fast_path_limit_amount == amount, 0);
        };
        test_scenario::return_shared(bridge);
    }


    public fun remove_token_on_token_list(
        env: &mut BridgeEnv,
        source_chain: u8,
        target_chain: u8,
        token_id: u64,
    ){
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        {
            let uid=test_load_mut_uid(&mut bridge);
            let exist= if (env.chain_id==source_chain){
                tokenlist::is_supported_from_benfen(uid,target_chain as u64,token_id)
            }else{
                tokenlist::is_supported_to_benfen(uid,source_chain as u64,token_id)
            };
            assert!(exist,0);
        };
        let remove_message = create_remove_token_on_token_list(env.chain_id, bridge.get_seq_num_for(message_types::remove_token_on_token_list()),source_chain,target_chain , token_id);
        let signatures = env.sign_message(remove_message);
        bridge.execute_system_message_with_ctx(remove_message, signatures,env.scenario.ctx());
        //check
        {
            let uid=test_load_mut_uid(&mut bridge);
            let exist= if (env.chain_id==source_chain){
                tokenlist::is_supported_from_benfen(uid,target_chain as u64,token_id)
            }else{
                tokenlist::is_supported_to_benfen(uid,source_chain as u64,token_id)
            };
            assert!(!exist,0);
        };
        test_scenario::return_shared(bridge);
    }

    public fun remove_external_coin_admin(
        env: &mut BridgeEnv,
        coin_type_name: String,
        address: String,
    )    {
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let remove_message = create_remove_external_coin_admin_message(
            env.chain_id,
            bridge.get_seq_num_for(message_types::remove_external_coin_admin()),
            coin_type_name,
            address,
        );
        let signatures = env.sign_message(remove_message);
        bridge.execute_system_message(remove_message, signatures);

        // check
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let admin_cap = treasury.external_coin_admin_address();
        std::debug::print( admin_cap);
        let admins = admin_cap.try_get(&coin_type_name);
        if (admins.is_some()) {
            let admins = admins.destroy_some();
            assert!(!admins.contains(&address));
        };

        test_scenario::return_shared(bridge);
    }

    public fun add_external_coin_target(
        env: &mut BridgeEnv,
        coin_type_name: String,
        addr: String,
    ){
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let add_message = create_add_external_coin_target_message(env.chain_id, bridge.get_seq_num_for(message_types::add_external_coin_target()), coin_type_name, *addr.as_bytes());
        let signatures = env.sign_message(add_message);
        bridge.execute_system_message(add_message, signatures);
        // check
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let admin_cap = treasury.external_coin_target_address();
        std::debug::print( admin_cap);
        let admins = admin_cap.try_get(&coin_type_name);
        assert!(admins.is_some());
        let admins = admins.destroy_some();
        debug::print(&admins);
        debug::print(&addr);
        assert!(admins.contains(&addr));

        test_scenario::return_shared(bridge);
    }

    public fun remove_external_coin_target(
        env: &mut BridgeEnv,
        coin_type_name: String,
        addr: String,
    ){
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let remove_message = create_remove_external_coin_target_message(
            env.chain_id,
            bridge.get_seq_num_for(message_types::remove_external_coin_target()),
            coin_type_name,
            *addr.as_bytes(),
        );
        let signatures = env.sign_message(remove_message);
        bridge.execute_system_message(remove_message, signatures);

        // check
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let admin_cap = treasury.external_coin_target_address();
        let admins = admin_cap.try_get(&coin_type_name);
        if (admins.is_some()) {
            let admins = admins.destroy_some();
            assert!(!admins.contains(&addr));
        };

        test_scenario::return_shared(bridge);
    }


    //
    // Utility functions for custom behavior
    //

    public fun token_type<T>(env: &mut BridgeEnv): u64 {
        env.scenario.next_tx(@0x0);
        let bridge = env.scenario.take_shared<Bridge>();
        let inner = bridge.test_load_inner();
        let token_id = inner.inner_treasury().token_id<T>();
        test_scenario::return_shared(bridge);
        token_id
    }

    const SUI_MESSAGE_PREFIX: vector<u8> = b"SUI_BRIDGE_MESSAGE";

    fun sign_message(
        env: &BridgeEnv,
        message: BridgeMessage,
    ): vector<vector<u8>> {
        let mut message_bytes = SUI_MESSAGE_PREFIX;
        message_bytes.append(message.serialize_message());
        let mut message_bytes = SUI_MESSAGE_PREFIX;
        message_bytes.append(message.serialize_message());
        env
            .validators
            .map_ref!(
                |validator| {
                    secp256k1_sign(
                        validator.key_pair.private_key(),
                        &message_bytes,
                        0,
                        true,
                    )
                },
            )
    }

    public fun sign_message_with(
        env: &BridgeEnv,
        message: BridgeMessage,
        validator_idxs: vector<u64>,
    ): vector<vector<u8>> {
        let mut message_bytes = SUI_MESSAGE_PREFIX;
        message_bytes.append(message.serialize_message());
        validator_idxs.map!(
            |idx| {
                secp256k1_sign(
                    env.validators[idx].key_pair.private_key(),
                    &message_bytes,
                    0,
                    true,
                )
            },
        )
    }

    public fun sign_message_with_mut(
        env: &mut BridgeEnv,
        message: BridgeMessage,
        validator_idxs: vector<u64>,
    ): vector<vector<u8>> {
        let mut message_bytes = SUI_MESSAGE_PREFIX;
        message_bytes.append(message.serialize_message());
        validator_idxs.map!(
            |idx| {
                secp256k1_sign(
                    env.validators[idx].key_pair.private_key(),
                    &message_bytes,
                    0,
                    true,
                )
            },
        )
    }

    public fun bridge_in_message<Token>(
        env: &mut BridgeEnv,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: address,
        amount: u64,
    ): BridgeMessage {
        let token_type = env.token_type<Token>();

        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();

        let message = message::create_token_bridge_in_message(
            source_chain,
            bridge.get_seq_num_inc_for(message_types::token()),
            source_address,
            env.chain_id,
            address::to_bytes(target_address),
            token_type,
            amount,
            hex::decode(b""),
            0u16, // event_idx
            2u8,
        );
        test_scenario::return_shared(bridge);
        message
    }

    public fun bridge_out_message<Token>(
        env: &mut BridgeEnv,
        target_chain: u8,
        target_address: vector<u8>,
        source_address: address,
        amount: u64,
        transfer_id: u64,
    ): BridgeMessage {
        let token_type = env.token_type<Token>();

        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let bridge = scenario.take_shared<Bridge>();

        let message = message::create_token_bridge_message_v2(
            env.chain_id,
            transfer_id,
            address::to_bytes(source_address),
            target_chain,
            target_address,
            token_type,
            amount,
            hex::decode(b""),
            0u16, // event_idx
        );
        test_scenario::return_shared(bridge);
        message
    }

    public fun bridge_token_signed_message<Token>(
        env: &mut BridgeEnv,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: address,
        amount: u64,
    ): (BridgeMessage, vector<vector<u8>>) {
        let token_type = env.token_type<Token>();
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();
        let seq_num = bridge.get_seq_num_inc_for(message_types::token());
        test_scenario::return_shared(bridge);
        let message = message::create_token_bridge_message_v2(
            source_chain,
            seq_num,
            source_address,
            env.chain_id,
            address::to_bytes(target_address),
            token_type,
            amount,
            hex::decode(b""),
            0u16, // event_idx
        );
        let signatures = env.sign_message(message);
        (message, signatures)
    }

    // Bridge the `amount` of the given `Token` from the `source_chain`.
    public fun bridge_to_sui<Token>(
        env: &mut BridgeEnv,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: address,
        amount: u64,
    ): u64 {
        let token_type = env.token_type<Token>();

        // setup
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();

        // sign message
        let seq_num = bridge.get_seq_num_inc_for(message_types::token());
        let message = message::create_token_bridge_in_message(
            source_chain,
            seq_num,
            source_address,
            env.chain_id,
            address::to_bytes(target_address),
            token_type,
            amount,
            hex::decode(b""),
            0u16, // event_idx
            2u8,
        );
        let signatures = env.sign_message(message);

        // run approval
        bridge.approve_token_transfer_in(message, signatures);

        // verify approval events
        let approved_events = event::events_by_type<TokenTransferApproved>();
        let already_approved_events = event::events_by_type<
            TokenTransferAlreadyApproved,
        >();
        assert!(
            approved_events.length() == 1 ||
            already_approved_events.length() == 1,
        );
        let key = if (approved_events.length() == 1) {
            approved_events[0].transfer_approve_key()
        } else {
            already_approved_events[0].transfer_already_approved_key()
        };
        let (sc, mt, sn) = key.unpack_message();
        assert!(source_chain == sc);
        assert!(mt == message_types::token());
        assert!(sn == seq_num);

        // tear down
        test_scenario::return_shared(bridge);
        seq_num
    }

     public fun bridge_external_coin_v2<T>(
        env: &mut BridgeEnv,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        tx_hash: ascii::String,
        amount: u64,
    ) {
        // setup
        let token_type = env.token_type<T>();
        env.scenario.next_tx(@0x0);
        let mut bridge = env.scenario.take_shared<Bridge>();
        let total_supply_before = get_total_supply<T>(&bridge);

        let message = message::create_token_bridge_message(
            source_chain,
            0,
            source_address,
            env.chain_id,
            target_address,
            token_type,
            amount,
            *tx_hash.as_bytes(),
            0u8, // event_idx
        );

        let signatures = env.sign_message(message);

        bridge.approval_and_claimed_external_coin<T>(message, signatures, env.scenario.ctx());
        let approved = event::events_by_type<ExternalDepositedApprovedEvent>();
        let deposited = event::events_by_type<ExternalDepositedEventV2>();
        assert!(approved.length() == 0 && deposited.length() == 1);
        {
            let (
                tx_hash,
                token_type,
                source_chain,
                target_chain,
                source_address,
                target_address,
                amount_before_fee,
                amount_after_fee
            ) = deposited[0].unwrap_external_deposited_event_v2();
            assert!(
                tx_hash == tx_hash &&
                token_type ==token_type &&
                source_chain == source_chain &&
                target_chain == env.chain_id &&
                source_address == source_address &&
                target_address == target_address &&
                amount == amount_before_fee,
            );
        };

        env.scenario.next_tx(@0x0);
        let token = env.scenario.take_from_address<Coin<T>>(address::from_bytes(target_address));
        // verify value change and claim events
        let token_value = token.value();
        assert!(token.balance().value() == amount);
        assert!(
            total_supply_before + token_value == get_total_supply<T>(&bridge),
        );

        // withdraw coin
        env.scenario.next_tx(address::from_bytes(target_address));
        withdraw_external_coin_for_testing<T>(
            &mut bridge,
            source_chain,
            source_address,
            token,
            env.scenario.ctx(),
        );

        let fee=get_cross_out_fee_amount<T>(&bridge,source_chain as u64,amount);
        assert!(amount>fee,1);
        let withdraw = event::events_by_type<ExternalWithdrawEventV2>();
        assert!(withdraw.length() == 1);
        {
            debug::print(&withdraw);
            let (
                event_token_type,
                event_source_chain,
                event_target_chain,
                event_source_address,
                event_target_address,
                event_amount_before_fee,
                event_amount_after_fee,
            ) = withdraw[0].unwrap_external_withdrawn_v2_event();
            assert!(event_token_type == token_type);
            assert!(event_source_chain == env.chain_id );
            assert!(event_target_chain == source_chain);
            assert!(event_source_address == target_address);
            assert!(event_target_address == source_address);
            assert!(event_amount_before_fee == amount);
            std::debug::print(&fee);
            assert!(event_amount_after_fee == amount-fee);
            assert!(
                total_supply_before+fee == get_total_supply<T>(&bridge),
            );
        };

        env.scenario.next_tx(@0x0);
        bridge.approval_and_claimed_external_coin<T>(message, signatures, env.scenario.ctx());
        let approved = event::events_by_type<ExternalDepositedApprovedEvent>();
        let deposited = event::events_by_type<ExternalDepositedEventV2>();
        assert!(approved.length() == 1 && deposited.length() == 0);
        {
            let (
                tx_hash,
                coin_type,
                source_chain,
                target_chain,
                source_address,
                target_address,
                amount,
            ) = approved[0].unwrap_external_deposited_approved_event();
            assert!(
                tx_hash == tx_hash &&
                coin_type == type_name::get<T>().into_string() &&
                source_chain == source_chain &&
                target_chain == env.chain_id &&
                source_address == source_address &&
                target_address == target_address &&
                amount == amount,
            );
        };

        env.scenario.next_tx(@0x0);
        assert!(
            total_supply_before + fee == get_total_supply<T>(&bridge),
        );

        // tear down
        test_scenario::return_shared(bridge);
    }

    // Approves a token transer
    public fun approve_token_transfer(
        env: &mut BridgeEnv,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
    ): u8 {
        let msg_key = message.key();

        // set up
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();

        // run approval
        bridge.approve_token_transfer_v2(message, signatures);

        // verify approval events
        let approved = event::events_by_type<TokenTransferApproved>();
        let already_approved = event::events_by_type<
            TokenTransferAlreadyApproved,
        >();
        assert!(approved.length() == 1 || already_approved.length() == 1);
        let (key, approve_status) = if (approved.length() == 1) {
            (approved[0].transfer_approve_key(), APPROVED)
        } else {
            (
                already_approved[0].transfer_already_approved_key(),
                ALREADY_APPROVED,
            )
        };
        assert!(msg_key == key);

        // tear down
        test_scenario::return_shared(bridge);
        approve_status
    }

    public fun approve_token_transfer_in(
        env: &mut BridgeEnv,
        message: BridgeMessage,
        signatures: vector<vector<u8>>,
    ): u8 {
        let msg_key = message.key();

        // set up
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();

        // run approval
        bridge.approve_token_transfer_in(message, signatures);

        // verify approval events
        let approved = event::events_by_type<TokenTransferApproved>();
        let already_approved = event::events_by_type<
            TokenTransferAlreadyApproved,
        >();
        assert!(approved.length() == 1 || already_approved.length() == 1);
        let (key, approve_status) = if (approved.length() == 1) {
            (approved[0].transfer_approve_key(), APPROVED)
        } else {
            (
                already_approved[0].transfer_already_approved_key(),
                ALREADY_APPROVED,
            )
        };
        assert!(msg_key == key);

        // tear down
        test_scenario::return_shared(bridge);
        approve_status
    }

    // Clain a token transfer and returns the coin
    public fun claim_token<T>(
        env: &mut BridgeEnv,
        sender: address,
        source_chain: u8,
        bridge_seq_num: u64,
    ): Coin<T> {
        // set up
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let clock = &env.clock;
        let mut bridge = scenario.take_shared<Bridge>();
        let ctx = scenario.ctx();
        let total_supply_before = get_total_supply<T>(&bridge);
        // run claim
        let token = bridge.claim_token<T>(
            clock,
            source_chain,
            bridge_seq_num,
            ctx,
        );

        // verify value change and claim events
        let token_value = token.value();
        assert!(
            total_supply_before + token_value == get_total_supply<T>(&bridge),
        );
        let claimed = event::events_by_type<TokenTransferClaimed>();
        let already_claimed = event::events_by_type<
            TokenTransferAlreadyClaimed,
        >();
        let limit_exceeded = event::events_by_type<TokenTransferLimitExceed>();
        assert!(
            claimed.length() == 1 || already_claimed.length() == 1 ||
            limit_exceeded.length() == 1,
        );
        let key = if (claimed.length() == 1) {
            claimed[0].transfer_claimed_key()
        } else if (already_claimed.length() == 1) {
            already_claimed[0].transfer_already_claimed_key()
        } else {
            limit_exceeded[0].transfer_limit_exceed_key()
        };
        let (sc, mt, sn) = key.unpack_message();
        assert!(source_chain == sc);
        assert!(mt == message_types::token());
        assert!(sn == bridge_seq_num);

        // tear down
        test_scenario::return_shared(bridge);
        token
    }

    // Claim a token and transfer to the receiver in the bridge message
    public fun claim_and_transfer_token<T>(
        env: &mut BridgeEnv,
        source_chain: u8,
        bridge_seq_num: u64,
    ): u8 {
        // set up
        let sender = @0xA1B2C3; // random sender
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let clock = &env.clock;
        let mut bridge = scenario.take_shared<Bridge>();
        let ctx = scenario.ctx();
        let total_supply_before = get_total_supply<T>(&bridge);

        // run claim and transfer
        bridge.claim_and_transfer_token<T>(
            clock,
            source_chain,
            bridge_seq_num,
            ctx,
        );

        // verify claim events
        let claimed = event::events_by_type<TokenTransferClaimed>();
        let already_claimed = event::events_by_type<
            TokenTransferAlreadyClaimed,
        >();
        let limit_exceeded = event::events_by_type<TokenTransferLimitExceed>();
        assert!(
            claimed.length() == 1 || already_claimed.length() == 1 ||
            limit_exceeded.length() == 1,
        );
        let (key, claim_status) = if (claimed.length() == 1) {
            (claimed[0].transfer_claimed_key(), CLAIMED)
        } else if (already_claimed.length() == 1) {
            (already_claimed[0].transfer_already_claimed_key(), ALREADY_CLAIMED)
        } else {
            (limit_exceeded[0].transfer_limit_exceed_key(), LIMIT_EXCEEDED)
        };
        let (sc, mt, sn) = key.unpack_message();
        assert!(source_chain == sc);
        assert!(mt == message_types::token());
        assert!(sn == bridge_seq_num);

        // verify effects
        let effects = scenario.next_tx(@0xABCDEF);
        let created = effects.created();
        if (!created.is_empty()) {
            let token_id = effects.created()[0];
            let token = scenario.take_from_sender_by_id<Coin<T>>(token_id);
            let token_value = token.value();
            assert!(
                total_supply_before + token_value ==
                get_total_supply<T>(&bridge),
            );
            scenario.return_to_sender(token);
        };

        // tear down
        test_scenario::return_shared(bridge);
        claim_status
    }

    public fun verify_bitcoin_signatures<T>(
        env: &mut BridgeEnv,
        sender: address,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
        signatures: vector<u8>,
    ):bool{
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let bridge = scenario.take_shared<Bridge>();
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let suc=treasury.verify_bitcoin_signatures<T>(source_chain, source_address, target_address, amount, tx_hash, signatures);
        test_scenario::return_shared(bridge);
        suc
    }

    public fun pre_deposit_external_coin_for_testing<T>(
        env: &mut BridgeEnv,
        sender: address,
        source_chain: u8,
        source_address:vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
        signatures: vector<u8>,
     ) {
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();

        bridge.pre_deposit_external_coin<T>(
            source_chain,
            source_address,
            target_address,
            amount, tx_hash,
            signatures,
            scenario.ctx(),
        );

        test_scenario::return_shared(bridge);
    }

    public fun deposit_external_coin_for_testing<T>(
        bridge: &mut Bridge,
        source_chain: u8,
        source_address:vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
        signatures: vector<u8>,
        ctx: &mut TxContext
     ) {
        bridge.deposit_external_coin<T>(
            source_chain,
            source_address,
            target_address,
            amount, tx_hash,
            signatures,
            ctx,
        );
    }

    public fun withdraw_external_coin_for_testing<T>(
        bridge: &mut Bridge,
        target_chain: u8,
        target_address: vector<u8>,
        token: Coin<T>,
        ctx: &mut TxContext
     ) {
        bridge.withdraw_external_coin<T>(
            target_chain,
            target_address,
            token,
            ctx,
        );
     }

    public fun deposit_and_withdraw_external_coin<T>(
        env: &mut BridgeEnv,
        sender: address,
        source_chain: u8,
        target_chain: u8,
        source_address: vector<u8>, //btc address
        target_address: vector<u8>, //benfen address
        amount: u64,
        signatures: vector<u8>,
        tx_hash: ascii::String,
    ) {
        // set up
        let token_type = env.token_type<T>();
        let message = message::create_token_bridge_message(
            source_chain,
            0,
            source_address,
            env.chain_id,
            target_address,
            token_type,
            amount,
            *tx_hash.as_bytes(),
            0u8, // event_idx
        );

        let node_signatures = env.sign_message(message);

        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        let total_supply_before = get_total_supply<T>(&bridge);
        let coin_type = type_name::into_string(type_name::get<T>());

        // deposit coin
        deposit_external_coin_for_testing<T>(
            &mut bridge,
            source_chain,
            source_address,
            target_address,
            amount, tx_hash,
            signatures,
            scenario.ctx(),
        );

        bridge.approval_and_claimed_external_coin<T>(message, node_signatures, scenario.ctx());
        let approved = event::events_by_type<ExternalDepositedApprovedEvent>();
        let deposited = event::events_by_type<ExternalDepositedEventV2>();
        assert!(approved.length() == 0 && deposited.length() == 1);
        {
            let (
                tx_hash,
                token_type,
                source_chain,
                target_chain,
                source_address,
                target_address,
                amount_before_fee,
                amount_after_fee,
            ) = deposited[0].unwrap_external_deposited_event_v2();
            assert!(
                tx_hash == tx_hash &&
                token_type == token_type &&
                source_chain == source_chain &&
                target_chain == env.chain_id &&
                source_address == source_address &&
                target_address == target_address &&
                amount == amount_before_fee,
            );
        };

        let deposited = event::events_by_type<ExternalDepositedEventV2>();
        assert!(deposited.length() == 1);
        debug::print(&deposited);
        let (
            event_tx_hash,
            token_type,
            event_source_chain,
            event_target_chain,
            event_source_address,
            event_target_address,
            event_amount_before_fee,
            event_amount_after_fee,
        ) = deposited[0].unwrap_external_deposited_event_v2();
        assert!(event_tx_hash == tx_hash);
        assert!(token_type == token_type);
        assert!(event_source_chain == source_chain);
        assert!(event_target_chain == target_chain);
        assert!(event_source_address == source_address);
        assert!(event_target_address == target_address);
        assert!(event_amount_before_fee == amount);
        assert_external_records(&bridge, source_chain, target_chain, source_address, target_address, amount, tx_hash);

        scenario.next_tx(sender);
        let token = scenario.take_from_address<Coin<T>>(address::from_bytes(target_address));
        // verify value change and claim events
        let token_value = token.value();
        assert!(token.balance().value() == amount);
        assert!(
            total_supply_before + token_value  == get_total_supply<T>(&bridge),
        );

        // withdraw coin
        withdraw_external_coin_for_testing<T>(
            &mut bridge,
            source_chain,
            source_address,
            token,
            scenario.ctx(),
        );

        let fee=get_cross_out_fee_amount<T>(&bridge,source_chain as u64,amount);
        assert!(amount>fee,1);

        let withdraw = event::events_by_type<ExternalWithdrawEventV2>();
        assert!(withdraw.length() == 1);
        debug::print(&withdraw);
        let (
            event_token_type,
            event_source_chain,
            event_target_chain,
            event_source_address,
            event_target_address,
            event_amount_before_fee,
            event_amount_after_fee,
        ) = withdraw[0].unwrap_external_withdrawn_v2_event();
        assert!(event_token_type == token_type);
        assert!(event_source_chain == target_chain );
        assert!(event_target_chain == source_chain);
        assert!(event_source_address == sender.to_bytes());
        assert!(event_target_address == source_address);
        assert!(event_amount_before_fee == amount);
        assert!(event_amount_after_fee==amount-fee);
        assert!(
            total_supply_before + fee == get_total_supply<T>(&bridge),
        );

        // tear down
        test_scenario::return_shared(bridge);
    }

    public fun env_get_external_token_transfer_action_status(
        env: &mut BridgeEnv,
        source_chain: u8,
        source_address: vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
        sender: address,
    ): u8{
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let bridge = scenario.take_shared<Bridge>();
        let status =test_get_external_token_transfer_action_status(&bridge,source_chain, source_address, target_address, amount, tx_hash);
        test_scenario::return_shared(bridge);
        status
    }

    public fun env_get_available_claim_amount<T>(
        env: &mut BridgeEnv,
        source_chain: u8,
        sender: address,
    ): u128{
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let bridge = scenario.take_shared<Bridge>();
        let limit=get_available_claim_amount<T>(
            &bridge,
            source_chain,
        );
        test_scenario::return_shared(bridge);
        limit
    }

    // Send a coin (token) to the target chain
    public fun send_token<T>(
        env: &mut BridgeEnv,
        sender: address,
        target_chain_id: u8,
        eth_address: vector<u8>,
        coin: Coin<T>,
    ): u64 {
        // set up
        let chain_id = env.chain_id;
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        let coin_value = coin.value();
        let total_supply_before = get_total_supply<T>(&bridge);
        let seq_num = bridge.get_seq_num_for(message_types::token());

        // run send
        bridge.send_token(target_chain_id, eth_address, coin, scenario.ctx());
        // verify send events
        assert!(
            total_supply_before - coin_value == get_total_supply<T>(&bridge),
        );
        if (target_chain_id == chain_ids::solana_testnet() || target_chain_id == chain_ids::solana_mainnet()) {
            let deposited_events = event::events_by_type<TokenDepositedEventForSolanaV2>();
            assert!(deposited_events.length() == 1);
            let (
                event_seq_num,
                _event_source_chain,
                _event_sender_address,
                _event_target_chain,
                _event_target_address,
                _event_token_type,
                event_amount_before_fee,
                _event_amount_after_fee
            ) = deposited_events[0].unwrap_deposited_event_for_solana_v2();
            assert!(event_seq_num == seq_num);
            assert!(event_amount_before_fee == coin_value);
        } else {
            let deposited_events = event::events_by_type<TokenDepositedEventV2>();
            assert!(deposited_events.length() == 1);
            let (
                event_seq_num,
                _event_source_chain,
                _event_sender_address,
                _event_target_chain,
                _event_target_address,
                _event_token_type,
                event_amount_before_fee,
                _event_amount_after_fee
            ) = deposited_events[0].unwrap_deposited_event_v2();
            assert!(event_seq_num == seq_num);
            assert!(event_amount_before_fee == coin_value);
        };
        assert_key(chain_id, &bridge);

        // tear down
        test_scenario::return_shared(bridge);
        seq_num
    }

    // Send a coin (token) to the target chain
    public fun send_back_token(
        env: &mut BridgeEnv,
        sender: address,
        target_chain_id: u8,
        eth_address: vector<u8>,
        token_type: u64,
        amount: u64,
        tx_hash: vector<u8>,
    ): u64 {
        // set up
        let chain_id = env.chain_id;
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        let seq_num = bridge.get_seq_num_for(message_types::token());
        // run send
        bridge.send_back_token_v2(target_chain_id, eth_address,token_type, amount, tx_hash, 0u16, scenario.ctx());
        // verify send events
        if (target_chain_id == chain_ids::solana_testnet() || target_chain_id == chain_ids::solana_mainnet()) {
            let send_back_events = event::events_by_type<TokenSendBackEventForSolanaV2>();
            assert!(send_back_events.length() == 1);
            let (
                event_seq_num,
                _event_source_chain,
                _event_sender_address,
                _event_target_chain,
                _event_target_address,
                _event_token_type,
                event_amount,
                event_tx_hash,
            ) = send_back_events[0].unwrap_send_back_event_for_solana_v2();
            assert!(event_seq_num == seq_num);
            assert!(event_amount == amount);
            assert!(event_tx_hash == tx_hash);
        } else {
            let send_back_events = event::events_by_type<TokenSendBackEventV2>();
            assert!(send_back_events.length() == 1);
            let (
                event_seq_num,
                _event_source_chain,
                _event_sender_address,
                _event_target_chain,
                _event_target_address,
                _event_token_type,
                event_amount,
                event_tx_hash,
            ) = send_back_events[0].unwrap_send_back_event_v2();
            assert!(event_seq_num == seq_num);
            assert!(event_amount == amount);
            assert!(event_tx_hash == tx_hash);
        };
        assert_key(chain_id, &bridge);

        // tear down
        test_scenario::return_shared(bridge);
        seq_num
    }

    // Update the limit for a given route
    public fun update_bridge_limit(
        env: &mut BridgeEnv,
        sender: address,
        receiving_chain: u8,
        sending_chain: u8,
        limit: u64,
    ): u64 {
        // set up
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();

        // message signed
        let msg = message::create_update_bridge_limit_message(
            receiving_chain,
            bridge.get_seq_num_for(message_types::update_bridge_limit()),
            sending_chain,
            limit,
        );
        let signatures = env.sign_message(msg);

        // run limit update
        bridge.execute_system_message(msg, signatures);

        // verify limit events
        let limit_events = event::events_by_type<UpdateRouteLimitEvent>();
        assert!(limit_events.length() == 1);
        let event = limit_events[0];
        let (sc, rc, new_limit) = event.unpack_route_limit_event();
        assert!(sc == sending_chain);
        assert!(rc == receiving_chain);
        assert!(new_limit == limit);

        // tear down
        test_scenario::return_shared(bridge);
        new_limit
    }

    // Update a given asset price (notional value)
    public fun update_asset_price(
        env: &mut BridgeEnv,
        sender: address,
        token_id: u64,
        value: u64,
    ) {
        // set up
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();

        // message signed
        let message = message::create_update_asset_price_message(
            token_id,
            env.chain_id,
            bridge.get_seq_num_for(message_types::update_asset_price()),
            value,
        );
        let signatures = env.sign_message(message);

        // run price update
        bridge.execute_system_message(message, signatures);

        // verify price events
        let update_events = event::events_by_type<UpdateTokenPriceEvent>();
        assert!(update_events.length() == 1);
        let (event_token_id, event_new_price) = update_events[
            0
        ].unwrap_update_event();
        assert!(event_token_id == token_id);
        assert!(event_new_price == value);

        // tear down
        test_scenario::return_shared(bridge);
    }

    // Register the `TEST_TOKEN` token
    public fun register_test_token(env: &mut BridgeEnv) {
        // set up
        let scenario = &mut env.scenario;
        scenario.next_tx(@0x0);
        let mut bridge = scenario.take_shared<Bridge>();

        // "create" the `Coin`
        let (
            upgrade_cap,
            treasury_cap,
            metadata,
        ) = test_token::create_bridge_token(scenario.ctx());
        // register the coin/token with the bridge
        bridge.register_foreign_token<TEST_TOKEN>(
            treasury_cap,
            upgrade_cap,
            &metadata,
        );

        // verify registration events
        let register_events = event::events_by_type<TokenRegistrationEvent>();
        assert!(register_events.length() == 1);
        let (type_name, decimal, nat) = register_events[
            0
        ].unwrap_registration_event();
        assert!(type_name == type_name::get<TEST_TOKEN>());
        assert!(decimal == 8);
        assert!(nat == false);

        // tear down
        destroy(metadata);
        test_scenario::return_shared(bridge);
    }

    // Add a list of tokens to the bridge.
    public fun add_tokens(
        env: &mut BridgeEnv,
        sender: address,
        native_token: bool,
        token_ids: vector<u64>,
        type_names: vector<String>,
        token_prices: vector<u64>,
    ) {
        // set up
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();

        // message signed
        let message = create_add_tokens_on_sui_message(
            env.chain_id,
            bridge.get_seq_num_for(message_types::add_tokens_on_sui()),
            native_token,
            token_ids,
            type_names,
            token_prices,
        );
        let signatures = env.sign_message(message);

        // run token addition
        bridge.execute_system_message(message, signatures);

        // verify token addition events
        let new_tokens_events = event::events_by_type<NewTokenEvent>();
        assert!(new_tokens_events.length() <= token_ids.length());

        // tear down
        test_scenario::return_shared(bridge);
    }

    // Blocklist a list of bridge nodes
    public fun execute_blocklist(
        env: &mut BridgeEnv,
        sender: address,
        chain_id: u8,
        blocklist_type: u8,
        validator_ecdsa_addresses: vector<vector<u8>>,
    ) {
        // set up
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();

        // message signed
        let blocklist = create_blocklist_message(
            chain_id,
            bridge.get_seq_num_for(message_types::committee_blocklist()),
            blocklist_type,
            validator_ecdsa_addresses,
        );
        let signatures = env.sign_message(blocklist);

        // run blocklist
        bridge.execute_system_message(blocklist, signatures);

        // verify blocklist events
        let block_list_events = event::events_by_type<
            BlocklistValidatorEvent,
        >();
        assert!(
            block_list_events.length() == validator_ecdsa_addresses.length(),
        );

        // tear down
        test_scenario::return_shared(bridge);
    }

    // Register new token
    public fun register_foreign_token<T>(
        env: &mut BridgeEnv,
        treasury_cap: TreasuryCap<T>,
        upgrade_cap: UpgradeCap,
        metadata: CoinMetadata<T>,
        sender: address,
    ) {
        // set up
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();

        // run registration
        bridge.register_foreign_token<T>(treasury_cap, upgrade_cap, &metadata);

        // verify registration events
        let register_events = event::events_by_type<TokenRegistrationEvent>();
        assert!(register_events.length() == 1);

        // verify changes in bridge
        let type_name = type_name::get<T>();
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let waiting_room = treasury.waiting_room();
        assert!(waiting_room.contains(type_name::into_string(type_name)));
        let treasuries = treasury.treasuries();
        assert!(treasuries.contains(type_name));

        // tear down
        test_scenario::return_shared(bridge);
        destroy(metadata);
    }

    // Freeze the bridge
    public fun freeze_bridge(env: &mut BridgeEnv, sender: address, error: u64) {
        // set up
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        let seq_num = bridge.get_seq_num_for(message_types::emergency_op());

        // message signed
        let msg = message::create_emergency_op_message(
            env.chain_id,
            seq_num,
            emergency_op_pause(),
        );
        let signatures = env.sign_message(msg);

        // run freeze
        bridge.execute_system_message(msg, signatures);

        // verify freeze events
        let register_events = event::events_by_type<EmergencyOpEvent>();
        assert!(register_events.length() == 1);
        assert!(register_events[0].unwrap_emergency_op_event() == true);

        // verify freeze
        let inner = bridge.test_load_inner_mut();
        inner.assert_paused(error);

        // tear down
        test_scenario::return_shared(bridge);
    }

    // Unfreeze the bridge
    public fun unfreeze_bridge(
        env: &mut BridgeEnv,
        sender: address,
        error: u64,
    ) {
        // set up
        let scenario = env.scenario();
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        let seq_num = bridge.get_seq_num_for(message_types::emergency_op());

        // message signed
        let msg = message::create_emergency_op_message(
            env.chain_id,
            seq_num,
            emergency_op_unpause(),
        );
        let signatures = env.sign_message(msg);

        // run unfreeze
        bridge.execute_system_message(msg, signatures);
        let register_events = event::events_by_type<EmergencyOpEvent>();
        assert!(register_events.length() == 1);
        assert!(register_events[0].unwrap_emergency_op_event() == false);

        // verify unfreeze events

        // verify unfreeze
        let inner = bridge.test_load_inner_mut();
        inner.assert_not_paused(error);

        // tear down
        test_scenario::return_shared(bridge);
    }

    //
    // Getters
    //

    public fun ctx(env: &mut BridgeEnv): &mut TxContext {
        env.scenario.ctx()
    }

    public fun scenario(env: &mut BridgeEnv): &mut Scenario {
        &mut env.scenario
    }

    public fun chain_id(env: &mut BridgeEnv): u8 {
        env.chain_id
    }

    public fun validators(env: &BridgeEnv): &vector<ValidatorInfo> {
        &env.validators
    }

    public fun get_btc(env: &mut BridgeEnv, amount: u64): Coin<BTC> {
        let scenario = &mut env.scenario;
        let ctx = scenario.ctx();
        env.vault.btc_coins.split(amount, ctx)
    }

    public fun get_eth(env: &mut BridgeEnv, amount: u64): Coin<ETH> {
        let scenario = &mut env.scenario;
        let ctx = scenario.ctx();
        env.vault.eth_coins.split(amount, ctx)
    }

    public fun get_usdc(env: &mut BridgeEnv, amount: u64): Coin<USDC> {
        let scenario = &mut env.scenario;
        let ctx = scenario.ctx();
        env.vault.usdc_coins.split(amount, ctx)
    }

    public fun get_usdt(env: &mut BridgeEnv, amount: u64): Coin<USDT> {
        let scenario = &mut env.scenario;
        let ctx = scenario.ctx();
        env.vault.usdt_coins.split(amount, ctx)
    }

    public fun get_bnb(env: &mut BridgeEnv, amount: u64): Coin<BNB> {
        let scenario = &mut env.scenario;
        let ctx = scenario.ctx();
        env.vault.bnb_coins.split(amount, ctx)
    }

    public fun limits(env: &mut BridgeEnv, dest: u8): u64 {
        let scenario = env.scenario();
        scenario.next_tx(@0x0);
        let bridge = scenario.take_shared<Bridge>();
        let route = chain_ids::get_route(dest, env.chain_id);
        let limits = bridge
            .test_load_inner()
            .inner_limiter()
            .get_route_limit(&route);
        test_scenario::return_shared(bridge);
        limits
    }

    fun assert_key(chain_id: u8, bridge: &Bridge) {
        let inner = bridge.test_load_inner();
        let transfer_record = inner.inner_token_transfer_records();
        let seq_num = inner.sequence_nums()[&message_types::token()] - 1;
        let key = message::create_key(
            chain_id,
            message_types::token(),
            seq_num,
        );
        assert!(transfer_record.contains(key));
    }

    fun assert_external_records(
        bridge: &Bridge,
        source_chain: u8,
        target_chain: u8,
        source_address:vector<u8>,
        target_address: vector<u8>,
        amount: u64,
        tx_hash: ascii::String,
    ) {
        let record = bridge.find_external_bridge_record(
            source_chain,
            source_address,
            target_address,
            amount,
            tx_hash,
        );
        assert!(record.is_some());
        let r: ExternalBridgeRecord = record.destroy_some();
        let (
            r_source_chain,
            r_target_chain,
            r_source_address,
            r_target_address,
            r_amount,
        ) = r.unwrap_external_bridge_record();
        assert!(r_source_chain == source_chain);
        assert!(r_target_chain == target_chain);
        assert!(r_source_address == source_address);
        assert!(r_target_address == target_address);
        assert!(r_amount == amount);
    }

    //
    // Internal functions
    //

    // Destroy the vault
    fun destroy_valut(vault: Vault) {
        let Vault {
            btc_coins,
            eth_coins,
            usdc_coins,
            usdt_coins,
            test_coins,
            bnb_coins,
            op_coins
        } = vault;
        btc_coins.burn_for_testing();
        eth_coins.burn_for_testing();
        usdc_coins.burn_for_testing();
        usdt_coins.burn_for_testing();
        test_coins.burn_for_testing();
        bnb_coins.burn_for_testing();
        op_coins.burn_for_testing();
    }

    // Load the vault with some coins
    fun load_vault(env: &mut BridgeEnv, sender: address) {
        let scenario = &mut env.scenario;
        scenario.next_tx(sender);
        let mut bridge = scenario.take_shared<Bridge>();
        let vault = &mut env.vault;
        vault.btc_coins.join(mint_some(&mut bridge, scenario.ctx()));
        vault.eth_coins.join(mint_some(&mut bridge, scenario.ctx()));
        vault.usdc_coins.join(mint_some(&mut bridge, scenario.ctx()));
        vault.usdt_coins.join(mint_some(&mut bridge, scenario.ctx()));
        vault.bnb_coins.join(mint_some(&mut bridge, scenario.ctx()));
        test_scenario::return_shared(bridge);
    }

    // Mint some coins
    fun mint_some<T>(bridge: &mut Bridge, ctx: &mut TxContext): Coin<T> {
        let treasury = bridge.test_load_inner_mut().inner_treasury_mut();
        let coin = treasury.mint<T>(1_000_000_000, ctx);
        coin
    }

    fun get_total_supply<T>(bridge: &Bridge): u64 {
        let inner = bridge.test_load_inner();
        let treasury = inner.inner_treasury();
        let treasuries = treasury.treasuries();
        let tc: &TreasuryCap<T> = &treasuries[type_name::get<T>()];
        tc.total_supply()
    }
}

//
// Test Coins
//

#[test_only]
module bridge::test_token {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct TEST_TOKEN has drop {}

    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<TEST_TOKEN>, CoinMetadata<TEST_TOKEN>) {
        let otw = create_one_time_witness<TEST_TOKEN>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            8,
            b"tst",
            b"test",
            b"bridge test token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<TEST_TOKEN>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}

#[test_only]
module bridge::btc {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct BTC has drop {}

    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<BTC>, CoinMetadata<BTC>) {
        let otw = create_one_time_witness<BTC>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            8,
            b"btc",
            b"bitcoin",
            b"bridge bitcoin token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<BTC>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}

#[test_only]
module bridge::eth {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct ETH has drop {}

    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<ETH>, CoinMetadata<ETH>) {
        let otw = create_one_time_witness<ETH>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            8,
            b"eth",
            b"eth",
            b"bridge ethereum token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<ETH>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}

#[test_only]
module bridge::usdc {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct USDC has drop {}

    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<USDC>, CoinMetadata<USDC>) {
        let otw = create_one_time_witness<USDC>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            6,
            b"usdc",
            b"usdc",
            b"bridge usdc token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<USDC>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}

#[test_only]
module bridge::usdt {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct USDT has drop {}

    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<USDT>, CoinMetadata<USDT>) {
        let otw = create_one_time_witness<USDT>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            6,
            b"usdt",
            b"usdt",
            b"bridge usdt token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<USDT>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}

#[test_only]
module bridge::bnb {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct BNB has drop {}

    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<BNB>, CoinMetadata<BNB>) {
        let otw = create_one_time_witness<BNB>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            8,
            b"BNB",
            b"BSC",
            b"bridge BSC token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<BNB>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}


#[test_only]
module bridge::op {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct OP has drop {}

    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<OP>, CoinMetadata<OP>) {
        let otw = create_one_time_witness<OP>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            8,
            b"OP",
            b"OP",
            b"bridge OP token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<OP>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}

#[test_only]
module bridge::busd {
    use std::ascii;
    use std::type_name;
    use sui::address;
    use sui::coin::{CoinMetadata, TreasuryCap, create_currency};
    use sui::hex;
    use sui::package::{UpgradeCap, test_publish};
    use sui::test_utils::create_one_time_witness;

    public struct BUSD has drop {}


    public fun create_bridge_token(
        ctx: &mut TxContext,
    ): (UpgradeCap, TreasuryCap<BUSD>, CoinMetadata<BUSD>) {
        let otw = create_one_time_witness<BUSD>();
        let (treasury_cap, metadata) = create_currency(
            otw,
            9,
            b"BUSD",
            b"BUSD",
            b"bridge BUSD token",
            option::none(),
            ctx,
        );

        let type_name = type_name::get<BUSD>();
        let address_bytes = hex::decode(
            ascii::into_bytes(type_name::get_address(&type_name)),
        );
        let coin_id = address::from_bytes(address_bytes).to_id();
        let upgrade_cap = test_publish(coin_id, ctx);

        (upgrade_cap, treasury_cap, metadata)
    }
}
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::bridge_tests;
use bridge::bridge::{
    inner_limiter,
    inner_paused,
    inner_treasury,
    inner_token_transfer_records_mut,
    new_bridge_record_for_testing,
    new_for_testing,
    test_get_current_seq_num_and_increment,
    test_execute_update_asset_price,
    test_get_token_transfer_action_signatures,
    test_load_inner,
    test_load_inner_mut,
    test_get_token_transfer_action_status,
    transfer_status_approved,
    transfer_status_claimed,
    transfer_status_not_found,
    transfer_status_pending,
    Bridge
};
use bridge::bridge_env::{
    pre_deposit_external_coin_for_testing,
    deposit_external_coin_for_testing,
    withdraw_external_coin_for_testing,
    btc_id,
    create_bridge,
    create_bridge_default,
    create_env,
    create_validator,
    eth_id,
    freeze_bridge,
    init_committee,
    register_committee,
    unfreeze_bridge,
    test_token_id,
    chain_id,
    token_type,
    sign_message_with
};
use bridge::btc::BTC;
use bridge::chain_ids;
use bridge::eth::ETH;
use bridge::message::{Self, to_parsed_token_transfer_message_v2};
use bridge::message_types;
use bridge::test_token::{TEST_TOKEN, create_bridge_token as create_test_token};
use bridge::usdc::USDC;
use std::type_name;
use sui::address;
use sui::balance;
use sui::coin::{Self, Coin};
use sui::hex;
use sui::package::test_publish;
use sui::test_scenario;
use sui::test_utils::destroy;
use std::ascii;
use sui::ecdsa_k1;
use sui::hash;
use std::unit_test::assert_eq;
use bfc_system::bfc_system::BfcSystemState;
use bfc_system::bfc_system;
use bfc_system::bfc_system_tests::public_setup;
use bfc_system::busd::BUSD;
use bridge::busd;


use bfc_system::bfc_system_state_inner::BfcSystemModifyCap;
use bridge::bridge_env::get_usdc;
use sui::event;

// common error start code for unexpected errors in tests (assertions).
// If more than one assert in a test needs to use an unexpected error code,
// use this as the starting error and add 1 to subsequent errors
const UNEXPECTED_ERROR: u64 = 10293847;
// use on tests that fail to save cleanup
const TEST_DONE: u64 = 74839201;

const MINT_BUSD_RIGHT_KEY: vector<u8> = b"MINT-BUSD-right_key";
const STAKE: u8 = 0;
const UNSTAKE: u8 = 1;

#[test]
fun test_bridge_create() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge(@0x0);

    let bridge = env.bridge(@0x0);
    let inner = bridge.bridge_ref().test_load_inner();
    inner.assert_not_paused(UNEXPECTED_ERROR);
    assert!(inner.inner_token_transfer_records().length() == 0);
    bridge.return_bridge();

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::ENotSystemAddress)]
fun test_bridge_create_non_system_addr() {
    let mut env = create_env(chain_ids::sui_mainnet());
    env.create_bridge(@0x1);

    abort TEST_DONE
}

#[test]
fun test_create_bridge_default() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    env.destroy_env();
}

#[test]
fun test_init_committee_twice() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.init_committee(@0x0); // second time is a no-op

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::ENotSystemAddress)]
fun test_init_committee_non_system_addr() {
    let mut env = create_env(chain_ids::sui_mainnet());
    env.setup_validators(vector[
        create_validator(@0xA, 100, &b"12345678901234567890123456789012"),
    ]);
    env.create_bridge(@0x0);
    env.register_committee();
    env.init_committee(@0xA);

    abort TEST_DONE
}

#[test]
#[expected_failure(abort_code = bridge::committee::ECommitteeAlreadyInitiated)]
fun test_register_committee_after_init() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    env.register_committee();

    abort TEST_DONE
}

#[test]
fun test_register_foreign_token() {
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let (upgrade_cap, treasury_cap, metadata) = create_test_token(env
        .scenario()
        .ctx());
    env.register_foreign_token<TEST_TOKEN>(
        treasury_cap,
        upgrade_cap,
        metadata,
        addr,
    );
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::treasury::ETokenSupplyNonZero)]
fun test_register_foreign_token_non_zero_supply() {
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let
    (upgrade_cap, mut
    treasury_cap,
    metadata) =
    create_test_token(env
        .scenario()
        .ctx());
    let _coin = treasury_cap.mint(1, env.scenario().ctx());
    env.register_foreign_token<TEST_TOKEN>(
        treasury_cap,
        upgrade_cap,
        metadata,
        addr,
    );

    abort 0
}

#[test]
#[expected_failure(abort_code = bridge::treasury::EInvalidNotionalValue)]
fun test_add_token_price_zero_value() {
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.add_tokens(
        addr,
        false,
        vector[test_token_id()],
        vector[type_name::get<TEST_TOKEN>().into_string()],
        vector[0],
    );

    abort 0
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EMalformedMessageError)]
fun test_add_token_malformed_1() {
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.add_tokens(
        addr,
        false,
        vector[test_token_id(), eth_id()],
        vector[type_name::get<TEST_TOKEN>().into_string()],
        vector[10],
    );

    abort 0
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EMalformedMessageError)]
fun test_add_token_malformed_2() {
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.add_tokens(
        addr,
        false,
        vector[test_token_id()],
        vector[
            type_name::get<TEST_TOKEN>().into_string(),
            type_name::get<BTC>().into_string(),
        ],
        vector[10],
    );

    abort 0
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EMalformedMessageError)]
fun test_add_token_malformed_3() {
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.add_tokens(
        addr,
        false,
        vector[test_token_id()],
        vector[type_name::get<TEST_TOKEN>().into_string()],
        vector[10, 20],
    );

    abort 0
}

#[test]
fun test_add_native_token_nop() {
    // adding a native token is simply a NO-OP at the moment
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.add_tokens(
        addr,
        true,
        vector[test_token_id()],
        vector[type_name::get<TEST_TOKEN>().into_string()],
        vector[100],
    );
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::treasury::EInvalidUpgradeCap)]
fun test_register_foreign_token_bad_upgrade_cap() {
    let addr = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let (_upgrade_cap, treasury_cap, metadata) = create_test_token(env
        .scenario()
        .ctx());
    let upgrade_cap = test_publish(@0x42.to_id(), env.scenario().ctx());
    env.register_foreign_token<TEST_TOKEN>(
        treasury_cap,
        upgrade_cap,
        metadata,
        addr,
    );

    abort 0
}

#[test]
fun test_execute_send_token() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let btc: Coin<ETH> = env.get_eth(1);
    let eth_address = x"0000000000000000000000000000000000000000";
    env.send_token(@0xABCD, chain_ids::eth_sepolia(), eth_address, btc);
    env.destroy_env();
}

#[test]
fun test_btc_bridge_v2() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let source_address = b"bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
    let sender = @0xA;
    let target_address = address::to_bytes(sender);

    env.bridge_external_coin_v2<BTC>(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        ascii::string(b"ddd"),
        100_000_000,
    );

    env.destroy_env();
}

#[test]
fun test_btc_bridge_add_remove_external_coin_admin() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();

    env.add_external_coin_admin(coin_type, @0xABCD1.to_ascii_string());
    env.add_external_coin_admin(coin_type, @0xABCD2.to_ascii_string());
    env.add_external_coin_admin(coin_type, @0xABCD3.to_ascii_string());
    env.add_external_coin_admin(coin_type, @0xABCD4.to_ascii_string());

    env.remove_external_coin_admin(coin_type, @0xABCD1.to_ascii_string());
    env.remove_external_coin_admin(coin_type, @0xABCD2.to_ascii_string());
    env.remove_external_coin_admin(coin_type, @0xABCD3.to_ascii_string());
    env.remove_external_coin_admin(coin_type, @0xABCD4.to_ascii_string());

    env.remove_external_coin_admin(coin_type, @0xABCD5.to_ascii_string());


    env.destroy_env();
}

fun mock_bitcoin_message(): (vector<u8>, vector<u8>, vector<u8>, vector<u8>, vector<u8>, u64) {
    let witness = x"17CcfCeD39fF9e7818DF026fB2df9b8ca2f6424f";
    let private_key = x"4f0adab8fe9f36875f6b7f28d9679c37ab2c96224e50224b5bda5add5b1ee7bb";
    let source_address = b"tb1pxafm6dv7rj8x8st44n64f58nuy5r9vaplvfgdy747gdeug7xcvuqx98ude";
    let tx_hash = b"ff8305c804598c3afadb63611b4af1cd0e60fc9adef6f82b991789982b9bd712";
    let target_address = x"0255c0bd6eea8ea62db08f2d7d209858115c6e555e306ccb9e8b443f6e1f7729";
    let amount = 1*100_000_000;
    (witness, private_key, source_address, tx_hash, target_address, amount)
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnpassedMultiSignature)]
fun test_btc_bridge_deposit_without_multi_signature() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender = @0xABCD;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender.to_ascii_string());
    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string()
    );

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnpassedMultiSignature)]
fun test_btc_bridge_deposit_with_insufficient_multi_signature() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender1 = @0xA;
    let sender2 = @0xB;
    let sender3 = @0xC;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender1.to_ascii_string());
    env.add_external_coin_admin(coin_type, sender2.to_ascii_string());
    env.add_external_coin_admin(coin_type, sender3.to_ascii_string());


    env.pre_deposit_external_coin_for_testing<BTC>(
        sender1,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender1,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender1,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.deposit_and_withdraw_external_coin<BTC>(
        sender1,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.destroy_env();
}

#[test]
fun test_verify_bitcoin_signatures() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let sender2 = @0xB;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    let suc = env.verify_bitcoin_signatures<BTC>(
        sender2,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures
    );
    assert!(suc);
    env.destroy_env();
}

#[test]
fun test_remove_witness() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let sender1 = x"17CcfCeD39fF9e7818DF026fB2df9b8ca2f6424f";
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    env.add_external_coin_witness(coin_type, sender1);
    env.remove_external_coin_witness(coin_type, sender1);
    env.destroy_env();
}

#[test]
fun test_add_target_address() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let addr = b"n1sfLwoLTnLFxj2BT8kNETsLDM8xMecYn3";
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    env.add_external_coin_target(coin_type, addr.to_ascii_string());
    env.destroy_env();
}

#[test]
fun test_remove_target_address() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let addr = b"n1sfLwoLTnLFxj2BT8kNETsLDM8xMecYn3";
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    env.add_external_coin_target(coin_type, addr.to_ascii_string());
    env.remove_external_coin_target(coin_type, addr.to_ascii_string());
    env.destroy_env();
}

#[test]
fun test_remove_witness_verify_bitcoin_signatures() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    env.add_external_coin_witness(coin_type, witness);
    env.remove_external_coin_witness(coin_type, witness);
    let sender2 = @0xB;
    let bitcoin_message = message::create_bitcoin_message(
        1,
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    let suc = env.verify_bitcoin_signatures<BTC>(
        sender2,
        1,
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures
    );
    assert!(!suc); //judge
    env.destroy_env();
}


#[test]
fun test_btc_bridge_deposit_with_sufficient_multi_signature() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender1 = @0xA;
    let sender2 = @0xB;
    let sender3 = @0xC;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender1.to_ascii_string());
    env.add_external_coin_admin(coin_type, sender2.to_ascii_string());
    env.add_external_coin_admin(coin_type, sender3.to_ascii_string());


    env.pre_deposit_external_coin_for_testing<BTC>(
        sender1,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender2,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.deposit_and_withdraw_external_coin<BTC>(
        sender1,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.destroy_env();
}

#[test]
fun test_get_external_token_transfer_action_not_found_status() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let source_addr = b"tb1pxafm6dv7rj8x8st44n64f58nuy5r9vaplvfgdy747gdeug7xcvuqx98ude";
    let target_addr = @0x1234;
    let sender = @0x87;
    assert!(env.env_get_external_token_transfer_action_status(
        chain_ids::btc_testnet(),
        source_addr,
        target_addr.to_bytes(),
        1*100_000_000,
        ascii::string(b""),
        sender,
    ) == transfer_status_not_found(), 0);
    env.destroy_env();
}

#[test]
fun test_get_external_token_transfer_action_claimed_status() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let sender = @0xABCD;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender.to_ascii_string());

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    assert!(env.env_get_external_token_transfer_action_status(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        sender,
    ) == transfer_status_claimed(), 0);


    env.destroy_env();
}

#[test]
fun test_btc_bridge_deposit_and_withdraw_external_btc() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender = @0xABCD;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender.to_ascii_string());

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );
    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EDuplicatedMessage)]
fun test_btc_bridge_recall_deposit_and_withdraw_external_btc() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender = @0xABCD;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender.to_ascii_string());

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );
    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnknownExternalCoinOrSender)]
fun test_btc_bridge_deposit_and_withdraw_external_btc_after_remove_admin_cap() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender = @0xABCD;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender.to_ascii_string());

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.remove_external_coin_admin(coin_type, sender.to_ascii_string());

    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnpassedWitnessSignature)]
fun test_btc_bridge_deposit_and_withdraw_external_btc_after_remove_witness_admin_cap() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender = @0xABCD;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);
    env.add_external_coin_admin(coin_type, sender.to_ascii_string());

    env.pre_deposit_external_coin_for_testing<BTC>(
        sender,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.remove_external_coin_witness(coin_type, witness);
    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnknownExternalCoinOrSender)]
fun test_btc_bridge_deposit_and_withdraw_external_btc_without_admin_cap() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);

    let sender = @0xABCD;
    env.deposit_and_withdraw_external_coin<BTC>(
        sender,
        chain_ids::btc_testnet(),
        chain_ids::sui_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        signatures,
        tx_hash.to_ascii_string(),
    );

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnknownExternalCoinOrSender)]
fun test_btc_bridge_deposit_external_btc_without_admin_cap() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<BTC>();
    let coin_type = type_name.into_string();
    let bitcoin_message = message::create_bitcoin_message(
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        amount,
        tx_hash,
        *type_name.into_string().as_bytes()
    );
    let msg = hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
    let signatures = ecdsa_k1::secp256k1_sign(&private_key, &msg, 0, true);
    env.add_external_coin_witness(coin_type, witness);

    let sender = @0xABCD;
    let mut bridge_wrap = env.bridge(sender);
    let bridge = bridge_wrap.bridge_ref_mut();
    deposit_external_coin_for_testing<BTC>(
        bridge,
        chain_ids::btc_testnet(),
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
        env.ctx(),
    );

    bridge_wrap.return_bridge();
    env.destroy_env();
}

#[test]
fun test_btc_bridge_withdraw_external_btc() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let btc: Coin<BTC> = env.get_btc(1*100_000_000);
    let target_address = x"0000000000000000000000000000000000000000000000000000000000000001";

    let sender = @0xABCD;
    let mut bridge_wrap = env.bridge(sender);
    let bridge = bridge_wrap.bridge_ref_mut();

    withdraw_external_coin_for_testing<BTC>(
        bridge,
        chain_ids::btc_testnet(),
        target_address,
        btc,
        env.ctx(),
    );

    bridge_wrap.return_bridge();
    env.destroy_env();
}

#[test]
fun test_execute_send_back_token() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let eth_address = x"0000000000000000000000000000000000000000";
    let tx_hash = hex::decode(b"56335bb5461b430c3ccf94efe91494e64f21e12e9b8b007b0e1c56c7d1e8de3b");
    let btc_id = 1;
    let btc_amount = 100;
    env.send_back_token(@0xABCD, chain_ids::eth_sepolia(), eth_address, btc_id, btc_amount, tx_hash);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidSender)]
fun test_execute_send_back_token_should_fail_if_not_refund_admin() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let eth_address = x"0000000000000000000000000000000000000000";
    let tx_hash = hex::decode(b"56335bb5461b430c3ccf94efe91494e64f21e12e9b8b007b0e1c56c7d1e8de3b");
    let btc_id = 1;
    let btc_amount = 100;
    env.send_back_token(@0xABC, chain_ids::eth_sepolia(), eth_address, btc_id, btc_amount, tx_hash);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidTxHash)]
fun test_execute_send_back_token_invalid_tx_hash() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let eth_address = x"0000000000000000000000000000000000000000";
    let tx_hash = hex::decode(b"");
    let btc_id = 1;
    let btc_amount = 100;
    env.send_back_token(@0x0, chain_ids::eth_sepolia(), eth_address, btc_id, btc_amount, tx_hash);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::ETokenValueIsZero)]
fun test_execute_send_token_zero_value() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let btc: Coin<BTC> = env.get_btc(0);
    let eth_address = x"0000000000000000000000000000000000000000";
    env.send_token(@0x0, chain_ids::eth_sepolia(), eth_address, btc);

    abort TEST_DONE
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidEvmAddress)]
fun test_execute_send_token_invalid_evem_address() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let btc: Coin<BTC> = env.get_btc(1);
    let eth_address = x"1234";
    let val_addr = env.validators()[0].addr();
    env.send_token(val_addr, chain_ids::eth_sepolia(), eth_address, btc);

    abort TEST_DONE
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EBridgeUnavailable)]
fun test_execute_send_token_frozen() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let eth: Coin<ETH> = env.get_eth(1);
    let eth_address = x"0000000000000000000000000000000000000000";
    env.freeze_bridge(@0x0, UNEXPECTED_ERROR);
    env.send_token(@0xAAAA, chain_ids::eth_sepolia(), eth_address, eth);

    abort TEST_DONE
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidBridgeRoute)]
fun test_execute_send_token_invalid_route() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let usdc: Coin<USDC> = env.get_usdc(100);
    let eth_address = x"0000000000000000000000000000000000000000";
    env.send_token(@0xABCDEF, chain_ids::eth_mainnet(), eth_address, usdc);

    abort TEST_DONE
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnexpectedChainID)]
fun test_system_msg_incorrect_chain_id() {
    let sender = @0x0;
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.execute_blocklist(sender, chain_ids::sui_mainnet(), 0, vector[]);

    abort TEST_DONE
}

#[test]
fun test_get_seq_num_and_increment() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = scenario.ctx();
    let chain_id = chain_ids::sui_testnet();
    let mut bridge = new_for_testing(chain_id, ctx);

    let inner = bridge.test_load_inner_mut();
    assert!(
        inner.test_get_current_seq_num_and_increment(
            message_types::committee_blocklist(),
        ) ==
            0,
    );
    assert!(
        inner.sequence_nums()[&message_types::committee_blocklist()] == 1,
    );
    assert!(
        inner.test_get_current_seq_num_and_increment(
            message_types::committee_blocklist(),
        ) ==
            1,
    );
    // other message type nonce does not change
    assert!(
        !inner.sequence_nums().contains(&message_types::token()),
    );
    assert!(
        !inner.sequence_nums().contains(&message_types::emergency_op()),
    );
    assert!(
        !inner.sequence_nums().contains(&message_types::update_bridge_limit()),
    );
    assert!(
        !inner.sequence_nums().contains(&message_types::update_asset_price()),
    );
    assert!(
        inner.test_get_current_seq_num_and_increment(message_types::token()) ==
            0,
    );
    assert!(
        inner.test_get_current_seq_num_and_increment(
            message_types::emergency_op(),
        ) ==
            0,
    );
    assert!(
        inner.test_get_current_seq_num_and_increment(
            message_types::update_bridge_limit(),
        ) ==
            0,
    );
    assert!(
        inner.test_get_current_seq_num_and_increment(
            message_types::update_asset_price(),
        ) ==
            0,
    );

    destroy(bridge);
    scenario.end();
}

#[test]
fun test_update_limit() {
    let chain_id = chain_ids::sui_mainnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();

    let bridge = env.bridge(@0x0);
    let inner = bridge.bridge_ref().test_load_inner();
    // Assert the starting limit is a different value
    assert!(
        inner
            .inner_limiter()
            .get_route_limit(
                &chain_ids::get_route(
                    chain_ids::eth_mainnet(),
                    chain_ids::sui_mainnet(),
                ),
            ) !=
            1,
    );
    bridge.return_bridge();

    // update limit
    env.update_bridge_limit(
        @0x0,
        chain_ids::sui_mainnet(),
        chain_ids::eth_mainnet(),
        1,
    );

    let bridge = env.bridge(@0x0);
    let inner = bridge.bridge_ref().test_load_inner();
    // Assert the starting limit is a different value
    assert!(
        inner
            .inner_limiter()
            .get_route_limit(
                &chain_ids::get_route(
                    chain_ids::eth_mainnet(),
                    chain_ids::sui_mainnet(),
                ),
            ) ==
            1,
    );
    // other routes are not impacted
    assert!(
        inner
            .inner_limiter()
            .get_route_limit(
                &chain_ids::get_route(
                    chain_ids::eth_sepolia(),
                    chain_ids::sui_testnet(),
                ),
            ) !=
            1,
    );
    bridge.return_bridge();

    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EUnexpectedChainID)]
fun test_execute_update_bridge_limit_abort_with_unexpected_chain_id() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    // This abort because the receiving_chain (sui_mainnet) is not the same as
    // the bridge's chain_id (sui_devnet)
    env.update_bridge_limit(
        @0x0,
        chain_ids::sui_mainnet(),
        chain_ids::eth_mainnet(),
        1,
    );

    abort TEST_DONE
}

#[test]
fun test_update_asset_price() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let scenario = env.scenario();
    scenario.next_tx(@0x0);
    let mut bridge = scenario.take_shared<Bridge>();
    let inner = bridge.test_load_inner_mut();

    // Assert the starting limit is a different value
    assert!(
        inner.inner_treasury().notional_value<BTC>() != 1_001_000_000,
    );
    // now change it to 100_001_000
    let msg = message::create_update_asset_price_message(
        inner.inner_treasury().token_id<BTC>(),
        chain_ids::sui_mainnet(),
        0,
        1_001_000_000,
    );
    let payload = msg.extract_update_asset_price();
    inner.test_execute_update_asset_price(payload);

    // should be 1_001_000_000 now
    assert!(inner.inner_treasury().notional_value<BTC>() == 1_001_000_000);
    // other assets are not impacted
    assert!(inner.inner_treasury().notional_value<ETH>() != 1_001_000_000);

    destroy(bridge);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::treasury::EInvalidNotionalValue)]
fun test_invalid_price_update() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.update_asset_price(@0x0, btc_id(), 0);

    abort 0
}

#[test]
#[expected_failure(abort_code = bridge::treasury::EUnsupportedTokenType)]
fun test_unsupported_token_type() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    env.update_asset_price(@0x0, 42, 100);

    abort 0
}

#[test]
fun test_execute_freeze_unfreeze() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.freeze_bridge(@0x0, UNEXPECTED_ERROR + 1);
    let bridge = env.bridge(@0x0);
    assert!(bridge.bridge_ref().test_load_inner().inner_paused());
    bridge.return_bridge();
    env.unfreeze_bridge(@0x0, UNEXPECTED_ERROR + 2);
    let bridge = env.bridge(@0x0);
    assert!(!bridge.bridge_ref().test_load_inner().inner_paused());
    bridge.return_bridge();
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EBridgeNotPaused)]
fun test_execute_unfreeze_err() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let bridge = env.bridge(@0x0);
    assert!(!bridge.bridge_ref().test_load_inner().inner_paused());
    bridge.return_bridge();
    env.unfreeze_bridge(@0x0, UNEXPECTED_ERROR + 2);

    abort TEST_DONE
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EBridgeAlreadyPaused)]
fun test_execute_emergency_op_abort_when_already_frozen() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();

    // initially it's unfrozen
    let bridge = env.bridge(@0x0);
    assert!(!bridge.bridge_ref().test_load_inner().inner_paused());
    bridge.return_bridge();
    // freeze it
    env.freeze_bridge(@0x0, UNEXPECTED_ERROR);
    let bridge = env.bridge(@0x0);
    assert!(bridge.bridge_ref().test_load_inner().inner_paused());
    bridge.return_bridge();
    // freeze it again, should abort
    env.freeze_bridge(@0x0, UNEXPECTED_ERROR);

    abort TEST_DONE
}

#[test]
fun test_get_token_transfer_action_data() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = scenario.ctx();
    let chain_id = chain_ids::sui_testnet();
    let mut bridge = new_for_testing(chain_id, ctx);
    let coin = coin::mint_for_testing<ETH>(12345, ctx);

    // Test when pending
    let message = message::create_token_bridge_message_v2(
        chain_ids::sui_testnet(), // source chain
        10, // seq_num
        address::to_bytes(ctx.sender()), // sender address
        chain_ids::eth_sepolia(), // target_chain
        hex::decode(
            b"00000000000000000000000000000000000000c8",
        ), // target_address
        1u64, // token_type
        coin.balance().value(),
        hex::decode(b""), // tx_hash
        0u16, // event_idx
    );

    let key = message.key();
    bridge
        .test_load_inner_mut()
        .inner_token_transfer_records_mut()
        .push_back(
            key,
            new_bridge_record_for_testing(message, option::none(), false),
        );
    assert!(
        bridge.test_get_token_transfer_action_status(chain_id, 10) ==
            transfer_status_pending(),
    );
    assert!(
        bridge.test_get_token_transfer_action_signatures(chain_id, 10) ==
            option::none(),
    );

    // Test when ready for claim
    let message = message::create_token_bridge_message_v2(
        chain_ids::sui_testnet(), // source chain
        11, // seq_num
        address::to_bytes(ctx.sender()), // sender address
        chain_ids::eth_sepolia(), // target_chain
        hex::decode(
            b"00000000000000000000000000000000000000c8",
        ), // target_address
        1u64, // token_type
        balance::value(coin::balance(&coin)),
        hex::decode(b""), // tx_hash
        0u16, // event_idx
    );
    let key = message.key();
    bridge
        .test_load_inner_mut()
        .inner_token_transfer_records_mut()
        .push_back(
            key,
            new_bridge_record_for_testing(
                message,
                option::some(vector[]),
                false,
            ),
        );
    assert!(
        bridge.test_get_token_transfer_action_status(chain_id, 11) ==
            transfer_status_approved(),
    );
    assert!(
        bridge.test_get_token_transfer_action_signatures(chain_id, 11) ==
            option::some(vector[]),
    );
    assert!(
        bridge.test_get_parsed_token_transfer_message_v2(chain_id, 11) ==
        option::some(
            to_parsed_token_transfer_message_v2(&message),
        ),
    );

    // Test when already claimed
    let message = message::create_token_bridge_message_v2(
        chain_ids::sui_testnet(), // source chain
        12, // seq_num
        address::to_bytes(ctx.sender()), // sender address
        chain_ids::eth_sepolia(), // target_chain
        hex::decode(
            b"00000000000000000000000000000000000000c8",
        ), // target_address
        1u64, // token_type
        balance::value(coin::balance(&coin)),
        hex::decode(b""), // tx_hash
        0u16, // event_idx
    );
    let key = message.key();
    bridge
        .test_load_inner_mut()
        .inner_token_transfer_records_mut()
        .push_back(
            key,
            new_bridge_record_for_testing(
                message,
                option::some(vector[b"1234"]),
                true,
            ),
        );
    assert!(
        bridge.test_get_token_transfer_action_status(chain_id, 12) ==
            transfer_status_claimed(),
    );
    assert!(
        bridge.test_get_token_transfer_action_signatures(chain_id, 12) ==
            option::some(vector[b"1234"]),
    );
    assert!(
        bridge.test_get_parsed_token_transfer_message_v2(chain_id, 12) ==
        option::some(
            to_parsed_token_transfer_message_v2(&message),
        ),
    );

    // Test when message not found
    assert!(
        bridge.test_get_token_transfer_action_status(chain_id, 13) ==
            transfer_status_not_found(),
    );
    assert!(
        bridge.test_get_token_transfer_action_signatures(chain_id, 13) ==
            option::none(),
    );
    assert!(
        bridge.test_get_parsed_token_transfer_message_v2(chain_id, 13) ==
        option::none(),
    );

    destroy(bridge);
    coin.burn_for_testing();
    scenario.end();
}

#[test]
#[expected_failure(abort_code = bridge::treasury::EUnsupportedTokenType)]
fun test_get_metadata_no_token() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let bridge = env.bridge(@0x0);
    let treasury = bridge.bridge_ref().test_load_inner().inner_treasury();
    treasury.notional_value<TEST_TOKEN>();

    abort 0
}

#[test]
fun change_url() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let mut bridge = env.bridge(@0xAAAA);
    bridge
        .bridge_ref_mut()
        .update_node_url(b"<url_here>", env.scenario().ctx());
    bridge.return_bridge();
    env.destroy_env();
}

#[test]
#[
expected_failure(
    abort_code = bridge::committee::ESenderIsNotInBridgeCommittee,
),
]
fun change_url_bad_sender() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let mut bridge = env.bridge(@0x0);
    bridge
        .bridge_ref_mut()
        .update_node_url(b"<url_here>", env.scenario().ctx());
    abort 0
}

#[test]
fun read_router_limit() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let bridge = env.bridge(@0x0);
    let limits = bridge.bridge_ref().test_load_limiter();
    let bsc_route = chain_ids::get_route(chain_ids::bsc_mainnet(), chain_ids::sui_mainnet());
    let op_route = chain_ids::get_route(chain_ids::op_mainnet(), chain_ids::sui_mainnet());
    let base_route = chain_ids::get_route(chain_ids::base_mainnet(), chain_ids::sui_mainnet());
    assert_eq!(limits.get_route_limit(&bsc_route), 1_000_000_000 * 100000000);
    assert_eq!(limits.get_route_limit(&op_route), 1_000_000_000 * 100000000);
    assert_eq!(limits.get_route_limit(&base_route), 1_000_000_000 * 100000000);
    bridge.return_bridge();
    env.destroy_env();
}

#[test]
#[
expected_failure(
    abort_code = bridge::tokenlist::EBridgeTokenListRegistryAlreadyExists,
),
]
fun test_twice_call_init_token_list() {
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let mut bridge = env.bridge(@0x0);
    let bridge_inner = bridge.bridge_ref_mut();
    bridge_inner.init_token_list(env.scenario().ctx());
    bridge.return_bridge();
    env.destroy_env();
}


#[test]
#[
expected_failure(
    abort_code = bridge::bridge_fee::EBridgeFeeRegistryAlreadyExists,
)]
fun test_twice_call_migrate(){
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let mut bridge = env.bridge(@0x0);
    let bridge_inner = bridge.bridge_ref_mut();
    bridge_inner.migrate(env.scenario().ctx());
    //bridge_inner.migrate(env.scenario().ctx());
    bridge.return_bridge();
    env.destroy_env();
}

#[test]
fun test_add_token_on_benfen(){
    let chain_id = chain_ids::sui_testnet();
    let target_id=chain_ids::aptos_testnet();
    let token_id=11; //aptos coin
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.add_token_on_token_list(chain_id,target_id, token_id);
    env.destroy_env();
}

#[test]
fun test_add_token_from_benfen(){
    let chain_id = chain_ids::sui_testnet();
    let source_chain=chain_ids::aptos_testnet();
    let token_id=11; //aptos coin
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.add_token_on_token_list(source_chain,chain_id, token_id);
    env.destroy_env();
}

#[test]
fun test_set_cross_in_bridge_fee_with_fixed(){
    let chain_id = chain_ids::sui_testnet();
    let source_chain=chain_ids::aptos_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.set_cross_in_bridge_fee<BTC>(source_chain,0,100,1_000_000_000);
    env.destroy_env();
}


#[test]
fun test_set_cross_in_bridge_fee_with_percentage(){
    let chain_id = chain_ids::sui_testnet();
    let source_chain=chain_ids::aptos_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.set_cross_in_bridge_fee<BTC>(source_chain,1,500,1_000_000_000);
    env.destroy_env();
}



#[test]
fun test_set_cross_out_bridge_fee_with_fixed(){
    let chain_id = chain_ids::sui_testnet();
    let to_chain=chain_ids::aptos_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.set_cross_out_bridge_fee<BTC>(to_chain,0,80000,1_000_000_000);
    env.destroy_env();
}


#[test]
fun test_set_cross_out_bridge_fee_with_percentage(){
    let chain_id = chain_ids::sui_testnet();
    let to_chain=chain_ids::aptos_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.set_cross_out_bridge_fee<BTC>(to_chain,1,800,1_000_000_000);
    env.destroy_env();
}


#[test]
fun test_get_withdraw_fee_cap(){
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();

    env.withdraw_bridge_fee_cap<BTC>(10000);
    env.destroy_env();

}

#[test]
fun test_fast_path_limit_update(){
    let chain_id = chain_ids::sui_testnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();

    env.fast_path_limit_update(100000000000);
    env.destroy_env();

}

#[test]
fun test_remove_token_on_benfen(){
    let chain_id = chain_ids::sui_testnet();
    let target_id=chain_ids::aptos_testnet();
    let token_id=11; //aptos coin
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.add_token_on_token_list(chain_id,target_id, token_id);
    env.remove_token_on_token_list(chain_id, target_id, token_id);
    env.destroy_env();
}

#[test]
fun test_remove_token_from_benfen(){
    let chain_id = chain_ids::sui_testnet();
    let source_chain=chain_ids::aptos_testnet();
    let token_id=11; //aptos coin
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    env.add_token_on_token_list(source_chain,chain_id, token_id);
    env.remove_token_on_token_list(source_chain, chain_id, token_id);
    env.destroy_env();
}


// #[test]
// #[
// expected_failure(
//     abort_code = bridge::tokenlist::EBridgeCenterTokenLisAlreadyExists,
// )]
// fun test_twice_call_add_center_token_list(){
//     let chain_id = chain_ids::sui_testnet();
//     let mut env = create_env(chain_id);
//     env.create_bridge_default();
//     let mut bridge = env.bridge(@0x0);
//     let bridge_inner = bridge.bridge_ref_mut();
//     bridge_inner.migrate(env.scenario().ctx());
//     //bridge_inner.migrate(env.scenario().ctx());
//     bridge.return_bridge();
//     env.destroy_env();
// }

#[test]
fun test_get_available_claim_amount_for_router_limit() {
    let chain_id = chain_ids::sui_mainnet();
    let mut env = create_env(chain_id);
    env.create_bridge_default();
    let mut bridge = env.bridge(@0x0);
    let bridge_inner = bridge.bridge_ref_mut();

    bridge_inner.get_available_claim_amount<USDC>(chain_ids::bsc_mainnet());
    bridge_inner.get_available_claim_amount<USDC>(chain_ids::op_mainnet());
    bridge_inner.get_available_claim_amount<USDC>(chain_ids::base_mainnet());

    bridge.return_bridge();
    env.destroy_env();
}

#[test]
fun test_external_busd_approval_and_claimed_external_busd_coin() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    let chain_id_val = chain_id(&mut env);
    let token_type_val = token_type<USDC>(&mut env);
    let source_chain = chain_ids::bsc_mainnet();
    let source_address = vector[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let target_address = address::to_bytes(@0x123);
    let tx_hash = ascii::string(b"abc123");
    let amount = 1000u64;
    // 构造 message
    let message = bridge::message::create_token_bridge_message(
        source_chain,
        0,
        source_address,
        chain_id_val,
        target_address,
        token_type_val,
        amount,
        *tx_hash.as_bytes(),
        0u8,
    );
    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);
    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();

    let mut scenario = public_setup(1_000_000_000_000_000_000, b"MINT-BUSD-right_key");
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    // 调用 approval_and_claimed_external_busd_coin
    bridge.bridge_ref_mut().approval_and_claimed_external_busd_coin<busd::BUSD>(
        message,
        signatures,
        &mut bfc_system_state,
        &cap,
        ctx,
    );
    let deposited = sui::event::events_by_type<bridge::bridge::ExternalDepositedEventV2>();
    assert!(deposited.length() == 1);
    {
        let (
            tx_hash,
            token_type,
            source_chain,
            target_chain,
            source_address,
            target_address,
            amount_before_fee,
            _,
        ) = deposited[0].unwrap_external_deposited_event_v2();
        assert!(
            tx_hash == tx_hash &&
                token_type == token_type &&
                source_chain == source_chain &&
                target_chain == chain_ids::sui_custom() &&
                source_address == source_address &&
                target_address == target_address &&
                amount == amount_before_fee,
        );
    };

    scenario.next_tx(@0x0);
    let token = scenario.take_from_address<Coin<BUSD>>(address::from_bytes(target_address));
    assert!(token.balance().value() == amount);

    sui::test_scenario::return_to_address(address::from_bytes(target_address), token);
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    bridge.return_bridge();
    env.destroy_env();
}

#[test]
fun test_external_busd_withdraw_external_busd_coin_tron_test() {
    test_external_busd_withdraw_external_busd_coin(4u64,  chain_ids::tron_testnet(),chain_ids::sui_custom());
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidBridgeRoute)]
fun test_external_busd_withdraw_external_busd_coin_tron_main() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::tron_mainnet(),chain_ids::sui_custom())
}

#[test]
fun test_external_busd_withdraw_external_busd_coin_sol_test() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::solana_testnet(),chain_ids::sui_custom())
}

#[test]
fun test_external_busd_withdraw_external_busd_coin_sui_testnet_tron_test() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::tron_testnet(),chain_ids::sui_testnet());
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidBridgeRoute)]
fun test_external_busd_withdraw_external_busd_coin_sui_testnet_tron_main() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::tron_mainnet(),chain_ids::sui_testnet())
}

#[test]
fun test_external_busd_withdraw_external_busd_coin_sui_testnet_sol_test() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::solana_testnet(),chain_ids::sui_testnet())
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidBridgeRoute)]
fun test_external_busd_withdraw_external_busd_coin_sui_testnet_sol_main() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::solana_mainnet(),chain_ids::sui_testnet())
}

#[test]
fun test_external_busd_withdraw_external_busd_coin_sui_mainnet_tron_main() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::tron_mainnet(),chain_ids::sui_mainnet());
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidBridgeRoute)]
fun test_external_busd_withdraw_external_busd_coin_sui_mainnet_tron_test() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::tron_testnet(),chain_ids::sui_mainnet())
}

#[test]
fun test_external_busd_withdraw_external_busd_coin_sui_mainnet_sol_mainnet() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::solana_mainnet(),chain_ids::sui_mainnet())
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidBridgeRoute)]
fun test_external_busd_withdraw_external_busd_coin_sui_mainnet_sol_test() {
    test_external_busd_withdraw_external_busd_coin(4u64, chain_ids::solana_testnet(),chain_ids::sui_mainnet())
}

#[test]
fun test_external_busd_withdraw_external_busd_coin_sui_mainnet_sol_mainnet_except_id() {
    test_external_busd_withdraw_external_busd_coin(3u64, chain_ids::solana_mainnet(),chain_ids::sui_mainnet())
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidChainIDAndTokenIDExpect)]
fun test_external_busd_withdraw_external_busd_coin_sui_mainnet_sol_mainnet_unexcept_id() {
    test_external_busd_withdraw_external_busd_coin(1u64,chain_ids::solana_mainnet(), chain_ids::sui_mainnet())
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EOnlySupportBusd)]
fun test_external_busd_other_coin_withdraw_external_busd_coin() {
    let source_address = vector[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    let amount = 1000u64;
    let coin = get_usdc(&mut env, amount);

    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();
    let scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);


    bridge.bridge_ref_mut().withdraw_external_busd_coin<USDC>(
        chain_ids::tron_testnet(),
        source_address,
        coin,
        4u64,
        &mut bfc_system_state,
        ctx,
    );
    let withdraw = sui::event::events_by_type<bridge::bridge::ExternalWithdrawEvent>();
    assert!(withdraw.length() == 1);


    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::end(scenario);
    bridge.return_bridge();
    env.destroy_env();
}

// Test for defi_stake_success function
#[test]
fun test_defi_stake_success() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    let source_chain = chain_ids::eth_mainnet();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0xABCD);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 1;
    let protocol_token_id = 3; // USDC

    // Create a BridgeMessage for defi transfer in
    let message = message::create_defi_transfer_in_message(
        source_chain,
        seq_num,
        sender_address,
        target_chain,
        amount,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        0u64,
        STAKE
    );

    // Create signatures
    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    // Call approve_defi_transfer_in which will internally call defi_stake_success
    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();
    
    bridge.approve_defi_transfer_in(message, signatures);

    // Check that the DefiTokensStakedEvent was emitted
    let staked_events = sui::event::events_by_type<bridge::bridge::DefiTokensStakedEvent>();
    assert!(staked_events.length() == 1, 0);

    // Check that the defi_holders record was updated
    let defi_protocol_key = bridge::bridge::create_defi_protocol_key_for_testing(
        protocol_type,
        protocol_version,
        protocol_token_id
    );
    
    let holder_amount = bridge.test_defi_holders_get(address::from_bytes(sender_address), defi_protocol_key);
    assert!(holder_amount == amount, 0);

    bridge_wrap.return_bridge();
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EBridgeUnavailable)]
fun test_defi_stake_success_bridge_paused() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    // Pause the bridge
    env.freeze_bridge(@0x0, 1000);

    let source_chain = chain_ids::eth_mainnet();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0xABCD);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 1;
    let protocol_token_id = 3; // USDC

    // Create a BridgeMessage for defi transfer in
    let message = message::create_defi_transfer_in_message(
        source_chain,
        seq_num,
        sender_address,
        target_chain,
        amount,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        0u64,
        STAKE
    );

    // Create signatures
    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    // Call approve_defi_transfer_in which will internally call defi_stake_success
    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();
    
    // This should fail because the bridge is paused
    bridge.approve_defi_transfer_in(message, signatures);

    bridge_wrap.return_bridge();
    env.destroy_env();
}

#[test]
fun test_defi_stake_success_multiple_stakes_same_user() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    let source_chain = chain_ids::eth_mainnet();
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 1;
    let protocol_token_id = 3; // USDC
    let sender_address = address::to_bytes(@0xABCD);

    // First stake
    let seq_num1 = 100;
    let amount1 = 1000;

    let message1 = message::create_defi_transfer_in_message(
        source_chain,
        seq_num1,
        sender_address,
        target_chain,
        amount1,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        0u64,
        STAKE
    );

    let signatures1 = sign_message_with(&env, message1, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();
    bridge.approve_defi_transfer_in(message1, signatures1);

    // Second stake with same user and protocol
    let seq_num2 = 101;
    let amount2 = 2000;

    let message2 = message::create_defi_transfer_in_message(
        source_chain,
        seq_num2,
        sender_address,
        target_chain,
        amount2,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        0u64,
        STAKE
    );

    let signatures2 = sign_message_with(&env, message2, vector[0, 1, 2]);

    bridge.approve_defi_transfer_in(message2, signatures2);

    // Check that the defi_holders record was updated with combined amount
    let defi_protocol_key = bridge::bridge::create_defi_protocol_key_for_testing(
        protocol_type,
        protocol_version,
        protocol_token_id
    );
    
    let holder_amount = bridge.test_defi_holders_get(address::from_bytes(sender_address), defi_protocol_key);
    assert!(holder_amount == (amount1 + amount2), 0);

    // Check that two DefiTokensStakedEvent events were emitted
    let staked_events = sui::event::events_by_type<bridge::bridge::DefiTokensStakedEvent>();
    assert!(staked_events.length() == 2, 0);

    bridge_wrap.return_bridge();
    env.destroy_env();
}

#[test]
fun test_defi_stake_success_different_users_protocols() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    let source_chain = chain_ids::eth_mainnet();
    let target_chain = chain_ids::sui_custom();
    
    // User 1 with protocol 1
    let user1_address = address::to_bytes(@0x1);
    let protocol_type1 = 1;
    let protocol_version1 = 1;
    let protocol_token_id1 = 3; // USDC
    let seq_num1 = 100;
    let amount1 = 1000;

    let message1 = message::create_defi_transfer_in_message(
        source_chain,
        seq_num1,
        user1_address,
        target_chain,
        amount1,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type1,
        protocol_version1,
        protocol_token_id1,
        0u64,
        STAKE
    );

    let signatures1 = sign_message_with(&env, message1, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();
    bridge.approve_defi_transfer_in(message1, signatures1);

    // User 2 with protocol 2
    let user2_address = address::to_bytes(@0x2);
    let protocol_type2 = 2;
    let protocol_version2 = 1;
    let protocol_token_id2 = 4; // USDT
    let seq_num2 = 101;
    let amount2 = 2000;

    let message2 = message::create_defi_transfer_in_message(
        source_chain,
        seq_num2,
        user2_address,
        target_chain,
        amount2,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type2,
        protocol_version2,
        protocol_token_id2,
        0u64,
        STAKE
    );

    let signatures2 = sign_message_with(&env, message2, vector[0, 1, 2]);

    bridge.approve_defi_transfer_in(message2, signatures2);

    // Check that the defi_holders records were updated correctly
    let defi_protocol_key1 = bridge::bridge::create_defi_protocol_key_for_testing(
        protocol_type1,
        protocol_version1,
        protocol_token_id1
    );
    
    let defi_protocol_key2 = bridge::bridge::create_defi_protocol_key_for_testing(
        protocol_type2,
        protocol_version2,
        protocol_token_id2
    );
    
    let user1_amount = bridge.test_defi_holders_get(address::from_bytes(user1_address), defi_protocol_key1);
    assert!(user1_amount == amount1, 0);
    
    let user2_amount = bridge.test_defi_holders_get(address::from_bytes(user2_address), defi_protocol_key2);
    assert!(user2_amount == amount2, 0);

    // User 1 should not have any amount for protocol 2
    let user1_amount_protocol2 = bridge.test_defi_holders_get(address::from_bytes(user1_address), defi_protocol_key2);
    assert!(user1_amount_protocol2 == 0, 0);

    // Check that two DefiTokensStakedEvent events were emitted
    let staked_events = sui::event::events_by_type<bridge::bridge::DefiTokensStakedEvent>();
    assert!(staked_events.length() == 2, 0);

    bridge_wrap.return_bridge();
    env.destroy_env();
}

fun test_external_busd_withdraw_external_busd_coin(token_id_expect: u64, target_chain: u8, source_chain: u8) {
    let source_address = vector[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let mut env = create_env(source_chain);
    env.create_bridge_default();

    let amount = 100*1_000_000_000u64;

    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();
    let scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, ctx);

    bridge.bridge_ref_mut().withdraw_external_busd_coin<BUSD>(
        target_chain,
        source_address,
        coin,
        token_id_expect,
        &mut bfc_system_state,
        ctx,
    );
    let withdraw = sui::event::events_by_type<bridge::bridge::ExternalWithdrawEventV2>();
    assert!(withdraw.length() == 1);
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);

    bridge.return_bridge();
    env.destroy_env();
}

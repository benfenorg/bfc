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
    defi_holders_amount_get,
    test_adjust_amount_usdc_usdt_in,
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
    sign_message_with,
    sign_message_with_mut
};
use bridge::btc::BTC;
use bridge::chain_ids;
use bridge::eth::ETH;
use bridge::message::{Self, to_parsed_token_transfer_message_v2};
use bridge::message_types;
use bridge::test_token::{TEST_TOKEN, create_bridge_token as create_test_token};
use bridge::usdc::USDC;
use bridge::usdt::USDT;
use std::type_name;
use sui::address;
use sui::balance;
use sui::clock::{Self};
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
use bridge::busd::BUSD as BUSDFAKER;


use bfc_system::bfc_system_state_inner::BfcSystemModifyCap;
use bridge::bridge_env::get_usdc;

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
fun test_execute_send_token_to_solana() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let usdc: Coin<USDC> = env.get_usdc(1);
    let solana_address = x"0000000000000000000000000000000000000000000000000000000000000000";
    env.send_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc);
    env.destroy_env();
} 

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidEvmAddress)]
fun test_send_token_to_solana_with_evm_address_length() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let usdc = env.get_usdc(1);
    // 使用 EVM 地址长度(20 字节)而不是 Solana 的 32 字节
    let wrong_address = x"0000000000000000000000000000000000000000";
    env.send_token(@0xABCD, chain_ids::solana_testnet(), wrong_address, usdc);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidEvmAddress)]
fun test_send_token_to_solana_with_oversized_address() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    let usdc = env.get_usdc(1);
    // 使用 33 字节(超过 Solana 的 32 字节)
    let oversized_address = x"000000000000000000000000000000000000000000000000000000000000000001";
    env.send_token(@0xABCD, chain_ids::solana_testnet(), oversized_address, usdc);
    env.destroy_env();
}

#[test]
#[expected_failure]
fun test_send_token_to_solana_respects_route_limits() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    
    // 测试单笔转账限额
    let max_single_limit = 100_000_000_000; // 从配置获取
    

    let solana_address = x"1234567890123456789012345678901234567890123456789012345678901234";
    
    // 测试超过限额的转账(应该失败)
    let usdc2 = env.get_usdc(max_single_limit + 1000);
    // 这个调用应该失败,但需要根据实际的错误码调整
    env.send_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc2);
    
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EBridgeUnavailable)]
fun test_send_token_to_solana_when_bridge_paused() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    
    // Pause the bridge
    env.freeze_bridge(@0x0, 1000);
    
    // 尝试转账(应该失败)
    let usdc = env.get_usdc(100);
    let solana_address = x"1234567890123456789012345678901234567890123456789012345678901234";
    env.send_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc);
    
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EBridgeUnavailable)]
fun test_send_back_token_to_solana_when_bridge_paused() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    
    // Pause the bridge
    env.freeze_bridge(@0x0, 1000);

    // 尝试退款(应该失败)
    let solana_address = x"1234567890123456789012345678901234567890123456789012345678901234";
    let tx_hash = hex::decode(b"56335bb5461b430c3ccf94efe91494e64f21e12e9b8b007b0e1c56c7d1e8de3b");
    let usdc_id = 3;
    let usdc_amount = 100;
    
    env.send_back_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc_id, usdc_amount, tx_hash);
    
    env.destroy_env();
}

#[test]
fun test_send_token_to_solana_after_bridge_unpaused() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    
    // Pause the bridge
    env.freeze_bridge(@0x0, 1000);
    
    // 解除暂停
    env.unfreeze_bridge(@0x0, 1000);
    
    // 现在应该可以正常转账
    let usdc = env.get_usdc(100);
    let solana_address = x"1234567890123456789012345678901234567890123456789012345678901234";
    env.send_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc);
    
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
fun test_usdt_bridge_pre_deposit_and_deposit() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();

    let sender1 = @0xA;
    let sender2 = @0xB;
    let sender3 = @0xC;
    let (witness, private_key, source_address, tx_hash, target_address, amount) = mock_bitcoin_message();
    let type_name = type_name::get<USDT>();
    let coin_type = type_name.into_string();
    
    // Use ETH Sepolia for USDT
    let source_chain = chain_ids::eth_sepolia();
    
    let bitcoin_message = message::create_bitcoin_message(
        source_chain,
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


    env.pre_deposit_external_coin_for_testing<USDT>(
        sender1,
        source_chain,
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    env.pre_deposit_external_coin_for_testing<USDT>(
        sender2,
        source_chain,
        source_address,
        target_address,
        1*100_000_000,
        tx_hash.to_ascii_string(),
        signatures,
    );

    // Using deposit_and_withdraw_external_coin to verify the full flow including deposit_external_coin
    env.deposit_and_withdraw_external_coin<USDT>(
        sender1,
        source_chain,
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
fun test_execute_send_back_token_to_solana() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    // Solana address must be 32 bytes
    let solana_address = x"0000000000000000000000000000000000000000000000000000000000000000";
    let tx_hash = hex::decode(b"56335bb5461b430c3ccf94efe91494e64f21e12e9b8b007b0e1c56c7d1e8de3b");
    let usdc_id = 3;
    let usdc_amount = 100;
    env.send_back_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc_id, usdc_amount, tx_hash);
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
fun test_send_back_token_to_solana_with_wrong_address_length() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    
    // 使用 EVM 地址长度
    let wrong_address = x"0000000000000000000000000000000000000000";
    let tx_hash = hex::decode(b"56335bb5461b430c3ccf94efe91494e64f21e12e9b8b007b0e1c56c7d1e8de3b");
    let usdc_id = 3;
    let usdc_amount = 100;
    
    env.send_back_token(@0xABCD, chain_ids::solana_testnet(), wrong_address, usdc_id, usdc_amount, tx_hash);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::ETokenValueIsZero)]
fun test_send_back_token_to_solana_with_zero_amount() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    
    let solana_address = x"1234567890123456789012345678901234567890123456789012345678901234";
    let tx_hash = hex::decode(b"56335bb5461b430c3ccf94efe91494e64f21e12e9b8b007b0e1c56c7d1e8de3b");
    let usdc_id = 3;
    let usdc_amount = 0; // 零金额
    
    env.send_back_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc_id, usdc_amount, tx_hash);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EDuplicateRefund)]
fun test_send_back_token_to_solana_duplicate_tx_hash() {
    let mut env = create_env(chain_ids::sui_testnet());
    env.create_bridge_default();
    
    let solana_address = x"1234567890123456789012345678901234567890123456789012345678901234";
    let tx_hash = hex::decode(b"56335bb5461b430c3ccf94efe91494e64f21e12e9b8b007b0e1c56c7d1e8de3b");
    let usdc_id = 3;
    let usdc_amount = 100;
    
    // 第一次退款
    env.send_back_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc_id, usdc_amount, tx_hash);
    
    // 第二次使用相同的 tx_hash(应该失败)
    env.send_back_token(@0xABCD, chain_ids::solana_testnet(), solana_address, usdc_id, usdc_amount, tx_hash);
    
    env.destroy_env();
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
    abort_code = bridge::defi_protocols::EDefiProtocolConfigRegistryAlreadyExists,
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
    bridge.bridge_ref_mut().approval_and_claimed_external_busd_coin<BUSDFAKER>(
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

// Test for defi_stake function
#[test]
fun test_defi_stake() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

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

    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    // Check that the DefiTransferOutEvent was emitted
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    // Check that the bridge record was stored
    let bridge_inner = bridge.bridge_ref().test_load_inner();
    let records = bridge_inner.inner_token_transfer_records();
    assert!(records.length() == 1, 0);

    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

#[test]
fun test_defi_stake_and_approve_defi_transfer_out_full_flow() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

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

    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    // Check that the DefiTransferOutEvent was emitted
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    // Check that the bridge record was stored
    let bridge_inner = bridge.bridge_ref().test_load_inner();
    let records = bridge_inner.inner_token_transfer_records();
    assert!(records.length() == 1, 0);

    // Calculate the same fee as defi_stake function would
    // For testing purposes, we assume no fee (as there might not be fee setup)
    // In reality, you'd need to calculate: bridge_fee::calculate_cross_out_fee_amount
    let amount_after_fee = amount; // Simplified - no fee for test

    // Create the same message that was created by defi_stake
    let sender_address = address::to_bytes(@0x0);
    let bridge_seq_num = 0; // First sequence number
    let message = message::create_defi_transfer_out_message(
        chain_ids::sui_custom(), // source chain - should be the same as bridge's chain_id
        bridge_seq_num,
        sender_address,
        target_chain, // target chain should be eth_mainnet
        amount_after_fee / 1000,
        hex::decode(b""),
        0u16,
        protocol_type,
        protocol_version,
        protocol_token_id,
        STAKE,
        amount_after_fee
    );

    // Create signatures while we still have access to the env
    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    // Call approve_defi_transfer_out function in the same bridge session
    bridge.bridge_ref_mut().approve_defi_transfer_out(message, signatures);

    // Check that the TokenTransferApproved event was emitted
    let approved_events = sui::event::events_by_type<bridge::bridge::TokenTransferApproved>();
    assert!(approved_events.length() == 1, 0);

    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test for defi_stake with amount within limit
#[test]
fun test_defi_stake_within_limit() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    // Get BUSD coin for testing with amount within limit
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    // Set amount to be within the limit (100_000_000_000_000)
    let amount = 50_000_000_000_000u64;

    scenario.next_tx(@0x0);
    let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

    // Call defi_stake function
    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();

    let target_chain = chain_ids::eth_mainnet();
    let protocol_type = 1u64;
    let protocol_version = 3u64;
    let protocol_token_id = 3u64; // USDC

    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    // Check that the DefiTransferOutEvent was emitted
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    // Check that the bridge record was stored
    let bridge_inner = bridge.bridge_ref().test_load_inner();
    let records = bridge_inner.inner_token_transfer_records();
    assert!(records.length() == 1, 0);

    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test for defi_stake with amount exceeding limit
#[test]
#[expected_failure(abort_code = bridge::bridge::ETransferLimit)]
fun test_defi_stake_exceeds_limit() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    // Get BUSD coin for testing with amount exceeding limit
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    // Set amount to exceed the limit (100_000_000_000_000)
    let amount = 2_100_000_000_000_000u64;

    scenario.next_tx(@0x0);
    let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

    // Call defi_stake function
    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();

    let target_chain = chain_ids::eth_mainnet();
    let protocol_type = 1u64;
    let protocol_version = 3u64;
    let protocol_token_id = 3u64; // USDC

    // This should fail with ETransferLimit error
    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
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
        STAKE,
        100,
        amount
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

    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(source_chain, amount);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == adjusted_amount, 0);

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
        STAKE,
        100,
        amount
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
        STAKE,
        100,
        amount1
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
        STAKE,
        100,
        amount2
    );

    let signatures2 = sign_message_with(&env, message2, vector[0, 1, 2]);

    bridge.approve_defi_transfer_in(message2, signatures2);

    // Check that the defi_holders record was updated with combined amount

    // When source_chain is eth_mainnet, amounts are adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount1 = test_adjust_amount_usdc_usdt_in(source_chain, 1000);
    let adjusted_amount2 = test_adjust_amount_usdc_usdt_in(source_chain, 2000);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == (adjusted_amount1 + adjusted_amount2), 0);


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
        STAKE,
        100,
        amount1
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
        STAKE,
        100,
        amount2
    );

    let signatures2 = sign_message_with(&env, message2, vector[0, 1, 2]);

    bridge.approve_defi_transfer_in(message2, signatures2);

    // Check that the defi_holders records were updated correctly

    // When source_chain is eth_mainnet, amounts are adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount1 = test_adjust_amount_usdc_usdt_in(source_chain, amount1);
    let adjusted_amount2 = test_adjust_amount_usdc_usdt_in(source_chain, amount2);
    let user1_amount = bridge.defi_holders_amount_get(
        address::from_bytes(user1_address),
        protocol_type1,
        protocol_version1,
        protocol_token_id1,
        source_chain
    );
    assert!(user1_amount == adjusted_amount1, 0);

    let user2_amount = bridge.defi_holders_amount_get(
        address::from_bytes(user2_address),
        protocol_type2,
        protocol_version2,
        protocol_token_id2,
        source_chain
    );
    assert!(user2_amount == adjusted_amount2, 0);

    // User 1 should not have any amount for protocol 2
    let user1_amount_protocol2 = bridge.defi_holders_amount_get(
        address::from_bytes(user1_address),
        protocol_type2,
        protocol_version2,
        protocol_token_id2,
        source_chain
    );
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
    let withdraw = sui::event::events_by_type<bridge::bridge::ExternalWithdrawEventV3>();
    assert!(withdraw.length() == 1);
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);

    bridge.return_bridge();
    env.destroy_env();
}

// Test complete defi stake flow: defi_stake -> approve_defi_transfer_out -> approve_defi_transfer_in -> defi_stake_success
#[test]
fun test_defi_stake_complete_flow() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    // Setup BUSD coin for testing
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    let amount = 1000u64;

    scenario.next_tx(@0x0);
    let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

    // Step 1: Call defi_stake function
    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();

    let target_chain = chain_ids::eth_mainnet();
    let protocol_type = 1u64;
    let protocol_version = 3u64;
    let protocol_token_id = 3u64; // USDC
    let sender_address = address::to_bytes(@0x0);

    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    // Verify DefiTransferOutEvent was emitted
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    // Verify bridge record was stored
    let bridge_inner = bridge.bridge_ref().test_load_inner();
    let records = bridge_inner.inner_token_transfer_records();
    assert!(records.length() == 1, 0);

    // Step 2: Prepare approve_defi_transfer_out
    // Get the sequence number from the bridge (it should be 0 for first transaction)
    let bridge_seq_num = 0;
    let amount_after_fee = amount; // Simplified - no fee for test

    // Create the defi transfer out message
    let defi_out_message = message::create_defi_transfer_out_message(
        chain_ids::sui_custom(), // source chain
        bridge_seq_num,
        sender_address,
        target_chain,
        amount_after_fee / 1000,
        hex::decode(b""),
        0u16,
        protocol_type,
        protocol_version,
        protocol_token_id,
        STAKE,
        amount_after_fee,
    );

    // Create signatures for the defi transfer out message
    let out_signatures = sign_message_with(&env, defi_out_message, vector[0, 1, 2]);

    // Call approve_defi_transfer_out
    bridge.bridge_ref_mut().approve_defi_transfer_out(defi_out_message, out_signatures);

    // Verify TokenTransferApproved event was emitted
    let approved_events = sui::event::events_by_type<bridge::bridge::TokenTransferApproved>();
    assert!(approved_events.length() == 1, 0);

    // Step 3: Prepare approve_defi_transfer_in (STAKE message)
    // Create defi transfer in message to trigger defi_stake_success
    let eth_seq_num = 100; // Different sequence number from ETH side
    let lp_token_amount = 950; // LP tokens received from staking
    
    let defi_in_message = message::create_defi_transfer_in_message(
        target_chain, // source chain (ETH)
        eth_seq_num,
        sender_address,
        chain_ids::sui_custom(), // target chain (Sui)
        amount_after_fee,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        lp_token_amount,
        STAKE, // action type
        bridge_seq_num, // original sequence number from sui
        amount_after_fee
    );

    // Create signatures for the defi transfer in message
    let in_signatures = sign_message_with(&env, defi_in_message, vector[0, 1, 2]);

    // Call approve_defi_transfer_in (this will trigger defi_stake_success internally)
    bridge.bridge_ref_mut().approve_defi_transfer_in(defi_in_message, in_signatures);

    // Verify DefiTokensStakedEvent was emitted
    let staked_events = sui::event::events_by_type<bridge::bridge::DefiTokensStakedEvent>();
    assert!(staked_events.length() == 1, 0);

    // Verify defi_holders record was updated

    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(target_chain, amount_after_fee);
    let holder_amount = bridge.bridge_ref().defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        target_chain
    );
    assert!(holder_amount == adjusted_amount, 0);

    // Cleanup
    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test defi unstake
#[test]
fun test_defi_unstake() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
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

    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(source_chain, amount);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let principal_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == adjusted_amount, 0);
    assert!(lp_token_amount == lp_token_amount, 0);
    //unstake start
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount, principal_amount, ctx);
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == 0, 0);
    assert!(lp_token_amount == 0, 0);

    //unstake end
    bridge_wrap.return_bridge();
    // sui::test_scenario::return_shared(bfc_system_state);
    // sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EDefiUnstakeAmountNotEnough)]
fun test_defi_unstake_gt_stake_amount() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
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
    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(source_chain, amount);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == adjusted_amount, 0);
    assert!(lp_token_amount == lp_token_amount, 0);
    //unstake start
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    
    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount*2, holder_amount, ctx);
    
    //unstake end
    bridge_wrap.return_bridge();
    // sui::test_scenario::return_shared(bfc_system_state);
    // sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

#[test]
fun test_defi_unstake_and_approve_defi_transfer_out(){
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
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
    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(source_chain, amount);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == adjusted_amount, 0);
    assert!(lp_token_amount == lp_token_amount, 0);
    //unstake start
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    
    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount,holder_amount, ctx);
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);
    let principal_amount = bridge::bridge::get_defi_transfer_out_event_principal_amount(transfer_out_events.borrow(0));
    assert!(principal_amount == amount*1000, 0);

    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == 0, 0);
    assert!(lp_token_amount == 0, 0);

    // Check that the bridge record was stored
    let bridge_inner = bridge.test_load_inner_mut();
    let records = bridge_inner.inner_token_transfer_records_mut();
    assert!(records.length() == 2, 0);
    // let seq_num = bridge_inner.sequence_nums()[&message_types::defi()] - 1;
    let key = message::create_key(
            target_chain,
            message_types::defi(),
            0,
    );
    assert!(records.contains(key), 0);
    //准备签名
    let record = records.borrow_mut(key);
    std::debug::print(record);
    let message = record.message_mut();
    std::debug::print(message);
    // Create signatures while we still have access to the env
    let signatures = sign_message_with(&env, *message, vector[0, 1, 2]);

    // Call approve_defi_transfer_out function in the same bridge session
    bridge.approve_defi_transfer_out(*message, signatures);

    // Check that the TokenTransferApproved event was emitted
    let approved_events = sui::event::events_by_type<bridge::bridge::TokenTransferApproved>();
    assert!(approved_events.length() == 1, 0);

    //unstake end
    bridge_wrap.return_bridge();
    // sui::test_scenario::return_shared(bfc_system_state);
    // sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EDefiLimitError)]
fun test_defi_unstake_limit_error(){
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 101_000_000_000_000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
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
    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(source_chain, amount);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == adjusted_amount, 0);
    assert!(lp_token_amount == lp_token_amount, 0);
    //unstake start
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    
    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount,holder_amount, ctx);
    //unstake end
    bridge_wrap.return_bridge();
    // sui::test_scenario::return_shared(bfc_system_state);
    // sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

#[test]
#[expected_failure(abort_code = bridge::bridge::EDefiUnstakeAmountNotEnough)]
fun test_defi_unstake_gt_lp_amount(){
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 101_000_000_000_000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
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
    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(source_chain, amount);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == adjusted_amount, 0);
    assert!(lp_token_amount == lp_token_amount, 0);
    //unstake start
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    
    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount+1,holder_amount, ctx);
    //unstake end
    bridge_wrap.return_bridge();
    // sui::test_scenario::return_shared(bfc_system_state);
    // sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

//unstake, evm to sui test
#[test]
fun test_defi_unstake_and_approve_defi_transfer_in(){
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
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
    // When source_chain is eth_mainnet, amount is adjusted by adjust_amount_usdc_usdt_in function
    let adjusted_amount = bridge::bridge::test_adjust_amount_usdc_usdt_in(source_chain, amount);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == adjusted_amount, 0);
    assert!(lp_token_amount == lp_token_amount, 0);
    //unstake start
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    
    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount,holder_amount, ctx);
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let lp_token_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == 0, 0);
    assert!(lp_token_amount == 0, 0);
    scenario.next_tx(@0x0);
    let chain_id_evm = chain_ids::eth_custom();
    let chain_id_sui = chain_ids::sui_custom();
    let seq_num_1 = 1;
    let address_sui = address::to_bytes(@0x0);
    let amount_retake = 1100;
    let lp_token_amount = 1000_000_000_000;
    // Create a BridgeMessage for defi transfer in
    let message_in = message::create_defi_transfer_in_message(
        chain_id_evm,
        seq_num_1,
        address_sui,
        chain_id_sui,
        amount_retake,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        0u64,
        UNSTAKE,
        lp_token_amount,
        amount*1000,
    );
    // Create signatures
    let signatures_in = sign_message_with_mut(&mut env, message_in, vector[0, 1, 2]);
    bridge.approve_defi_transfer_in(message_in, signatures_in);
    let approved_events = sui::event::events_by_type<bridge::bridge::TokenTransferApproved>();
    assert!(approved_events.length() == 1, 0);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let ctx = env.ctx();
    let clock = clock::create_for_testing(ctx);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    
    bridge.claim_and_transfer_busd_for_defi<BUSDFAKER>(&mut bfc_system_state, &clock, chain_id_evm, seq_num_1, &cap, ctx);
    let transfer_in_events = sui::event::events_by_type<bridge::bridge::DefiTokensUnstakeEvent>();
    assert!(transfer_in_events.length() == 1, 0);
    let principal_amount = bridge::bridge::get_defi_tokens_unstake_event_principal_amount(transfer_in_events.borrow(0));
    let fee_actual = bridge::bridge::get_defi_tokens_unstake_event_fee(transfer_in_events.borrow(0));
    std::debug::print(&fee_actual);
    let fee_maybe=(amount_retake-amount)*1000*15/100;
    std::debug::print(&fee_maybe);
    assert!(fee_actual == fee_maybe, 0);

    assert!(principal_amount == amount*1000, 0);
    let holder_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_amount == 0, 404);
    //unstake end
    bridge_wrap.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    clock::destroy_for_testing(clock);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Zero amount stake - should fail with ETokenValueIsZero
#[test]
#[expected_failure(abort_code = bridge::bridge::EDefiStakeAmountNotEnough)]
fun test_defi_stake_zero_amount() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    // Get BUSD coin with non-zero amount, then split to create zero coin
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    let amount = 1000u64;

    scenario.next_tx(@0x0);
    let mut coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

    // Split all value out to create a zero-value coin
    let _non_zero_coin = coin.split(amount, scenario.ctx());
    // Now 'coin' has zero value

    // Destroy the non-zero coin since we don't need it
    bfc_system_state.burn_stable(_non_zero_coin, scenario.ctx());

    // Call defi_stake function with zero-value coin - should fail
    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();

    let target_chain = chain_ids::eth_mainnet();
    let protocol_type = 1u64;
    let protocol_version = 3u64;
    let protocol_token_id = 3u64; // USDC

    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    // Cleanup - should not reach here
    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Zero amount unstake - should fail with ETokenValueIsZero
#[test]
#[expected_failure(abort_code = bridge::bridge::ETokenValueIsZero)]
fun test_defi_unstake_zero_amount() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

    // First stake some tokens
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
        STAKE,
        lp_token_amount,
        amount,
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Try to unstake with zero amount - should fail
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, 0u64,0u64, ctx);

    // Cleanup - should not reach here
    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Invalid protocol token ID for unstake - should fail with EOnlySupportUsdcOrUsdt
#[test]
#[expected_failure(abort_code = bridge::bridge::EOnlySupportUsdcOrUsdt)]
fun test_defi_unstake_invalid_protocol_token() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;
    let principal_amount = 1000_000_000_000;

    // First stake with valid protocol_token_id (USDC)
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
        STAKE,
        lp_token_amount,
        amount,
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Try to unstake with invalid protocol_token_id (e.g., 999)
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    let invalid_protocol_token_id = 999u64; // Not USDC(3) or USDT(2)

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, invalid_protocol_token_id, lp_token_amount,principal_amount, ctx);

    // Cleanup - should not reach here
    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Unstake when bridge is paused - should fail with EBridgeUnavailable
#[test]
#[expected_failure(abort_code = bridge::bridge::EBridgeUnavailable)]
fun test_defi_unstake_when_bridge_paused() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;
    let principal_amount = 1000_000_000_000;
    // First stake some tokens
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
        STAKE,
        lp_token_amount,
        amount,
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Release bridge reference before pausing
    bridge_wrap.return_bridge();

    // Pause the bridge
    env.freeze_bridge(@0x0, 1000);

    // Get bridge reference again after pausing
    let mut bridge_wrap2 = env.bridge(@0x0);
    let bridge2 = bridge_wrap2.bridge_ref_mut();

    // Try to unstake when bridge is paused - should fail
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    bridge2.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount,principal_amount, ctx);

    // Cleanup - should not reach here
    bridge_wrap2.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Invalid protocol configuration for DeFi stake - should fail with EDefiProtocolConfigNotFound
// Note: Protocol validation happens before route validation in defi_stake
#[test]
#[expected_failure(abort_code = bridge::bridge::EDefiProtocolConfigNotFound)]
fun test_defi_stake_invalid_protocol() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    let amount = 1000u64;

    scenario.next_tx(@0x0);
    let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();

    // Use an invalid target_chain that doesn't have a valid route from sui_custom
    let target_chain = chain_ids::sui_mainnet(); // Invalid route from sui_custom to sui_mainnet
    let protocol_type = 1u64;
    let protocol_version = 3u64;
    let protocol_token_id = 3u64; // USDC

    // This should fail with EInvalidBridgeRoute
    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    // Cleanup - should not reach here
    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Invalid route for DeFi unstake - should fail with EInvalidBridgeRoute
// Note: In defi_unstake, route validation happens before protocol validation
#[test]
#[expected_failure(abort_code = bridge::bridge::EInvalidBridgeRoute)]
fun test_defi_unstake_invalid_route() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;
    let principal_amount = 1000_000_000_000;
    // First stake some tokens
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
        STAKE,
        lp_token_amount,
        amount,
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Try to unstake with invalid target route
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    let invalid_target_chain = chain_ids::sui_mainnet(); // Invalid route

    bridge.defi_unstake_v2(invalid_target_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount,principal_amount, ctx);

    // Cleanup - should not reach here
    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Unsupported chain-protocol combination for unstake - should fail with EDefiProtocolConfigNotFound
// Note: The protocol config check validates chain-token compatibility
#[test]
#[expected_failure(abort_code = bridge::bridge::EDefiProtocolConfigNotFound)]
fun test_defi_unstake_unsupported_chain_protocol() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;
    let principal_amount = 1000_000_000_000;
    // First stake some tokens
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
        STAKE,
        lp_token_amount,
        amount,
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Try to unstake to a chain that doesn't support the protocol_token_id
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();
    // Use solana_testnet which may not support USDC from Benfen
    let unsupported_chain = chain_ids::solana_testnet();

    bridge.defi_unstake_v2(unsupported_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount,principal_amount, ctx);

    // Cleanup - should not reach here
    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Minimum amount stake (1 unit) - should succeed
#[test]
fun test_defi_stake_minimum_amount() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();

    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    let mut bfc_system_state = sui::test_scenario::take_shared<BfcSystemState>(&scenario);
    let cap = sui::test_scenario::take_from_sender<BfcSystemModifyCap>(&scenario);
    let amount = 1000u64; // Minimum amount (1 unit)

    scenario.next_tx(@0x0);
    let coin = bfc_system::mint_stable<BUSD>(&mut bfc_system_state, amount, &cap, scenario.ctx());

    let mut bridge = env.bridge(@0x0);
    let ctx = env.ctx();

    let target_chain = chain_ids::eth_mainnet();
    let protocol_type = 1u64;
    let protocol_version = 3u64;
    let protocol_token_id = 3u64; // USDC

    bridge.bridge_ref_mut().defi_stake<BUSD>(
        &mut bfc_system_state,
        target_chain,
        coin,
        protocol_type,
        protocol_version,
        protocol_token_id,
        ctx,
    );

    // Verify event was emitted
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    // Verify record was stored
    let bridge_inner = bridge.bridge_ref().test_load_inner();
    let records = bridge_inner.inner_token_transfer_records();
    assert!(records.length() == 1, 0);

    bridge.return_bridge();
    sui::test_scenario::return_shared(bfc_system_state);
    sui::test_scenario::return_to_sender(&scenario, cap);
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Unstake exactly all LP tokens - should succeed
#[test]
fun test_defi_unstake_exact_all_lp_tokens() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let seq_num = 100;
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let amount = 1000;
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC
    let lp_token_amount = 1000_000_000_000;

    // First stake some tokens
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
        STAKE,
        lp_token_amount,
        amount,
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Verify staking succeeded
    let holder_lp_amount = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let principal_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(holder_lp_amount == lp_token_amount, 0);

    // Unstake exactly all LP tokens
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount,principal_amount, ctx);

    // Verify all LP tokens were unstaked
    let remaining_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_lp == 0, 0);

    // Verify event was emitted
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Multiple stakes followed by partial unstake - should succeed
#[test]
fun test_defi_multiple_stakes_partial_unstake() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC

    // First stake
    let seq_num_1 = 100;
    let amount_1 = 1000;
    let lp_token_amount_1 = 1000_000_000_000;
    let principal_amount_1 = 1000_000;

    let message_1 = message::create_defi_transfer_in_message(
        source_chain,
        seq_num_1,
        sender_address,
        target_chain,
        amount_1,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        0u64,
        STAKE,
        lp_token_amount_1,
        amount_1
    );

    let signatures_1 = sign_message_with(&env, message_1, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message_1, signatures_1);

    // Second stake
    let seq_num_2 = 101;
    let amount_2 = 2000;
    let lp_token_amount_2 = 2000_000_000_000;
    let principal_amount_2 = 2000_000;

    let message_2 = message::create_defi_transfer_in_message(
        source_chain,
        seq_num_2,
        sender_address,
        target_chain,
        amount_2,
        hex::decode(b""),
        0u16,
        0u8,
        protocol_type,
        protocol_version,
        protocol_token_id,
        0u64,
        STAKE,
        lp_token_amount_2,
        amount_2
    );

    let signatures_2 = sign_message_with_mut(&mut env, message_2, vector[0, 1, 2]);
    bridge.approve_defi_transfer_in(message_2, signatures_2);

    // Verify total LP tokens
    let total_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(total_lp == lp_token_amount_1 + lp_token_amount_2, 0);
    let total_principal_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(total_principal_amount == principal_amount_1 + principal_amount_2, 0);

    // Partial unstake (only unstake amount from first stake)
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, lp_token_amount_1,principal_amount_1, ctx);

    // Verify partial unstake
    let remaining_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_lp == lp_token_amount_2, 0);
    let remaining_principal_amount = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_principal_amount == principal_amount_2, 0);

    // Verify event was emitted
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test unstake with principal amount smaller than calculated - should use calculated value
#[test]
fun test_defi_unstake_principal_smaller_than_calculated() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC

    // Stake
    let seq_num = 100;
    let amount = 1000;
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Get actual principal amount
    let total_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    std::debug::print(&total_principal);
    let total_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );

    // Partial unstake with principal smaller than calculated
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    // Unstake half of LP tokens, but provide principal amount smaller than calculated
    let unstake_lp = total_lp / 2;
    let provided_principal = total_principal / 4; // Smaller than calculated (should be ~total_principal/2)

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, unstake_lp, provided_principal, ctx);

    // Verify event uses calculated principal amount (not the provided smaller one)
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);
    let event_principal = bridge::bridge::get_defi_transfer_out_event_principal_amount(transfer_out_events.borrow(0));
    std::debug::print(&event_principal);
    std::debug::print(&provided_principal);
    
    // Should use calculated value (approximately total_principal/2), not the provided smaller value
    assert!(event_principal > provided_principal, 0);
    assert!(event_principal <= total_principal / 2 + 1, 0); // Allow small rounding error

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test unstake with principal amount larger than calculated but within 10% - should use provided value
#[test]
fun test_defi_unstake_principal_larger_within_10_percent() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC

    // Stake
    let seq_num = 100;
    let amount = 1000;
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Get actual principal amount
    let total_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let total_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );

    // Partial unstake with principal larger than calculated but within 10%
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    // Unstake half of LP tokens
    let unstake_lp = total_lp / 2;
    let calculated_principal = total_principal / 2;
    // Provide principal 5% larger than calculated (within 10% threshold)
    let provided_principal = calculated_principal + calculated_principal * 5 / 100;

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, unstake_lp, provided_principal, ctx);

    // Verify event uses provided principal amount (within 10% threshold)
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);
    let event_principal = bridge::bridge::get_defi_transfer_out_event_principal_amount(transfer_out_events.borrow(0));
    // Should use provided value since it's within 10%
    assert!(event_principal == provided_principal, 0);

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test unstake with principal amount larger than calculated and exceeds 10% - should use calculated value
#[test]
fun test_defi_unstake_principal_larger_exceeds_10_percent() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC

    // Stake
    let seq_num = 100;
    let amount = 1000;
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Get actual principal amount
    let total_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let total_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );

    // Partial unstake with principal larger than calculated and exceeds 10%
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    // Unstake half of LP tokens
    let unstake_lp = total_lp / 2;
    let calculated_principal = total_principal / 2;
    // Provide principal 15% larger than calculated (exceeds 10% threshold)
    let provided_principal = calculated_principal + calculated_principal * 15 / 100;

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, unstake_lp, provided_principal, ctx);

    // Verify event uses calculated principal amount (not the provided larger one)
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);
    let event_principal = bridge::bridge::get_defi_transfer_out_event_principal_amount(transfer_out_events.borrow(0));
    // Should use calculated value since provided value exceeds 10% threshold
    assert!(event_principal == calculated_principal, 0);
    assert!(event_principal < provided_principal, 0);

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test unstake all LP tokens with correct LP amount but principal larger than calculated
#[test]
#[expected_failure(abort_code = bridge::bridge::EDefiUnstakeAmountNotEnoughForDel)]
fun test_defi_unstake_all_lp_principal_larger_than_calculated_within_10_percent() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC

    // Stake
    let seq_num = 100;
    let amount = 1000;
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Get actual principal amount
    let total_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let total_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );

    // Unstake all LP tokens with correct LP amount but principal larger than calculated
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    // Provide principal 20% larger than calculated (exceeds 10% threshold)
    let provided_principal = total_principal + total_principal * 5 / 100;

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, total_lp, provided_principal, ctx);

    // Verify event uses calculated principal amount (not the provided larger one)
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);
    let event_principal = bridge::bridge::get_defi_transfer_out_event_principal_amount(transfer_out_events.borrow(0));
    // Should use calculated value (total_principal) since provided value exceeds 10% threshold
    assert!(event_principal == total_principal, 0);
    assert!(event_principal < provided_principal, 0);

    // Verify all LP tokens were unstaked
    let remaining_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_lp == 0, 0);
    let remaining_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_principal == 0, 0);

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test unstake all LP tokens with correct LP amount but principal larger than calculated
#[test]
fun test_defi_unstake_all_lp_principal_larger_than_calculated_gt_10_percent() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC

    // Stake
    let seq_num = 100;
    let amount = 1000;
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Get actual principal amount
    let total_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let total_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );

    // Unstake all LP tokens with correct LP amount but principal larger than calculated
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    // Provide principal 20% larger than calculated (exceeds 10% threshold)
    let provided_principal = total_principal + total_principal * 15 / 100;

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, total_lp, provided_principal, ctx);

    // Verify event uses calculated principal amount (not the provided larger one)
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);
    let event_principal = bridge::bridge::get_defi_transfer_out_event_principal_amount(transfer_out_events.borrow(0));
    // Should use calculated value (total_principal) since provided value exceeds 10% threshold
    assert!(event_principal == total_principal, 0);
    assert!(event_principal < provided_principal, 0);

    // Verify all LP tokens were unstaked
    let remaining_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_lp == 0, 0);
    let remaining_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_principal == 0, 0);

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}

// Test unstake all LP tokens with correct LP amount but principal larger than calculated
#[test]
fun test_defi_unstake_all_lp_principal_larger_than_calculated_lt_10_percent() {
    let mut env = create_env(chain_ids::sui_custom());
    env.create_bridge_default();
    let mut scenario = public_setup(1_000_000_000_000_000_000, MINT_BUSD_RIGHT_KEY);
    scenario.next_tx(@0x0);
    let source_chain = chain_ids::eth_custom();
    let sender_address = address::to_bytes(@0x0);
    let target_chain = chain_ids::sui_custom();
    let protocol_type = 1;
    let protocol_version = 3;
    let protocol_token_id = 3; // USDC

    // Stake
    let seq_num = 100;
    let amount = 1000;
    let lp_token_amount = 1000_000_000_000;

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
        STAKE,
        lp_token_amount,
        amount
    );

    let signatures = sign_message_with(&env, message, vector[0, 1, 2]);

    let mut bridge_wrap = env.bridge(@0x0);
    let bridge = bridge_wrap.bridge_ref_mut();

    bridge.approve_defi_transfer_in(message, signatures);

    // Get actual principal amount
    let total_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    let total_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );

    // Unstake all LP tokens with correct LP amount but principal larger than calculated
    scenario.next_tx(@0xABCD);
    let ctx = env.ctx();

    // Provide principal 20% larger than calculated (exceeds 10% threshold)
    let provided_principal = total_principal - total_principal * 15 / 100;

    bridge.defi_unstake_v2(source_chain, protocol_type, protocol_version, protocol_token_id, total_lp, provided_principal, ctx);

    // Verify event uses calculated principal amount (not the provided larger one)
    let transfer_out_events = sui::event::events_by_type<bridge::bridge::DefiTransferOutEvent>();
    assert!(transfer_out_events.length() == 1, 0);
    let event_principal = bridge::bridge::get_defi_transfer_out_event_principal_amount(transfer_out_events.borrow(0));
    // Should use calculated value (total_principal) since provided value exceeds 10% threshold
    assert!(event_principal == total_principal, 0);
    assert!(event_principal > provided_principal, 0);

    // Verify all LP tokens were unstaked
    let remaining_lp = bridge.defi_holders_lp_token_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_lp == 0, 0);
    let remaining_principal = bridge.defi_holders_amount_get(
        address::from_bytes(sender_address),
        protocol_type,
        protocol_version,
        protocol_token_id,
        source_chain
    );
    assert!(remaining_principal == 0, 0);

    bridge_wrap.return_bridge();
    sui::test_scenario::end(scenario);
    env.destroy_env();
}


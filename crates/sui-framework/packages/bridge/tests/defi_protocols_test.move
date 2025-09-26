// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::defi_protocols_test;
    use sui::test_scenario;
    use sui::test_utils;
    use std::unit_test::assert_eq;
    use bridge::defi_protocols;

    const ETH_MAINNET: u8 = 10;
    const PROTOCOL_TYPE_AAVE: u64 = 1;
    const PROTOCOL_VERSION_AAVE: u64 = 1;
    const PROTOCOL_TOKEN_ID_AAVE: u64 = 3;
    const FEE_TYPE_PERCENTAGE: u8 = 0;
    const FEE_TYPE_FIXED: u8 = 1;
    /// 15%
    const FEE_RATE_PERCENTAGE: u64 = 0_150_000_000;
    /// 5 USDC
    const FEE_RATE_FIXED: u64 = 5_000_000_000;
    const LIMIT_STAKE_AMOUNT: u64 = 100_000_000_000_000;
    const LIMIT_UNSTAKE_AMOUNT: u64 = 100_000_000_000_000;
    
    
    public struct DefiProtocolConfigObject has key,store {
        id: UID
    }

    public fun new(ctx: &mut TxContext) : DefiProtocolConfigObject{
        DefiProtocolConfigObject {
            id: object::new(ctx),
        }
    }

    #[test]
    fun test_new_defi_protocol_config() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        let protocol_info = defi_protocols::get_protocol_info(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET);
        assert_eq!(protocol_info.protocol_type(), PROTOCOL_TYPE_AAVE);
        assert_eq!(protocol_info.protocol_version(), PROTOCOL_VERSION_AAVE);
        assert_eq!(protocol_info.protocol_token_id(), PROTOCOL_TOKEN_ID_AAVE);
        assert_eq!(protocol_info.chain_id(), ETH_MAINNET);
        assert_eq!(protocol_info.fee_type(), FEE_TYPE_PERCENTAGE);
        assert_eq!(protocol_info.fee_rate(), FEE_RATE_PERCENTAGE);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    

    #[test]
    fun test_update_existing_protocol() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        //update fee 
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_FIXED, FEE_RATE_FIXED, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        let protocol_info = defi_protocols::get_protocol_info(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET);
        assert_eq!(protocol_info.fee_type(), FEE_TYPE_FIXED);
        assert_eq!(protocol_info.fee_rate(), FEE_RATE_FIXED);
        assert_eq!(protocol_info.protocol_type(), PROTOCOL_TYPE_AAVE);
        assert_eq!(protocol_info.protocol_version(), PROTOCOL_VERSION_AAVE);
        assert_eq!(protocol_info.protocol_token_id(), PROTOCOL_TOKEN_ID_AAVE);
        assert_eq!(protocol_info.chain_id(), ETH_MAINNET);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }
    #[test]
    #[expected_failure(abort_code = defi_protocols::EDefiProtocolConfigNotFound)]
    fun test_delete_existing_protocol() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        //update fee 
        defi_protocols::delete_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET);
        defi_protocols::get_protocol_info(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_manage_fee_gt_0() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        // 测试百分比费率
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100, // lp_amount_withdraw
            1100, // amount_withdraw
            2000, // amount_in_record
            100 // lp_amount_in_record
        );
        //percentage=100/(100+100)=0.5
        //赎回本金=2000*0.5=1000
        //赎回利息=1100-1000=100
        //fee=100*0.15=15
        assert_eq!(fee, 15);

        // 测试固定费率
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_FIXED, FEE_RATE_FIXED, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        let decimal_token=1_000_000_000;
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100*decimal_token, // lp_amount_withdraw
            1100*decimal_token, // amount_withdraw
            1000*decimal_token, // amount_in_record
            100*decimal_token // lp_amount_in_record
        );
        assert_eq!(fee, FEE_RATE_FIXED);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_manage_fee_profit_lt_0() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        // 测试百分比费率
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100, // lp_amount_withdraw
            997, // amount_withdraw
            2000, // amount_in_record
            100 // lp_amount_in_record
        );
        //percentage=100/(100+100)=0.5
        //赎回本金=2000*0.5=1000
        //赎回利息=997-1000=-3
        //fee=0
        assert_eq!(fee, 0);

        // 测试固定费率
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_FIXED, FEE_RATE_FIXED, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100, // lp_amount_withdraw
            997, // amount_withdraw
            2000, // amount_in_record
            100 // lp_amount_in_record
        );
        assert_eq!(fee, 0);

        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_manage_fee_profit_eq_0() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        // 测试百分比费率
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100, // lp_amount_withdraw
            1000, // amount_withdraw
            2000, // amount_in_record
            100 // lp_amount_in_record
        );
        //percentage=100/(100+100)=0.5
        //赎回本金=2000*0.5=1000
        //赎回利息=1000-1000=0
        //fee=0
        assert_eq!(fee, 0);

        // 测试固定费率
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_FIXED, FEE_RATE_FIXED, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100, // lp_amount_withdraw
            1000, // amount_withdraw
            2000, // amount_in_record
            100 // lp_amount_in_record
        );
        assert_eq!(fee, 0);

        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_manage_fee_unstake_all() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        // 测试百分比费率
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100, // lp_amount_withdraw
            1100, // amount_withdraw
            1000, // amount_in_record
            0 // lp_amount_in_record
        );
        //percentage=100/(100+0)=1
        //赎回本金=1000*1=1000
        //赎回利息=1100-1000=100
        //fee=100*0.15=15
        assert_eq!(fee, 15);

        // 测试固定费率
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_FIXED, FEE_RATE_FIXED, LIMIT_STAKE_AMOUNT, LIMIT_UNSTAKE_AMOUNT);
        let decimal_token=1_000_000_000;
        let (fee, _principal) = defi_protocols::manage_fee(
            &obj.id,
            PROTOCOL_TYPE_AAVE,
            PROTOCOL_VERSION_AAVE,
            PROTOCOL_TOKEN_ID_AAVE,
            ETH_MAINNET,
            100*decimal_token, // lp_amount_withdraw
            1100*decimal_token, // amount_withdraw
            1000*decimal_token, // amount_in_record
            0 // lp_amount_in_record
        );
        assert_eq!(fee, FEE_RATE_FIXED);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }

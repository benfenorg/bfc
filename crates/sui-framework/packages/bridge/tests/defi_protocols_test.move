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
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE);
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
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE);
        //update fee 
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_FIXED, FEE_RATE_FIXED);
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
    #[expected_failure(abort_code = 1)] // EDefiProtocolConfigNotFound
    fun test_delete_existing_protocol() {
        let mut scenario = test_scenario::begin(@0x1);
        let ctx = test_scenario::ctx(&mut scenario);
        let mut obj = new(ctx);
        defi_protocols::new_defi_protocol_config_for_testing(&mut obj.id,ctx);
        defi_protocols::add_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET, FEE_TYPE_PERCENTAGE, FEE_RATE_PERCENTAGE);
        //update fee 
        defi_protocols::delete_defi_protocol(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET);
        defi_protocols::get_protocol_info(&mut obj.id, PROTOCOL_TYPE_AAVE, PROTOCOL_VERSION_AAVE, PROTOCOL_TOKEN_ID_AAVE, ETH_MAINNET);
        test_utils::destroy(obj);
        test_scenario::end(scenario);
    }



    // #[test]
    // #[expected_failure(abort_code = 1)] // EDefiProtocolConfigNotFound
    // fun test_get_nonexistent_protocol() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     let mut parent = object::new(&mut ctx);
    //     let parent_id = object::uid_to_inner(&parent);
        
    //     defi_protocols::registry(parent_id, &mut ctx);
        
    //     // 尝试获取不存在的协议
    //     defi_protocols::get_protocol_info(parent_id, PROTOCOL_TYPE_1, PROTOCOL_VERSION_1, PROTOCOL_TOKEN_ID_1);
        
    //     object::delete(parent);
    //     test::end(scenario);
    // }

    // #[test]
    // fun test_protocol_info_getters() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     let mut parent = object::new(&mut ctx);
    //     let parent_id = object::uid_to_inner(&parent);
        
    //     defi_protocols::registry(parent_id, &mut ctx);
    //     defi_protocols::add_defi_protocol(parent_id, PROTOCOL_TYPE_1, PROTOCOL_VERSION_1, PROTOCOL_TOKEN_ID_1, CHAIN_ID_1);
        
    //     let protocol_info = defi_protocols::get_protocol_info(parent_id, PROTOCOL_TYPE_1, PROTOCOL_VERSION_1, PROTOCOL_TOKEN_ID_1);
        
    //     // 测试所有 getter 方法
    //     assert!(defi_protocols::protocol_type(&protocol_info) == PROTOCOL_TYPE_1, 0);
    //     assert!(defi_protocols::protocol_version(&protocol_info) == PROTOCOL_VERSION_1, 1);
    //     assert!(defi_protocols::chain_id(&protocol_info) == CHAIN_ID_1, 2);
        
    //     object::delete(parent);
    //     test::end(scenario);
    // }

    // #[test]
    // fun test_defi_protocol_config_key() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     // 测试 DefiProtocolConfigKey 的创建和比较
    //     let key1 = DefiProtocolConfigKey {
    //         protocol_type: PROTOCOL_TYPE_1,
    //         protocol_version: PROTOCOL_VERSION_1,
    //         protocol_token_id: PROTOCOL_TOKEN_ID_1,
    //     };
        
    //     let key2 = DefiProtocolConfigKey {
    //         protocol_type: PROTOCOL_TYPE_1,
    //         protocol_version: PROTOCOL_VERSION_1,
    //         protocol_token_id: PROTOCOL_TOKEN_ID_1,
    //     };
        
    //     let key3 = DefiProtocolConfigKey {
    //         protocol_type: PROTOCOL_TYPE_2,
    //         protocol_version: PROTOCOL_VERSION_1,
    //         protocol_token_id: PROTOCOL_TOKEN_ID_1,
    //     };
        
    //     // 相同键应该相等
    //     assert!(key1 == key2, 0);
    //     // 不同键应该不相等
    //     assert!(key1 != key3, 1);
        
    //     test::end(scenario);
    // }

    // #[test]
    // fun test_defi_protocol_info() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     // 测试 DefiProtocolInfo 的创建和字段访问
    //     let info = DefiProtocolInfo {
    //         protocol_type: PROTOCOL_TYPE_1,
    //         protocol_version: PROTOCOL_VERSION_1,
    //         protocol_token_id: PROTOCOL_TOKEN_ID_1,
    //         chain_id: CHAIN_ID_1,
    //     };
        
    //     assert!(info.protocol_type == PROTOCOL_TYPE_1, 0);
    //     assert!(info.protocol_version == PROTOCOL_VERSION_1, 1);
    //     assert!(info.protocol_token_id == PROTOCOL_TOKEN_ID_1, 2);
    //     assert!(info.chain_id == CHAIN_ID_1, 3);
        
    //     test::end(scenario);
    // }

    // #[test]
    // fun test_defi_protocol_event() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     // 测试 DefiProtocolEvent 的创建
    //     let event = DefiProtocolEvent {
    //         protocol_type: PROTOCOL_TYPE_1,
    //         protocol_version: PROTOCOL_VERSION_1,
    //         protocol_token_id: PROTOCOL_TOKEN_ID_1,
    //         chain_id: CHAIN_ID_1,
    //     };
        
    //     assert!(event.protocol_type == PROTOCOL_TYPE_1, 0);
    //     assert!(event.protocol_version == PROTOCOL_VERSION_1, 1);
    //     assert!(event.protocol_token_id == PROTOCOL_TOKEN_ID_1, 2);
    //     assert!(event.chain_id == CHAIN_ID_1, 3);
        
    //     test::end(scenario);
    // }

    // #[test]
    // fun test_new_limiter_fast_path_for_testing() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     let mut parent = object::new(&mut ctx);
    //     let parent_id = object::uid_to_inner(&parent);
        
    //     // 使用测试专用函数
    //     defi_protocols::new_limiter_fast_path_for_testing(parent_id, &mut ctx);
        
    //     // 验证注册成功
    //     let config = defi_protocols::borrow(parent_id);
    //     assert!(config != &defi_protocols::DefiProtocolConfig { protocol_info_map: defi_protocols::table::new(&mut ctx) }, 0);
        
    //     object::delete(parent);
    //     test::end(scenario);
    // }

    // #[test]
    // #[expected_failure(abort_code = 0)] // EDefiProtocolConfigRegistryAlreadyExists
    // fun test_new_limiter_fast_path_already_exists() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     let mut parent = object::new(&mut ctx);
    //     let parent_id = object::uid_to_inner(&parent);
        
    //     // 第一次注册
    //     defi_protocols::new_limiter_fast_path_for_testing(parent_id, &mut ctx);
        
    //     // 第二次注册应该失败
    //     defi_protocols::new_limiter_fast_path_for_testing(parent_id, &mut ctx);
        
    //     object::delete(parent);
    //     test::end(scenario);
    // }

    // #[test]
    // fun test_complex_protocol_scenario() {
    //     let mut scenario = test::begin(@0x1);
    //     let mut ctx = ctx(&mut scenario);
        
    //     let mut parent = object::new(&mut ctx);
    //     let parent_id = object::uid_to_inner(&parent);
        
    //     defi_protocols::registry(parent_id, &mut ctx);
        
    //     // 添加多个不同版本的协议
    //     defi_protocols::add_defi_protocol(parent_id, 1, 1, 100, 1); // Uniswap V1
    //     defi_protocols::add_defi_protocol(parent_id, 1, 2, 100, 1); // Uniswap V2
    //     defi_protocols::add_defi_protocol(parent_id, 1, 3, 100, 1); // Uniswap V3
    //     defi_protocols::add_defi_protocol(parent_id, 2, 1, 200, 2); // Compound V1
    //     defi_protocols::add_defi_protocol(parent_id, 2, 2, 200, 2); // Compound V2
        
    //     // 验证所有协议
    //     let uniswap_v1 = defi_protocols::get_protocol_info(parent_id, 1, 1, 100);
    //     let uniswap_v2 = defi_protocols::get_protocol_info(parent_id, 1, 2, 100);
    //     let uniswap_v3 = defi_protocols::get_protocol_info(parent_id, 1, 3, 100);
    //     let compound_v1 = defi_protocols::get_protocol_info(parent_id, 2, 1, 200);
    //     let compound_v2 = defi_protocols::get_protocol_info(parent_id, 2, 2, 200);
        
    //     assert!(defi_protocols::protocol_type(&uniswap_v1) == 1, 0);
    //     assert!(defi_protocols::protocol_version(&uniswap_v1) == 1, 1);
    //     assert!(defi_protocols::protocol_type(&uniswap_v2) == 1, 2);
    //     assert!(defi_protocols::protocol_version(&uniswap_v2) == 2, 3);
    //     assert!(defi_protocols::protocol_type(&uniswap_v3) == 1, 4);
    //     assert!(defi_protocols::protocol_version(&uniswap_v3) == 3, 5);
    //     assert!(defi_protocols::protocol_type(&compound_v1) == 2, 6);
    //     assert!(defi_protocols::protocol_version(&compound_v1) == 1, 7);
    //     assert!(defi_protocols::protocol_type(&compound_v2) == 2, 8);
    //     assert!(defi_protocols::protocol_version(&compound_v2) == 2, 9);
        
    //     object::delete(parent);
    //     test::end(scenario);
    // }
}

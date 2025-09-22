// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::defi_protocols {
    use sui::event::emit;
    use sui::table::{Self, Table};

    use sui::dynamic_field;
    const KEY: vector<u8> = b"defi_protocols";
    // Error codes
    const EDefiProtocolConfigRegistryAlreadyExists: u64 = 0;
    const EDefiProtocolConfigNotFound: u64 = 1;

    const FEE_TYPE_PERCENTAGE: u8 = 0;
    const FEE_TYPE_FIXED: u8 = 1;
    

    /// token id 映射表
    public struct DefiProtocolConfig has store {
        protocol_info_map: Table<DefiProtocolKey, DefiProtocolInfo>,
    }

    /// 协议信息，存储每个协议的信息
    public struct DefiProtocolKey has store, copy,drop {
        /// 协议类型
        protocol_type: u64,
        /// 协议版本
        protocol_version: u64,
        /// token id
        protocol_token_id: u64,
        /// chain id
        chain_id: u8,
    }

    /// 协议信息，存储每个协议的信息
    public struct DefiProtocolInfo has store, copy,drop {
        /// 协议类型
        protocol_type: u64,
        /// 协议版本
        protocol_version: u64,
        /// token id
        protocol_token_id: u64,
        /// chain id
        chain_id: u8,
        /// fee type, 0: fixed, 1: percentage
        fee_type: u8,
        /// fee rate,decimal precision is 1e9
        fee_rate: u64,
    }

    /// 用户限额使用事件
    public struct DefiProtocolEvent has copy, drop {
        /// 协议类型
        protocol_type: u64,
        /// 协议版本
        protocol_version: u64,
        /// token id
        protocol_token_id: u64,
        /// chain id
        chain_id: u8,
    }

    public(package) fun borrow(parent_id: &UID): &DefiProtocolConfig{
        dynamic_field::borrow<vector<u8>,DefiProtocolConfig>(parent_id, KEY)
    }

    public(package) fun borrow_mut(parent_id: &mut UID): &mut DefiProtocolConfig{
        dynamic_field::borrow_mut<vector<u8>,DefiProtocolConfig>(parent_id, KEY)
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    public(package) fun registry(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            EDefiProtocolConfigRegistryAlreadyExists
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
    }

    public(package) fun add_defi_protocol(
        parent_id: &mut UID,
        protocol_type: u64,
        protocol_version: u64,
        protocol_token_id: u64,
        chain_id: u8,
        fee_type: u8,
        fee_rate: u64,
    ) {
        let self=borrow_mut(parent_id);
        let config_key = DefiProtocolKey { protocol_type, protocol_version, protocol_token_id, chain_id };
        if (!self.protocol_info_map.contains(config_key)) {
            self.protocol_info_map.add(config_key, DefiProtocolInfo { protocol_type, protocol_version, protocol_token_id, chain_id, fee_type, fee_rate });
        };
        *self.protocol_info_map.borrow_mut(config_key) = DefiProtocolInfo { protocol_type, protocol_version, protocol_token_id, chain_id, fee_type, fee_rate };
        emit(DefiProtocolEvent { protocol_type, protocol_version, protocol_token_id, chain_id });
    }

    public(package) fun delete_defi_protocol(
        parent_id: &mut UID,
        protocol_type: u64,
        protocol_version: u64,
        protocol_token_id: u64,
        chain_id: u8,
    ) {
        let self=borrow_mut(parent_id);
        let config_key = DefiProtocolKey { protocol_type, protocol_version, protocol_token_id, chain_id };
        assert!(self.protocol_info_map.contains(config_key), EDefiProtocolConfigNotFound);
        self.protocol_info_map.remove(config_key);
        emit(DefiProtocolEvent { protocol_type, protocol_version, protocol_token_id, chain_id });
    }

    public(package) fun manage_fee(
        parent_id: &mut UID,
        protocol_type: u64,
        protocol_version: u64,
        protocol_token_id: u64,
        chain_id: u8,
        lp_amount_withdraw: u64,
        amount_withdraw: u64,
        amount_in_record: u64,
        lp_amount_in_record: u64,
    ): u64 {
        let self=borrow(parent_id);
        let config_key = DefiProtocolKey { protocol_type, protocol_version, protocol_token_id, chain_id };
        assert!(self.protocol_info_map.contains(config_key), EDefiProtocolConfigNotFound);
        let protocol_info = self.protocol_info_map.borrow(config_key);

        let lp_amount_withdraw_u256 = lp_amount_withdraw as u256;
        let lp_amount_in_record_u256 = lp_amount_in_record as u256;
        let amount_withdraw_u256 = amount_withdraw as u256;
        let amount_in_record_u256 = amount_in_record as u256;
        //赎回的LP占比
        let lp_percent=lp_amount_withdraw_u256/(lp_amount_withdraw_u256+lp_amount_in_record_u256);
        //赎回的本金
        let principal=amount_in_record_u256*lp_percent;
        //赎回的利息
        let profit = amount_withdraw_u256-principal;
        //计算管理费用
        let fee: u64 = if (protocol_info.fee_type == FEE_TYPE_FIXED) {
            protocol_info.fee_rate
        } else {
            ((profit * (protocol_info.fee_rate as u256)) / 1_000_000_000) as u64
        };
        fee
    }



    /// 创建新的用户限额管理器
    public fun new(ctx: &mut TxContext): DefiProtocolConfig {
        DefiProtocolConfig {
            protocol_info_map: table::new(ctx),
        }
    }

    public fun get_protocol_info(
        parent_id: &mut UID,
        protocol_type: u64,
        protocol_version: u64,
        protocol_token_id: u64,
        chain_id: u8,
    ): DefiProtocolInfo {
        let self=borrow(parent_id);
        let config_key = DefiProtocolKey { protocol_type, protocol_version, protocol_token_id, chain_id };
        assert!(self.protocol_info_map.contains(config_key), EDefiProtocolConfigNotFound);
        *self.protocol_info_map.borrow(config_key)
    }

    //getter
    public fun chain_id(self: &DefiProtocolInfo): u8 {
        self.chain_id
    }

    public fun protocol_type(self: &DefiProtocolInfo): u64 {
        self.protocol_type
    }

    public fun protocol_version(self: &DefiProtocolInfo): u64 {
        self.protocol_version
    }

    public fun protocol_token_id(self: &DefiProtocolInfo): u64 {
        self.protocol_token_id
    }

    public fun fee_type(self: &DefiProtocolInfo): u8 {
        self.fee_type
    }

    public fun fee_rate(self: &DefiProtocolInfo): u64 {
        self.fee_rate
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test_only]
    public(package) fun new_defi_protocol_config_for_testing(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            EDefiProtocolConfigRegistryAlreadyExists
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
    }
}
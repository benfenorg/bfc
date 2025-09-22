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
        fee_type: u64,
        /// fee rate
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
        fee_type: u64,
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

    public fun fee_type(self: &DefiProtocolInfo): u64 {
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
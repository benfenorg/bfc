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
        /// token id 映射表
        token_id_map: Table<DefiProtocolConfigKey, DefiProtocolInfo>,
    }

    /// 协议信息，存储每个协议的信息
    public struct DefiProtocolInfo has store, copy,drop {
        /// 协议类型
        protocol_type: u64,
        /// 协议版本
        protocol_version: u64,
        /// token id
        token_id: u64,
        /// chain id
        chain_id: u8,
    }

    public struct DefiProtocolConfigKey has copy, drop ,store{
        token_id: u64,
    }

    /// 用户限额使用事件
    public struct DefiProtocolEvent has copy, drop {
        /// 协议类型
        protocol_type: u64,
        /// 协议版本
        protocol_version: u64,
        /// token id
        token_id: u64,
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
        token_id: u64,
        chain_id: u8,
        protocol_type: u64,
        protocol_version: u64,
    ) {
        let self=borrow_mut(parent_id);
        let config_key = DefiProtocolConfigKey { token_id };
        if (!self.token_id_map.contains(config_key)) {
            self.token_id_map.add(config_key, DefiProtocolInfo { protocol_type, protocol_version, token_id, chain_id });
        };
        *self.token_id_map.borrow_mut(config_key) = DefiProtocolInfo { protocol_type, protocol_version, token_id, chain_id };
        emit(DefiProtocolEvent { protocol_type, protocol_version, token_id, chain_id });
    }



    /// 创建新的用户限额管理器
    public fun new(ctx: &mut TxContext): DefiProtocolConfig {
        DefiProtocolConfig {
            token_id_map: table::new(ctx),
        }
    }

    public fun get_protocol_info(
        parent_id: &mut UID,
        token_id: u64,
    ): DefiProtocolInfo {
        let self=borrow(parent_id);
        let config_key = DefiProtocolConfigKey { token_id };
        assert!(self.token_id_map.contains(config_key), EDefiProtocolConfigNotFound);
        *self.token_id_map.borrow(config_key)
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test_only]
    public(package) fun new_limiter_fast_path_for_testing(parent_id: &mut UID,ctx: &mut TxContext) {
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
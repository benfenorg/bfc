// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::limiter_fast_path {
    use sui::clock::{Self, Clock};
    use sui::event::emit;
    use sui::table::{Self, Table};

    use bridge::chain_ids::Self;
    use sui::dynamic_field;
    const KEY: vector<u8> = b"limiter_fast_path";
    // Error codes
    const ELimiterFastPathRegistryAlreadyExists: u64 = 0;
    const EInvalidLimitAmount: u64 = 3;
    const EInvalidTimeWindow: u64 = 4;

    // Constants
    const USER_LIMIT_5K_IN_BUSD: u64 = 5000_000_000_000; // 10B in smallest unit
    const USER_LIMIT_1K_IN_BUSD: u64 = 1000_000_000_000; // 10B in smallest unit
    const USER_LIMIT_100_IN_BUSD: u64 = 100_000_000_000; 
    const DEFAULT_TIME_WINDOW_HOURS: u64 = 24;
    const TOKEN_ID_BUSD: u64 = 5;

    // 用户限额相关常量
    const USER_TIME_WINDOW_HOURS: u64 = 24;

    //////////////////////////////////////////////////////
    // Types
    //

    /// 用户限额记录，存储每个用户的限额信息
    public struct UserLimitRecord has store {
        /// 用户地址
        sender_address: vector<u8>,
        /// 当前滑动窗口的头（最新小时）
        hour_head: u64,
        /// 当前滑动窗口的尾（最早小时）
        hour_tail: u64,
        /// 每小时的使用量（长度最多为 USER_TIME_WINDOW_HOURS）
        per_hour_amounts: vector<u64>,
        /// 当前窗口内总使用量
        total_amount: u64,
    }

    /// 用户限额管理器
    public struct UserLimiter has store {
        /// 用户限额记录表
        user_records: Table<UserLimiterKey, UserLimitRecord>,
        /// 限额配置
        limit_configs: Table<LimitConfigKey, u64>,
        /// 全局默认限额
        default_limit: u64,
        /// 全局默认时间窗口
        default_time_window: u64,
        /// 是否启用用户限额
        enabled: bool,
    }

    public struct LimitConfigKey has copy, drop ,store{
        chain_id: u8,
        token_id: u64,
    }

    public struct UserLimiterKey has copy, drop ,store{
        sender_address: vector<u8>,
        chain_id: u8,
        token_id: u64,
    }

    /// 用户限额使用事件
    public struct UserLimitUsedEvent has copy, drop {
        sender_address: vector<u8>,
        used_amount: u64,
        remaining_limit: u64,
        window_start_hour: u64,
    }

    public(package) fun borrow(parent_id: &UID): &UserLimiter{
        dynamic_field::borrow<vector<u8>,UserLimiter>(parent_id, KEY)
    }

    public(package) fun borrow_mut(parent_id: &mut UID): &mut UserLimiter{
        dynamic_field::borrow_mut<vector<u8>,UserLimiter>(parent_id, KEY)
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    public(package) fun registry(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            ELimiterFastPathRegistryAlreadyExists
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
        initial_limiter_fast_path(parent_id);
    }

    public(package) fun initial_limiter_fast_path(parent_id: &mut UID) {
        //eth
        add_limiter(parent_id, chain_ids::eth_mainnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_5K_IN_BUSD);
        add_limiter(parent_id, chain_ids::eth_sepolia(), TOKEN_ID_BUSD as u64, USER_LIMIT_5K_IN_BUSD);
        add_limiter(parent_id, chain_ids::eth_custom(), TOKEN_ID_BUSD as u64, USER_LIMIT_100_IN_BUSD);
        //base
        add_limiter(parent_id, chain_ids::base_mainnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::base_testnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::base_custom(), TOKEN_ID_BUSD as u64, USER_LIMIT_100_IN_BUSD);
        //arb
        add_limiter(parent_id, chain_ids::arb_mainnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::arb_testnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::arb_custom(), TOKEN_ID_BUSD as u64, USER_LIMIT_100_IN_BUSD);
        //op
        add_limiter(parent_id, chain_ids::op_mainnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::op_testnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::op_custom(), TOKEN_ID_BUSD as u64, USER_LIMIT_100_IN_BUSD);
        //pol
        add_limiter(parent_id, chain_ids::pol_mainnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::pol_testnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::pol_custom(), TOKEN_ID_BUSD as u64, USER_LIMIT_100_IN_BUSD);
        //avax
        add_limiter(parent_id, chain_ids::avax_mainnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::avax_testnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::avax_custom(), TOKEN_ID_BUSD as u64, USER_LIMIT_100_IN_BUSD);
        //bsc
        add_limiter(parent_id, chain_ids::bsc_mainnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::bsc_testnet(), TOKEN_ID_BUSD as u64, USER_LIMIT_1K_IN_BUSD);
        add_limiter(parent_id, chain_ids::bsc_custom(), TOKEN_ID_BUSD as u64, USER_LIMIT_100_IN_BUSD);
    }

    public(package) fun add_limiter(
        parent_id: &mut UID,
        chain_id: u8,
        token_id: u64,
        amount:u64,
    ) {
        let self=borrow_mut(parent_id);
        let limit_config_key = LimitConfigKey { chain_id, token_id };
        if (!self.limit_configs.contains(limit_config_key)) {
            self.limit_configs.add(limit_config_key, amount);
        };
        *self.limit_configs.borrow_mut(limit_config_key) = amount;
    }



    /// 创建新的用户限额管理器
    public fun new(ctx: &mut TxContext): UserLimiter {
        UserLimiter {
            user_records: table::new(ctx),
            limit_configs: table::new(ctx),
            default_limit: USER_LIMIT_1K_IN_BUSD,
            default_time_window: DEFAULT_TIME_WINDOW_HOURS,
            enabled: true,
        }
    }

    /// 检查并记录用户限额使用
    public fun check_and_record_user_limit(
        parent_id: &mut UID,
        sender_address: vector<u8>,
        chain_id: u8,
        token_id: u64,
        amount: u64,
        clock: &Clock,
    ): bool {
        let self=borrow_mut(parent_id);
        if (!self.enabled) {
            return true
        };

        let current_hour = current_hour_since_epoch(clock);
        
        // 如果用户没有限额记录，初始化
        if (!table::contains(&self.user_records, UserLimiterKey { sender_address, chain_id, token_id })) {
            let record = UserLimitRecord {
                sender_address,
                hour_head: current_hour,
                hour_tail: current_hour,
                per_hour_amounts: vector[0],
                total_amount: 0,
            };
            table::add(&mut self.user_records, UserLimiterKey { sender_address, chain_id, token_id }, record);
        };

        let record = table::borrow_mut(&mut self.user_records, UserLimiterKey { sender_address, chain_id, token_id });
        adjust_user_limit_records(record, current_hour);

        let limit_config_key = LimitConfigKey { chain_id, token_id };
        let limit_amount = if (!table::contains(&self.limit_configs, limit_config_key)) {
            self.default_limit
        }else{
            let limit_config = table::borrow(&self.limit_configs, limit_config_key);
            *limit_config
        };

        // 检查限额是否足够
        if (record.total_amount + amount > limit_amount) {
            return false
        };

        // record the amount of this hour
        let new_amount = record.per_hour_amounts.pop_back() + amount;
        record.per_hour_amounts.push_back(new_amount);
        record.total_amount = record.total_amount + amount;

        emit(UserLimitUsedEvent {
            sender_address,
            used_amount: amount,
            remaining_limit: limit_amount - record.total_amount,
            window_start_hour: record.hour_tail,
        });
        true
    }

    /// 获取用户当前限额信息
    public fun get_user_limit_info(
        parent_id: &mut UID,
        sender_address: vector<u8>,
        chain_id: u8,
        token_id: u64,
        clock: &Clock,
    ): Option<UserLimitInfo> {
        let self=borrow_mut(parent_id);
        if (!table::contains(&self.user_records, UserLimiterKey { sender_address, chain_id, token_id })) {
            return option::none()
        };
        let record = table::borrow_mut(&mut self.user_records, UserLimiterKey { sender_address, chain_id, token_id });
        adjust_user_limit_records(record, current_hour_since_epoch(clock));

        let limit_config_key = LimitConfigKey { chain_id, token_id };
        let limit = if (!table::contains(&self.limit_configs, limit_config_key)) {
            self.default_limit
        }else{
            let limit_config = table::borrow(&self.limit_configs, limit_config_key);
            *limit_config
        };
        
        
        option::some(UserLimitInfo {
            sender_address,
            limit_amount: limit,
            used_amount: record.total_amount,
            remaining_limit: limit - record.total_amount,
            time_window_hours: USER_TIME_WINDOW_HOURS,
            window_start_hour: record.hour_tail,
            window_end_hour: record.hour_head,
        })
    }

    /// 获取用户剩余限额
    public fun get_user_remaining_limit(
        self: &mut UID,
        sender_address: vector<u8>,
        chain_id: u8,
        token_id: u64,
        clock: &Clock,
    ): u64 {
        let limit_info_opt = get_user_limit_info(self, sender_address, chain_id, token_id, clock);
        if (option::is_none(&limit_info_opt)) {
            let self=borrow_mut(self);
            if(!table::contains(&self.limit_configs, LimitConfigKey { chain_id, token_id })){
                return self.default_limit
            };
            let limit_config = table::borrow(&self.limit_configs, LimitConfigKey { chain_id, token_id });
            *limit_config
        } else {
            let limit_info = option::destroy_some(limit_info_opt);
            limit_info.remaining_limit
        }
    }

    // /// 重置用户限额使用记录
    // public fun reset_user_limit_usage(
    //     self: &mut UserLimiter,
    //     user_address: address,
    //     chain_id: u8,
    //     token_id: u64,
    //     clock: &Clock,
    //     ctx: &mut TxContext,
    // ) {
    //     if (!table::contains(&self.user_records, UserLimiterKey { user_address, chain_id, token_id })) {
    //         return
    //     };
    //     let record = table::borrow_mut(&mut self.user_records, UserLimiterKey { user_address, chain_id, token_id });
    //     let current_hour = current_hour_since_epoch(clock);
    //     record.hour_head = current_hour;
    //     record.hour_tail = current_hour;
    //     record.per_hour_amounts = vector[0];
    //     record.total_amount = 0;
    // }

    // /// 删除用户限额记录
    // public fun remove_user_limit(
    //     self: &mut UserLimiter,
    //     user_address: address,
    //     chain_id: u8,
    //     token_id: u64,
    // ) {
    //     if (table::contains(&self.user_records, UserLimiterKey { user_address, chain_id, token_id })) {
    //         let record = table::remove(&mut self.user_records, UserLimiterKey { user_address, chain_id, token_id });
    //         // Drop the record since we don't need it
    //         let UserLimitRecord { user_address: _, hour_head: _, hour_tail: _, per_hour_amounts: _, total_amount: _ } = record;
    //     }
    // }

    /// 设置全局默认限额
    public fun set_default_limit(self: &mut UID, new_limit: u64) {
        let self=borrow_mut(self);
        assert!(new_limit > 0, EInvalidLimitAmount);
        self.default_limit = new_limit;
    }

    public fun get_default_limit(self: &UID): u64 {
        let self=borrow(self);
        self.default_limit
    }

    /// 设置全局默认时间窗口
    public fun set_default_time_window(self: &mut UID, new_time_window: u64) {
        let self=borrow_mut(self);
        assert!(new_time_window > 0, EInvalidTimeWindow);
        self.default_time_window = new_time_window;
    }

    public fun get_default_time_window(self: &UID): u64 {
        let self=borrow(self);
        self.default_time_window
    }

    /// 启用或禁用用户限额
    public fun set_enabled(self: &mut UID, enabled: bool) {
        let self=borrow_mut(self);
        self.enabled = enabled;
    }

    public fun get_enabled(self: &UID): bool {
        let self=borrow(self);
        self.enabled
    }

    /// 获取用户限额记录数量
    public fun get_user_count(self: &UID): u64 {
        let self=borrow(self);
        table::length(&self.user_records)
    }

    //////////////////////////////////////////////////////
    // Internal functions
    //

    /// 获取当前小时（从 Unix epoch 开始）
    fun current_hour_since_epoch(clock: &Clock): u64 {
        clock::timestamp_ms(clock) / 3600000
    }

    // /// 计算总使用量
    // fun calculate_total_usage(per_hour_amounts: &vector<u64>, skip: u64, len: u64): u64 {
    //     let total = 0u64;
    //     let i = 0u64;
    //     calculate_total_usage_loop(per_hour_amounts, skip, len, total, i)
    // }

    // /// 计算总使用量的循环
    // fun calculate_total_usage_loop(
    //     per_hour_amounts: &vector<u64>, 
    //     skip: u64, 
    //     len: u64, 
    //     total: u64, 
    //     i: u64
    // ): u64 {
    //     if (i >= len as u64) {
    //         total
    //     } else {
    //         let new_total = if (i >= skip) { 
    //             total + *vector::borrow(per_hour_amounts, i) 
    //         } else { 
    //             total 
    //         };
    //         calculate_total_usage_loop(per_hour_amounts, skip, len, new_total, i + 1)
    //     }
    // }

    /// 滑动窗口，移除过期小时
    fun adjust_user_limit_records(record: &mut UserLimitRecord, current_hour: u64) {
        if(record.hour_head==current_hour) {
            return // nothing to backfill
        };
        
        let target_tail = current_hour - 23;
        // if `hour_head` is even older than 24 hours ago, it means all items in
        // `per_hour_amounts` are to be evicted.
        if (record.hour_head < target_tail) {
            record.per_hour_amounts = vector[];
            record.total_amount = 0;
            record.hour_tail = target_tail;
            record.hour_head = target_tail;
            record.per_hour_amounts.push_back(0);
        }else{
            // `hour_head` is within 24 hour range.
            // some items in `per_hour_amounts` are still valid, we remove stale hours.
            while(record.hour_tail < target_tail) {
                record.total_amount = record.total_amount - record.per_hour_amounts.remove(0);
                record.hour_tail = record.hour_tail + 1;
            }
        };
        
        // Backfill from hour_head to current hour
        while(record.hour_head < current_hour) {
            record.per_hour_amounts.push_back(0);
            record.hour_head = record.hour_head + 1;
        }
    }
    //////////////////////////////////////////////////////
    // Types for public interface
    //

    /// 用户限额信息
    public struct UserLimitInfo has copy, drop {
        sender_address: vector<u8>,
        limit_amount: u64,
        used_amount: u64,
        remaining_limit: u64,
        time_window_hours: u64,
        window_start_hour: u64,
        window_end_hour: u64,
    }

    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test_only]
    public(package) fun new_limiter_fast_path_for_testing(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            ELimiterFastPathRegistryAlreadyExists
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
    }

    #[test_only]
    public(package) fun registry_for_testing(parent_id: &mut UID) {
        initial_limiter_fast_path(parent_id);
    }



}
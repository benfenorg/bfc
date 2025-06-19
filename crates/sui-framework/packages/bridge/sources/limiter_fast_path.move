// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::limiter_fast_path {
    use sui::clock::{Self, Clock};
    use sui::event::emit;
    use sui::table::{Self, Table};
    use sui::address;
    use sui::vec_map::{Self, VecMap};
    use std::debug;

    use bridge::chain_ids::{Self, BridgeRoute};

    // Error codes
    const EUserLimitExceeded: u64 = 0;
    const EUserLimitNotFound: u64 = 1;
    const EInvalidLimitAmount: u64 = 2;
    const EInvalidTimeWindow: u64 = 3;

    // Constants
    const DEFAULT_USER_LIMIT: u64 = 10_000_000_000; // 10B in smallest unit
    const DEFAULT_TIME_WINDOW_HOURS: u64 = 24;
    const USD_VALUE_MULTIPLIER: u64 = 100000000; // 8 DP accuracy

    // 用户限额相关常量
    const USER_LIMIT_AMOUNT: u64 = 10_000_000_000; // 10B in smallest unit
    const USER_TIME_WINDOW_HOURS: u64 = 24;

    //////////////////////////////////////////////////////
    // Types
    //

    /// 用户限额记录，存储每个用户的限额信息
    public struct UserLimitRecord has store {
        /// 用户地址
        user_address: address,
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
    public struct UserLimiter has key {
        id: UID,
        /// 用户限额记录表
        user_records: Table<address, UserLimitRecord>,
        /// 全局默认限额
        default_limit: u64,
        /// 全局默认时间窗口
        default_time_window: u64,
        /// 是否启用用户限额
        enabled: bool,
    }

    /// 用户限额使用事件
    public struct UserLimitUsedEvent has copy, drop {
        user_address: address,
        used_amount: u64,
        remaining_limit: u64,
        window_start_hour: u64,
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    /// 创建新的用户限额管理器
    public fun new(ctx: &mut TxContext): UserLimiter {
        UserLimiter {
            id: object::new(ctx),
            user_records: table::new(ctx),
            default_limit: DEFAULT_USER_LIMIT,
            default_time_window: DEFAULT_TIME_WINDOW_HOURS,
            enabled: true,
        }
    }

    /// 检查并记录用户限额使用
    public fun check_and_record_user_limit(
        self: &mut UserLimiter,
        user_address: address,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ): bool {
        if (!self.enabled) {
            return true
        };

        let current_hour = current_hour_since_epoch(clock);
        
        // 如果用户没有限额记录，初始化
        if (!table::contains(&self.user_records, user_address)) {
            let record = UserLimitRecord {
                user_address,
                hour_head: current_hour,
                hour_tail: current_hour,
                per_hour_amounts: vector[0],
                total_amount: 0,
            };
            table::add(&mut self.user_records, user_address, record);
        };

        let record = table::borrow_mut(&mut self.user_records, user_address);
        adjust_user_limit_records(record, current_hour);

        // 检查限额是否足够
        if (record.total_amount + amount > USER_LIMIT_AMOUNT) {
            return false
        };

        // record the amount of this hour
        let new_amount = record.per_hour_amounts.pop_back() + amount;
        record.per_hour_amounts.push_back(new_amount);
        record.total_amount = record.total_amount + amount;

        emit(UserLimitUsedEvent {
            user_address,
            used_amount: amount,
            remaining_limit: USER_LIMIT_AMOUNT - record.total_amount,
            window_start_hour: record.hour_tail,
        });
        true
    }

    /// 获取用户当前限额信息
    public fun get_user_limit_info(
        self: &mut UserLimiter,
        user_address: address,
        clock: &Clock,
    ): Option<UserLimitInfo> {
        if (!table::contains(&self.user_records, user_address)) {
            return option::none()
        };
        let record = table::borrow_mut(&mut self.user_records, user_address);
        adjust_user_limit_records(record, current_hour_since_epoch(clock));
        
        option::some(UserLimitInfo {
            user_address,
            limit_amount: USER_LIMIT_AMOUNT,
            used_amount: record.total_amount,
            remaining_limit: USER_LIMIT_AMOUNT - record.total_amount,
            time_window_hours: USER_TIME_WINDOW_HOURS,
            window_start_hour: record.hour_tail,
            window_end_hour: record.hour_head,
        })
    }

    /// 获取用户剩余限额
    public fun get_user_remaining_limit(
        self: &mut UserLimiter,
        user_address: address,
        clock: &Clock,
    ): u64 {
        let limit_info_opt = get_user_limit_info(self, user_address, clock);
        if (option::is_none(&limit_info_opt)) {
            USER_LIMIT_AMOUNT
        } else {
            let limit_info = option::destroy_some(limit_info_opt);
            limit_info.remaining_limit
        }
    }

    /// 重置用户限额使用记录
    public fun reset_user_limit_usage(
        self: &mut UserLimiter,
        user_address: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        if (!table::contains(&self.user_records, user_address)) {
            return
        };
        let record = table::borrow_mut(&mut self.user_records, user_address);
        let current_hour = current_hour_since_epoch(clock);
        record.hour_head = current_hour;
        record.hour_tail = current_hour;
        record.per_hour_amounts = vector[0];
        record.total_amount = 0;
    }

    /// 删除用户限额记录
    public fun remove_user_limit(
        self: &mut UserLimiter,
        user_address: address,
    ) {
        if (table::contains(&self.user_records, user_address)) {
            let record = table::remove(&mut self.user_records, user_address);
            // Drop the record since we don't need it
            let UserLimitRecord { user_address: _, hour_head: _, hour_tail: _, per_hour_amounts: _, total_amount: _ } = record;
        }
    }

    /// 设置全局默认限额
    public fun set_default_limit(self: &mut UserLimiter, new_limit: u64) {
        assert!(new_limit > 0, EInvalidLimitAmount);
        self.default_limit = new_limit;
    }

    /// 设置全局默认时间窗口
    public fun set_default_time_window(self: &mut UserLimiter, new_time_window: u64) {
        assert!(new_time_window > 0, EInvalidTimeWindow);
        self.default_time_window = new_time_window;
    }

    /// 启用或禁用用户限额
    public fun set_enabled(self: &mut UserLimiter, enabled: bool) {
        self.enabled = enabled;
    }

    /// 获取用户限额记录数量
    public fun get_user_count(self: &UserLimiter): u64 {
        table::length(&self.user_records)
    }

    //////////////////////////////////////////////////////
    // Internal functions
    //

    /// 获取当前小时（从 Unix epoch 开始）
    fun current_hour_since_epoch(clock: &Clock): u64 {
        clock::timestamp_ms(clock) / 3600000
    }

    /// 计算总使用量
    fun calculate_total_usage(per_hour_amounts: &vector<u64>, skip: u64, len: u64): u64 {
        let total = 0u64;
        let i = 0u64;
        calculate_total_usage_loop(per_hour_amounts, skip, len, total, i)
    }

    /// 计算总使用量的循环
    fun calculate_total_usage_loop(
        per_hour_amounts: &vector<u64>, 
        skip: u64, 
        len: u64, 
        total: u64, 
        i: u64
    ): u64 {
        if (i >= len as u64) {
            total
        } else {
            let new_total = if (i >= skip) { 
                total + *vector::borrow(per_hour_amounts, i) 
            } else { 
                total 
            };
            calculate_total_usage_loop(per_hour_amounts, skip, len, new_total, i + 1)
        }
    }

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
        user_address: address,
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
    public fun create_for_testing(ctx: &mut TxContext): UserLimiter {
        new(ctx)
    }

    #[test_only]
    public fun test_check_and_record_user_limit(
        self: &mut UserLimiter,
        user_address: address,
        amount: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ): bool {
        check_and_record_user_limit(self, user_address, amount, clock, ctx)
    }

    #[test_only]
    public fun test_get_user_remaining_limit(
        self: &mut UserLimiter,
        user_address: address,
        clock: &Clock,
    ): u64 {
        get_user_remaining_limit(self, user_address, clock)
    }

    #[test_only]
    public fun test_reset_user_limit_usage(
        self: &mut UserLimiter,
        user_address: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        reset_user_limit_usage(self, user_address, clock, ctx)
    }

    #[test_only]
    public fun test_remove_user_limit(
        self: &mut UserLimiter,
        user_address: address,
    ) {
        remove_user_limit(self, user_address)
    }

    #[test_only]
    public fun test_set_enabled(self: &mut UserLimiter, enabled: bool) {
        set_enabled(self, enabled)
    }

    #[test_only]
    public fun test_get_user_count(self: &UserLimiter): u64 {
        get_user_count(self)
    }

    #[test_only]
    public(package) fun default_limit(self: &UserLimiter): u64 {
        self.default_limit
    }

    #[test_only]
    public(package) fun default_time_window(self: &UserLimiter): u64 {
        self.default_time_window
    }

    #[test_only]
    public(package) fun enabled(self: &UserLimiter): bool {
        self.enabled
    }


}
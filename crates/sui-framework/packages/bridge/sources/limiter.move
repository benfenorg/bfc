// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::limiter {
    use sui::bag;
    use sui::bag::Bag;
    use sui::clock::{Self, Clock};
    use sui::event::emit;
    use sui::vec_map::{Self, VecMap};
    use sui::dynamic_field;

    use bridge::chain_ids::{Self, BridgeRoute};
    use bridge::treasury::BridgeTreasury;

    const ELimitNotFoundForRoute: u64 = 0;
    const EExternalLimitKeyExist: u64 = 1;
    const EExternalLimitNotFoundForRoute: u64 = 3;

    // TODO: U64::MAX, make this configurable?
    const MAX_TRANSFER_LIMIT: u64 = 18_446_744_073_709_551_615;

    const USD_VALUE_MULTIPLIER: u64 = 100000000; // 8 DP accuracy

    const DEFAULT_MAX_MINT_BUSD_LIMIT: u64 = 500_000 * 1000_000_000; // 50W BUSD

    const EXTERNAL_LIMITS_KEY: vector<u8> = b"bridge_external_limits";

    //////////////////////////////////////////////////////
    // Types
    //

    public struct TransferLimiter has store {
        transfer_limits: VecMap<BridgeRoute, u64>,
        // Per hour transfer amount for each bridge route
        transfer_records: VecMap<BridgeRoute, TransferRecord>,
        // Each time the maximum mint value
        max_mint_busd_limit: u64,
    }
    public struct ExternalLimiter has store {
        transfer_out_limits: VecMap<BridgeRoute, u64>,
        external: Bag,
    }

    public struct TransferRecord has store {
        hour_head: u64,
        hour_tail: u64,
        per_hour_amounts: vector<u64>,
        // total amount in USD, 4 DP accuracy, so 10000 => 1USD
        total_amount: u64
    }

    public struct UpdateRouteLimitEvent has copy, drop {
        sending_chain: u8,
        receiving_chain: u8,
        new_limit: u64,
    }

    //////////////////////////////////////////////////////
    // Public functions
    //

    // Abort if the route limit is not found
    public fun get_route_limit(self: &TransferLimiter, route: &BridgeRoute): u64 {
        self.transfer_limits[route]
    }

    // Return the mint busd max limit
    public fun get_mint_busd_max_limit(self: &TransferLimiter): u64 {
        self.max_mint_busd_limit
    }

    public(package) fun set_mint_busd_max_limit(self: &mut TransferLimiter, new_limit: u64) {
        self.max_mint_busd_limit = new_limit;
    }

    //////////////////////////////////////////////////////
    // Internal functions
    //

    public(package) fun new(): TransferLimiter {
        // hardcoded limit for bridge genesis
        TransferLimiter {
            transfer_limits: initial_transfer_limits(),
            transfer_records: vec_map::empty(),
            max_mint_busd_limit: DEFAULT_MAX_MINT_BUSD_LIMIT,
        }
    }

    public(package) fun new_external_limits(parent_id: &mut UID, ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, EXTERNAL_LIMITS_KEY),
            EExternalLimitKeyExist
        );
        dynamic_field::add(
            parent_id,
            EXTERNAL_LIMITS_KEY,
            ExternalLimiter {
                transfer_out_limits: vec_map::empty(),
                external: bag::new(ctx),
            },
        );
        initial_external_limits(parent_id);
    }

    public(package) fun initial_external_limits(
        parent_id: &mut UID,
    ) {
        // assert!(dynamic_field::exists_(parent_id, EXTERNAL_LIMITS_KEY), 9999);
        let external_limiter = dynamic_field::borrow_mut<vector<u8>, ExternalLimiter>(parent_id, EXTERNAL_LIMITS_KEY);
        // Initialize the external limits with the default values
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::btc_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::eth_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::bsc_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::base_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::op_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::arb_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::pol_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::avax_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::tron_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::solana_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::ltc_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::doge_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::sui_official_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route(chain_ids::sui_mainnet(), chain_ids::aptos_mainnet()),
            100_000 * USD_VALUE_MULTIPLIER,
        );

        // Testnet and custom chains
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::eth_sepolia()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::eth_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::btc_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::bsc_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::bsc_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::base_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::base_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::op_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::op_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::arb_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::arb_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::pol_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::pol_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::avax_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::avax_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::tron_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::solana_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::ltc_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::doge_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::sui_official_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_testnet(), chain_ids::aptos_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        //custom chains
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::eth_sepolia()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::eth_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::btc_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::bsc_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::bsc_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::base_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::base_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::op_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::op_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::arb_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::arb_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::pol_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::pol_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::avax_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::avax_custom()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::tron_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::solana_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::ltc_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::doge_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::sui_official_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
        add_external_out_limit(
            external_limiter,
            &chain_ids::get_route( chain_ids::sui_custom(), chain_ids::aptos_testnet()),
            100 * USD_VALUE_MULTIPLIER,
        );
    }

    public(package) fun get_external_limiter(
        parent_id: &UID
    ): &ExternalLimiter {
        dynamic_field::borrow<vector<u8>, ExternalLimiter>(parent_id, EXTERNAL_LIMITS_KEY)
    }

    public(package) fun update_external_out_limit(
        parent_id: &mut UID,
        route: &BridgeRoute,
        limit: u64,
    ) {
        let external_limiter = dynamic_field::borrow_mut<vector<u8>, ExternalLimiter>(parent_id, EXTERNAL_LIMITS_KEY);
        external_limiter.add_external_out_limit(route, limit);
    }

    public(package) fun add_external_out_limit(
        external_limits: &mut ExternalLimiter,
        route: &BridgeRoute,
        limit: u64,
    ) {
        if (!external_limits.transfer_out_limits.contains(route)) {
            external_limits.transfer_out_limits.insert(*route, limit);
        } else {
            *&mut external_limits.transfer_out_limits[route] = limit;
        };
    }

    public(package) fun get_external_out_limit(
        parent_id: &UID,
        route: &BridgeRoute
    ): u64 {
        let external_limiter = get_external_limiter(parent_id);
        let limit = external_limiter.transfer_out_limits.try_get(route);
        assert!(limit.is_some(), EExternalLimitNotFoundForRoute);
        limit.destroy_some()
    }


    public fun get_available_claim_amount<T>(
        self: &TransferLimiter,
        treasury: &BridgeTreasury,
        route: BridgeRoute,
    ): u128{
        let route_limit = self.transfer_limits.try_get(&route);
        assert!(route_limit.is_some(), ELimitNotFoundForRoute);
        let route_limit = route_limit.destroy_some();

        let price = (treasury.notional_value<T>() as u128);
        if (price == 0) {
            return 0
        };
        let route_limit_adjusted =
            (route_limit as u128) * (USD_VALUE_MULTIPLIER as u128);

        if (!self.transfer_records.contains(&route)) {
            return (route_limit_adjusted / price)
        };

        let record=self.transfer_records.get(&route);

        let total_adjusted= (record.total_amount as u128 ) * (USD_VALUE_MULTIPLIER as u128);
        if (total_adjusted >= route_limit_adjusted){
            return 0
        };

        let available_amount=((route_limit_adjusted-total_adjusted) / price);
        available_amount
    }
    public(package) fun check_and_record_sending_transfer<T>(
        self: &mut TransferLimiter,
        treasury: &BridgeTreasury,
        clock: &Clock,
        route: BridgeRoute,
        amount: u64
    ): bool {
        // Create record for route if not exists
        if (!self.transfer_records.contains(&route)) {
            self.transfer_records.insert(route, TransferRecord {
                hour_head: 0,
                hour_tail: 0,
                per_hour_amounts: vector[],
                total_amount: 0
            })
        };
        let record = self.transfer_records.get_mut(&route);
        let current_hour_since_epoch = current_hour_since_epoch(clock);

        record.adjust_transfer_records(current_hour_since_epoch);

        // Get limit for the route
        let route_limit = self.transfer_limits.try_get(&route);
        assert!(route_limit.is_some(), ELimitNotFoundForRoute);
        let route_limit = route_limit.destroy_some();
        let route_limit_adjusted =
            (route_limit as u128) * (treasury.decimal_multiplier<T>() as u128);

        // Compute notional amount
        // Upcast to u128 to prevent overflow, to not miss out on small amounts.
        let value = (treasury.notional_value<T>() as u128);
        let notional_amount_with_token_multiplier = value * (amount as u128);

        // Check if transfer amount exceed limit
        // Upscale them to the token's decimal.
        if ((record.total_amount as u128)
            * (treasury.decimal_multiplier<T>() as u128)
            + notional_amount_with_token_multiplier > route_limit_adjusted
        ) {
            return false
        };

        // Now scale down to notional value
        let notional_amount = notional_amount_with_token_multiplier
            / (treasury.decimal_multiplier<T>() as u128);
        // Should be safe to downcast to u64 after dividing by the decimals
        let notional_amount = (notional_amount as u64);

        // Record transfer value
        let new_amount = record.per_hour_amounts.pop_back() + notional_amount;
        record.per_hour_amounts.push_back(new_amount);
        record.total_amount = record.total_amount + notional_amount;
        true
    }

    public(package) fun update_route_limit(
        self: &mut TransferLimiter,
        route: &BridgeRoute,
        new_usd_limit: u64
    ) {
        let receiving_chain = *route.destination();

        if (!self.transfer_limits.contains(route)) {
            self.transfer_limits.insert(*route, new_usd_limit);
        } else {
            *&mut self.transfer_limits[route] = new_usd_limit;
        };

        emit(UpdateRouteLimitEvent {
            sending_chain: *route.source(),
            receiving_chain,
            new_limit: new_usd_limit,
        })
    }

    // Current hour since unix epoch
    fun current_hour_since_epoch(clock: &Clock): u64 {
        clock::timestamp_ms(clock) / 3600000
    }

    fun adjust_transfer_records(self: &mut TransferRecord, current_hour_since_epoch: u64) {
        if (self.hour_head == current_hour_since_epoch) {
            if(self.per_hour_amounts.length() == 0) {
                self.per_hour_amounts.push_back(0);
            };
            return // nothing to backfill
        };

        let target_tail = current_hour_since_epoch - 23;

        // If `hour_head` is even older than 24 hours ago, it means all items in
        // `per_hour_amounts` are to be evicted.
        if (self.hour_head < target_tail) {
            self.per_hour_amounts = vector[];
            self.total_amount = 0;
            self.hour_tail = target_tail;
            self.hour_head = target_tail;
            // Don't forget to insert this hour's record
            self.per_hour_amounts.push_back(0);
        } else {
            // self.hour_head is within 24 hour range.
            // some items in `per_hour_amounts` are still valid, we remove stale hours.
            while (self.hour_tail < target_tail) {
                self.total_amount = self.total_amount - self.per_hour_amounts.remove(0);
                self.hour_tail = self.hour_tail + 1;
            }
        };

        // Backfill from hour_head to current hour
        while (self.hour_head < current_hour_since_epoch) {
            self.per_hour_amounts.push_back(0);
            self.hour_head = self.hour_head + 1;
        }
    }

    // It's tedious to list every pair, but it's safer to do so so we don't
    // accidentally turn off limiter for a new production route in the future.
    // Note limiter only takes effects on the receiving chain, so we only need to
    // specify routes from Ethereum to Sui.
    fun initial_transfer_limits(): VecMap<BridgeRoute, u64> {
        let mut transfer_limits = vec_map::empty();
        // 5M limit on Sui -> Ethereum mainnet
        transfer_limits.insert(
            chain_ids::get_route(chain_ids::eth_mainnet(), chain_ids::sui_mainnet()),
            500_000 * USD_VALUE_MULTIPLIER
        );
        // MAX limit for testnet and devnet
        transfer_limits.insert(
            chain_ids::get_route(chain_ids::eth_sepolia(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        transfer_limits.insert(
            chain_ids::get_route(chain_ids::eth_sepolia(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        transfer_limits.insert(
            chain_ids::get_route(chain_ids::eth_custom(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        transfer_limits.insert(
            chain_ids::get_route(chain_ids::eth_custom(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );
        transfer_limits
    }

    public(package) fun update_transfer_limits(self: &mut TransferLimiter)
    {
        // 1B limit on Sui -> BSC mainnet
        self.update_route_limit(
            &chain_ids::get_route(chain_ids::bsc_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );
        // 1B limit on Sui -> Base mainnet
        self.update_route_limit(
            &chain_ids::get_route(chain_ids::base_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::op_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::bsc_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::pol_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::arb_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::avax_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::solana_mainnet(), chain_ids::sui_mainnet()),
            1_000_000_000 * USD_VALUE_MULTIPLIER
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::bsc_testnet(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::bsc_testnet(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::bsc_custom(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::bsc_custom(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::base_testnet(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::base_testnet(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::base_custom(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::base_custom(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::op_testnet(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::op_testnet(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::op_custom(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::op_custom(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::pol_testnet(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::pol_testnet(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::pol_custom(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::pol_custom(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::arb_testnet(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::arb_testnet(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::arb_custom(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::arb_custom(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::avax_testnet(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::avax_testnet(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::avax_custom(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::avax_custom(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::solana_testnet(), chain_ids::sui_testnet()),
            MAX_TRANSFER_LIMIT
        );

        self.update_route_limit(
            &chain_ids::get_route(chain_ids::solana_testnet(), chain_ids::sui_custom()),
            MAX_TRANSFER_LIMIT
        );
    }
    //////////////////////////////////////////////////////
    // Test functions
    //

    #[test_only]
    public(package) fun transfer_limits(limiter: &TransferLimiter): &VecMap<BridgeRoute, u64> {
        &limiter.transfer_limits
    }

    #[test_only]
    public(package) fun transfer_limits_mut(
        limiter: &mut TransferLimiter,
    ): &mut VecMap<BridgeRoute, u64> {
        &mut limiter.transfer_limits
    }

    #[test_only]
    public(package) fun transfer_records(
        limiter: &TransferLimiter,
    ): &VecMap<BridgeRoute, TransferRecord> {
        &limiter.transfer_records
    }

    #[test_only]
    public(package) fun transfer_records_mut(
        limiter: &mut TransferLimiter,
    ): &mut VecMap<BridgeRoute, TransferRecord> {
        &mut limiter.transfer_records
    }

    #[test_only]
    public(package) fun usd_value_multiplier(): u64 {
        USD_VALUE_MULTIPLIER
    }

    #[test_only]
    public(package) fun max_transfer_limit(): u64 {
        MAX_TRANSFER_LIMIT
    }

    #[test_only]
    public(package) fun make_transfer_limiter(): TransferLimiter {
        TransferLimiter {
            transfer_limits: vec_map::empty(),
            transfer_records: vec_map::empty(),
            max_mint_busd_limit: DEFAULT_MAX_MINT_BUSD_LIMIT,
        }
    }

    #[test_only]
    public(package) fun total_amount(record: &TransferRecord): u64 {
        record.total_amount
    }

    #[test_only]
    public(package) fun per_hour_amounts(record: &TransferRecord): &vector<u64> {
        &record.per_hour_amounts
    }

    #[test_only]
    public(package) fun hour_head(record: &TransferRecord): u64 {
        record.hour_head
    }

    #[test_only]
    public(package) fun hour_tail(record: &TransferRecord): u64 {
        record.hour_tail
    }

    #[test_only]
    public(package) fun unpack_route_limit_event(event: UpdateRouteLimitEvent):
        (u8, u8, u64)
    {
        (event.sending_chain, event.receiving_chain, event.new_limit)
    }
}

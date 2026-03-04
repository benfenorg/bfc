module bridge::bridge_min_config{
    use sui::table::{Self, Table};
    use sui::dynamic_field;
    use bridge::chain_ids;
    use bridge::tokenlist;

    use bridge::bridge_fee;
    use bridge::treasury;

    const KEY: vector<u8> = b"bridge_min_config";
    const EBridgeMinConfigRegistryAlreadyExists: u64=0;

    public struct BridgeMinConfig has store {
        min_limit_out: Table<u64, u64>,
        min_limit_in: Table<u64, u64>,
        fee_limit: Table<u64, Table<u64, FeeLimit>>,
    }

    public struct FeeLimit has store, copy, drop {
        cross_in_fee_min: u64,
        cross_out_fee_min: u64,
    }

    public(package) fun new_bridge_min_config_registry(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            EBridgeMinConfigRegistryAlreadyExists
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
    }

    public(package) fun new(ctx: &mut TxContext): BridgeMinConfig {
        let (min_limit_out, min_limit_in, fee_limit) = empty(ctx);
        BridgeMinConfig {
            min_limit_out,
            min_limit_in,
            fee_limit,
        }
    }

    fun empty(ctx: &mut TxContext): (
        Table<u64, u64>,
        Table<u64, u64>,
        Table<u64, Table<u64, FeeLimit>>,
    ) {
        let min_limit_out = table::new<u64, u64>(ctx);
        let min_limit_in = table::new<u64, u64>(ctx);
        let fee_limit = table::new<u64, Table<u64, FeeLimit>>(ctx);
        (min_limit_out, min_limit_in, fee_limit)
    }

    public(package) fun borrow(parent_id: &UID): &BridgeMinConfig{
        dynamic_field::borrow<vector<u8>,BridgeMinConfig>(parent_id, KEY)
    }

    public(package) fun borrow_mut(parent_id: &mut UID): &mut BridgeMinConfig{
        dynamic_field::borrow_mut<vector<u8>,BridgeMinConfig>(parent_id, KEY)
    }

    public fun exists(parent_id: &UID): bool {
        dynamic_field::exists_(parent_id, KEY)
    }

    public(package) fun set_min_limit_cross_out(
        parent_id: &mut UID,
        chain_id: u64,
        amount: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.min_limit_out.contains(chain_id)) {
            self.min_limit_out.add(chain_id, amount);
        }else{
            *self.min_limit_out.borrow_mut(chain_id)=amount
        }
    }

    public(package) fun set_min_limit_cross_in(
        parent_id: &mut UID,
        chain_id: u64,
        amount: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.min_limit_in.contains(chain_id)) {
            self.min_limit_in.add(chain_id, amount);
        }else{
            *self.min_limit_in.borrow_mut(chain_id)=amount
        }
    }

    public(package) fun set_fee_limit(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        cross_in_fee_min: u64,
        cross_out_fee_min: u64,
        ctx: &mut TxContext,
    ) {
        let self=borrow_mut(parent_id);
        if (!self.fee_limit.contains(chain_id)) {
            self.fee_limit.add(chain_id, table::new(ctx));
        };
        let limit = FeeLimit { cross_in_fee_min, cross_out_fee_min };
        if (!self.fee_limit.borrow(chain_id).contains(token_id)) {
            self.fee_limit.borrow_mut(chain_id).add(token_id, limit);
        }else{
            *self.fee_limit.borrow_mut(chain_id).borrow_mut(token_id)=limit
        }
    }

    public(package) fun set_min_fee_cross_out(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        fee_amount: u64,
        ctx: &mut TxContext,
    ) {
        let self=borrow_mut(parent_id);
        if (!self.fee_limit.contains(chain_id)) {
            self.fee_limit.add(chain_id, table::new(ctx));
        };
        if (!self.fee_limit.borrow(chain_id).contains(token_id)) {
            self.fee_limit.borrow_mut(chain_id).add(token_id, FeeLimit { cross_in_fee_min: 0, cross_out_fee_min: fee_amount });
        }else{
            let mut v = self.fee_limit.borrow_mut(chain_id).borrow_mut(token_id);
            v.cross_out_fee_min = fee_amount
        }
    }

    public(package) fun set_min_fee_cross_in(
        parent_id: &mut UID,
        chain_id: u64,
        token_id: u64,
        fee_amount: u64,
        ctx: &mut TxContext,
    ) {
        let self=borrow_mut(parent_id);
        if (!self.fee_limit.contains(chain_id)) {
            self.fee_limit.add(chain_id, table::new(ctx));
        };
        if (!self.fee_limit.borrow(chain_id).contains(token_id)) {
            self.fee_limit.borrow_mut(chain_id).add(token_id, FeeLimit { cross_in_fee_min: fee_amount, cross_out_fee_min: 0 });
        }else{
            let mut v = self.fee_limit.borrow_mut(chain_id).borrow_mut(token_id);
            v.cross_in_fee_min = fee_amount
        }
    }

    public fun get_min_limit_cross_out(parent_id: &UID,chain_id: u64): u64{
        let self=borrow(parent_id);
        if (!self.min_limit_out.contains(chain_id)) {
            return 0
        };
        *self.min_limit_out.borrow(chain_id)
    }

    public fun get_min_limit_cross_in(parent_id: &UID,chain_id: u64): u64{
        let self=borrow(parent_id);
        if (!self.min_limit_in.contains(chain_id)) {
            return 0
        };
        *self.min_limit_in.borrow(chain_id)
    }

    public fun get_min_fee_cross_out(parent_id: &UID,chain_id: u64, token_id: u64): u64{
        if (!dynamic_field::exists_(parent_id, KEY)) {
            return 0
        };
        let self=borrow(parent_id);
        if (!self.fee_limit.contains(chain_id)) {
            return 0
        };
        let inner = self.fee_limit.borrow(chain_id);
        if (!inner.contains(token_id)) {
            return 0
        };
        inner.borrow(token_id).cross_out_fee_min
    }

    public fun get_min_fee_cross_in(parent_id: &UID,chain_id: u64, token_id: u64): u64{
        if (!dynamic_field::exists_(parent_id, KEY)) {
            return 0
        };
        let self=borrow(parent_id);
        if (!self.fee_limit.contains(chain_id)) {
            return 0
        };
        let inner = self.fee_limit.borrow(chain_id);
        if (!inner.contains(token_id)) {
            return 0
        };
        inner.borrow(token_id).cross_in_fee_min
    }

    /// Returns true if cross-in amount meets the chain's single-tx cross-in minimum. If registry is not installed or not configured, no minimum is enforced and returns true.
    /// For BTC chains min is in satoshi and amount is token amount (satoshi), compared directly; for other chains min is 8-decimal USD, amount is converted to USD before comparison.
    public fun check_cross_in_amount_ok(
        parent_id: &UID,
        treasury: &treasury::BridgeTreasury,
        chain_id: u64,
        token_id: u64,
        amount: u64,
    ): bool {
        if (!dynamic_field::exists_(parent_id, KEY)) {
            return true
        };
        let min = get_min_limit_cross_in(parent_id, chain_id);
        if (chain_id == (chain_ids::btc_mainnet() as u64) || chain_id == (chain_ids::btc_testnet() as u64)) {
            amount >= min
        } else {
            let amount_usd = treasury::get_amount_in_usd_by_token_id(treasury, token_id, amount);
            amount_usd >= min
        }
    }

    /// Returns true if cross-out amount meets the chain's single-tx cross-out minimum. If registry is not installed or not configured, no minimum is enforced and returns true.
    /// For BTC chains min is in satoshi and amount is token amount (satoshi), compared directly; for other chains min is 8-decimal USD, amount is converted to USD before comparison.
    public fun check_cross_out_amount_ok(
        parent_id: &UID,
        treasury: &treasury::BridgeTreasury,
        chain_id: u64,
        token_id: u64,
        amount: u64,
    ): bool {
        if (!dynamic_field::exists_(parent_id, KEY)) {
            return true
        };
        let min = get_min_limit_cross_out(parent_id, chain_id);
        if (chain_id == (chain_ids::btc_mainnet() as u64) || chain_id == (chain_ids::btc_testnet() as u64)) {
            amount >= min
        } else {
            let amount_usd = treasury::get_amount_in_usd_by_token_id(treasury, token_id, amount);
            amount_usd >= min
        }
    }

    /// Computes cross-out fee and returns the greater of that and the configured cross-out fee minimum.
    /// Configured fee minimum is in token amount (smallest unit for the token, e.g. wei for ETH); comparison is done in token.
    public fun get_effective_cross_out_fee(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
        amount: u64,
    ): u64 {
        let fee_token = bridge_fee::calculate_cross_out_fee_amount(parent_id, chain_id, token_id, amount);
        let min_token = get_min_fee_cross_out(parent_id, chain_id, token_id);
        if (fee_token >= min_token) { fee_token } else { min_token }
    }

    /// Computes cross-in fee and returns the greater of that and the configured cross-in fee minimum.
    /// Configured fee minimum is in token amount (smallest unit for the token, e.g. wei for ETH); comparison is done in token.
    public fun get_effective_cross_in_fee(
        parent_id: &UID,
        chain_id: u64,
        token_id: u64,
        amount: u64,
    ): u64 {
        let fee_token = bridge_fee::calculate_cross_in_fee_amount(parent_id, chain_id, token_id, amount);
        let min_token = get_min_fee_cross_in(parent_id, chain_id, token_id);
        if (fee_token >= min_token) { fee_token } else { min_token }
    }

    /// Converts token amount (in smallest unit for the given token_id) to USD value with 8 decimals (same as limiter USD precision).
    /// Requires treasury that holds price info for the token.
    public fun get_usd_value(treasury: &treasury::BridgeTreasury, token_id: u64, amount: u64): u64 {
        treasury::get_amount_in_usd_by_token_id(treasury, token_id, amount)
    }

    public(package) fun initial_min_fee_limits(parent_id: &mut UID, ctx: &mut TxContext) {
        // ========================
        // Mainnet configuration
        // ========================
        {
            // BTC: min single cross-in/out = 0.0001 BTC (10_000 satoshi)
            let chain = chain_ids::btc_mainnet() as u64;
            set_min_limit_cross_in(parent_id, chain, 10_000);
            set_min_limit_cross_out(parent_id, chain, 10_000);
        };
        {
            // ETH: min_in = 0 USD, min_out = 1 USD (8 dp: 100_000_000)
            let chain = chain_ids::eth_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Base: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::base_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Optimism: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::op_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // BSC: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::bsc_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Polygon: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::pol_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Arbitrum: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::arb_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // AVAX-C: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::avax_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Tron: min_in = 0 USD, min_out = 10 USD (8 dp: 1_000_000_000)
            let chain = chain_ids::tron_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 1_000_000_000);

            // Tron USDT min cross-out fee: 5 USDT (6 dp: 5_000_000)
            let usdt = tokenlist::get_usdt_token_id();
            // set_min_fee_cross_in(parent_id, chain, usdt, 0, ctx);
            set_min_fee_cross_out(parent_id, chain, usdt, 5_000_000, ctx);
        };
        {
            // Solana: min_in = 0 USD, min_out = 10 USD
            let chain = chain_ids::solana_mainnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 1_000_000_000);

            // Solana stablecoins min cross-out fee: USDT/USDC = 0.5 USD (6 dp: 500_000)
            let usdt = tokenlist::get_usdt_token_id();
            let usdc = tokenlist::get_usdc_token_id();
            set_min_fee_cross_out(parent_id, chain, usdt, 500_000, ctx);
            set_min_fee_cross_in(parent_id, chain, usdt, 0, ctx);
            set_min_fee_cross_out(parent_id, chain, usdc, 500_000, ctx);
            set_min_fee_cross_in(parent_id, chain, usdc, 0, ctx);
        };

        // ========================
        // Testnet configuration
        // ========================
        {
            // BTC Testnet: 0.0001 BTC (satoshi)
            let chain = chain_ids::btc_testnet() as u64;
            set_min_limit_cross_in(parent_id, chain, 10_000);
            set_min_limit_cross_out(parent_id, chain, 10_000);
        };
        {
            // ETH Sepolia: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::eth_sepolia() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Base Testnet: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::base_testnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Optimism Testnet: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::op_testnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // BSC Testnet: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::bsc_testnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Polygon Testnet: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::pol_testnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Arbitrum Testnet: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::arb_testnet() as u64;
            set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // AVAX-C Testnet: min_in = 0 USD, min_out = 1 USD
            let chain = chain_ids::avax_testnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 100_000_000);
        };
        {
            // Tron Testnet: min_in = 0 USD, min_out = 10 USD
            let chain = chain_ids::tron_testnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 1_000_000_000);

            // Tron Testnet USDT min cross-out fee: 5 USDT
            let usdt = tokenlist::get_usdt_token_id();
            set_min_fee_cross_out(parent_id, chain, usdt, 5_000_000, ctx);
            // set_min_fee_cross_in(parent_id, chain, usdt, 0, ctx);
        };
        {
            // Solana Testnet: min_in = 0 USD, min_out = 10 USD
            let chain = chain_ids::solana_testnet() as u64;
            // set_min_limit_cross_in(parent_id, chain, 0);
            set_min_limit_cross_out(parent_id, chain, 1_000_000_000);
            // Solana Testnet stablecoins min cross-out fee: USDT/USDC = 0.5 USD
            let usdt = tokenlist::get_usdt_token_id();
            let usdc = tokenlist::get_usdc_token_id();
            set_min_fee_cross_out(parent_id, chain, usdt, 500_000, ctx);
            // set_min_fee_cross_in(parent_id, chain, usdt, 0, ctx);
            set_min_fee_cross_out(parent_id, chain, usdc, 500_000, ctx);
            // set_min_fee_cross_in(parent_id, chain, usdc, 0, ctx);
        };
    }
}

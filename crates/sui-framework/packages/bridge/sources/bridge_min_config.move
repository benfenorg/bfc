module bridge::bridge_min_config{
    use sui::table::{Self, Table};
    use sui::dynamic_field;

    use bridge::bridge_fee;
    use bridge::chain_ids;
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

    /// 判断跨入金额是否满足该链单笔跨入下限。未配置或未安装 registry 时视为无下限要求，返回 true。
    /// BTC 链的 min 为 satoshi，amount 为 token 数量（satoshi），直接比较；其他链的 min 为 8 位小数 USD，先将 amount 转为 USD 再比较。
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

    /// 判断跨出金额是否满足该链单笔跨出下限。未配置或未安装 registry 时视为无下限要求，返回 true。
    /// BTC 链的 min 为 satoshi，amount 为 token 数量（satoshi），直接比较；其他链的 min 为 8 位小数 USD，先将 amount 转为 USD 再比较。
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

    /// 计算跨出手续费并取与配置的跨出手续费下限的较大值，返回实际应收取的手续费（token 数量）。
    /// 配置中的手续费下限为美元价值（8 位小数），fee 计算为 token 数量；内部先统一为 USD 比较后再换算回 token。
    public fun get_effective_cross_out_fee(
        parent_id: &UID,
        treasury: &treasury::BridgeTreasury,
        chain_id: u64,
        token_id: u64,
        amount: u64,
    ): u64 {
        let fee_token = bridge_fee::calculate_cross_out_fee_amount(parent_id, chain_id, token_id, amount);
        let fee_usd = treasury::get_amount_in_usd_by_token_id(treasury, token_id, fee_token);
        let min_usd = get_min_fee_cross_out(parent_id, chain_id, token_id);
        let effective_usd = if (fee_usd >= min_usd) { fee_usd } else { min_usd };
        treasury::get_token_amount_by_usd(treasury, token_id, effective_usd)
    }

    /// 计算跨入手续费并取与配置的跨入手续费下限的较大值，返回实际应收取的手续费（token 数量）。
    /// 配置中的手续费下限为美元价值（8 位小数），fee 计算为 token 数量；内部先统一为 USD 比较后再换算回 token。
    public fun get_effective_cross_in_fee(
        parent_id: &UID,
        treasury: &treasury::BridgeTreasury,
        chain_id: u64,
        token_id: u64,
        amount: u64,
    ): u64 {
        let fee_token = bridge_fee::calculate_cross_in_fee_amount(parent_id, chain_id, token_id, amount);
        let fee_usd = treasury::get_amount_in_usd_by_token_id(treasury, token_id, fee_token);
        let min_usd = get_min_fee_cross_in(parent_id, chain_id, token_id);
        let effective_usd = if (fee_usd >= min_usd) { fee_usd } else { min_usd };
        treasury::get_token_amount_by_usd(treasury, token_id, effective_usd)
    }

    /// 根据 token_id 和 amount（该代币最小单位）计算 USD 价值，返回值使用 8 位小数（与 limiter 的 USD 精度一致）。
    /// 需传入持有该 token 价格信息的 treasury。
    public fun get_usd_value(treasury: &treasury::BridgeTreasury, token_id: u64, amount: u64): u64 {
        treasury::get_amount_in_usd_by_token_id(treasury, token_id, amount)
    }
}

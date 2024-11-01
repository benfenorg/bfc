#[allow(unused_const,unused_mut_parameter)]
module bfc_system::bfc_system_state_inner {
    use std::ascii;
    use std::ascii::String;
    use bfc_system::usdt;
    use bfc_system::usdc;
    use sui::bag;
    use sui::bag::Bag;
    use bfc_system::usdt::USDT;
    use bfc_system::usdc::USDC;
    use sui::balance;
    use sui::balance::{Balance, Supply};
    use sui::bfc::BFC;
    use sui::clock::Clock;
    use sui::coin;
    use sui::coin::Coin;
    use sui::vec_map::{Self, VecMap, size};
    use sui::vec_set;
    use sui::vec_set::VecSet;

    use bfc_system::bars::BARS;
    use bfc_system::baud::BAUD;
    use bfc_system::bbrl::BBRL;
    use bfc_system::bcad::BCAD;
    use bfc_system::beur::BEUR;
    use bfc_system::bfc_dao::{Self, Dao, Proposal, Vote};
    use bfc_system::bfc_dao_manager::{BFCDaoManageKey, ManagerKeyBfc};
    use bfc_system::bgbp::BGBP;
    use bfc_system::bidr::BIDR;
    use bfc_system::binr::BINR;
    use bfc_system::bjpy::BJPY;
    use bfc_system::bkrw::BKRW;
    use bfc_system::bmxn::BMXN;
    use bfc_system::brub::BRUB;
    use bfc_system::bsar::BSAR;
    use bfc_system::btry::BTRY;
    use bfc_system::busd::BUSD;
    use bfc_system::bzar::BZAR;
    use bfc_system::mgg::MGG;
    use bfc_system::treasury::{Self, Treasury, TreasuryPauseCap};
    use bfc_system::treasury_pool;
    use bfc_system::treasury_pool::TreasuryPool;
    use bfc_system::vault;
    use bfc_system::vault::VaultInfo;
    use bfc_system::voting_pool::VotingBfc;
    use bfc_system::position::Position;
    use bfc_system::tick::Tick;
    use std::type_name;

    ///Default stable base points
    const DEFAULT_STABLE_BASE_POINTS: u64 = 10;
    ///Default reward rate 50% ,base point is 100
    const DEFAULT_REWARD_RATE: u64 = 50;
    const DEFAULT_STABLE_RATE: u64 = 1_000_000_000;
    const BFC_SYSTEM_STATE_START_ROUND: u64 = 0;
    const DEFAULT_ADMIN_ADDRESSES: vector<address> = vector[@0x0];
    const DEFAULT_TREASURY_ADMIN: address = @0x0;
    const INNER_STABLECOIN_TO_BFC_LIMIT: u64 = 1000000000_000_000_000;

    /// Errors
    const ERR_INNER_STABLECOIN_TO_BFC_LIMIT: u64 = 1000;
    const ERR_NOT_SYSTEM_ADDRESS: u64 = 1001;
    const ERR_DAILY_LIMIT: u64 = 1002;
    const ERR_SWAP_STABLE_NOT_ENOUGH: u64 = 1003;
    const ERR_MINT_UNAUTHORIZED: u64 = 1004;
    const ERR_MINT_BUSD: u64 = 1005;

    //spec module { pragma verify = false; }

    public struct BfcSystemStateInner has store {
        round: u64,
        stable_base_points: u64,
        reward_rate: u64,
        dao: Dao,
        treasury: Treasury,
        treasury_pool: TreasuryPool,
        stable_rate: VecMap<ascii::String, u64>,
    }

    public struct BfcSystemStateInnerV2 has store {
        round: u64,
        stable_base_points: u64,
        reward_rate: u64,
        dao: Dao,
        treasury: Treasury,
        treasury_pool: TreasuryPool,
        stable_rate: VecMap<ascii::String, u64>,

        stake_coins: Bag,
        daily_out_limit: u64,
        daily_use_out_limit: u64,
        // other dapps can use this cap to mint stable coin
        operation_capability: VecMap<String, VecSet<address>>,
        oracle_address: Option<address>,
    }

    public struct TreasuryParameters has drop, copy {
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        max_counter_times: u32,
        base_point: u64,
        initialize_price: u128,
    }

    public struct BfcSystemParameters has drop, copy {
        chain_start_timestamp_ms: u64,
        time_interval: u32,
        treasury_parameters: VecMap<ascii::String, TreasuryParameters>,
    }

    const BFC_SYSTEM_TREASURY_KEY: u64 = 1;

    public(package) fun create_inner_state(
        bfc_balance: Balance<BFC>,
        usd_supply: Supply<BUSD>,
        jpy_supply: Supply<BJPY>,
        krw_supply: Supply<BKRW>,
        aud_supply: Supply<BAUD>,
        ars_supply: Supply<BARS>,
        brl_supply: Supply<BBRL>,
        cad_supply: Supply<BCAD>,
        eur_supply: Supply<BEUR>,
        gbp_supply: Supply<BGBP>,
        idr_supply: Supply<BIDR>,
        inr_supply: Supply<BINR>,
        rub_supply: Supply<BRUB>,
        sar_supply: Supply<BSAR>,
        try_supply: Supply<BTRY>,
        zar_supply: Supply<BZAR>,
        mxn_supply: Supply<BMXN>,
        mgg_supply: Supply<MGG>,
        parameters: BfcSystemParameters,
        ctx: &mut TxContext,
    ): BfcSystemStateInner {

        let dao = bfc_dao::create_dao(DEFAULT_ADMIN_ADDRESSES, ctx);
        treasury::create_treasury_pause_cap(DEFAULT_TREASURY_ADMIN, ctx);
        let (t, remain_balance, rate_map) = create_treasury(
            bfc_balance,
            usd_supply,
            jpy_supply,
            krw_supply,
            aud_supply,
            ars_supply,
            brl_supply,
            cad_supply,
            eur_supply,
            gbp_supply,
            idr_supply,
            inr_supply,
            rub_supply,
            sar_supply,
            try_supply,
            zar_supply,
            mxn_supply,
            mgg_supply,
            parameters,
            ctx);
        let tp = treasury_pool::create_treasury_pool(remain_balance, ctx);

        BfcSystemStateInner {
            round: BFC_SYSTEM_STATE_START_ROUND,
            stable_base_points: DEFAULT_STABLE_BASE_POINTS,
            reward_rate: DEFAULT_REWARD_RATE,
            dao,
            treasury: t,
            treasury_pool: tp,
            stable_rate: rate_map,
        }
    }

    public(package) fun create_stake_manager_key(payment: Coin<BFC>,
                                                ctx: &mut TxContext) {
        bfc_dao::create_stake_manager_key(payment, ctx);
    }

    public(package) fun unstake_manager_key(key: BFCDaoManageKey,
                                           token: ManagerKeyBfc,
                                           ctx: &mut TxContext) {
        bfc_dao::unstake_manager_key(key, token, ctx);
    }

    public(package) fun update_round(
        inner: &mut BfcSystemStateInnerV2,
        round: u64,
    ) {
        _ = round;
        inner.stable_rate = treasury::get_exchange_rates(&inner.treasury);
    }

    fun init_vault_with_positions<StableCoinType>(
        _treasury: &mut Treasury,
        _key: ascii::String,
        _supply: Supply<StableCoinType>,
        _parameters: BfcSystemParameters,
        ctx: &mut TxContext
    ) {
        let p = vec_map::get(&_parameters.treasury_parameters, &_key);
        treasury::init_vault_with_positions<StableCoinType>(
            _treasury,
            _supply,
            p.initialize_price,
            p.base_point,
            p.position_number,
            p.tick_spacing,
            p.spacing_times,
            p.max_counter_times,
            _parameters.chain_start_timestamp_ms,
            ctx,
        );
    }

    /// X treasury  init treasury
    public(package) fun create_treasury(
        mut bfc_balance: Balance<BFC>,
        usd_supply: Supply<BUSD>,
        jpy_supply: Supply<BJPY>,
        krw_supply: Supply<BKRW>,
        aud_supply: Supply<BAUD>,
        ars_supply: Supply<BARS>,
        brl_supply: Supply<BBRL>,
        cad_supply: Supply<BCAD>,
        eur_supply: Supply<BEUR>,
        gbp_supply: Supply<BGBP>,
        idr_supply: Supply<BIDR>,
        inr_supply: Supply<BINR>,
        rub_supply: Supply<BRUB>,
        sar_supply: Supply<BSAR>,
        try_supply: Supply<BTRY>,
        zar_supply: Supply<BZAR>,
        mxn_supply: Supply<BMXN>,
        mgg_supply: Supply<MGG>,
        parameters: BfcSystemParameters,
        ctx: &mut TxContext
    ): (Treasury, Balance<BFC>, VecMap<ascii::String, u64>) {
        let mut t = treasury::create_treasury(parameters.time_interval, balance::value(&bfc_balance), ctx);

        init_vault_with_positions<BUSD>(&mut t, ascii::string(b"BUSD"), usd_supply, parameters, ctx);
        init_vault_with_positions<BJPY>(&mut t, ascii::string(b"BJPY"), jpy_supply, parameters, ctx);
        init_vault_with_positions<BKRW>(&mut t, ascii::string(b"BKRW"), krw_supply, parameters, ctx);
        init_vault_with_positions<BAUD>(&mut t, ascii::string(b"BAUD"), aud_supply, parameters, ctx);
        init_vault_with_positions<BARS>(&mut t, ascii::string(b"BARS"), ars_supply, parameters, ctx);
        init_vault_with_positions<BBRL>(&mut t, ascii::string(b"BBRL"), brl_supply, parameters, ctx);
        init_vault_with_positions<BCAD>(&mut t, ascii::string(b"BCAD"), cad_supply, parameters, ctx);
        init_vault_with_positions<BEUR>(&mut t, ascii::string(b"BEUR"), eur_supply, parameters, ctx);
        init_vault_with_positions<BGBP>(&mut t, ascii::string(b"BGBP"), gbp_supply, parameters, ctx);
        init_vault_with_positions<BIDR>(&mut t, ascii::string(b"BIDR"), idr_supply, parameters, ctx);
        init_vault_with_positions<BINR>(&mut t, ascii::string(b"BINR"), inr_supply, parameters, ctx);
        init_vault_with_positions<BRUB>(&mut t, ascii::string(b"BRUB"), rub_supply, parameters, ctx);
        init_vault_with_positions<BSAR>(&mut t, ascii::string(b"BSAR"), sar_supply, parameters, ctx);
        init_vault_with_positions<BTRY>(&mut t, ascii::string(b"BTRY"), try_supply, parameters, ctx);
        init_vault_with_positions<BZAR>(&mut t, ascii::string(b"BZAR"), zar_supply, parameters, ctx);
        init_vault_with_positions<BMXN>(&mut t, ascii::string(b"BMXN"), mxn_supply, parameters, ctx);
        init_vault_with_positions<MGG>(&mut t, ascii::string(b"MGG"), mgg_supply, parameters, ctx);

        let mut rate_map = vec_map::empty<ascii::String, u64>();
        if (balance::value<BFC>(&bfc_balance) > 0) {
            let deposit_balance = balance::split(&mut bfc_balance, treasury::bfc_required(&t));
            treasury::deposit(&mut t, coin::from_balance(deposit_balance, ctx));
            treasury::rebalance_internal(&mut t, false, ctx);
            rate_map = treasury::get_exchange_rates(&t);
        };
        (t, bfc_balance, rate_map)
    }

    public(package) fun get_rate_map(self: &BfcSystemStateInnerV2): VecMap<ascii::String, u64> {
        self.stable_rate
    }

    /// swap bfc to stablecoin
    public(package) fun swap_bfc_to_stablecoin<StableCoinType>(
        self: &mut BfcSystemStateInnerV2,
        coin_bfc: Coin<BFC>,
        clock: &Clock,
        amount: u64,
        min_amount: u64,
        deadline: u64,
        ctx: &mut TxContext,
    ) {
        treasury::mint<StableCoinType>(&mut self.treasury, coin_bfc, clock, amount, min_amount, deadline, ctx);
    }

    public(package) fun swap_bfc_to_stablecoin_balance<StableCoinType>(
        self: &mut BfcSystemStateInner,
        coin_bfc: Coin<BFC>,
        amount: u64,
        ctx: &mut TxContext,
    ): Balance<StableCoinType> {
        treasury::mint_internal<StableCoinType>(&mut self.treasury, coin_bfc, amount, ctx)
    }

    /// swap stablecoin to bfc
    public(package) fun swap_stablecoin_to_bfc<StableCoinType>(
        self: &mut BfcSystemStateInnerV2,
        coin_sc: Coin<StableCoinType>,
        clock: &Clock,
        amount: u64,
        min_amount: u64,
        deadline: u64,
        ctx: &mut TxContext,
    ) {
        treasury::redeem<StableCoinType>(&mut self.treasury, coin_sc, clock, amount, min_amount, deadline, ctx);
    }

    public(package) fun swap_stablecoin_to_bfc_balance<StableCoinType>(
        self: &mut BfcSystemStateInnerV2,
        coin_sc: Coin<StableCoinType>,
        expected_amount: u64,
        ctx: &mut TxContext,
    ): Balance<BFC> {
        let amount = coin::value(&coin_sc);
        assert!(amount <= INNER_STABLECOIN_TO_BFC_LIMIT, ERR_INNER_STABLECOIN_TO_BFC_LIMIT);
        assert!(tx_context::sender(ctx) == @0x0, ERR_NOT_SYSTEM_ADDRESS);
        let mut result_balance= treasury::redeem_internal<StableCoinType>(&mut self.treasury, coin_sc, amount, ctx);
        if (expected_amount == 0||balance::value(&result_balance) == expected_amount) {
            result_balance
        }
        else if (balance::value(&result_balance) > expected_amount) {
            let result = balance::split(&mut result_balance, expected_amount);
            treasury_pool::deposit_to_treasury_pool(&mut self.treasury_pool, coin::from_balance(result_balance, ctx));
            result
        } else {
            let amount = expected_amount - balance::value(&result_balance) ;
            let mut result = request_gas_balance(self, amount, ctx);
            balance::join(&mut result,result_balance);
            result
        }
    }

    public(package) fun get_stablecoin_by_bfc<StableCoinType>(
        self: &BfcSystemStateInnerV2,
        amount: u64
    ): vault::CalculatedSwapResult
    {
        treasury::calculate_swap_result<StableCoinType>(&self.treasury, false, amount)
    }

    public(package) fun get_bfc_by_stablecoin<StableCoinType>(
        self: &BfcSystemStateInnerV2,
        amount: u64
    ): vault::CalculatedSwapResult
    {
        treasury::calculate_swap_result<StableCoinType>(&self.treasury, true, amount)
    }

    public(package) fun get_bfc_exchange_rate<CoinType>(self: &BfcSystemStateInnerV2): u64 {
        vault::calculated_swap_result_amount_out(&get_stablecoin_by_bfc<CoinType>(
            self,
            DEFAULT_STABLE_RATE,
        ))
    }

    public(package) fun get_stablecoin_exchange_rate<CoinType>(self: &BfcSystemStateInnerV2): u64 {
        vault::calculated_swap_result_amount_out(&get_bfc_by_stablecoin<CoinType>(
            self,
            DEFAULT_STABLE_RATE,
        ))
    }

    /// deprecated
    public fun next_epoch_bfc_required(self: &BfcSystemStateInner): u64 {
        treasury::bfc_required(&self.treasury)
    }

    public(package) fun next_epoch_bfc_required_v2(self: &BfcSystemStateInnerV2): u64 {
        treasury::bfc_required(&self.treasury)
    }

    #[allow(unused_variable)]
    public(package) fun bfc_required(self: &BfcSystemStateInnerV2): u64 {
        1
        //todo:treasury::bfc_required(&self.treasury)
    }

    public(package) fun bfc_required_with_one_stablecoin<StableCoinType>(self: &BfcSystemStateInnerV2): u64 {
        treasury::bfc_required_with_one_stablecoin<StableCoinType>(&self.treasury)
    }

    /// deprecated
    public fun treasury_balance(self: &BfcSystemStateInner): u64 {
        treasury::get_balance(&self.treasury)
    }

    public(package) fun treasury_balance_v2(self: &BfcSystemStateInnerV2): u64 {
        treasury::get_balance(&self.treasury)
    }

    public(package) fun deposit_to_treasury(self: &mut BfcSystemStateInnerV2, coin_bfc: Coin<BFC>) {
        treasury::deposit(&mut self.treasury, coin_bfc);
    }

    public(package) fun deposit_to_treasury_pool(self: &mut BfcSystemStateInnerV2, coin_bfc: Coin<BFC>) {
        treasury_pool::deposit_to_treasury_pool(&mut self.treasury_pool, coin_bfc);
    }

    public(package) fun rebalance(
        self: &mut BfcSystemStateInnerV2,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let amount = treasury::bfc_required(&self.treasury);
        if (amount > 0) {
            let withdraw_balance = treasury_pool::withdraw_to_treasury(&mut self.treasury_pool, amount, ctx);
            if (balance::value(&withdraw_balance) > 0) {
                treasury::deposit(&mut self.treasury, coin::from_balance(withdraw_balance, ctx));
            } else {
                balance::destroy_zero(withdraw_balance);
            };
        };
        let pool_balance = treasury_pool::get_balance(&self.treasury_pool);
        treasury::rebalance(&mut self.treasury, pool_balance, true, clock, ctx);
    }

    public(package) fun rebalance_with_one_stablecoin<StableCoinType>(
        self: &mut BfcSystemStateInnerV2,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let amount = treasury::bfc_required_with_one_stablecoin<StableCoinType>(&self.treasury);
        if (amount > 0) {
            let withdraw_balance = treasury_pool::withdraw_to_treasury(&mut self.treasury_pool, amount, ctx);
            if (balance::value(&withdraw_balance) > 0) {
                treasury::deposit_with_one_stablecoin<StableCoinType>(&mut self.treasury, coin::from_balance(withdraw_balance, ctx));
            } else {
                balance::destroy_zero(withdraw_balance);
            };
        };
        let pool_balance = treasury_pool::get_balance(&self.treasury_pool);
        treasury::rebalance_with_one_stablecoin<StableCoinType>(&mut self.treasury, pool_balance, true, clock, ctx);
    }

    public(package) fun request_gas_balance(
        self: &mut BfcSystemStateInnerV2,
        amount: u64,
        ctx: &mut TxContext,
    ): Balance<BFC> {
        treasury_pool::withdraw_to_treasury(&mut self.treasury_pool, amount, ctx)
    }

    public(package) fun mint_stable<StableCoinType>(
        inner_state: &mut BfcSystemStateInnerV2,
        amount: u64,
        key: &String,
        ctx: &mut TxContext,
    ): Coin<StableCoinType> {
        assert!(verify_operation_capability(inner_state, key, ctx.sender()), ERR_MINT_UNAUTHORIZED);
        assert!(type_name::get<StableCoinType>() == type_name::get<BUSD>(), ERR_MINT_BUSD);
        let usdc_usdt_coint = type_name::get<StableCoinType>() == type_name::get<USDT>() || type_name::get<StableCoinType>() == type_name::get<USDC>();
        if (usdc_usdt_coint) {
            return treasury::mint_stable<StableCoinType>(&mut inner_state.treasury, amount, ctx)
        };
        let vault_mut = treasury::borrow_mut_vault<StableCoinType>(&mut inner_state.treasury, treasury::get_vault_key<StableCoinType>());
        let (balance_stable,_balance_bfc) = vault::balances<StableCoinType>(vault_mut);
        if(balance_stable>=amount){
            return vault::decrease_coin_a(vault_mut, amount, ctx)
        };
        treasury::mint_stable<StableCoinType>(&mut inner_state.treasury, amount, ctx)
    }

    public(package) fun exchange_stable_to_busd<StableCoinType>(
        inner_state: &mut BfcSystemStateInnerV2,
        stable_coin: Coin<StableCoinType>,
        recipient: address,
        ctx: &mut TxContext,
    ) {
        let amount: u64 = stable_coin.value();

        // stake coin into stable_coins
        let key = treasury::get_vault_key<StableCoinType>();
        let mut exchange_key_bytes = std::ascii::into_bytes(key);
        exchange_key_bytes.append( b"-exchange");
        let exchange_key = std::ascii::string(exchange_key_bytes);
        let coin = bag::borrow_mut<String, Coin<StableCoinType>>(&mut inner_state.stake_coins, exchange_key);
        coin::join(coin, stable_coin);

        // increase busd
        let supply = treasury::get_busd_supply_mut(&mut inner_state.treasury);
        let busd_balance = balance::increase_supply(supply, amount);
        let busd = sui::coin::from_balance(busd_balance, ctx);

        // transfer busd to receiver
        transfer::public_transfer(busd, recipient);
    }

    #[allow(lint(self_transfer))]
    public(package) fun exchange_busd_to_stable<StableCoinType>(
        system_state: &mut BfcSystemStateInnerV2,
        busd_coin: Coin<BUSD>,
        ctx: &mut TxContext,
    ) {
        let amount: u64 = busd_coin.value();
        assert!(amount + system_state.daily_use_out_limit <= system_state.daily_out_limit, ERR_DAILY_LIMIT);

        let key = treasury::get_vault_key<StableCoinType>();
        let mut exchange_key_bytes = std::ascii::into_bytes(key);
        exchange_key_bytes.append( b"-exchange");
        let exchange_key = std::ascii::string(exchange_key_bytes);

        let amount: u64 = busd_coin.value();
        let stable_sum = bag::borrow_mut<String, Coin<StableCoinType>>(&mut system_state.stake_coins, exchange_key);
        assert!(stable_sum.value() >= amount, ERR_SWAP_STABLE_NOT_ENOUGH);
        let stable_back = coin::split(stable_sum, amount, ctx);
        let busd_sum = treasury::get_busd_supply_mut(&mut system_state.treasury);
        balance::decrease_supply(busd_sum, coin::into_balance(busd_coin));
        treasury::exchange_busd_to_stable<StableCoinType>(&mut system_state.treasury, stable_back, ctx);

        system_state.daily_use_out_limit = system_state.daily_use_out_limit + amount;
    }

    public(package) fun get_all_stable_rate(self: & BfcSystemStateInnerV2): VecMap<String, u64> {
        self.stable_rate
    }

    /// X-vault
    /// deprecated
    public fun vault_info<StableCoinType>(self: &BfcSystemStateInner): VaultInfo {
        treasury::vault_info<StableCoinType>(&self.treasury)
    }

    /// deprecated
    public fun vault_ticks<StableCoinType>(self: &BfcSystemStateInner): vector<Tick> {
        treasury::fetch_ticks<StableCoinType>(&self.treasury)
    }

    /// deprecated
    public fun vault_positions<StableCoinType>(self: &BfcSystemStateInner): vector<Position> {
        treasury::fetch_positions<StableCoinType>(&self.treasury)
    }

    /// deprecated
    public fun get_total_supply<StableCoinType>(self: &BfcSystemStateInner): u64 {
        treasury::get_total_supply<StableCoinType>(&self.treasury)
    }

    /// deprecated
    public fun vault_set_pause<StableCoinType>(cap: &TreasuryPauseCap, self: &mut BfcSystemStateInner, pause: bool) {
        treasury::vault_set_pause<StableCoinType>(cap, &mut self.treasury, pause)
    }

    public(package) fun vault_info_v2<StableCoinType>(self: &BfcSystemStateInnerV2): VaultInfo {
        treasury::vault_info<StableCoinType>(&self.treasury)
    }

    public(package) fun vault_ticks_v2<StableCoinType>(self: &BfcSystemStateInnerV2): vector<Tick> {
        treasury::fetch_ticks<StableCoinType>(&self.treasury)
    }

    public(package) fun vault_positions_v2<StableCoinType>(self: &BfcSystemStateInnerV2): vector<Position> {
        treasury::fetch_positions<StableCoinType>(&self.treasury)
    }

    public(package) fun get_total_supply_v2<StableCoinType>(self: &BfcSystemStateInnerV2): u64 {
        treasury::get_total_supply<StableCoinType>(&self.treasury)
    }

    public(package) fun vault_set_pause_v2<StableCoinType>(cap: &TreasuryPauseCap, self: &mut BfcSystemStateInnerV2, pause: bool) {
        treasury::vault_set_pause<StableCoinType>(cap, &mut self.treasury, pause)
    }

    public(package) fun bfc_system_parameters(
        time_interval: u32,
        chain_start_timestamp_ms: u64,
        treasury_parameters: VecMap<ascii::String, TreasuryParameters>,
    ): BfcSystemParameters {
        BfcSystemParameters {
            time_interval,
            chain_start_timestamp_ms,
            treasury_parameters,
        }
    }

    public(package) fun bfc_system_treasury_parameters(
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        initialize_price: u128,
        base_point: u64,
        max_counter_times: u32,
    ): TreasuryParameters {
        TreasuryParameters {
            position_number,
            tick_spacing,
            spacing_times,
            initialize_price,
            base_point,
            max_counter_times,
        }
    }

    public(package) fun create_bfcdao_action(
        self: &mut BfcSystemStateInnerV2,
        payment: &mut Coin<BFC>,
        actionName: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext) {
        bfc_dao::create_bfcdao_action(&mut self.dao, payment, actionName, clock, ctx);
    }

    public(package) fun propose(
        self: &mut BfcSystemStateInnerV2,
        version_id: u64,
        payment: &mut Coin<BFC>,
        action_id: u64,
        action_delay: u64,
        description: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::propose(&mut self.dao, version_id, payment, action_id, action_delay, description, clock, ctx);
    }

    public(package) fun remove_proposal(self: &mut BfcSystemStateInnerV2,key: &BFCDaoManageKey,proposal_id: u64){
        bfc_dao::remove_proposal(&mut self.dao,key,proposal_id);
    }

    public(package) fun remove_action(self: &mut BfcSystemStateInnerV2,key: &BFCDaoManageKey,action_id: u64){
        bfc_dao::remove_action(&mut self.dao,key,action_id);
    }

    public(package) fun set_voting_delay(self: &mut BfcSystemStateInnerV2, manager_key: &BFCDaoManageKey, value: u64) {
        bfc_dao::set_voting_delay(&mut self.dao, manager_key, value);
    }

    public(package) fun set_voting_period(
        self: &mut BfcSystemStateInnerV2,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        bfc_dao::set_voting_period(&mut self.dao, manager_key, value);
    }

    public(package) fun set_voting_quorum_rate(
        self: &mut BfcSystemStateInnerV2,
        manager_key: &BFCDaoManageKey,
        value: u8,
    ) {
        bfc_dao::set_voting_quorum_rate(&mut self.dao, manager_key, value);
    }

    public(package) fun set_min_action_delay(
        self: &mut BfcSystemStateInnerV2,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        bfc_dao::set_min_action_delay(&mut self.dao, manager_key, value);
    }


    public(package) fun destroy_terminated_proposal(
        self: &mut BfcSystemStateInnerV2,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock
    ) {
        bfc_dao::destroy_terminated_proposal(&mut self.dao, manager_key, proposal, clock);
    }

    public(package) fun judge_proposal_state(wrapper: &mut BfcSystemStateInnerV2, current_time: u64) {
        let proposal_record = bfc_dao::getProposalRecord(&mut wrapper.dao);
        let size: u64 = vec_map::size(&proposal_record);
        let mut i = 0;
        while (i < size) {
            let (_, proposalInfo) = vec_map::get_entry_by_idx(&proposal_record, i);
            let cur_status = bfc_dao::judge_proposal_state(proposalInfo, current_time);
            bfc_dao::set_current_status_into_dao(&mut wrapper.dao, proposalInfo, cur_status);
            i = i + 1;
        };
    }

    public(package) fun modify_proposal(
        system_state: &mut BfcSystemStateInner,
        proposal_obj: &mut Proposal,
        index: u8,
        clock: &Clock
    ) {
        bfc_dao::modify_proposal_obj(&mut system_state.dao, proposal_obj, index, clock);
    }

    public(package) fun cast_vote(
        system_state: &mut BfcSystemStateInnerV2,
        proposal: &mut Proposal,
        coin: VotingBfc,
        agreeInt: u8,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::cast_vote(&mut system_state.dao, proposal, coin, agreeInt, clock, ctx);
    }

    public(package) fun change_vote(
        system_state: &mut BfcSystemStateInnerV2,
        my_vote: &mut Vote,
        proposal: &mut Proposal,
        agree: bool,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::change_vote(&mut system_state.dao, my_vote, proposal, agree, clock, ctx);
    }

    public(package) fun queue_proposal_action(
        system_state: &mut BfcSystemStateInnerV2,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        bfc_dao::queue_proposal_action(&mut system_state.dao, manager_key, proposal, clock);
    }

    public(package) fun revoke_vote(
        system_state: &mut BfcSystemStateInnerV2,
        proposal: &mut Proposal,
        my_vote: Vote,
        voting_power: u64,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::revoke_vote(&mut system_state.dao, proposal, my_vote, voting_power, clock, ctx);
    }

    public(package) fun withdraw_voting(system_state: &mut BfcSystemStateInnerV2,
                               voting_bfc: VotingBfc,
                               clock: & Clock,
                               ctx: &mut TxContext) {
        bfc_dao::withdraw_voting(&mut system_state.dao, voting_bfc, clock, ctx);
    }

    public(package) fun create_voting_bfc(system_state: &mut BfcSystemStateInnerV2,
                                         coin: Coin<BFC>,
                                         clock: & Clock,
                                         ctx: &mut TxContext) {
        bfc_dao::create_voting_bfc(&mut system_state.dao, coin, clock, ctx);
    }

    public(package) fun v1_to_v2(self: BfcSystemStateInner, _ctx: &mut TxContext): (BfcSystemStateInnerV2, &mut TxContext) {
        let BfcSystemStateInner {
            round,
            stable_base_points,
            reward_rate,
            dao,
            treasury,
            treasury_pool,
            stable_rate,
        } = self;
        let coin_bag = bag::new(_ctx);
        (BfcSystemStateInnerV2 {
            round,
            stable_base_points,
            reward_rate,
            dao,
            treasury,
            treasury_pool,
            stable_rate,
            stake_coins: coin_bag,
            daily_out_limit : 40000_000_000_000u64,
            daily_use_out_limit: 0u64,
            operation_capability: vec_map::empty(),
            oracle_address: option::none(),
        }, _ctx)
    }

    public(package) fun get_daily_out_limit(self: &BfcSystemStateInnerV2): u64 {
        self.daily_out_limit
    }

    public(package) fun reset_daily_use_out_limit(self: &mut BfcSystemStateInnerV2) {
        self.daily_use_out_limit = 0u64;
    }

    public(package) fun set_daily_out_limit(self: &mut BfcSystemStateInnerV2, new_limit: u64) {
        self.daily_out_limit = new_limit;
    }

    public(package) fun init_bfc_system_state_v2(self: &mut BfcSystemStateInnerV2, _ctx: &mut TxContext) {
        let treasury = &mut self.treasury;
        let usdc_supply = usdc::new(_ctx);
        let usdt_supply = usdt::new(_ctx);

        treasury::add_supply<USDC>(treasury, usdc_supply);
        treasury::add_supply<USDT>(treasury, usdt_supply);

        let coin_bag = &mut self.stake_coins;
        let usdc_coin = coin::zero<USDC>(_ctx);
        let usdt_coin = coin::zero<USDT>(_ctx);
        add_coin_2_stake_pool<USDC>(coin_bag, usdc_coin);
        add_coin_2_stake_pool<USDT>(coin_bag, usdt_coin);

        let stable_rate = get_rate_map(self);
        let length = stable_rate.size();

        let mut i = 0;
        while (i < length) {
            let (key, _value) = stable_rate.get_entry_by_idx(i);
            if (*key == treasury::get_vault_key<BUSD>()) {
                i = i + 1;
                continue
            };
            i = i + 1;
        }
    }

    fun add_coin_2_stake_pool<StableCoinType>(bag: &mut Bag, coin: Coin<StableCoinType>) {
        let key = treasury::get_vault_key<StableCoinType>();
        let mut exchange_key_bytes = std::ascii::into_bytes(key);
        exchange_key_bytes.append( b"-exchange");
        let exchange_key = std::ascii::string(exchange_key_bytes);

        bag::add(bag, exchange_key, coin);
    }

    public(package)  fun get_operation_capability(self: &BfcSystemStateInnerV2): VecMap<String, VecSet<address>> {
        self.operation_capability
    }

    public(package) fun get_operation_capability_by_key(self: &BfcSystemStateInnerV2, key: &String): VecSet<address> {
        let result: Option<VecSet<address>> = vec_map::try_get(&self.operation_capability, key);

        if (option::is_some(&result)) {
            *option::borrow(&result)
        } else {
            vec_set::empty()
        }
    }

    public(package) fun set_operation_capability(self: &mut BfcSystemStateInnerV2, key: String, value: VecSet<address>) {
        if (vec_map::contains(&self.operation_capability, &key)) {
            let new_capability = vec_map::get_mut(&mut self.operation_capability, &key);
            let source_contents = vec_set::keys(&value);
            let mut i = 0;
            while (i < vec_set::size(&value)) {
                let addr = &source_contents[i];
                if (!vec_set::contains(new_capability, addr)) {
                    vec_set::insert(new_capability, *addr);
                };
                i = i + 1;
            };
        } else {
            vec_map::insert(&mut self.operation_capability, key, value);
        }
    }

    public(package) fun add_operation_capability(self: &mut BfcSystemStateInnerV2, key: String, value: address) {
        if (vec_map::contains(&self.operation_capability, &key)) {
            let new_capability = vec_map::get_mut(&mut self.operation_capability, &key);
            vec_set::insert(new_capability, value);
        } else {
            let mut new_set = vec_set::empty();
            vec_set::insert(&mut new_set, value);
            vec_map::insert(&mut self.operation_capability, key, new_set);
        }
    }

    public(package) fun remove_operation_capability(self: &mut BfcSystemStateInnerV2, key: &String, value: address) {
        if (vec_map::contains(&self.operation_capability, key)) {
            let new_capability = vec_map::get_mut(&mut self.operation_capability, key);
            vec_set::remove(new_capability, &value);
        }
    }

    public(package) fun verify_operation_capability(self: &BfcSystemStateInnerV2, key: &String, value: address): bool {
        if (vec_map::contains(&self.operation_capability, key)) {
            let new_capability = vec_map::get(&self.operation_capability, key);
            vec_set::contains(new_capability, &value)
        } else {
            false
        }
    }

    public(package) fun withdraw_balance(
        self: &mut BfcSystemStateInnerV2,
        amount: u64
    ): Balance<BFC> {
        treasury::withdraw_balance(&mut self.treasury, amount)
    }

    public(package) fun add_balance_to_vault<StableCoinType>(
        self: &mut BfcSystemStateInnerV2,
        balance: Balance<StableCoinType>,
        _ctx: &mut TxContext
    ) {
        treasury::increase_other_stablecoin_balance<StableCoinType>(&mut self.treasury, balance);
    }

}

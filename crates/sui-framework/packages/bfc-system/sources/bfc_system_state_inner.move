#[allow(unused_const, unused_mut_parameter)]
module bfc_system::bfc_system_state_inner {
    use std::ascii;
    use std::ascii::String;
    use sui::bag;
    use sui::bag::Bag;
    use sui::balance;
    use sui::balance::{Balance, Supply};
    use sui::bfc::BFC;
    use sui::clock::Clock;
    use sui::coin;
    use sui::coin::Coin;
    use sui::vec_map::{Self, VecMap};
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
    use bfc_system::math_u64;
    use bfc_system::treasury_pool;
    use bfc_system::treasury_pool::TreasuryPool;
    use bfc_system::vault;
    use bfc_system::vault::VaultInfo;
    use bfc_system::voting_pool::VotingBfc;
    use bfc_system::position::Position;
    use bfc_system::tick::Tick;
    use std::type_name;
    use sui::tx_context::{sender};
    use bfc_system::auth_utils;

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
    const ERR_MINT_UNAUTHORIZED: u64 = 1004;
    const ERR_REBALANCE_NOT_BUSD: u64 = 1006;
    const ERR_MINT_AMOUNT_ZERO: u64 = 1007;
    const ERR_MINT_OPERATION_UNAUTHORIZED: u64 = 1008;
    const ERR_ADD_ADMIN_COUNT_ZERO: u64 = 1009;
    const ERR_ADMIN_COUNT_ZERO: u64 = 1010;
    const ERR_SET_CONFIG_UNAUTHORIZED: u64 = 1011;
    const ERR_ADMIN_ALREADY_INITED: u64 = 1012;
    const ERROR_MINT_COIN_TYPE: u64 =1013;


    const KEY_EXTERNAL_STABLE_GAS_COIN_LIST:  vector<u8>  = b"ExternalStableCoinList";
    const KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST:  vector<u8>  = b"ToDeleteExternalStableCoinList";
    //spec module { pragma verify = false; }

    /// Key for deposit stable gas coin map in extra_fields
    const KEY_DEPOSIT_STABLE_GAS_COIN_MAP: vector<u8> = b"DepositStableGasCoinMap";


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

        // other dapps can use this cap to mint stable coin
        operation_capability: VecMap<String, VecSet<address>>,
        admin_capability_addresses: VecSet<address>,
        admin_init: bool,
        oracle_address: Option<address>,
        extra_fields: Bag,
    }

    public struct BfcSystemAdminCap has key, store {
        id: UID,
    }

    public struct BfcSystemModifyCap has key, store {
        id: UID,
        key: ascii::String,
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
        bfc_skip_init_vault:u32,
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
            bfc_skip_init_vault,
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

    public(package) fun update_round_v2(
        inner: &mut BfcSystemStateInnerV2,
        round: u64,
        stable_type_name_vector: vector<ascii::String>,
        stable_rate_vector: vector<u64>,
    ) {
        // check
        if (vector::length(&stable_type_name_vector) != vector::length(&stable_rate_vector)) {
            return
        };

        // delete external stable coin in to delete list
        clear_to_delete_external_stable_gas_coin_list(inner);

        _ = round;
        let stable_rate_map = treasury::get_exchange_rates(&inner.treasury);
        let busd_vault_key = treasury::get_vault_key<BUSD>();
        let mut busd_rate_some = stable_rate_map.try_get(&busd_vault_key);

        if (busd_rate_some.is_none()) return;

        let busd_rate: u64 = busd_rate_some.extract();
        // update busd rate
        if (inner.stable_rate.contains(&busd_vault_key)) {
            inner.stable_rate.remove(&busd_vault_key);
            inner.stable_rate.insert(busd_vault_key, busd_rate);
        };

        // update other stable rate
        let len = vector::length(&stable_type_name_vector);
        let mut i = 0;
        while (i < len) {
            let stable_type_name = stable_type_name_vector[i];
            let rate_against_busd = stable_rate_vector[i];
            if ((treasury::has_vault(&inner.treasury, stable_type_name) || 
                inner.in_external_stable_gas_coin_list(stable_type_name)) &&
                stable_type_name != busd_vault_key && rate_against_busd > 0) {
                if (inner.stable_rate.contains(&stable_type_name)) {
                    inner.stable_rate.remove(&stable_type_name);
                };

                // oracle price decimal = 1_000_000_000
                let mut _rate_against_bfc = 0;
                let (temp_rate, overflowing) = math_u64::overflowing_mul(busd_rate, rate_against_busd);
                if (overflowing) {
                    _rate_against_bfc = math_u64::wrapping_mul(busd_rate / 1_000_000_000, rate_against_busd);
                } else {
                    _rate_against_bfc = temp_rate / 1_000_000_000;
                };
                
                inner.stable_rate.insert(stable_type_name, _rate_against_bfc);
            }; 
            i = i + 1;
        }
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
        bfc_skip_init_vault:u32,
        ctx: &mut TxContext
    ): (Treasury, Balance<BFC>, VecMap<ascii::String, u64>) {
        let mut t = treasury::create_treasury(parameters.time_interval, balance::value(&bfc_balance), ctx);
        if (bfc_skip_init_vault == 0) {
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
        }else{
            init_vault_with_positions<BUSD>(&mut t, ascii::string(b"BUSD"), usd_supply, parameters, ctx);
        };
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

    #[test_only]
    public(package) fun get_treasury_and_treasury_pool(self: &BfcSystemStateInnerV2): (&Treasury, &TreasuryPool) {
        (&self.treasury, &self.treasury_pool)
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
        self: &mut BfcSystemStateInnerV2,
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
        let mut result_balance = treasury::redeem_internal<StableCoinType>(&mut self.treasury, coin_sc, amount, ctx);
        if (expected_amount == 0 || balance::value(&result_balance) == expected_amount) {
            result_balance
        }
        else if (balance::value(&result_balance) > expected_amount) {
            let result = balance::split(&mut result_balance, expected_amount);
            treasury_pool::deposit_to_treasury_pool(&mut self.treasury_pool, coin::from_balance(result_balance, ctx));
            result
        } else {
            let amount = expected_amount - balance::value(&result_balance) ;
            let mut result = request_gas_balance(self, amount, ctx);
            balance::join(&mut result, result_balance);
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
        treasury::bfc_required_v2(&self.treasury)
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
        treasury::deposit_v2(&mut self.treasury, coin_bfc);
    }

    public(package) fun deposit_to_treasury_pool(self: &mut BfcSystemStateInnerV2, coin_bfc: Coin<BFC>) {
        treasury_pool::deposit_to_treasury_pool(&mut self.treasury_pool, coin_bfc);
    }

    public(package) fun rebalance(
        _self: &mut BfcSystemStateInnerV2,
        _clock: &Clock,
        _ctx: &mut TxContext,
    ) {
    }

    public(package) fun rebalance_with_one_stablecoin<StableCoinType>(
        self: &mut BfcSystemStateInnerV2,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let vault_key = treasury::get_vault_key<StableCoinType>();
        assert!(vault_key == type_name::into_string(type_name::get<BUSD>()), ERR_REBALANCE_NOT_BUSD);

        let amount = treasury::bfc_required_with_one_stablecoin<StableCoinType>(&self.treasury);
        if (amount > 0) {
            let withdraw_balance = treasury_pool::withdraw_to_treasury(&mut self.treasury_pool, amount, ctx);
            if (balance::value(&withdraw_balance) > 0) {
                treasury::deposit_with_one_stablecoin<StableCoinType>(
                    &mut self.treasury,
                    coin::from_balance(withdraw_balance, ctx)
                );
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
        assert!(std::type_name::get<StableCoinType>() == std::type_name::get<BUSD>(), ERROR_MINT_COIN_TYPE);
        assert!(amount > 0, ERR_MINT_AMOUNT_ZERO);
        assert!(verify_operation_capability(inner_state, key, ctx.sender()), ERR_MINT_UNAUTHORIZED);
        let vault_key = treasury::get_vault_key<StableCoinType>();
        let busd_key = treasury::get_vault_key<BUSD>();
        if (vault_key == busd_key) {
            assert!(auth_utils::has_mint_busd(key), ERR_MINT_OPERATION_UNAUTHORIZED);
            return treasury::mint_stable<StableCoinType>(&mut inner_state.treasury, amount, ctx)
        };
        assert!(auth_utils::has_mint_other_stablecoin(key), ERR_MINT_OPERATION_UNAUTHORIZED);
        let vault_mut = treasury::borrow_mut_vault<StableCoinType>(
            &mut inner_state.treasury,
            treasury::get_vault_key<StableCoinType>()
        );
        let (balance_stable, _balance_bfc) = vault::balances<StableCoinType>(vault_mut);
        if (balance_stable >= amount) {
            return vault::decrease_coin_a(vault_mut, amount, ctx)
        };

        treasury::mint_stable<StableCoinType>(&mut inner_state.treasury, amount, ctx)
    }

    public(package) fun burn_stable<StableCoinType>(
        inner_state: &mut BfcSystemStateInnerV2,
        token: Coin<StableCoinType>,
    ){
        let vault_key = treasury::get_vault_key<StableCoinType>();
        treasury::check_vault(&inner_state.treasury, vault_key);
        treasury::burn_stable(&mut inner_state.treasury, token)
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

    public(package) fun vault_set_pause_v2<StableCoinType>(
        cap: &TreasuryPauseCap,
        self: &mut BfcSystemStateInnerV2,
        pause: bool
    ) {
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

    public(package) fun remove_proposal(self: &mut BfcSystemStateInnerV2, key: &BFCDaoManageKey, proposal_id: u64) {
        bfc_dao::remove_proposal(&mut self.dao, key, proposal_id);
    }

    public(package) fun remove_action(self: &mut BfcSystemStateInnerV2, key: &BFCDaoManageKey, action_id: u64) {
        bfc_dao::remove_action(&mut self.dao, key, action_id);
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
        system_state: &mut BfcSystemStateInnerV2,
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

    public(package) fun v1_to_v2(
        self: BfcSystemStateInner,
        _ctx: &mut TxContext
    ): (BfcSystemStateInnerV2, &mut TxContext) {
        let BfcSystemStateInner {
            round,
            stable_base_points,
            reward_rate,
            dao,
            treasury,
            treasury_pool,
            stable_rate,
        } = self;
        (BfcSystemStateInnerV2 {
            round,
            stable_base_points,
            reward_rate,
            dao,
            treasury,
            treasury_pool,
            stable_rate,
            operation_capability: vec_map::empty(),
            admin_capability_addresses: vec_set::empty(),
            admin_init: false,
            oracle_address: option::none(),
            extra_fields: bag::new(_ctx),
        }, _ctx)
    }

    public(package) fun init_bfc_system_state_v2(self: &mut BfcSystemStateInnerV2, _ctx: &mut TxContext) {
        transfer_bfc_from_vault_to_treasury_pool<MGG>(self);
        transfer_bfc_from_vault_to_treasury_pool<BJPY>(self);
        transfer_bfc_from_vault_to_treasury_pool<BKRW>(self);
        transfer_bfc_from_vault_to_treasury_pool<BAUD>(self);
        transfer_bfc_from_vault_to_treasury_pool<BARS>(self);
        transfer_bfc_from_vault_to_treasury_pool<BBRL>(self);
        transfer_bfc_from_vault_to_treasury_pool<BCAD>(self);
        transfer_bfc_from_vault_to_treasury_pool<BEUR>(self);
        transfer_bfc_from_vault_to_treasury_pool<BGBP>(self);
        transfer_bfc_from_vault_to_treasury_pool<BIDR>(self);
        transfer_bfc_from_vault_to_treasury_pool<BINR>(self);
        transfer_bfc_from_vault_to_treasury_pool<BRUB>(self);
        transfer_bfc_from_vault_to_treasury_pool<BSAR>(self);
        transfer_bfc_from_vault_to_treasury_pool<BTRY>(self);
        transfer_bfc_from_vault_to_treasury_pool<BZAR>(self);
        transfer_bfc_from_vault_to_treasury_pool<BMXN>(self);
    }

    fun transfer_bfc_from_vault_to_treasury_pool<StableCoinType>(self: &mut BfcSystemStateInnerV2) {
        let vault_key = treasury::get_vault_key<StableCoinType>();
        if (treasury::has_vault(&self.treasury, vault_key)) {
            let vault = treasury::borrow_mut_vault<StableCoinType>(&mut self.treasury, vault_key);
            let bfc_balance = vault::clear_coin_b(vault);
            treasury_pool::increase_balance(&mut self.treasury_pool, bfc_balance, vault_key);
        }
    }

    public(package) fun get_operation_capability(self: &BfcSystemStateInnerV2): VecMap<String, VecSet<address>> {
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

    #[test_only]
    public(package) fun get_admin_capability(self: &BfcSystemStateInnerV2): VecSet<address> {
        self.admin_capability_addresses
    }

    public(package) fun set_operation_capability(
        self: &mut BfcSystemStateInnerV2,
        key: String,
        value: VecSet<address>,
        ctx: &mut TxContext,
    ) {
        verify_admin_capability(self, sender(ctx));
        let source_contents = vec_set::keys(&value);
        if (vec_map::contains(&self.operation_capability, &key)) {
            let new_capability = vec_map::get_mut(&mut self.operation_capability, &key);
            let mut i = 0;
            while (i < vec_set::size(&value)) {
                let addr = &source_contents[i];
                if (!vec_set::contains(new_capability, addr)) {
                    vec_set::insert(new_capability, *addr);
                    create_bfc_system_modify_cap(ctx, *addr, key);
                };
                i = i + 1;
            };
        } else {
            vec_map::insert(&mut self.operation_capability, key, value);
            let mut i = 0;
            let length = vec_set::size(&value);
            while (i < length) {
                let addr = &source_contents[i];
                create_bfc_system_modify_cap(ctx, *addr, key);
                i = i + 1;
            };
        }
    }

    public(package) fun add_operation_capability(
        self: &mut BfcSystemStateInnerV2,
        key: String,
        value: address,
        ctx: &mut TxContext,
    ) {
        verify_admin_capability(self, sender(ctx));
        if (vec_map::contains(&self.operation_capability, &key)) {
            let new_capability = vec_map::get_mut(&mut self.operation_capability, &key);
            vec_set::insert(new_capability, value);
        } else {
            let mut new_set = vec_set::empty();
            vec_set::insert(&mut new_set, value);
            vec_map::insert(&mut self.operation_capability, key, new_set);
        };
        create_bfc_system_modify_cap(ctx, value, key);
    }

    public(package) fun remove_operation_capability(
        self: &mut BfcSystemStateInnerV2,
        key: &String,
        value: address,
        ctx: &mut TxContext,
    ) {
        verify_admin_capability(self, sender(ctx));
        if (vec_map::contains(&self.operation_capability, key)) {
            let new_capability = vec_map::get_mut(&mut self.operation_capability, key);
            if (vec_set::contains(new_capability, &value)) {
                vec_set::remove(new_capability, &value);
            }
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

    // set oracle address
    public(package) fun set_oracle_address(self: &mut BfcSystemStateInnerV2, address: address, ctx: &mut TxContext) {
        verify_admin_capability(self, sender(ctx));
        self.oracle_address = option::some(address);
    }

    // get oracle address
    public(package) fun get_oracle_address(self: &BfcSystemStateInnerV2, _ctx: &mut TxContext): Option<address> {
        self.oracle_address
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

    public(package) fun get_bfc_system_modify_cap_key(self: &BfcSystemModifyCap) : String {
        self.key
    }

    fun create_bfc_system_admin_cap(ctx: &mut TxContext, recipient: address) {
        let cap = BfcSystemAdminCap {
            id : object::new(ctx),
        };
        transfer::transfer(cap, recipient);
    }

    public(package) fun verify_admin_capability(self: &BfcSystemStateInnerV2, addr: address) {
        assert!(vec_set::contains(&self.admin_capability_addresses, &addr), ERR_SET_CONFIG_UNAUTHORIZED);
    }

    public(package) fun init_bfc_system_admins(self: &mut BfcSystemStateInnerV2, ctx: &mut TxContext, admins: vector<address>) {
        assert!(!self.admin_init, ERR_ADMIN_ALREADY_INITED);
        self.admin_capability_addresses = vec_set::empty();
        add_bfc_system_admin_cap(self, ctx, admins);
        self.admin_init = true;
    }

    public(package) fun add_bfc_system_admin_cap(self: &mut BfcSystemStateInnerV2, ctx: &mut TxContext, admins: vector<address>) {
        let count = vector::length(&admins);
        assert!(count > 0, ERR_ADD_ADMIN_COUNT_ZERO);

        let mut i = 0;
        while (i < count) {
            let admin = vector::borrow(&admins, i);
            self.admin_capability_addresses.insert(*admin);
            create_bfc_system_admin_cap(ctx, *admin);
            i = i + 1;
        };
    }

    public(package) fun remove_bfc_system_admin_cap(self: &mut BfcSystemStateInnerV2, addr: address, ctx: &mut TxContext) {
        verify_admin_capability(self, ctx.sender());
        if (self.admin_capability_addresses.contains(&addr)) {
            self.admin_capability_addresses.remove(&addr);
        };

        assert!(self.admin_capability_addresses.size() > 0, ERR_ADMIN_COUNT_ZERO);
    }

    fun create_bfc_system_modify_cap(ctx: &mut TxContext, recipient: address, key: std::ascii::String) {
        let cap = BfcSystemModifyCap {
            id: object::new(ctx),
            key,
        };
        transfer::transfer(cap, recipient);
    }

    public(package) fun get_extra_fields(self: &BfcSystemStateInnerV2): &Bag {
        &self.extra_fields
    }

    /// Deposit any type of stable gas coin balance into extra_fields, grouped by coin type
    public(package) fun deposit_stable_gas_coin<StableCoinType>(
        self: &mut BfcSystemStateInnerV2,
        balance: Balance<StableCoinType>,
        _ctx: &mut TxContext,
    ) {
        let coin_type_key = type_name::into_string(type_name::get<StableCoinType>());
        // VecMap<String, Balance<StableCoinType>>
        // Here we use any to store different types of balance, make sure type safety when retrieving
        if (self.extra_fields.contains(&KEY_DEPOSIT_STABLE_GAS_COIN_MAP)) {
            let map = self.extra_fields.borrow_mut<vector<u8>, VecMap<String, Balance<StableCoinType>>>(&KEY_DEPOSIT_STABLE_GAS_COIN_MAP);
            if (vec_map::contains(map, &coin_type_key)) {
                let mut old_balance = vec_map::remove(map, &coin_type_key);
                balance::join(&mut old_balance, balance);
                vec_map::insert(map, coin_type_key, old_balance);
            } else {
                vec_map::insert(map, coin_type_key, balance);
            }
        } else {
            let mut map = vec_map::empty<String, Balance<StableCoinType>>();
            vec_map::insert(&mut map, coin_type_key, balance);
            self.extra_fields.add(KEY_DEPOSIT_STABLE_GAS_COIN_MAP, map);
        }
    }

    #[test_only]
    public fun create_bfc_system_modify_cap_for_test(ctx: &mut TxContext, recipient: address, key: String) {
        let cap = BfcSystemModifyCap {
            key,
            id: object::new(ctx),
        };
        transfer::transfer(cap, recipient);
    }
    #[test_only]
    public(package) fun get_external_stable_gas_coin_list(self: &BfcSystemStateInnerV2): &vector<ascii::String> {
        if (self.extra_fields.contains(KEY_EXTERNAL_STABLE_GAS_COIN_LIST)) {
            let list = self.extra_fields.borrow<vector<u8>, vector<ascii::String>>(KEY_EXTERNAL_STABLE_GAS_COIN_LIST);
            return list
        };

        abort 1
    }

    #[test_only]
    public(package) fun get_to_delete_stable_gas_coin_list(self: &BfcSystemStateInnerV2): &vector<ascii::String> {
        if (self.extra_fields.contains(KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST)) {
            let list = self.extra_fields.borrow<vector<u8>, vector<ascii::String>>(KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST);
            return list
        };

        abort 1
    }

    public(package) fun in_external_stable_gas_coin_list(self: &BfcSystemStateInnerV2, value: ascii::String): bool {
        if (self.extra_fields.contains(KEY_EXTERNAL_STABLE_GAS_COIN_LIST)) {
            let list = self.extra_fields.borrow<vector<u8>, vector<ascii::String>>(KEY_EXTERNAL_STABLE_GAS_COIN_LIST);

            return list.any!(|x| x == &value);
        };

        false
    }

    public(package) fun add_external_stable_gas_coin(self: &mut BfcSystemStateInnerV2, value: vector<ascii::String>, ctx: &mut TxContext) {
        verify_admin_capability(self, sender(ctx));

        if (self.extra_fields.contains(KEY_EXTERNAL_STABLE_GAS_COIN_LIST)) {
            let list = self.extra_fields.borrow_mut<vector<u8>, vector<ascii::String>>(KEY_EXTERNAL_STABLE_GAS_COIN_LIST);

            let mut allow_list = vector::empty<ascii::String>();
            let mut i = 0;
            while (i < value.length()) {
                if (!list.any!(|x| x == &value[i])) {
                   allow_list.insert(value[i], 0);
                };
               
                i = i + 1;
            };

            list.append(allow_list);
            return;
        }; 

        let mut list = vector::empty<ascii::String>();
        list.append(value);
        self.extra_fields.add(KEY_EXTERNAL_STABLE_GAS_COIN_LIST, list);
    }

    public(package) fun delete_external_stable_gas_coin(self: &mut BfcSystemStateInnerV2, value: ascii::String, ctx: &mut TxContext) {
        verify_admin_capability(self, sender(ctx));

        // add to delete list, delete it from rate map after next epoch
        if (!self.extra_fields.contains(KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST)) {
            let mut list = vector::empty<ascii::String>();
            list.insert(value, 0);
            self.extra_fields.add(KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST, list);
        } else {
            let list = self.extra_fields.borrow_mut<vector<u8>, vector<ascii::String>>(KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST);
            if (!list.any!(|x| x == &value)) {
                list.insert(value, 0);
            };
        };
    }

    public(package) fun clear_to_delete_external_stable_gas_coin_list (self: &mut BfcSystemStateInnerV2) {
        if (!self.extra_fields.contains(KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST)) {
           return
        };

        let to_delete_list = self.extra_fields.remove<vector<u8>, vector<ascii::String>>( KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST);

        let stable_gas_list: &mut vector<ascii::String>;
        if (self.extra_fields.contains(KEY_EXTERNAL_STABLE_GAS_COIN_LIST)) {
            stable_gas_list = self.extra_fields.borrow_mut<vector<u8>, vector<ascii::String>>(KEY_EXTERNAL_STABLE_GAS_COIN_LIST);
        } else {
            stable_gas_list = &mut vector::empty<ascii::String>();
        };
        let mut i = 0;
        while (i < to_delete_list.length()) {
            delete_from_list(stable_gas_list, to_delete_list[i]);
            // remove from stable rate map
            if (self.stable_rate.contains(&to_delete_list[i])) {
                self.stable_rate.remove(&to_delete_list[i]);
            };

            i = i + 1;
        };   
    }

    public(package) fun delete_from_list(list:  &mut vector<ascii::String>, value: ascii::String) {
        let mut i = 0;
        while (i < list.length()) {
            if (list[i] == value) {
                list.remove(i);
                return
            };

            i = i + 1;
        };
    }
}

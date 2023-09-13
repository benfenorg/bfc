module obc_system::obc_system_state_inner {
    use obc_system::voting_pool::VotingObc;
    use sui::balance::{Balance, Supply};
    use sui::clock::Clock;
    use sui::coin::Coin;
    use sui::obc::OBC;
    use sui::stable::STABLE;
    use sui::tx_context::TxContext;
    use sui::vec_map;

    use obc_system::exchange_inner;
    use obc_system::exchange_inner::ExchangePool;
    use obc_system::gas_coin_map;
    use obc_system::gas_coin_map::{GasCoinMap, GasCoinEntity};
    use obc_system::treasury::{Self, Treasury};
    use obc_system::usd::USD;
    use obc_system::obc_dao_manager::{OBCDaoManageKey};
    use obc_system::obc_dao::{Dao, Proposal, Self};

    friend obc_system::obc_system;

    const OBC_SYSTEM_STATE_START_ROUND: u64 = 0;
    const DEFAULT_ADMIN_ADDRESSES: vector<address> = vector[
        @0x23027681c7d461e3db271aeed97b5da2b6e157350fa2ff659a7ff9cccb28cc00,
        @0x905973e8fae0c89c6c1da33751db3f828bda228e0171231b02052fbbebd48f68,
        @0x363e4d3ee8a6400e21bd0cb0c8ecc876f3a1fe1e0f06ffdd67369bd982d39faf,
        @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590,
        @0x2f76370f2b5f77bcaa47f4e65be0d762738bfbe7c29e374a72bf4d1b5960b47e,
    ];

    spec module { pragma verify = false; }

    struct ObcSystemStateInner has store {
        round: u64,
        /// Contains gas coin information
        gas_coin_map: GasCoinMap,
        /// Exchange gas coin pool
        exchange_pool: ExchangePool<STABLE>,
        dao: Dao,
        treasury: Treasury,
    }

    struct TreasuryParameters has drop, copy {
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        initialize_price: u128,
        time_interval: u32,
        base_point: u64,
        max_counter_times: u32,
    }

    struct ObcSystemParameters has drop, copy {
        treasury_parameters: TreasuryParameters,
        chain_start_timestamp_ms: u64,
    }

    const OBC_SYSTEM_TREASURY_KEY: u64 = 1;

    public(friend) fun create_inner_state(
        usd_supply: Supply<USD>,
        parameters: ObcSystemParameters,
        ctx: &mut TxContext,
    ): ObcSystemStateInner {
        // init gas coin mappings
        let init_gas_coins_map = vec_map::empty<address, GasCoinEntity>();
        let gas_coin_map = gas_coin_map::new(init_gas_coins_map, ctx);
        let exchange_pool = exchange_inner::new_exchange_pool<STABLE>(ctx, 0);
        let dao = obc_dao::create_dao(DEFAULT_ADMIN_ADDRESSES, ctx);
        let t = create_treasury(usd_supply, parameters, ctx);
        ObcSystemStateInner {
            round: OBC_SYSTEM_STATE_START_ROUND,
            gas_coin_map,
            exchange_pool,
            dao,
            treasury: t,
        }
    }

    public(friend) fun update_round(
        inner: &mut ObcSystemStateInner,
        round: u64,
    ) {
        inner.round = round;
    }

    public(friend) fun request_exchange_stable(
        inner: &mut ObcSystemStateInner,
        stable: Coin<STABLE>,
        ctx: &mut TxContext,
    ): Balance<OBC> {
        //get exchange rate
        let rate = gas_coin_map::requst_get_exchange_rate<STABLE>(&inner.gas_coin_map, &stable);
        exchange_inner::request_exchange_stable<STABLE>(rate, &mut inner.exchange_pool, stable, ctx)
    }

    public(friend) fun request_exchange_all(
        inner: &mut ObcSystemStateInner,
        ctx: &mut TxContext
    ) {
        exchange_inner::request_exchange_all<STABLE>(&mut inner.exchange_pool, ctx)
    }

    ///Request withdraw stable coin.
    public(friend) fun request_withdraw_stable(
        inner: &mut ObcSystemStateInner,
    ): Balance<STABLE> {
        exchange_inner::request_withdraw_stable(&mut inner.exchange_pool)
    }

    /// Getter of the gas coin exchange pool rate.
    public(friend) fun requst_get_exchange_rate<CoinType>(
        self: &ObcSystemStateInner,
        stable: &Coin<CoinType>
    ): u64 {
        gas_coin_map::requst_get_exchange_rate<CoinType>(&self.gas_coin_map, stable)
    }

    public(friend) fun request_add_gas_coin<CoinType>(
        self: &mut ObcSystemStateInner,
        gas_coin: &Coin<CoinType>,
        rate: u64,
    ) {
        gas_coin_map::request_add_gas_coin<CoinType>(&mut self.gas_coin_map, gas_coin, rate)
    }

    public(friend) fun request_update_gas_coin<CoinType>(
        self: &mut ObcSystemStateInner,
        gas_coin: &Coin<CoinType>,
        rate: u64,
    ) {
        gas_coin_map::request_update_gas_coin(&mut self.gas_coin_map, gas_coin, rate)
    }

    public(friend) fun request_remove_gas_coin<CoinType>(
        self: &mut ObcSystemStateInner,
        gas_coin: &Coin<CoinType>,
    ) {
        gas_coin_map::request_remove_gas_coin<CoinType>(&mut self.gas_coin_map, gas_coin)
    }

    /// Init exchange pool by add obc coin.
    public fun init_exchange_pool(
        self: &mut ObcSystemStateInner,
        coin: Coin<OBC>,
    ) {
        exchange_inner::add_obc_to_pool(&mut self.exchange_pool, coin)
    }

    /// X treasury  init treasury
    public(friend) fun create_treasury(
        supply: Supply<USD>,
        parameters: ObcSystemParameters,
        ctx: &mut TxContext
    ): Treasury {
        let treasury_parameters = parameters.treasury_parameters;
        let t = treasury::create_treasury(treasury_parameters.time_interval, ctx);
        treasury::init_vault_with_positions<USD>(
            &mut t,
            supply,
            treasury_parameters.initialize_price,
            treasury_parameters.base_point,
            treasury_parameters.position_number,
            treasury_parameters.tick_spacing,
            treasury_parameters.spacing_times,
            treasury_parameters.max_counter_times,
            parameters.chain_start_timestamp_ms,
            ctx,
        );
        t
    }

    /// swap obc to stablecoin
    public(friend) fun swap_obc_to_stablecoin<StableCoinType>(
        self: &mut ObcSystemStateInner,
        coin_obc: Coin<OBC>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        treasury::mint<StableCoinType>(&mut self.treasury, coin_obc, amount, ctx);
    }

    public(friend) fun swap_obc_to_stablecoin_balance<StableCoinType>(
        self: &mut ObcSystemStateInner,
        coin_obc: Coin<OBC>,
        amount: u64,
        ctx: &mut TxContext,
    ): Balance<StableCoinType> {
        treasury::mint_internal<StableCoinType>(&mut self.treasury, coin_obc, amount, ctx)
    }

    /// swap stablecoin to obc
    public(friend) fun swap_stablecoin_to_obc<StableCoinType>(
        self: &mut ObcSystemStateInner,
        coin_sc: Coin<StableCoinType>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        treasury::redeem<StableCoinType>(&mut self.treasury, coin_sc, amount, ctx);
    }

    public(friend) fun swap_stablecoin_to_obc_balance<StableCoinType>(
        self: &mut ObcSystemStateInner,
        coin_sc: Coin<StableCoinType>,
        amount: u64,
        ctx: &mut TxContext,
    ): Balance<OBC> {
        treasury::redeem_internal<StableCoinType>(&mut self.treasury, coin_sc, amount, ctx)
    }

    public(friend) fun get_stablecoin_by_obc<StableCoinType>(
        self: &ObcSystemStateInner,
        amount: u64
    ): u64
    {
        treasury::calculate_swap_result<StableCoinType>(&self.treasury, false, amount)
    }

    public(friend) fun get_obc_by_stablecoin<StableCoinType>(
        self: &ObcSystemStateInner,
        amount: u64
    ): u64
    {
        treasury::calculate_swap_result<StableCoinType>(&self.treasury, true, amount)
    }

    /// X-treasury
    public fun next_epoch_obc_required(self: &ObcSystemStateInner): u64 {
        treasury::next_epoch_obc_required(&self.treasury)
    }

    public fun treasury_balance(self: &ObcSystemStateInner): u64 {
        treasury::get_balance(&self.treasury)
    }

    public(friend) fun deposit_to_treasury(self: &mut ObcSystemStateInner, coin_obc: Coin<OBC>) {
        treasury::deposit(&mut self.treasury, coin_obc);
    }

    public(friend) fun rebalance(
        self: &mut ObcSystemStateInner,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        treasury::rebalance(&mut self.treasury, clock, ctx);
    }

    public(friend) fun obc_system_stat_parameter(
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        initialize_price: u128,
        time_interval: u32,
        base_point: u64,
        max_counter_times: u32,
        chain_start_timestamp_ms: u64,
    ): ObcSystemParameters {
        let treasury_parameters = TreasuryParameters {
            position_number,
            tick_spacing,
            spacing_times,
            initialize_price,
            time_interval,
            max_counter_times,
            base_point,
        };
        ObcSystemParameters {
            treasury_parameters,
            chain_start_timestamp_ms,
        }
    }

    public fun create_obcdao_action(
        self: &mut ObcSystemStateInner, _: &OBCDaoManageKey,
        actionName: vector<u8>,
        ctx: &mut TxContext) {
        obc_dao::create_obcdao_action(&mut self.dao, _, actionName, ctx);
    }

    public fun propose(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        version_id: u64,
        payment: Coin<OBC>,
        action_id: u64,
        action_delay: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        obc_dao:: propose(&mut self.dao, manager_key, version_id, payment, action_id, action_delay, clock, ctx);
    }

    public fun set_voting_delay(self: &mut ObcSystemStateInner, manager_key: &OBCDaoManageKey, value: u64) {
        obc_dao::set_voting_delay(&mut self.dao, manager_key, value);
    }

    public fun set_voting_period(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        value: u64,
    ) {
        obc_dao::set_voting_period(&mut self.dao, manager_key, value);
    }

    public fun set_voting_quorum_rate(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        value: u8,
    ) {
        obc_dao::set_voting_quorum_rate(&mut self.dao, manager_key, value);
    }

    public fun set_min_action_delay(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        value: u64,
    ) {
        obc_dao::set_min_action_delay(&mut self.dao, manager_key, value);
    }


    public fun destroy_terminated_proposal(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock
    ) {
        obc_dao::destroy_terminated_proposal(&mut self.dao, manager_key, proposal, clock);
    }

    public fun judge_proposal_state(wrapper: &mut ObcSystemStateInner, current_time: u64) {
        let proposal_record = obc_dao::getProposalRecord(&mut wrapper.dao);
        let size: u64 = vec_map::size(&proposal_record);
        if (size == 0) {
            return
        };

        let i = 0;
        while (i < size) {
            let (_, proposalInfo) = vec_map::get_entry_by_idx(&proposal_record, size - 1);
            let cur_status = obc_dao::judge_proposal_state(proposalInfo, current_time);
            obc_dao::set_current_status_into_dao(&mut wrapper.dao, proposalInfo, cur_status);
            i = i + 1;
        };
    }

    public fun modify_proposal(system_state: &mut ObcSystemStateInner, index: u8, clock: &Clock) {
        let proposal_record = obc_dao::getProposalRecord(&mut system_state.dao);
        let size: u64 = vec_map::size(&proposal_record);
        if (size == 0) {
            return
        };
        let (_, proposalInfo) = vec_map::get_entry_by_idx_mut(&mut proposal_record, size - 1);
        obc_dao::modify_proposal(proposalInfo, index, clock);

        obc_dao::setProposalRecord(&mut system_state.dao, proposal_record);
    }

    public fun withdraw_voting(system_state: &mut ObcSystemStateInner,
                                       voting_obc: VotingObc,
                                       ctx: &mut TxContext ) {
        obc_dao::withdraw_voting(&mut system_state.dao, voting_obc, ctx);
    }

    public fun create_voting_obc(system_state: &mut ObcSystemStateInner,
                                       coin: Coin<OBC>,
                                       ctx: &mut TxContext) {
        obc_dao::create_voting_obc(&mut system_state.dao, coin, ctx);
    }
}

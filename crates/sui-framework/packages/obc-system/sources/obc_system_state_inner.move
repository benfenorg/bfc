module obc_system::obc_system_state_inner {
    use sui::balance;
    use sui::balance::{Balance, Supply};
    use sui::clock::Clock;
    use sui::coin;
    use sui::coin::Coin;
    use sui::obc::OBC;
    use sui::tx_context::TxContext;
    use sui::vec_map;

    use obc_system::exchange_inner;
    use obc_system::exchange_inner::ExchangePool;
    use obc_system::gas_coin_map;
    use obc_system::gas_coin_map::{GasCoinEntity, GasCoinMap};
    use obc_system::obc_dao::{Self, Dao, Proposal, Vote};
    use obc_system::obc_dao_manager::{OBCDaoManageKey, ManagerKeyObc};
    use obc_system::treasury::{Self, Treasury};
    use obc_system::treasury_pool;
    use obc_system::treasury_pool::TreasuryPool;
    use obc_system::usd::USD;
    use obc_system::vault::VaultInfo;
    use obc_system::voting_pool::VotingObc;

    friend obc_system::obc_system;

    const OBC_SYSTEM_STATE_START_ROUND: u64 = 0;
    const DEFAULT_ADMIN_ADDRESSES: vector<address> = vector[
        @0x23027681c7d461e3db271aeed97b5da2b6e157350fa2ff659a7ff9cccb28cc00,
        @0x905973e8fae0c89c6c1da33751db3f828bda228e0171231b02052fbbebd48f68,
        @0x363e4d3ee8a6400e21bd0cb0c8ecc876f3a1fe1e0f06ffdd67369bd982d39faf,
        @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590,
        @0xdcbb951dc6c91cb4838876825daef3b361ca84d3f1e56e89ede66ef15975b4b8,
        @0x4cfd9d0cb99b422416a680868f2e4e04446a15939042c2fd42104e99fc1da57b,
        @0x3212e3b30a5571b6538560ece888482f2908bd5a95cbf6305ed4052ceb1899dd,
    ];

    spec module { pragma verify = false; }

    struct ObcSystemStateInner has store {
        round: u64,
        /// Contains gas coin information
        gas_coin_map: GasCoinMap,
        /// Exchange gas coin pool
        exchange_pool: ExchangePool<USD>,
        dao: Dao,
        treasury: Treasury,
        treasury_pool: TreasuryPool,
    }

    struct TreasuryParameters has drop, copy {
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        time_interval: u32,
        max_counter_times: u32,
        base_point: u64,
        initialize_price: u128,
    }

    struct ObcSystemParameters has drop, copy {
        chain_start_timestamp_ms: u64,
        treasury_parameters: TreasuryParameters,
    }

    const OBC_SYSTEM_TREASURY_KEY: u64 = 1;

    public(friend) fun create_inner_state(
        usd_supply: Supply<USD>,
        obc_balance: Balance<OBC>,
        parameters: ObcSystemParameters,
        ctx: &mut TxContext,
    ): ObcSystemStateInner {
        // init gas coin mappings
        let init_gas_coins_map = vec_map::empty<address, GasCoinEntity>();
        let gas_coin_map = gas_coin_map::new(init_gas_coins_map, ctx);
        let exchange_pool = exchange_inner::new_exchange_pool<USD>(ctx, 0);
        let dao = obc_dao::create_dao(DEFAULT_ADMIN_ADDRESSES, ctx);
        let (t, remain_balance) = create_treasury(usd_supply, obc_balance, parameters, ctx);
        let tp = treasury_pool::create_treasury_pool(remain_balance, ctx);

        ObcSystemStateInner {
            round: OBC_SYSTEM_STATE_START_ROUND,
            gas_coin_map,
            exchange_pool,
            dao,
            treasury: t,
            treasury_pool: tp,
        }
    }

    public (friend) fun create_stake_manager_key( payment: Coin<OBC>,
                                                  ctx: &mut TxContext) {
        obc_dao::create_stake_manager_key(payment, ctx);
    }

    public(friend) fun unstake_manager_key(key: OBCDaoManageKey,
                                           token: ManagerKeyObc,
                                           ctx: &mut TxContext) {
        obc_dao::unstake_manager_key(key, token, ctx);
    }

    public(friend) fun update_round(
        inner: &mut ObcSystemStateInner,
        round: u64,
    ) {
        inner.round = round;
    }

    public(friend) fun request_exchange_stable(
        inner: &mut ObcSystemStateInner,
        stable: Coin<USD>,
        ctx: &mut TxContext,
    ): Balance<OBC> {
        //get exchange rate
        let rate = gas_coin_map::requst_get_exchange_rate<USD>(&inner.gas_coin_map, &stable);
        exchange_inner::request_exchange_stable<USD>(rate, &mut inner.exchange_pool, stable, ctx)
    }

    public(friend) fun request_exchange_all(
        inner: &mut ObcSystemStateInner,
        ctx: &mut TxContext
    ) {
        //get obc amount of inner exchange pool
        let obc_amount = exchange_inner::get_obc_amount(&inner.exchange_pool);
        if (obc_amount > 0) {
            //set pool is disactivate
            let epoch = exchange_inner::dis_activate(&mut inner.exchange_pool);
            //get stable balance
            let stable_balance = exchange_inner::request_withdraw_all_stable(&mut inner.exchange_pool);
            //exchange from stable swap
            let obc_balance = swap_stablecoin_to_obc_balance(
                inner,
                coin::from_balance(stable_balance, ctx),
                ctx,
            );
            //add obc to inner exchange pool
            exchange_inner::request_deposit_obc_balance(&mut inner.exchange_pool, obc_balance);
            // active pool
            exchange_inner::activate(&mut inner.exchange_pool, epoch);
        }
    }

    ///Request withdraw stable coin.
    public(friend) fun request_withdraw_stable(
        inner: &mut ObcSystemStateInner,
    ): Balance<USD> {
        exchange_inner::request_withdraw_all_stable(&mut inner.exchange_pool)
    }

    /// Getter of the gas coin exchange pool rate.
    public(friend) fun requst_get_exchange_rate<CoinType>(
        self: &ObcSystemStateInner,
        _stable: &Coin<CoinType>
    ): u64 {
        get_stablecoin_by_obc<CoinType>(
            self,
            gas_coin_map::get_default_rate(),
        )
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
    ) {
        let rate = get_stablecoin_by_obc<CoinType>(
            self,
            gas_coin_map::get_default_rate(),
        );
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
        obc_balance: Balance<OBC>,
        parameters: ObcSystemParameters,
        ctx: &mut TxContext
    ): (Treasury, Balance<OBC>) {
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
        if (balance::value<OBC>(&obc_balance) > 0) {
            let deposit_balance = balance::split(&mut obc_balance, treasury::next_epoch_obc_required(&t));
            treasury::deposit(&mut t, coin::from_balance(deposit_balance, ctx));
            treasury::rebalance_first_init(&mut t, ctx);
        };
        (t, obc_balance)
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
        ctx: &mut TxContext,
    ): Balance<OBC> {
        let amount = coin::value(&coin_sc);
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
        let amount = treasury::next_epoch_obc_required(&self.treasury);
        let withdraw_balance =
            treasury_pool::withdraw_to_treasury(&mut self.treasury_pool, amount, ctx);
        if (balance::value(&withdraw_balance) > 0) {
            treasury::deposit(&mut self.treasury, coin::from_balance(withdraw_balance, ctx));
        } else {
            balance::destroy_zero(withdraw_balance);
        };
        treasury::rebalance(&mut self.treasury, clock, ctx);
    }

    /// X-vault
    public fun vault_info<StableCoinType>(self: &ObcSystemStateInner): VaultInfo {
        treasury::vault_info<StableCoinType>(&self.treasury)
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

    public(friend) fun create_obcdao_action(
        self: &mut ObcSystemStateInner,
        payment: Coin<OBC>,

        actionName: vector<u8>,
        ctx: &mut TxContext) {
        obc_dao::create_obcdao_action(&mut self.dao, payment, actionName, ctx);
    }

    public(friend) fun propose(
        self: &mut ObcSystemStateInner,
        version_id: u64,
        payment: Coin<OBC>,
        action_id: u64,
        action_delay: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        obc_dao:: propose(&mut self.dao, version_id, payment, action_id, action_delay, clock, ctx);
    }

    public(friend) fun set_voting_delay(self: &mut ObcSystemStateInner, manager_key: &OBCDaoManageKey, value: u64) {
        obc_dao::set_voting_delay(&mut self.dao, manager_key, value);
    }

    public(friend) fun set_voting_period(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        value: u64,
    ) {
        obc_dao::set_voting_period(&mut self.dao, manager_key, value);
    }

    public(friend) fun set_voting_quorum_rate(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        value: u8,
    ) {
        obc_dao::set_voting_quorum_rate(&mut self.dao, manager_key, value);
    }

    public(friend) fun set_min_action_delay(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        value: u64,
    ) {
        obc_dao::set_min_action_delay(&mut self.dao, manager_key, value);
    }


    public(friend) fun destroy_terminated_proposal(
        self: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock
    ) {
        obc_dao::destroy_terminated_proposal(&mut self.dao, manager_key, proposal, clock);
    }

    public(friend) fun judge_proposal_state(wrapper: &mut ObcSystemStateInner, current_time: u64) {
        let proposal_record = obc_dao::getProposalRecord(&mut wrapper.dao);
        let size: u64 = vec_map::size(&proposal_record);
        let i = 0;
        while (i < size) {
            let (_, proposalInfo) = vec_map::get_entry_by_idx(&proposal_record, size - 1);
            let cur_status = obc_dao::judge_proposal_state(proposalInfo, current_time);
            obc_dao::set_current_status_into_dao(&mut wrapper.dao, proposalInfo, cur_status);
            i = i + 1;
        };
    }

    public(friend) fun modify_proposal(
        system_state: &mut ObcSystemStateInner,
        proposal_obj: &mut Proposal,
        index: u8,
        clock: &Clock
    ) {
        obc_dao::modify_proposal_obj(&mut system_state.dao, proposal_obj, index, clock);
    }

    public(friend) fun cast_vote(
        system_state: &mut ObcSystemStateInner,
        proposal: &mut Proposal,
        coin: VotingObc,
        agreeInt: u8,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        obc_dao::cast_vote(&mut system_state.dao, proposal, coin, agreeInt, clock, ctx);
    }

    public(friend) fun change_vote(
        system_state: &mut ObcSystemStateInner,
        my_vote: &mut Vote,
        proposal: &mut Proposal,
        agree: bool,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        obc_dao::change_vote(&mut system_state.dao, my_vote, proposal, agree, clock, ctx);
    }

    public(friend) fun queue_proposal_action(
        system_state: &mut ObcSystemStateInner,
        manager_key: &OBCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        obc_dao::queue_proposal_action(&mut system_state.dao, manager_key, proposal, clock);
    }

    public(friend) fun revoke_vote(
        system_state: &mut ObcSystemStateInner,
        proposal: &mut Proposal,
        my_vote: Vote,
        voting_power: u64,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        obc_dao::revoke_vote(&mut system_state.dao, proposal, my_vote, voting_power, clock, ctx);
    }

    public fun withdraw_voting(system_state: &mut ObcSystemStateInner,
                               voting_obc: VotingObc,
                               ctx: &mut TxContext) {
        obc_dao::withdraw_voting(&mut system_state.dao, voting_obc, ctx);
    }

    public(friend) fun create_voting_obc(system_state: &mut ObcSystemStateInner,
                                         coin: Coin<OBC>,
                                         ctx: &mut TxContext) {
        obc_dao::create_voting_obc(&mut system_state.dao, coin, ctx);
    }
}

module bfc_system::bfc_system_state_inner {
    use std::string;
    use sui::balance;
    use sui::balance::{Balance, Supply};
    use sui::clock::Clock;
    use sui::coin;
    use sui::coin::Coin;
    use sui::bfc::BFC;
    use sui::tx_context::TxContext;
    use sui::vec_map;

    use bfc_system::exchange_inner;
    use bfc_system::exchange_inner::ExchangePool;
    use bfc_system::gas_coin_map;
    use bfc_system::gas_coin_map::{GasCoinEntity, GasCoinMap};
    use bfc_system::bfc_dao::{Self, Dao, Proposal, Vote};
    use bfc_system::bfc_dao_manager::{BFCDaoManageKey, ManagerKeyBfc};
    use bfc_system::treasury::{Self, Treasury};
    use bfc_system::treasury_pool;
    use bfc_system::treasury_pool::TreasuryPool;
    use bfc_system::busd::BUSD;
    use bfc_system::vault;
    use bfc_system::vault::VaultInfo;
    use bfc_system::voting_pool::VotingBfc;

    friend bfc_system::bfc_system;

    const BFC_SYSTEM_STATE_START_ROUND: u64 = 0;
    const DEFAULT_ADMIN_ADDRESSES: vector<address> = vector[
        // @0x23027681c7d461e3db271aeed97b5da2b6e157350fa2ff659a7ff9cccb28cc00,
        // @0x905973e8fae0c89c6c1da33751db3f828bda228e0171231b02052fbbebd48f68,
        // @0x363e4d3ee8a6400e21bd0cb0c8ecc876f3a1fe1e0f06ffdd67369bd982d39faf,
        // @0x7113a31aa484dfca371f854ae74918c7463c7b3f1bf4c1fe8ef28835e88fd590,
        // @0xdcbb951dc6c91cb4838876825daef3b361ca84d3f1e56e89ede66ef15975b4b8,
        // @0x4cfd9d0cb99b422416a680868f2e4e04446a15939042c2fd42104e99fc1da57b,
        // @0x3212e3b30a5571b6538560ece888482f2908bd5a95cbf6305ed4052ceb1899dd,
        @0x0,
    ];

    spec module { pragma verify = false; }

    struct BfcSystemStateInner has store {
        round: u64,
        /// Contains gas coin information
        gas_coin_map: GasCoinMap,
        /// Exchange gas coin pool
        exchange_pool: ExchangePool<BUSD>,
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

    struct BfcSystemParameters has drop, copy {
        chain_start_timestamp_ms: u64,
        treasury_parameters: TreasuryParameters,
    }

    const BFC_SYSTEM_TREASURY_KEY: u64 = 1;

    public(friend) fun create_inner_state(
        usd_supply: Supply<BUSD>,
        bfc_balance: Balance<BFC>,
        parameters: BfcSystemParameters,
        ctx: &mut TxContext,
    ): BfcSystemStateInner {
        // init gas coin mappings
        let init_gas_coins_map = vec_map::empty<address, GasCoinEntity>();
        let gas_coin_map = gas_coin_map::new(init_gas_coins_map, ctx);
        let exchange_pool = exchange_inner::new_exchange_pool<BUSD>(ctx, 0);
        let dao = bfc_dao::create_dao(DEFAULT_ADMIN_ADDRESSES, ctx);
        let (t, remain_balance) = create_treasury(usd_supply, bfc_balance, parameters, ctx);
        let tp = treasury_pool::create_treasury_pool(remain_balance, ctx);

        BfcSystemStateInner {
            round: BFC_SYSTEM_STATE_START_ROUND,
            gas_coin_map,
            exchange_pool,
            dao,
            treasury: t,
            treasury_pool: tp,
        }
    }

    public (friend) fun create_stake_manager_key( payment: Coin<BFC>,
                                                  ctx: &mut TxContext) {
        bfc_dao::create_stake_manager_key(payment, ctx);
    }

    public(friend) fun unstake_manager_key(key: BFCDaoManageKey,
                                           token: ManagerKeyBfc,
                                           ctx: &mut TxContext) {
        bfc_dao::unstake_manager_key(key, token, ctx);
    }

    public(friend) fun update_round(
        inner: &mut BfcSystemStateInner,
        round: u64,
    ) {
        inner.round = round;
    }

    public(friend) fun request_exchange_stable(
        inner: &mut BfcSystemStateInner,
        stable: Coin<BUSD>,
        ctx: &mut TxContext,
    ): Balance<BFC> {
        //get exchange rate
        let rate = gas_coin_map::requst_get_exchange_rate<BUSD>(&inner.gas_coin_map, &stable);
        exchange_inner::request_exchange_stable<BUSD>(rate, &mut inner.exchange_pool, stable, ctx)
    }

    public(friend) fun request_exchange_all(
        inner: &mut BfcSystemStateInner,
        ctx: &mut TxContext
    ) {
        //get bfc amount of inner exchange pool
        let bfc_amount = exchange_inner::get_bfc_amount(&inner.exchange_pool);
        if (bfc_amount > 0) {
            //set pool is disactivate
            let epoch = exchange_inner::dis_activate(&mut inner.exchange_pool);
            //get stable balance
            let stable_balance = exchange_inner::request_withdraw_all_stable(&mut inner.exchange_pool);
            //exchange from stable swap
            let bfc_balance = swap_stablecoin_to_bfc_balance(
                inner,
                coin::from_balance(stable_balance, ctx),
                ctx,
            );
            //add bfc to inner exchange pool
            exchange_inner::request_deposit_bfc_balance(&mut inner.exchange_pool, bfc_balance);
            // active pool
            exchange_inner::activate(&mut inner.exchange_pool, epoch);
        }
    }

    ///Request withdraw stable coin.
    public(friend) fun request_withdraw_stable(
        inner: &mut BfcSystemStateInner,
    ): Balance<BUSD> {
        exchange_inner::request_withdraw_all_stable(&mut inner.exchange_pool)
    }

    /// Getter of the gas coin exchange pool rate.
    public(friend) fun requst_get_exchange_rate<CoinType>(
        self: &BfcSystemStateInner,
        _stable: &Coin<CoinType>
    ): u64 {
        vault::calculated_swap_result_amount_out(&get_stablecoin_by_bfc<CoinType>(
            self,
            gas_coin_map::get_default_rate(),
        ))
    }

    public(friend) fun request_add_gas_coin<CoinType>(
        self: &mut BfcSystemStateInner,
        gas_coin: &Coin<CoinType>,
        rate: u64,
    ) {
        gas_coin_map::request_add_gas_coin<CoinType>(&mut self.gas_coin_map, gas_coin, rate)
    }

    public(friend) fun request_update_gas_coin<CoinType>(
        self: &mut BfcSystemStateInner,
        gas_coin: &Coin<CoinType>,
    ) {
        let rate = vault::calculated_swap_result_amount_out(&get_stablecoin_by_bfc<CoinType>(
            self,
            gas_coin_map::get_default_rate(),
        ));
        gas_coin_map::request_update_gas_coin(&mut self.gas_coin_map, gas_coin, rate)
    }

    public(friend) fun request_remove_gas_coin<CoinType>(
        self: &mut BfcSystemStateInner,
        gas_coin: &Coin<CoinType>,
    ) {
        gas_coin_map::request_remove_gas_coin<CoinType>(&mut self.gas_coin_map, gas_coin)
    }

    /// Init exchange pool by add bfc coin.
    public(friend) fun init_exchange_pool(
        self: &mut BfcSystemStateInner,
        coin: Coin<BFC>,
    ) {
        exchange_inner::add_bfc_to_pool(&mut self.exchange_pool, coin)
    }

    public(friend) fun get_bfc_amount(
        self:&BfcSystemStateInner,
    ):u64 {
    exchange_inner::get_bfc_amount(&self.exchange_pool)
   }

    /// X treasury  init treasury
    public(friend) fun create_treasury(
        supply: Supply<BUSD>,
        bfc_balance: Balance<BFC>,
        parameters: BfcSystemParameters,
        ctx: &mut TxContext
    ): (Treasury, Balance<BFC>) {
        let treasury_parameters = parameters.treasury_parameters;
        let t = treasury::create_treasury(treasury_parameters.time_interval, ctx);

        treasury::init_vault_with_positions<BUSD>(
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
        if (balance::value<BFC>(&bfc_balance) > 0) {
            let deposit_balance = balance::split(&mut bfc_balance, treasury::next_epoch_bfc_required(&t));
            treasury::deposit(&mut t, coin::from_balance(deposit_balance, ctx));
            treasury::rebalance_first_init(&mut t, ctx);
        };
        (t, bfc_balance)
    }

    /// swap bfc to stablecoin
    public(friend) fun swap_bfc_to_stablecoin<StableCoinType>(
        self: &mut BfcSystemStateInner,
        coin_bfc: Coin<BFC>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        treasury::mint<StableCoinType>(&mut self.treasury, coin_bfc, amount, ctx);
    }

    public(friend) fun swap_bfc_to_stablecoin_balance<StableCoinType>(
        self: &mut BfcSystemStateInner,
        coin_bfc: Coin<BFC>,
        amount: u64,
        ctx: &mut TxContext,
    ): Balance<StableCoinType> {
        treasury::mint_internal<StableCoinType>(&mut self.treasury, coin_bfc, amount, ctx)
    }

    /// swap stablecoin to bfc
    public(friend) fun swap_stablecoin_to_bfc<StableCoinType>(
        self: &mut BfcSystemStateInner,
        coin_sc: Coin<StableCoinType>,
        amount: u64,
        ctx: &mut TxContext,
    ) {
        treasury::redeem<StableCoinType>(&mut self.treasury, coin_sc, amount, ctx);
    }

    public(friend) fun swap_stablecoin_to_bfc_balance<StableCoinType>(
        self: &mut BfcSystemStateInner,
        coin_sc: Coin<StableCoinType>,
        ctx: &mut TxContext,
    ): Balance<BFC> {
        let amount = coin::value(&coin_sc);
        treasury::redeem_internal<StableCoinType>(&mut self.treasury, coin_sc, amount, ctx)
    }

    public(friend) fun get_stablecoin_by_bfc<StableCoinType>(
        self: &BfcSystemStateInner,
        amount: u64
    ): vault::CalculatedSwapResult
    {
        treasury::calculate_swap_result<StableCoinType>(&self.treasury, false, amount)
    }

    public(friend) fun get_bfc_by_stablecoin<StableCoinType>(
        self: &BfcSystemStateInner,
        amount: u64
    ): vault::CalculatedSwapResult
    {
        treasury::calculate_swap_result<StableCoinType>(&self.treasury, true, amount)
    }

    /// X-treasury
    public fun next_epoch_bfc_required(self: &BfcSystemStateInner): u64 {
        treasury::next_epoch_bfc_required(&self.treasury)
    }

    public fun treasury_balance(self: &BfcSystemStateInner): u64 {
        treasury::get_balance(&self.treasury)
    }

    public(friend) fun deposit_to_treasury(self: &mut BfcSystemStateInner, coin_bfc: Coin<BFC>) {
        treasury::deposit(&mut self.treasury, coin_bfc);
    }

    public(friend) fun rebalance(
        self: &mut BfcSystemStateInner,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let amount = treasury::next_epoch_bfc_required(&self.treasury);
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
    public fun vault_info<StableCoinType>(self: &BfcSystemStateInner): VaultInfo {
        treasury::vault_info<StableCoinType>(&self.treasury)
    }

    public(friend) fun bfc_system_stat_parameter(
        position_number: u32,
        tick_spacing: u32,
        spacing_times: u32,
        initialize_price: u128,
        time_interval: u32,
        base_point: u64,
        max_counter_times: u32,
        chain_start_timestamp_ms: u64,
    ): BfcSystemParameters {
        let treasury_parameters = TreasuryParameters {
            position_number,
            tick_spacing,
            spacing_times,
            initialize_price,
            time_interval,
            max_counter_times,
            base_point,
        };
        BfcSystemParameters {
            treasury_parameters,
            chain_start_timestamp_ms,
        }
    }

    public(friend) fun create_bfcdao_action(
        self: &mut BfcSystemStateInner,
        payment: Coin<BFC>,
        actionName: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext) {
        bfc_dao::create_bfcdao_action(&mut self.dao, payment, actionName,clock, ctx);
    }

    public(friend) fun propose(
        self: &mut BfcSystemStateInner,
        version_id: u64,
        payment: Coin<BFC>,
        action_id: u64,
        action_delay: u64,
        description: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao:: propose(&mut self.dao, version_id, payment, action_id, action_delay, description, clock, ctx);
    }

    public(friend) fun set_voting_delay(self: &mut BfcSystemStateInner, manager_key: &BFCDaoManageKey, value: u64) {
        bfc_dao::set_voting_delay(&mut self.dao, manager_key, value);
    }

    public(friend) fun set_voting_period(
        self: &mut BfcSystemStateInner,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        bfc_dao::set_voting_period(&mut self.dao, manager_key, value);
    }

    public(friend) fun set_voting_quorum_rate(
        self: &mut BfcSystemStateInner,
        manager_key: &BFCDaoManageKey,
        value: u8,
    ) {
        bfc_dao::set_voting_quorum_rate(&mut self.dao, manager_key, value);
    }

    public(friend) fun set_min_action_delay(
        self: &mut BfcSystemStateInner,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        bfc_dao::set_min_action_delay(&mut self.dao, manager_key, value);
    }


    public(friend) fun destroy_terminated_proposal(
        self: &mut BfcSystemStateInner,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock
    ) {
        bfc_dao::destroy_terminated_proposal(&mut self.dao, manager_key, proposal, clock);
    }

    public(friend) fun judge_proposal_state(wrapper: &mut BfcSystemStateInner, current_time: u64) {
        let proposal_record = bfc_dao::getProposalRecord(&mut wrapper.dao);
        let size: u64 = vec_map::size(&proposal_record);
        let i = 0;
        while (i < size) {
            let (_, proposalInfo) = vec_map::get_entry_by_idx(&proposal_record, i);
            let cur_status = bfc_dao::judge_proposal_state(proposalInfo, current_time);
            bfc_dao::set_current_status_into_dao(&mut wrapper.dao, proposalInfo, cur_status);
            i = i + 1;
        };
    }

    public(friend) fun modify_proposal(
        system_state: &mut BfcSystemStateInner,
        proposal_obj: &mut Proposal,
        index: u8,
        clock: &Clock
    ) {
        bfc_dao::modify_proposal_obj(&mut system_state.dao, proposal_obj, index, clock);
    }

    public(friend) fun cast_vote(
        system_state: &mut BfcSystemStateInner,
        proposal: &mut Proposal,
        coin: VotingBfc,
        agreeInt: u8,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::cast_vote(&mut system_state.dao, proposal, coin, agreeInt, clock, ctx);
    }

    public(friend) fun change_vote(
        system_state: &mut BfcSystemStateInner,
        my_vote: &mut Vote,
        proposal: &mut Proposal,
        agree: bool,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::change_vote(&mut system_state.dao, my_vote, proposal, agree, clock, ctx);
    }

    public(friend) fun queue_proposal_action(
        system_state: &mut BfcSystemStateInner,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        bfc_dao::queue_proposal_action(&mut system_state.dao, manager_key, proposal, clock);
    }

    public(friend) fun revoke_vote(
        system_state: &mut BfcSystemStateInner,
        proposal: &mut Proposal,
        my_vote: Vote,
        voting_power: u64,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::revoke_vote(&mut system_state.dao, proposal, my_vote, voting_power, clock, ctx);
    }

    public fun withdraw_voting(system_state: &mut BfcSystemStateInner,
                               voting_bfc: VotingBfc,
                                clock: & Clock,
                               ctx: &mut TxContext) {
        bfc_dao::withdraw_voting(&mut system_state.dao, voting_bfc, clock, ctx);
    }

    public(friend) fun create_voting_bfc(system_state: &mut BfcSystemStateInner,
                                         coin: Coin<BFC>,
                                        clock: & Clock,
                                         ctx: &mut TxContext) {
        bfc_dao::create_voting_bfc(&mut system_state.dao, coin, clock, ctx);
    }
}

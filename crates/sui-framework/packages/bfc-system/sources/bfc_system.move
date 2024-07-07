module bfc_system::bfc_system {
    use std::ascii;
    use bfc_system::bfc_dao;
    use bfc_system::voting_pool::VotingBfc;
    use sui::balance::{Balance, Supply};
    use sui::coin;
    use sui::coin::Coin;
    use sui::clock::{Clock};
    use sui::dynamic_field;

    use sui::bfc::BFC;
    use sui::object::UID;
    use sui::transfer;
    use sui::tx_context::TxContext;
    use sui::vec_map::VecMap;

    use bfc_system::busd::{BUSD};
    use bfc_system::bjpy::{BJPY};
    use bfc_system::bkrw::{BKRW};
    use bfc_system::baud::{BAUD};
    use bfc_system::bars::{BARS};
    use bfc_system::bbrl::{BBRL};
    use bfc_system::bcad::{BCAD};
    use bfc_system::beur::{BEUR};
    use bfc_system::bgbp::{BGBP};
    use bfc_system::bidr::{BIDR};
    use bfc_system::binr::{BINR};
    use bfc_system::brub::{BRUB};
    use bfc_system::bsar::{BSAR};
    use bfc_system::btry::{BTRY};
    use bfc_system::bzar::{BZAR};
    use bfc_system::bmxn::{BMXN};
    use bfc_system::mgg::{MGG};

    use bfc_system::vault;
    use bfc_system::vault::VaultInfo;
    use bfc_system::position::Position;
    use bfc_system::treasury::TreasuryPauseCap;
    use bfc_system::tick::Tick;
    use bfc_system::bfc_dao_manager::{BFCDaoManageKey, ManagerKeyBfc};
    use bfc_system::bfc_dao::{Proposal, Vote};
    use bfc_system::bfc_system_state_inner;
    use bfc_system::bfc_system_state_inner::{BfcSystemStateInner, BfcSystemParameters};

    #[test_only]
    friend bfc_system::bfc_system_tests;

    struct BfcSystemState has key {
        id: UID,
        version: u64
    }

    public entry fun create_stake_manager_key( payment: Coin<BFC>,
                                               ctx: &mut TxContext) {
        bfc_system_state_inner::create_stake_manager_key(payment, ctx);
    }

    public entry fun unstake_manager_key(key: BFCDaoManageKey,
                                         token: ManagerKeyBfc,
                                         ctx: &mut TxContext) {
        bfc_system_state_inner::unstake_manager_key(key, token, ctx);
    }


    const BFC_SYSTEM_STATE_VERSION_V1: u64 = 1;

    spec module { pragma verify = false; }

    public(friend) fun create(
        id: UID,
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
        ctx: &mut TxContext
    ) {
        let inner_state = bfc_system_state_inner::create_inner_state(
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
            ctx,
        );
        let self = BfcSystemState {
            id,
            version: BFC_SYSTEM_STATE_VERSION_V1
        };

        dynamic_field::add(&mut self.id, BFC_SYSTEM_STATE_VERSION_V1, inner_state);

        transfer::share_object(self);
    }

    #[test_only]
    public fun change_round( wrapper: &mut BfcSystemState, round: u64) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::update_round(inner_state, round);
    }

    #[test_only]
    public fun bfc_round_test(
        wrapper: &mut BfcSystemState,
        clock: &Clock,
        round: u64,
        epoch_start_time: u64,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::update_round(inner_state, round);
        // X-treasury rebalance
        bfc_system_state_inner::rebalance(inner_state, clock, ctx);
        bfc_system_state_inner::judge_proposal_state(inner_state, epoch_start_time);
    }

    #[allow(unused_function)]
    fun bfc_round(
        wrapper: &mut BfcSystemState,
        round: u64,
        epoch_start_time: u64,
    ) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::update_round(inner_state, round);
        bfc_system_state_inner::judge_proposal_state(inner_state, epoch_start_time);
    }

    #[allow(unused_function)]
    fun inner_stablecoin_to_bfc<StableCoinType>(
        _self: &mut BfcSystemState,
        _balance: Balance<StableCoinType>,
        expect: u64,
        _ctx: &mut TxContext,
    ): Balance<BFC>
    {
        // wouldn't return remain balance<StableCoinType> to system
        let inner_state = load_system_state_mut(_self);
        let bfc_balance = bfc_system_state_inner::swap_stablecoin_to_bfc_balance(inner_state, coin::from_balance(_balance, _ctx), expect,_ctx);
        bfc_balance
    }

    fun load_system_state(
        self: &BfcSystemState,
    ): &BfcSystemStateInner {
        dynamic_field::borrow(&self.id, self.version)
    }
    public fun load_bfc_system_state(id: &UID): &BfcSystemStateInner {
        dynamic_field::borrow(id, BFC_SYSTEM_STATE_VERSION_V1)
    }
    public fun load_bfc_system_state_mut(id: &mut UID): &mut BfcSystemStateInner {
        dynamic_field::borrow_mut(id, BFC_SYSTEM_STATE_VERSION_V1)
    }

    fun load_system_state_mut(
        self: &mut BfcSystemState
    ): &mut BfcSystemStateInner {
        dynamic_field::borrow_mut(&mut self.id, self.version)
    }

    public fun get_exchange_rate(id: &UID): VecMap<ascii::String, u64> {
        let inner = load_bfc_system_state(id);
        bfc_system_state_inner::get_rate_map(inner)
    }

    public entry fun destroy_terminated_proposal(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::destroy_terminated_proposal(system_state, manager_key, proposal, clock);
    }

    public entry fun propose(
        wrapper: &mut BfcSystemState,
        version_id : u64,
        payment: &mut Coin<BFC>,
        action_id: u64,
        action_delay: u64,
        description: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::propose(system_state, version_id, payment, action_id, action_delay, description, clock, ctx);
    }

    public entry fun remove_propose( wrapper: &mut BfcSystemState,key: &BFCDaoManageKey,proposal_id: u64){
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::remove_proposal(system_state,key,proposal_id);
    }

    public entry fun remove_action( wrapper: &mut BfcSystemState,key: &BFCDaoManageKey,action_id: u64){
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::remove_action(system_state,key,action_id);
    }

    public entry fun create_bfcdao_action(
        wrapper: &mut BfcSystemState,
        payment: &mut Coin<BFC>,
        actionName: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::create_bfcdao_action(system_state, payment, actionName,clock, ctx);
    }

    public entry fun judge_proposal_state(wrapper: &mut BfcSystemState, current_time: u64) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::judge_proposal_state(system_state, current_time);
    }

    public entry fun set_voting_period(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::set_voting_period(system_state, manager_key, value);
    }

    // public fun modify_proposal(wrapper: &mut BfcSystemState, proposal_obj: &mut Proposal, index: u8, clock: &Clock) {
    //     let system_state = load_system_state_mut(wrapper);
    //     bfc_system_state_inner::modify_proposal(system_state, proposal_obj, index, clock);
    // }

    public entry fun set_voting_quorum_rate(wrapper: &mut BfcSystemState, manager_key: &BFCDaoManageKey, value: u8,){
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::set_voting_quorum_rate(system_state, manager_key, value);
    }

    public entry fun set_min_action_delay(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::set_min_action_delay(system_state, manager_key, value);
    }

    public entry fun withdraw_voting(   wrapper: &mut BfcSystemState,
                                 voting_bfc: VotingBfc,
                                    clock: &Clock,
                                 ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::withdraw_voting(system_state, voting_bfc,clock, ctx);
    }

    public entry fun create_voting_bfc(wrapper: &mut BfcSystemState,
                                 coin: Coin<BFC>,
                                    clock: &Clock,
                                 ctx: &mut TxContext) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::create_voting_bfc(system_state, coin,clock, ctx);
    }

    /// X treasury rebalance
    public fun rebalance(
        wrapper: &mut BfcSystemState,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::rebalance(inner_state, clock, ctx);
    }

    public fun rebalance_with_one_stablecoin<StableCoinType>(
        wrapper: &mut BfcSystemState,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::rebalance_with_one_stablecoin<StableCoinType>(inner_state, clock, ctx);
    }

    /// X treasury  swap bfc to stablecoin
    public entry fun swap_bfc_to_stablecoin<StableCoinType>(
        wrapper: &mut BfcSystemState,
        native_coin: Coin<BFC>,
        clock: &Clock,
        amount: u64,
        min_amount: u64,
        deadline: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::swap_bfc_to_stablecoin<StableCoinType>(system_state, native_coin, clock, amount, min_amount, deadline, ctx);
    }

    /// X treasury  swap stablecoin to bfc
    public entry fun swap_stablecoin_to_bfc<StableCoinType>(
        wrapper: &mut BfcSystemState,
        stable_coin: Coin<StableCoinType>,
        clock: &Clock,
        amount: u64,
        min_amount: u64,
        deadline: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::swap_stablecoin_to_bfc<StableCoinType>(system_state, stable_coin, clock, amount, min_amount, deadline, ctx);
    }

    public fun get_stablecoin_by_bfc<StableCoinType>(
        wrapper: &BfcSystemState,
        amount: u64,
    ): vault::CalculatedSwapResult
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_stablecoin_by_bfc<StableCoinType>(system_state, amount)
    }

    public fun get_bfc_by_stablecoin<StableCoinType>(
        wrapper: &BfcSystemState,
        amount: u64,
    ): vault::CalculatedSwapResult
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_bfc_by_stablecoin<StableCoinType>(system_state, amount)
    }

    public fun vault_info<StableCoinType>(wrapper: &BfcSystemState): VaultInfo {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::vault_info<StableCoinType>(inner_state)
    }

    public fun vault_ticks<StableCoinType>(wrapper: &BfcSystemState): vector<Tick> {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::vault_ticks<StableCoinType>(inner_state)
    }

    public fun vault_positions<StableCoinType>(wrapper: &BfcSystemState): vector<Position> {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::vault_positions<StableCoinType>(inner_state)
    }

    public fun total_supply<StableCoinType>(wrapper: &BfcSystemState): u64 {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::get_total_supply<StableCoinType>(inner_state)
    }

    public fun get_bfc_exchange_rate<StableCoinType>(wrapper: &BfcSystemState): u64
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_bfc_exchange_rate<StableCoinType>(system_state)
    }

    public fun get_stablecoin_exchange_rate<StableCoinType>(wrapper: &BfcSystemState): u64
    {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_stablecoin_exchange_rate<StableCoinType>(system_state)
    }

    public fun bfc_required(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::bfc_required(system_state)
    }

    public fun bfc_required_with_one_stablecoin<StableCoinType>(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::bfc_required_with_one_stablecoin<StableCoinType>(system_state)
    }

    public fun treasury_balance(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::treasury_balance(system_state)
    }

    public entry fun deposit_to_treasury(self: &mut BfcSystemState, bfc: Coin<BFC>) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::deposit_to_treasury(inner_state, bfc)
    }

    public entry fun deposit_to_treasury_pool(self: &mut BfcSystemState, bfc: Coin<BFC>) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::deposit_to_treasury_pool(inner_state, bfc)
    }

    public fun deposit_to_treasury_pool_no_entry(self: &mut BfcSystemState, bfc_balance: Balance<BFC>, ctx: &mut TxContext) {
        let inner_state = load_system_state_mut(self);
        let bfc= coin::from_balance(bfc_balance, ctx);
        bfc_system_state_inner::deposit_to_treasury_pool(inner_state, bfc)
    }

    public entry fun vault_set_pause<StableCoinType>(
        cap: &TreasuryPauseCap,
        wrapper: &mut BfcSystemState,
        pause: bool
    ) {
        let inner_state = load_system_state_mut(wrapper);
        bfc_system_state_inner::vault_set_pause<StableCoinType>(cap, inner_state, pause)
    }

    /// DAO
    public entry fun set_voting_delay(
        self: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::set_voting_delay(inner_state, manager_key, value);
    }

    public entry fun cast_vote(
        self: &mut BfcSystemState,
        proposal: &mut Proposal,
        coin: VotingBfc,
        agreeInt: u8,
        clock: & Clock,
        ctx: &mut TxContext,
    )  {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::cast_vote(inner_state, proposal, coin, agreeInt, clock, ctx);
    }

    public entry fun change_vote(
        self: &mut BfcSystemState,
        my_vote: &mut Vote,
        proposal: &mut Proposal,
        agree: bool,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::change_vote(inner_state, my_vote, proposal, agree, clock, ctx);
    }

    public entry fun queue_proposal_action(
        self: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::queue_proposal_action(inner_state, manager_key, proposal, clock);
    }

    public entry fun revoke_vote(
        self: &mut BfcSystemState,
        proposal: &mut Proposal,
        my_vote:  Vote,
        voting_power: u64,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        let inner_state = load_system_state_mut(self);
        bfc_system_state_inner::revoke_vote(inner_state, proposal, my_vote, voting_power, clock, ctx);
    }

    public entry fun unvote_votes(
        proposal: &mut Proposal,
        vote: Vote,
        clock: & Clock,
        ctx: &mut TxContext,
    ) {
        bfc_dao::unvote_votes(proposal, vote, clock, ctx);
    }

    public entry fun vote_of(
        vote: &Vote,
        proposal: &Proposal,
        ctx: &mut TxContext,
    ) {
        bfc_dao::vote_of(vote, proposal, ctx);
    }

    public entry fun has_vote(
        vote: &Vote,
        proposal: &Proposal,
    ) {
        bfc_dao::has_vote(vote, proposal);
    }


    // public entry fun cluster_add_admin(
    //     new_admin:address,
    //     ctx: &mut TxContext,
    // ) {
    //     bfc_dao::add_admin(new_admin, ctx);
    // }
}

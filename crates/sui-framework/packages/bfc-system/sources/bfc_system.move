module bfc_system::bfc_system {
    use std::ascii;
    use bfc_system::position::Position;
    use bfc_system::tick::Tick;
    use bfc_system::bfc_dao;
    use bfc_system::voting_pool::VotingBfc;
    use sui::balance::{Balance, Supply};
    use sui::coin;
    use sui::coin::Coin;
    use sui::clock::{Clock};
    use sui::dynamic_field;

    use sui::bfc::BFC;
    use sui::clock;
    use sui::vec_map::VecMap;
    use sui::vec_set;
    use sui::vec_set::VecSet;

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
    use bfc_system::bfc_dao_manager::{BFCDaoManageKey, ManagerKeyBfc};
    use bfc_system::bfc_dao::{Proposal, Vote};
    use bfc_system::bfc_system_state_inner;
    use bfc_system::bfc_system_state_inner::{BfcSystemStateInner, BfcSystemParameters, BfcSystemStateInnerV2, BfcSystemModifyCap,
        BfcSystemAdminCap
    };
    use bfc_system::treasury::{TreasuryPauseCap};

    public struct BfcSystemState has key {
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
    const BFC_SYSTEM_STATE_VERSION_V2: u64 = 2;

    //spec module { pragma verify = false; }

    public(package) fun create(
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
        bfc_skip_init_vault: u32,
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
            bfc_skip_init_vault,
            ctx,
        );
        let mut self = BfcSystemState {
            id,
            version: BFC_SYSTEM_STATE_VERSION_V1
        };

        dynamic_field::add(&mut self.id, BFC_SYSTEM_STATE_VERSION_V1, inner_state);

        transfer::share_object(self);
    }

    #[test_only]
    public(package) fun create_for_test(
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
        bfc_skip_init_vault: u32,
        ctx: &mut TxContext
    ): address {
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
            bfc_skip_init_vault,
            ctx,
        );

        let id = sui::object::new(ctx);

        let mut self = BfcSystemState {
            id: id,
            version: BFC_SYSTEM_STATE_VERSION_V1
        };

        let system_address = object::id_address<BfcSystemState>(&self);


        dynamic_field::add(&mut self.id, BFC_SYSTEM_STATE_VERSION_V1, inner_state);
        load_system_state_mut(&mut self, ctx);

        transfer::share_object(self);
        system_address
    }

    entry public fun change_round( wrapper: &mut BfcSystemState, round: u64) {
        let inner_state = load_system_state_mut_no_ctx(wrapper);
        bfc_system_state_inner::update_round(inner_state, round);
    }

    #[allow(unused_function)]
    fun bfc_round(
        wrapper: &mut BfcSystemState,
        round: u64,
        epoch_start_time: u64,
        ctx: &mut TxContext
    ) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::update_round(inner_state, round);
        bfc_system_state_inner::judge_proposal_state(inner_state, epoch_start_time);
    }

    #[allow(unused_function)]
    fun bfc_round_v2(
        wrapper: &mut BfcSystemState,
        round: u64,
        epoch_start_time: u64,
        stable_type_name_vector : vector<ascii::String>,
        stable_rate_vector : vector<u64>,
        ctx: &mut TxContext
    ) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::update_round_v2(inner_state, round, stable_type_name_vector, stable_rate_vector);
        bfc_system_state_inner::judge_proposal_state(inner_state, epoch_start_time);
    }

    #[test_only]
    public fun load_system_state_mut_test(
        _self: &mut BfcSystemState,
        _ctx: &mut TxContext
    ): (&mut BfcSystemStateInnerV2, &mut TxContext) {
        load_system_state_mut(_self, _ctx)
    }

    #[test_only]
    public fun bfc_round_test(
        wrapper: &mut BfcSystemState,
        clock: &Clock,
        round: u64,
        epoch_start_time: u64,
        ctx: &mut TxContext,
    ) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::update_round(inner_state, round);
        // X-treasury rebalance
        bfc_system_state_inner::rebalance(inner_state, clock, _ctx);
        bfc_system_state_inner::judge_proposal_state(inner_state, epoch_start_time);
    }

    #[test_only]
    public fun bfc_round_v2_test(
        wrapper: &mut BfcSystemState,
        clock: &Clock,
        round: u64,
        epoch_start_time: u64,
        stable_type_name_vector : vector<ascii::String>,
        stable_rate_vector : vector<u64>,
        ctx: &mut TxContext,
    ) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::update_round_v2(inner_state, round, stable_type_name_vector, stable_rate_vector);
        // X-treasury rebalance
        bfc_system_state_inner::rebalance(inner_state, clock, _ctx);
        bfc_system_state_inner::judge_proposal_state(inner_state, epoch_start_time);
    }

    #[allow(unused_function)]
    fun inner_stablecoin_to_bfc<StableCoinType>(
    _self: &mut BfcSystemState,
    _balance: Balance<StableCoinType>,
    expect: u64,
    _ctx: &mut TxContext,
    ): Balance<BFC> {
        let (inner_state, ctx) = load_system_state_mut(_self, _ctx);
        if (std::type_name::get<StableCoinType>() == std::type_name::get<BUSD>()) {
            bfc_system_state_inner::swap_stablecoin_to_bfc_balance(inner_state, coin::from_balance(_balance, ctx), expect, ctx)
        } else {
            bfc_system_state_inner::add_balance_to_vault(inner_state, _balance, ctx);
            bfc_system_state_inner::withdraw_balance(inner_state, expect)
        }
    }

    #[allow(unused_function)]
    fun inner_withdraw_balance(
        _self: &mut BfcSystemState,
        expect: u64,
        _ctx: &mut TxContext,
    ): Balance<BFC> {
        let (inner_state, _ctx) = load_system_state_mut(_self, _ctx);
        bfc_system_state_inner::withdraw_balance(inner_state, expect)
    }

    fun deposit_stable_gas_coin(wrapper: &mut BfcSystemState, balance: Balance<StableCoinType>, ctx: &mut TxContext) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        inner_state.deposit_stable_gas_coin(balance, _ctx);
    }

    #[test_only]
    public fun inner_stablecoin_to_bfc_test<StableCoinType>(
        wrapper: &mut BfcSystemState,
        balance: Balance<StableCoinType>,
        expect: u64,
        ctx: &mut TxContext,
    ): Balance<BFC> {
        inner_stablecoin_to_bfc<StableCoinType>(wrapper, balance, expect, ctx)
    }

    public fun request_gas_balance(
        wrapper: &mut BfcSystemState,
        amount: u64,
        ctx: &mut TxContext,
    ): Balance<BFC> {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::request_gas_balance(inner_state, amount, _ctx)
    }

    fun load_system_state_by_uid(
        id: &UID,
    ): &BfcSystemStateInnerV2 {
        dynamic_field::borrow(id, BFC_SYSTEM_STATE_VERSION_V2)
    }

    fun load_system_state_mut_by_uid(
        id: &mut UID,
    ): &mut BfcSystemStateInnerV2 {
        dynamic_field::borrow_mut(id, BFC_SYSTEM_STATE_VERSION_V2)
    }

    fun load_system_state(
        self: &BfcSystemState,
    ): &BfcSystemStateInnerV2 {
        dynamic_field::borrow(&self.id, BFC_SYSTEM_STATE_VERSION_V2)
    }

    /// deprecated
    public fun load_bfc_system_state(id: &UID): &BfcSystemStateInner {
        dynamic_field::borrow(id, BFC_SYSTEM_STATE_VERSION_V1)
    }
    /// deprecated
    public fun load_bfc_system_state_mut(id: &mut UID): &mut BfcSystemStateInner {
        dynamic_field::borrow_mut(id, BFC_SYSTEM_STATE_VERSION_V1)
    }

    fun load_system_state_mut_no_ctx(
        self: &mut BfcSystemState,
    ): (&mut BfcSystemStateInnerV2) {
        dynamic_field::borrow_mut(&mut self.id, BFC_SYSTEM_STATE_VERSION_V2)
    }

    fun load_system_state_mut(
        _self: &mut BfcSystemState,
        _ctx: &mut TxContext
    ): (&mut BfcSystemStateInnerV2, &mut TxContext) {
        if (_self.version == 1) {
            let v1: BfcSystemStateInner = dynamic_field::remove(&mut _self.id, _self.version);
            let (mut v2, ctx) = v1.v1_to_v2(_ctx);
            let _ctx = ctx;
            bfc_system_state_inner::init_bfc_system_state_v2(&mut v2, _ctx);

            _self.version = 2;
            dynamic_field::add(&mut _self.id, _self.version, v2);

        };

        let inner: &mut BfcSystemStateInnerV2 = dynamic_field::borrow_mut(
            &mut _self.id,
             _self.version
        );
        (inner, _ctx)
    }

    #[test_only]
    public fun load_system_state_mut_for_test(self: &mut BfcSystemState,
            _ctx: &mut TxContext
    ): (&mut BfcSystemStateInnerV2, &mut TxContext) {
        return load_system_state_mut(self, _ctx)
    }

    public fun get_exchange_rate(id: &UID): VecMap<ascii::String, u64> {
        let inner = load_system_state_by_uid(id);
        bfc_system_state_inner::get_rate_map(inner)
    }

    public fun get_operation_capability(
        wrapper: &mut BfcSystemState,
        ctx: &mut TxContext,
    ): VecMap<ascii::String, VecSet<address>> {
        let (inner, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::get_operation_capability(inner)
    }

    public fun get_operation_capability_by_key(
        wrapper: &mut BfcSystemState,
        key: vector<u8>,
        ctx: &mut TxContext,
    ): VecSet<address> {
        let (inner, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::get_operation_capability_by_key(inner, &std::ascii::string(key))
    }

    public fun add_operation_capability(
        wrapper: &mut BfcSystemState,
        _cap: &BfcSystemAdminCap,
        key: vector<u8>,
        address: address,
        ctx: &mut TxContext,
    ) {
        let (inner, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::add_operation_capability(inner, std::ascii::string(key), address, _ctx)
    }

    public fun remove_operation_capability(
        wrapper: &mut BfcSystemState,
        _cap: &BfcSystemAdminCap,
        key: vector<u8>,
        address: address,
        ctx: &mut TxContext,
    ) {
        let (inner, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::remove_operation_capability(inner, &std::ascii::string(key), address, ctx)
    }

    public fun set_operation_capability(
        wrapper: &mut BfcSystemState,
        _cap: &BfcSystemAdminCap,
        key: vector<u8>,
        addresses: VecSet<address>,
        ctx: &mut TxContext,
    ) {
        let (inner, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::set_operation_capability(inner, std::ascii::string(key), addresses, ctx)
    }

    public fun set_single_operation_capability(
        wrapper: &mut BfcSystemState,
        _cap: &BfcSystemAdminCap,
        key: vector<u8>,
        address: address,
        ctx: &mut TxContext,
    ) {
        let (inner, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::set_operation_capability(inner, std::ascii::string(key), vec_set::singleton(address), ctx)
    }

    public fun set_oracle_address(wrapper: &mut BfcSystemState, address: address, ctx: &mut TxContext) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        inner_state.set_oracle_address(address, _ctx)
    }

    public fun get_oracle_address(wrapper: &mut BfcSystemState, ctx: &mut TxContext): Option<address> {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        inner_state.get_oracle_address(_ctx)
    }

    public fun add_external_stable_gas_coin(wrapper: &mut BfcSystemState, value: vector<ascii::String>, ctx: &mut TxContext) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        inner_state.add_external_stable_gas_coin(value, _ctx)
    }

    public fun delete_external_stable_gas_coin(wrapper: &mut BfcSystemState, value: ascii::String, ctx: &mut TxContext) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        inner_state.delete_external_stable_gas_coin(value, _ctx)
    }

    public entry fun remove_propose(wrapper: &mut BfcSystemState, key: &BFCDaoManageKey, proposal_id: u64) {
        let system_state = load_system_state_mut_by_uid(&mut wrapper.id);
        bfc_system_state_inner::remove_proposal(system_state, key, proposal_id);
    }

    public entry fun remove_action(wrapper: &mut BfcSystemState, key: &BFCDaoManageKey, action_id: u64) {
        let system_state = load_system_state_mut_by_uid(&mut wrapper.id);
        bfc_system_state_inner::remove_action(system_state, key, action_id);
    }

    public entry fun destroy_terminated_proposal(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let system_state = load_system_state_mut_by_uid(&mut wrapper.id);
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
        let (system_state, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::propose(system_state, version_id, payment, action_id, action_delay, description, clock, ctx);
    }

    public entry fun create_bfcdao_action(
        wrapper: &mut BfcSystemState,
        payment: &mut Coin<BFC>,
        actionName: vector<u8>,
        clock: &Clock,
        ctx: &mut TxContext) {
        let (system_state, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::create_bfcdao_action(system_state, payment, actionName,clock, ctx);
    }

    public entry fun judge_proposal_state(_wrapper: &mut BfcSystemState, _current_time: u64) {
        //let system_state = load_system_state_mut(wrapper);
        //bfc_system_state_inner::judge_proposal_state(system_state, current_time);
    }
    public entry fun judge_proposal_state_with_clock(wrapper: &mut BfcSystemState, clock: &Clock) {
        let system_state = load_system_state_mut_by_uid(&mut wrapper.id);
        let current_time = clock::timestamp_ms(clock);
        bfc_system_state_inner::judge_proposal_state(system_state, current_time);
    }


    public entry fun set_voting_period(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut_by_uid(&mut wrapper.id);
        bfc_system_state_inner::set_voting_period(system_state, manager_key, value);
    }

    // public fun modify_proposal(wrapper: &mut BfcSystemState, proposal_obj: &mut Proposal, index: u8, clock: &Clock) {
    //     let system_state = load_system_state_mut(wrapper);
    //     bfc_system_state_inner::modify_proposal(system_state, proposal_obj, index, clock);
    // }

    public entry fun set_voting_quorum_rate(wrapper: &mut BfcSystemState, manager_key: &BFCDaoManageKey, value: u8,){
        let system_state = load_system_state_mut_by_uid(&mut wrapper.id);
        bfc_system_state_inner::set_voting_quorum_rate(system_state, manager_key, value);
    }

    public entry fun set_min_action_delay(
        wrapper: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let system_state = load_system_state_mut_by_uid(&mut wrapper.id);
        bfc_system_state_inner::set_min_action_delay(system_state, manager_key, value);
    }

    public entry fun withdraw_voting(   wrapper: &mut BfcSystemState,
                                 voting_bfc: VotingBfc,
                                    clock: &Clock,
                                 ctx: &mut TxContext) {
        let (system_state, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::withdraw_voting(system_state, voting_bfc,clock, ctx);
    }

    public entry fun create_voting_bfc(wrapper: &mut BfcSystemState,
                                 coin: Coin<BFC>,
                                    clock: &Clock,
                                 ctx: &mut TxContext) {
        let (system_state, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::create_voting_bfc(system_state, coin,clock, ctx);
    }

    /// X treasury rebalance
    public fun rebalance(
        _wrapper: &mut BfcSystemState,
        _clock: &Clock,
        _ctx: &mut TxContext,
    ) {

    }

    public fun rebalance_with_one_stablecoin<StableCoinType>(
        wrapper: &mut BfcSystemState,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let (inner_state, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::rebalance_with_one_stablecoin<StableCoinType>(inner_state, clock, ctx);
    }

    public entry fun mint_stable_entry<StableCoinType>(
        wrapper: &mut BfcSystemState,
        amount: u64,
        cap: &BfcSystemModifyCap,
        ctx: &mut TxContext,
    ) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        let coin = bfc_system_state_inner::mint_stable<StableCoinType>(inner_state, amount, &cap.get_bfc_system_modify_cap_key(), _ctx);
        transfer::public_transfer(coin, tx_context::sender(ctx));
    }

    public entry fun mint_stable_entry_to_address<StableCoinType>(
        wrapper: &mut BfcSystemState,
        amount: u64,
        cap: &BfcSystemModifyCap,
        address: address,
        ctx: &mut TxContext,
    ) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        let coin = bfc_system_state_inner::mint_stable<StableCoinType>(inner_state, amount, &cap.get_bfc_system_modify_cap_key(), _ctx);
        transfer::public_transfer(coin, address);
    }

    public fun mint_stable<StableCoinType>(
        wrapper: &mut BfcSystemState,
        amount: u64,
        cap: &BfcSystemModifyCap,
        ctx: &mut TxContext,
    ): Coin<StableCoinType> {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::mint_stable<StableCoinType>(inner_state, amount, &cap.get_bfc_system_modify_cap_key(), _ctx)
    }

    public fun verify_capability(
        wrapper: &BfcSystemState,
        cap: &BfcSystemModifyCap,
        ctx: &TxContext,
    ): bool {
        let inner_state = load_system_state(wrapper);
        inner_state.verify_operation_capability(&cap.get_bfc_system_modify_cap_key(), ctx.sender())
    }

    entry public fun burn_stable<StableCoinType>(
        wrapper: &mut BfcSystemState,
        token: Coin<StableCoinType>,
        ctx: &mut TxContext,
    ){
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::burn_stable<StableCoinType>(inner_state, token);
    }

    public entry fun init_admin_capability(wrapper: &mut BfcSystemState, addresses: vector<address>, ctx: &mut TxContext) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::init_bfc_system_admins(inner_state, _ctx, addresses);
    }

    public entry fun add_admin_capability(wrapper: &mut BfcSystemState, addresses: vector<address>, cap: &BfcSystemAdminCap, ctx: &mut TxContext) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        let _ =  cap;
        bfc_system_state_inner::verify_admin_capability(inner_state, _ctx.sender());
        bfc_system_state_inner::add_bfc_system_admin_cap(inner_state, _ctx, addresses);
    }

    public entry fun remove_admin_capability(wrapper: &mut BfcSystemState, address: address, cap: &BfcSystemAdminCap, ctx: &mut TxContext) {
        let (inner_state, _ctx) = load_system_state_mut(wrapper, ctx);
        let _ =  cap;
        bfc_system_state_inner::remove_bfc_system_admin_cap(inner_state, address, _ctx);
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
        assert!(std::type_name::get<StableCoinType>() == std::type_name::get<BUSD>(), 0);
        let (system_state, ctx) = load_system_state_mut(wrapper, ctx);
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
        assert!(std::type_name::get<StableCoinType>() == std::type_name::get<BUSD>(), 0);
        let (system_state, ctx) = load_system_state_mut(wrapper, ctx);
        bfc_system_state_inner::swap_stablecoin_to_bfc<StableCoinType>(system_state, stable_coin, clock, amount, min_amount, deadline, ctx);
    }

    public fun get_stablecoin_by_bfc<StableCoinType>(
        wrapper: &BfcSystemState,
        amount: u64,
    ): vault::CalculatedSwapResult
    {
        assert!(std::type_name::get<StableCoinType>() == std::type_name::get<BUSD>(), 0);
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_stablecoin_by_bfc<StableCoinType>(system_state, amount)
    }

    public fun get_bfc_by_stablecoin<StableCoinType>(
        wrapper: &BfcSystemState,
        amount: u64,
    ): vault::CalculatedSwapResult
    {
        assert!(std::type_name::get<StableCoinType>() == std::type_name::get<BUSD>(), 0);
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::get_bfc_by_stablecoin<StableCoinType>(system_state, amount)
    }

    public fun vault_info<StableCoinType>(wrapper: &BfcSystemState): VaultInfo {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::vault_info_v2<StableCoinType>(inner_state)
    }

    public fun vault_ticks<StableCoinType>(wrapper: &BfcSystemState): vector<Tick> {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::vault_ticks_v2<StableCoinType>(inner_state)
    }

    public fun vault_positions<StableCoinType>(wrapper: &BfcSystemState): vector<Position> {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::vault_positions_v2<StableCoinType>(inner_state)
    }

    public fun total_supply<StableCoinType>(wrapper: &BfcSystemState): u64 {
        let inner_state = load_system_state(wrapper);
        bfc_system_state_inner::get_total_supply_v2<StableCoinType>(inner_state)
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

    public fun next_epoch_bfc_required(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::next_epoch_bfc_required_v2(system_state)
    }

    public fun bfc_required_with_one_stablecoin<StableCoinType>(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::bfc_required_with_one_stablecoin<StableCoinType>(system_state)
    }

    public fun treasury_balance(wrapper: &BfcSystemState): u64 {
        let system_state = load_system_state(wrapper);
        bfc_system_state_inner::treasury_balance_v2(system_state)
    }

    public entry fun deposit_to_treasury(self: &mut BfcSystemState, bfc: Coin<BFC>) {
        let inner_state = load_system_state_mut_by_uid(&mut self.id);
        bfc_system_state_inner::deposit_to_treasury(inner_state, bfc)
    }

    public fun deposit_to_treasury_inner(self: &mut BfcSystemState, bfc_balance: Balance<BFC>, ctx: &mut TxContext,
    ) {
        let (inner_state, ctx) = load_system_state_mut(self, ctx);
        let bfc= coin::from_balance(bfc_balance, ctx);
        bfc_system_state_inner::deposit_to_treasury(inner_state, bfc)
    }

    public entry fun deposit_to_treasury_pool(self: &mut BfcSystemState, bfc: Coin<BFC>) {
        let inner_state = load_system_state_mut_by_uid(&mut self.id);
        bfc_system_state_inner::deposit_to_treasury_pool(inner_state, bfc)
    }

    public  fun deposit_to_treasury_pool_no_entry(self: &mut BfcSystemState, bfc_balance: Balance<BFC>, ctx: &mut TxContext) {
        let (inner_state, ctx) = load_system_state_mut(self, ctx);
        let bfc= coin::from_balance(bfc_balance, ctx);
        bfc_system_state_inner::deposit_to_treasury_pool(inner_state, bfc)
    }

    public entry fun vault_set_pause<StableCoinType>(
        cap: &TreasuryPauseCap,
        wrapper: &mut BfcSystemState,
        pause: bool
    ) {
        let inner_state = load_system_state_mut_no_ctx(wrapper);
        bfc_system_state_inner::vault_set_pause_v2<StableCoinType>(cap, inner_state, pause)
    }

    public entry fun set_voting_delay(
        self: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        value: u64,
    ) {
        let inner_state = load_system_state_mut_by_uid(&mut self.id);
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
        let (inner_state, ctx) = load_system_state_mut(self, ctx);
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
        let (inner_state, ctx) = load_system_state_mut(self, ctx);
        bfc_system_state_inner::change_vote(inner_state, my_vote, proposal, agree, clock, ctx);
    }

    public entry fun queue_proposal_action(
        self: &mut BfcSystemState,
        manager_key: &BFCDaoManageKey,
        proposal: &mut Proposal,
        clock: & Clock,
    ) {
        let inner_state = load_system_state_mut_by_uid(&mut self.id);
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
        let (inner_state, ctx) = load_system_state_mut(self, ctx);
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

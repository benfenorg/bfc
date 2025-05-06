---
title: Module `0xc8::bfc_system`
---



-  [Resource `BfcSystemState`](#0xc8_bfc_system_BfcSystemState)
-  [Constants](#@Constants_0)
-  [Function `create_stake_manager_key`](#0xc8_bfc_system_create_stake_manager_key)
-  [Function `unstake_manager_key`](#0xc8_bfc_system_unstake_manager_key)
-  [Function `allocate_abfc`](#0xc8_bfc_system_allocate_abfc)
-  [Function `create`](#0xc8_bfc_system_create)
-  [Function `change_round`](#0xc8_bfc_system_change_round)
-  [Function `bfc_round`](#0xc8_bfc_system_bfc_round)
-  [Function `bfc_round_v2`](#0xc8_bfc_system_bfc_round_v2)
-  [Function `inner_stablecoin_to_bfc`](#0xc8_bfc_system_inner_stablecoin_to_bfc)
-  [Function `inner_withdraw_balance`](#0xc8_bfc_system_inner_withdraw_balance)
-  [Function `request_gas_balance`](#0xc8_bfc_system_request_gas_balance)
-  [Function `load_system_state_by_uid`](#0xc8_bfc_system_load_system_state_by_uid)
-  [Function `load_system_state_mut_by_uid`](#0xc8_bfc_system_load_system_state_mut_by_uid)
-  [Function `load_system_state`](#0xc8_bfc_system_load_system_state)
-  [Function `load_bfc_system_state`](#0xc8_bfc_system_load_bfc_system_state)
-  [Function `load_bfc_system_state_mut`](#0xc8_bfc_system_load_bfc_system_state_mut)
-  [Function `load_system_state_mut_no_ctx`](#0xc8_bfc_system_load_system_state_mut_no_ctx)
-  [Function `load_system_state_mut`](#0xc8_bfc_system_load_system_state_mut)
-  [Function `get_exchange_rate`](#0xc8_bfc_system_get_exchange_rate)
-  [Function `get_operation_capability`](#0xc8_bfc_system_get_operation_capability)
-  [Function `get_operation_capability_by_key`](#0xc8_bfc_system_get_operation_capability_by_key)
-  [Function `add_operation_capability`](#0xc8_bfc_system_add_operation_capability)
-  [Function `remove_operation_capability`](#0xc8_bfc_system_remove_operation_capability)
-  [Function `set_operation_capability`](#0xc8_bfc_system_set_operation_capability)
-  [Function `set_single_operation_capability`](#0xc8_bfc_system_set_single_operation_capability)
-  [Function `set_oracle_address`](#0xc8_bfc_system_set_oracle_address)
-  [Function `get_oracle_address`](#0xc8_bfc_system_get_oracle_address)
-  [Function `add_external_stable_gas_coin`](#0xc8_bfc_system_add_external_stable_gas_coin)
-  [Function `delete_external_stable_gas_coin`](#0xc8_bfc_system_delete_external_stable_gas_coin)
-  [Function `remove_propose`](#0xc8_bfc_system_remove_propose)
-  [Function `remove_action`](#0xc8_bfc_system_remove_action)
-  [Function `destroy_terminated_proposal`](#0xc8_bfc_system_destroy_terminated_proposal)
-  [Function `propose`](#0xc8_bfc_system_propose)
-  [Function `create_bfcdao_action`](#0xc8_bfc_system_create_bfcdao_action)
-  [Function `judge_proposal_state`](#0xc8_bfc_system_judge_proposal_state)
-  [Function `judge_proposal_state_with_clock`](#0xc8_bfc_system_judge_proposal_state_with_clock)
-  [Function `set_voting_period`](#0xc8_bfc_system_set_voting_period)
-  [Function `set_voting_quorum_rate`](#0xc8_bfc_system_set_voting_quorum_rate)
-  [Function `set_min_action_delay`](#0xc8_bfc_system_set_min_action_delay)
-  [Function `withdraw_voting`](#0xc8_bfc_system_withdraw_voting)
-  [Function `create_voting_bfc`](#0xc8_bfc_system_create_voting_bfc)
-  [Function `rebalance`](#0xc8_bfc_system_rebalance)
-  [Function `rebalance_with_one_stablecoin`](#0xc8_bfc_system_rebalance_with_one_stablecoin)
-  [Function `mint_stable_entry`](#0xc8_bfc_system_mint_stable_entry)
-  [Function `mint_stable_entry_to_address`](#0xc8_bfc_system_mint_stable_entry_to_address)
-  [Function `mint_stable`](#0xc8_bfc_system_mint_stable)
-  [Function `verify_capability`](#0xc8_bfc_system_verify_capability)
-  [Function `burn_stable`](#0xc8_bfc_system_burn_stable)
-  [Function `init_admin_capability`](#0xc8_bfc_system_init_admin_capability)
-  [Function `add_admin_capability`](#0xc8_bfc_system_add_admin_capability)
-  [Function `remove_admin_capability`](#0xc8_bfc_system_remove_admin_capability)
-  [Function `swap_bfc_to_stablecoin`](#0xc8_bfc_system_swap_bfc_to_stablecoin)
-  [Function `swap_stablecoin_to_bfc`](#0xc8_bfc_system_swap_stablecoin_to_bfc)
-  [Function `get_stablecoin_by_bfc`](#0xc8_bfc_system_get_stablecoin_by_bfc)
-  [Function `get_bfc_by_stablecoin`](#0xc8_bfc_system_get_bfc_by_stablecoin)
-  [Function `vault_info`](#0xc8_bfc_system_vault_info)
-  [Function `vault_ticks`](#0xc8_bfc_system_vault_ticks)
-  [Function `vault_positions`](#0xc8_bfc_system_vault_positions)
-  [Function `total_supply`](#0xc8_bfc_system_total_supply)
-  [Function `get_bfc_exchange_rate`](#0xc8_bfc_system_get_bfc_exchange_rate)
-  [Function `get_stablecoin_exchange_rate`](#0xc8_bfc_system_get_stablecoin_exchange_rate)
-  [Function `bfc_required`](#0xc8_bfc_system_bfc_required)
-  [Function `next_epoch_bfc_required`](#0xc8_bfc_system_next_epoch_bfc_required)
-  [Function `bfc_required_with_one_stablecoin`](#0xc8_bfc_system_bfc_required_with_one_stablecoin)
-  [Function `treasury_balance`](#0xc8_bfc_system_treasury_balance)
-  [Function `deposit_to_treasury`](#0xc8_bfc_system_deposit_to_treasury)
-  [Function `deposit_to_treasury_inner`](#0xc8_bfc_system_deposit_to_treasury_inner)
-  [Function `deposit_to_treasury_pool`](#0xc8_bfc_system_deposit_to_treasury_pool)
-  [Function `deposit_to_treasury_pool_no_entry`](#0xc8_bfc_system_deposit_to_treasury_pool_no_entry)
-  [Function `vault_set_pause`](#0xc8_bfc_system_vault_set_pause)
-  [Function `set_voting_delay`](#0xc8_bfc_system_set_voting_delay)
-  [Function `cast_vote`](#0xc8_bfc_system_cast_vote)
-  [Function `change_vote`](#0xc8_bfc_system_change_vote)
-  [Function `queue_proposal_action`](#0xc8_bfc_system_queue_proposal_action)
-  [Function `revoke_vote`](#0xc8_bfc_system_revoke_vote)
-  [Function `unvote_votes`](#0xc8_bfc_system_unvote_votes)
-  [Function `vote_of`](#0xc8_bfc_system_vote_of)
-  [Function `has_vote`](#0xc8_bfc_system_has_vote)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc">0x2::abfc</a>;
<b>use</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance">0x2::anonymous_balance</a>;
<b>use</b> <a href="../sui-framework/anonymous_coin.md#0x2_anonymous_coin">0x2::anonymous_coin</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
<b>use</b> <a href="../bfc-system/bars.md#0xc8_bars">0xc8::bars</a>;
<b>use</b> <a href="../bfc-system/baud.md#0xc8_baud">0xc8::baud</a>;
<b>use</b> <a href="../bfc-system/bbrl.md#0xc8_bbrl">0xc8::bbrl</a>;
<b>use</b> <a href="../bfc-system/bcad.md#0xc8_bcad">0xc8::bcad</a>;
<b>use</b> <a href="../bfc-system/beur.md#0xc8_beur">0xc8::beur</a>;
<b>use</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao">0xc8::bfc_dao</a>;
<b>use</b> <a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager">0xc8::bfc_dao_manager</a>;
<b>use</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner">0xc8::bfc_system_state_inner</a>;
<b>use</b> <a href="../bfc-system/bgbp.md#0xc8_bgbp">0xc8::bgbp</a>;
<b>use</b> <a href="../bfc-system/bidr.md#0xc8_bidr">0xc8::bidr</a>;
<b>use</b> <a href="../bfc-system/binr.md#0xc8_binr">0xc8::binr</a>;
<b>use</b> <a href="../bfc-system/bjpy.md#0xc8_bjpy">0xc8::bjpy</a>;
<b>use</b> <a href="../bfc-system/bkrw.md#0xc8_bkrw">0xc8::bkrw</a>;
<b>use</b> <a href="../bfc-system/bmxn.md#0xc8_bmxn">0xc8::bmxn</a>;
<b>use</b> <a href="../bfc-system/brub.md#0xc8_brub">0xc8::brub</a>;
<b>use</b> <a href="../bfc-system/bsar.md#0xc8_bsar">0xc8::bsar</a>;
<b>use</b> <a href="../bfc-system/btry.md#0xc8_btry">0xc8::btry</a>;
<b>use</b> <a href="../bfc-system/busd.md#0xc8_busd">0xc8::busd</a>;
<b>use</b> <a href="../bfc-system/bzar.md#0xc8_bzar">0xc8::bzar</a>;
<b>use</b> <a href="../bfc-system/mgg.md#0xc8_mgg">0xc8::mgg</a>;
<b>use</b> <a href="../bfc-system/position.md#0xc8_position">0xc8::position</a>;
<b>use</b> <a href="../bfc-system/tick.md#0xc8_tick">0xc8::tick</a>;
<b>use</b> <a href="../bfc-system/treasury.md#0xc8_treasury">0xc8::treasury</a>;
<b>use</b> <a href="../bfc-system/vault.md#0xc8_vault">0xc8::vault</a>;
<b>use</b> <a href="../bfc-system/bfc_dao_voting_pool.md#0xc8_voting_pool">0xc8::voting_pool</a>;
</code></pre>



<a name="0xc8_bfc_system_BfcSystemState"></a>

## Resource `BfcSystemState`



<pre><code><b>struct</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V2"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V2">BFC_SYSTEM_STATE_VERSION_V2</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0xc8_bfc_system_create_stake_manager_key"></a>

## Function `create_stake_manager_key`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create_stake_manager_key">create_stake_manager_key</a>(payment: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create_stake_manager_key">create_stake_manager_key</a>( payment: Coin&lt;BFC&gt;,
                                           ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_stake_manager_key">bfc_system_state_inner::create_stake_manager_key</a>(payment, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_unstake_manager_key"></a>

## Function `unstake_manager_key`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_unstake_manager_key">unstake_manager_key</a>(key: <a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, token: <a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_ManagerKeyBfc">bfc_dao_manager::ManagerKeyBfc</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_unstake_manager_key">unstake_manager_key</a>(key: BFCDaoManageKey,
                                     token: ManagerKeyBfc,
                                     ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_unstake_manager_key">bfc_system_state_inner::unstake_manager_key</a>(key, token, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_allocate_abfc"></a>

## Function `allocate_abfc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_allocate_abfc">allocate_abfc</a>(abfc_balance: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymos_Balance">anonymous_balance::Anonymos_Balance</a>&lt;<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">abfc::ABFC</a>&gt;, admin: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_allocate_abfc">allocate_abfc</a>(
    abfc_balance: Anonymos_Balance&lt;ABFC&gt;,
    admin: <b>address</b>,
    ctx: &<b>mut</b> TxContext
){
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(abfc_balance.into_coin(ctx), admin);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create">create</a>(id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, bfc_balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, usd_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;, jpy_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bjpy.md#0xc8_bjpy_BJPY">bjpy::BJPY</a>&gt;, krw_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bkrw.md#0xc8_bkrw_BKRW">bkrw::BKRW</a>&gt;, aud_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/baud.md#0xc8_baud_BAUD">baud::BAUD</a>&gt;, ars_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bars.md#0xc8_bars_BARS">bars::BARS</a>&gt;, brl_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bbrl.md#0xc8_bbrl_BBRL">bbrl::BBRL</a>&gt;, cad_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bcad.md#0xc8_bcad_BCAD">bcad::BCAD</a>&gt;, eur_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/beur.md#0xc8_beur_BEUR">beur::BEUR</a>&gt;, gbp_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bgbp.md#0xc8_bgbp_BGBP">bgbp::BGBP</a>&gt;, idr_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bidr.md#0xc8_bidr_BIDR">bidr::BIDR</a>&gt;, inr_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/binr.md#0xc8_binr_BINR">binr::BINR</a>&gt;, rub_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/brub.md#0xc8_brub_BRUB">brub::BRUB</a>&gt;, sar_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bsar.md#0xc8_bsar_BSAR">bsar::BSAR</a>&gt;, try_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/btry.md#0xc8_btry_BTRY">btry::BTRY</a>&gt;, zar_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bzar.md#0xc8_bzar_BZAR">bzar::BZAR</a>&gt;, mxn_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bmxn.md#0xc8_bmxn_BMXN">bmxn::BMXN</a>&gt;, mgg_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/mgg.md#0xc8_mgg_MGG">mgg::MGG</a>&gt;, parameters: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>, bfc_skip_init_vault: u32, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create">create</a>(
    id: UID,
    bfc_balance: Balance&lt;BFC&gt;,
    usd_supply: Supply&lt;BUSD&gt;,
    jpy_supply: Supply&lt;BJPY&gt;,
    krw_supply: Supply&lt;BKRW&gt;,
    aud_supply: Supply&lt;BAUD&gt;,
    ars_supply: Supply&lt;BARS&gt;,
    brl_supply: Supply&lt;BBRL&gt;,
    cad_supply: Supply&lt;BCAD&gt;,
    eur_supply: Supply&lt;BEUR&gt;,
    gbp_supply: Supply&lt;BGBP&gt;,
    idr_supply: Supply&lt;BIDR&gt;,
    inr_supply: Supply&lt;BINR&gt;,
    rub_supply: Supply&lt;BRUB&gt;,
    sar_supply: Supply&lt;BSAR&gt;,
    try_supply: Supply&lt;BTRY&gt;,
    zar_supply: Supply&lt;BZAR&gt;,
    mxn_supply: Supply&lt;BMXN&gt;,
    mgg_supply: Supply&lt;MGG&gt;,
    parameters: BfcSystemParameters,
    bfc_skip_init_vault: u32,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_inner_state">bfc_system_state_inner::create_inner_state</a>(
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
    <b>let</b> <b>mut</b> self = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a> {
        id,
        version: <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>
    };

    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>, inner_state);

    <a href="../sui-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_change_round"></a>

## Function `change_round`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_change_round">change_round</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_change_round">change_round</a>( wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_no_ctx">load_system_state_mut_no_ctx</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round">bfc_system_state_inner::update_round</a>(inner_state, round);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_bfc_round"></a>

## Function `bfc_round`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_round">bfc_round</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch_start_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_round">bfc_round</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch_start_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round">bfc_system_state_inner::update_round</a>(inner_state, round);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">bfc_system_state_inner::judge_proposal_state</a>(inner_state, epoch_start_time);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_bfc_round_v2"></a>

## Function `bfc_round_v2`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_round_v2">bfc_round_v2</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch_start_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, stable_type_name_vector: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, stable_rate_vector: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_round_v2">bfc_round_v2</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch_start_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    stable_type_name_vector : <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;,
    stable_rate_vector : <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round_v2">bfc_system_state_inner::update_round_v2</a>(inner_state, round, stable_type_name_vector, stable_rate_vector);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">bfc_system_state_inner::judge_proposal_state</a>(inner_state, epoch_start_time);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_inner_stablecoin_to_bfc"></a>

## Function `inner_stablecoin_to_bfc`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_inner_stablecoin_to_bfc">inner_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(_self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, expect: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_inner_stablecoin_to_bfc">inner_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(
_self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
_balance: Balance&lt;StableCoinType&gt;,
expect: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
_ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <b>let</b> (inner_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(_self, _ctx);
    <b>if</b> (std::type_name::get&lt;StableCoinType&gt;() == std::type_name::get&lt;BUSD&gt;()) {
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance">bfc_system_state_inner::swap_stablecoin_to_bfc_balance</a>(inner_state, <a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(_balance, ctx), expect, ctx)
    } <b>else</b> {
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_balance_to_vault">bfc_system_state_inner::add_balance_to_vault</a>(inner_state, _balance, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_balance">bfc_system_state_inner::withdraw_balance</a>(inner_state, expect)
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_inner_withdraw_balance"></a>

## Function `inner_withdraw_balance`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_inner_withdraw_balance">inner_withdraw_balance</a>(_self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, expect: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_inner_withdraw_balance">inner_withdraw_balance</a>(
    _self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    expect: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    _ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(_self, _ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_balance">bfc_system_state_inner::withdraw_balance</a>(inner_state, expect)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_request_gas_balance"></a>

## Function `request_gas_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_request_gas_balance">request_gas_balance</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_request_gas_balance">request_gas_balance</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_gas_balance">bfc_system_state_inner::request_gas_balance</a>(inner_state, amount, _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_system_state_by_uid"></a>

## Function `load_system_state_by_uid`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_by_uid">load_system_state_by_uid</a>(id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>): &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_by_uid">load_system_state_by_uid</a>(
    id: &UID,
): &BfcSystemStateInnerV2 {
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(id, <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V2">BFC_SYSTEM_STATE_VERSION_V2</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_system_state_mut_by_uid"></a>

## Function `load_system_state_mut_by_uid`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>): &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(
    id: &<b>mut</b> UID,
): &<b>mut</b> BfcSystemStateInnerV2 {
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(id, <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V2">BFC_SYSTEM_STATE_VERSION_V2</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_system_state"></a>

## Function `load_system_state`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(self: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(
    self: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
): &BfcSystemStateInnerV2 {
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(&self.id, <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V2">BFC_SYSTEM_STATE_VERSION_V2</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_bfc_system_state"></a>

## Function `load_bfc_system_state`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_bfc_system_state">load_bfc_system_state</a>(id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>): &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_bfc_system_state">load_bfc_system_state</a>(id: &UID): &BfcSystemStateInner {
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(id, <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_bfc_system_state_mut"></a>

## Function `load_bfc_system_state_mut`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_bfc_system_state_mut">load_bfc_system_state_mut</a>(id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>): &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_bfc_system_state_mut">load_bfc_system_state_mut</a>(id: &<b>mut</b> UID): &<b>mut</b> BfcSystemStateInner {
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(id, <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_system_state_mut_no_ctx"></a>

## Function `load_system_state_mut_no_ctx`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_no_ctx">load_system_state_mut_no_ctx</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_no_ctx">load_system_state_mut_no_ctx</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
): (&<b>mut</b> BfcSystemStateInnerV2) {
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(&<b>mut</b> self.id, <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V2">BFC_SYSTEM_STATE_VERSION_V2</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_system_state_mut"></a>

## Function `load_system_state_mut`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(_self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (&<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(
    _self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    _ctx: &<b>mut</b> TxContext
): (&<b>mut</b> BfcSystemStateInnerV2, &<b>mut</b> TxContext) {
    <b>if</b> (_self.version == 1) {
        <b>let</b> v1: BfcSystemStateInner = <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_remove">dynamic_field::remove</a>(&<b>mut</b> _self.id, _self.version);
        <b>let</b> (<b>mut</b> v2, ctx) = v1.v1_to_v2(_ctx);
        <b>let</b> _ctx = ctx;
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_bfc_system_state_v2">bfc_system_state_inner::init_bfc_system_state_v2</a>(&<b>mut</b> v2, _ctx);

        _self.version = 2;
        <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> _self.id, _self.version, v2);

    };

    <b>let</b> inner: &<b>mut</b> BfcSystemStateInnerV2 = <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(
        &<b>mut</b> _self.id,
         _self.version
    );
    (inner, _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_exchange_rate"></a>

## Function `get_exchange_rate`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_exchange_rate">get_exchange_rate</a>(id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>): <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_exchange_rate">get_exchange_rate</a>(id: &UID): VecMap&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    <b>let</b> inner = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_by_uid">load_system_state_by_uid</a>(id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_rate_map">bfc_system_state_inner::get_rate_map</a>(inner)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_operation_capability"></a>

## Function `get_operation_capability`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_operation_capability">get_operation_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_operation_capability">get_operation_capability</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    ctx: &<b>mut</b> TxContext,
): VecMap&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, VecSet&lt;<b>address</b>&gt;&gt; {
    <b>let</b> (inner, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_operation_capability">bfc_system_state_inner::get_operation_capability</a>(inner)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_operation_capability_by_key"></a>

## Function `get_operation_capability_by_key`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_operation_capability_by_key">get_operation_capability_by_key</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_operation_capability_by_key">get_operation_capability_by_key</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
): VecSet&lt;<b>address</b>&gt; {
    <b>let</b> (inner, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_operation_capability_by_key">bfc_system_state_inner::get_operation_capability_by_key</a>(inner, &std::ascii::string(key))
}
</code></pre>



</details>

<a name="0xc8_bfc_system_add_operation_capability"></a>

## Function `add_operation_capability`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_add_operation_capability">add_operation_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">bfc_system_state_inner::BfcSystemAdminCap</a>, key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_add_operation_capability">add_operation_capability</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    _cap: &BfcSystemAdminCap,
    key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <b>address</b>: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_operation_capability">bfc_system_state_inner::add_operation_capability</a>(inner, std::ascii::string(key), <b>address</b>, _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_remove_operation_capability"></a>

## Function `remove_operation_capability`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_operation_capability">remove_operation_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">bfc_system_state_inner::BfcSystemAdminCap</a>, key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_operation_capability">remove_operation_capability</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    _cap: &BfcSystemAdminCap,
    key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <b>address</b>: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_operation_capability">bfc_system_state_inner::remove_operation_capability</a>(inner, &std::ascii::string(key), <b>address</b>, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_operation_capability"></a>

## Function `set_operation_capability`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_operation_capability">set_operation_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">bfc_system_state_inner::BfcSystemAdminCap</a>, key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, addresses: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_operation_capability">set_operation_capability</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    _cap: &BfcSystemAdminCap,
    key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    addresses: VecSet&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_operation_capability">bfc_system_state_inner::set_operation_capability</a>(inner, std::ascii::string(key), addresses, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_single_operation_capability"></a>

## Function `set_single_operation_capability`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_single_operation_capability">set_single_operation_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">bfc_system_state_inner::BfcSystemAdminCap</a>, key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_single_operation_capability">set_single_operation_capability</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    _cap: &BfcSystemAdminCap,
    key: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <b>address</b>: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_operation_capability">bfc_system_state_inner::set_operation_capability</a>(inner, std::ascii::string(key), <a href="../sui-framework/vec_set.md#0x2_vec_set_singleton">vec_set::singleton</a>(<b>address</b>), ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_oracle_address"></a>

## Function `set_oracle_address`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_oracle_address">set_oracle_address</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_oracle_address">set_oracle_address</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    inner_state.<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_oracle_address">set_oracle_address</a>(<b>address</b>, _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_oracle_address"></a>

## Function `get_oracle_address`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_oracle_address">get_oracle_address</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_oracle_address">get_oracle_address</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, ctx: &<b>mut</b> TxContext): Option&lt;<b>address</b>&gt; {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    inner_state.<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_oracle_address">get_oracle_address</a>(_ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_add_external_stable_gas_coin"></a>

## Function `add_external_stable_gas_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_add_external_stable_gas_coin">add_external_stable_gas_coin</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, value: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_add_external_stable_gas_coin">add_external_stable_gas_coin</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, value: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    inner_state.<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_add_external_stable_gas_coin">add_external_stable_gas_coin</a>(value, _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_delete_external_stable_gas_coin"></a>

## Function `delete_external_stable_gas_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_delete_external_stable_gas_coin">delete_external_stable_gas_coin</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_delete_external_stable_gas_coin">delete_external_stable_gas_coin</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    inner_state.<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_delete_external_stable_gas_coin">delete_external_stable_gas_coin</a>(value, _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_remove_propose"></a>

## Function `remove_propose`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_propose">remove_propose</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_propose">remove_propose</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, key: &BFCDaoManageKey, proposal_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> wrapper.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_proposal">bfc_system_state_inner::remove_proposal</a>(system_state, key, proposal_id);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_remove_action"></a>

## Function `remove_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_action">remove_action</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_action">remove_action</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, key: &BFCDaoManageKey, action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> wrapper.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_action">bfc_system_state_inner::remove_action</a>(system_state, key, action_id);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_destroy_terminated_proposal">destroy_terminated_proposal</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> wrapper.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_destroy_terminated_proposal">bfc_system_state_inner::destroy_terminated_proposal</a>(system_state, manager_key, proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_propose"></a>

## Function `propose`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_propose">propose</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, version_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, payment: &<b>mut</b> <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, action_delay: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_propose">propose</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    version_id : <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    action_delay: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (system_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_propose">bfc_system_state_inner::propose</a>(system_state, version_id, payment, action_id, action_delay, description, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_create_bfcdao_action"></a>

## Function `create_bfcdao_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create_bfcdao_action">create_bfcdao_action</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, payment: &<b>mut</b> <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, actionName: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create_bfcdao_action">create_bfcdao_action</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    actionName: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext) {
    <b>let</b> (system_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfcdao_action">bfc_system_state_inner::create_bfcdao_action</a>(system_state, payment, actionName,<a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_judge_proposal_state">judge_proposal_state</a>(_wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _current_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_judge_proposal_state">judge_proposal_state</a>(_wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, _current_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    //<b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    //<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">bfc_system_state_inner::judge_proposal_state</a>(system_state, current_time);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_judge_proposal_state_with_clock"></a>

## Function `judge_proposal_state_with_clock`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_judge_proposal_state_with_clock">judge_proposal_state_with_clock</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_judge_proposal_state_with_clock">judge_proposal_state_with_clock</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock) {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> wrapper.id);
    <b>let</b> current_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">bfc_system_state_inner::judge_proposal_state</a>(system_state, current_time);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_voting_period"></a>

## Function `set_voting_period`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_voting_period">set_voting_period</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_voting_period">set_voting_period</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> wrapper.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_period">bfc_system_state_inner::set_voting_period</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_voting_quorum_rate"></a>

## Function `set_voting_quorum_rate`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_voting_quorum_rate">set_voting_quorum_rate</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_voting_quorum_rate">set_voting_quorum_rate</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, manager_key: &BFCDaoManageKey, value: u8,){
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> wrapper.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_quorum_rate">bfc_system_state_inner::set_voting_quorum_rate</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_min_action_delay"></a>

## Function `set_min_action_delay`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_min_action_delay">set_min_action_delay</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_min_action_delay">set_min_action_delay</a>(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> wrapper.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_min_action_delay">bfc_system_state_inner::set_min_action_delay</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_withdraw_voting"></a>

## Function `withdraw_voting`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_withdraw_voting">withdraw_voting</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, voting_bfc: <a href="../bfc-system/bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_withdraw_voting">withdraw_voting</a>(   wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
                             voting_bfc: VotingBfc,
                                <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
                             ctx: &<b>mut</b> TxContext) {
    <b>let</b> (system_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_voting">bfc_system_state_inner::withdraw_voting</a>(system_state, voting_bfc,<a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_create_voting_bfc"></a>

## Function `create_voting_bfc`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create_voting_bfc">create_voting_bfc</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../sui-framework/coin.md#0x2_coin">coin</a>: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_create_voting_bfc">create_voting_bfc</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
                             <a href="../sui-framework/coin.md#0x2_coin">coin</a>: Coin&lt;BFC&gt;,
                                <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
                             ctx: &<b>mut</b> TxContext) {
    <b>let</b> (system_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_voting_bfc">bfc_system_state_inner::create_voting_bfc</a>(system_state, <a href="../sui-framework/coin.md#0x2_coin">coin</a>,<a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_rebalance"></a>

## Function `rebalance`

X treasury rebalance


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_rebalance">rebalance</a>(_wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _clock: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_rebalance">rebalance</a>(
    _wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    _clock: &Clock,
    _ctx: &<b>mut</b> TxContext,
) {

}
</code></pre>



</details>

<a name="0xc8_bfc_system_rebalance_with_one_stablecoin"></a>

## Function `rebalance_with_one_stablecoin`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_rebalance_with_one_stablecoin">rebalance_with_one_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_rebalance_with_one_stablecoin">rebalance_with_one_stablecoin</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance_with_one_stablecoin">bfc_system_state_inner::rebalance_with_one_stablecoin</a>&lt;StableCoinType&gt;(inner_state, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_mint_stable_entry"></a>

## Function `mint_stable_entry`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_mint_stable_entry">mint_stable_entry</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_mint_stable_entry">mint_stable_entry</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    cap: &BfcSystemModifyCap,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <b>let</b> <a href="../sui-framework/coin.md#0x2_coin">coin</a> = <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_mint_stable">bfc_system_state_inner::mint_stable</a>&lt;StableCoinType&gt;(inner_state, amount, &cap.get_bfc_system_modify_cap_key(), _ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../sui-framework/coin.md#0x2_coin">coin</a>, <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0xc8_bfc_system_mint_stable_entry_to_address"></a>

## Function `mint_stable_entry_to_address`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_mint_stable_entry_to_address">mint_stable_entry_to_address</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_mint_stable_entry_to_address">mint_stable_entry_to_address</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    cap: &BfcSystemModifyCap,
    <b>address</b>: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <b>let</b> <a href="../sui-framework/coin.md#0x2_coin">coin</a> = <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_mint_stable">bfc_system_state_inner::mint_stable</a>&lt;StableCoinType&gt;(inner_state, amount, &cap.get_bfc_system_modify_cap_key(), _ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../sui-framework/coin.md#0x2_coin">coin</a>, <b>address</b>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_mint_stable"></a>

## Function `mint_stable`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_mint_stable">mint_stable</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_mint_stable">mint_stable</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    cap: &BfcSystemModifyCap,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;StableCoinType&gt; {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_mint_stable">bfc_system_state_inner::mint_stable</a>&lt;StableCoinType&gt;(inner_state, amount, &cap.get_bfc_system_modify_cap_key(), _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_verify_capability"></a>

## Function `verify_capability`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_verify_capability">verify_capability</a>(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_verify_capability">verify_capability</a>(
    wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    cap: &BfcSystemModifyCap,
    ctx: &TxContext,
): bool {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    inner_state.verify_operation_capability(&cap.get_bfc_system_modify_cap_key(), ctx.sender())
}
</code></pre>



</details>

<a name="0xc8_bfc_system_burn_stable"></a>

## Function `burn_stable`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_burn_stable">burn_stable</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, token: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_burn_stable">burn_stable</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    token: Coin&lt;StableCoinType&gt;,
    ctx: &<b>mut</b> TxContext,
){
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_burn_stable">bfc_system_state_inner::burn_stable</a>&lt;StableCoinType&gt;(inner_state, token);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_init_admin_capability"></a>

## Function `init_admin_capability`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_init_admin_capability">init_admin_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, addresses: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_init_admin_capability">init_admin_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, addresses: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_bfc_system_admins">bfc_system_state_inner::init_bfc_system_admins</a>(inner_state, _ctx, addresses);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_add_admin_capability"></a>

## Function `add_admin_capability`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_add_admin_capability">add_admin_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, addresses: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">bfc_system_state_inner::BfcSystemAdminCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_add_admin_capability">add_admin_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, addresses: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, cap: &BfcSystemAdminCap, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <b>let</b> _ =  cap;
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">bfc_system_state_inner::verify_admin_capability</a>(inner_state, _ctx.sender());
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_bfc_system_admin_cap">bfc_system_state_inner::add_bfc_system_admin_cap</a>(inner_state, _ctx, addresses);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_remove_admin_capability"></a>

## Function `remove_admin_capability`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_admin_capability">remove_admin_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <b>address</b>: <b>address</b>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">bfc_system_state_inner::BfcSystemAdminCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_remove_admin_capability">remove_admin_capability</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, <b>address</b>: <b>address</b>, cap: &BfcSystemAdminCap, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (inner_state, _ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <b>let</b> _ =  cap;
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_bfc_system_admin_cap">bfc_system_state_inner::remove_bfc_system_admin_cap</a>(inner_state, <b>address</b>, _ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_swap_bfc_to_stablecoin"></a>

## Function `swap_bfc_to_stablecoin`

X treasury  swap bfc to stablecoin


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, native_coin: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    native_coin: Coin&lt;BFC&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(std::type_name::get&lt;StableCoinType&gt;() == std::type_name::get&lt;BUSD&gt;(), 0);
    <b>let</b> (system_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin">bfc_system_state_inner::swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(system_state, native_coin, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, amount, min_amount, deadline, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_swap_stablecoin_to_bfc"></a>

## Function `swap_stablecoin_to_bfc`

X treasury  swap stablecoin to bfc


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, stable_coin: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    stable_coin: Coin&lt;StableCoinType&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(std::type_name::get&lt;StableCoinType&gt;() == std::type_name::get&lt;BUSD&gt;(), 0);
    <b>let</b> (system_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc">bfc_system_state_inner::swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(system_state, stable_coin, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, amount, min_amount, deadline, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_stablecoin_by_bfc"></a>

## Function `get_stablecoin_by_bfc`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(
    wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
{
    <b>assert</b>!(std::type_name::get&lt;StableCoinType&gt;() == std::type_name::get&lt;BUSD&gt;(), 0);
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">bfc_system_state_inner::get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(system_state, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_bfc_by_stablecoin"></a>

## Function `get_bfc_by_stablecoin`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(
    wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
{
    <b>assert</b>!(std::type_name::get&lt;StableCoinType&gt;() == std::type_name::get&lt;BUSD&gt;(), 0);
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin">bfc_system_state_inner::get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(system_state, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_info"></a>

## Function `vault_info`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_info">vault_info</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../bfc-system/vault.md#0xc8_vault_VaultInfo">vault::VaultInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_info">vault_info</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): VaultInfo {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info_v2">bfc_system_state_inner::vault_info_v2</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_ticks"></a>

## Function `vault_ticks`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_ticks">vault_ticks</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_ticks">vault_ticks</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Tick&gt; {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_ticks_v2">bfc_system_state_inner::vault_ticks_v2</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_positions"></a>

## Function `vault_positions`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_positions">vault_positions</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/position.md#0xc8_position_Position">position::Position</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_positions">vault_positions</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Position&gt; {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_positions_v2">bfc_system_state_inner::vault_positions_v2</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_total_supply"></a>

## Function `total_supply`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_total_supply">total_supply</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_total_supply">total_supply</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_total_supply_v2">bfc_system_state_inner::get_total_supply_v2</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_bfc_exchange_rate"></a>

## Function `get_bfc_exchange_rate`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_bfc_exchange_rate">get_bfc_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_bfc_exchange_rate">get_bfc_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
{
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_exchange_rate">bfc_system_state_inner::get_bfc_exchange_rate</a>&lt;StableCoinType&gt;(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_stablecoin_exchange_rate"></a>

## Function `get_stablecoin_exchange_rate`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_stablecoin_exchange_rate">get_stablecoin_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_get_stablecoin_exchange_rate">get_stablecoin_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
{
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_exchange_rate">bfc_system_state_inner::get_stablecoin_exchange_rate</a>&lt;StableCoinType&gt;(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_bfc_required"></a>

## Function `bfc_required`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_required">bfc_required</a>(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_required">bfc_required</a>(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_required">bfc_system_state_inner::bfc_required</a>(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_next_epoch_bfc_required"></a>

## Function `next_epoch_bfc_required`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_next_epoch_bfc_required">next_epoch_bfc_required</a>(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_next_epoch_bfc_required">next_epoch_bfc_required</a>(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_next_epoch_bfc_required_v2">bfc_system_state_inner::next_epoch_bfc_required_v2</a>(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_bfc_required_with_one_stablecoin"></a>

## Function `bfc_required_with_one_stablecoin`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_required_with_one_stablecoin">bfc_required_with_one_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_bfc_required_with_one_stablecoin">bfc_required_with_one_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_required_with_one_stablecoin">bfc_system_state_inner::bfc_required_with_one_stablecoin</a>&lt;StableCoinType&gt;(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_treasury_balance"></a>

## Function `treasury_balance`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_treasury_balance">treasury_balance</a>(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_treasury_balance">treasury_balance</a>(wrapper: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> system_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance_v2">bfc_system_state_inner::treasury_balance_v2</a>(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_deposit_to_treasury"></a>

## Function `deposit_to_treasury`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>: Coin&lt;BFC&gt;) {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> self.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury">bfc_system_state_inner::deposit_to_treasury</a>(inner_state, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_deposit_to_treasury_inner"></a>

## Function `deposit_to_treasury_inner`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury_inner">deposit_to_treasury_inner</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, bfc_balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury_inner">deposit_to_treasury_inner</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, bfc_balance: Balance&lt;BFC&gt;, ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self, ctx);
    <b>let</b> <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>= <a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(bfc_balance, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury">bfc_system_state_inner::deposit_to_treasury</a>(inner_state, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_deposit_to_treasury_pool"></a>

## Function `deposit_to_treasury_pool`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>: Coin&lt;BFC&gt;) {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> self.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury_pool">bfc_system_state_inner::deposit_to_treasury_pool</a>(inner_state, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_deposit_to_treasury_pool_no_entry"></a>

## Function `deposit_to_treasury_pool_no_entry`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool_no_entry">deposit_to_treasury_pool_no_entry</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, bfc_balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>  <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool_no_entry">deposit_to_treasury_pool_no_entry</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, bfc_balance: Balance&lt;BFC&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> (inner_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self, ctx);
    <b>let</b> <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>= <a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(bfc_balance, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury_pool">bfc_system_state_inner::deposit_to_treasury_pool</a>(inner_state, <a href="../sui-framework/bfc.md#0x2_bfc">bfc</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_set_pause"></a>

## Function `vault_set_pause`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(cap: &<a href="../bfc-system/treasury.md#0xc8_treasury_TreasuryPauseCap">treasury::TreasuryPauseCap</a>, wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, pause: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(
    cap: &TreasuryPauseCap,
    wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    pause: bool
) {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_no_ctx">load_system_state_mut_no_ctx</a>(wrapper);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_set_pause_v2">bfc_system_state_inner::vault_set_pause_v2</a>&lt;StableCoinType&gt;(cap, inner_state, pause)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_voting_delay"></a>

## Function `set_voting_delay`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_set_voting_delay">set_voting_delay</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> self.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_delay">bfc_system_state_inner::set_voting_delay</a>(inner_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_cast_vote"></a>

## Function `cast_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_cast_vote">cast_vote</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/coin.md#0x2_coin">coin</a>: <a href="../bfc-system/bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, agreeInt: u8, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_cast_vote">cast_vote</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    proposal: &<b>mut</b> Proposal,
    <a href="../sui-framework/coin.md#0x2_coin">coin</a>: VotingBfc,
    agreeInt: u8,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
)  {
    <b>let</b> (inner_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_cast_vote">bfc_system_state_inner::cast_vote</a>(inner_state, proposal, <a href="../sui-framework/coin.md#0x2_coin">coin</a>, agreeInt, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_change_vote"></a>

## Function `change_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_change_vote">change_vote</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, my_vote: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, agree: bool, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_change_vote">change_vote</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    my_vote: &<b>mut</b> Vote,
    proposal: &<b>mut</b> Proposal,
    agree: bool,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_change_vote">bfc_system_state_inner::change_vote</a>(inner_state, my_vote, proposal, agree, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_queue_proposal_action"></a>

## Function `queue_proposal_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_queue_proposal_action">queue_proposal_action</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_queue_proposal_action">queue_proposal_action</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <b>let</b> inner_state = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut_by_uid">load_system_state_mut_by_uid</a>(&<b>mut</b> self.id);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_queue_proposal_action">bfc_system_state_inner::queue_proposal_action</a>(inner_state, manager_key, proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_revoke_vote"></a>

## Function `revoke_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_revoke_vote">revoke_vote</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, my_vote: <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, <a href="../../voting_power.md#0x3_voting_power">voting_power</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_revoke_vote">revoke_vote</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    proposal: &<b>mut</b> Proposal,
    my_vote:  Vote,
    <a href="../../voting_power.md#0x3_voting_power">voting_power</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (inner_state, ctx) = <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self, ctx);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_revoke_vote">bfc_system_state_inner::revoke_vote</a>(inner_state, proposal, my_vote, <a href="../../voting_power.md#0x3_voting_power">voting_power</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_unvote_votes"></a>

## Function `unvote_votes`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_unvote_votes">unvote_votes</a>(proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, vote: <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_unvote_votes">unvote_votes</a>(
    proposal: &<b>mut</b> Proposal,
    vote: Vote,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_unvote_votes">bfc_dao::unvote_votes</a>(proposal, vote, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vote_of"></a>

## Function `vote_of`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vote_of">vote_of</a>(vote: &<a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_vote_of">vote_of</a>(
    vote: &Vote,
    proposal: &Proposal,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_vote_of">bfc_dao::vote_of</a>(vote, proposal, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_has_vote"></a>

## Function `has_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_has_vote">has_vote</a>(vote: &<a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_has_vote">has_vote</a>(
    vote: &Vote,
    proposal: &Proposal,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_has_vote">bfc_dao::has_vote</a>(vote, proposal);
}
</code></pre>



</details>


<a name="0xc8_bfc_system"></a>

# Module `0xc8::bfc_system`



-  [Resource `BfcSystemState`](#0xc8_bfc_system_BfcSystemState)
-  [Constants](#@Constants_0)
-  [Function `create_stake_manager_key`](#0xc8_bfc_system_create_stake_manager_key)
-  [Function `unstake_manager_key`](#0xc8_bfc_system_unstake_manager_key)
-  [Function `create`](#0xc8_bfc_system_create)
-  [Function `bfc_round`](#0xc8_bfc_system_bfc_round)
-  [Function `inner_stablecoin_to_bfc`](#0xc8_bfc_system_inner_stablecoin_to_bfc)
-  [Function `load_system_state`](#0xc8_bfc_system_load_system_state)
-  [Function `load_bfc_system_state`](#0xc8_bfc_system_load_bfc_system_state)
-  [Function `load_bfc_system_state_mut`](#0xc8_bfc_system_load_bfc_system_state_mut)
-  [Function `load_system_state_mut`](#0xc8_bfc_system_load_system_state_mut)
-  [Function `get_exchange_rate`](#0xc8_bfc_system_get_exchange_rate)
-  [Function `destroy_terminated_proposal`](#0xc8_bfc_system_destroy_terminated_proposal)
-  [Function `propose`](#0xc8_bfc_system_propose)
-  [Function `remove_action`](#0xc8_bfc_system_remove_action)
-  [Function `create_bfcdao_action`](#0xc8_bfc_system_create_bfcdao_action)
-  [Function `judge_proposal_state`](#0xc8_bfc_system_judge_proposal_state)
-  [Function `set_voting_period`](#0xc8_bfc_system_set_voting_period)
-  [Function `set_voting_quorum_rate`](#0xc8_bfc_system_set_voting_quorum_rate)
-  [Function `set_min_action_delay`](#0xc8_bfc_system_set_min_action_delay)
-  [Function `withdraw_voting`](#0xc8_bfc_system_withdraw_voting)
-  [Function `create_voting_bfc`](#0xc8_bfc_system_create_voting_bfc)
-  [Function `rebalance`](#0xc8_bfc_system_rebalance)
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
-  [Function `treasury_balance`](#0xc8_bfc_system_treasury_balance)
-  [Function `deposit_to_treasury`](#0xc8_bfc_system_deposit_to_treasury)
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
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="bars.md#0xc8_bars">0xc8::bars</a>;
<b>use</b> <a href="baud.md#0xc8_baud">0xc8::baud</a>;
<b>use</b> <a href="bbrl.md#0xc8_bbrl">0xc8::bbrl</a>;
<b>use</b> <a href="bcad.md#0xc8_bcad">0xc8::bcad</a>;
<b>use</b> <a href="beur.md#0xc8_beur">0xc8::beur</a>;
<b>use</b> <a href="bfc_dao.md#0xc8_bfc_dao">0xc8::bfc_dao</a>;
<b>use</b> <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager">0xc8::bfc_dao_manager</a>;
<b>use</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner">0xc8::bfc_system_state_inner</a>;
<b>use</b> <a href="bgbp.md#0xc8_bgbp">0xc8::bgbp</a>;
<b>use</b> <a href="bidr.md#0xc8_bidr">0xc8::bidr</a>;
<b>use</b> <a href="binr.md#0xc8_binr">0xc8::binr</a>;
<b>use</b> <a href="bjpy.md#0xc8_bjpy">0xc8::bjpy</a>;
<b>use</b> <a href="bkrw.md#0xc8_bkrw">0xc8::bkrw</a>;
<b>use</b> <a href="bmxn.md#0xc8_bmxn">0xc8::bmxn</a>;
<b>use</b> <a href="brub.md#0xc8_brub">0xc8::brub</a>;
<b>use</b> <a href="bsar.md#0xc8_bsar">0xc8::bsar</a>;
<b>use</b> <a href="btry.md#0xc8_btry">0xc8::btry</a>;
<b>use</b> <a href="busd.md#0xc8_busd">0xc8::busd</a>;
<b>use</b> <a href="bzar.md#0xc8_bzar">0xc8::bzar</a>;
<b>use</b> <a href="mgg.md#0xc8_mgg">0xc8::mgg</a>;
<b>use</b> <a href="position.md#0xc8_position">0xc8::position</a>;
<b>use</b> <a href="tick.md#0xc8_tick">0xc8::tick</a>;
<b>use</b> <a href="treasury.md#0xc8_treasury">0xc8::treasury</a>;
<b>use</b> <a href="vault.md#0xc8_vault">0xc8::vault</a>;
<b>use</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool">0xc8::voting_pool</a>;
</code></pre>



<a name="0xc8_bfc_system_BfcSystemState"></a>

## Resource `BfcSystemState`



<pre><code><b>struct</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1"></a>



<pre><code><b>const</b> <a href="bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>: u64 = 1;
</code></pre>



<a name="0xc8_bfc_system_create_stake_manager_key"></a>

## Function `create_stake_manager_key`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create_stake_manager_key">create_stake_manager_key</a>(payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create_stake_manager_key">create_stake_manager_key</a>( payment: Coin&lt;BFC&gt;,
                                           ctx: &<b>mut</b> TxContext) {
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_stake_manager_key">bfc_system_state_inner::create_stake_manager_key</a>(payment, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_unstake_manager_key"></a>

## Function `unstake_manager_key`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_unstake_manager_key">unstake_manager_key</a>(key: <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, token: <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_ManagerKeyBfc">bfc_dao_manager::ManagerKeyBfc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_unstake_manager_key">unstake_manager_key</a>(key: BFCDaoManageKey,
                                     token: ManagerKeyBfc,
                                     ctx: &<b>mut</b> TxContext) {
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_unstake_manager_key">bfc_system_state_inner::unstake_manager_key</a>(key, token, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create">create</a>(id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>, bfc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, usd_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;, jpy_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bjpy.md#0xc8_bjpy_BJPY">bjpy::BJPY</a>&gt;, krw_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bkrw.md#0xc8_bkrw_BKRW">bkrw::BKRW</a>&gt;, aud_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="baud.md#0xc8_baud_BAUD">baud::BAUD</a>&gt;, ars_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bars.md#0xc8_bars_BARS">bars::BARS</a>&gt;, brl_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bbrl.md#0xc8_bbrl_BBRL">bbrl::BBRL</a>&gt;, cad_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bcad.md#0xc8_bcad_BCAD">bcad::BCAD</a>&gt;, eur_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="beur.md#0xc8_beur_BEUR">beur::BEUR</a>&gt;, gbp_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bgbp.md#0xc8_bgbp_BGBP">bgbp::BGBP</a>&gt;, idr_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bidr.md#0xc8_bidr_BIDR">bidr::BIDR</a>&gt;, inr_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="binr.md#0xc8_binr_BINR">binr::BINR</a>&gt;, rub_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="brub.md#0xc8_brub_BRUB">brub::BRUB</a>&gt;, sar_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bsar.md#0xc8_bsar_BSAR">bsar::BSAR</a>&gt;, try_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="btry.md#0xc8_btry_BTRY">btry::BTRY</a>&gt;, zar_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bzar.md#0xc8_bzar_BZAR">bzar::BZAR</a>&gt;, mxn_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bmxn.md#0xc8_bmxn_BMXN">bmxn::BMXN</a>&gt;, mgg_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="mgg.md#0xc8_mgg_MGG">mgg::MGG</a>&gt;, parameters: <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create">create</a>(
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
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner_state = <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_inner_state">bfc_system_state_inner::create_inner_state</a>(
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
    <b>let</b> self = <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a> {
        id,
        version: <a href="bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>
    };

    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, <a href="bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>, inner_state);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_bfc_round"></a>

## Function `bfc_round`



<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_bfc_round">bfc_round</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, round: u64, epoch_start_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_bfc_round">bfc_round</a>(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    round: u64,
    epoch_start_time: u64,
) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round">bfc_system_state_inner::update_round</a>(inner_state, round);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">bfc_system_state_inner::judge_proposal_state</a>(inner_state, epoch_start_time);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_inner_stablecoin_to_bfc"></a>

## Function `inner_stablecoin_to_bfc`



<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_inner_stablecoin_to_bfc">inner_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(_self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, _balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, expect: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_inner_stablecoin_to_bfc">inner_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(
    _self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    _balance: Balance&lt;StableCoinType&gt;,
    expect: u64,
    _ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt;
{
    // wouldn't <b>return</b> remain <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>&lt;StableCoinType&gt; <b>to</b> system
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(_self);
    <b>let</b> bfc_balance = <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance">bfc_system_state_inner::swap_stablecoin_to_bfc_balance</a>(inner_state, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(_balance, _ctx), expect,_ctx);
    bfc_balance
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_system_state"></a>

## Function `load_system_state`



<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(self: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(
    self: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
): &BfcSystemStateInner {
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(&self.id, self.version)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_bfc_system_state"></a>

## Function `load_bfc_system_state`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_bfc_system_state">load_bfc_system_state</a>(id: &<a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>): &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_bfc_system_state">load_bfc_system_state</a>(id: &UID): &BfcSystemStateInner {
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(id, <a href="bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_bfc_system_state_mut"></a>

## Function `load_bfc_system_state_mut`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_bfc_system_state_mut">load_bfc_system_state_mut</a>(id: &<b>mut</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>): &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_bfc_system_state_mut">load_bfc_system_state_mut</a>(id: &<b>mut</b> UID): &<b>mut</b> BfcSystemStateInner {
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(id, <a href="bfc_system.md#0xc8_bfc_system_BFC_SYSTEM_STATE_VERSION_V1">BFC_SYSTEM_STATE_VERSION_V1</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_load_system_state_mut"></a>

## Function `load_system_state_mut`



<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(
    self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>
): &<b>mut</b> BfcSystemStateInner {
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(&<b>mut</b> self.id, self.version)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_exchange_rate"></a>

## Function `get_exchange_rate`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_exchange_rate">get_exchange_rate</a>(id: &<a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>): <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_exchange_rate">get_exchange_rate</a>(id: &UID): VecMap&lt;<a href="_String">ascii::String</a>, u64&gt; {
    <b>let</b> inner = <a href="bfc_system.md#0xc8_bfc_system_load_bfc_system_state">load_bfc_system_state</a>(id);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_rate_map">bfc_system_state_inner::get_rate_map</a>(inner)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_destroy_terminated_proposal">destroy_terminated_proposal</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_destroy_terminated_proposal">bfc_system_state_inner::destroy_terminated_proposal</a>(system_state, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_propose"></a>

## Function `propose`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_propose">propose</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, version_id: u64, payment: &<b>mut</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, action_id: u64, action_delay: u64, description: <a href="">vector</a>&lt;u8&gt;, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_propose">propose</a>(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    version_id : u64,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    action_id: u64,
    action_delay: u64,
    description: <a href="">vector</a>&lt;u8&gt;,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_propose">bfc_system_state_inner::propose</a>(system_state, version_id, payment, action_id, action_delay, description, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_remove_action"></a>

## Function `remove_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_remove_action">remove_action</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, action_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_remove_action">remove_action</a>( wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,key: &BFCDaoManageKey,action_id: u64){
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_action">bfc_system_state_inner::remove_action</a>(system_state,key,action_id);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_create_bfcdao_action"></a>

## Function `create_bfcdao_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create_bfcdao_action">create_bfcdao_action</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, payment: &<b>mut</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, actionName: <a href="">vector</a>&lt;u8&gt;, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create_bfcdao_action">create_bfcdao_action</a>(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    actionName: <a href="">vector</a>&lt;u8&gt;,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfcdao_action">bfc_system_state_inner::create_bfcdao_action</a>(system_state, payment, actionName,<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, current_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, current_time: u64) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">bfc_system_state_inner::judge_proposal_state</a>(system_state, current_time);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_voting_period"></a>

## Function `set_voting_period`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_voting_period">set_voting_period</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_voting_period">set_voting_period</a>(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    value: u64,
) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_period">bfc_system_state_inner::set_voting_period</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_voting_quorum_rate"></a>

## Function `set_voting_quorum_rate`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_voting_quorum_rate">set_voting_quorum_rate</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_voting_quorum_rate">set_voting_quorum_rate</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, manager_key: &BFCDaoManageKey, value: u8,){
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_quorum_rate">bfc_system_state_inner::set_voting_quorum_rate</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_min_action_delay"></a>

## Function `set_min_action_delay`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_min_action_delay">set_min_action_delay</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_min_action_delay">set_min_action_delay</a>(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    value: u64,
) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_min_action_delay">bfc_system_state_inner::set_min_action_delay</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_withdraw_voting"></a>

## Function `withdraw_voting`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_withdraw_voting">withdraw_voting</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_withdraw_voting">withdraw_voting</a>(   wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
                             voting_bfc: VotingBfc,
                                <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
                             ctx: &<b>mut</b> TxContext) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_voting">bfc_system_state_inner::withdraw_voting</a>(system_state, voting_bfc,<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_create_voting_bfc"></a>

## Function `create_voting_bfc`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create_voting_bfc">create_voting_bfc</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_create_voting_bfc">create_voting_bfc</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
                             <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;BFC&gt;,
                                <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
                             ctx: &<b>mut</b> TxContext) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_voting_bfc">bfc_system_state_inner::create_voting_bfc</a>(system_state, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>,<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_rebalance"></a>

## Function `rebalance`

X treasury rebalance


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_rebalance">rebalance</a>(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_rebalance">rebalance</a>(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance">bfc_system_state_inner::rebalance</a>(inner_state, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_swap_bfc_to_stablecoin"></a>

## Function `swap_bfc_to_stablecoin`

X treasury  swap bfc to stablecoin


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, native_coin: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, amount: u64, min_amount: u64, deadline: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    native_coin: Coin&lt;BFC&gt;,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    amount: u64,
    min_amount: u64,
    deadline: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin">bfc_system_state_inner::swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(system_state, native_coin, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, amount, min_amount, deadline, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_swap_stablecoin_to_bfc"></a>

## Function `swap_stablecoin_to_bfc`

X treasury  swap stablecoin to bfc


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, stable_coin: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, amount: u64, min_amount: u64, deadline: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    stable_coin: Coin&lt;StableCoinType&gt;,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    amount: u64,
    min_amount: u64,
    deadline: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc">bfc_system_state_inner::swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(system_state, stable_coin, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, amount, min_amount, deadline, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_stablecoin_by_bfc"></a>

## Function `get_stablecoin_by_bfc`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: u64): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(
    wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: u64,
): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
{
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">bfc_system_state_inner::get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(system_state, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_bfc_by_stablecoin"></a>

## Function `get_bfc_by_stablecoin`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, amount: u64): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(
    wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    amount: u64,
): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
{
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin">bfc_system_state_inner::get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(system_state, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_info"></a>

## Function `vault_info`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_info">vault_info</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="vault.md#0xc8_vault_VaultInfo">vault::VaultInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_info">vault_info</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): VaultInfo {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info">bfc_system_state_inner::vault_info</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_ticks"></a>

## Function `vault_ticks`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_ticks">vault_ticks</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="">vector</a>&lt;<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_ticks">vault_ticks</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="">vector</a>&lt;Tick&gt; {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_ticks">bfc_system_state_inner::vault_ticks</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_positions"></a>

## Function `vault_positions`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_positions">vault_positions</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): <a href="">vector</a>&lt;<a href="position.md#0xc8_position_Position">position::Position</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_positions">vault_positions</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): <a href="">vector</a>&lt;Position&gt; {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_positions">bfc_system_state_inner::vault_positions</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_total_supply"></a>

## Function `total_supply`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_total_supply">total_supply</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_total_supply">total_supply</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): u64 {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_total_supply">bfc_system_state_inner::get_total_supply</a>&lt;StableCoinType&gt;(inner_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_bfc_exchange_rate"></a>

## Function `get_bfc_exchange_rate`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_bfc_exchange_rate">get_bfc_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_bfc_exchange_rate">get_bfc_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): u64
{
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_exchange_rate">bfc_system_state_inner::get_bfc_exchange_rate</a>&lt;StableCoinType&gt;(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_get_stablecoin_exchange_rate"></a>

## Function `get_stablecoin_exchange_rate`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_stablecoin_exchange_rate">get_stablecoin_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_get_stablecoin_exchange_rate">get_stablecoin_exchange_rate</a>&lt;StableCoinType&gt;(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): u64
{
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_exchange_rate">bfc_system_state_inner::get_stablecoin_exchange_rate</a>&lt;StableCoinType&gt;(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_bfc_required"></a>

## Function `bfc_required`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_bfc_required">bfc_required</a>(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_bfc_required">bfc_required</a>(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): u64 {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_required">bfc_system_state_inner::bfc_required</a>(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_treasury_balance"></a>

## Function `treasury_balance`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_treasury_balance">treasury_balance</a>(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_treasury_balance">treasury_balance</a>(wrapper: &<a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>): u64 {
    <b>let</b> system_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance">bfc_system_state_inner::treasury_balance</a>(system_state)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_deposit_to_treasury"></a>

## Function `deposit_to_treasury`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>: Coin&lt;BFC&gt;) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury">bfc_system_state_inner::deposit_to_treasury</a>(inner_state, <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_deposit_to_treasury_pool"></a>

## Function `deposit_to_treasury_pool`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>: Coin&lt;BFC&gt;) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury_pool">bfc_system_state_inner::deposit_to_treasury_pool</a>(inner_state, <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_deposit_to_treasury_pool_no_entry"></a>

## Function `deposit_to_treasury_pool_no_entry`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool_no_entry">deposit_to_treasury_pool_no_entry</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, bfc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_deposit_to_treasury_pool_no_entry">deposit_to_treasury_pool_no_entry</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>, bfc_balance: Balance&lt;BFC&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <b>let</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>= <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(bfc_balance, ctx);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury_pool">bfc_system_state_inner::deposit_to_treasury_pool</a>(inner_state, <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vault_set_pause"></a>

## Function `vault_set_pause`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(cap: &<a href="treasury.md#0xc8_treasury_TreasuryPauseCap">treasury::TreasuryPauseCap</a>, wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, pause: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(
    cap: &TreasuryPauseCap,
    wrapper: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    pause: bool
) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_set_pause">bfc_system_state_inner::vault_set_pause</a>&lt;StableCoinType&gt;(cap, inner_state, pause)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_set_voting_delay"></a>

## Function `set_voting_delay`

DAO


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_set_voting_delay">set_voting_delay</a>(
    self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    value: u64,
) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_delay">bfc_system_state_inner::set_voting_delay</a>(inner_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_cast_vote"></a>

## Function `cast_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_cast_vote">cast_vote</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, agreeInt: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_cast_vote">cast_vote</a>(
    self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: VotingBfc,
    agreeInt: u8,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
)  {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_cast_vote">bfc_system_state_inner::cast_vote</a>(inner_state, proposal, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, agreeInt, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_change_vote"></a>

## Function `change_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_change_vote">change_vote</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, my_vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, agree: bool, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_change_vote">change_vote</a>(
    self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    my_vote: &<b>mut</b> Vote,
    proposal: &<b>mut</b> Proposal,
    agree: bool,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_change_vote">bfc_system_state_inner::change_vote</a>(inner_state, my_vote, proposal, agree, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_queue_proposal_action"></a>

## Function `queue_proposal_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_queue_proposal_action">queue_proposal_action</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_queue_proposal_action">queue_proposal_action</a>(
    self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_queue_proposal_action">bfc_system_state_inner::queue_proposal_action</a>(inner_state, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_revoke_vote"></a>

## Function `revoke_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_revoke_vote">revoke_vote</a>(self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, my_vote: <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, voting_power: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_revoke_vote">revoke_vote</a>(
    self: &<b>mut</b> <a href="bfc_system.md#0xc8_bfc_system_BfcSystemState">BfcSystemState</a>,
    proposal: &<b>mut</b> Proposal,
    my_vote:  Vote,
    voting_power: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="bfc_system.md#0xc8_bfc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_revoke_vote">bfc_system_state_inner::revoke_vote</a>(inner_state, proposal, my_vote, voting_power, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_unvote_votes"></a>

## Function `unvote_votes`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_unvote_votes">unvote_votes</a>(proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, vote: <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_unvote_votes">unvote_votes</a>(
    proposal: &<b>mut</b> Proposal,
    vote: Vote,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_unvote_votes">bfc_dao::unvote_votes</a>(proposal, vote, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_vote_of"></a>

## Function `vote_of`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vote_of">vote_of</a>(vote: &<a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_vote_of">vote_of</a>(
    vote: &Vote,
    proposal: &Proposal,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_vote_of">bfc_dao::vote_of</a>(vote, proposal, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_has_vote"></a>

## Function `has_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_has_vote">has_vote</a>(vote: &<a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="bfc_system.md#0xc8_bfc_system_has_vote">has_vote</a>(
    vote: &Vote,
    proposal: &Proposal,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_has_vote">bfc_dao::has_vote</a>(vote, proposal);
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

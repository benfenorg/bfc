
<a name="0xc8_obc_system"></a>

# Module `0xc8::obc_system`



-  [Resource `ObcSystemState`](#0xc8_obc_system_ObcSystemState)
-  [Constants](#@Constants_0)
-  [Function `create`](#0xc8_obc_system_create)
-  [Function `obc_round`](#0xc8_obc_system_obc_round)
-  [Function `update_round`](#0xc8_obc_system_update_round)
-  [Function `load_system_state`](#0xc8_obc_system_load_system_state)
-  [Function `load_system_state_mut`](#0xc8_obc_system_load_system_state_mut)
-  [Function `request_get_exchange_rate`](#0xc8_obc_system_request_get_exchange_rate)
-  [Function `request_add_gas_coin`](#0xc8_obc_system_request_add_gas_coin)
-  [Function `request_update_gas_coin`](#0xc8_obc_system_request_update_gas_coin)
-  [Function `request_remove_gas_coin`](#0xc8_obc_system_request_remove_gas_coin)
-  [Function `request_exchange_stable`](#0xc8_obc_system_request_exchange_stable)
-  [Function `request_exchange_all`](#0xc8_obc_system_request_exchange_all)
-  [Function `request_withdraw_stable`](#0xc8_obc_system_request_withdraw_stable)
-  [Function `request_withdraw_stable_no_entry`](#0xc8_obc_system_request_withdraw_stable_no_entry)
-  [Function `init_exchange_pool`](#0xc8_obc_system_init_exchange_pool)
-  [Function `obc_system_stat_parameter`](#0xc8_obc_system_obc_system_stat_parameter)
-  [Function `destroy_terminated_proposal`](#0xc8_obc_system_destroy_terminated_proposal)
-  [Function `propose`](#0xc8_obc_system_propose)
-  [Function `create_obcdao_action`](#0xc8_obc_system_create_obcdao_action)
-  [Function `judge_proposal_state`](#0xc8_obc_system_judge_proposal_state)
-  [Function `set_voting_period`](#0xc8_obc_system_set_voting_period)
-  [Function `modify_proposal`](#0xc8_obc_system_modify_proposal)
-  [Function `set_voting_quorum_rate`](#0xc8_obc_system_set_voting_quorum_rate)
-  [Function `set_min_action_delay`](#0xc8_obc_system_set_min_action_delay)
-  [Function `withdraw_voting`](#0xc8_obc_system_withdraw_voting)
-  [Function `create_voting_obc`](#0xc8_obc_system_create_voting_obc)
-  [Function `swap_obc_to_stablecoin`](#0xc8_obc_system_swap_obc_to_stablecoin)
-  [Function `swap_stablecoin_to_obc`](#0xc8_obc_system_swap_stablecoin_to_obc)
-  [Function `get_stablecoin_by_obc`](#0xc8_obc_system_get_stablecoin_by_obc)
-  [Function `get_obc_by_stablecoin`](#0xc8_obc_system_get_obc_by_stablecoin)
-  [Function `next_epoch_obc_required`](#0xc8_obc_system_next_epoch_obc_required)
-  [Function `treasury_balance`](#0xc8_obc_system_treasury_balance)
-  [Function `deposit_to_treasury`](#0xc8_obc_system_deposit_to_treasury)
-  [Function `set_voting_delay`](#0xc8_obc_system_set_voting_delay)
-  [Function `cast_vote`](#0xc8_obc_system_cast_vote)
-  [Function `change_vote`](#0xc8_obc_system_change_vote)
-  [Function `queue_proposal_action`](#0xc8_obc_system_queue_proposal_action)
-  [Function `revoke_vote`](#0xc8_obc_system_revoke_vote)
-  [Function `unvote_votes`](#0xc8_obc_system_unvote_votes)
-  [Function `vote_of`](#0xc8_obc_system_vote_of)
-  [Function `has_vote`](#0xc8_obc_system_has_vote)
-  [Function `cluster_add_admin`](#0xc8_obc_system_cluster_add_admin)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="obc_dao.md#0xc8_obc_dao">0xc8::obc_dao</a>;
<b>use</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager">0xc8::obc_dao_manager</a>;
<b>use</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner">0xc8::obc_system_state_inner</a>;
<b>use</b> <a href="usd.md#0xc8_usd">0xc8::usd</a>;
<b>use</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool">0xc8::voting_pool</a>;
</code></pre>



<a name="0xc8_obc_system_ObcSystemState"></a>

## Resource `ObcSystemState`



<pre><code><b>struct</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a> <b>has</b> key
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


<a name="0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1"></a>



<pre><code><b>const</b> <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>: u64 = 1;
</code></pre>



<a name="0xc8_obc_system_create"></a>

## Function `create`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>, usd_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, obc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(
    id: UID,
    usd_supply: Supply&lt;USD&gt;,
    obc_balance: Balance&lt;OBC&gt;,
    parameters: ObcSystemParameters,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner_state = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">obc_system_state_inner::create_inner_state</a>(
        usd_supply,
        obc_balance,
        parameters,
        ctx,
    );
    <b>let</b> self = <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a> {
        id,
        version: <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>
    };

    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>, inner_state);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0xc8_obc_system_obc_round"></a>

## Function `obc_round`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, round: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    round: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_update_round">obc_system_state_inner::update_round</a>(inner_state, round);
    //exchange all <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> <b>to</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">obc</a>.
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_all">obc_system_state_inner::request_exchange_all</a>(inner_state, ctx);
    //<b>update</b> inner exchange rate from <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>-swap.
    <b>let</b> <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;USD&gt;(ctx);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">obc_system_state_inner::request_update_gas_coin</a>(inner_state, &<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>));
    // X-<a href="treasury.md#0xc8_treasury">treasury</a> rebalance
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_rebalance">obc_system_state_inner::rebalance</a>(inner_state, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);

    <a href="obc_system.md#0xc8_obc_system_judge_proposal_state">judge_proposal_state</a>(wrapper, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>));
}
</code></pre>



</details>

<a name="0xc8_obc_system_update_round"></a>

## Function `update_round`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_update_round">update_round</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_update_round">update_round</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
	<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
){
    <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(wrapper,  <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>,200, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_load_system_state"></a>

## Function `load_system_state`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(self: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>): &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(
    self: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
): &ObcSystemStateInner {
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(&self.id, self.version)
}
</code></pre>



</details>

<a name="0xc8_obc_system_load_system_state_mut"></a>

## Function `load_system_state_mut`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>): &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>
): &<b>mut</b> ObcSystemStateInner {
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(&<b>mut</b> self.id, self.version)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_get_exchange_rate"></a>

## Function `request_get_exchange_rate`

Getter of the gas coin exchange pool rate.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_get_exchange_rate">request_get_exchange_rate</a>(self: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_get_exchange_rate">request_get_exchange_rate</a>(
    self: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: &Coin&lt;USD&gt;
): u64 {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_requst_get_exchange_rate">obc_system_state_inner::requst_get_exchange_rate</a>&lt;USD&gt;(inner_state, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_add_gas_coin"></a>

## Function `request_add_gas_coin`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_add_gas_coin">request_add_gas_coin</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_add_gas_coin">request_add_gas_coin</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    gas_coin: &Coin&lt;USD&gt;,
    rate: u64,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_add_gas_coin">obc_system_state_inner::request_add_gas_coin</a>(inner_state, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_update_gas_coin"></a>

## Function `request_update_gas_coin`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_update_gas_coin">request_update_gas_coin</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_update_gas_coin">request_update_gas_coin</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    gas_coin: &Coin&lt;USD&gt;,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">obc_system_state_inner::request_update_gas_coin</a>(inner_state, gas_coin)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_remove_gas_coin"></a>

## Function `request_remove_gas_coin`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_remove_gas_coin">request_remove_gas_coin</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_remove_gas_coin">request_remove_gas_coin</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    gas_coin: &Coin&lt;USD&gt;,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_remove_gas_coin">obc_system_state_inner::request_remove_gas_coin</a>(inner_state, gas_coin)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_exchange_stable"></a>

## Function `request_exchange_stable`

Request exchange stable coin to obc.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable">request_exchange_stable</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable">request_exchange_stable</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: Coin&lt;USD&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <b>let</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a> = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_stablecoin_to_obc_balance">obc_system_state_inner::swap_stablecoin_to_obc_balance</a>&lt;USD&gt;(
        inner_state,
        <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>,
        ctx);
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>, ctx), <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_exchange_all"></a>

## Function `request_exchange_all`

Request exchange all stable coin to obc.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_all">request_exchange_all</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_all">request_exchange_all</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_all">obc_system_state_inner::request_exchange_all</a>(inner_state, ctx)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_withdraw_stable"></a>

## Function `request_withdraw_stable`

Request withdraw stable coin.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_withdraw_stable">request_withdraw_stable</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_withdraw_stable">request_withdraw_stable</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> stables = <a href="obc_system.md#0xc8_obc_system_request_withdraw_stable_no_entry">request_withdraw_stable_no_entry</a>(self);
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(stables, ctx), <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_withdraw_stable_no_entry"></a>

## Function `request_withdraw_stable_no_entry`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_withdraw_stable_no_entry">request_withdraw_stable_no_entry</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_withdraw_stable_no_entry">request_withdraw_stable_no_entry</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
): Balance&lt;USD&gt; {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_withdraw_stable">obc_system_state_inner::request_withdraw_stable</a>(inner_state)
}
</code></pre>



</details>

<a name="0xc8_obc_system_init_exchange_pool"></a>

## Function `init_exchange_pool`

Init exchange pool by add obc coin.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_init_exchange_pool">init_exchange_pool</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_init_exchange_pool">init_exchange_pool</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;OBC&gt;,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_init_exchange_pool">obc_system_state_inner::init_exchange_pool</a>(inner_state, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_obc_system_stat_parameter"></a>

## Function `obc_system_stat_parameter`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_system_stat_parameter">obc_system_stat_parameter</a>(position_number: u32, tick_spacing: u32, spacing_times: u32, initialize_price: u128, time_interval: u32, base_point: u64, max_counter_times: u32, chain_start_timestamp_ms: u64): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_system_stat_parameter">obc_system_stat_parameter</a>(
    position_number: u32,
    tick_spacing: u32,
    spacing_times: u32,
    initialize_price: u128,
    time_interval: u32,
    base_point: u64,
    max_counter_times: u32,
    chain_start_timestamp_ms: u64,
): ObcSystemParameters {
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_obc_system_stat_parameter">obc_system_state_inner::obc_system_stat_parameter</a>(
        position_number,
        tick_spacing,
        spacing_times,
        initialize_price,
        time_interval,
        base_point,
        max_counter_times,
        chain_start_timestamp_ms,
    )
}
</code></pre>



</details>

<a name="0xc8_obc_system_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_destroy_terminated_proposal">destroy_terminated_proposal</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    manager_key: &OBCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_destroy_terminated_proposal">obc_system_state_inner::destroy_terminated_proposal</a>(system_state, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_obc_system_propose"></a>

## Function `propose`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_propose">propose</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, version_id: u64, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, action_id: u64, action_delay: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_propose">propose</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    version_id : u64,
    payment: Coin&lt;OBC&gt;,
    action_id: u64,
    action_delay: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_propose">obc_system_state_inner::propose</a>(system_state, version_id, payment, action_id, action_delay, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_create_obcdao_action"></a>

## Function `create_obcdao_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_obcdao_action">create_obcdao_action</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, actionName: <a href="">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_obcdao_action">create_obcdao_action</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    payment: Coin&lt;OBC&gt;,
    actionName: <a href="">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_obcdao_action">obc_system_state_inner::create_obcdao_action</a>(system_state, payment, actionName, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, current_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>, current_time: u64) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_judge_proposal_state">obc_system_state_inner::judge_proposal_state</a>(system_state, current_time);
}
</code></pre>



</details>

<a name="0xc8_obc_system_set_voting_period"></a>

## Function `set_voting_period`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_voting_period">set_voting_period</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_voting_period">set_voting_period</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    manager_key: &OBCDaoManageKey,
    value: u64,
) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_period">obc_system_state_inner::set_voting_period</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_modify_proposal"></a>

## Function `modify_proposal`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_modify_proposal">modify_proposal</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, proposal_obj: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_modify_proposal">modify_proposal</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>, proposal_obj: &<b>mut</b> Proposal, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_modify_proposal">obc_system_state_inner::modify_proposal</a>(system_state, proposal_obj, index, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_obc_system_set_voting_quorum_rate"></a>

## Function `set_voting_quorum_rate`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_voting_quorum_rate">set_voting_quorum_rate</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_voting_quorum_rate">set_voting_quorum_rate</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>, manager_key: &OBCDaoManageKey, value: u8,){
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_quorum_rate">obc_system_state_inner::set_voting_quorum_rate</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_set_min_action_delay"></a>

## Function `set_min_action_delay`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_min_action_delay">set_min_action_delay</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_min_action_delay">set_min_action_delay</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    manager_key: &OBCDaoManageKey,
    value: u64,
) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_min_action_delay">obc_system_state_inner::set_min_action_delay</a>(system_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_withdraw_voting"></a>

## Function `withdraw_voting`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_withdraw_voting">withdraw_voting</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_withdraw_voting">withdraw_voting</a>(   wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
                             voting_obc: VotingObc,
                             ctx: &<b>mut</b> TxContext) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_withdraw_voting">obc_system_state_inner::withdraw_voting</a>(system_state, voting_obc, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_create_voting_obc"></a>

## Function `create_voting_obc`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_voting_obc">create_voting_obc</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_voting_obc">create_voting_obc</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
                             <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;OBC&gt;,
                             ctx: &<b>mut</b> TxContext) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_voting_obc">obc_system_state_inner::create_voting_obc</a>(system_state, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_swap_obc_to_stablecoin"></a>

## Function `swap_obc_to_stablecoin`

X treasury  swap obc to stablecoin


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_swap_obc_to_stablecoin">swap_obc_to_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, coin_obc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_swap_obc_to_stablecoin">swap_obc_to_stablecoin</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    coin_obc: Coin&lt;OBC&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_obc_to_stablecoin">obc_system_state_inner::swap_obc_to_stablecoin</a>&lt;StableCoinType&gt;(system_state, coin_obc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_swap_stablecoin_to_obc"></a>

## Function `swap_stablecoin_to_obc`

X treasury  swap stablecoin to obc


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_swap_stablecoin_to_obc">swap_stablecoin_to_obc</a>&lt;StableCoinType&gt;(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_swap_stablecoin_to_obc">swap_stablecoin_to_obc</a>&lt;StableCoinType&gt;(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_stablecoin_to_obc">obc_system_state_inner::swap_stablecoin_to_obc</a>&lt;StableCoinType&gt;(system_state, coin_sc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_get_stablecoin_by_obc"></a>

## Function `get_stablecoin_by_obc`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_get_stablecoin_by_obc">get_stablecoin_by_obc</a>&lt;StableCoinType&gt;(wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_get_stablecoin_by_obc">get_stablecoin_by_obc</a>&lt;StableCoinType&gt;(
    wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    amount: u64,
): u64
{
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_stablecoin_by_obc">obc_system_state_inner::get_stablecoin_by_obc</a>&lt;StableCoinType&gt;(system_state, amount)
}
</code></pre>



</details>

<a name="0xc8_obc_system_get_obc_by_stablecoin"></a>

## Function `get_obc_by_stablecoin`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_get_obc_by_stablecoin">get_obc_by_stablecoin</a>&lt;StableCoinType&gt;(wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_get_obc_by_stablecoin">get_obc_by_stablecoin</a>&lt;StableCoinType&gt;(
    wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    amount: u64,
): u64
{
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_obc_by_stablecoin">obc_system_state_inner::get_obc_by_stablecoin</a>&lt;StableCoinType&gt;(system_state, amount)
}
</code></pre>



</details>

<a name="0xc8_obc_system_next_epoch_obc_required"></a>

## Function `next_epoch_obc_required`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_next_epoch_obc_required">next_epoch_obc_required</a>(wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_next_epoch_obc_required">next_epoch_obc_required</a>(wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>): u64 {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_next_epoch_obc_required">obc_system_state_inner::next_epoch_obc_required</a>(system_state)
}
</code></pre>



</details>

<a name="0xc8_obc_system_treasury_balance"></a>

## Function `treasury_balance`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_treasury_balance">treasury_balance</a>(wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_treasury_balance">treasury_balance</a>(wrapper: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>): u64 {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_treasury_balance">obc_system_state_inner::treasury_balance</a>(system_state)
}
</code></pre>



</details>

<a name="0xc8_obc_system_deposit_to_treasury"></a>

## Function `deposit_to_treasury`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;OBC&gt;) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_deposit_to_treasury">obc_system_state_inner::deposit_to_treasury</a>(inner_state, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_set_voting_delay"></a>

## Function `set_voting_delay`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_set_voting_delay">set_voting_delay</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    manager_key: &OBCDaoManageKey,
    value: u64,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_delay">obc_system_state_inner::set_voting_delay</a>(inner_state, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_cast_vote"></a>

## Function `cast_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_cast_vote">cast_vote</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, agreeInt: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_cast_vote">cast_vote</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: VotingObc,
    agreeInt: u8,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
)  {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_cast_vote">obc_system_state_inner::cast_vote</a>(inner_state, proposal, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, agreeInt, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_change_vote"></a>

## Function `change_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_change_vote">change_vote</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, my_vote: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Vote">obc_dao::Vote</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, agree: bool, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_change_vote">change_vote</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    my_vote: &<b>mut</b> Vote,
    proposal: &<b>mut</b> Proposal,
    agree: bool,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_change_vote">obc_system_state_inner::change_vote</a>(inner_state, my_vote, proposal, agree, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_queue_proposal_action"></a>

## Function `queue_proposal_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_queue_proposal_action">queue_proposal_action</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_queue_proposal_action">queue_proposal_action</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    manager_key: &OBCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_queue_proposal_action">obc_system_state_inner::queue_proposal_action</a>(inner_state, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_obc_system_revoke_vote"></a>

## Function `revoke_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_revoke_vote">revoke_vote</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, my_vote: <a href="obc_dao.md#0xc8_obc_dao_Vote">obc_dao::Vote</a>, voting_power: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_revoke_vote">revoke_vote</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    proposal: &<b>mut</b> Proposal,
    my_vote:  Vote,
    voting_power: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_revoke_vote">obc_system_state_inner::revoke_vote</a>(inner_state, proposal, my_vote, voting_power, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_unvote_votes"></a>

## Function `unvote_votes`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_unvote_votes">unvote_votes</a>(proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, vote: <a href="obc_dao.md#0xc8_obc_dao_Vote">obc_dao::Vote</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_unvote_votes">unvote_votes</a>(
    proposal: &<b>mut</b> Proposal,
    vote: Vote,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_dao.md#0xc8_obc_dao_unvote_votes">obc_dao::unvote_votes</a>(proposal, vote, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_vote_of"></a>

## Function `vote_of`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_vote_of">vote_of</a>(vote: &<a href="obc_dao.md#0xc8_obc_dao_Vote">obc_dao::Vote</a>, proposal: &<a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_vote_of">vote_of</a>(
    vote: &Vote,
    proposal: &Proposal,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_dao.md#0xc8_obc_dao_vote_of">obc_dao::vote_of</a>(vote, proposal, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_has_vote"></a>

## Function `has_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_has_vote">has_vote</a>(vote: &<a href="obc_dao.md#0xc8_obc_dao_Vote">obc_dao::Vote</a>, proposal: &<a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_has_vote">has_vote</a>(
    vote: &Vote,
    proposal: &Proposal,
) {
    <a href="obc_dao.md#0xc8_obc_dao_has_vote">obc_dao::has_vote</a>(vote, proposal);
}
</code></pre>



</details>

<a name="0xc8_obc_system_cluster_add_admin"></a>

## Function `cluster_add_admin`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_cluster_add_admin">cluster_add_admin</a>(new_admin: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_cluster_add_admin">cluster_add_admin</a>(
    new_admin:<b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_dao.md#0xc8_obc_dao_add_admin">obc_dao::add_admin</a>(new_admin, ctx);
    //<a href="obc_dao_manager.md#0xc8_obc_dao_manager_new">obc_dao_manager::new</a>(new_admin, ctx);
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

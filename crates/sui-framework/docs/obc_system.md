
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
-  [Function `request_exchange_stable_no_entry`](#0xc8_obc_system_request_exchange_stable_no_entry)
-  [Function `request_exchange_all`](#0xc8_obc_system_request_exchange_all)
-  [Function `request_withdraw_stable`](#0xc8_obc_system_request_withdraw_stable)
-  [Function `request_withdraw_stable_no_entry`](#0xc8_obc_system_request_withdraw_stable_no_entry)
-  [Function `init_exchange_pool`](#0xc8_obc_system_init_exchange_pool)
-  [Function `obc_system_stat_parameter`](#0xc8_obc_system_obc_system_stat_parameter)
-  [Function `destroy_terminated_proposal`](#0xc8_obc_system_destroy_terminated_proposal)
-  [Function `propose`](#0xc8_obc_system_propose)
-  [Function `create_obcdao_action`](#0xc8_obc_system_create_obcdao_action)
-  [Function `judge_proposal_state`](#0xc8_obc_system_judge_proposal_state)
-  [Function `modify_proposal`](#0xc8_obc_system_modify_proposal)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field">0x2::dynamic_object_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">0x2::stable</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="obc_dao.md#0xc8_obc_dao">0xc8::obc_dao</a>;
<b>use</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager">0xc8::obc_dao_manager</a>;
<b>use</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner">0xc8::obc_system_state_inner</a>;
<b>use</b> <a href="usd.md#0xc8_usd">0xc8::usd</a>;
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



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>, usd_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(
    id: UID,
    usd_supply: Supply&lt;USD&gt;,
<<<<<<< HEAD
    parameters: ObcSystemParameters,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner_state = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">obc_system_state_inner::create_inner_state</a>(
        usd_supply,
        parameters,
        ctx,
    );
=======
    parameters: <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">ObcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">obc_system_state_inner::create_inner_state</a>(ctx);
>>>>>>> feature/any_coin_gas
    <b>let</b> self = <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a> {
        id,
        version: <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>
    };

    <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_add">dynamic_object_field::add</a>(&<b>mut</b> self.id, <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>, inner_state);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0xc8_obc_system_obc_round"></a>

## Function `obc_round`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, round: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    round: u64,
<<<<<<< HEAD
=======
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
>>>>>>> feature/any_coin_gas
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_update_round">obc_system_state_inner::update_round</a>(inner_state, round);
    //exchange all <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> <b>to</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">obc</a>.
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_all">obc_system_state_inner::request_exchange_all</a>(inner_state, ctx);
    // //<b>update</b> inner exchange rate from <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>-<a href="swap.md#0xc8_swap">swap</a>.
    // <b>let</b> <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;STABLE&gt;(ctx);
    //todo read rate from <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>-<a href="swap.md#0xc8_swap">swap</a>.
    // <b>let</b> rate = 1000000000;
    // <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">obc_system_state_inner::request_update_gas_coin</a>(inner_state, &<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>, rate);
    // <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>));
}
</code></pre>



</details>

<a name="0xc8_obc_system_update_round"></a>

## Function `update_round`



<<<<<<< HEAD
<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_update_round">update_round</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
=======
<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_update_round">update_round</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
>>>>>>> feature/any_coin_gas
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_update_round">update_round</a>(
<<<<<<< HEAD
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(wrapper, 200, ctx)
=======
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock, ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(wrapper, 200, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
>>>>>>> feature/any_coin_gas
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
    self: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>
): &ObcSystemStateInner {
<<<<<<< HEAD
    <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_borrow">dynamic_object_field::borrow</a>(&self.id, self.version)
=======
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>(&self.id, self.version)
>>>>>>> feature/any_coin_gas
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
    <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_borrow_mut">dynamic_object_field::borrow_mut</a>(&<b>mut</b> self.id, self.version)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_get_exchange_rate"></a>

## Function `request_get_exchange_rate`

Getter of the gas coin exchange pool rate.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_get_exchange_rate">request_get_exchange_rate</a>(self: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_get_exchange_rate">request_get_exchange_rate</a>(
    self: &<a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: &Coin&lt;STABLE&gt;
): u64 {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state">load_system_state</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_requst_get_exchange_rate">obc_system_state_inner::requst_get_exchange_rate</a>&lt;STABLE&gt;(inner_state, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_add_gas_coin"></a>

## Function `request_add_gas_coin`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_add_gas_coin">request_add_gas_coin</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;, rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_add_gas_coin">request_add_gas_coin</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    gas_coin: &Coin&lt;STABLE&gt;,
    rate: u64,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_add_gas_coin">obc_system_state_inner::request_add_gas_coin</a>(inner_state, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_update_gas_coin"></a>

## Function `request_update_gas_coin`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_update_gas_coin">request_update_gas_coin</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;, rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_update_gas_coin">request_update_gas_coin</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    gas_coin: &Coin&lt;STABLE&gt;,
    rate: u64,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">obc_system_state_inner::request_update_gas_coin</a>(inner_state, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_remove_gas_coin"></a>

## Function `request_remove_gas_coin`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_remove_gas_coin">request_remove_gas_coin</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_remove_gas_coin">request_remove_gas_coin</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    gas_coin: &Coin&lt;STABLE&gt;,
) {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_remove_gas_coin">obc_system_state_inner::request_remove_gas_coin</a>(inner_state, gas_coin)
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_exchange_stable"></a>

## Function `request_exchange_stable`

Request exchange stable coin to obc.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable">request_exchange_stable</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable">request_exchange_stable</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: Coin&lt;STABLE&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a> = <a href="obc_system.md#0xc8_obc_system_request_exchange_stable_no_entry">request_exchange_stable_no_entry</a>(self, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>, ctx);
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>, ctx), <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_exchange_stable_no_entry"></a>

## Function `request_exchange_stable_no_entry`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable_no_entry">request_exchange_stable_no_entry</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable_no_entry">request_exchange_stable_no_entry</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: Coin&lt;STABLE&gt;,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;OBC&gt; {
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_stable">obc_system_state_inner::request_exchange_stable</a>(inner_state, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>, ctx)
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



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_withdraw_stable_no_entry">request_withdraw_stable_no_entry</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_withdraw_stable_no_entry">request_withdraw_stable_no_entry</a>(
    self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
): Balance&lt;STABLE&gt; {
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



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_system_stat_parameter">obc_system_stat_parameter</a>(position_number: u32, tick_spacing: u32, spacing_times: u32, initialize_price: u128, time_interval: u32, base_point: u64, chain_start_timestamp_ms: u64): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>
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
    chain_start_timestamp_ms: u64,
<<<<<<< HEAD
): ObcSystemParameters {
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_obc_system_stat_parameter">obc_system_state_inner::obc_system_stat_parameter</a>(
=======
): <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">ObcSystemParameters</a> {
    <b>let</b> treasury_parameters = <a href="obc_system.md#0xc8_obc_system_TreasuryParameters">TreasuryParameters</a> {
>>>>>>> feature/any_coin_gas
        position_number,
        tick_spacing,
        spacing_times,
        initialize_price,
        time_interval,
        base_point,
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


<pre><code>entry <b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_destroy_terminated_proposal">destroy_terminated_proposal</a>(
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



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_propose">propose</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;obc::OBC&gt;, action_id: u64, action_delay: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_propose">propose</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    manager_key: &OBCDaoManageKey,
    payment: Coin&lt;OBC&gt;,
    action_id: u64,
    action_delay: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_propose">obc_system_state_inner::propose</a>(system_state, manager_key, payment, action_id, action_delay, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_create_obcdao_action"></a>

## Function `create_obcdao_action`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_obcdao_action">create_obcdao_action</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, _: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, actionName: <a href="">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_obcdao_action">create_obcdao_action</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    _: &OBCDaoManageKey,
    actionName: <a href="">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_obcdao_action">obc_system_state_inner::create_obcdao_action</a>(system_state, _, actionName, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, current_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>, current_time: u64) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_judge_proposal_state">obc_system_state_inner::judge_proposal_state</a>(system_state, current_time);
}
</code></pre>



</details>

<a name="0xc8_obc_system_modify_proposal"></a>

## Function `modify_proposal`



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_modify_proposal">modify_proposal</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_modify_proposal">modify_proposal</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock) {
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_modify_proposal">obc_system_state_inner::modify_proposal</a>(system_state, index, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

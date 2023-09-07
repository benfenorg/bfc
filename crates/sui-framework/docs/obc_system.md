
<a name="0xc8_obc_system"></a>

# Module `0xc8::obc_system`



-  [Resource `ObcSystemState`](#0xc8_obc_system_ObcSystemState)
-  [Struct `TreasuryParameters`](#0xc8_obc_system_TreasuryParameters)
-  [Struct `ObcSystemParameters`](#0xc8_obc_system_ObcSystemParameters)
-  [Constants](#@Constants_0)
-  [Function `create`](#0xc8_obc_system_create)
-  [Function `create_treasury`](#0xc8_obc_system_create_treasury)
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


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field">0x2::dynamic_object_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">0x2::stable</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner">0xc8::obc_system_state_inner</a>;
<b>use</b> <a href="treasury.md#0xc8_treasury">0xc8::treasury</a>;
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

<a name="0xc8_obc_system_TreasuryParameters"></a>

## Struct `TreasuryParameters`



<pre><code><b>struct</b> <a href="obc_system.md#0xc8_obc_system_TreasuryParameters">TreasuryParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>position_number: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>tick_spacing: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>initialize_price: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_obc_system_ObcSystemParameters"></a>

## Struct `ObcSystemParameters`



<pre><code><b>struct</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">ObcSystemParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>treasury_parameters: <a href="obc_system.md#0xc8_obc_system_TreasuryParameters">obc_system::TreasuryParameters</a></code>
</dt>
<dd>

</dd>
<dt>
<code>chain_start_timestamp_ms: u64</code>
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



<a name="0xc8_obc_system_OBC_SYSTEM_TREASURY_KEY"></a>



<pre><code><b>const</b> <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_TREASURY_KEY">OBC_SYSTEM_TREASURY_KEY</a>: u64 = 3;
</code></pre>



<a name="0xc8_obc_system_create"></a>

## Function `create`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>, usd_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, parameters: <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">obc_system::ObcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(
    id: UID,
    usd_supply: Supply&lt;USD&gt;,
    parameters: <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">ObcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext,
){
    <b>let</b> inner_state = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">obc_system_state_inner::create_inner_state</a>(ctx);
    <b>let</b> self = <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a> {
        id,
        version: <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>
    };

    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>, inner_state);

    <a href="obc_system.md#0xc8_obc_system_create_treasury">create_treasury</a>(
        &<b>mut</b> self,
        usd_supply,
        parameters.treasury_parameters,
        parameters.chain_start_timestamp_ms,
        ctx
    );
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0xc8_obc_system_create_treasury"></a>

## Function `create_treasury`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_treasury">create_treasury</a>(obcsystem: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, treasury_parameters: <a href="obc_system.md#0xc8_obc_system_TreasuryParameters">obc_system::TreasuryParameters</a>, ts: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_create_treasury">create_treasury</a>(
    obcsystem: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    supply: Supply&lt;USD&gt;,
    treasury_parameters: <a href="obc_system.md#0xc8_obc_system_TreasuryParameters">TreasuryParameters</a>,
    ts: u64,
    ctx: &<b>mut</b> TxContext
)
{
    <b>let</b> t = <a href="treasury.md#0xc8_treasury_create_treasury">treasury::create_treasury</a>(ctx);
    <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_add">dynamic_object_field::add</a>(&<b>mut</b> obcsystem.id, <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_TREASURY_KEY">OBC_SYSTEM_TREASURY_KEY</a>, t);

    <b>let</b> mut_t = <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_borrow_mut">dynamic_object_field::borrow_mut</a>&lt;u64, <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>&gt;(
        &<b>mut</b> obcsystem.id,
        <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_TREASURY_KEY">OBC_SYSTEM_TREASURY_KEY</a>
    );
    // create <a href="obc.md#0xc8_obc">obc</a>-<a href="usd.md#0xc8_usd">usd</a> pool
    <a href="treasury.md#0xc8_treasury_create_vault">treasury::create_vault</a>&lt;OBC, USD, USD&gt;(
        mut_t,
        supply,
        treasury_parameters.position_number,
        treasury_parameters.tick_spacing,
        treasury_parameters.initialize_price,
        ts,
        ctx,
    );
    // init positions
    <a href="treasury.md#0xc8_treasury_init_positions">treasury::init_positions</a>&lt;OBC, USD&gt;(mut_t, treasury_parameters.tick_spacing, 10, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_obc_round"></a>

## Function `obc_round`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, round: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
    round:  u64,
    ctx: &<b>mut</b> TxContext,
){
    <b>let</b> inner_state = <a href="obc_system.md#0xc8_obc_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_update_round">obc_system_state_inner::update_round</a>(inner_state, round);
    //exchange all <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> <b>to</b> <a href="obc.md#0xc8_obc">obc</a>.
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



<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_update_round">update_round</a>(wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_update_round">update_round</a>(
    wrapper: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a>,
){
    <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(wrapper,200)
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



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable_no_entry">request_exchange_stable_no_entry</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;obc::OBC&gt;
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


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_system.md#0xc8_obc_system_init_exchange_pool">init_exchange_pool</a>(self: &<b>mut</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">obc_system::ObcSystemState</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;obc::OBC&gt;)
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



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_system_stat_parameter">obc_system_stat_parameter</a>(position_number: u32, tick_spacing: u32, initialize_price: u128, chain_start_timestamp_ms: u64): <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">obc_system::ObcSystemParameters</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_system_stat_parameter">obc_system_stat_parameter</a>(
    position_number: u32,
    tick_spacing: u32,
    initialize_price: u128,
    chain_start_timestamp_ms: u64,
) : <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">ObcSystemParameters</a> {
    <b>let</b> treasury_parameters = <a href="obc_system.md#0xc8_obc_system_TreasuryParameters">TreasuryParameters</a> {
        position_number,
        tick_spacing,
        initialize_price,
    };
    <a href="obc_system.md#0xc8_obc_system_ObcSystemParameters">ObcSystemParameters</a> {
        treasury_parameters,
        chain_start_timestamp_ms,
    }
}
</code></pre>



</details>

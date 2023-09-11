
<a name="0xc8_obc_system_state_inner"></a>

# Module `0xc8::obc_system_state_inner`



-  [Struct `ObcSystemStateInner`](#0xc8_obc_system_state_inner_ObcSystemStateInner)
-  [Struct `TreasuryParameters`](#0xc8_obc_system_state_inner_TreasuryParameters)
-  [Struct `ObcSystemParameters`](#0xc8_obc_system_state_inner_ObcSystemParameters)
-  [Constants](#@Constants_0)
-  [Function `create_inner_state`](#0xc8_obc_system_state_inner_create_inner_state)
-  [Function `update_round`](#0xc8_obc_system_state_inner_update_round)
-  [Function `request_exchange_stable`](#0xc8_obc_system_state_inner_request_exchange_stable)
-  [Function `request_exchange_all`](#0xc8_obc_system_state_inner_request_exchange_all)
-  [Function `request_withdraw_stable`](#0xc8_obc_system_state_inner_request_withdraw_stable)
-  [Function `requst_get_exchange_rate`](#0xc8_obc_system_state_inner_requst_get_exchange_rate)
-  [Function `request_add_gas_coin`](#0xc8_obc_system_state_inner_request_add_gas_coin)
-  [Function `request_update_gas_coin`](#0xc8_obc_system_state_inner_request_update_gas_coin)
-  [Function `request_remove_gas_coin`](#0xc8_obc_system_state_inner_request_remove_gas_coin)
-  [Function `init_exchange_pool`](#0xc8_obc_system_state_inner_init_exchange_pool)
-  [Function `create_treasury`](#0xc8_obc_system_state_inner_create_treasury)
-  [Function `mint`](#0xc8_obc_system_state_inner_mint)
-  [Function `redeem`](#0xc8_obc_system_state_inner_redeem)
-  [Function `next_epoch_obc_required`](#0xc8_obc_system_state_inner_next_epoch_obc_required)
-  [Function `treasury_balance`](#0xc8_obc_system_state_inner_treasury_balance)
-  [Function `deposit_to_treasury`](#0xc8_obc_system_state_inner_deposit_to_treasury)
-  [Function `rebalance`](#0xc8_obc_system_state_inner_rebalance)
-  [Function `obc_system_stat_parameter`](#0xc8_obc_system_state_inner_obc_system_stat_parameter)
-  [Function `create_obcdao_action`](#0xc8_obc_system_state_inner_create_obcdao_action)
-  [Function `propose`](#0xc8_obc_system_state_inner_propose)
-  [Function `destroy_terminated_proposal`](#0xc8_obc_system_state_inner_destroy_terminated_proposal)
-  [Function `judge_proposal_state`](#0xc8_obc_system_state_inner_judge_proposal_state)
-  [Function `modify_proposal`](#0xc8_obc_system_state_inner_modify_proposal)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">0x2::stable</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="exchange_inner.md#0xc8_exchange_inner">0xc8::exchange_inner</a>;
<b>use</b> <a href="gas_coin_map.md#0xc8_gas_coin_map">0xc8::gas_coin_map</a>;
<b>use</b> <a href="obc_dao.md#0xc8_obc_dao">0xc8::obc_dao</a>;
<b>use</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager">0xc8::obc_dao_manager</a>;
<b>use</b> <a href="treasury.md#0xc8_treasury">0xc8::treasury</a>;
<b>use</b> <a href="usd.md#0xc8_usd">0xc8::usd</a>;
</code></pre>



<a name="0xc8_obc_system_state_inner_ObcSystemStateInner"></a>

## Struct `ObcSystemStateInner`



<pre><code><b>struct</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>round: u64</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>: <a href="gas_coin_map.md#0xc8_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a></code>
</dt>
<dd>
 Contains gas coin information
</dd>
<dt>
<code>exchange_pool: <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;</code>
</dt>
<dd>
 Exchange gas coin pool
</dd>
<dt>
<code>dao: <a href="obc_dao.md#0xc8_obc_dao_Dao">obc_dao::Dao</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="treasury.md#0xc8_treasury">treasury</a>: <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_obc_system_state_inner_TreasuryParameters"></a>

## Struct `TreasuryParameters`



<pre><code><b>struct</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_TreasuryParameters">TreasuryParameters</a> <b>has</b> <b>copy</b>, drop
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
<code>spacing_times: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>initialize_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>time_interval: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>base_point: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_counter_times: u32</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_obc_system_state_inner_ObcSystemParameters"></a>

## Struct `ObcSystemParameters`



<pre><code><b>struct</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>treasury_parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_TreasuryParameters">obc_system_state_inner::TreasuryParameters</a></code>
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


<a name="0xc8_obc_system_state_inner_DEFAULT_ADMIN_ADDRESSES"></a>



<pre><code><b>const</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>: <a href="">vector</a>&lt;<b>address</b>&gt; = [15835301299490436797531864052756717558919000202302735629799491391021929516032, 65291099566713687366712577645016528323844253956509590706950403481947121946472, 24534971471998884076320588073588140533011227823836498849038009586284449996719, 51146047687078908496806713158095522211891725112102928692628967303014320690576, 21467571842073344777586981157680533501982638629804211914469802462091489096830];
</code></pre>



<a name="0xc8_obc_system_state_inner_OBC_SYSTEM_STATE_START_ROUND"></a>



<pre><code><b>const</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_OBC_SYSTEM_STATE_START_ROUND">OBC_SYSTEM_STATE_START_ROUND</a>: u64 = 0;
</code></pre>



<a name="0xc8_obc_system_state_inner_OBC_SYSTEM_TREASURY_KEY"></a>



<pre><code><b>const</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_OBC_SYSTEM_TREASURY_KEY">OBC_SYSTEM_TREASURY_KEY</a>: u64 = 1;
</code></pre>



<a name="0xc8_obc_system_state_inner_create_inner_state"></a>

## Function `create_inner_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">create_inner_state</a>(usd_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">create_inner_state</a>(
    usd_supply: Supply&lt;USD&gt;,
    parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a> {
    // init gas <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a> mappings
    <b>let</b> init_gas_coins_map = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>&lt;<b>address</b>, GasCoinEntity&gt;();
    <b>let</b> <a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a> = <a href="gas_coin_map.md#0xc8_gas_coin_map_new">gas_coin_map::new</a>(init_gas_coins_map, ctx);
    <b>let</b> exchange_pool = <a href="exchange_inner.md#0xc8_exchange_inner_new_exchange_pool">exchange_inner::new_exchange_pool</a>&lt;STABLE&gt;(ctx, 0);
    <b>let</b> dao = <a href="obc_dao.md#0xc8_obc_dao_create_dao">obc_dao::create_dao</a>(<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>, ctx);
    <b>let</b> t = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_treasury">create_treasury</a>(usd_supply, parameters, ctx);
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a> {
        round: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_OBC_SYSTEM_STATE_START_ROUND">OBC_SYSTEM_STATE_START_ROUND</a>,
        <a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>,
        exchange_pool,
        dao,
        <a href="treasury.md#0xc8_treasury">treasury</a>: t,
    }
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_update_round"></a>

## Function `update_round`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_update_round">update_round</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, round: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_update_round">update_round</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    round: u64,
) {
    inner.round = round;
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_exchange_stable"></a>

## Function `request_exchange_stable`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_stable">request_exchange_stable</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_stable">request_exchange_stable</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: Coin&lt;STABLE&gt;,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;OBC&gt; {
    //get exchange rate
    <b>let</b> rate = <a href="gas_coin_map.md#0xc8_gas_coin_map_requst_get_exchange_rate">gas_coin_map::requst_get_exchange_rate</a>&lt;STABLE&gt;(&inner.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, &<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>);
    <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_stable">exchange_inner::request_exchange_stable</a>&lt;STABLE&gt;(rate, &<b>mut</b> inner.exchange_pool, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>, ctx)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_exchange_all"></a>

## Function `request_exchange_all`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_all">request_exchange_all</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_all">request_exchange_all</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    ctx: &<b>mut</b> TxContext
) {
    <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_all">exchange_inner::request_exchange_all</a>&lt;STABLE&gt;(&<b>mut</b> inner.exchange_pool, ctx)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_withdraw_stable"></a>

## Function `request_withdraw_stable`

Request withdraw stable coin.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_withdraw_stable">request_withdraw_stable</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/stable.md#0x2_stable_STABLE">stable::STABLE</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_withdraw_stable">request_withdraw_stable</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
): Balance&lt;STABLE&gt; {
    <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_stable">exchange_inner::request_withdraw_stable</a>(&<b>mut</b> inner.exchange_pool)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_requst_get_exchange_rate"></a>

## Function `requst_get_exchange_rate`

Getter of the gas coin exchange pool rate.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(
    self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: &Coin&lt;CoinType&gt;
): u64 {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_requst_get_exchange_rate">gas_coin_map::requst_get_exchange_rate</a>&lt;CoinType&gt;(&self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_add_gas_coin"></a>

## Function `request_add_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;, rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
    rate: u64,
) {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_add_gas_coin">gas_coin_map::request_add_gas_coin</a>&lt;CoinType&gt;(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_update_gas_coin"></a>

## Function `request_update_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;, rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
    rate: u64,
) {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_update_gas_coin">gas_coin_map::request_update_gas_coin</a>(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_remove_gas_coin"></a>

## Function `request_remove_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
) {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_remove_gas_coin">gas_coin_map::request_remove_gas_coin</a>&lt;CoinType&gt;(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_init_exchange_pool"></a>

## Function `init_exchange_pool`

Init exchange pool by add obc coin.


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_init_exchange_pool">init_exchange_pool</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_init_exchange_pool">init_exchange_pool</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;OBC&gt;,
) {
    <a href="exchange_inner.md#0xc8_exchange_inner_add_obc_to_pool">exchange_inner::add_obc_to_pool</a>(&<b>mut</b> self.exchange_pool, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_create_treasury"></a>

## Function `create_treasury`

X treasury  init treasury


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_treasury">create_treasury</a>(supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_treasury">create_treasury</a>(
    supply: Supply&lt;USD&gt;,
    parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext
): Treasury {
    <b>let</b> treasury_parameters = parameters.treasury_parameters;
    <b>let</b> t = <a href="treasury.md#0xc8_treasury_create_treasury">treasury::create_treasury</a>(treasury_parameters.time_interval, ctx);
    <a href="treasury.md#0xc8_treasury_init_vault_with_positions">treasury::init_vault_with_positions</a>&lt;USD&gt;(
        &<b>mut</b> t,
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
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_mint"></a>

## Function `mint`

swap obc to stablecoin


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_mint">mint</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_obc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_mint">mint</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    coin_obc: Coin&lt;OBC&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="treasury.md#0xc8_treasury_mint">treasury::mint</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_obc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_redeem"></a>

## Function `redeem`

swap stablecoin to obc


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_redeem">redeem</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_redeem">redeem</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="treasury.md#0xc8_treasury_redeem">treasury::redeem</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_sc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_next_epoch_obc_required"></a>

## Function `next_epoch_obc_required`

X-treasury


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_next_epoch_obc_required">next_epoch_obc_required</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_next_epoch_obc_required">next_epoch_obc_required</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>): u64 {
    <a href="treasury.md#0xc8_treasury_next_epoch_obc_required">treasury::next_epoch_obc_required</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_treasury_balance"></a>

## Function `treasury_balance`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>): u64 {
    <a href="treasury.md#0xc8_treasury_get_balance">treasury::get_balance</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_deposit_to_treasury"></a>

## Function `deposit_to_treasury`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_obc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>, coin_obc: Coin&lt;OBC&gt;) {
    <a href="treasury.md#0xc8_treasury_deposit">treasury::deposit</a>(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_obc);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_rebalance">rebalance</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_rebalance">rebalance</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="treasury.md#0xc8_treasury_rebalance">treasury::rebalance</a>(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_obc_system_stat_parameter"></a>

## Function `obc_system_stat_parameter`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_obc_system_stat_parameter">obc_system_stat_parameter</a>(position_number: u32, tick_spacing: u32, spacing_times: u32, initialize_price: u128, time_interval: u32, base_point: u64, max_counter_times: u32, chain_start_timestamp_ms: u64): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_obc_system_stat_parameter">obc_system_stat_parameter</a>(
    position_number: u32,
    tick_spacing: u32,
    spacing_times: u32,
    initialize_price: u128,
    time_interval: u32,
    base_point: u64,
    max_counter_times: u32,
    chain_start_timestamp_ms: u64,
): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a> {
    <b>let</b> treasury_parameters = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_TreasuryParameters">TreasuryParameters</a> {
        position_number,
        tick_spacing,
        spacing_times,
        initialize_price,
        time_interval,
        max_counter_times,
        base_point,
    };
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a> {
        treasury_parameters,
        chain_start_timestamp_ms,
    }
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_create_obcdao_action"></a>

## Function `create_obcdao_action`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_obcdao_action">create_obcdao_action</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, _: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, actionName: <a href="">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_obcdao_action">create_obcdao_action</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>, _: &OBCDaoManageKey,
    actionName: <a href="">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext) {
    <a href="obc_dao.md#0xc8_obc_dao_create_obcdao_action">obc_dao::create_obcdao_action</a>(&<b>mut</b> self.dao, _, actionName, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_propose"></a>

## Function `propose`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_propose">propose</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, action_id: u64, action_delay: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_propose">propose</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    manager_key: &OBCDaoManageKey,
    payment: Coin&lt;OBC&gt;,
    action_id: u64,
    action_delay: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    obc_dao:: propose(&<b>mut</b> self.dao, manager_key, payment, action_id, action_delay, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    manager_key: &OBCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock
) {
    <a href="obc_dao.md#0xc8_obc_dao_destroy_terminated_proposal">obc_dao::destroy_terminated_proposal</a>(&<b>mut</b> self.dao, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, current_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>, current_time: u64) {
    <b>let</b> proposal_record = <a href="obc_dao.md#0xc8_obc_dao_getProposalRecord">obc_dao::getProposalRecord</a>(&<b>mut</b> wrapper.dao);
    <b>let</b> size: u64 = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&proposal_record);
    <b>if</b> (size == 0) {
        <b>return</b>
    };

    <b>let</b> i = 0;
    <b>while</b> (i &lt; size) {
        <b>let</b> (_, proposalInfo) = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get_entry_by_idx">vec_map::get_entry_by_idx</a>(&proposal_record, size - 1);
        <b>let</b> cur_status = <a href="obc_dao.md#0xc8_obc_dao_judge_proposal_state">obc_dao::judge_proposal_state</a>(proposalInfo, current_time);
        <a href="obc_dao.md#0xc8_obc_dao_set_current_status_into_dao">obc_dao::set_current_status_into_dao</a>(&<b>mut</b> wrapper.dao, proposalInfo, cur_status);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_modify_proposal"></a>

## Function `modify_proposal`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_modify_proposal">modify_proposal</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_modify_proposal">modify_proposal</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock) {
    <b>let</b> proposal_record = <a href="obc_dao.md#0xc8_obc_dao_getProposalRecord">obc_dao::getProposalRecord</a>(&<b>mut</b> system_state.dao);
    <b>let</b> size: u64 = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&proposal_record);
    <b>if</b> (size == 0) {
        <b>return</b>
    };
    <b>let</b> (_, proposalInfo) = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get_entry_by_idx_mut">vec_map::get_entry_by_idx_mut</a>(&<b>mut</b> proposal_record, size - 1);
    <a href="obc_dao.md#0xc8_obc_dao_modify_proposal">obc_dao::modify_proposal</a>(proposalInfo, index, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);

    <a href="obc_dao.md#0xc8_obc_dao_setProposalRecord">obc_dao::setProposalRecord</a>(&<b>mut</b> system_state.dao, proposal_record);
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

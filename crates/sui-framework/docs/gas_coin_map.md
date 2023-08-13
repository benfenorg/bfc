
<a name="0x3_gas_coin_map"></a>

# Module `0x3::gas_coin_map`



-  [Struct `GasCoinMap`](#0x3_gas_coin_map_GasCoinMap)
-  [Struct `GasCoinEntity`](#0x3_gas_coin_map_GasCoinEntity)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x3_gas_coin_map_new)
-  [Function `new_default_entity`](#0x3_gas_coin_map_new_default_entity)
-  [Function `new_entity`](#0x3_gas_coin_map_new_entity)
-  [Function `map_size`](#0x3_gas_coin_map_map_size)
-  [Function `request_add_gas_coin`](#0x3_gas_coin_map_request_add_gas_coin)
-  [Function `request_update_gas_coin`](#0x3_gas_coin_map_request_update_gas_coin)
-  [Function `requst_get_exchange_rate`](#0x3_gas_coin_map_requst_get_exchange_rate)
-  [Function `request_remove_gas_coin`](#0x3_gas_coin_map_request_remove_gas_coin)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
</code></pre>



<a name="0x3_gas_coin_map_GasCoinMap"></a>

## Struct `GasCoinMap`



<pre><code><b>struct</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>active_gas_coins: <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<b>address</b>, <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">gas_coin_map::GasCoinEntity</a>&gt;</code>
</dt>
<dd>
The current active gas coin
</dd>
</dl>


</details>

<a name="0x3_gas_coin_map_GasCoinEntity"></a>

## Struct `GasCoinEntity`



<pre><code><b>struct</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>exchange_rate: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_gas_coin_map_DEFAULT_EXCHANGE_RATE"></a>

Default exchange rate


<pre><code><b>const</b> <a href="gas_coin_map.md#0x3_gas_coin_map_DEFAULT_EXCHANGE_RATE">DEFAULT_EXCHANGE_RATE</a>: u64 = 1000000000;
</code></pre>



<a name="0x3_gas_coin_map_new"></a>

## Function `new`

Init gas coin map


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_new">new</a>(init_gas_coins: <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<b>address</b>, <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">gas_coin_map::GasCoinEntity</a>&gt;, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_new">new</a>(init_gas_coins: VecMap&lt;<b>address</b>, <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a>&gt;, _ctx: &<b>mut</b> TxContext): <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a> {
    <b>let</b> active_gas_coins = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>&lt;<b>address</b>, <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a>&gt;();
    <b>let</b> init_keys = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_keys">vec_map::keys</a>(&init_gas_coins);
    <b>let</b> num_coins = <a href="_length">vector::length</a>(&init_keys);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; num_coins) {
        <b>let</b> (id, gasCoin) = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_pop">vec_map::pop</a>(&<b>mut</b> init_gas_coins);
        <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> active_gas_coins, id, gasCoin);
        i = i + 1;
    };
   <b>let</b> map = <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a> {
        active_gas_coins
    };
    map
}
</code></pre>



</details>

<a name="0x3_gas_coin_map_new_default_entity"></a>

## Function `new_default_entity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_new_default_entity">new_default_entity</a>(id_address: <b>address</b>): <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">gas_coin_map::GasCoinEntity</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_new_default_entity">new_default_entity</a>(id_address: <b>address</b>): <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a> {
    <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a> {
        id_address,
        exchange_rate:<a href="gas_coin_map.md#0x3_gas_coin_map_DEFAULT_EXCHANGE_RATE">DEFAULT_EXCHANGE_RATE</a>
    }
}
</code></pre>



</details>

<a name="0x3_gas_coin_map_new_entity"></a>

## Function `new_entity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_new_entity">new_entity</a>(id_address: <b>address</b>, exchange_rate: u64): <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">gas_coin_map::GasCoinEntity</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_new_entity">new_entity</a>(id_address: <b>address</b>, exchange_rate: u64): <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a> {
    <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a> {
        id_address,
        exchange_rate
    }
}
</code></pre>



</details>

<a name="0x3_gas_coin_map_map_size"></a>

## Function `map_size`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_map_size">map_size</a>(self: &<a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_map_size">map_size</a>(self: &<a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a>): u64 {
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&self.active_gas_coins)
}
</code></pre>



</details>

<a name="0x3_gas_coin_map_request_add_gas_coin"></a>

## Function `request_add_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a>,
    gas_coin: &Coin&lt;CoinType&gt;) {
    <b>let</b> id_address = <a href="../../../.././build/Sui/docs/object.md#0x2_object_id_address">object::id_address</a>&lt;Coin&lt;CoinType&gt;&gt;(gas_coin);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> self.active_gas_coins, id_address, <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinEntity">GasCoinEntity</a> {
        id_address,
        exchange_rate: <a href="gas_coin_map.md#0x3_gas_coin_map_DEFAULT_EXCHANGE_RATE">DEFAULT_EXCHANGE_RATE</a>
    });
}
</code></pre>



</details>

<a name="0x3_gas_coin_map_request_update_gas_coin"></a>

## Function `request_update_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;, exchange_rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a>,
    gas_coin: &Coin&lt;CoinType&gt;, exchange_rate: u64) {
    <b>let</b> id_address = <a href="../../../.././build/Sui/docs/object.md#0x2_object_id_address">object::id_address</a>&lt;Coin&lt;CoinType&gt;&gt;(gas_coin);
    <b>let</b> entity = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get_mut">vec_map::get_mut</a>(&<b>mut</b> self.active_gas_coins, &id_address);
    entity.exchange_rate = exchange_rate
}
</code></pre>



</details>

<a name="0x3_gas_coin_map_requst_get_exchange_rate"></a>

## Function `requst_get_exchange_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(
    self: &<a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a>,
    gas_coin: &Coin&lt;CoinType&gt;): u64 {
    <b>let</b> id_address = <a href="../../../.././build/Sui/docs/object.md#0x2_object_id_address">object::id_address</a>&lt;Coin&lt;CoinType&gt;&gt;(gas_coin);
    <b>let</b> gas_entity = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get">vec_map::get</a>(&self.active_gas_coins, &id_address);
    gas_entity.exchange_rate
}
</code></pre>



</details>

<a name="0x3_gas_coin_map_request_remove_gas_coin"></a>

## Function `request_remove_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="gas_coin_map.md#0x3_gas_coin_map_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="gas_coin_map.md#0x3_gas_coin_map_GasCoinMap">GasCoinMap</a>,
    gas_coin: &Coin&lt;CoinType&gt;,) {
    <b>let</b> id_address = <a href="../../../.././build/Sui/docs/object.md#0x2_object_id_address">object::id_address</a>&lt;Coin&lt;CoinType&gt;&gt;(gas_coin);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>(&<b>mut</b> self.active_gas_coins, &id_address);
}
</code></pre>



</details>

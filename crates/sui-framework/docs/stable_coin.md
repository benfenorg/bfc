
<a name="0x3_stable_coin"></a>

# Module `0x3::stable_coin`



-  [Struct `DummyCoin`](#0x3_stable_coin_DummyCoin)
-  [Function `new_dummy`](#0x3_stable_coin_new_dummy)
-  [Function `request_swap_obc`](#0x3_stable_coin_request_swap_obc)
-  [Function `request_swap`](#0x3_stable_coin_request_swap)
-  [Function `request_price`](#0x3_stable_coin_request_price)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x3_stable_coin_DummyCoin"></a>

## Struct `DummyCoin`



<pre><code><b>struct</b> <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>&lt;T&gt; <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_stable_coin_new_dummy"></a>

## Function `new_dummy`



<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_new_dummy">new_dummy</a>&lt;T&gt;(): <a href="stable_coin.md#0x3_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_new_dummy">new_dummy</a>&lt;T&gt;(): <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>&lt;T&gt; {
    <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>{}
}
</code></pre>



</details>

<a name="0x3_stable_coin_request_swap_obc"></a>

## Function `request_swap_obc`

Request of swap obc coin


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_request_swap_obc">request_swap_obc</a>&lt;CoinType&gt;(_stable_coin: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;CoinType&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_request_swap_obc">request_swap_obc</a>&lt;CoinType&gt;(
    _stable_coin: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>&lt;CoinType&gt;, ctx: &<b>mut</b> TxContext): Coin&lt;OBC&gt;  {
   // mock for rust test
   <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;OBC&gt;(ctx)
}
</code></pre>



</details>

<a name="0x3_stable_coin_request_swap"></a>

## Function `request_swap`

Request of swap two stable coin


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_request_swap">request_swap</a>&lt;CoinX, CoinY&gt;(_stable_coin: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;CoinX&gt;, _stable2: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;CoinY&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_request_swap">request_swap</a>&lt;CoinX, CoinY&gt;(
    _stable_coin: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>&lt;CoinX&gt;, _stable2: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>&lt;CoinY&gt;)  {
}
</code></pre>



</details>

<a name="0x3_stable_coin_request_price"></a>

## Function `request_price`

Request of price of two stable coin


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_request_price">request_price</a>&lt;CoinX, CoinY&gt;(_stable_coin: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;CoinX&gt;, _stable2: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;CoinY&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0x3_stable_coin_request_price">request_price</a>&lt;CoinX, CoinY&gt;(
    _stable_coin: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>&lt;CoinX&gt;, _stable2: <a href="stable_coin.md#0x3_stable_coin_DummyCoin">DummyCoin</a>&lt;CoinY&gt;)  {
}
</code></pre>



</details>

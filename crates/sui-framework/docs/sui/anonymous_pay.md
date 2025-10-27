---
title: Module `sui::anonymous_pay`
---

This module provides handy functionality for wallets and <code>sui::Coin</code> management.


-  [Constants](#@Constants_0)
-  [Function `keep`](#sui_anonymous_pay_keep)
-  [Function `split`](#sui_anonymous_pay_split)
-  [Function `split_anonymous`](#sui_anonymous_pay_split_anonymous)
-  [Function `split_vec`](#sui_anonymous_pay_split_vec)
-  [Function `split_and_transfer`](#sui_anonymous_pay_split_and_transfer)
-  [Function `split_and_transfer_anonymous`](#sui_anonymous_pay_split_and_transfer_anonymous)
-  [Function `join`](#sui_anonymous_pay_join)
-  [Function `join_vec`](#sui_anonymous_pay_join_vec)
-  [Function `join_vec_and_transfer`](#sui_anonymous_pay_join_vec_and_transfer)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance">sui::anonymous_balance</a>;
<b>use</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin">sui::anonymous_coin</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/hfe_ops.md#sui_hfe_ops">sui::hfe_ops</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="sui_anonymous_pay_ENoCoins"></a>

For when empty vector is supplied into join function.


<pre><code><b>const</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_ENoCoins">ENoCoins</a>: u64 = 0;
</code></pre>



<a name="sui_anonymous_pay_keep"></a>

## Function `keep`

Transfer <code>c</code> to the sender of the current transaction


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_keep">keep</a>&lt;T&gt;(c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_keep">keep</a>&lt;T&gt;(c: Anonymous_Coin&lt;T&gt;, ctx: &TxContext) {
    <a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(c, ctx.sender())
}
</code></pre>



</details>

<a name="sui_anonymous_pay_split"></a>

## Function `split`

Split coin <code>self</code> to two coins, one with balance <code>split_amount</code>,
and the remaining balance is left is <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split">split</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split">split</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: &<b>mut</b> Anonymous_Coin&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    <a href="../sui/anonymous_pay.md#sui_anonymous_pay_keep">keep</a>(<a href="../sui/coin.md#sui_coin">coin</a>.<a href="../sui/anonymous_pay.md#sui_anonymous_pay_split">split</a>(split_amount, ctx), ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_pay_split_anonymous"></a>

## Function `split_anonymous`

Split coin <code>self</code> to two coins, one with balance <code>split_amount</code>,
and the remaining balance is left is <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_anonymous">split_anonymous</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, value1: vector&lt;u8&gt;, value2: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_anonymous">split_anonymous</a>&lt;T&gt;(
    <a href="../sui/coin.md#sui_coin">coin</a>: &<b>mut</b> Anonymous_Coin&lt;T&gt;,
    value1: vector&lt;u8&gt;,
    value2: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext) {
    <a href="../sui/anonymous_pay.md#sui_anonymous_pay_keep">keep</a>(<a href="../sui/coin.md#sui_coin">coin</a>.<a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_anonymous">split_anonymous</a>(value1, value2, ctx), ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_pay_split_vec"></a>

## Function `split_vec`

Split coin <code>self</code> into multiple coins, each with balance specified
in <code>split_amounts</code>. Remaining balance is left in <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> Anonymous_Coin&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b>
    ( <b>mut</b>
    i,
    len) =
    (0, split_amounts.length());
    <b>while</b> (i &lt; len) {
        <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split">split</a>(self, split_amounts[i], ctx);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="sui_anonymous_pay_split_and_transfer"></a>

## Function `split_and_transfer`

Send <code>amount</code> units of <code>c</code> to <code>recipient</code>
Aborts with <code>EVALUE</code> if <code>amount</code> is greater than or equal to <code>amount</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(_c: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, _amount: u64, _recipient: <b>address</b>, _ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(
    _c: &<b>mut</b> Anonymous_Coin&lt;T&gt;,
    _amount: u64,
    _recipient: <b>address</b>,
    _ctx: &<b>mut</b> TxContext,
) {
    //should not <b>use</b> this
    <b>abort</b> 99
    //todo : add <b>abort</b>
    //<a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(c.<a href="../sui/anonymous_pay.md#sui_anonymous_pay_split">split</a>(amount, ctx), recipient)
}
</code></pre>



</details>

<a name="sui_anonymous_pay_split_and_transfer_anonymous"></a>

## Function `split_and_transfer_anonymous`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_and_transfer_anonymous">split_and_transfer_anonymous</a>&lt;T&gt;(c: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, value1: vector&lt;u8&gt;, value2: vector&lt;u8&gt;, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_and_transfer_anonymous">split_and_transfer_anonymous</a>&lt;T&gt;(
    c: &<b>mut</b> Anonymous_Coin&lt;T&gt;,
    value1: vector&lt;u8&gt;,
    value2: vector&lt;u8&gt;,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(c.<a href="../sui/anonymous_pay.md#sui_anonymous_pay_split_anonymous">split_anonymous</a>(value1, value2, ctx), recipient)
}
</code></pre>



</details>

<a name="sui_anonymous_pay_join"></a>

## Function `join`

Join <code><a href="../sui/coin.md#sui_coin">coin</a></code> into <code>self</code>. Re-exports <code><a href="../sui/coin.md#sui_coin_join">coin::join</a></code> function.
Deprecated: you should call <code><a href="../sui/coin.md#sui_coin">coin</a>.<a href="../sui/anonymous_pay.md#sui_anonymous_pay_join">join</a>(other)</code> directly.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, <a href="../sui/coin.md#sui_coin">coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> Anonymous_Coin&lt;T&gt;, <a href="../sui/coin.md#sui_coin">coin</a>: Anonymous_Coin&lt;T&gt;) {
    self.<a href="../sui/anonymous_pay.md#sui_anonymous_pay_join">join</a>(<a href="../sui/coin.md#sui_coin">coin</a>)
}
</code></pre>



</details>

<a name="sui_anonymous_pay_join_vec"></a>

## Function `join_vec`

Join everything in <code>coins</code> with <code>self</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, coins: vector&lt;<a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> Anonymous_Coin&lt;T&gt;, <b>mut</b> coins: vector&lt;Anonymous_Coin&lt;T&gt;&gt;) {
<b>let</b> ( <b>mut</b> i, len) = (0, coins.length());
<b>while</b> (i &lt; len) {
<b>let</b> <a href="../sui/coin.md#sui_coin">coin</a> = coins.pop_back();
self.<a href="../sui/anonymous_pay.md#sui_anonymous_pay_join">join</a>(<a href="../sui/coin.md#sui_coin">coin</a>);
i = i + 1
};
// safe because we've drained the vector
coins.destroy_empty()
}
</code></pre>



</details>

<a name="sui_anonymous_pay_join_vec_and_transfer"></a>

## Function `join_vec_and_transfer`

Join a vector of <code>Coin</code> into a single object and transfer it to <code>receiver</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;(coins: vector&lt;<a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;&gt;, receiver: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_pay.md#sui_anonymous_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;( <b>mut</b> coins: vector&lt;Anonymous_Coin&lt;T&gt;&gt;, receiver: <b>address</b>) {
<b>assert</b>!(coins.length() &gt; 0, <a href="../sui/anonymous_pay.md#sui_anonymous_pay_ENoCoins">ENoCoins</a>);
<b>let</b> <b>mut</b> self = coins.pop_back();
<a href="../sui/anonymous_pay.md#sui_anonymous_pay_join_vec">join_vec</a>(&<b>mut</b> self, coins);
<a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(self, receiver)
}
</code></pre>



</details>

---
title: Module `sui::anonymous_balance`
---

A storable handler for Balances in general. Is used in the <code>Coin</code>
module to allow balance operations and can be used to implement
custom coins with <code><a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a></code> and <code>Balance</code>s.


-  [Struct `Supply`](#sui_anonymous_balance_Supply)
-  [Struct `Anonymous_Balance`](#sui_anonymous_balance_Anonymous_Balance)
-  [Constants](#@Constants_0)
-  [Function `create_by_value`](#sui_anonymous_balance_create_by_value)
-  [Function `create_by_value1_and_value2`](#sui_anonymous_balance_create_by_value1_and_value2)
-  [Function `value1`](#sui_anonymous_balance_value1)
-  [Function `value2`](#sui_anonymous_balance_value2)
-  [Function `get_encode_data`](#sui_anonymous_balance_get_encode_data)
-  [Function `supply_value`](#sui_anonymous_balance_supply_value)
-  [Function `create_supply`](#sui_anonymous_balance_create_supply)
-  [Function `increase_supply`](#sui_anonymous_balance_increase_supply)
-  [Function `zero`](#sui_anonymous_balance_zero)
-  [Function `update_encode_data`](#sui_anonymous_balance_update_encode_data)
-  [Function `join`](#sui_anonymous_balance_join)
-  [Function `compare`](#sui_anonymous_balance_compare)
-  [Function `split`](#sui_anonymous_balance_split)
-  [Function `split_anonymous`](#sui_anonymous_balance_split_anonymous)
-  [Function `destroy_zero`](#sui_anonymous_balance_destroy_zero)
-  [Function `destroy_supply`](#sui_anonymous_balance_destroy_supply)


<pre><code><b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/hfe_ops.md#sui_hfe_ops">sui::hfe_ops</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="sui_anonymous_balance_Supply"></a>

## Struct `Supply`

A Supply of T. Used for minting and burning.
Wrapped into a <code>TreasuryCap</code> in the <code>Coin</code> module.


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a>&lt;<b>phantom</b> T&gt; <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_anonymous_balance_Anonymous_Balance"></a>

## Struct `Anonymous_Balance`

Storable balance - an inner struct of a Coin type.
Can be use
d to store coins which don't need the key ability.


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance_type: u32</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>encode_data: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="sui_anonymous_balance_ENonZero"></a>

For when trying to destroy a non-zero balance.


<pre><code><b>const</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_ENonZero">ENonZero</a>: u64 = 0;
</code></pre>



<a name="sui_anonymous_balance_EOverflow"></a>

For when an overflow is happening on Supply operations.


<pre><code><b>const</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_EOverflow">EOverflow</a>: u64 = 1;
</code></pre>



<a name="sui_anonymous_balance_ENotEnough"></a>

For when trying to withdraw more than there is.


<pre><code><b>const</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_ENotEnough">ENotEnough</a>: u64 = 2;
</code></pre>



<a name="sui_anonymous_balance_DEFAULT_COMPARE_RESULT_LESS_THAN"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_DEFAULT_COMPARE_RESULT_LESS_THAN">DEFAULT_COMPARE_RESULT_LESS_THAN</a>: u8 = 2;
</code></pre>



<a name="sui_anonymous_balance_BALANCE_TYPE_SHARING"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_BALANCE_TYPE_SHARING">BALANCE_TYPE_SHARING</a>: u32 = 1;
</code></pre>



<a name="sui_anonymous_balance_create_by_value"></a>

## Function `create_by_value`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value">create_by_value</a>&lt;T&gt;(value: u64, owner: <b>address</b>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value">create_by_value</a>&lt;T&gt;(value: u64, owner: <b>address</b>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>let</b> <b>mut</b> encode_data = b"";
    <b>let</b> balance_type = <a href="../sui/anonymous_balance.md#sui_anonymous_balance_BALANCE_TYPE_SHARING">BALANCE_TYPE_SHARING</a>;
    <b>let</b> version = 0;
    <b>let</b> (<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>)   = hfe_ops_encode_data(value, owner);
    vector::append(&<b>mut</b> encode_data, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>);
    vector::append(&<b>mut</b> encode_data, b",");
    vector::append(&<b>mut</b> encode_data, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>);
    <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>,
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>,
        encode_data,
        balance_type: balance_type,
        version: version
    }
}
</code></pre>



</details>

<a name="sui_anonymous_balance_create_by_value1_and_value2"></a>

## Function `create_by_value1_and_value2`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value1_and_value2">create_by_value1_and_value2</a>&lt;T&gt;(<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: vector&lt;u8&gt;, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: vector&lt;u8&gt;): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value1_and_value2">create_by_value1_and_value2</a>&lt;T&gt;(<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: vector&lt;u8&gt;, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: vector&lt;u8&gt;): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>let</b> <b>mut</b> encode_data = b"";
    <b>let</b> balance_type = <a href="../sui/anonymous_balance.md#sui_anonymous_balance_BALANCE_TYPE_SHARING">BALANCE_TYPE_SHARING</a>;
    <b>let</b>  version = 0;
    vector::append(&<b>mut</b> encode_data, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>);
    vector::append(&<b>mut</b> encode_data, b",");
    vector::append(&<b>mut</b> encode_data, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>);
    <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>,
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>,
        encode_data,
        balance_type: balance_type,
        version: version
    }
}
</code></pre>



</details>

<a name="sui_anonymous_balance_value1"></a>

## Function `value1`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>&lt;T&gt;(self: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>&lt;T&gt;(self: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;): vector&lt;u8&gt; {
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>
}
</code></pre>



</details>

<a name="sui_anonymous_balance_value2"></a>

## Function `value2`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>&lt;T&gt;(self: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>&lt;T&gt;(self: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;): vector&lt;u8&gt; {
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>
}
</code></pre>



</details>

<a name="sui_anonymous_balance_get_encode_data"></a>

## Function `get_encode_data`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_get_encode_data">get_encode_data</a>&lt;T&gt;(self: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_get_encode_data">get_encode_data</a>&lt;T&gt;(self: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;): vector&lt;u8&gt; {
    self.encode_data
}
</code></pre>



</details>

<a name="sui_anonymous_balance_supply_value"></a>

## Function `supply_value`

Get the <code><a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a></code> value.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a>&lt;T&gt;): u64 {
    supply.value
}
</code></pre>



</details>

<a name="sui_anonymous_balance_create_supply"></a>

## Function `create_supply`

Create a new supply for type T.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a>&lt;T&gt; {
    <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a> { value: 0 }
}
</code></pre>



</details>

<a name="sui_anonymous_balance_increase_supply"></a>

## Function `increase_supply`

Increase supply by <code>value</code> and create a new <code>Balance&lt;T&gt;</code> with this value.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;, value: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a>&lt;T&gt;, value: u64, ctx: &<b>mut</b> TxContext): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>assert</b>!(value &lt; (18446744073709551615u64 - self.value), <a href="../sui/anonymous_balance.md#sui_anonymous_balance_EOverflow">EOverflow</a>);
    self.value = self.value + value;
    <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value">create_by_value</a>(value, ctx.sender())
}
</code></pre>



</details>

<a name="sui_anonymous_balance_zero"></a>

## Function `zero`

Create a zero <code>Balance</code> for type <code>T</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_zero">zero</a>&lt;T&gt;(ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_zero">zero</a>&lt;T&gt;(ctx: &TxContext): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value">create_by_value</a>(0, ctx.sender())
}
</code></pre>



</details>

<a name="sui_anonymous_balance_update_encode_data"></a>

## Function `update_encode_data`



<pre><code><b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_update_encode_data">update_encode_data</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_update_encode_data">update_encode_data</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;) {
    // Update the encode data based on the current <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a> and <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>
    <b>let</b> <b>mut</b> encode_data = b"";
    vector::append(&<b>mut</b> encode_data, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>);
    vector::append(&<b>mut</b> encode_data, b",");
    vector::append(&<b>mut</b> encode_data, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>);
    self.encode_data = encode_data;
}
</code></pre>



</details>

<a name="sui_anonymous_balance_join"></a>

## Function `join`

Join two balances together.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, <a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, owner: <b>address</b>): (vector&lt;u8&gt;, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, <a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, owner: <b>address</b>): (vector&lt;u8&gt;, vector&lt;u8&gt;) {
    <b>let</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        encode_data: _,
        version: _,
        balance_type: _,
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>,
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>
    } = <a href="../sui/balance.md#sui_balance">balance</a>;
    <b>let</b> (val0, val1) = hfe_ops_add(self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>,
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, owner);
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a> = val0;
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a> = val1;
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_update_encode_data">update_encode_data</a>();
    (self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>)
}
</code></pre>



</details>

<a name="sui_anonymous_balance_compare"></a>

## Function `compare`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_compare">compare</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, value: u64, owner: <b>address</b>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_compare">compare</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, value: u64, owner: <b>address</b>): u8 {
    hfe_ops_compare_value(self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, value, owner)
}
</code></pre>



</details>

<a name="sui_anonymous_balance_split"></a>

## Function `split`

Split a <code>Balance</code> and take a sub balance from it.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, value: u64, owner: <b>address</b>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, value: u64, owner: <b>address</b>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>let</b> compare_result: u8 = hfe_ops_compare_value(self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, value, owner);
    <b>assert</b>!(compare_result != <a href="../sui/anonymous_balance.md#sui_anonymous_balance_DEFAULT_COMPARE_RESULT_LESS_THAN">DEFAULT_COMPARE_RESULT_LESS_THAN</a>, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_ENotEnough">ENotEnough</a>);
    <b>let</b> anonymous_coin_value = <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value">create_by_value</a>(value, owner);
    <b>let</b> (val0, val1) = hfe_ops_minus(self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>,
        anonymous_coin_value.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, anonymous_coin_value.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, owner);
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a> = val0;
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a> = val1;
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_update_encode_data">update_encode_data</a>();
    anonymous_coin_value
}
</code></pre>



</details>

<a name="sui_anonymous_balance_split_anonymous"></a>

## Function `split_anonymous`

Split a <code>Balance</code> and take a sub balance from it.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_split_anonymous">split_anonymous</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: vector&lt;u8&gt;, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: vector&lt;u8&gt;, owner: <b>address</b>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_split_anonymous">split_anonymous</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;,  <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: vector&lt;u8&gt;,  <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: vector&lt;u8&gt;, owner: <b>address</b>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>let</b> compare_result: u8 = hfe_ops_compare_value1_and_value2(self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, owner);
    <b>assert</b>!(compare_result != <a href="../sui/anonymous_balance.md#sui_anonymous_balance_DEFAULT_COMPARE_RESULT_LESS_THAN">DEFAULT_COMPARE_RESULT_LESS_THAN</a>, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_ENotEnough">ENotEnough</a>);
    <b>let</b> anonymous_coin_value = <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_by_value1_and_value2">create_by_value1_and_value2</a>(<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>);
    <b>let</b> (val0, val1) = hfe_ops_minus(self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>,
        anonymous_coin_value.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, anonymous_coin_value.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, owner);
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a> = val0;
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a> = val1;
    self.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_update_encode_data">update_encode_data</a>();
    anonymous_coin_value
}
</code></pre>



</details>

<a name="sui_anonymous_balance_destroy_zero"></a>

## Function `destroy_zero`

Destroy a zero <code>Balance</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, owner: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, owner: <b>address</b>) {
    <b>let</b> compare_result = hfe_ops_compare_value(<a href="../sui/balance.md#sui_balance">balance</a>.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>, <a href="../sui/balance.md#sui_balance">balance</a>.<a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>, 0, owner);
    <b>assert</b>!(compare_result == 0, <a href="../sui/anonymous_balance.md#sui_anonymous_balance_ENonZero">ENonZero</a>);
    <b>let</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        encode_data: _,
        version: _,
        balance_type: _,
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value1">value1</a>: _,
        <a href="../sui/anonymous_balance.md#sui_anonymous_balance_value2">value2</a>: _,
    } = <a href="../sui/balance.md#sui_balance">balance</a>;
}
</code></pre>



</details>

<a name="sui_anonymous_balance_destroy_supply"></a>

## Function `destroy_supply`

Destroy a <code><a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a></code> preventing any further minting and burning.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a>&lt;T&gt;): u64 {
    <b>let</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">Supply</a> { value } = self;
    value
}
</code></pre>



</details>

---
title: Module `0x2::anonymous_balance`
---

A storable handler for Balances in general. Is used in the <code>Coin</code>
module to allow balance operations and can be used to implement
custom coins with <code><a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a></code> and <code>Balance</code>s.


-  [Struct `Supply`](#0x2_anonymous_balance_Supply)
-  [Struct `Anonymous_Balance`](#0x2_anonymous_balance_Anonymous_Balance)
-  [Constants](#@Constants_0)
-  [Function `get_anonymous_value`](#0x2_anonymous_balance_get_anonymous_value)
-  [Function `convert_to_string`](#0x2_anonymous_balance_convert_to_string)
-  [Function `create_by_value`](#0x2_anonymous_balance_create_by_value)
-  [Function `value`](#0x2_anonymous_balance_value)
-  [Function `value1`](#0x2_anonymous_balance_value1)
-  [Function `value2`](#0x2_anonymous_balance_value2)
-  [Function `get_encode_data`](#0x2_anonymous_balance_get_encode_data)
-  [Function `supply_value`](#0x2_anonymous_balance_supply_value)
-  [Function `create_supply`](#0x2_anonymous_balance_create_supply)
-  [Function `increase_supply`](#0x2_anonymous_balance_increase_supply)
-  [Function `decrease_supply`](#0x2_anonymous_balance_decrease_supply)
-  [Function `zero`](#0x2_anonymous_balance_zero)
-  [Function `update_encode_data`](#0x2_anonymous_balance_update_encode_data)
-  [Function `join`](#0x2_anonymous_balance_join)
-  [Function `split`](#0x2_anonymous_balance_split)
-  [Function `destroy_zero`](#0x2_anonymous_balance_destroy_zero)
-  [Function `destroy_supply`](#0x2_anonymous_balance_destroy_supply)


<pre><code><b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/hfe_ops.md#0x2_hfe_ops">0x2::hfe_ops</a>;
</code></pre>



<a name="0x2_anonymous_balance_Supply"></a>

## Struct `Supply`

Sender is not @0x0 the system address.
System operation performed for a coin other than SUI
A Supply of T. Used for minting and burning.
Wrapped into a <code>TreasuryCap</code> in the <code>Coin</code> module.


<pre><code><b>struct</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a>&lt;T&gt; <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_anonymous_balance_Anonymous_Balance"></a>

## Struct `Anonymous_Balance`

Storable balance - an inner struct of a Coin type.
Can be use
d to store coins which don't need the key ability.


<pre><code><b>struct</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; <b>has</b> store
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
<code>value1: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>value2: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>encode_data: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
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


<a name="0x2_anonymous_balance_ENonZero"></a>

For when trying to destroy a non-zero balance.


<pre><code><b>const</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_ENonZero">ENonZero</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x2_anonymous_balance_ENotEnough"></a>

For when trying to withdraw more than there is.


<pre><code><b>const</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_ENotEnough">ENotEnough</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x2_anonymous_balance_EOverflow"></a>

For when an overflow is happening on Supply operations.


<pre><code><b>const</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_EOverflow">EOverflow</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x2_anonymous_balance_BALANCE_TYPE_SHARING"></a>



<pre><code><b>const</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_BALANCE_TYPE_SHARING">BALANCE_TYPE_SHARING</a>: u32 = 1;
</code></pre>



<a name="0x2_anonymous_balance_get_anonymous_value"></a>

## Function `get_anonymous_value`



<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_get_anonymous_value">get_anonymous_value</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_get_anonymous_value">get_anonymous_value</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    //todo : <b>use</b> <a href="../sui-framework/hfe_ops.md#0x2_hfe_ops">hfe_ops</a> <b>to</b> get the value from value1 and value2
    hfe_ops_restore_value(self.value1, self.value2, signatures, id, publickey)
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_convert_to_string"></a>

## Function `convert_to_string`



<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_convert_to_string">convert_to_string</a>(value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_convert_to_string">convert_to_string</a>(<b>mut</b> value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
       <b>if</b> (value == 0) {
           <b>return</b> <a href="../move-stdlib/string.md#0x1_string_utf8">string::utf8</a>(b"0").into_bytes()
       };
       <b>let</b> <b>mut</b> buffer = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;u8&gt;();
       <b>while</b> (value != 0) {
           <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> buffer, ((48 + value % 10) <b>as</b> u8));
           value = value / 10;
       };
       <a href="../move-stdlib/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> buffer);
       <a href="../move-stdlib/string.md#0x1_string_utf8">string::utf8</a>(buffer).into_bytes()
   }
</code></pre>



</details>

<a name="0x2_anonymous_balance_create_by_value"></a>

## Function `create_by_value`



<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_create_by_value">create_by_value</a>&lt;T&gt;(value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_create_by_value">create_by_value</a>&lt;T&gt;(value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) : <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>let</b> <b>mut</b> encode_data = <a href="../move-stdlib/string.md#0x1_string_utf8">string::utf8</a>(b"");
    <b>let</b> balance_type = <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_BALANCE_TYPE_SHARING">BALANCE_TYPE_SHARING</a>;
    <b>let</b> version = 0;

    //todo: <b>use</b> <a href="../sui-framework/hfe_ops.md#0x2_hfe_ops">hfe_ops</a> <b>to</b> split the value into two parts
    // <b>let</b> value1 = value/2;
    // <b>let</b> value2 = value - value1;

    <b>let</b> result =  hfe_ops_split_value(value);
    <b>let</b> value1 = result[0];
    <b>let</b> value2 = result[1];

    <a href="../move-stdlib/string.md#0x1_string_append_utf8">string::append_utf8</a>(&<b>mut</b> encode_data, <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_convert_to_string">convert_to_string</a>(value1));
    <a href="../move-stdlib/string.md#0x1_string_append_utf8">string::append_utf8</a>(&<b>mut</b> encode_data, b",");
    <a href="../move-stdlib/string.md#0x1_string_append_utf8">string::append_utf8</a>(&<b>mut</b> encode_data, <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_convert_to_string">convert_to_string</a>(value2));



    <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        value1: value1,
        value2: value2,
        encode_data,
        balance_type: balance_type,
        version:version }
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_value"></a>

## Function `value`

Get the amount stored in a <code>Balance</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_value">value</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_value">value</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_get_anonymous_value">get_anonymous_value</a>(signatures, id, publickey)
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_value1"></a>

## Function `value1`



<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_value1">value1</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_value1">value1</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.value1
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_value2"></a>

## Function `value2`



<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_value2">value2</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_value2">value2</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.value2
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_get_encode_data"></a>

## Function `get_encode_data`



<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_get_encode_data">get_encode_data</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;): <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_get_encode_data">get_encode_data</a>&lt;T&gt;(self: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;): String {
    self.encode_data
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_supply_value"></a>

## Function `supply_value`

Get the <code><a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a></code> value.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">anonymous_balance::Supply</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_supply_value">supply_value</a>&lt;T&gt;(supply: &<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    supply.value
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_create_supply"></a>

## Function `create_supply`

Create a new supply for type T.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">anonymous_balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_create_supply">create_supply</a>&lt;T: drop&gt;(_: T): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a>&lt;T&gt; {
    <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a> { value: 0 }
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_increase_supply"></a>

## Function `increase_supply`

Increase supply by <code>value</code> and create a new <code>Balance&lt;T&gt;</code> with this value.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">anonymous_balance::Supply</a>&lt;T&gt;, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_increase_supply">increase_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a>&lt;T&gt;, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>assert</b>!(<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_value">value</a> &lt; (18446744073709551615u64 - self.value), <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_EOverflow">EOverflow</a>);
    self.value = self.value + value;
    <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_create_by_value">create_by_value</a>(value)
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_decrease_supply"></a>

## Function `decrease_supply`

Burn a Balance<T> and decrease Supply<T>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_decrease_supply">decrease_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">anonymous_balance::Supply</a>&lt;T&gt;, <a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_decrease_supply">decrease_supply</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a>&lt;T&gt;, <a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        encode_data: _,
        version: _,
        balance_type: _,
        value1: value1,
        value2: value2} = <a href="../sui-framework/balance.md#0x2_balance">balance</a>;
    <b>let</b> value = hfe_ops_restore_value(value1, value2, signatures, id, publickey);
    <b>assert</b>!(self.value &gt;= value, <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_EOverflow">EOverflow</a>);
    self.value = self.value - value;
    value
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_zero"></a>

## Function `zero`

Create a zero <code>Balance</code> for type <code>T</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_zero">zero</a>&lt;T&gt;(): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_zero">zero</a>&lt;T&gt;(): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_create_by_value">create_by_value</a>(0)
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_update_encode_data"></a>

## Function `update_encode_data`



<pre><code><b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_update_encode_data">update_encode_data</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_update_encode_data">update_encode_data</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;) {
    // Update the encode data based on the current value1 and value2
    <b>let</b> <b>mut</b> encode_data = <a href="../move-stdlib/string.md#0x1_string_utf8">string::utf8</a>(b"");
    <a href="../move-stdlib/string.md#0x1_string_append_utf8">string::append_utf8</a>(&<b>mut</b> encode_data, <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_convert_to_string">convert_to_string</a>(self.value1));
    <a href="../move-stdlib/string.md#0x1_string_append_utf8">string::append_utf8</a>(&<b>mut</b> encode_data, b",");
    <a href="../move-stdlib/string.md#0x1_string_append_utf8">string::append_utf8</a>(&<b>mut</b> encode_data, <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_convert_to_string">convert_to_string</a>(self.value2));
    self.encode_data = encode_data;
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_join"></a>

## Function `join`

Join two balances together.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, <a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, <a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        encode_data: _,
        version: _,
        balance_type: _,
        value1: value1,
        value2: value2} = <a href="../sui-framework/balance.md#0x2_balance">balance</a>;

    <b>let</b> result =  hfe_ops_add(self.value1, self.value2, value1, value2);
    self.value1 = result[0];
    self.value2 = result[1];


    self.<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_update_encode_data">update_encode_data</a>();
    (self.value1,self.value2)
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_split"></a>

## Function `split`

Split a <code>Balance</code> and take a sub balance from it.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt; {
    <b>let</b> compare_result = hfe_ops_compare_value(self.value1, self.value2, value);
    <b>assert</b>!(compare_result != 2, <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_ENotEnough">ENotEnough</a>);
    <b>let</b> value3 = value/2;
    <b>let</b> value4 = value - value3;
    <b>let</b> result = hfe_ops_minus(self.value1, self.value2, value3, value4);
    self.value1 = result[0];
    self.value2 = result[1];

    self.<a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_update_encode_data">update_encode_data</a>();

    <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_create_by_value">create_by_value</a>(value)
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_destroy_zero"></a>

## Function `destroy_zero`

Withdraw all balance. After this the remaining balance must be 0.
Destroy a zero <code>Balance</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_destroy_zero">destroy_zero</a>&lt;T&gt;(<a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a>&lt;T&gt;, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, id: <b>address</b>, publickey: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {

    <b>let</b> value = hfe_ops_restore_value(<a href="../sui-framework/balance.md#0x2_balance">balance</a>.value1, <a href="../sui-framework/balance.md#0x2_balance">balance</a>.value2, signatures, id, publickey);
    <b>assert</b>!(value == 0, <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_ENonZero">ENonZero</a>);
    <b>let</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymous_Balance">Anonymous_Balance</a> {
        encode_data: _,
        version: _,
        balance_type: _,
        value1: _,
        value2: _,
        } = <a href="../sui-framework/balance.md#0x2_balance">balance</a>;
}
</code></pre>



</details>

<a name="0x2_anonymous_balance_destroy_supply"></a>

## Function `destroy_supply`

Destroy a <code><a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a></code> preventing any further minting and burning.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">anonymous_balance::Supply</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_destroy_supply">destroy_supply</a>&lt;T&gt;(self: <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a>&lt;T&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Supply">Supply</a> { value } = self;
    value
}
</code></pre>



</details>

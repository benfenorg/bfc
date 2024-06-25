---
title: Module `0xc8::utils`
---



-  [Function `to_string`](#0xc8_utils_to_string)
-  [Function `cmp`](#0xc8_utils_cmp)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="comparator.md#0xc8_comparator">0xc8::comparator</a>;
</code></pre>



<a name="0xc8_utils_to_string"></a>

## Function `to_string`



<pre><code><b>public</b> <b>fun</b> <a href="utils.md#0xc8_utils_to_string">to_string</a>(value: u128): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="utils.md#0xc8_utils_to_string">to_string</a>(<b>mut</b> value: u128): String {
    <b>if</b> (value == 0) {
        <b>return</b> <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"0")
    };
    <b>let</b> <b>mut</b> buffer = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;u8&gt;();
    <b>while</b> (value != 0) {
        <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> buffer, ((48 + value % 10) <b>as</b> u8));
        value = value / 10;
    };
    <a href="../move-stdlib/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> buffer);
    <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(buffer)
}
</code></pre>



</details>

<a name="0xc8_utils_cmp"></a>

## Function `cmp`

0: x < y  1: x = y  2: x > y


<pre><code><b>public</b> <b>fun</b> <a href="utils.md#0xc8_utils_cmp">cmp</a>&lt;X, Y&gt;(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="utils.md#0xc8_utils_cmp">cmp</a>&lt;X, Y&gt;(): u8 {
    <b>let</b> comp = <a href="comparator.md#0xc8_comparator_compare">comparator::compare</a>(&get&lt;X&gt;(), &get&lt;Y&gt;());
    <b>if</b> (<a href="comparator.md#0xc8_comparator_is_equal">comparator::is_equal</a>(&comp)) {
        1
    } <b>else</b> <b>if</b> (<a href="comparator.md#0xc8_comparator_is_smaller_than">comparator::is_smaller_than</a>(&comp)) {
        0
    } <b>else</b> {
        2
    }
}
</code></pre>



</details>

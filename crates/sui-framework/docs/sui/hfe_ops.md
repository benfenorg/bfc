---
title: Module `sui::hfe_ops`
---



-  [Function `hfe_ops_add`](#sui_hfe_ops_hfe_ops_add)
-  [Function `hfe_ops_minus`](#sui_hfe_ops_hfe_ops_minus)
-  [Function `hfe_ops_multiplied`](#sui_hfe_ops_hfe_ops_multiplied)
-  [Function `hfe_ops_compare_value`](#sui_hfe_ops_hfe_ops_compare_value)
-  [Function `hfe_ops_encode_data`](#sui_hfe_ops_hfe_ops_encode_data)
-  [Function `hfe_ops_restore_value`](#sui_hfe_ops_hfe_ops_restore_value)


<pre><code></code></pre>



<a name="sui_hfe_ops_hfe_ops_add"></a>

## Function `hfe_ops_add`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_add">hfe_ops_add</a>(input_1: vector&lt;u8&gt;, input_2: vector&lt;u8&gt;, input_3: vector&lt;u8&gt;, input_4: vector&lt;u8&gt;): (vector&lt;u8&gt;, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_add">hfe_ops_add</a>(
    input_1: vector&lt;u8&gt;,
    input_2: vector&lt;u8&gt;,
    input_3: vector&lt;u8&gt;,
    input_4: vector&lt;u8&gt;,
): (vector&lt;u8&gt;, vector&lt;u8&gt;);
</code></pre>



</details>

<a name="sui_hfe_ops_hfe_ops_minus"></a>

## Function `hfe_ops_minus`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_minus">hfe_ops_minus</a>(input_1: vector&lt;u8&gt;, input_2: vector&lt;u8&gt;, input_3: vector&lt;u8&gt;, input_4: vector&lt;u8&gt;): (vector&lt;u8&gt;, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_minus">hfe_ops_minus</a>(
    input_1: vector&lt;u8&gt;,
    input_2: vector&lt;u8&gt;,
    input_3: vector&lt;u8&gt;,
    input_4: vector&lt;u8&gt;,
): (vector&lt;u8&gt;, vector&lt;u8&gt;);
</code></pre>



</details>

<a name="sui_hfe_ops_hfe_ops_multiplied"></a>

## Function `hfe_ops_multiplied`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_multiplied">hfe_ops_multiplied</a>(input_1: vector&lt;u8&gt;, input_2: vector&lt;u8&gt;, input_3: vector&lt;u8&gt;, input_4: vector&lt;u8&gt;): (vector&lt;u8&gt;, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_multiplied">hfe_ops_multiplied</a>(
    input_1: vector&lt;u8&gt;,
    input_2: vector&lt;u8&gt;,
    input_3: vector&lt;u8&gt;,
    input_4: vector&lt;u8&gt;,
): (vector&lt;u8&gt;, vector&lt;u8&gt;);
</code></pre>



</details>

<a name="sui_hfe_ops_hfe_ops_compare_value"></a>

## Function `hfe_ops_compare_value`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_compare_value">hfe_ops_compare_value</a>(input_1: vector&lt;u8&gt;, input_2: vector&lt;u8&gt;, input_3: u64): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_compare_value">hfe_ops_compare_value</a>(
    input_1: vector&lt;u8&gt;,
    input_2: vector&lt;u8&gt;,
    input_3: u64,
): u8;
</code></pre>



</details>

<a name="sui_hfe_ops_hfe_ops_encode_data"></a>

## Function `hfe_ops_encode_data`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_encode_data">hfe_ops_encode_data</a>(value: u64): (vector&lt;u8&gt;, vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_encode_data">hfe_ops_encode_data</a>(
    value: u64,
): (vector&lt;u8&gt;, vector&lt;u8&gt;);
</code></pre>



</details>

<a name="sui_hfe_ops_hfe_ops_restore_value"></a>

## Function `hfe_ops_restore_value`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_restore_value">hfe_ops_restore_value</a>(value1: vector&lt;u8&gt;, value2: vector&lt;u8&gt;, signature: vector&lt;u8&gt;, id: <b>address</b>, publickey: vector&lt;u8&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="../sui/hfe_ops.md#sui_hfe_ops_hfe_ops_restore_value">hfe_ops_restore_value</a>(
    value1: vector&lt;u8&gt;,
    value2: vector&lt;u8&gt;,
    signature: vector&lt;u8&gt;,
    id: <b>address</b>,
    publickey: vector&lt;u8&gt;,
): u64;
</code></pre>



</details>

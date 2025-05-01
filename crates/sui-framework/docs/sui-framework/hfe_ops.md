---
title: Module `0x2::hfe_ops`
---



-  [Function `hfe_ops_add`](#0x2_hfe_ops_hfe_ops_add)
-  [Function `split_data`](#0x2_hfe_ops_split_data)


<pre><code></code></pre>



<a name="0x2_hfe_ops_hfe_ops_add"></a>

## Function `hfe_ops_add`



<pre><code><b>public</b> <b>fun</b> <a href="hfe_ops.md#0x2_hfe_ops_hfe_ops_add">hfe_ops_add</a>(input_1: u8, input_2: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="hfe_ops.md#0x2_hfe_ops_hfe_ops_add">hfe_ops_add</a>(
    input_1: u8,
    input_2: u8,
): u8;
</code></pre>



</details>

<a name="0x2_hfe_ops_split_data"></a>

## Function `split_data`



<pre><code><b>public</b> <b>fun</b> <a href="hfe_ops.md#0x2_hfe_ops_split_data">split_data</a>(data: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, n: u8, index: u8): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="hfe_ops.md#0x2_hfe_ops_split_data">split_data</a>(
    data: &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    n: u8,
    index: u8,
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



</details>

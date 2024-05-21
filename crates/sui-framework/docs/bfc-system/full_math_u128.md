---
title: Module `0xc8::full_math_u128`
---



-  [Function `mul_div_floor`](#0xc8_full_math_u128_mul_div_floor)
-  [Function `mul_div_round`](#0xc8_full_math_u128_mul_div_round)
-  [Function `mul_div_ceil`](#0xc8_full_math_u128_mul_div_ceil)
-  [Function `mul_shr`](#0xc8_full_math_u128_mul_shr)
-  [Function `mul_shl`](#0xc8_full_math_u128_mul_shl)
-  [Function `full_mul`](#0xc8_full_math_u128_full_mul)


<pre><code></code></pre>



<a name="0xc8_full_math_u128_mul_div_floor"></a>

## Function `mul_div_floor`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_div_floor">mul_div_floor</a>(num1: u128, num2: u128, denom: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_div_floor">mul_div_floor</a>(num1: u128, num2: u128, denom: u128): u128 {
    <b>let</b> r = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_mul</a>(num1, num2) / (denom <b>as</b> u256);
    (r <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_full_math_u128_mul_div_round"></a>

## Function `mul_div_round`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_div_round">mul_div_round</a>(num1: u128, num2: u128, denom: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_div_round">mul_div_round</a>(num1: u128, num2: u128, denom: u128): u128 {
    <b>let</b> r = (<a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_mul</a>(num1, num2) + ((denom <b>as</b> u256) &gt;&gt; 1)) / (denom <b>as</b> u256);
    (r <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_full_math_u128_mul_div_ceil"></a>

## Function `mul_div_ceil`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_div_ceil">mul_div_ceil</a>(num1: u128, num2: u128, denom: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_div_ceil">mul_div_ceil</a>(num1: u128, num2: u128, denom: u128): u128 {
    <b>let</b> r = (<a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_mul</a>(num1, num2) + ((denom <b>as</b> u256) - 1)) / (denom <b>as</b> u256);
    (r <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_full_math_u128_mul_shr"></a>

## Function `mul_shr`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">mul_shr</a>(num1: u128, num2: u128, shift: u8): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">mul_shr</a>(num1: u128, num2: u128, shift: u8): u128 {
    <b>let</b> product = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_mul</a>(num1, num2) &gt;&gt; shift;
    (product <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_full_math_u128_mul_shl"></a>

## Function `mul_shl`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shl">mul_shl</a>(num1: u128, num2: u128, shift: u8): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shl">mul_shl</a>(num1: u128, num2: u128, shift: u8): u128 {
    <b>let</b> product = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_mul</a>(num1, num2) &lt;&lt; shift;
    (product <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_full_math_u128_full_mul"></a>

## Function `full_mul`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_mul</a>(num1: u128, num2: u128): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_mul</a>(num1: u128, num2: u128): u256 {
    (num1 <b>as</b> u256) * (num2 <b>as</b> u256)
}
</code></pre>



</details>

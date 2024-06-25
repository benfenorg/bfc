---
title: Module `0xc8::full_math_u64`
---



-  [Function `mul_div_floor`](#0xc8_full_math_u64_mul_div_floor)
-  [Function `mul_div_round`](#0xc8_full_math_u64_mul_div_round)
-  [Function `mul_div_ceil`](#0xc8_full_math_u64_mul_div_ceil)
-  [Function `mul_shr`](#0xc8_full_math_u64_mul_shr)
-  [Function `mul_shl`](#0xc8_full_math_u64_mul_shl)
-  [Function `full_mul`](#0xc8_full_math_u64_full_mul)


<pre><code></code></pre>



<a name="0xc8_full_math_u64_mul_div_floor"></a>

## Function `mul_div_floor`



<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_div_floor">mul_div_floor</a>(num1: u64, num2: u64, denom: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_div_floor">mul_div_floor</a>(num1: u64, num2: u64, denom: u64): u64 {
    <b>let</b> r = <a href="full_math_u64.md#0xc8_full_math_u64_full_mul">full_mul</a>(num1, num2) / (denom <b>as</b> u128);
    (r <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_full_math_u64_mul_div_round"></a>

## Function `mul_div_round`



<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_div_round">mul_div_round</a>(num1: u64, num2: u64, denom: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_div_round">mul_div_round</a>(num1: u64, num2: u64, denom: u64): u64 {
    <b>let</b> r = (<a href="full_math_u64.md#0xc8_full_math_u64_full_mul">full_mul</a>(num1, num2) + ((denom <b>as</b> u128) &gt;&gt; 1)) / (denom <b>as</b> u128);
    (r <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_full_math_u64_mul_div_ceil"></a>

## Function `mul_div_ceil`



<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_div_ceil">mul_div_ceil</a>(num1: u64, num2: u64, denom: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_div_ceil">mul_div_ceil</a>(num1: u64, num2: u64, denom: u64): u64 {
    <b>let</b> r = (<a href="full_math_u64.md#0xc8_full_math_u64_full_mul">full_mul</a>(num1, num2) + ((denom <b>as</b> u128) - 1)) / (denom <b>as</b> u128);
    (r <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_full_math_u64_mul_shr"></a>

## Function `mul_shr`



<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_shr">mul_shr</a>(num1: u64, num2: u64, shift: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_shr">mul_shr</a>(num1: u64, num2: u64, shift: u8): u64 {
    <b>let</b> r = <a href="full_math_u64.md#0xc8_full_math_u64_full_mul">full_mul</a>(num1, num2) &gt;&gt; shift;
    (r <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_full_math_u64_mul_shl"></a>

## Function `mul_shl`



<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_shl">mul_shl</a>(num1: u64, num2: u64, shift: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_mul_shl">mul_shl</a>(num1: u64, num2: u64, shift: u8): u64 {
    <b>let</b> r = <a href="full_math_u64.md#0xc8_full_math_u64_full_mul">full_mul</a>(num1, num2) &lt;&lt; shift;
    (r <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_full_math_u64_full_mul"></a>

## Function `full_mul`



<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_full_mul">full_mul</a>(num1: u64, num2: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="full_math_u64.md#0xc8_full_math_u64_full_mul">full_mul</a>(num1: u64, num2: u64): u128 {
    ((num1 <b>as</b> u128) * (num2 <b>as</b> u128))
}
</code></pre>



</details>

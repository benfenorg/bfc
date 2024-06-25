---
title: Module `0xc8::math_u64`
---



-  [Constants](#@Constants_0)
-  [Function `wrapping_add`](#0xc8_math_u64_wrapping_add)
-  [Function `overflowing_add`](#0xc8_math_u64_overflowing_add)
-  [Function `wrapping_sub`](#0xc8_math_u64_wrapping_sub)
-  [Function `overflowing_sub`](#0xc8_math_u64_overflowing_sub)
-  [Function `wrapping_mul`](#0xc8_math_u64_wrapping_mul)
-  [Function `overflowing_mul`](#0xc8_math_u64_overflowing_mul)
-  [Function `carry_add`](#0xc8_math_u64_carry_add)
-  [Function `add_check`](#0xc8_math_u64_add_check)


<pre><code></code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0xc8_math_u64_HI_64_MASK"></a>



<pre><code><b>const</b> <a href="math_u64.md#0xc8_math_u64_HI_64_MASK">HI_64_MASK</a>: u128 = 340282366920938463444927863358058659840;
</code></pre>



<a name="0xc8_math_u64_LO_64_MASK"></a>



<pre><code><b>const</b> <a href="math_u64.md#0xc8_math_u64_LO_64_MASK">LO_64_MASK</a>: u128 = 18446744073709551615;
</code></pre>



<a name="0xc8_math_u64_MAX_U64"></a>



<pre><code><b>const</b> <a href="math_u64.md#0xc8_math_u64_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="0xc8_math_u64_wrapping_add"></a>

## Function `wrapping_add`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_wrapping_add">wrapping_add</a>(n1: u64, n2: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_wrapping_add">wrapping_add</a>(n1: u64, n2: u64): u64 {
    <b>let</b> (sum, _) = <a href="math_u64.md#0xc8_math_u64_overflowing_add">overflowing_add</a>(n1, n2);
    sum
}
</code></pre>



</details>

<a name="0xc8_math_u64_overflowing_add"></a>

## Function `overflowing_add`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_overflowing_add">overflowing_add</a>(n1: u64, n2: u64): (u64, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_overflowing_add">overflowing_add</a>(n1: u64, n2: u64): (u64, bool) {
    <b>let</b> sum = (n1 <b>as</b> u128) + (n2 <b>as</b> u128);
    <b>if</b> (sum &gt; (<a href="math_u64.md#0xc8_math_u64_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
        (((sum & <a href="math_u64.md#0xc8_math_u64_LO_64_MASK">LO_64_MASK</a>) <b>as</b> u64), <b>true</b>)
    } <b>else</b> {
        ((sum <b>as</b> u64), <b>false</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_math_u64_wrapping_sub"></a>

## Function `wrapping_sub`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_wrapping_sub">wrapping_sub</a>(n1: u64, n2: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_wrapping_sub">wrapping_sub</a>(n1: u64, n2: u64): u64 {
    <b>let</b> (result, _) = <a href="math_u64.md#0xc8_math_u64_overflowing_sub">overflowing_sub</a>(n1, n2);
    result
}
</code></pre>



</details>

<a name="0xc8_math_u64_overflowing_sub"></a>

## Function `overflowing_sub`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_overflowing_sub">overflowing_sub</a>(n1: u64, n2: u64): (u64, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_overflowing_sub">overflowing_sub</a>(n1: u64, n2: u64): (u64, bool) {
    <b>if</b> (n1 &gt;= n2) {
        ((n1 - n2), <b>false</b>)
    } <b>else</b> {
        ((<a href="math_u64.md#0xc8_math_u64_MAX_U64">MAX_U64</a> - n2 + n1 + 1), <b>true</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_math_u64_wrapping_mul"></a>

## Function `wrapping_mul`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_wrapping_mul">wrapping_mul</a>(n1: u64, n2: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_wrapping_mul">wrapping_mul</a>(n1: u64, n2: u64): u64 {
    <b>let</b> (m, _) = <a href="math_u64.md#0xc8_math_u64_overflowing_mul">overflowing_mul</a>(n1, n2);
    m
}
</code></pre>



</details>

<a name="0xc8_math_u64_overflowing_mul"></a>

## Function `overflowing_mul`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_overflowing_mul">overflowing_mul</a>(n1: u64, n2: u64): (u64, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_overflowing_mul">overflowing_mul</a>(n1: u64, n2: u64): (u64, bool) {
    <b>let</b> m = (n1 <b>as</b> u128) * (n2 <b>as</b> u128);
    (((m & <a href="math_u64.md#0xc8_math_u64_LO_64_MASK">LO_64_MASK</a>) <b>as</b> u64), (m & <a href="math_u64.md#0xc8_math_u64_HI_64_MASK">HI_64_MASK</a>) &gt; 0)
}
</code></pre>



</details>

<a name="0xc8_math_u64_carry_add"></a>

## Function `carry_add`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_carry_add">carry_add</a>(n1: u64, n2: u64, carry: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_carry_add">carry_add</a>(n1: u64, n2: u64, carry: u64): (u64, u64) {
    <b>assert</b>!(carry &lt;= 1, 0);
    <b>let</b> sum = (n1 <b>as</b> u128) + (n2 <b>as</b> u128) + (carry <b>as</b> u128);
    <b>if</b> (sum &gt; <a href="math_u64.md#0xc8_math_u64_LO_64_MASK">LO_64_MASK</a>) {
        (((sum & <a href="math_u64.md#0xc8_math_u64_LO_64_MASK">LO_64_MASK</a>) <b>as</b> u64), 1)
    } <b>else</b> {
        ((sum <b>as</b> u64), 0)
    }
}
</code></pre>



</details>

<a name="0xc8_math_u64_add_check"></a>

## Function `add_check`



<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_add_check">add_check</a>(n1: u64, n2: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u64.md#0xc8_math_u64_add_check">add_check</a>(n1: u64, n2: u64): bool {
    (<a href="math_u64.md#0xc8_math_u64_MAX_U64">MAX_U64</a> - n1 &gt;= n2)
}
</code></pre>



</details>

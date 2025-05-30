---
title: Module `0xc8::math_u128`
---



-  [Constants](#@Constants_0)
-  [Function `wrapping_add`](#0xc8_math_u128_wrapping_add)
-  [Function `overflowing_add`](#0xc8_math_u128_overflowing_add)
-  [Function `wrapping_sub`](#0xc8_math_u128_wrapping_sub)
-  [Function `overflowing_sub`](#0xc8_math_u128_overflowing_sub)
-  [Function `wrapping_mul`](#0xc8_math_u128_wrapping_mul)
-  [Function `overflowing_mul`](#0xc8_math_u128_overflowing_mul)
-  [Function `full_mul`](#0xc8_math_u128_full_mul)
-  [Function `hi`](#0xc8_math_u128_hi)
-  [Function `lo`](#0xc8_math_u128_lo)
-  [Function `hi_u128`](#0xc8_math_u128_hi_u128)
-  [Function `lo_u128`](#0xc8_math_u128_lo_u128)
-  [Function `from_lo_hi`](#0xc8_math_u128_from_lo_hi)
-  [Function `checked_div_round`](#0xc8_math_u128_checked_div_round)
-  [Function `max`](#0xc8_math_u128_max)
-  [Function `min`](#0xc8_math_u128_min)
-  [Function `add_check`](#0xc8_math_u128_add_check)


<pre><code></code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0xc8_math_u128_DIV_BY_ZERO"></a>



<pre><code><b>const</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_DIV_BY_ZERO">DIV_BY_ZERO</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xc8_math_u128_HI_64_MASK"></a>



<pre><code><b>const</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_HI_64_MASK">HI_64_MASK</a>: u128 = 340282366920938463444927863358058659840;
</code></pre>



<a name="0xc8_math_u128_LO_128_MASK"></a>



<pre><code><b>const</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_LO_128_MASK">LO_128_MASK</a>: u256 = 340282366920938463463374607431768211455;
</code></pre>



<a name="0xc8_math_u128_LO_64_MASK"></a>



<pre><code><b>const</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_LO_64_MASK">LO_64_MASK</a>: u128 = 18446744073709551615;
</code></pre>



<a name="0xc8_math_u128_MAX_U128"></a>



<pre><code><b>const</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_MAX_U128">MAX_U128</a>: u128 = 340282366920938463463374607431768211455;
</code></pre>



<a name="0xc8_math_u128_wrapping_add"></a>

## Function `wrapping_add`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_wrapping_add">wrapping_add</a>(n1: u128, n2: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_wrapping_add">wrapping_add</a>(n1: u128, n2: u128): u128 {
    <b>let</b> (sum, _) = <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_add">overflowing_add</a>(n1, n2);
    sum
}
</code></pre>



</details>

<a name="0xc8_math_u128_overflowing_add"></a>

## Function `overflowing_add`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_add">overflowing_add</a>(n1: u128, n2: u128): (u128, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_add">overflowing_add</a>(n1: u128, n2: u128): (u128, bool) {
    <b>let</b> sum = (n1 <b>as</b> u256) + (n2 <b>as</b> u256);
    <b>if</b> (sum &gt; (<a href="../bfc-system/math_u128.md#0xc8_math_u128_MAX_U128">MAX_U128</a> <b>as</b> u256)) {
        (((sum & <a href="../bfc-system/math_u128.md#0xc8_math_u128_LO_128_MASK">LO_128_MASK</a>) <b>as</b> u128), <b>true</b>)
    } <b>else</b> {
        ((sum <b>as</b> u128), <b>false</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_math_u128_wrapping_sub"></a>

## Function `wrapping_sub`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_wrapping_sub">wrapping_sub</a>(n1: u128, n2: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_wrapping_sub">wrapping_sub</a>(n1: u128, n2: u128): u128 {
    <b>let</b> (result, _) = <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_sub">overflowing_sub</a>(n1, n2);
    result
}
</code></pre>



</details>

<a name="0xc8_math_u128_overflowing_sub"></a>

## Function `overflowing_sub`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_sub">overflowing_sub</a>(n1: u128, n2: u128): (u128, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_sub">overflowing_sub</a>(n1: u128, n2: u128): (u128, bool) {
    <b>if</b> (n1 &gt;= n2) {
        ((n1 - n2), <b>false</b>)
    } <b>else</b> {
        ((<a href="../bfc-system/math_u128.md#0xc8_math_u128_MAX_U128">MAX_U128</a> - n2 + n1 + 1), <b>true</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_math_u128_wrapping_mul"></a>

## Function `wrapping_mul`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_wrapping_mul">wrapping_mul</a>(n1: u128, n2: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_wrapping_mul">wrapping_mul</a>(n1: u128, n2: u128): u128 {
    <b>let</b> (m, _) = <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_mul">overflowing_mul</a>(n1, n2);
    m
}
</code></pre>



</details>

<a name="0xc8_math_u128_overflowing_mul"></a>

## Function `overflowing_mul`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_mul">overflowing_mul</a>(n1: u128, n2: u128): (u128, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_overflowing_mul">overflowing_mul</a>(n1: u128, n2: u128): (u128, bool) {
    <b>let</b> (c0, c1) = <a href="../bfc-system/math_u128.md#0xc8_math_u128_full_mul">full_mul</a>(n1, n2);
    <b>if</b> (c1 &gt; 0) {
        (c0, <b>true</b>)
    } <b>else</b> {
        (c0, <b>false</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_math_u128_full_mul"></a>

## Function `full_mul`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_full_mul">full_mul</a>(n1: u128, n2: u128): (u128, u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_full_mul">full_mul</a>(n1: u128, n2: u128): (u128, u128) {
    <b>let</b> hi_mask: u256 = 0xffffffffffffffffffffffffffffffff00000000000000000000000000000000;
    <b>let</b> lo_mask: u256 = 0x00000000000000000000000000000000ffffffffffffffffffffffffffffffff;
    <b>let</b> r = (n1 <b>as</b> u256) * (n2 <b>as</b> u256);
    <b>let</b> hi = (((r & hi_mask) &gt;&gt; 128) <b>as</b> u128);
    <b>let</b> lo = ((r & lo_mask) <b>as</b> u128);
    (lo, hi)
}
</code></pre>



</details>

<a name="0xc8_math_u128_hi"></a>

## Function `hi`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_hi">hi</a>(n: u128): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_hi">hi</a>(n: u128): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    (((n & <a href="../bfc-system/math_u128.md#0xc8_math_u128_HI_64_MASK">HI_64_MASK</a>) &gt;&gt; 64) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
}
</code></pre>



</details>

<a name="0xc8_math_u128_lo"></a>

## Function `lo`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_lo">lo</a>(n: u128): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_lo">lo</a>(n: u128): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    ((n & <a href="../bfc-system/math_u128.md#0xc8_math_u128_LO_64_MASK">LO_64_MASK</a>) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
}
</code></pre>



</details>

<a name="0xc8_math_u128_hi_u128"></a>

## Function `hi_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_hi_u128">hi_u128</a>(n: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_hi_u128">hi_u128</a>(n: u128): u128 {
    (n & <a href="../bfc-system/math_u128.md#0xc8_math_u128_HI_64_MASK">HI_64_MASK</a>) &gt;&gt; 64
}
</code></pre>



</details>

<a name="0xc8_math_u128_lo_u128"></a>

## Function `lo_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_lo_u128">lo_u128</a>(n: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_lo_u128">lo_u128</a>(n: u128): u128 {
    (n & <a href="../bfc-system/math_u128.md#0xc8_math_u128_LO_64_MASK">LO_64_MASK</a>)
}
</code></pre>



</details>

<a name="0xc8_math_u128_from_lo_hi"></a>

## Function `from_lo_hi`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_from_lo_hi">from_lo_hi</a>(lo: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, hi: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_from_lo_hi">from_lo_hi</a>(lo: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, hi: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): u128 {
    ((hi <b>as</b> u128) &lt;&lt; 64) + (lo <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_math_u128_checked_div_round"></a>

## Function `checked_div_round`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_checked_div_round">checked_div_round</a>(num: u128, denom: u128, round_up: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_checked_div_round">checked_div_round</a>(num: u128, denom: u128, round_up: bool): u128 {
    <b>if</b> (denom == 0) {
        <b>abort</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_DIV_BY_ZERO">DIV_BY_ZERO</a>
    };
    <b>let</b> quotient = num / denom;
    <b>let</b> remainer = num % denom;
    <b>if</b> (round_up && (remainer &gt; 0)) {
        <b>return</b> (quotient + 1)
    };
    quotient
}
</code></pre>



</details>

<a name="0xc8_math_u128_max"></a>

## Function `max`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_max">max</a>(num1: u128, num2: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_max">max</a>(num1: u128, num2: u128): u128 {
    <b>if</b> (num1 &gt; num2) {
        num1
    } <b>else</b> {
        num2
    }
}
</code></pre>



</details>

<a name="0xc8_math_u128_min"></a>

## Function `min`



<pre><code><b>public</b> <b>fun</b> <b>min</b>(num1: u128, num2: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <b>min</b>(num1: u128, num2: u128): u128 {
    <b>if</b> (num1 &lt; num2) {
        num1
    } <b>else</b> {
        num2
    }
}
</code></pre>



</details>

<a name="0xc8_math_u128_add_check"></a>

## Function `add_check`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_add_check">add_check</a>(num1: u128, num2: u128): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128_add_check">add_check</a>(num1: u128, num2: u128): bool {
    (<a href="../bfc-system/math_u128.md#0xc8_math_u128_MAX_U128">MAX_U128</a> - num1 &gt;= num2)
}
</code></pre>



</details>

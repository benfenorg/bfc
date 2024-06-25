---
title: Module `0xc8::math_u256`
---



-  [Constants](#@Constants_0)
-  [Function `div_mod`](#0xc8_math_u256_div_mod)
-  [Function `shlw`](#0xc8_math_u256_shlw)
-  [Function `shrw`](#0xc8_math_u256_shrw)
-  [Function `checked_shlw`](#0xc8_math_u256_checked_shlw)
-  [Function `div_round`](#0xc8_math_u256_div_round)
-  [Function `add_check`](#0xc8_math_u256_add_check)


<pre><code></code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0xc8_math_u256_MAX_U256"></a>



<pre><code><b>const</b> <a href="math_u256.md#0xc8_math_u256_MAX_U256">MAX_U256</a>: u256 = 115792089237316195423570985008687907853269984665640564039457584007913129639935;
</code></pre>



<a name="0xc8_math_u256_div_mod"></a>

## Function `div_mod`



<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_div_mod">div_mod</a>(num: u256, denom: u256): (u256, u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_div_mod">div_mod</a>(num: u256, denom: u256): (u256, u256) {
    <b>let</b> p = num / denom;
    <b>let</b> r: u256 = num - (p * denom);
    (p, r)
}
</code></pre>



</details>

<a name="0xc8_math_u256_shlw"></a>

## Function `shlw`



<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_shlw">shlw</a>(n: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_shlw">shlw</a>(n: u256): u256 {
    n &lt;&lt; 64
}
</code></pre>



</details>

<a name="0xc8_math_u256_shrw"></a>

## Function `shrw`



<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_shrw">shrw</a>(n: u256): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_shrw">shrw</a>(n: u256): u256 {
    n &gt;&gt; 64
}
</code></pre>



</details>

<a name="0xc8_math_u256_checked_shlw"></a>

## Function `checked_shlw`



<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_checked_shlw">checked_shlw</a>(n: u256): (u256, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_checked_shlw">checked_shlw</a>(n: u256): (u256, bool) {
    <b>let</b> mask = 0xffffffffffffffff &lt;&lt; 192;
    <b>if</b> (n &gt; mask) {
        (0, <b>true</b>)
    } <b>else</b> {
        ((n &lt;&lt; 64), <b>false</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_math_u256_div_round"></a>

## Function `div_round`



<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_div_round">div_round</a>(num: u256, denom: u256, round_up: bool): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_div_round">div_round</a>(num: u256, denom: u256, round_up: bool): u256 {
    <b>let</b> p = num / denom;
    <b>if</b> (round_up && ((p * denom) != num)) {
        p + 1
    } <b>else</b> {
        p
    }
}
</code></pre>



</details>

<a name="0xc8_math_u256_add_check"></a>

## Function `add_check`



<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_add_check">add_check</a>(num1: u256, num2: u256): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="math_u256.md#0xc8_math_u256_add_check">add_check</a>(num1: u256, num2: u256): bool {
    (<a href="math_u256.md#0xc8_math_u256_MAX_U256">MAX_U256</a> - num1 &gt;= num2)
}
</code></pre>



</details>

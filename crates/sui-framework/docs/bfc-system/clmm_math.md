---
title: Module `0xc8::clmm_math`
---



-  [Constants](#@Constants_0)
-  [Function `get_liquidity_from_a`](#0xc8_clmm_math_get_liquidity_from_a)
-  [Function `get_liquidity_from_b`](#0xc8_clmm_math_get_liquidity_from_b)
-  [Function `get_delta_a`](#0xc8_clmm_math_get_delta_a)
-  [Function `get_delta_b`](#0xc8_clmm_math_get_delta_b)
-  [Function `get_next_sqrt_price_a_up`](#0xc8_clmm_math_get_next_sqrt_price_a_up)
-  [Function `get_next_sqrt_price_b_down`](#0xc8_clmm_math_get_next_sqrt_price_b_down)
-  [Function `get_next_sqrt_price_from_input`](#0xc8_clmm_math_get_next_sqrt_price_from_input)
-  [Function `get_next_sqrt_price_from_output`](#0xc8_clmm_math_get_next_sqrt_price_from_output)
-  [Function `get_delta_up_from_input`](#0xc8_clmm_math_get_delta_up_from_input)
-  [Function `get_delta_down_from_output`](#0xc8_clmm_math_get_delta_down_from_output)
-  [Function `compute_swap_step`](#0xc8_clmm_math_compute_swap_step)
-  [Function `get_amount_by_liquidity`](#0xc8_clmm_math_get_amount_by_liquidity)
-  [Function `get_liquidity_by_amount`](#0xc8_clmm_math_get_liquidity_by_amount)


<pre><code><b>use</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128">0xc8::full_math_u128</a>;
<b>use</b> <a href="../bfc-system/i32.md#0xc8_i32">0xc8::i32</a>;
<b>use</b> <a href="../bfc-system/math_u128.md#0xc8_math_u128">0xc8::math_u128</a>;
<b>use</b> <a href="../bfc-system/math_u256.md#0xc8_math_u256">0xc8::math_u256</a>;
<b>use</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math">0xc8::tick_math</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0xc8_clmm_math_EINVALID_FIXED_TOKEN_TYPE"></a>



<pre><code><b>const</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EINVALID_FIXED_TOKEN_TYPE">EINVALID_FIXED_TOKEN_TYPE</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1005;
</code></pre>



<a name="0xc8_clmm_math_EINVALID_SQRT_PRICE_INPUT"></a>



<pre><code><b>const</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EINVALID_SQRT_PRICE_INPUT">EINVALID_SQRT_PRICE_INPUT</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1004;
</code></pre>



<a name="0xc8_clmm_math_EMULTIPLICATION_OVERFLOW"></a>



<pre><code><b>const</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EMULTIPLICATION_OVERFLOW">EMULTIPLICATION_OVERFLOW</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1002;
</code></pre>



<a name="0xc8_clmm_math_ETOKEN_AMOUNT_MAX_EXCEEDED"></a>



<pre><code><b>const</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_ETOKEN_AMOUNT_MAX_EXCEEDED">ETOKEN_AMOUNT_MAX_EXCEEDED</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000;
</code></pre>



<a name="0xc8_clmm_math_ETOKEN_AMOUNT_MIN_SUBCEEDED"></a>



<pre><code><b>const</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_ETOKEN_AMOUNT_MIN_SUBCEEDED">ETOKEN_AMOUNT_MIN_SUBCEEDED</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1001;
</code></pre>



<a name="0xc8_clmm_math_get_liquidity_from_a"></a>

## Function `get_liquidity_from_a`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(sqrt_price_0: u128, sqrt_price_1: u128, amount_a: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, round_up: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    amount_a: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    round_up: bool
): u128 {
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    <b>let</b> numberator = (<a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price_0, sqrt_price_1) &gt;&gt; 64) * (amount_a <b>as</b> u256);
    <b>let</b> div_res = <a href="../bfc-system/math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, (sqrt_price_diff <b>as</b> u256), round_up);
    (div_res <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_liquidity_from_b"></a>

## Function `get_liquidity_from_b`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(sqrt_price_0: u128, sqrt_price_1: u128, amount_b: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, round_up: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    amount_b: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    round_up: bool
): u128 {
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    <b>let</b> div_res = <a href="../bfc-system/math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(
        ((amount_b <b>as</b> u256) &lt;&lt; 64),
        (sqrt_price_diff <b>as</b> u256),
        round_up
    );
    (div_res <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_a"></a>

## Function `get_delta_a`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(sqrt_price_0: u128, sqrt_price_1: u128, liquidity: u128, round_up: bool): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    liquidity: u128,
    round_up: bool
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        <b>return</b> 0
    };
    <b>let</b> (numberator, overflowing) = <a href="../bfc-system/math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(
        <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff)
    );
    <b>if</b> (overflowing) {
        <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EMULTIPLICATION_OVERFLOW">EMULTIPLICATION_OVERFLOW</a>
    };
    <b>let</b> denominator = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price_0, sqrt_price_1);
    <b>let</b> quotient = <a href="../bfc-system/math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, denominator, round_up);
    (quotient <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_b"></a>

## Function `get_delta_b`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(sqrt_price_0: u128, sqrt_price_1: u128, liquidity: u128, round_up: bool): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    liquidity: u128,
    round_up: bool
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        <b>return</b> 0
    };
    <b>let</b> lo64_mask = 0x000000000000000000000000000000000000000000000000ffffffffffffffff;
    <b>let</b> product = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff);
    <b>let</b> should_round_up = (round_up) && ((product & lo64_mask) &gt; 0);
    <b>if</b> (should_round_up) {
        <b>return</b> (((product &gt;&gt; 64) + 1) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
    };
    ((product &gt;&gt; 64) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_a_up"></a>

## Function `get_next_sqrt_price_a_up`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(sqrt_price: u128, liquidity: u128, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, by_amount_input: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    by_amount_input: bool,
): u128 {
    <b>if</b> (amount == 0) {
        <b>return</b> sqrt_price
    };
    <b>let</b> (numberator, overflowing) = <a href="../bfc-system/math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(
        <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price, liquidity)
    );
    <b>if</b> (overflowing) {
        <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EMULTIPLICATION_OVERFLOW">EMULTIPLICATION_OVERFLOW</a>
    };

    <b>let</b> liquidity_shl_64 = (liquidity <b>as</b> u256) &lt;&lt; 64;
    <b>let</b> product = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price, (amount <b>as</b> u128));
    <b>let</b> new_sqrt_price = <b>if</b> (by_amount_input) {
        (<a href="../bfc-system/math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, (liquidity_shl_64 + product), <b>true</b>) <b>as</b> u128)
    } <b>else</b> {
        (<a href="../bfc-system/math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, (liquidity_shl_64 - product), <b>true</b>) <b>as</b> u128)
    };

    <b>if</b> (new_sqrt_price &gt; <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_sqrt_price">tick_math::max_sqrt_price</a>()) {
        <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_ETOKEN_AMOUNT_MAX_EXCEEDED">ETOKEN_AMOUNT_MAX_EXCEEDED</a>
    } <b>else</b> <b>if</b> (new_sqrt_price &lt; <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_sqrt_price">tick_math::min_sqrt_price</a>()) {
        <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_ETOKEN_AMOUNT_MIN_SUBCEEDED">ETOKEN_AMOUNT_MIN_SUBCEEDED</a>
    };

    new_sqrt_price
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_b_down"></a>

## Function `get_next_sqrt_price_b_down`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(sqrt_price: u128, liquidity: u128, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, by_amount_input: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    by_amount_input: bool,
): u128 {
    <b>let</b> delta_sqrt_price = <a href="../bfc-system/math_u128.md#0xc8_math_u128_checked_div_round">math_u128::checked_div_round</a>(((amount <b>as</b> u128) &lt;&lt; 64), liquidity, !by_amount_input);
    <b>let</b> new_sqrt_price = <b>if</b> (by_amount_input) {
        sqrt_price + delta_sqrt_price
    } <b>else</b> {
        sqrt_price - delta_sqrt_price
    };

    <b>if</b> (new_sqrt_price &gt; <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_sqrt_price">tick_math::max_sqrt_price</a>()) {
        <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_ETOKEN_AMOUNT_MAX_EXCEEDED">ETOKEN_AMOUNT_MAX_EXCEEDED</a>
    } <b>else</b> <b>if</b> (new_sqrt_price &lt; <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_sqrt_price">tick_math::min_sqrt_price</a>()) {
        <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_ETOKEN_AMOUNT_MIN_SUBCEEDED">ETOKEN_AMOUNT_MIN_SUBCEEDED</a>
    };

    new_sqrt_price
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_from_input"></a>

## Function `get_next_sqrt_price_from_input`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_input">get_next_sqrt_price_from_input</a>(sqrt_price: u128, liquidity: u128, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, a_to_b: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_input">get_next_sqrt_price_from_input</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    a_to_b: bool,
): u128 {
    <b>if</b> (a_to_b) {
        <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(sqrt_price, liquidity, amount, <b>true</b>)
    } <b>else</b> {
        <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(sqrt_price, liquidity, amount, <b>true</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_from_output"></a>

## Function `get_next_sqrt_price_from_output`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_output">get_next_sqrt_price_from_output</a>(sqrt_price: u128, liquidity: u128, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, a_to_b: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_output">get_next_sqrt_price_from_output</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    a_to_b: bool,
): u128 {
    <b>if</b> (a_to_b) {
        <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(sqrt_price, liquidity, amount, <b>false</b>)
    } <b>else</b> {
        <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(sqrt_price, liquidity, amount, <b>false</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_up_from_input"></a>

## Function `get_delta_up_from_input`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(current_sqrt_price: u128, target_sqrt_price: u128, liquidity: u128, a_to_b: bool): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(
    current_sqrt_price: u128,
    target_sqrt_price: u128,
    liquidity: u128,
    a_to_b: bool,
): u256 {
    <b>let</b> sqrt_price_diff = <b>if</b> (current_sqrt_price &gt; target_sqrt_price) {
        current_sqrt_price - target_sqrt_price
    } <b>else</b> {
        target_sqrt_price - current_sqrt_price
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        <b>return</b> 0
    };
    <b>if</b> (a_to_b) {
        <b>let</b> (numberator, overflowing) = <a href="../bfc-system/math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(
            <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff)
        );
        <b>if</b> (overflowing) {
            <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EMULTIPLICATION_OVERFLOW">EMULTIPLICATION_OVERFLOW</a>
        };
        <b>let</b> denominator = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(current_sqrt_price, target_sqrt_price);
        <a href="../bfc-system/math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, denominator, <b>true</b>)
    } <b>else</b> {
        <b>let</b> product = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff);
        <b>let</b> lo64_mask = 0x000000000000000000000000000000000000000000000000ffffffffffffffff;
        <b>let</b> should_round_up = (product & lo64_mask) &gt; 0;
        <b>if</b> (should_round_up) {
            <b>return</b> (product &gt;&gt; 64) + 1
        };
        product &gt;&gt; 64
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_down_from_output"></a>

## Function `get_delta_down_from_output`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(current_sqrt_price: u128, target_sqrt_price: u128, liquidity: u128, a_to_b: bool): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(
    current_sqrt_price: u128,
    target_sqrt_price: u128,
    liquidity: u128,
    a_to_b: bool,
): u256 {
    <b>let</b> sqrt_price_diff = <b>if</b> (current_sqrt_price &gt; target_sqrt_price) {
        current_sqrt_price - target_sqrt_price
    } <b>else</b> {
        target_sqrt_price - current_sqrt_price
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        <b>return</b> 0
    };
    <b>if</b> (a_to_b) {
        <b>let</b> product = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff);
        product &gt;&gt; 64
    } <b>else</b> {
        <b>let</b> (numberator, overflowing) = <a href="../bfc-system/math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(
            <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff)
        );
        <b>if</b> (overflowing) {
            <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EMULTIPLICATION_OVERFLOW">EMULTIPLICATION_OVERFLOW</a>
        };
        <b>let</b> denominator = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(current_sqrt_price, target_sqrt_price);
        <a href="../bfc-system/math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, denominator, <b>false</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_compute_swap_step"></a>

## Function `compute_swap_step`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_compute_swap_step">compute_swap_step</a>(current_sqrt_price: u128, target_sqrt_price: u128, liquidity: u128, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, a2b: bool, by_amount_in: bool): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_compute_swap_step">compute_swap_step</a>(
    current_sqrt_price: u128,
    target_sqrt_price: u128,
    liquidity: u128,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    a2b: bool,
    by_amount_in: bool
): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, u128) {
    <b>let</b> <b>mut</b> next_sqrt_price = target_sqrt_price;
    <b>let</b> <b>mut</b> amount_in: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>let</b> <b>mut</b> amount_out: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>if</b> (liquidity == 0) {
        <b>return</b> (
            amount_in,
            amount_out,
            next_sqrt_price,
        )
    };
    <b>if</b> (a2b) {
        <b>assert</b>!(current_sqrt_price &gt;= target_sqrt_price, <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EINVALID_SQRT_PRICE_INPUT">EINVALID_SQRT_PRICE_INPUT</a>)
    } <b>else</b> {
        <b>assert</b>!(current_sqrt_price &lt; target_sqrt_price, <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EINVALID_SQRT_PRICE_INPUT">EINVALID_SQRT_PRICE_INPUT</a>)
    };

    <b>if</b> (by_amount_in) {
        <b>let</b> amount_remain = amount;
        <b>let</b> max_amount_in =
            <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(current_sqrt_price, target_sqrt_price, liquidity, a2b);
        <b>if</b> (max_amount_in &gt; (amount_remain <b>as</b> u256)) {
            amount_in = amount_remain;
            next_sqrt_price = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_input">get_next_sqrt_price_from_input</a>(
                current_sqrt_price,
                liquidity,
                amount_remain,
                a2b
            );
        } <b>else</b> {
            // it will never overflow here, because max_amount_in &lt; amount_remain and amount_remain's type is <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
            amount_in = (max_amount_in <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
            next_sqrt_price = target_sqrt_price;
        };
        amount_out = (<a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(current_sqrt_price, next_sqrt_price, liquidity, a2b) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
    } <b>else</b> {
        <b>let</b> max_amount_out = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(
            current_sqrt_price,
            target_sqrt_price,
            liquidity,
            a2b
        );
        <b>if</b> (max_amount_out &gt; (amount <b>as</b> u256)) {
            amount_out = amount;
            next_sqrt_price =
                <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_output">get_next_sqrt_price_from_output</a>(current_sqrt_price, liquidity, amount, a2b);
        } <b>else</b> {
            amount_out = (max_amount_out <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
            next_sqrt_price = target_sqrt_price;
        };
        amount_in = (<a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(current_sqrt_price, next_sqrt_price, liquidity, a2b) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
    };

    (
        amount_in,
        amount_out,
        next_sqrt_price
    )
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_amount_by_liquidity"></a>

## Function `get_amount_by_liquidity`

Get the coin amount by liquidity
Params
- tick_lower The liquidity's lower tick
- tick_upper The liquidity's upper tick
- current_tick_index
Returns
- amount_a
- amount_b


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_amount_by_liquidity">get_amount_by_liquidity</a>(tick_lower: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, tick_upper: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, current_tick_index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, current_sqrt_price: u128, liquidity: u128, round_up: bool): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_amount_by_liquidity">get_amount_by_liquidity</a>(
    tick_lower: I32,
    tick_upper: I32,
    current_tick_index: I32,
    current_sqrt_price: u128,
    liquidity: u128,
    round_up: bool
): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>if</b> (liquidity == 0) {
        <b>return</b> (0, 0)
    };
    <b>let</b> lower_price = <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(tick_lower);
    <b>let</b> upper_price = <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(tick_upper);
    // Only <a href="../sui-framework/coin.md#0x2_coin">coin</a> a

    <b>let</b> (amount_a, amount_b) = <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_lt">i32::lt</a>(current_tick_index, tick_lower)) {
        (<a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(lower_price, upper_price, liquidity, round_up), 0)
    } <b>else</b> <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_lt">i32::lt</a>(current_tick_index, tick_upper)) {
        (
            <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(current_sqrt_price, upper_price, liquidity, round_up),
            <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(lower_price, current_sqrt_price, liquidity, round_up)
        )
    } <b>else</b> {
        (0, <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(lower_price, upper_price, liquidity, round_up))
    };
    (amount_a, amount_b)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_liquidity_by_amount"></a>

## Function `get_liquidity_by_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_by_amount">get_liquidity_by_amount</a>(lower_index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, upper_index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, current_tick_index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, current_sqrt_price: u128, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, is_fixed_a: bool): (u128, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_by_amount">get_liquidity_by_amount</a>(
    lower_index: I32,
    upper_index: I32,
    current_tick_index: I32,
    current_sqrt_price: u128,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    is_fixed_a: bool
): (u128, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> lower_price = <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(lower_index);
    <b>let</b> upper_price = <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(upper_index);
    <b>let</b> <b>mut</b> amount_a: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>let</b> <b>mut</b> amount_b: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
    <b>let</b> <b>mut</b> _liquidity: u128 = 0;
    <b>if</b> (is_fixed_a) {
        amount_a = amount;
        <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_lt">i32::lt</a>(current_tick_index, lower_index)) {
            _liquidity = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(lower_price, upper_price, amount, <b>false</b>);
        }<b>else</b> <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_lt">i32::lt</a>(current_tick_index, upper_index)) {
            _liquidity = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(current_sqrt_price, upper_price, amount, <b>false</b>);
            amount_b = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(current_sqrt_price, lower_price, _liquidity, <b>true</b>);
        }<b>else</b> {
            <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EINVALID_FIXED_TOKEN_TYPE">EINVALID_FIXED_TOKEN_TYPE</a>
        };
    }<b>else</b> {
        amount_b = amount;
        <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_gte">i32::gte</a>(current_tick_index, upper_index)) {
            _liquidity = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(lower_price, upper_price, amount, <b>false</b>);
        }<b>else</b> <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_gte">i32::gte</a>(current_tick_index, lower_index)) {
            _liquidity = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(lower_price, current_sqrt_price, amount, <b>false</b>);
            amount_a = <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(current_sqrt_price, upper_price, _liquidity, <b>true</b>);
        }<b>else</b> {
            <b>abort</b> <a href="../bfc-system/clmm_math.md#0xc8_clmm_math_EINVALID_FIXED_TOKEN_TYPE">EINVALID_FIXED_TOKEN_TYPE</a>
        }
    };
    (_liquidity, amount_a, amount_b)
}
</code></pre>



</details>


<a name="0xc8_clmm_math"></a>

# Module `0xc8::clmm_math`



-  [Constants](#@Constants_0)
-  [Function `get_liquidity_from_a`](#0xc8_clmm_math_get_liquidity_from_a)
-  [Function `get_liquidity_from_b`](#0xc8_clmm_math_get_liquidity_from_b)
-  [Function `get_delta_a`](#0xc8_clmm_math_get_delta_a)
    -  [Formula](#@Formula_1)
    -  [Params](#@Params_2)
-  [Function `get_delta_b`](#0xc8_clmm_math_get_delta_b)
    -  [Formula](#@Formula_3)
    -  [Params](#@Params_4)
-  [Function `get_next_sqrt_price_a_up`](#0xc8_clmm_math_get_next_sqrt_price_a_up)
    -  [Formula](#@Formula_5)
    -  [Arguments](#@Arguments_6)
-  [Function `get_next_sqrt_price_b_down`](#0xc8_clmm_math_get_next_sqrt_price_b_down)
    -  [Formula](#@Formula_7)
    -  [Arguments](#@Arguments_8)
-  [Function `get_next_sqrt_price_from_input`](#0xc8_clmm_math_get_next_sqrt_price_from_input)
-  [Function `get_next_sqrt_price_from_output`](#0xc8_clmm_math_get_next_sqrt_price_from_output)
-  [Function `get_delta_up_from_input`](#0xc8_clmm_math_get_delta_up_from_input)
-  [Function `get_delta_down_from_output`](#0xc8_clmm_math_get_delta_down_from_output)
-  [Function `compute_swap_step`](#0xc8_clmm_math_compute_swap_step)
-  [Function `get_liquidity_by_amount`](#0xc8_clmm_math_get_liquidity_by_amount)
-  [Function `get_amount_by_liquidity`](#0xc8_clmm_math_get_amount_by_liquidity)


<pre><code><b>use</b> <a href="full_math_u128.md#0xc8_full_math_u128">0xc8::full_math_u128</a>;
<b>use</b> <a href="i32.md#0xc8_i32">0xc8::i32</a>;
<b>use</b> <a href="math_u128.md#0xc8_math_u128">0xc8::math_u128</a>;
<b>use</b> <a href="math_u256.md#0xc8_math_u256">0xc8::math_u256</a>;
<b>use</b> <a href="tick_math.md#0xc8_tick_math">0xc8::tick_math</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0xc8_clmm_math_EMATH_U256_CHECKED_SHLW_FAILED"></a>



<pre><code><b>const</b> <a href="clmm_math.md#0xc8_clmm_math_EMATH_U256_CHECKED_SHLW_FAILED">EMATH_U256_CHECKED_SHLW_FAILED</a>: u64 = 1000;
</code></pre>



<a name="0xc8_clmm_math_ETICK_EXCEED_MAXIMUM"></a>



<pre><code><b>const</b> <a href="clmm_math.md#0xc8_clmm_math_ETICK_EXCEED_MAXIMUM">ETICK_EXCEED_MAXIMUM</a>: u64 = 1003;
</code></pre>



<a name="0xc8_clmm_math_ETICK_LESS_MINIMUM"></a>



<pre><code><b>const</b> <a href="clmm_math.md#0xc8_clmm_math_ETICK_LESS_MINIMUM">ETICK_LESS_MINIMUM</a>: u64 = 1004;
</code></pre>



<a name="0xc8_clmm_math_ETICK_MATH_EXCEED_MAX_SQRT_PRICE"></a>



<pre><code><b>const</b> <a href="clmm_math.md#0xc8_clmm_math_ETICK_MATH_EXCEED_MAX_SQRT_PRICE">ETICK_MATH_EXCEED_MAX_SQRT_PRICE</a>: u64 = 1001;
</code></pre>



<a name="0xc8_clmm_math_ETICK_MATH_LESS_MIN_SQRT_PRICE"></a>



<pre><code><b>const</b> <a href="clmm_math.md#0xc8_clmm_math_ETICK_MATH_LESS_MIN_SQRT_PRICE">ETICK_MATH_LESS_MIN_SQRT_PRICE</a>: u64 = 1002;
</code></pre>



<a name="0xc8_clmm_math_get_liquidity_from_a"></a>

## Function `get_liquidity_from_a`

<code>liquidity = ( sqrt_price_upper * sqrt_price_lower * delta_a ) / delta_sqrt_price</code>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(sqrt_price_0: u128, sqrt_price_1: u128, amount_a: u64, round_up: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    amount_a: u64,
    round_up: bool
): u128
{
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    <b>let</b> v1 = <a href="math_u256.md#0xc8_math_u256_shrw">math_u256::shrw</a>(<a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price_0, sqrt_price_1));
    <b>let</b> v2 = (amount_a <b>as</b> u256) * (sqrt_price_diff <b>as</b> u256);
    (<a href="math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(v1, v2, round_up) <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_liquidity_from_b"></a>

## Function `get_liquidity_from_b`

<code>liquidity = delta_b / delta_sqrt_price</code>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(sqrt_price_0: u128, sqrt_price_1: u128, amount_b: u64, round_up: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    amount_b: u64,
    round_up: bool,
): u128
{
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    (<a href="math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(
        <a href="math_u256.md#0xc8_math_u256_shlw">math_u256::shlw</a>((amount_b <b>as</b> u256)),
        <a href="math_u256.md#0xc8_math_u256_shlw">math_u256::shlw</a>((sqrt_price_diff <b>as</b> u256)),
        round_up
    ) <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_a"></a>

## Function `get_delta_a`

Gets the amount_a delta between two prices, for given amount of liquidity

<a name="@Formula_1"></a>

### Formula

<code>delta_a = (liquidity * delta_sqrt_price) / (sqrt_price_upper * sqrt_price_lower)</code>

<a name="@Params_2"></a>

### Params

* <code>sqrt_price_0</code> - A sqrt price
* <code>sqrt_price_1</code> - Another sqrt price
* <code>liquidity</code> - The amount of usable liquidity
* <code>round_up</code>- Whether to round the amount up or down


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(sqrt_price_0: u128, sqrt_price_1: u128, liquidity: u128, round_up: bool): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    liquidity: u128,
    round_up: bool,
): u64
{
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        <b>return</b> 0
    };
    <b>let</b> (numberator, is_ok) = <a href="math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(<a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff));
    <b>assert</b>!(!is_ok, <a href="clmm_math.md#0xc8_clmm_math_EMATH_U256_CHECKED_SHLW_FAILED">EMATH_U256_CHECKED_SHLW_FAILED</a>);
    <b>let</b> denomminator = <a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price_0, sqrt_price_1);
    (<a href="math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, denomminator, round_up) <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_b"></a>

## Function `get_delta_b`

Gets the amount_b delta between two prices, for given amount of liquidity

<a name="@Formula_3"></a>

### Formula

* <code>delta_b = delta_sqrt_price * liquidity</code>

<a name="@Params_4"></a>

### Params

* <code>sqrt_price_0</code> - A sqrt price
* <code>sqrt_price_1</code> - Another sqrt price
* <code>liquidity</code> - The amount of usable liquidity
* <code>round_up</code>- Whether to round the amount up or down


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(sqrt_price_0: u128, sqrt_price_1: u128, liquidity: u128, round_up: bool): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(
    sqrt_price_0: u128,
    sqrt_price_1: u128,
    liquidity: u128,
    round_up: bool,
): u64
{
    <b>let</b> sqrt_price_diff = <b>if</b> (sqrt_price_0 &gt; sqrt_price_1) {
        sqrt_price_0 - sqrt_price_1
    } <b>else</b> {
        sqrt_price_1 - sqrt_price_0
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        <b>return</b> 0
    };
    <b>let</b> product = <a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff);
    <b>if</b> (round_up) {
        <b>if</b> (product & (<a href="math_u256.md#0xc8_math_u256_shlw">math_u256::shlw</a>(1) - 1) &gt; 0) {
            <b>return</b> (<a href="math_u256.md#0xc8_math_u256_shrw">math_u256::shrw</a>(product) + 1 <b>as</b> u64)
        };
    };
    (<a href="math_u256.md#0xc8_math_u256_shrw">math_u256::shrw</a>(product) <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_a_up"></a>

## Function `get_next_sqrt_price_a_up`

Gets the next sqrt price from given a delta of token_a

<a name="@Formula_5"></a>

### Formula

<code>sqrt_price_new = (sqrt_price * liquidity) / (liquidity +- amount * sqrt_price)</code>

<a name="@Arguments_6"></a>

### Arguments

* <code>sqrt_price</code> - The starting price <code>sqrt(P)</code>
* <code>liquidity</code> - The amount of usable liquidity L
* <code>amount</code> - Delta of token a
* <code>add</code> - Whether to add or remove the amount of token_a


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(sqrt_price: u128, liquidity: u128, amount: u64, by_amount_input: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: u64,
    by_amount_input: bool,
): u128
{
    <b>if</b> (amount == 0) {
        <b>return</b> sqrt_price
    };
    <b>let</b> (numberator, is_ok) = <a href="math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(<a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price, liquidity));
    <b>assert</b>!(!is_ok, <a href="clmm_math.md#0xc8_clmm_math_EMATH_U256_CHECKED_SHLW_FAILED">EMATH_U256_CHECKED_SHLW_FAILED</a>);
    <b>let</b> liquidity_shl_64 = <a href="math_u256.md#0xc8_math_u256_shlw">math_u256::shlw</a>((liquidity <b>as</b> u256));
    <b>let</b> product = <a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price, (amount <b>as</b> u128));
    <b>let</b> quotient = <b>if</b> (by_amount_input) {
        <a href="math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, product + liquidity_shl_64, <b>true</b>)
    } <b>else</b> {
        <a href="math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, liquidity_shl_64 - product, <b>true</b>)
    };
    <b>assert</b>!((quotient <b>as</b> u128) &lt;= <a href="tick_math.md#0xc8_tick_math_max_sqrt_price">tick_math::max_sqrt_price</a>(), <a href="clmm_math.md#0xc8_clmm_math_ETICK_MATH_EXCEED_MAX_SQRT_PRICE">ETICK_MATH_EXCEED_MAX_SQRT_PRICE</a>);
    <b>assert</b>!((quotient <b>as</b> u128) &gt;= <a href="tick_math.md#0xc8_tick_math_min_sqrt_price">tick_math::min_sqrt_price</a>(), <a href="clmm_math.md#0xc8_clmm_math_ETICK_MATH_LESS_MIN_SQRT_PRICE">ETICK_MATH_LESS_MIN_SQRT_PRICE</a>);
    (quotient <b>as</b> u128)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_b_down"></a>

## Function `get_next_sqrt_price_b_down`

Gets the next sqrt price given a delta of token_b

<a name="@Formula_7"></a>

### Formula

* <code>new_sqrt_price = sqrt_price + (delta_b / liquidity)</code>

<a name="@Arguments_8"></a>

### Arguments

* <code>sqrt_price</code> - The starting price <code>sqrt(P)</code>, i.e., before accounting for the token_1 delta
* <code>liquidity</code> - The amount of usable liquidity L
* <code>amount</code> - Delta of token 1 (dy) to add or remove from virtual reserves
* <code>add</code> - Whether to add or remove the amount of token_1


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(sqrt_price: u128, liquidity: u128, amount: u64, by_amount_input: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: u64,
    by_amount_input: bool,
): u128
{
    <b>let</b> delta_sqrt_price = <a href="math_u128.md#0xc8_math_u128_checked_div_round">math_u128::checked_div_round</a>((amount <b>as</b> u128) &lt;&lt; 64, liquidity, by_amount_input);
    <b>let</b> new_sqrt_price = <b>if</b> (by_amount_input) {
        sqrt_price + delta_sqrt_price
    } <b>else</b> {
        sqrt_price - delta_sqrt_price
    };
    <b>assert</b>!(new_sqrt_price &lt;= <a href="tick_math.md#0xc8_tick_math_max_sqrt_price">tick_math::max_sqrt_price</a>(), <a href="clmm_math.md#0xc8_clmm_math_ETICK_MATH_EXCEED_MAX_SQRT_PRICE">ETICK_MATH_EXCEED_MAX_SQRT_PRICE</a>);
    <b>assert</b>!(new_sqrt_price &gt;= <a href="tick_math.md#0xc8_tick_math_min_sqrt_price">tick_math::min_sqrt_price</a>(), <a href="clmm_math.md#0xc8_clmm_math_ETICK_MATH_LESS_MIN_SQRT_PRICE">ETICK_MATH_LESS_MIN_SQRT_PRICE</a>);
    new_sqrt_price
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_from_input"></a>

## Function `get_next_sqrt_price_from_input`



<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_input">get_next_sqrt_price_from_input</a>(sqrt_price: u128, liquidity: u128, amount: u64, a2b: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_input">get_next_sqrt_price_from_input</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: u64,
    a2b: bool,
): u128
{
    <b>if</b> (a2b) {
        <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(sqrt_price, liquidity, amount, <b>true</b>)
    } <b>else</b> {
        <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(sqrt_price, liquidity, amount, <b>true</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_next_sqrt_price_from_output"></a>

## Function `get_next_sqrt_price_from_output`



<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_output">get_next_sqrt_price_from_output</a>(sqrt_price: u128, liquidity: u128, amount: u64, a2b: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_output">get_next_sqrt_price_from_output</a>(
    sqrt_price: u128,
    liquidity: u128,
    amount: u64,
    a2b: bool,
): u128
{
    <b>if</b> (a2b) {
        <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_b_down">get_next_sqrt_price_b_down</a>(sqrt_price, liquidity, amount, <b>false</b>)
    } <b>else</b> {
        <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_a_up">get_next_sqrt_price_a_up</a>(sqrt_price, liquidity, amount, <b>false</b>)
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_up_from_input"></a>

## Function `get_delta_up_from_input`



<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(current_sqrt_price: u128, target_sqrt_price: u128, liquidity: u128, a2b: bool): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(
    current_sqrt_price: u128,
    target_sqrt_price: u128,
    liquidity: u128,
    a2b: bool,
): u256
{
    <b>let</b> sqrt_price_diff = <b>if</b> (current_sqrt_price &gt; target_sqrt_price) {
        current_sqrt_price - target_sqrt_price
    } <b>else</b> {
        target_sqrt_price - current_sqrt_price
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        0
    } <b>else</b> {
        <b>if</b> (a2b) {
            <b>let</b> (numberator, is_ok) = <a href="math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(<a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(sqrt_price_diff, liquidity));
            <b>assert</b>!(!is_ok, <a href="clmm_math.md#0xc8_clmm_math_EMATH_U256_CHECKED_SHLW_FAILED">EMATH_U256_CHECKED_SHLW_FAILED</a>);
            <b>let</b> denomminator = <a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(current_sqrt_price, target_sqrt_price);
            <a href="math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, denomminator, <b>true</b>)
        } <b>else</b> {
            <b>let</b> product = <a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff);
            <b>if</b> (product & (<a href="math_u256.md#0xc8_math_u256_shlw">math_u256::shlw</a>(1) - 1) &gt; 0) {
                <a href="math_u256.md#0xc8_math_u256_shrw">math_u256::shrw</a>(product) + 1
            } <b>else</b> {
                <a href="math_u256.md#0xc8_math_u256_shrw">math_u256::shrw</a>(product)
            }
        }
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_delta_down_from_output"></a>

## Function `get_delta_down_from_output`



<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(current_sqrt_price: u128, target_sqrt_price: u128, liquidity: u128, a2b: bool): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(
    current_sqrt_price: u128,
    target_sqrt_price: u128,
    liquidity: u128,
    a2b: bool,
): u256
{
    <b>let</b> sqrt_price_diff = <b>if</b> (current_sqrt_price &gt; target_sqrt_price) {
        current_sqrt_price - target_sqrt_price
    } <b>else</b> {
        target_sqrt_price - current_sqrt_price
    };
    <b>if</b> (sqrt_price_diff == 0 || liquidity == 0) {
        0
    } <b>else</b> {
        <b>if</b> (a2b) {
            <a href="math_u256.md#0xc8_math_u256_shrw">math_u256::shrw</a>(<a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff))
        } <b>else</b> {
            <b>let</b> (numberator, is_ok) = <a href="math_u256.md#0xc8_math_u256_checked_shlw">math_u256::checked_shlw</a>(<a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(liquidity, sqrt_price_diff));
            <b>assert</b>!(!is_ok, <a href="clmm_math.md#0xc8_clmm_math_EMATH_U256_CHECKED_SHLW_FAILED">EMATH_U256_CHECKED_SHLW_FAILED</a>);
            <b>let</b> denomminator = <a href="full_math_u128.md#0xc8_full_math_u128_full_mul">full_math_u128::full_mul</a>(current_sqrt_price, target_sqrt_price);
            <a href="math_u256.md#0xc8_math_u256_div_round">math_u256::div_round</a>(numberator, denomminator, <b>false</b>)
        }
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_compute_swap_step"></a>

## Function `compute_swap_step`



<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_compute_swap_step">compute_swap_step</a>(current_sqrt_price: u128, target_sqrt_price: u128, liquidity: u128, amount: u64, a2b: bool, by_amount_input: bool): (u64, u64, u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_compute_swap_step">compute_swap_step</a>(
    current_sqrt_price: u128,
    target_sqrt_price: u128,
    liquidity: u128,
    amount: u64,
    a2b: bool,
    by_amount_input: bool,
): (u64, u64, u128) {
    <b>if</b> (liquidity == 0) {
        (0, 0, target_sqrt_price)
    } <b>else</b> {
        <b>let</b> next_sqrt_price;
        <b>let</b> amount_in: u64;
        <b>let</b> amount_out: u64;
        <b>if</b> (by_amount_input) {
            <b>let</b> max_amount_in =
                <a href="clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(current_sqrt_price, target_sqrt_price, liquidity, a2b);
            <b>if</b> (max_amount_in &gt; (amount <b>as</b> u256)) {
                amount_in = amount;
                next_sqrt_price = <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_input">get_next_sqrt_price_from_input</a>(
                    current_sqrt_price,
                    liquidity,
                    amount,
                    a2b,
                );
            } <b>else</b> {
                amount_in = (max_amount_in <b>as</b> u64);
                next_sqrt_price = target_sqrt_price;
            };
            amount_out =
                (<a href="clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(current_sqrt_price, next_sqrt_price, liquidity, a2b) <b>as</b> u64);
        } <b>else</b> {
            <b>let</b> max_amount_out = <a href="clmm_math.md#0xc8_clmm_math_get_delta_down_from_output">get_delta_down_from_output</a>(
                current_sqrt_price,
                target_sqrt_price,
                liquidity,
                a2b,
            );
            <b>if</b> (max_amount_out &gt; (amount <b>as</b> u256)) {
                amount_out = amount;
                next_sqrt_price =
                    <a href="clmm_math.md#0xc8_clmm_math_get_next_sqrt_price_from_output">get_next_sqrt_price_from_output</a>(current_sqrt_price, liquidity, amount, a2b);
            } <b>else</b> {
                amount_out = (max_amount_out <b>as</b> u64);
                next_sqrt_price = target_sqrt_price;
            };
            amount_in =
                (<a href="clmm_math.md#0xc8_clmm_math_get_delta_up_from_input">get_delta_up_from_input</a>(current_sqrt_price, next_sqrt_price, liquidity, a2b) <b>as</b> u64);
        };
        (amount_in, amount_out, next_sqrt_price)
    }
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_liquidity_by_amount"></a>

## Function `get_liquidity_by_amount`



<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_by_amount">get_liquidity_by_amount</a>(_tick_lower_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _current_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _current_sqrt_price: u128, _amount: u64, _fix_amount_a: bool): (u128, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_by_amount">get_liquidity_by_amount</a>(
    _tick_lower_index: I32,
    _tick_upper_index: I32,
    _current_tick_index: I32,
    _current_sqrt_price: u128,
    _amount: u64,
    _fix_amount_a: bool
): (u128, u64, u64)
{
    <b>let</b> tick_lower_price = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(_tick_lower_index);
    <b>let</b> tick_upper_price = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(_tick_upper_index);
    <b>let</b> liquidity: u128;
    <b>let</b> amount_a: u64 = 0;
    <b>let</b> amount_b: u64 = 0;

    <b>if</b> (_fix_amount_a) {
        amount_a = _amount;
        <b>if</b> (<a href="i32.md#0xc8_i32_lt">i32::lt</a>(_current_tick_index, _tick_lower_index)) {
            liquidity = <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(tick_lower_price, tick_upper_price, amount_a, <b>false</b>);
        } <b>else</b> {
            <b>assert</b>!(<a href="i32.md#0xc8_i32_lt">i32::lt</a>(_current_tick_index, _tick_upper_index), <a href="clmm_math.md#0xc8_clmm_math_ETICK_EXCEED_MAXIMUM">ETICK_EXCEED_MAXIMUM</a>);
            liquidity = <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_a">get_liquidity_from_a</a>(_current_sqrt_price, tick_upper_price, _amount, <b>false</b>);
            amount_b = <a href="clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(_current_sqrt_price, tick_lower_price, liquidity, <b>true</b>);
        };
    } <b>else</b> {
        amount_b = _amount;
        <b>if</b> (<a href="i32.md#0xc8_i32_gte">i32::gte</a>(_tick_upper_index, _tick_lower_index)) {
            liquidity = <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(tick_lower_price, tick_upper_price, _amount, <b>false</b>)
        } <b>else</b> {
            <b>assert</b>!(<a href="i32.md#0xc8_i32_gte">i32::gte</a>(_current_tick_index, _tick_lower_index), <a href="clmm_math.md#0xc8_clmm_math_ETICK_LESS_MINIMUM">ETICK_LESS_MINIMUM</a>);
            liquidity = <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_from_b">get_liquidity_from_b</a>(tick_lower_price, _current_sqrt_price, _amount, <b>false</b>);
            amount_a = <a href="clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(_current_sqrt_price, tick_upper_price, liquidity, <b>true</b>)
        };
    };
    (liquidity, amount_a, amount_b)
}
</code></pre>



</details>

<a name="0xc8_clmm_math_get_amount_by_liquidity"></a>

## Function `get_amount_by_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_amount_by_liquidity">get_amount_by_liquidity</a>(_tick_lower_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _current_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _current_sqrt_price: u128, _liquidity_delta: u128, _round_up: bool): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="clmm_math.md#0xc8_clmm_math_get_amount_by_liquidity">get_amount_by_liquidity</a>(
    _tick_lower_index: I32,
    _tick_upper_index: I32,
    _current_tick_index: I32,
    _current_sqrt_price: u128,
    _liquidity_delta: u128,
    _round_up: bool
): (u64, u64)
{
    <b>if</b> (_liquidity_delta == 0) {
        <b>return</b> (0, 0)
    };
    <b>let</b> tick_lower_price = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(_tick_lower_index);
    <b>let</b> tick_upper_price = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(_tick_upper_index);
    <b>let</b> amount_a: u64;
    <b>let</b> amount_b: u64;
    <b>if</b> (<a href="i32.md#0xc8_i32_lt">i32::lt</a>(_tick_lower_index, _current_tick_index)) {
        amount_a = <a href="clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(tick_lower_price, tick_upper_price, _liquidity_delta, _round_up);
        amount_b = 0;
    } <b>else</b> {
        <b>if</b> (<a href="i32.md#0xc8_i32_lt">i32::lt</a>(_tick_upper_index, _current_tick_index)) {
            amount_a = <a href="clmm_math.md#0xc8_clmm_math_get_delta_a">get_delta_a</a>(_current_sqrt_price, tick_upper_price, _liquidity_delta, _round_up);
            amount_b = <a href="clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(tick_lower_price, _current_sqrt_price, _liquidity_delta, _round_up);
        } <b>else</b> {
            amount_a = 0;
            amount_b = <a href="clmm_math.md#0xc8_clmm_math_get_delta_b">get_delta_b</a>(tick_lower_price, tick_upper_price, _liquidity_delta, _round_up);
        }
    };
    (amount_a, amount_b)
}
</code></pre>



</details>

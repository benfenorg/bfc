---
title: Module `0xc8::tick_math`
---



-  [Constants](#@Constants_0)
-  [Function `max_tick`](#0xc8_tick_math_max_tick)
-  [Function `min_tick`](#0xc8_tick_math_min_tick)
-  [Function `max_sqrt_price`](#0xc8_tick_math_max_sqrt_price)
-  [Function `min_sqrt_price`](#0xc8_tick_math_min_sqrt_price)
-  [Function `tick_bound`](#0xc8_tick_math_tick_bound)
-  [Function `get_sqrt_price_at_tick`](#0xc8_tick_math_get_sqrt_price_at_tick)
-  [Function `is_valid_index`](#0xc8_tick_math_is_valid_index)
-  [Function `adjust_tick`](#0xc8_tick_math_adjust_tick)
-  [Function `get_tick_at_sqrt_price`](#0xc8_tick_math_get_tick_at_sqrt_price)
-  [Function `as_u8`](#0xc8_tick_math_as_u8)
-  [Function `get_sqrt_price_at_negative_tick`](#0xc8_tick_math_get_sqrt_price_at_negative_tick)
-  [Function `get_sqrt_price_at_positive_tick`](#0xc8_tick_math_get_sqrt_price_at_positive_tick)
-  [Function `get_valid_tick_index`](#0xc8_tick_math_get_valid_tick_index)
-  [Function `get_next_valid_tick_index`](#0xc8_tick_math_get_next_valid_tick_index)
-  [Function `get_prev_valid_tick_index`](#0xc8_tick_math_get_prev_valid_tick_index)
-  [Function `get_default_sqrt_price_limit`](#0xc8_tick_math_get_default_sqrt_price_limit)


<pre><code><b>use</b> <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128">0xc8::full_math_u128</a>;
<b>use</b> <a href="../bfc-system/i128.md#0xc8_i128">0xc8::i128</a>;
<b>use</b> <a href="../bfc-system/i32.md#0xc8_i32">0xc8::i32</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0xc8_tick_math_EINVALID_SQRT_PRICE"></a>



<pre><code><b>const</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_EINVALID_SQRT_PRICE">EINVALID_SQRT_PRICE</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xc8_tick_math_EINVALID_TICK"></a>

Errors


<pre><code><b>const</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_EINVALID_TICK">EINVALID_TICK</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xc8_tick_math_MAX_SQRT_PRICE_X64"></a>



<pre><code><b>const</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_MAX_SQRT_PRICE_X64">MAX_SQRT_PRICE_X64</a>: u128 = 79226673515401279992447579055;
</code></pre>



<a name="0xc8_tick_math_MAX_TICK"></a>



<pre><code><b>const</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_MAX_TICK">MAX_TICK</a>: u32 = 443636;
</code></pre>



<a name="0xc8_tick_math_MIN_SQRT_PRICE_X64"></a>



<pre><code><b>const</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_MIN_SQRT_PRICE_X64">MIN_SQRT_PRICE_X64</a>: u128 = 4295048016;
</code></pre>



<a name="0xc8_tick_math_max_tick"></a>

## Function `max_tick`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_tick">max_tick</a>(): <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_tick">max_tick</a>(): I32 {
    <a href="../bfc-system/i32.md#0xc8_i32_from">i32::from</a>(<a href="../bfc-system/tick_math.md#0xc8_tick_math_MAX_TICK">MAX_TICK</a>)
}
</code></pre>



</details>

<a name="0xc8_tick_math_min_tick"></a>

## Function `min_tick`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_tick">min_tick</a>(): <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_tick">min_tick</a>(): I32 {
    <a href="../bfc-system/i32.md#0xc8_i32_neg_from">i32::neg_from</a>(<a href="../bfc-system/tick_math.md#0xc8_tick_math_MAX_TICK">MAX_TICK</a>)
}
</code></pre>



</details>

<a name="0xc8_tick_math_max_sqrt_price"></a>

## Function `max_sqrt_price`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_sqrt_price">max_sqrt_price</a>(): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_sqrt_price">max_sqrt_price</a>(): u128 {
    <a href="../bfc-system/tick_math.md#0xc8_tick_math_MAX_SQRT_PRICE_X64">MAX_SQRT_PRICE_X64</a>
}
</code></pre>



</details>

<a name="0xc8_tick_math_min_sqrt_price"></a>

## Function `min_sqrt_price`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_sqrt_price">min_sqrt_price</a>(): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_sqrt_price">min_sqrt_price</a>(): u128 {
    <a href="../bfc-system/tick_math.md#0xc8_tick_math_MIN_SQRT_PRICE_X64">MIN_SQRT_PRICE_X64</a>
}
</code></pre>



</details>

<a name="0xc8_tick_math_tick_bound"></a>

## Function `tick_bound`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_tick_bound">tick_bound</a>(): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_tick_bound">tick_bound</a>(): u32 {
    <a href="../bfc-system/tick_math.md#0xc8_tick_math_MAX_TICK">MAX_TICK</a>
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_sqrt_price_at_tick"></a>

## Function `get_sqrt_price_at_tick`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">get_sqrt_price_at_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">get_sqrt_price_at_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>: I32): u128 {
    <b>assert</b>!(<a href="../bfc-system/i32.md#0xc8_i32_gte">i32::gte</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>, <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_tick">min_tick</a>()) && <a href="../bfc-system/i32.md#0xc8_i32_lte">i32::lte</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>, <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_tick">max_tick</a>()), <a href="../bfc-system/tick_math.md#0xc8_tick_math_EINVALID_TICK">EINVALID_TICK</a>);
    <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_is_neg">i32::is_neg</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>)) {
        <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_negative_tick">get_sqrt_price_at_negative_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>)
    } <b>else</b> {
        <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_positive_tick">get_sqrt_price_at_positive_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>)
    }
}
</code></pre>



</details>

<a name="0xc8_tick_math_is_valid_index"></a>

## Function `is_valid_index`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_is_valid_index">is_valid_index</a>(index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, tick_spacing: u32): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_is_valid_index">is_valid_index</a>(index: I32, tick_spacing: u32): bool {
    <b>let</b> in_range = <a href="../bfc-system/i32.md#0xc8_i32_gte">i32::gte</a>(index, <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_tick">min_tick</a>()) && <a href="../bfc-system/i32.md#0xc8_i32_lte">i32::lte</a>(index, <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_tick">max_tick</a>());
    in_range && (<a href="../bfc-system/i32.md#0xc8_i32_mod">i32::mod</a>(index, <a href="../bfc-system/i32.md#0xc8_i32_from">i32::from</a>(tick_spacing)) == <a href="../bfc-system/i32.md#0xc8_i32_from">i32::from</a>(0))
}
</code></pre>



</details>

<a name="0xc8_tick_math_adjust_tick"></a>

## Function `adjust_tick`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_adjust_tick">adjust_tick</a>(index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, tick_spacing: u32): <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_adjust_tick">adjust_tick</a>(index: I32, tick_spacing: u32): I32 {
    <a href="../bfc-system/i32.md#0xc8_i32_mul">i32::mul</a>(<a href="../bfc-system/i32.md#0xc8_i32_div">i32::div</a>(index, <a href="../bfc-system/i32.md#0xc8_i32_from">i32::from</a>(tick_spacing)), <a href="../bfc-system/i32.md#0xc8_i32_from">i32::from</a>(tick_spacing))
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_tick_at_sqrt_price"></a>

## Function `get_tick_at_sqrt_price`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_tick_at_sqrt_price">get_tick_at_sqrt_price</a>(sqrt_price: u128): <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_tick_at_sqrt_price">get_tick_at_sqrt_price</a>(sqrt_price: u128): I32 {
    <b>assert</b>!(sqrt_price &gt;= <a href="../bfc-system/tick_math.md#0xc8_tick_math_MIN_SQRT_PRICE_X64">MIN_SQRT_PRICE_X64</a> && sqrt_price &lt;= <a href="../bfc-system/tick_math.md#0xc8_tick_math_MAX_SQRT_PRICE_X64">MAX_SQRT_PRICE_X64</a>, <a href="../bfc-system/tick_math.md#0xc8_tick_math_EINVALID_SQRT_PRICE">EINVALID_SQRT_PRICE</a>);
    <b>let</b> <b>mut</b> r = sqrt_price;

    <b>let</b> <b>mut</b> msb = 0;

    <b>let</b> <b>mut</b> f: u8 = <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(r &gt;= 0x10000000000000000) &lt;&lt; 6; // If r &gt;= 2^64, f = 64 <b>else</b> 0
    msb = msb | f;
    r = r &gt;&gt; f;
    f = <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(r &gt;= 0x100000000) &lt;&lt; 5; // 2^32
    msb = msb | f;
    r = r &gt;&gt; f;
    f = <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(r &gt;= 0x10000) &lt;&lt; 4; // 2^16
    msb = msb | f;
    r = r &gt;&gt; f;
    f = <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(r &gt;= 0x100) &lt;&lt; 3; // 2^8
    msb = msb | f;
    r = r &gt;&gt; f;
    f = <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(r &gt;= 0x10) &lt;&lt; 2; // 2^4
    msb = msb | f;
    r = r &gt;&gt; f;
    f = <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(r &gt;= 0x4) &lt;&lt; 1; // 2^2
    msb = msb | f;
    r = r &gt;&gt; f;
    f = <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(r &gt;= 0x2) &lt;&lt; 0; // 2^0
    msb = msb | f;

    <b>let</b> <b>mut</b> log_2_x32 = <a href="../bfc-system/i128.md#0xc8_i128_shl">i128::shl</a>(<a href="../bfc-system/i128.md#0xc8_i128_sub">i128::sub</a>(<a href="../bfc-system/i128.md#0xc8_i128_from">i128::from</a>((msb <b>as</b> u128)), <a href="../bfc-system/i128.md#0xc8_i128_from">i128::from</a>(64)), 32);

    r = <b>if</b> (msb &gt;= 64) {
        sqrt_price &gt;&gt; (msb - 63)
    } <b>else</b> {
        sqrt_price &lt;&lt; (63 - msb)
    };

    <b>let</b> <b>mut</b> shift = 31;
    <b>while</b> (shift &gt;= 18) {
        r = ((r * r) &gt;&gt; 63);
        f = ((r &gt;&gt; 64) <b>as</b> u8);
        log_2_x32 = <a href="../bfc-system/i128.md#0xc8_i128_or">i128::or</a>(log_2_x32, <a href="../bfc-system/i128.md#0xc8_i128_shl">i128::shl</a>(<a href="../bfc-system/i128.md#0xc8_i128_from">i128::from</a>((f <b>as</b> u128)), shift));
        r = r &gt;&gt; f;
        shift = shift - 1;
    };

    <b>let</b> log_sqrt_10001 = <a href="../bfc-system/i128.md#0xc8_i128_mul">i128::mul</a>(log_2_x32, <a href="../bfc-system/i128.md#0xc8_i128_from">i128::from</a>(59543866431366u128));

    <b>let</b> tick_low = <a href="../bfc-system/i128.md#0xc8_i128_as_i32">i128::as_i32</a>(<a href="../bfc-system/i128.md#0xc8_i128_shr">i128::shr</a>(<a href="../bfc-system/i128.md#0xc8_i128_sub">i128::sub</a>(log_sqrt_10001, <a href="../bfc-system/i128.md#0xc8_i128_from">i128::from</a>(184467440737095516u128)), 64));
    <b>let</b> tick_high = <a href="../bfc-system/i128.md#0xc8_i128_as_i32">i128::as_i32</a>(<a href="../bfc-system/i128.md#0xc8_i128_shr">i128::shr</a>(<a href="../bfc-system/i128.md#0xc8_i128_add">i128::add</a>(log_sqrt_10001, <a href="../bfc-system/i128.md#0xc8_i128_from">i128::from</a>(15793534762490258745u128)), 64));

    <b>if</b> (<a href="../bfc-system/i32.md#0xc8_i32_eq">i32::eq</a>(tick_low, tick_high)) {
        <b>return</b> tick_low
    } <b>else</b> <b>if</b> (<a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">get_sqrt_price_at_tick</a>(tick_high) &lt;= sqrt_price) {
        <b>return</b> tick_high
    } <b>else</b> {
        <b>return</b> tick_low
    }
}
</code></pre>



</details>

<a name="0xc8_tick_math_as_u8"></a>

## Function `as_u8`



<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(b: bool): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_as_u8">as_u8</a>(b: bool): u8 {
    <b>if</b> (b) {
        1
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_sqrt_price_at_negative_tick"></a>

## Function `get_sqrt_price_at_negative_tick`



<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_negative_tick">get_sqrt_price_at_negative_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_negative_tick">get_sqrt_price_at_negative_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>: I32): u128 {
    <b>let</b> abs_tick = <a href="../bfc-system/i32.md#0xc8_i32_as_u32">i32::as_u32</a>(<a href="../bfc-system/i32.md#0xc8_i32_abs">i32::abs</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>));
    <b>let</b> <b>mut</b> ratio = <b>if</b> (abs_tick & 0x1 != 0) {
        18445821805675392311u128
    } <b>else</b> {
        18446744073709551616u128
    };
    <b>if</b> (abs_tick & 0x2 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18444899583751176498u128, 64u8)
    };
    <b>if</b> (abs_tick & 0x4 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18443055278223354162u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x8 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18439367220385604838u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x10 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18431993317065449817u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x20 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18417254355718160513u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x40 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18387811781193591352u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x80 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18329067761203520168u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x100 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 18212142134806087854u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x200 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 17980523815641551639u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x400 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 17526086738831147013u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x800 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 16651378430235024244u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x1000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 15030750278693429944u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x2000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 12247334978882834399u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x4000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 8131365268884726200u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x8000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 3584323654723342297u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x10000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 696457651847595233u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x20000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 26294789957452057u128, 64u8);
    };
    <b>if</b> (abs_tick & 0x40000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 37481735321082u128, 64u8);
    };

    ratio
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_sqrt_price_at_positive_tick"></a>

## Function `get_sqrt_price_at_positive_tick`



<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_positive_tick">get_sqrt_price_at_positive_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_sqrt_price_at_positive_tick">get_sqrt_price_at_positive_tick</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>: I32): u128 {
    <b>let</b> abs_tick = <a href="../bfc-system/i32.md#0xc8_i32_as_u32">i32::as_u32</a>(<a href="../bfc-system/i32.md#0xc8_i32_abs">i32::abs</a>(<a href="../bfc-system/tick.md#0xc8_tick">tick</a>));
    <b>let</b> <b>mut</b> ratio = <b>if</b> (abs_tick & 0x1 != 0) {
        79232123823359799118286999567u128
    } <b>else</b> {
        79228162514264337593543950336u128
    };

    <b>if</b> (abs_tick & 0x2 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 79236085330515764027303304731u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x4 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 79244008939048815603706035061u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x8 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 79259858533276714757314932305u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x10 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 79291567232598584799939703904u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x20 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 79355022692464371645785046466u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x40 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 79482085999252804386437311141u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x80 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 79736823300114093921829183326u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x100 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 80248749790819932309965073892u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x200 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 81282483887344747381513967011u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x400 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 83390072131320151908154831281u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x800 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 87770609709833776024991924138u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x1000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 97234110755111693312479820773u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x2000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 119332217159966728226237229890u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x4000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 179736315981702064433883588727u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x8000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 407748233172238350107850275304u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x10000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 2098478828474011932436660412517u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x20000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 55581415166113811149459800483533u128, 96u8)
    };
    <b>if</b> (abs_tick & 0x40000 != 0) {
        ratio = <a href="../bfc-system/full_math_u128.md#0xc8_full_math_u128_mul_shr">full_math_u128::mul_shr</a>(ratio, 38992368544603139932233054999993551u128, 96u8)
    };

    ratio &gt;&gt; 32
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_valid_tick_index"></a>

## Function `get_valid_tick_index`



<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_valid_tick_index">get_valid_tick_index</a>(index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, tick_spacing: u32, prev: bool): <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_valid_tick_index">get_valid_tick_index</a>(index: I32, tick_spacing: u32, prev: bool): I32 {
    <b>if</b> (<a href="../bfc-system/tick_math.md#0xc8_tick_math_is_valid_index">is_valid_index</a>(index, tick_spacing)) {
        index
    } <b>else</b> {
        <b>let</b> spacing = <a href="../bfc-system/i32.md#0xc8_i32_from">i32::from</a>(tick_spacing);
        <b>let</b> valid_index = <a href="../bfc-system/i32.md#0xc8_i32_sub">i32::sub</a>(index, <a href="../bfc-system/i32.md#0xc8_i32_mod">i32::mod</a>(index, spacing));
        <b>if</b> (prev) {
            <a href="../bfc-system/i32.md#0xc8_i32_sub">i32::sub</a>(valid_index, spacing)
        } <b>else</b> {
            <a href="../bfc-system/i32.md#0xc8_i32_add">i32::add</a>(valid_index, spacing)
        }
    }
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_next_valid_tick_index"></a>

## Function `get_next_valid_tick_index`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_next_valid_tick_index">get_next_valid_tick_index</a>(index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, tick_spacing: u32): <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_next_valid_tick_index">get_next_valid_tick_index</a>(index: I32, tick_spacing: u32): I32 {
    <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_valid_tick_index">get_valid_tick_index</a>(index, tick_spacing, <b>false</b>)
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_prev_valid_tick_index"></a>

## Function `get_prev_valid_tick_index`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_prev_valid_tick_index">get_prev_valid_tick_index</a>(index: <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>, tick_spacing: u32): <a href="../bfc-system/i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_prev_valid_tick_index">get_prev_valid_tick_index</a>(index: I32, tick_spacing: u32): I32 {
    <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_valid_tick_index">get_valid_tick_index</a>(index, tick_spacing, <b>true</b>)
}
</code></pre>



</details>

<a name="0xc8_tick_math_get_default_sqrt_price_limit"></a>

## Function `get_default_sqrt_price_limit`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_default_sqrt_price_limit">get_default_sqrt_price_limit</a>(_a2b: bool): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/tick_math.md#0xc8_tick_math_get_default_sqrt_price_limit">get_default_sqrt_price_limit</a>(_a2b: bool): u128 {
    <b>if</b> (_a2b) {
        <a href="../bfc-system/tick_math.md#0xc8_tick_math_min_sqrt_price">min_sqrt_price</a>()
    } <b>else</b> {
        <a href="../bfc-system/tick_math.md#0xc8_tick_math_max_sqrt_price">max_sqrt_price</a>()
    }
}
</code></pre>



</details>

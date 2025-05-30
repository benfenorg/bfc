---
title: Module `0xc8::option_u64`
---



-  [Struct `OptionU64`](#0xc8_option_u64_OptionU64)
-  [Constants](#@Constants_0)
-  [Function `some`](#0xc8_option_u64_some)
-  [Function `none`](#0xc8_option_u64_none)
-  [Function `borrow`](#0xc8_option_u64_borrow)
-  [Function `borrow_mut`](#0xc8_option_u64_borrow_mut)
-  [Function `swap_or_fill`](#0xc8_option_u64_swap_or_fill)
-  [Function `is_some`](#0xc8_option_u64_is_some)
-  [Function `is_none`](#0xc8_option_u64_is_none)
-  [Function `contains`](#0xc8_option_u64_contains)
-  [Function `is_some_and_lte`](#0xc8_option_u64_is_some_and_lte)


<pre><code></code></pre>



<a name="0xc8_option_u64_OptionU64"></a>

## Struct `OptionU64`



<pre><code><b>struct</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>is_none: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>v: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_option_u64_EOptionU64IsNone"></a>



<pre><code><b>const</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_EOptionU64IsNone">EOptionU64IsNone</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xc8_option_u64_some"></a>

## Function `some`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_some">some</a>(v: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_some">some</a>(v: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a> {
    <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a> {
        is_none: <b>false</b>,
        v
    }
}
</code></pre>



</details>

<a name="0xc8_option_u64_none"></a>

## Function `none`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_none">none</a>(): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_none">none</a>(): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a> {
    <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a> {
        is_none: <b>true</b>,
        v: 0
    }
}
</code></pre>



</details>

<a name="0xc8_option_u64_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">borrow</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">borrow</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>assert</b>!(!opt.is_none, <a href="../bfc-system/option_u64.md#0xc8_option_u64_EOptionU64IsNone">EOptionU64IsNone</a>);
    opt.v
}
</code></pre>



</details>

<a name="0xc8_option_u64_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow_mut">borrow_mut</a>(opt: &<b>mut</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>): &<b>mut</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow_mut">borrow_mut</a>(opt: &<b>mut</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a>): &<b>mut</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>assert</b>!(!opt.is_none, <a href="../bfc-system/option_u64.md#0xc8_option_u64_EOptionU64IsNone">EOptionU64IsNone</a>);
    &<b>mut</b> opt.v
}
</code></pre>



</details>

<a name="0xc8_option_u64_swap_or_fill"></a>

## Function `swap_or_fill`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_swap_or_fill">swap_or_fill</a>(opt: &<b>mut</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>, v: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_swap_or_fill">swap_or_fill</a>(opt: &<b>mut</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a>, v: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    opt.is_none = <b>false</b>;
    opt.v = v;
}
</code></pre>



</details>

<a name="0xc8_option_u64_is_some"></a>

## Function `is_some`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_is_some">is_some</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_is_some">is_some</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a>): bool {
    !opt.is_none
}
</code></pre>



</details>

<a name="0xc8_option_u64_is_none"></a>

## Function `is_none`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_is_none">is_none</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_is_none">is_none</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a>): bool {
    opt.is_none
}
</code></pre>



</details>

<a name="0xc8_option_u64_contains"></a>

## Function `contains`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_contains">contains</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>, e_ref: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_contains">contains</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a>, e_ref: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool {
    ((!opt.is_none) && (opt.v == e_ref))
}
</code></pre>



</details>

<a name="0xc8_option_u64_is_some_and_lte"></a>

## Function `is_some_and_lte`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_is_some_and_lte">is_some_and_lte</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>, v: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64_is_some_and_lte">is_some_and_lte</a>(opt: &<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">OptionU64</a>, v: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool {
    (!opt.is_none) && (opt.v &lt;= v)
}
</code></pre>



</details>

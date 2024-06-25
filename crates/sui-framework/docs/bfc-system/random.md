---
title: Module `0xc8::random`
---



-  [Struct `Random`](#0xc8_random_Random)
-  [Function `new`](#0xc8_random_new)
-  [Function `seed`](#0xc8_random_seed)
-  [Function `rand_n`](#0xc8_random_rand_n)
-  [Function `rand`](#0xc8_random_rand)
-  [Function `seed_rand`](#0xc8_random_seed_rand)


<pre><code></code></pre>



<a name="0xc8_random_Random"></a>

## Struct `Random`



<pre><code><b>struct</b> <a href="random.md#0xc8_random_Random">Random</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seed: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_random_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_new">new</a>(seed: u64): <a href="../sui-framework/random.md#0x2_random_Random">random::Random</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_new">new</a>(seed: u64): <a href="random.md#0xc8_random_Random">Random</a> {
    <a href="random.md#0xc8_random_Random">Random</a> {
        seed
    }
}
</code></pre>



</details>

<a name="0xc8_random_seed"></a>

## Function `seed`



<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_seed">seed</a>(r: &<b>mut</b> <a href="../sui-framework/random.md#0x2_random_Random">random::Random</a>, seed: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_seed">seed</a>(r: &<b>mut</b> <a href="random.md#0xc8_random_Random">Random</a>, seed: u64) {
    r.seed = ((((r.seed <b>as</b> u128) + (seed <b>as</b> u128) & 0x0000000000000000ffffffffffffffff)) <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_random_rand_n"></a>

## Function `rand_n`



<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_rand_n">rand_n</a>(r: &<b>mut</b> <a href="../sui-framework/random.md#0x2_random_Random">random::Random</a>, n: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_rand_n">rand_n</a>(r: &<b>mut</b> <a href="random.md#0xc8_random_Random">Random</a>, n: u64): u64 {
    r.seed = ((((9223372036854775783u128 * ((r.seed <b>as</b> u128) + 999983)) &gt;&gt; 1) & 0x0000000000000000ffffffffffffffff) <b>as</b> u64);
    r.seed % n
}
</code></pre>



</details>

<a name="0xc8_random_rand"></a>

## Function `rand`



<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_rand">rand</a>(r: &<b>mut</b> <a href="../sui-framework/random.md#0x2_random_Random">random::Random</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_rand">rand</a>(r: &<b>mut</b> <a href="random.md#0xc8_random_Random">Random</a>): u64 {
    r.seed = ((((9223372036854775783u128 * ((r.seed <b>as</b> u128)) + 999983) &gt;&gt; 1) & 0x0000000000000000ffffffffffffffff) <b>as</b> u64);
    r.seed
}
</code></pre>



</details>

<a name="0xc8_random_seed_rand"></a>

## Function `seed_rand`



<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_seed_rand">seed_rand</a>(r: &<b>mut</b> <a href="../sui-framework/random.md#0x2_random_Random">random::Random</a>, seed: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="random.md#0xc8_random_seed_rand">seed_rand</a>(r: &<b>mut</b> <a href="random.md#0xc8_random_Random">Random</a>, seed: u64): u64 {


    r.seed = ((((r.seed <b>as</b> u128) + (seed <b>as</b> u128) & 0x0000000000000000ffffffffffffffff)) <b>as</b> u64);
    r.seed = (((9223372036854775783u128 * ((r.seed <b>as</b> u128) + 999983) &gt;&gt; 1) & 0x0000000000000000ffffffffffffffff) <b>as</b> u64);
    r.seed
}
</code></pre>



</details>

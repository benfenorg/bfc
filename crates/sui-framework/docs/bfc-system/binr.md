---
title: Module `0xc8::binr`
---



-  [Struct `BINR`](#0xc8_binr_BINR)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_binr_new)
-  [Function `transfer`](#0xc8_binr_transfer)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0xc8_binr_BINR"></a>

## Struct `BINR`



<pre><code><b>struct</b> <a href="binr.md#0xc8_binr_BINR">BINR</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_binr_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="binr.md#0xc8_binr_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="0xc8_binr_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="binr.md#0xc8_binr_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="0xc8_binr_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="binr.md#0xc8_binr_new">new</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="binr.md#0xc8_binr_BINR">binr::BINR</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="binr.md#0xc8_binr_new">new</a>(ctx: &<b>mut</b> TxContext): Supply&lt;<a href="binr.md#0xc8_binr_BINR">BINR</a>&gt; {
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="binr.md#0xc8_binr_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="binr.md#0xc8_binr_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (cap, metadata) = <a href="../sui-framework/coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="binr.md#0xc8_binr_BINR">BINR</a> {},
        9,
        b"<a href="binr.md#0xc8_binr_BINR">BINR</a>",
        b"Benfen INR",
        b"",
        <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        ctx
    );
    <a href="../sui-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <a href="../sui-framework/coin.md#0x2_coin_treasury_into_supply">coin::treasury_into_supply</a>(cap)
}
</code></pre>



</details>

<a name="0xc8_binr_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="binr.md#0xc8_binr_BINR">binr::BINR</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="binr.md#0xc8_binr_BINR">BINR</a>&gt;, recipient: <b>address</b>) {
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>

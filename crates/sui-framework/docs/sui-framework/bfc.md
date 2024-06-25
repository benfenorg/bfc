---
title: Module `0x2::bfc`
---

Coin<BFC> is the token used to pay for gas in Sui.
It has 9 decimals, and the smallest unit (10^-9) is called "mist".


-  [Struct `BFC`](#0x2_bfc_BFC)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x2_bfc_new)
-  [Function `transfer`](#0x2_bfc_transfer)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x2_bfc_BFC"></a>

## Struct `BFC`

Name of the coin


<pre><code><b>struct</b> <a href="../sui-framework/bfc.md#0x2_bfc_BFC">BFC</a> <b>has</b> drop
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


<a name="0x2_bfc_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../sui-framework/bfc.md#0x2_bfc_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="0x2_bfc_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../sui-framework/bfc.md#0x2_bfc_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="0x2_bfc_TOTAL_SUPPLY_MIST"></a>

The amount of Mist per Sui token based on the the fact that mist is
10^-9 of a Sui token
The total supply of Sui denominated in whole Sui tokens (10 Billion)
The total supply of Sui denominated in Mist (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../sui-framework/bfc.md#0x2_bfc_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>: u64 = 1000000000000000000;
</code></pre>



<a name="0x2_bfc_new"></a>

## Function `new`

Register the <code>SUI</code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../sui-framework/bfc.md#0x2_bfc_new">new</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui-framework/bfc.md#0x2_bfc_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">BFC</a>&gt; {
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="../sui-framework/bfc.md#0x2_bfc_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="../sui-framework/bfc.md#0x2_bfc_EAlreadyMinted">EAlreadyMinted</a>);

    <b>let</b> (<a href="../../treasury.md#0xc8_treasury">treasury</a>, metadata) = <a href="../sui-framework/coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="../sui-framework/bfc.md#0x2_bfc_BFC">BFC</a>{},
        9,
        b"<a href="../sui-framework/bfc.md#0x2_bfc_BFC">BFC</a>",
        b"Bfc",
        // TODO: add appropriate description and logo <a href="../sui-framework/url.md#0x2_url">url</a>
        b"",
        <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        ctx
    );
    <a href="../sui-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = <a href="../sui-framework/coin.md#0x2_coin_treasury_into_supply">coin::treasury_into_supply</a>(<a href="../../treasury.md#0xc8_treasury">treasury</a>);
    <b>let</b> total_sui = <a href="../sui-framework/balance.md#0x2_balance_increase_supply">balance::increase_supply</a>(&<b>mut</b> supply, <a href="../sui-framework/bfc.md#0x2_bfc_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>);
    <a href="../sui-framework/balance.md#0x2_balance_destroy_supply">balance::destroy_supply</a>(supply);
    total_sui
}
</code></pre>



</details>

<a name="0x2_bfc_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">BFC</a>&gt;, recipient: <b>address</b>) {
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>

---
title: Module `0x2::abfc`
---



-  [Struct `ABFC`](#0x2_abfc_ABFC)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x2_abfc_new)
-  [Function `transfer`](#0x2_abfc_transfer)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance">0x2::anonymous_balance</a>;
<b>use</b> <a href="../sui-framework/anonymous_coin.md#0x2_anonymous_coin">0x2::anonymous_coin</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x2_abfc_ABFC"></a>

## Struct `ABFC`

Name of the coin


<pre><code><b>struct</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">ABFC</a> <b>has</b> drop
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


<a name="0x2_abfc_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x2_abfc_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_EAlreadyMinted">EAlreadyMinted</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x2_abfc_MIST_PER_SUI"></a>

The amount of Mist per Sui token based on the the fact that mist is
10^-9 of a Sui token
The amount of Mist per Sui token based on the fact that mist is
10^-9 of a Sui token


<pre><code><b>const</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_MIST_PER_SUI">MIST_PER_SUI</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000;
</code></pre>



<a name="0x2_abfc_TOTAL_SUPPLY_MIST"></a>

The total supply of Sui denominated in whole Sui tokens (10 Billion)


<pre><code><b>const</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 100000000000000000;
</code></pre>



<a name="0x2_abfc_new"></a>

## Function `new`

Register the <code>SUI</code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_new">new</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/anonymous_balance.md#0x2_anonymous_balance_Anonymos_Balance">anonymous_balance::Anonymos_Balance</a>&lt;<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">abfc::ABFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_new">new</a>(ctx: &<b>mut</b> TxContext): Anonymos_Balance&lt;<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">ABFC</a>&gt; {
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_EAlreadyMinted">EAlreadyMinted</a>);

    <b>let</b> (<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, metadata) = <a href="../sui-framework/anonymous_coin.md#0x2_anonymous_coin_create_currency">anonymous_coin::create_currency</a>(
        <a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">ABFC</a>{},
        9,
        b"<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">ABFC</a>",
        b"<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">ABFC</a>",
        // TODO: add appropriate description and logo <a href="../sui-framework/url.md#0x2_url">url</a>
        b"",
        <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        ctx
    );
    <a href="../sui-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.treasury_into_supply();
    <b>let</b> total_sui = supply.increase_supply(<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>);
    supply.destroy_supply();
    total_sui
}
</code></pre>



</details>

<a name="0x2_abfc_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../sui-framework/anonymous_coin.md#0x2_anonymous_coin_Anonymous_Coin">anonymous_coin::Anonymous_Coin</a>&lt;<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">abfc::ABFC</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a>(c: <a href="../sui-framework/anonymous_coin.md#0x2_anonymous_coin_Anonymous_Coin">anonymous_coin::Anonymous_Coin</a>&lt;<a href="../sui-framework/anonymous_bfc.md#0x2_abfc_ABFC">ABFC</a>&gt;, recipient: <b>address</b>) {
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>

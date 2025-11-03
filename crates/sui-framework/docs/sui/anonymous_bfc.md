---
title: Module `sui::abfc`
---



-  [Struct `ABFC`](#sui_abfc_ABFC)
-  [Constants](#@Constants_0)
-  [Function `new`](#sui_abfc_new)
-  [Function `transfer`](#sui_abfc_transfer)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance">sui::anonymous_balance</a>;
<b>use</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin">sui::anonymous_coin</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/hfe_ops.md#sui_hfe_ops">sui::hfe_ops</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="sui_abfc_ABFC"></a>

## Struct `ABFC`

Name of the coin


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_bfc.md#sui_abfc_ABFC">ABFC</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="sui_abfc_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../sui/anonymous_bfc.md#sui_abfc_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="sui_abfc_MIST_PER_SUI"></a>

The amount of Mist per Sui token based on the fact that mist is
10^-9 of a Sui token


<pre><code><b>const</b> <a href="../sui/anonymous_bfc.md#sui_abfc_MIST_PER_SUI">MIST_PER_SUI</a>: u64 = 1000000000;
</code></pre>



<a name="sui_abfc_TOTAL_SUPPLY_MIST"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_bfc.md#sui_abfc_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>: u64 = 100000000000000000;
</code></pre>



<a name="sui_abfc_new"></a>

## Function `new`

Register the <code>SUI</code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../sui/anonymous_bfc.md#sui_abfc_new">new</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;<a href="../sui/anonymous_bfc.md#sui_abfc_ABFC">sui::abfc::ABFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui/anonymous_bfc.md#sui_abfc_new">new</a>(ctx: &<b>mut</b> TxContext): Anonymous_Balance&lt;<a href="../sui/anonymous_bfc.md#sui_abfc_ABFC">ABFC</a>&gt; {
    <b>assert</b>!(<a href="../sui/tx_context.md#sui_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="../sui/anonymous_bfc.md#sui_abfc_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> (treasury, metadata) = <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">anonymous_coin::create_currency</a>(
        <a href="../sui/anonymous_bfc.md#sui_abfc_ABFC">ABFC</a> {},
        9,
        b"<a href="../sui/anonymous_bfc.md#sui_abfc_ABFC">ABFC</a>",
        b"ABfc",
        // TODO: add appropriate description and logo <a href="../sui/url.md#sui_url">url</a>
        b"",
        option::none(),
        ctx
    );
    <a href="../sui/transfer.md#sui_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = treasury.treasury_into_supply();
    <b>let</b> total_sui = supply.increase_supply(<a href="../sui/anonymous_bfc.md#sui_abfc_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>, ctx);
    supply.destroy_supply();
    total_sui
}
</code></pre>



</details>

<a name="sui_abfc_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/transfer.md#sui_transfer">transfer</a>(c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;<a href="../sui/anonymous_bfc.md#sui_abfc_ABFC">sui::abfc::ABFC</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/transfer.md#sui_transfer">transfer</a>(c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">anonymous_coin::Anonymous_Coin</a>&lt;<a href="../sui/anonymous_bfc.md#sui_abfc_ABFC">ABFC</a>&gt;, recipient: <b>address</b>) {
    //prepare..todo
    //kakaxi: need prepare...
    <a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>

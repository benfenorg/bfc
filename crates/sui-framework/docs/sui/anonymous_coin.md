---
title: Module `sui::anonymous_coin`
---

Defines the <code>Coin</code> type - platform wide representation of fungible
tokens and coins. <code>Coin</code> can be described as a secure wrapper around
<code>Balance</code> type.


-  [Struct `Anonymous_Coin`](#sui_anonymous_coin_Anonymous_Coin)
-  [Struct `SwapPool`](#sui_anonymous_coin_SwapPool)
-  [Struct `CoinMetadata`](#sui_anonymous_coin_CoinMetadata)
-  [Struct `RegulatedCoinMetadata`](#sui_anonymous_coin_RegulatedCoinMetadata)
-  [Struct `TreasuryCap`](#sui_anonymous_coin_TreasuryCap)
-  [Struct `DenyCapV2`](#sui_anonymous_coin_DenyCapV2)
-  [Struct `CurrencyCreated`](#sui_anonymous_coin_CurrencyCreated)
-  [Struct `DenyCap`](#sui_anonymous_coin_DenyCap)
-  [Constants](#@Constants_0)
-  [Function `bind_swap_pool`](#sui_anonymous_coin_bind_swap_pool)
-  [Function `swap_out_with_amount`](#sui_anonymous_coin_swap_out_with_amount)
-  [Function `swap_in`](#sui_anonymous_coin_swap_in)
-  [Function `total_supply`](#sui_anonymous_coin_total_supply)
-  [Function `treasury_into_supply`](#sui_anonymous_coin_treasury_into_supply)
-  [Function `supply_immut`](#sui_anonymous_coin_supply_immut)
-  [Function `supply_mut`](#sui_anonymous_coin_supply_mut)
-  [Function `balance`](#sui_anonymous_coin_balance)
-  [Function `balance_mut`](#sui_anonymous_coin_balance_mut)
-  [Function `from_balance`](#sui_anonymous_coin_from_balance)
-  [Function `into_balance`](#sui_anonymous_coin_into_balance)
-  [Function `take`](#sui_anonymous_coin_take)
-  [Function `put`](#sui_anonymous_coin_put)
-  [Function `join`](#sui_anonymous_coin_join)
-  [Function `split`](#sui_anonymous_coin_split)
-  [Function `zero`](#sui_anonymous_coin_zero)
-  [Function `destroy_zero`](#sui_anonymous_coin_destroy_zero)
-  [Function `create_currency`](#sui_anonymous_coin_create_currency)
-  [Function `create_regulated_currency_v2`](#sui_anonymous_coin_create_regulated_currency_v2)
-  [Function `migrate_regulated_currency_to_v2`](#sui_anonymous_coin_migrate_regulated_currency_to_v2)
-  [Function `mint`](#sui_anonymous_coin_mint)
-  [Function `mint_balance`](#sui_anonymous_coin_mint_balance)
-  [Function `burn`](#sui_anonymous_coin_burn)
-  [Function `deny_list_v2_add`](#sui_anonymous_coin_deny_list_v2_add)
-  [Function `deny_list_v2_remove`](#sui_anonymous_coin_deny_list_v2_remove)
-  [Function `deny_list_v2_contains_current_epoch`](#sui_anonymous_coin_deny_list_v2_contains_current_epoch)
-  [Function `deny_list_v2_contains_next_epoch`](#sui_anonymous_coin_deny_list_v2_contains_next_epoch)
-  [Function `deny_list_v2_enable_global_pause`](#sui_anonymous_coin_deny_list_v2_enable_global_pause)
-  [Function `deny_list_v2_disable_global_pause`](#sui_anonymous_coin_deny_list_v2_disable_global_pause)
-  [Function `deny_list_v2_is_global_pause_enabled_current_epoch`](#sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_current_epoch)
-  [Function `deny_list_v2_is_global_pause_enabled_next_epoch`](#sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_next_epoch)
-  [Function `mint_and_transfer`](#sui_anonymous_coin_mint_and_transfer)
-  [Function `update_name`](#sui_anonymous_coin_update_name)
-  [Function `update_symbol`](#sui_anonymous_coin_update_symbol)
-  [Function `update_description`](#sui_anonymous_coin_update_description)
-  [Function `update_icon_url`](#sui_anonymous_coin_update_icon_url)
-  [Function `get_decimals`](#sui_anonymous_coin_get_decimals)
-  [Function `get_name`](#sui_anonymous_coin_get_name)
-  [Function `get_symbol`](#sui_anonymous_coin_get_symbol)
-  [Function `get_description`](#sui_anonymous_coin_get_description)
-  [Function `get_icon_url`](#sui_anonymous_coin_get_icon_url)
-  [Function `supply`](#sui_anonymous_coin_supply)
-  [Function `create_regulated_currency`](#sui_anonymous_coin_create_regulated_currency)
-  [Function `deny_list_add`](#sui_anonymous_coin_deny_list_add)
-  [Function `deny_list_remove`](#sui_anonymous_coin_deny_list_remove)
-  [Function `deny_list_contains`](#sui_anonymous_coin_deny_list_contains)


<pre><code><b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance">sui::anonymous_balance</a>;
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



<a name="sui_anonymous_coin_Anonymous_Coin"></a>

## Struct `Anonymous_Coin`

A coin of type <code>T</code> worth <code>value</code>. Transferable and storable


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_anonymous_coin_SwapPool"></a>

## Struct `SwapPool`



<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_SwapPool">SwapPool</a>&lt;<b>phantom</b> T1, <b>phantom</b> T2&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T1&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>normal_coin: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T2&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>swap_rate: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_availalbe_normal_coin: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_anonymous_coin_CoinMetadata"></a>

## Struct `CoinMetadata`

Each Coin type T created through <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">create_currency</a></code> function will have a
unique instance of CoinMetadata<T> that stores the metadata for this coin type.


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>decimals: u8</code>
</dt>
<dd>
 Number of decimal places the coin uses.
 A coin with <code>value </code> N and <code>decimals</code> D should be shown as N / 10^D
 E.g., a coin with <code>value</code> 7002 and decimals 3 should be displayed as 7.002
 This is metadata for display usage only.
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Name for the token
</dd>
<dt>
<code>symbol: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
 Symbol for the token
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Description of the token
</dd>
<dt>
<code>icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/url.md#sui_url_Url">sui::url::Url</a>&gt;</code>
</dt>
<dd>
 URL for the token logo
</dd>
</dl>


</details>

<a name="sui_anonymous_coin_RegulatedCoinMetadata"></a>

## Struct `RegulatedCoinMetadata`

Similar to CoinMetadata, but created only for regulated coins that use the DenyList.
This object is always immutable.


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_RegulatedCoinMetadata">RegulatedCoinMetadata</a>&lt;<b>phantom</b> T&gt; <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>coin_metadata_object: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the coin's CoinMetadata object.
</dd>
<dt>
<code>deny_cap_object: <a href="../sui/object.md#sui_object_ID">sui::object::ID</a></code>
</dt>
<dd>
 The ID of the coin's DenyCap object.
</dd>
</dl>


</details>

<a name="sui_anonymous_coin_TreasuryCap"></a>

## Struct `TreasuryCap`

Capability allowing the bearer to mint and burn
coins of type <code>T</code>. Transferable


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_anonymous_coin_DenyCapV2"></a>

## Struct `DenyCapV2`

Capability allowing the bearer to deny addresses from using the currency's coins--
immediately preventing those addresses from interacting with the coin as an input to a
transaction and at the start of the next preventing them from receiving the coin.
If <code>allow_global_pause</code> is true, the bearer can enable a global pause that behaves as if
all addresses were added to the deny list.


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>allow_global_pause: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_anonymous_coin_CurrencyCreated"></a>

## Struct `CurrencyCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CurrencyCreated">CurrencyCreated</a>&lt;<b>phantom</b> T&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>decimals: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_anonymous_coin_DenyCap"></a>

## Struct `DenyCap`

Capability allowing the bearer to freeze addresses, preventing those addresses from
interacting with the coin as an input to a transaction.


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="sui_anonymous_coin_DENY_LIST_COIN_INDEX"></a>

The index into the deny list vector for the <code><a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a></code> type.


<pre><code><b>const</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>: u64 = 0;
</code></pre>



<a name="sui_anonymous_coin_EBadWitness"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_EBadWitness">EBadWitness</a>: u64 = 0;
</code></pre>



<a name="sui_anonymous_coin_EGlobalPauseNotAllowed"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_EGlobalPauseNotAllowed">EGlobalPauseNotAllowed</a>: u64 = 3;
</code></pre>



<a name="sui_anonymous_coin_ENotEnough"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_ENotEnough">ENotEnough</a>: u64 = 2;
</code></pre>



<a name="sui_anonymous_coin_bind_swap_pool"></a>

## Function `bind_swap_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_bind_swap_pool">bind_swap_pool</a>&lt;T1, T2&gt;(<a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T1&gt;, <a href="../sui/coin.md#sui_coin">coin</a>: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T2&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_bind_swap_pool">bind_swap_pool</a>&lt;T1, T2&gt;(<a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T1&gt;, <a href="../sui/coin.md#sui_coin">coin</a>: Coin&lt;T2&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> value = <a href="../sui/coin.md#sui_coin">coin</a>.value();
    <a href="../sui/transfer.md#sui_transfer_share_object">transfer::share_object</a>(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_SwapPool">SwapPool</a> {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        <a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>,
        normal_coin: <a href="../sui/coin.md#sui_coin">coin</a>,
        swap_rate: 1,
        max_availalbe_normal_coin: value,
    })
}
</code></pre>



</details>

<a name="sui_anonymous_coin_swap_out_with_amount"></a>

## Function `swap_out_with_amount`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_swap_out_with_amount">swap_out_with_amount</a>&lt;T1, T2&gt;(<a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T1&gt;, swap_out_amount: u64, swap_pool: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_SwapPool">sui::anonymous_coin::SwapPool</a>&lt;T1, T2&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_swap_out_with_amount">swap_out_with_amount</a>&lt;T1, T2&gt;(<a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T1&gt;,
                                              swap_out_amount: u64,
                                              swap_pool: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_SwapPool">SwapPool</a>&lt;T1, T2&gt;,
                                              ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(swap_out_amount &lt;= swap_pool.max_availalbe_normal_coin, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_ENotEnough">ENotEnough</a>);
    <b>let</b> swap_out_acoin = <a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_split">split</a>(swap_out_amount, ctx);
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_join">join</a>(&<b>mut</b> swap_pool.<a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>, swap_out_acoin);
    <b>let</b> normal_coin = <a href="../sui/coin.md#sui_coin_split">coin::split</a>(&<b>mut</b> swap_pool.normal_coin, swap_out_amount, ctx);
    swap_pool.max_availalbe_normal_coin = swap_pool.max_availalbe_normal_coin - swap_out_amount;
    <a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(normal_coin, <a href="../sui/tx_context.md#sui_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="sui_anonymous_coin_swap_in"></a>

## Function `swap_in`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_swap_in">swap_in</a>&lt;T1, T2&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T2&gt;, swap_pool: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_SwapPool">sui::anonymous_coin::SwapPool</a>&lt;T1, T2&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>entry</b> <b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_swap_in">swap_in</a>&lt;T1, T2&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: Coin&lt;T2&gt;, swap_pool: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_SwapPool">SwapPool</a>&lt;T1, T2&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> value = <a href="../sui/coin.md#sui_coin_balance">coin::balance</a>(&<a href="../sui/coin.md#sui_coin">coin</a>).value();
    <a href="../sui/coin.md#sui_coin_join">coin::join</a>(&<b>mut</b> swap_pool.normal_coin, <a href="../sui/coin.md#sui_coin">coin</a>);
    swap_pool.max_availalbe_normal_coin = swap_pool.max_availalbe_normal_coin + value;
    <b>let</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a> = <a href="../sui/anonymous_coin.md#sui_anonymous_coin_split">split</a>(&<b>mut</b> swap_pool.<a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>, value, ctx);
    <a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(<a href="../sui/anonymous_coin.md#sui_anonymous_coin">anonymous_coin</a>, <a href="../sui/tx_context.md#sui_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="sui_anonymous_coin_total_supply"></a>

## Function `total_supply`

Return the total number of <code>T</code>'s in circulation.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>&lt;T&gt;(cap: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>&lt;T&gt;(cap: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): u64 {
    <a href="../sui/anonymous_balance.md#sui_anonymous_balance_supply_value">anonymous_balance::supply_value</a>(&cap.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_treasury_into_supply"></a>

## Function `treasury_into_supply`

Unwrap <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a></code> getting the <code>Supply</code>.

Operation is irreversible. Supply cannot be converted into a <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a></code> due
to different security guarantees (TreasuryCap can be created only once for a type)


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_treasury_into_supply">treasury_into_supply</a>&lt;T&gt;(treasury: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_treasury_into_supply">treasury_into_supply</a>&lt;T&gt;(treasury: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): Supply&lt;T&gt; {
    <b>let</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a> { id, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a> } = treasury;
    id.delete();
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="sui_anonymous_coin_supply_immut"></a>

## Function `supply_immut`

Get immutable reference to the treasury's <code>Supply</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_supply_immut">supply_immut</a>&lt;T&gt;(treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;): &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_supply_immut">supply_immut</a>&lt;T&gt;(treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): &Supply&lt;T&gt; {
    &treasury.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="sui_anonymous_coin_supply_mut"></a>

## Function `supply_mut`

Get mutable reference to the treasury's <code>Supply</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_supply_mut">supply_mut</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;): &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_supply_mut">supply_mut</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): &<b>mut</b> Supply&lt;T&gt; {
    &<b>mut</b> treasury.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="sui_anonymous_coin_balance"></a>

## Function `balance`

Get immutable reference to the balance of a coin.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/balance.md#sui_balance">balance</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;): &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/balance.md#sui_balance">balance</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;): &Anonymous_Balance&lt;T&gt; {
    &<a href="../sui/coin.md#sui_coin">coin</a>.<a href="../sui/balance.md#sui_balance">balance</a>
}
</code></pre>



</details>

<a name="sui_anonymous_coin_balance_mut"></a>

## Function `balance_mut`

Get a mutable reference to the balance of a coin.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_balance_mut">balance_mut</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;): &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_balance_mut">balance_mut</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;): &<b>mut</b> Anonymous_Balance&lt;T&gt; {
    &<b>mut</b> <a href="../sui/coin.md#sui_coin">coin</a>.<a href="../sui/balance.md#sui_balance">balance</a>
}
</code></pre>



</details>

<a name="sui_anonymous_coin_from_balance"></a>

## Function `from_balance`

Wrap a balance into a Coin to make it transferable.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_from_balance">from_balance</a>&lt;T&gt;(<a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_from_balance">from_balance</a>&lt;T&gt;(<a href="../sui/balance.md#sui_balance">balance</a>: Anonymous_Balance&lt;T&gt;, ctx: &<b>mut</b> TxContext): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt; {
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> { id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx), <a href="../sui/balance.md#sui_balance">balance</a> }
}
</code></pre>



</details>

<a name="sui_anonymous_coin_into_balance"></a>

## Function `into_balance`

Destruct a Coin wrapper and keep the balance.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_into_balance">into_balance</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_into_balance">into_balance</a>&lt;T&gt;(<a href="../sui/coin.md#sui_coin">coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;): Anonymous_Balance&lt;T&gt; {
    <b>let</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> { id, <a href="../sui/balance.md#sui_balance">balance</a> } = <a href="../sui/coin.md#sui_coin">coin</a>;
    id.delete();
    <a href="../sui/balance.md#sui_balance">balance</a>
}
</code></pre>



</details>

<a name="sui_anonymous_coin_take"></a>

## Function `take`

Take a <code>Coin</code> worth of <code>value</code> from <code>Balance</code>.
Aborts if <code>value &gt; <a href="../sui/balance.md#sui_balance">balance</a>.value</code>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_take">take</a>&lt;T&gt;(<a href="../sui/balance.md#sui_balance">balance</a>: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, value: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_take">take</a>&lt;T&gt;(
    <a href="../sui/balance.md#sui_balance">balance</a>: &<b>mut</b> Anonymous_Balance&lt;T&gt;, value: u64, ctx: &<b>mut</b> TxContext,
): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt; {
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        <a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/balance.md#sui_balance">balance</a>.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_split">split</a>(value)
    }
}
</code></pre>



</details>

<a name="sui_anonymous_coin_put"></a>

## Function `put`

Put a <code>Coin&lt;T&gt;</code> to the <code>Balance&lt;T&gt;</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_put">put</a>&lt;T&gt;(<a href="../sui/balance.md#sui_balance">balance</a>: &<b>mut</b> <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;, <a href="../sui/coin.md#sui_coin">coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_put">put</a>&lt;T&gt;(<a href="../sui/balance.md#sui_balance">balance</a>: &<b>mut</b> Anonymous_Balance&lt;T&gt;, <a href="../sui/coin.md#sui_coin">coin</a>: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;) {
    <a href="../sui/balance.md#sui_balance">balance</a>.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_join">join</a>(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_into_balance">into_balance</a>(<a href="../sui/coin.md#sui_coin">coin</a>));
}
</code></pre>



</details>

<a name="sui_anonymous_coin_join"></a>

## Function `join`

Consume the coin <code>c</code> and add its value to <code>self</code>.
Aborts if <code>c.value + self.value &gt; U64_MAX</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;, c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;) {
    <b>let</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> { id, <a href="../sui/balance.md#sui_balance">balance</a> } = c;
    id.delete();
    self.<a href="../sui/balance.md#sui_balance">balance</a>.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_join">join</a>(<a href="../sui/balance.md#sui_balance">balance</a>);
}
</code></pre>



</details>

<a name="sui_anonymous_coin_split"></a>

## Function `split`

Split coin <code>self</code> to two coins, one with balance <code>split_amount</code>,
and the remaining balance is left is <code>self</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_split">split</a>&lt;T&gt;(self: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_split">split</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> TxContext
): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt; {
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_take">take</a>(&<b>mut</b> self.<a href="../sui/balance.md#sui_balance">balance</a>, split_amount, ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_zero"></a>

## Function `zero`

Make any Coin with a zero value. Useful for placeholding
bids/payments or preemptively making empty balances.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_zero">zero</a>&lt;T&gt;(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_zero">zero</a>&lt;T&gt;(ctx: &<b>mut</b> TxContext): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt; {
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> { id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx), <a href="../sui/balance.md#sui_balance">balance</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_zero">anonymous_balance::zero</a>() }
}
</code></pre>



</details>

<a name="sui_anonymous_coin_destroy_zero"></a>

## Function `destroy_zero`

Destroy a coin with value zero


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_destroy_zero">destroy_zero</a>&lt;T&gt;(c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, signatures: vector&lt;u8&gt;, publickey: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_destroy_zero">destroy_zero</a>&lt;T&gt;(c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;, signatures: vector&lt;u8&gt;, publickey: vector&lt;u8&gt;) {
    <b>let</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> { id, <a href="../sui/balance.md#sui_balance">balance</a> } = c;
    <b>let</b> <b>address</b> = <a href="../sui/object.md#sui_object_uid_to_address">object::uid_to_address</a>(&id);
    id.delete();
    <a href="../sui/balance.md#sui_balance">balance</a>.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_destroy_zero">destroy_zero</a>(signatures, <b>address</b>, publickey)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_create_currency"></a>

## Function `create_currency`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">create_currency</a>&lt;T: drop&gt;(witness: T, decimals: u8, symbol: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/url.md#sui_url_Url">sui::url::Url</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">create_currency</a>&lt;T: drop&gt;(
    witness: T,
    decimals: u8,
    symbol: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    icon_url: Option&lt;Url&gt;,
    ctx: &<b>mut</b> TxContext
): (<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;) {
    // Make sure there's only one instance of the type T
    //todo: open witness <b>for</b> ABFC
    <b>assert</b>!(<a href="../sui/types.md#sui_types_is_one_time_witness">sui::types::is_one_time_witness</a>(&witness), <a href="../sui/anonymous_coin.md#sui_anonymous_coin_EBadWitness">EBadWitness</a>);
    (
        <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a> {
            id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
            <a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>: <a href="../sui/anonymous_balance.md#sui_anonymous_balance_create_supply">anonymous_balance::create_supply</a>(witness)
        },
        <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a> {
            id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
            decimals,
            name: string::utf8(name),
            symbol: ascii::string(symbol),
            description: string::utf8(description),
            icon_url
        }
    )
}
</code></pre>



</details>

<a name="sui_anonymous_coin_create_regulated_currency_v2"></a>

## Function `create_regulated_currency_v2`

This creates a new currency, via <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">create_currency</a></code>, but with an extra capability that
allows for specific addresses to have their coins frozen. When an address is added to the
deny list, it is immediately unable to interact with the currency's coin as input objects.
Additionally at the start of the next epoch, they will be unable to receive the currency's
coin.
The <code>allow_global_pause</code> flag enables an additional API that will cause all addresses to be
be denied. Note however, that this doesn't affect per-address entries of the deny list and
will not change the result of the "contains" APIs.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_regulated_currency_v2">create_regulated_currency_v2</a>&lt;T: drop&gt;(witness: T, decimals: u8, symbol: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/url.md#sui_url_Url">sui::url::Url</a>&gt;, allow_global_pause: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">sui::anonymous_coin::DenyCapV2</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_regulated_currency_v2">create_regulated_currency_v2</a>&lt;T: drop&gt;(
    witness: T,
    decimals: u8,
    symbol: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    icon_url: Option&lt;Url&gt;,
    allow_global_pause: bool,
    ctx: &<b>mut</b> TxContext,
): (<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;) {
    <b>let</b> (treasury_cap, metadata) = <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">create_currency</a>(
        witness,
        decimals,
        symbol,
        name,
        description,
        icon_url,
        ctx
    );
    <b>let</b> deny_cap = <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a> {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        allow_global_pause,
    };
    <a href="../sui/transfer.md#sui_transfer_freeze_object">transfer::freeze_object</a>(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_RegulatedCoinMetadata">RegulatedCoinMetadata</a>&lt;T&gt; {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        coin_metadata_object: <a href="../sui/object.md#sui_object_id">object::id</a>(&metadata),
        deny_cap_object: <a href="../sui/object.md#sui_object_id">object::id</a>(&deny_cap),
    });
    (treasury_cap, deny_cap, metadata)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_migrate_regulated_currency_to_v2"></a>

## Function `migrate_regulated_currency_to_v2`

Given the <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a></code> for a regulated currency, migrate it to the new <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a></code> type.
All entries in the deny list will be migrated to the new format.
See <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_regulated_currency_v2">create_regulated_currency_v2</a></code> for details on the new v2 of the deny list.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_migrate_regulated_currency_to_v2">migrate_regulated_currency_to_v2</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> <a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, cap: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">sui::anonymous_coin::DenyCap</a>&lt;T&gt;, allow_global_pause: bool, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">sui::anonymous_coin::DenyCapV2</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_migrate_regulated_currency_to_v2">migrate_regulated_currency_to_v2</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    cap: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a>&lt;T&gt;,
    allow_global_pause: bool,
    ctx: &<b>mut</b> TxContext,
): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt; {
    <b>let</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a> { id } = cap;
    <a href="../sui/object.md#sui_object_delete">object::delete</a>(id);
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.migrate_v1_to_v2(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx);
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a> {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        allow_global_pause,
    }
}
</code></pre>



</details>

<a name="sui_anonymous_coin_mint"></a>

## Function `mint`

Create a coin worth <code>value</code> and increase the total supply
in <code>cap</code> accordingly.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint">mint</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, value: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint">mint</a>&lt;T&gt;(
    cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, value: u64, ctx: &<b>mut</b> TxContext,
): <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt; {
    <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        <a href="../sui/balance.md#sui_balance">balance</a>: cap.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>.increase_supply(value)
    }
}
</code></pre>



</details>

<a name="sui_anonymous_coin_mint_balance"></a>

## Function `mint_balance`

Mint some amount of T as a <code>Balance</code> and increase the total
supply in <code>cap</code> accordingly.
Aborts if <code>value</code> + <code>cap.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a></code> >= U64_MAX


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint_balance">mint_balance</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, value: u64): <a href="../sui/anonymous_balance.md#sui_anonymous_balance_Anonymous_Balance">sui::anonymous_balance::Anonymous_Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint_balance">mint_balance</a>&lt;T&gt;(
    cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, value: u64
): Anonymous_Balance&lt;T&gt; {
    cap.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>.increase_supply(value)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_burn"></a>

## Function `burn`

Destroy the coin <code>c</code> and decrease the total supply in <code>cap</code>
accordingly.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_burn">burn</a>&lt;T&gt;(cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">sui::anonymous_coin::Anonymous_Coin</a>&lt;T&gt;, signatures: vector&lt;u8&gt;, anonymous_coin_id: <b>address</b>, publickey: vector&lt;u8&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_burn">burn</a>&lt;T&gt;(
    cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;,
    c: <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a>&lt;T&gt;,
    signatures: vector&lt;u8&gt;,
    anonymous_coin_id: <b>address</b>,
    publickey: vector&lt;u8&gt;
): u64 {
    <b>let</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_Anonymous_Coin">Anonymous_Coin</a> { id, <a href="../sui/balance.md#sui_balance">balance</a> } = c;
    id.delete();
    cap.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>.decrease_supply(<a href="../sui/balance.md#sui_balance">balance</a>, signatures, anonymous_coin_id, publickey)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_add"></a>

## Function `deny_list_v2_add`

Adds the given address to the deny list, preventing it from interacting with the specified
coin type as an input to a transaction. Additionally at the start of the next epoch, the
address will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_add">deny_list_v2_add</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> <a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">sui::anonymous_coin::DenyCapV2</a>&lt;T&gt;, addr: <b>address</b>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_add">deny_list_v2_add</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    addr: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_add(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr, ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_remove"></a>

## Function `deny_list_v2_remove`

Removes an address from the deny list. Similar to <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_add">deny_list_v2_add</a></code>, the effect for input
objects will be immediate, but the effect for receiving objects will be delayed until the
next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_remove">deny_list_v2_remove</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> <a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">sui::anonymous_coin::DenyCapV2</a>&lt;T&gt;, addr: <b>address</b>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_remove">deny_list_v2_remove</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    addr: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_remove(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr, ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_contains_current_epoch"></a>

## Function `deny_list_v2_contains_current_epoch`

Check if the deny list contains the given address for the current epoch. Denied addresses
in the current epoch will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_contains_current_epoch">deny_list_v2_contains_current_epoch</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, addr: <b>address</b>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_contains_current_epoch">deny_list_v2_contains_current_epoch</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &DenyList,
    addr: <b>address</b>,
    ctx: &TxContext,
): bool {
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_contains_current_epoch(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr, ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_contains_next_epoch"></a>

## Function `deny_list_v2_contains_next_epoch`

Check if the deny list contains the given address for the next epoch. Denied addresses in
the next epoch will immediately be unable to use objects of this coin type as inputs. At the
start of the next epoch, the address will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_contains_next_epoch">deny_list_v2_contains_next_epoch</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_contains_next_epoch">deny_list_v2_contains_next_epoch</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &DenyList,
    addr: <b>address</b>,
): bool {
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_contains_next_epoch(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, addr)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_enable_global_pause"></a>

## Function `deny_list_v2_enable_global_pause`

Enable the global pause for the given coin type. This will immediately prevent all addresses
from using objects of this coin type as inputs. At the start of the next epoch, all
addresses will be unable to receive objects of this coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_enable_global_pause">deny_list_v2_enable_global_pause</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> <a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">sui::anonymous_coin::DenyCapV2</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_enable_global_pause">deny_list_v2_enable_global_pause</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(deny_cap.allow_global_pause, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_EGlobalPauseNotAllowed">EGlobalPauseNotAllowed</a>);
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_enable_global_pause(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_disable_global_pause"></a>

## Function `deny_list_v2_disable_global_pause`

Disable the global pause for the given coin type. This will immediately allow all addresses
to resume using objects of this coin type as inputs. However, receiving objects of this coin
type will still be paused until the start of the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_disable_global_pause">deny_list_v2_disable_global_pause</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> <a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">sui::anonymous_coin::DenyCapV2</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_disable_global_pause">deny_list_v2_disable_global_pause</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCapV2">DenyCapV2</a>&lt;T&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(deny_cap.allow_global_pause, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_EGlobalPauseNotAllowed">EGlobalPauseNotAllowed</a>);
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_disable_global_pause(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_current_epoch"></a>

## Function `deny_list_v2_is_global_pause_enabled_current_epoch`

Check if the global pause is enabled for the given coin type in the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_current_epoch">deny_list_v2_is_global_pause_enabled_current_epoch</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_current_epoch">deny_list_v2_is_global_pause_enabled_current_epoch</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &DenyList,
    ctx: &TxContext,
): bool {
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_is_global_pause_enabled_current_epoch(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty, ctx)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_next_epoch"></a>

## Function `deny_list_v2_is_global_pause_enabled_next_epoch`

Check if the global pause is enabled for the given coin type in the next epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_next_epoch">deny_list_v2_is_global_pause_enabled_next_epoch</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_v2_is_global_pause_enabled_next_epoch">deny_list_v2_is_global_pause_enabled_next_epoch</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &DenyList,
): bool {
    <b>let</b> ty = type_name::get_with_original_ids&lt;T&gt;().into_string().into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v2_is_global_pause_enabled_next_epoch(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, ty)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_mint_and_transfer"></a>

## Function `mint_and_transfer`

Mint <code>amount</code> of <code>Coin</code> and send it to <code>recipient</code>. Invokes <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint">mint</a>()</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint_and_transfer">mint_and_transfer</a>&lt;T&gt;(c: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, amount: u64, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint_and_transfer">mint_and_transfer</a>&lt;T&gt;(
    c: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, amount: u64, recipient: <b>address</b>, ctx: &<b>mut</b> TxContext
) {
    <a href="../sui/transfer.md#sui_transfer_public_transfer">transfer::public_transfer</a>(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_mint">mint</a>(c, amount, ctx), recipient)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_update_name"></a>

## Function `update_name`

Update name of the coin in <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_name">update_name</a>&lt;T&gt;(_treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_name">update_name</a>&lt;T&gt;(
    _treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;, name: string::String
) {
    metadata.name = name;
}
</code></pre>



</details>

<a name="sui_anonymous_coin_update_symbol"></a>

## Function `update_symbol`

Update the symbol of the coin in <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_symbol">update_symbol</a>&lt;T&gt;(_treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;, symbol: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_symbol">update_symbol</a>&lt;T&gt;(
    _treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;, symbol: ascii::String
) {
    metadata.symbol = symbol;
}
</code></pre>



</details>

<a name="sui_anonymous_coin_update_description"></a>

## Function `update_description`

Update the description of the coin in <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_description">update_description</a>&lt;T&gt;(_treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;, description: <a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_description">update_description</a>&lt;T&gt;(
    _treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;, description: string::String
) {
    metadata.description = description;
}
</code></pre>



</details>

<a name="sui_anonymous_coin_update_icon_url"></a>

## Function `update_icon_url`

Update the url of the coin in <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a></code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_icon_url">update_icon_url</a>&lt;T&gt;(_treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;, <a href="../sui/url.md#sui_url">url</a>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_update_icon_url">update_icon_url</a>&lt;T&gt;(
    _treasury: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, metadata: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;, <a href="../sui/url.md#sui_url">url</a>: ascii::String
) {
    metadata.icon_url = option::some(<a href="../sui/url.md#sui_url_new_unsafe">url::new_unsafe</a>(<a href="../sui/url.md#sui_url">url</a>));
}
</code></pre>



</details>

<a name="sui_anonymous_coin_get_decimals"></a>

## Function `get_decimals`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_decimals">get_decimals</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_decimals">get_decimals</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): u8 {
    metadata.decimals
}
</code></pre>



</details>

<a name="sui_anonymous_coin_get_name"></a>

## Function `get_name`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_name">get_name</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_name">get_name</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): string::String {
    metadata.name
}
</code></pre>



</details>

<a name="sui_anonymous_coin_get_symbol"></a>

## Function `get_symbol`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_symbol">get_symbol</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_symbol">get_symbol</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): ascii::String {
    metadata.symbol
}
</code></pre>



</details>

<a name="sui_anonymous_coin_get_description"></a>

## Function `get_description`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_description">get_description</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_description">get_description</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): string::String {
    metadata.description
}
</code></pre>



</details>

<a name="sui_anonymous_coin_get_icon_url"></a>

## Function `get_icon_url`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_icon_url">get_icon_url</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/url.md#sui_url_Url">sui::url::Url</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_get_icon_url">get_icon_url</a>&lt;T&gt;(metadata: &<a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;): Option&lt;Url&gt; {
    metadata.icon_url
}
</code></pre>



</details>

<a name="sui_anonymous_coin_supply"></a>

## Function `supply`



<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_supply">supply</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;): &<a href="../sui/anonymous_balance.md#sui_anonymous_balance_Supply">sui::anonymous_balance::Supply</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_supply">supply</a>&lt;T&gt;(treasury: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;): &Supply&lt;T&gt; {
    &treasury.<a href="../sui/anonymous_coin.md#sui_anonymous_coin_total_supply">total_supply</a>
}
</code></pre>



</details>

<a name="sui_anonymous_coin_create_regulated_currency"></a>

## Function `create_regulated_currency`

This creates a new currency, via <code><a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">create_currency</a></code>, but with an extra capability that
allows for specific addresses to have their coins frozen. Those addresses cannot interact
with the coin as input objects.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_regulated_currency">create_regulated_currency</a>&lt;T: drop&gt;(witness: T, decimals: u8, symbol: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, icon_url: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/url.md#sui_url_Url">sui::url::Url</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">sui::anonymous_coin::TreasuryCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">sui::anonymous_coin::DenyCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">sui::anonymous_coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_regulated_currency">create_regulated_currency</a>&lt;T: drop&gt;(
    witness: T,
    decimals: u8,
    symbol: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    icon_url: Option&lt;Url&gt;,
    ctx: &<b>mut</b> TxContext
): (<a href="../sui/anonymous_coin.md#sui_anonymous_coin_TreasuryCap">TreasuryCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a>&lt;T&gt;, <a href="../sui/anonymous_coin.md#sui_anonymous_coin_CoinMetadata">CoinMetadata</a>&lt;T&gt;) {
    <b>let</b> (treasury_cap, metadata) = <a href="../sui/anonymous_coin.md#sui_anonymous_coin_create_currency">create_currency</a>(
        witness,
        decimals,
        symbol,
        name,
        description,
        icon_url,
        ctx
    );
    <b>let</b> deny_cap = <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a> {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
    };
    <a href="../sui/transfer.md#sui_transfer_freeze_object">transfer::freeze_object</a>(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_RegulatedCoinMetadata">RegulatedCoinMetadata</a>&lt;T&gt; {
        id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        coin_metadata_object: <a href="../sui/object.md#sui_object_id">object::id</a>(&metadata),
        deny_cap_object: <a href="../sui/object.md#sui_object_id">object::id</a>(&deny_cap),
    });
    (treasury_cap, deny_cap, metadata)
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_add"></a>

## Function `deny_list_add`

Adds the given address to the deny list, preventing it
from interacting with the specified coin type as an input to a transaction.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_add">deny_list_add</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> <a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">sui::anonymous_coin::DenyCap</a>&lt;T&gt;, addr: <b>address</b>, _ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_add">deny_list_add</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a>&lt;T&gt;,
    addr: <b>address</b>,
    _ctx: &<b>mut</b> TxContext
) {
    <b>let</b> `type` =
        type_name::into_string(type_name::get_with_original_ids&lt;T&gt;()).into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v1_add(
        <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>,
        `type`,
        addr,
    )
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_remove"></a>

## Function `deny_list_remove`

Removes an address from the deny list.
Aborts with <code>ENotFrozen</code> if the address is not already in the list.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_remove">deny_list_remove</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> <a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">sui::anonymous_coin::DenyCap</a>&lt;T&gt;, addr: <b>address</b>, _ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_remove">deny_list_remove</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<b>mut</b> DenyList,
    _deny_cap: &<b>mut</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DenyCap">DenyCap</a>&lt;T&gt;,
    addr: <b>address</b>,
    _ctx: &<b>mut</b> TxContext
) {
    <b>let</b> `type` =
        type_name::into_string(type_name::get_with_original_ids&lt;T&gt;()).into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v1_remove(
        <a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>,
        `type`,
        addr,
    )
}
</code></pre>



</details>

<a name="sui_anonymous_coin_deny_list_contains"></a>

## Function `deny_list_contains`

Returns true iff the given address is denied for the given coin type. It will
return false if given a non-coin type.


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_contains">deny_list_contains</a>&lt;T&gt;(<a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &<a href="../sui/deny_list.md#sui_deny_list_DenyList">sui::deny_list::DenyList</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../sui/anonymous_coin.md#sui_anonymous_coin_deny_list_contains">deny_list_contains</a>&lt;T&gt;(
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>: &DenyList,
    addr: <b>address</b>,
): bool {
    <b>let</b> name = type_name::get_with_original_ids&lt;T&gt;();
    <b>if</b> (type_name::is_primitive(&name)) <b>return</b> <b>false</b>;
    <b>let</b> `type` = type_name::into_string(name).into_bytes();
    <a href="../sui/deny_list.md#sui_deny_list">deny_list</a>.v1_contains(<a href="../sui/anonymous_coin.md#sui_anonymous_coin_DENY_LIST_COIN_INDEX">DENY_LIST_COIN_INDEX</a>, `type`, addr)
}
</code></pre>



</details>

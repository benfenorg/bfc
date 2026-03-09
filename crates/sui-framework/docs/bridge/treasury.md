---
title: Module `bridge::treasury`
---



-  [Struct `BridgeTreasury`](#bridge_treasury_BridgeTreasury)
-  [Struct `BridgeTokenMetadata`](#bridge_treasury_BridgeTokenMetadata)
-  [Struct `ForeignTokenRegistration`](#bridge_treasury_ForeignTokenRegistration)
-  [Struct `UpdateTokenPriceEvent`](#bridge_treasury_UpdateTokenPriceEvent)
-  [Struct `NewTokenEvent`](#bridge_treasury_NewTokenEvent)
-  [Struct `TokenRegistrationEvent`](#bridge_treasury_TokenRegistrationEvent)
-  [Struct `AddExternalCoinAdminEvent`](#bridge_treasury_AddExternalCoinAdminEvent)
-  [Struct `RemoveExternalCoinAdminEvent`](#bridge_treasury_RemoveExternalCoinAdminEvent)
-  [Struct `AddExternalCoinWitnessEvent`](#bridge_treasury_AddExternalCoinWitnessEvent)
-  [Struct `RemoveExternalCoinWitnessEvent`](#bridge_treasury_RemoveExternalCoinWitnessEvent)
-  [Struct `AddExternalCoinTargetEvent`](#bridge_treasury_AddExternalCoinTargetEvent)
-  [Struct `RemoveExternalCoinTargetEvent`](#bridge_treasury_RemoveExternalCoinTargetEvent)
-  [Constants](#@Constants_0)
-  [Function `token_id`](#bridge_treasury_token_id)
-  [Function `decimal_multiplier`](#bridge_treasury_decimal_multiplier)
-  [Function `notional_value`](#bridge_treasury_notional_value)
-  [Function `register_foreign_token`](#bridge_treasury_register_foreign_token)
-  [Function `add_new_token`](#bridge_treasury_add_new_token)
-  [Function `create`](#bridge_treasury_create)
-  [Function `is_external_coin_admin`](#bridge_treasury_is_external_coin_admin)
-  [Function `is_external_coin_witness`](#bridge_treasury_is_external_coin_witness)
-  [Function `external_coin_admin_count`](#bridge_treasury_external_coin_admin_count)
-  [Function `add_external_coin_admin`](#bridge_treasury_add_external_coin_admin)
-  [Function `add_external_coin_target`](#bridge_treasury_add_external_coin_target)
-  [Function `add_external_coin_witness`](#bridge_treasury_add_external_coin_witness)
-  [Function `remove_external_coin_witness`](#bridge_treasury_remove_external_coin_witness)
-  [Function `verify_bitcoin_signatures`](#bridge_treasury_verify_bitcoin_signatures)
-  [Function `remove_external_coin_admin`](#bridge_treasury_remove_external_coin_admin)
-  [Function `remove_external_coin_target`](#bridge_treasury_remove_external_coin_target)
-  [Function `burn`](#bridge_treasury_burn)
-  [Function `mint`](#bridge_treasury_mint)
-  [Function `calculate_amount_in_usd`](#bridge_treasury_calculate_amount_in_usd)
-  [Function `get_amount_in_usd_by_token_id`](#bridge_treasury_get_amount_in_usd_by_token_id)
-  [Function `get_token_amount_by_usd`](#bridge_treasury_get_token_amount_by_usd)
-  [Function `update_asset_notional_price`](#bridge_treasury_update_asset_notional_price)
-  [Function `get_token_metadata`](#bridge_treasury_get_token_metadata)


<pre><code><b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../bridge/crypto.md#bridge_crypto">bridge::crypto</a>;
<b>use</b> <a href="../bridge/message.md#bridge_message">bridge::message</a>;
<b>use</b> <a href="../bridge/message_types.md#bridge_message_types">bridge::message_types</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/ecdsa_k1.md#sui_ecdsa_k1">sui::ecdsa_k1</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_bag.md#sui_object_bag">sui::object_bag</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="bridge_treasury_BridgeTreasury"></a>

## Struct `BridgeTreasury`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>external_coin_admin_address: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>external_coin_target_address: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>external_coin_witness_address: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;vector&lt;u8&gt;&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>treasuries: <a href="../sui/object_bag.md#sui_object_bag_ObjectBag">sui::object_bag::ObjectBag</a></code>
</dt>
<dd>
</dd>
<dt>
<code>supported_tokens: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>, <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">bridge::treasury::BridgeTokenMetadata</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>id_token_type_map: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u64, <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>waiting_room: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_BridgeTokenMetadata"></a>

## Struct `BridgeTokenMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>native_token: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_ForeignTokenRegistration"></a>

## Struct `ForeignTokenRegistration`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>uc: <a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a></code>
</dt>
<dd>
</dd>
<dt>
<code>decimal: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_UpdateTokenPriceEvent"></a>

## Struct `UpdateTokenPriceEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_UpdateTokenPriceEvent">UpdateTokenPriceEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_price: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_NewTokenEvent"></a>

## Struct `NewTokenEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_NewTokenEvent">NewTokenEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>native_token: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_TokenRegistrationEvent"></a>

## Struct `TokenRegistrationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_TokenRegistrationEvent">TokenRegistrationEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>decimal: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>native_token: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_AddExternalCoinAdminEvent"></a>

## Struct `AddExternalCoinAdminEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_AddExternalCoinAdminEvent">AddExternalCoinAdminEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_RemoveExternalCoinAdminEvent"></a>

## Struct `RemoveExternalCoinAdminEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_RemoveExternalCoinAdminEvent">RemoveExternalCoinAdminEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_AddExternalCoinWitnessEvent"></a>

## Struct `AddExternalCoinWitnessEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_AddExternalCoinWitnessEvent">AddExternalCoinWitnessEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><b>address</b>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_RemoveExternalCoinWitnessEvent"></a>

## Struct `RemoveExternalCoinWitnessEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_RemoveExternalCoinWitnessEvent">RemoveExternalCoinWitnessEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><b>address</b>: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_AddExternalCoinTargetEvent"></a>

## Struct `AddExternalCoinTargetEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_AddExternalCoinTargetEvent">AddExternalCoinTargetEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_RemoveExternalCoinTargetEvent"></a>

## Struct `RemoveExternalCoinTargetEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_RemoveExternalCoinTargetEvent">RemoveExternalCoinTargetEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_treasury_EInvalidNotionalValue"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>: u64 = 4;
</code></pre>



<a name="bridge_treasury_EInvalidUpgradeCap"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EInvalidUpgradeCap">EInvalidUpgradeCap</a>: u64 = 2;
</code></pre>



<a name="bridge_treasury_ETokenSupplyNonZero"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_ETokenSupplyNonZero">ETokenSupplyNonZero</a>: u64 = 3;
</code></pre>



<a name="bridge_treasury_EUnsupportedTokenType"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>: u64 = 1;
</code></pre>



<a name="bridge_treasury_token_id"></a>

## Function `token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): u64 {
    <b>let</b> metadata = self.<a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.id
}
</code></pre>



</details>

<a name="bridge_treasury_decimal_multiplier"></a>

## Function `decimal_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): u64 {
    <b>let</b> metadata = self.<a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.<a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>
}
</code></pre>



</details>

<a name="bridge_treasury_notional_value"></a>

## Function `notional_value`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): u64 {
    <b>let</b> metadata = self.<a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>
}
</code></pre>



</details>

<a name="bridge_treasury_register_foreign_token"></a>

## Function `register_foreign_token`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_register_foreign_token">register_foreign_token</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, tc: <a href="../sui/coin.md#sui_coin_TreasuryCap">sui::coin::TreasuryCap</a>&lt;T&gt;, uc: <a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>, metadata: &<a href="../sui/coin.md#sui_coin_CoinMetadata">sui::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_register_foreign_token">register_foreign_token</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    tc: TreasuryCap&lt;T&gt;,
    uc: UpgradeCap,
    metadata: &CoinMetadata&lt;T&gt;,
) {
    // Make sure TreasuryCap <b>has</b> not been minted before.
    <b>assert</b>!(coin::total_supply(&tc) == 0, <a href="../bridge/treasury.md#bridge_treasury_ETokenSupplyNonZero">ETokenSupplyNonZero</a>);
    <b>let</b> type_name = type_name::get&lt;T&gt;();
    <b>let</b> address_bytes = hex::decode(ascii::into_bytes(type_name::get_address(&type_name)));
    <b>let</b> coin_address = address::from_bytes(address_bytes);
    // Make sure upgrade cap is <b>for</b> the Coin package
    // FIXME: add test
    <b>assert</b>!(
        object::id_to_address(&package::upgrade_package(&uc))
            == coin_address, <a href="../bridge/treasury.md#bridge_treasury_EInvalidUpgradeCap">EInvalidUpgradeCap</a>
    );
    <b>let</b> registration = <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a> {
        type_name,
        uc,
        decimal: coin::get_decimals(metadata),
    };
    self.waiting_room.add(type_name::into_string(type_name), registration);
    self.treasuries.add(type_name, tc);
    emit(<a href="../bridge/treasury.md#bridge_treasury_TokenRegistrationEvent">TokenRegistrationEvent</a>{
        type_name,
        decimal: coin::get_decimals(metadata),
        native_token: <b>false</b>
    });
}
</code></pre>



</details>

<a name="bridge_treasury_add_new_token"></a>

## Function `add_new_token`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_new_token">add_new_token</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, token_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64, native_token: bool, <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_new_token">add_new_token</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    token_name: String,
    <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64,
    native_token: bool,
    <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64,
) {
    <b>if</b> (!native_token){
        <b>assert</b>!(<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> &gt; 0, <a href="../bridge/treasury.md#bridge_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>);
        <b>let</b> <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a>{
            type_name,
            uc,
            decimal,
        } = self.waiting_room.remove&lt;String, <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a>&gt;(token_name);
        <b>let</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a> = 10u64.pow(decimal);
        self.supported_tokens.insert(
            type_name,
            <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a>{
                id: <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>,
                <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>,
                <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>,
                native_token
            },
        );
        self.id_token_type_map.insert(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>, type_name);
        // Freeze upgrade cap to prevent changes to the coin
        transfer::public_freeze_object(uc);
        emit(<a href="../bridge/treasury.md#bridge_treasury_NewTokenEvent">NewTokenEvent</a>{
            <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>,
            type_name,
            native_token,
            <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>,
            <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>
        })
    } <b>else</b> {
        // Not implemented <b>for</b> V1
    }
}
</code></pre>



</details>

<a name="bridge_treasury_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_create">create</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_create">create</a>(ctx: &<b>mut</b> TxContext): <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a> {
    <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a> {
        external_coin_admin_address: vec_map::empty(),
        external_coin_target_address: vec_map::empty(),
        external_coin_witness_address: vec_map::empty(),
        treasuries: object_bag::new(ctx),
        supported_tokens: vec_map::empty(),
        id_token_type_map: vec_map::empty(),
        waiting_room: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="bridge_treasury_is_external_coin_admin"></a>

## Function `is_external_coin_admin`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_is_external_coin_admin">is_external_coin_admin</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_is_external_coin_admin">is_external_coin_admin</a>(
    self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    <b>address</b>: String,
): bool {
    <b>let</b> admins = self.external_coin_admin_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> admins = admins.destroy_some();
    admins.contains(&<b>address</b>)
}
</code></pre>



</details>

<a name="bridge_treasury_is_external_coin_witness"></a>

## Function `is_external_coin_witness`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_is_external_coin_witness">is_external_coin_witness</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, addr: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_is_external_coin_witness">is_external_coin_witness</a>(
  self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
  coin_type_name: String,
  addr: vector&lt;u8&gt;,
): bool {
  <b>let</b> admins = self.external_coin_witness_address.try_get(&coin_type_name);
  <b>if</b> (admins.is_none()) {
      <b>return</b> <b>false</b>
  };
  <b>let</b> admins = admins.destroy_some();
  admins.contains(&addr)
}
</code></pre>



</details>

<a name="bridge_treasury_external_coin_admin_count"></a>

## Function `external_coin_admin_count`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_external_coin_admin_count">external_coin_admin_count</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_external_coin_admin_count">external_coin_admin_count</a>(
    self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
): u64 {
    <b>let</b> admins = self.external_coin_admin_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        <b>return</b> 0
    };
    <b>let</b> admins = admins.destroy_some();
    <b>return</b> admins.size()
}
</code></pre>



</details>

<a name="bridge_treasury_add_external_coin_admin"></a>

## Function `add_external_coin_admin`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_external_coin_admin">add_external_coin_admin</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_external_coin_admin">add_external_coin_admin</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    <b>address</b>: String,
) {
    <b>let</b> admins = self.external_coin_admin_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        self.external_coin_admin_address.insert(coin_type_name, vec_set::empty());
    };
    <b>let</b> admins = self.external_coin_admin_address.get_mut(&coin_type_name);
    <b>if</b> (!admins.contains(&<b>address</b>)) {
        admins.insert(<b>address</b>);
    };
    emit(<a href="../bridge/treasury.md#bridge_treasury_AddExternalCoinAdminEvent">AddExternalCoinAdminEvent</a>{
        coin_type_name,
        <b>address</b>
    })
}
</code></pre>



</details>

<a name="bridge_treasury_add_external_coin_target"></a>

## Function `add_external_coin_target`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_external_coin_target">add_external_coin_target</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, addr: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_external_coin_target">add_external_coin_target</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    addr: String,
) {
    <b>let</b> admins = self.external_coin_target_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        self.external_coin_target_address.insert(coin_type_name, vec_set::empty());
    };
    <b>let</b> admins = self.external_coin_target_address.get_mut(&coin_type_name);
    <b>if</b> (!admins.contains(&addr)) {
        admins.insert(addr);
    };
    emit(<a href="../bridge/treasury.md#bridge_treasury_AddExternalCoinTargetEvent">AddExternalCoinTargetEvent</a>{
        coin_type_name,
        <b>address</b>: addr
    })
}
</code></pre>



</details>

<a name="bridge_treasury_add_external_coin_witness"></a>

## Function `add_external_coin_witness`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_external_coin_witness">add_external_coin_witness</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, addr: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_external_coin_witness">add_external_coin_witness</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    addr: vector&lt;u8&gt;,
) {
    <b>let</b> admins = self.external_coin_witness_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        self.external_coin_witness_address.insert(coin_type_name, vec_set::empty());
    };
    <b>let</b> admins = self.external_coin_witness_address.get_mut(&coin_type_name);
    <b>if</b> (!admins.contains(&addr)) {
        admins.insert(addr);
    };
    emit(<a href="../bridge/treasury.md#bridge_treasury_AddExternalCoinWitnessEvent">AddExternalCoinWitnessEvent</a>{
        coin_type_name,
        <b>address</b>: addr
    })
}
</code></pre>



</details>

<a name="bridge_treasury_remove_external_coin_witness"></a>

## Function `remove_external_coin_witness`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_remove_external_coin_witness">remove_external_coin_witness</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, addr: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_remove_external_coin_witness">remove_external_coin_witness</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    addr: vector&lt;u8&gt;,
 ) {
    <b>let</b> admins = self.external_coin_witness_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        <b>return</b>
    };
    <b>let</b> admins = self.external_coin_witness_address.get_mut(&coin_type_name);
    <b>if</b> (admins.contains(&addr)) {
        admins.remove(&addr);
        <b>if</b> (admins.size() == 0) {
            self.external_coin_witness_address.remove(&coin_type_name);
        };
        emit(<a href="../bridge/treasury.md#bridge_treasury_RemoveExternalCoinWitnessEvent">RemoveExternalCoinWitnessEvent</a>{
            coin_type_name,
            <b>address</b>: addr
        })
    }
}
</code></pre>



</details>

<a name="bridge_treasury_verify_bitcoin_signatures"></a>

## Function `verify_bitcoin_signatures`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_verify_bitcoin_signatures">verify_bitcoin_signatures</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, source_chain: u8, source_address: vector&lt;u8&gt;, target_address: vector&lt;u8&gt;, amount: u64, tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, signatures: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_verify_bitcoin_signatures">verify_bitcoin_signatures</a>&lt;T&gt;(
   self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
   source_chain: u8,
   source_address: vector&lt;u8&gt;,
   target_address: vector&lt;u8&gt;,
   amount: u64,
   tx_hash: ascii::String,
   signatures: vector&lt;u8&gt;,
):bool{
   <b>let</b> coin_type = type_name::into_string(type_name::get&lt;T&gt;());
   <b>let</b> bitcoin_message=<a href="../bridge/message.md#bridge_message_create_bitcoin_message">message::create_bitcoin_message</a>(
       source_chain,
       source_address,
       target_address,
       amount,
       *ascii::as_bytes(&tx_hash),
       *ascii::as_bytes(&coin_type)
   );
   <b>let</b> msg=hash::keccak256(&bitcoin_message.serialize_bitcoin_message());
   <b>let</b> pubkey =
       ecdsa_k1::decompress_pubkey(&ecdsa_k1::secp256k1_ecrecover(&signatures, &msg, 0));
   <b>let</b> addr=<a href="../bridge/crypto.md#bridge_crypto_ecdsa_pub_key_to_eth_address">crypto::ecdsa_pub_key_to_eth_address</a>(&pubkey);
   self.<a href="../bridge/treasury.md#bridge_treasury_is_external_coin_witness">is_external_coin_witness</a>(coin_type, addr)
}
</code></pre>



</details>

<a name="bridge_treasury_remove_external_coin_admin"></a>

## Function `remove_external_coin_admin`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_remove_external_coin_admin">remove_external_coin_admin</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_remove_external_coin_admin">remove_external_coin_admin</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    <b>address</b>: String,
) {
    <b>let</b> admins = self.external_coin_admin_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        <b>return</b>
    };
    <b>let</b> admins = self.external_coin_admin_address.get_mut(&coin_type_name);
    <b>if</b> (admins.contains(&<b>address</b>)) {
        admins.remove(&<b>address</b>);
        <b>if</b> (admins.size() == 0) {
            self.external_coin_admin_address.remove(&coin_type_name);
        };
        emit(<a href="../bridge/treasury.md#bridge_treasury_RemoveExternalCoinAdminEvent">RemoveExternalCoinAdminEvent</a>{
            coin_type_name,
            <b>address</b>
        })
    }
}
</code></pre>



</details>

<a name="bridge_treasury_remove_external_coin_target"></a>

## Function `remove_external_coin_target`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_remove_external_coin_target">remove_external_coin_target</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin_type_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, addr: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_remove_external_coin_target">remove_external_coin_target</a>(
   self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
   coin_type_name: String,
   addr: String,
) {
   <b>let</b> admins = self.external_coin_target_address.try_get(&coin_type_name);
   <b>if</b> (admins.is_none()) {
       <b>return</b>
   };
   <b>let</b> admins = self.external_coin_target_address.get_mut(&coin_type_name);
   <b>if</b> (admins.contains(&addr)) {
       admins.remove(&addr);
       <b>if</b> (admins.size() == 0) {
           self.external_coin_target_address.remove(&coin_type_name);
       };
       emit(<a href="../bridge/treasury.md#bridge_treasury_RemoveExternalCoinTargetEvent">RemoveExternalCoinTargetEvent</a>{
           coin_type_name,
           <b>address</b>: addr
       })
   }
}
</code></pre>



</details>

<a name="bridge_treasury_burn"></a>

## Function `burn`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_burn">burn</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, token: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_burn">burn</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, token: Coin&lt;T&gt;) {
    <b>let</b> <a href="../bridge/treasury.md#bridge_treasury">treasury</a> = &<b>mut</b> self.treasuries[type_name::get&lt;T&gt;()];
    coin::burn(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, token);
}
</code></pre>



</details>

<a name="bridge_treasury_mint"></a>

## Function `mint`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_mint">mint</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_mint">mint</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> <a href="../bridge/treasury.md#bridge_treasury">treasury</a> = &<b>mut</b> self.treasuries[type_name::get&lt;T&gt;()];
    coin::mint(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, amount, ctx)
}
</code></pre>



</details>

<a name="bridge_treasury_calculate_amount_in_usd"></a>

## Function `calculate_amount_in_usd`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_calculate_amount_in_usd">calculate_amount_in_usd</a>&lt;T&gt;(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package)  <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_calculate_amount_in_usd">calculate_amount_in_usd</a>&lt;T&gt;(
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    amount: u64): u64 {
    <b>let</b> metadata = <a href="../bridge/treasury.md#bridge_treasury">treasury</a>.<a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    ((amount <b>as</b> u128) * (metadata.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> <b>as</b> u128) / (metadata.<a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a> <b>as</b> u128)) <b>as</b> u64
}
</code></pre>



</details>

<a name="bridge_treasury_get_amount_in_usd_by_token_id"></a>

## Function `get_amount_in_usd_by_token_id`

根据 token_id 和数量 amount（该代币最小单位）计算 USD 价值，返回值使用 8 位小数（与 limiter 的 USD_VALUE_MULTIPLIER 一致）。


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_amount_in_usd_by_token_id">get_amount_in_usd_by_token_id</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_amount_in_usd_by_token_id">get_amount_in_usd_by_token_id</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64, amount: u64): u64 {
    <b>let</b> type_name_opt = self.id_token_type_map.try_get(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    <b>assert</b>!(type_name_opt.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> type_name = type_name_opt.destroy_some();
    <b>let</b> metadata_opt = self.supported_tokens.try_get(&type_name);
    <b>assert</b>!(metadata_opt.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> metadata = metadata_opt.destroy_some();
    ((amount <b>as</b> u128) * (metadata.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> <b>as</b> u128) / (metadata.<a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a> <b>as</b> u128)) <b>as</b> u64
}
</code></pre>



</details>

<a name="bridge_treasury_get_token_amount_by_usd"></a>

## Function `get_token_amount_by_usd`

根据 token_id 和 USD 价值（8 位小数）换算为该代币最小单位的数量。


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_token_amount_by_usd">get_token_amount_by_usd</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64, usd_amount_8dp: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_token_amount_by_usd">get_token_amount_by_usd</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64, usd_amount_8dp: u64): u64 {
    <b>let</b> type_name_opt = self.id_token_type_map.try_get(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    <b>assert</b>!(type_name_opt.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> type_name = type_name_opt.destroy_some();
    <b>let</b> metadata_opt = self.supported_tokens.try_get(&type_name);
    <b>assert</b>!(metadata_opt.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> metadata = metadata_opt.destroy_some();
    ((usd_amount_8dp <b>as</b> u128) * (metadata.<a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a> <b>as</b> u128) / (metadata.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> <b>as</b> u128)) <b>as</b> u64
}
</code></pre>



</details>

<a name="bridge_treasury_update_asset_notional_price"></a>

## Function `update_asset_notional_price`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_update_asset_notional_price">update_asset_notional_price</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64, new_usd_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_update_asset_notional_price">update_asset_notional_price</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u64,
    new_usd_price: u64,
) {
    <b>let</b> type_name = self.id_token_type_map.try_get(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    <b>assert</b>!(type_name.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>assert</b>!(new_usd_price &gt; 0, <a href="../bridge/treasury.md#bridge_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>);
    <b>let</b> type_name = type_name.destroy_some();
    <b>let</b> metadata = self.supported_tokens.get_mut(&type_name);
    metadata.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> = new_usd_price;
    emit(<a href="../bridge/treasury.md#bridge_treasury_UpdateTokenPriceEvent">UpdateTokenPriceEvent</a> {
        <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>,
        new_price: new_usd_price,
    })
}
</code></pre>



</details>

<a name="bridge_treasury_get_token_metadata"></a>

## Function `get_token_metadata`



<pre><code><b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">bridge::treasury::BridgeTokenMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> {
    <b>let</b> coin_type = type_name::get&lt;T&gt;();
    <b>let</b> metadata = self.supported_tokens.try_get(&coin_type);
    <b>assert</b>!(metadata.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    metadata.destroy_some()
}
</code></pre>



</details>

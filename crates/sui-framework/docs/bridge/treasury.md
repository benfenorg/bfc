---
title: Module `0xb::treasury`
---



-  [Struct `BridgeTreasury`](#0xb_treasury_BridgeTreasury)
-  [Struct `BridgeTokenMetadata`](#0xb_treasury_BridgeTokenMetadata)
-  [Struct `ForeignTokenRegistration`](#0xb_treasury_ForeignTokenRegistration)
-  [Struct `UpdateTokenPriceEvent`](#0xb_treasury_UpdateTokenPriceEvent)
-  [Struct `NewTokenEvent`](#0xb_treasury_NewTokenEvent)
-  [Struct `TokenRegistrationEvent`](#0xb_treasury_TokenRegistrationEvent)
-  [Struct `AddExternalCoinAdminEvent`](#0xb_treasury_AddExternalCoinAdminEvent)
-  [Struct `RemoveExternalCoinAdminEvent`](#0xb_treasury_RemoveExternalCoinAdminEvent)
-  [Struct `AddExternalCoinWitnessEvent`](#0xb_treasury_AddExternalCoinWitnessEvent)
-  [Struct `RemoveExternalCoinWitnessEvent`](#0xb_treasury_RemoveExternalCoinWitnessEvent)
-  [Struct `AddExternalCoinTargetEvent`](#0xb_treasury_AddExternalCoinTargetEvent)
-  [Struct `RemoveExternalCoinTargetEvent`](#0xb_treasury_RemoveExternalCoinTargetEvent)
-  [Constants](#@Constants_0)
-  [Function `token_id`](#0xb_treasury_token_id)
-  [Function `decimal_multiplier`](#0xb_treasury_decimal_multiplier)
-  [Function `notional_value`](#0xb_treasury_notional_value)
-  [Function `register_foreign_token`](#0xb_treasury_register_foreign_token)
-  [Function `add_new_token`](#0xb_treasury_add_new_token)
-  [Function `create`](#0xb_treasury_create)
-  [Function `is_external_coin_admin`](#0xb_treasury_is_external_coin_admin)
-  [Function `is_external_coin_witness`](#0xb_treasury_is_external_coin_witness)
-  [Function `external_coin_admin_count`](#0xb_treasury_external_coin_admin_count)
-  [Function `add_external_coin_admin`](#0xb_treasury_add_external_coin_admin)
-  [Function `add_external_coin_target`](#0xb_treasury_add_external_coin_target)
-  [Function `add_external_coin_witness`](#0xb_treasury_add_external_coin_witness)
-  [Function `remove_external_coin_witness`](#0xb_treasury_remove_external_coin_witness)
-  [Function `verify_bitcoin_signatures`](#0xb_treasury_verify_bitcoin_signatures)
-  [Function `remove_external_coin_admin`](#0xb_treasury_remove_external_coin_admin)
-  [Function `remove_external_coin_target`](#0xb_treasury_remove_external_coin_target)
-  [Function `burn`](#0xb_treasury_burn)
-  [Function `mint`](#0xb_treasury_mint)
-  [Function `update_asset_notional_price`](#0xb_treasury_update_asset_notional_price)
-  [Function `get_token_metadata`](#0xb_treasury_get_token_metadata)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../move-stdlib/u64.md#0x1_u64">0x1::u64</a>;
<b>use</b> <a href="../sui-framework/address.md#0x2_address">0x2::address</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/ecdsa_k1.md#0x2_ecdsa_k1">0x2::ecdsa_k1</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/hash.md#0x2_hash">0x2::hash</a>;
<b>use</b> <a href="../sui-framework/hex.md#0x2_hex">0x2::hex</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/object_bag.md#0x2_object_bag">0x2::object_bag</a>;
<b>use</b> <a href="../sui-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
<b>use</b> <a href="crypto.md#0xb_crypto">0xb::crypto</a>;
<b>use</b> <a href="message.md#0xb_message">0xb::message</a>;
</code></pre>



<a name="0xb_treasury_BridgeTreasury"></a>

## Struct `BridgeTreasury`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>external_coin_admin_address: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>external_coin_target_address: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>external_coin_witness_address: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>treasuries: <a href="../sui-framework/object_bag.md#0x2_object_bag_ObjectBag">object_bag::ObjectBag</a></code>
</dt>
<dd>

</dd>
<dt>
<code>supported_tokens: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/type_name.md#0x1_type_name_TypeName">type_name::TypeName</a>, treasury::BridgeTokenMetadata&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>id_token_type_map: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/type_name.md#0x1_type_name_TypeName">type_name::TypeName</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>waiting_room: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_BridgeTokenMetadata"></a>

## Struct `BridgeTokenMetadata`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>decimal_multiplier: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>notional_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
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

<a name="0xb_treasury_ForeignTokenRegistration"></a>

## Struct `ForeignTokenRegistration`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>: <a href="../move-stdlib/type_name.md#0x1_type_name_TypeName">type_name::TypeName</a></code>
</dt>
<dd>

</dd>
<dt>
<code>uc: <a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a></code>
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

<a name="0xb_treasury_UpdateTokenPriceEvent"></a>

## Struct `UpdateTokenPriceEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_UpdateTokenPriceEvent">UpdateTokenPriceEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>new_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_NewTokenEvent"></a>

## Struct `NewTokenEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_NewTokenEvent">NewTokenEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>: <a href="../move-stdlib/type_name.md#0x1_type_name_TypeName">type_name::TypeName</a></code>
</dt>
<dd>

</dd>
<dt>
<code>native_token: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>decimal_multiplier: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>notional_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_TokenRegistrationEvent"></a>

## Struct `TokenRegistrationEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_TokenRegistrationEvent">TokenRegistrationEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>: <a href="../move-stdlib/type_name.md#0x1_type_name_TypeName">type_name::TypeName</a></code>
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

<a name="0xb_treasury_AddExternalCoinAdminEvent"></a>

## Struct `AddExternalCoinAdminEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_AddExternalCoinAdminEvent">AddExternalCoinAdminEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_RemoveExternalCoinAdminEvent"></a>

## Struct `RemoveExternalCoinAdminEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_RemoveExternalCoinAdminEvent">RemoveExternalCoinAdminEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_AddExternalCoinWitnessEvent"></a>

## Struct `AddExternalCoinWitnessEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_AddExternalCoinWitnessEvent">AddExternalCoinWitnessEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><b>address</b>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_RemoveExternalCoinWitnessEvent"></a>

## Struct `RemoveExternalCoinWitnessEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_RemoveExternalCoinWitnessEvent">RemoveExternalCoinWitnessEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><b>address</b>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_AddExternalCoinTargetEvent"></a>

## Struct `AddExternalCoinTargetEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_AddExternalCoinTargetEvent">AddExternalCoinTargetEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_treasury_RemoveExternalCoinTargetEvent"></a>

## Struct `RemoveExternalCoinTargetEvent`



<pre><code><b>struct</b> <a href="treasury.md#0xb_treasury_RemoveExternalCoinTargetEvent">RemoveExternalCoinTargetEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xb_treasury_EInvalidNotionalValue"></a>



<pre><code><b>const</b> <a href="treasury.md#0xb_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0xb_treasury_EInvalidUpgradeCap"></a>



<pre><code><b>const</b> <a href="treasury.md#0xb_treasury_EInvalidUpgradeCap">EInvalidUpgradeCap</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0xb_treasury_ETokenSupplyNonZero"></a>



<pre><code><b>const</b> <a href="treasury.md#0xb_treasury_ETokenSupplyNonZero">ETokenSupplyNonZero</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0xb_treasury_EUnsupportedTokenType"></a>



<pre><code><b>const</b> <a href="treasury.md#0xb_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xb_treasury_token_id"></a>

## Function `token_id`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_token_id">token_id</a>&lt;T&gt;(self: &treasury::BridgeTreasury): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_token_id">token_id</a>&lt;T&gt;(self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> metadata = self.<a href="treasury.md#0xb_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.id
}
</code></pre>



</details>

<a name="0xb_treasury_decimal_multiplier"></a>

## Function `decimal_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_decimal_multiplier">decimal_multiplier</a>&lt;T&gt;(self: &treasury::BridgeTreasury): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_decimal_multiplier">decimal_multiplier</a>&lt;T&gt;(self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> metadata = self.<a href="treasury.md#0xb_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.decimal_multiplier
}
</code></pre>



</details>

<a name="0xb_treasury_notional_value"></a>

## Function `notional_value`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_notional_value">notional_value</a>&lt;T&gt;(self: &treasury::BridgeTreasury): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_notional_value">notional_value</a>&lt;T&gt;(self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> metadata = self.<a href="treasury.md#0xb_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.notional_value
}
</code></pre>



</details>

<a name="0xb_treasury_register_foreign_token"></a>

## Function `register_foreign_token`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_register_foreign_token">register_foreign_token</a>&lt;T&gt;(self: &<b>mut</b> treasury::BridgeTreasury, tc: <a href="../sui-framework/coin.md#0x2_coin_TreasuryCap">coin::TreasuryCap</a>&lt;T&gt;, uc: <a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>, metadata: &<a href="../sui-framework/coin.md#0x2_coin_CoinMetadata">coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_register_foreign_token">register_foreign_token</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    tc: TreasuryCap&lt;T&gt;,
    uc: UpgradeCap,
    metadata: &CoinMetadata&lt;T&gt;,
) {
    // Make sure TreasuryCap <b>has</b> not been minted before.
    <b>assert</b>!(<a href="../sui-framework/coin.md#0x2_coin_total_supply">coin::total_supply</a>(&tc) == 0, <a href="treasury.md#0xb_treasury_ETokenSupplyNonZero">ETokenSupplyNonZero</a>);
    <b>let</b> <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a> = <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;();
    <b>let</b> address_bytes = <a href="../sui-framework/hex.md#0x2_hex_decode">hex::decode</a>(<a href="../move-stdlib/ascii.md#0x1_ascii_into_bytes">ascii::into_bytes</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get_address">type_name::get_address</a>(&<a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>)));
    <b>let</b> coin_address = address::from_bytes(address_bytes);
    // Make sure upgrade cap is for the Coin <a href="../sui-framework/package.md#0x2_package">package</a>
    // FIXME: add test
    <b>assert</b>!(
        <a href="../sui-framework/object.md#0x2_object_id_to_address">object::id_to_address</a>(&<a href="../sui-framework/package.md#0x2_package_upgrade_package">package::upgrade_package</a>(&uc))
            == coin_address, <a href="treasury.md#0xb_treasury_EInvalidUpgradeCap">EInvalidUpgradeCap</a>
    );
    <b>let</b> registration = <a href="treasury.md#0xb_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a> {
        <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>,
        uc,
        decimal: <a href="../sui-framework/coin.md#0x2_coin_get_decimals">coin::get_decimals</a>(metadata),
    };
    self.waiting_room.add(<a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>), registration);
    self.treasuries.add(<a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>, tc);

    emit(<a href="treasury.md#0xb_treasury_TokenRegistrationEvent">TokenRegistrationEvent</a>{
        <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>,
        decimal: <a href="../sui-framework/coin.md#0x2_coin_get_decimals">coin::get_decimals</a>(metadata),
        native_token: <b>false</b>
    });
}
</code></pre>



</details>

<a name="0xb_treasury_add_new_token"></a>

## Function `add_new_token`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_new_token">add_new_token</a>(self: &<b>mut</b> treasury::BridgeTreasury, token_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, native_token: bool, notional_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_new_token">add_new_token</a>(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    token_name: String,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    native_token: bool,
    notional_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>if</b> (!native_token){
        <b>assert</b>!(notional_value &gt; 0, <a href="treasury.md#0xb_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>);
        <b>let</b> <a href="treasury.md#0xb_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a>{
            <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>,
            uc,
            decimal,
        } = self.waiting_room.remove&lt;String, <a href="treasury.md#0xb_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a>&gt;(token_name);
        <b>let</b> decimal_multiplier = 10u64.pow(decimal);
        self.supported_tokens.insert(
            <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>,
            <a href="treasury.md#0xb_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a>{
                id: token_id,
                decimal_multiplier,
                notional_value,
                native_token
            },
        );
        self.id_token_type_map.insert(token_id, <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>);

        // Freeze upgrade cap <b>to</b> prevent changes <b>to</b> the <a href="../sui-framework/coin.md#0x2_coin">coin</a>
        <a href="../sui-framework/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(uc);

        emit(<a href="treasury.md#0xb_treasury_NewTokenEvent">NewTokenEvent</a>{
            token_id,
            <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>,
            native_token,
            decimal_multiplier,
            notional_value
        })
    } <b>else</b> {
        // Not implemented for V1
    }
}
</code></pre>



</details>

<a name="0xb_treasury_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_create">create</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): treasury::BridgeTreasury
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_create">create</a>(ctx: &<b>mut</b> TxContext): <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a> {
    <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a> {
        external_coin_admin_address: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        external_coin_target_address: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        external_coin_witness_address: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        treasuries: <a href="../sui-framework/object_bag.md#0x2_object_bag_new">object_bag::new</a>(ctx),
        supported_tokens: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        id_token_type_map: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        waiting_room: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0xb_treasury_is_external_coin_admin"></a>

## Function `is_external_coin_admin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_is_external_coin_admin">is_external_coin_admin</a>(self: &treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_is_external_coin_admin">is_external_coin_admin</a>(
    self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
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

<a name="0xb_treasury_is_external_coin_witness"></a>

## Function `is_external_coin_witness`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_is_external_coin_witness">is_external_coin_witness</a>(self: &treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, addr: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_is_external_coin_witness">is_external_coin_witness</a>(
  self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
  coin_type_name: String,
  addr: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
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

<a name="0xb_treasury_external_coin_admin_count"></a>

## Function `external_coin_admin_count`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_external_coin_admin_count">external_coin_admin_count</a>(self: &treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_external_coin_admin_count">external_coin_admin_count</a>(
    self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> admins = self.external_coin_admin_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        <b>return</b> 0
    };
    <b>let</b> admins = admins.destroy_some();
    <b>return</b> admins.size()
}
</code></pre>



</details>

<a name="0xb_treasury_add_external_coin_admin"></a>

## Function `add_external_coin_admin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_external_coin_admin">add_external_coin_admin</a>(self: &<b>mut</b> treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_external_coin_admin">add_external_coin_admin</a>(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    <b>address</b>: String,
) {
    <b>let</b> admins = self.external_coin_admin_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        self.external_coin_admin_address.insert(coin_type_name, <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>());
    };
    <b>let</b> admins = self.external_coin_admin_address.get_mut(&coin_type_name);
    <b>if</b> (!admins.contains(&<b>address</b>)) {
        admins.insert(<b>address</b>);
    };
    emit(<a href="treasury.md#0xb_treasury_AddExternalCoinAdminEvent">AddExternalCoinAdminEvent</a>{
        coin_type_name,
        <b>address</b>
    })
}
</code></pre>



</details>

<a name="0xb_treasury_add_external_coin_target"></a>

## Function `add_external_coin_target`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_external_coin_target">add_external_coin_target</a>(self: &<b>mut</b> treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, addr: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_external_coin_target">add_external_coin_target</a>(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    addr: String,
) {
    <b>let</b> admins = self.external_coin_target_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        self.external_coin_target_address.insert(coin_type_name, <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>());
    };
    <b>let</b> admins = self.external_coin_target_address.get_mut(&coin_type_name);
    <b>if</b> (!admins.contains(&addr)) {
        admins.insert(addr);
    };
    emit(<a href="treasury.md#0xb_treasury_AddExternalCoinTargetEvent">AddExternalCoinTargetEvent</a>{
        coin_type_name,
        <b>address</b>: addr
    })

}
</code></pre>



</details>

<a name="0xb_treasury_add_external_coin_witness"></a>

## Function `add_external_coin_witness`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_external_coin_witness">add_external_coin_witness</a>(self: &<b>mut</b> treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, addr: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_add_external_coin_witness">add_external_coin_witness</a>(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    addr: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    <b>let</b> admins = self.external_coin_witness_address.try_get(&coin_type_name);
    <b>if</b> (admins.is_none()) {
        self.external_coin_witness_address.insert(coin_type_name, <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>());
    };
    <b>let</b> admins = self.external_coin_witness_address.get_mut(&coin_type_name);
    <b>if</b> (!admins.contains(&addr)) {
        admins.insert(addr);
    };
    emit(<a href="treasury.md#0xb_treasury_AddExternalCoinWitnessEvent">AddExternalCoinWitnessEvent</a>{
        coin_type_name,
        <b>address</b>: addr
    })
}
</code></pre>



</details>

<a name="0xb_treasury_remove_external_coin_witness"></a>

## Function `remove_external_coin_witness`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_remove_external_coin_witness">remove_external_coin_witness</a>(self: &<b>mut</b> treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, addr: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_remove_external_coin_witness">remove_external_coin_witness</a>(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    coin_type_name: String,
    addr: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
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
        emit(<a href="treasury.md#0xb_treasury_RemoveExternalCoinWitnessEvent">RemoveExternalCoinWitnessEvent</a>{
            coin_type_name,
            <b>address</b>: addr
        })
    }
}
</code></pre>



</details>

<a name="0xb_treasury_verify_bitcoin_signatures"></a>

## Function `verify_bitcoin_signatures`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_verify_bitcoin_signatures">verify_bitcoin_signatures</a>&lt;T&gt;(self: &treasury::BridgeTreasury, source_chain: u8, source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xb_treasury_verify_bitcoin_signatures">verify_bitcoin_signatures</a>&lt;T&gt;(
   self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
   source_chain: u8,
   source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
   target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
   amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
   tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>,
   signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
):bool{
   <b>let</b> coin_type = <a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;());

   <b>let</b> bitcoin_message=<a href="message.md#0xb_message_create_bitcoin_message">message::create_bitcoin_message</a>(
       source_chain,
       source_address,
       target_address,
       amount,
       *<a href="../move-stdlib/ascii.md#0x1_ascii_as_bytes">ascii::as_bytes</a>(&tx_hash),
       *<a href="../move-stdlib/ascii.md#0x1_ascii_as_bytes">ascii::as_bytes</a>(&coin_type)
   );
   <b>let</b> msg=<a href="../sui-framework/hash.md#0x2_hash_keccak256">hash::keccak256</a>(&bitcoin_message.serialize_bitcoin_message());
   <b>let</b> pubkey =
       <a href="../sui-framework/ecdsa_k1.md#0x2_ecdsa_k1_decompress_pubkey">ecdsa_k1::decompress_pubkey</a>(&<a href="../sui-framework/ecdsa_k1.md#0x2_ecdsa_k1_secp256k1_ecrecover">ecdsa_k1::secp256k1_ecrecover</a>(&signatures, &msg, 0));
   <b>let</b> addr=<a href="crypto.md#0xb_crypto_ecdsa_pub_key_to_eth_address">crypto::ecdsa_pub_key_to_eth_address</a>(&pubkey);
   self.<a href="treasury.md#0xb_treasury_is_external_coin_witness">is_external_coin_witness</a>(coin_type, addr)
}
</code></pre>



</details>

<a name="0xb_treasury_remove_external_coin_admin"></a>

## Function `remove_external_coin_admin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_remove_external_coin_admin">remove_external_coin_admin</a>(self: &<b>mut</b> treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_remove_external_coin_admin">remove_external_coin_admin</a>(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
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
        emit(<a href="treasury.md#0xb_treasury_RemoveExternalCoinAdminEvent">RemoveExternalCoinAdminEvent</a>{
            coin_type_name,
            <b>address</b>
        })
    }
}
</code></pre>



</details>

<a name="0xb_treasury_remove_external_coin_target"></a>

## Function `remove_external_coin_target`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_remove_external_coin_target">remove_external_coin_target</a>(self: &<b>mut</b> treasury::BridgeTreasury, coin_type_name: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, addr: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_remove_external_coin_target">remove_external_coin_target</a>(
   self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
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
       emit(<a href="treasury.md#0xb_treasury_RemoveExternalCoinTargetEvent">RemoveExternalCoinTargetEvent</a>{
           coin_type_name,
           <b>address</b>: addr
       })
   }
}
</code></pre>



</details>

<a name="0xb_treasury_burn"></a>

## Function `burn`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_burn">burn</a>&lt;T&gt;(self: &<b>mut</b> treasury::BridgeTreasury, token: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_burn">burn</a>&lt;T&gt;(self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>, token: Coin&lt;T&gt;) {
    <b>let</b> <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a> = &<b>mut</b> self.treasuries[<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;()];
    <a href="../sui-framework/coin.md#0x2_coin_burn">coin::burn</a>(<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, token);
}
</code></pre>



</details>

<a name="0xb_treasury_mint"></a>

## Function `mint`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_mint">mint</a>&lt;T&gt;(self: &<b>mut</b> treasury::BridgeTreasury, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_mint">mint</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a> = &<b>mut</b> self.treasuries[<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;()];
    <a href="../sui-framework/coin.md#0x2_coin_mint">coin::mint</a>(<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, amount, ctx)
}
</code></pre>



</details>

<a name="0xb_treasury_update_asset_notional_price"></a>

## Function `update_asset_notional_price`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xb_treasury_update_asset_notional_price">update_asset_notional_price</a>(self: &<b>mut</b> treasury::BridgeTreasury, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, new_usd_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="treasury.md#0xb_treasury_update_asset_notional_price">update_asset_notional_price</a>(
    self: &<b>mut</b> <a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    new_usd_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <b>let</b> <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a> = self.id_token_type_map.try_get(&token_id);
    <b>assert</b>!(<a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>.is_some(), <a href="treasury.md#0xb_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>assert</b>!(new_usd_price &gt; 0, <a href="treasury.md#0xb_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>);
    <b>let</b> <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a> = <a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>.destroy_some();
    <b>let</b> metadata = self.supported_tokens.get_mut(&<a href="../move-stdlib/type_name.md#0x1_type_name">type_name</a>);
    metadata.notional_value = new_usd_price;

    emit(<a href="treasury.md#0xb_treasury_UpdateTokenPriceEvent">UpdateTokenPriceEvent</a> {
        token_id,
        new_price: new_usd_price,
    })
}
</code></pre>



</details>

<a name="0xb_treasury_get_token_metadata"></a>

## Function `get_token_metadata`



<pre><code><b>fun</b> <a href="treasury.md#0xb_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;(self: &treasury::BridgeTreasury): treasury::BridgeTokenMetadata
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xb_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;(self: &<a href="treasury.md#0xb_treasury_BridgeTreasury">BridgeTreasury</a>): <a href="treasury.md#0xb_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> {
    <b>let</b> coin_type = <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;();
    <b>let</b> metadata = self.supported_tokens.try_get(&coin_type);
    <b>assert</b>!(metadata.is_some(), <a href="treasury.md#0xb_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    metadata.destroy_some()
}
</code></pre>



</details>

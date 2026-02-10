---
title: Module `bridge::limiter`
---



-  [Struct `ExternalTransferRecordsKey`](#bridge_limiter_ExternalTransferRecordsKey)
-  [Struct `ExternalTransfer24hLimitsKey`](#bridge_limiter_ExternalTransfer24hLimitsKey)
-  [Struct `TransferLimiter`](#bridge_limiter_TransferLimiter)
-  [Struct `ExternalLimiter`](#bridge_limiter_ExternalLimiter)
-  [Struct `TransferRecord`](#bridge_limiter_TransferRecord)
-  [Struct `UpdateRouteLimitEvent`](#bridge_limiter_UpdateRouteLimitEvent)
-  [Constants](#@Constants_0)
-  [Function `get_route_limit`](#bridge_limiter_get_route_limit)
-  [Function `get_mint_busd_max_limit`](#bridge_limiter_get_mint_busd_max_limit)
-  [Function `set_mint_busd_max_limit`](#bridge_limiter_set_mint_busd_max_limit)
-  [Function `new`](#bridge_limiter_new)
-  [Function `new_external_limits`](#bridge_limiter_new_external_limits)
-  [Function `initial_external_limits`](#bridge_limiter_initial_external_limits)
-  [Function `initial_external_24h_limits`](#bridge_limiter_initial_external_24h_limits)
-  [Function `get_external_limiter`](#bridge_limiter_get_external_limiter)
-  [Function `update_external_out_limit`](#bridge_limiter_update_external_out_limit)
-  [Function `add_external_out_limit`](#bridge_limiter_add_external_out_limit)
-  [Function `get_external_out_limit`](#bridge_limiter_get_external_out_limit)
-  [Function `update_external_24h_limit`](#bridge_limiter_update_external_24h_limit)
-  [Function `check_and_record_external_24h_transfer`](#bridge_limiter_check_and_record_external_24h_transfer)
-  [Function `get_external_available_transfer_amount`](#bridge_limiter_get_external_available_transfer_amount)
-  [Function `get_available_claim_amount`](#bridge_limiter_get_available_claim_amount)
-  [Function `check_and_record_sending_transfer`](#bridge_limiter_check_and_record_sending_transfer)
-  [Function `update_route_limit`](#bridge_limiter_update_route_limit)
-  [Function `current_hour_since_epoch`](#bridge_limiter_current_hour_since_epoch)
-  [Function `adjust_transfer_records`](#bridge_limiter_adjust_transfer_records)
-  [Function `initial_transfer_limits`](#bridge_limiter_initial_transfer_limits)
-  [Function `update_transfer_limits`](#bridge_limiter_update_transfer_limits)


<pre><code><b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../bridge/crypto.md#bridge_crypto">bridge::crypto</a>;
<b>use</b> <a href="../bridge/message.md#bridge_message">bridge::message</a>;
<b>use</b> <a href="../bridge/message_types.md#bridge_message_types">bridge::message_types</a>;
<b>use</b> <a href="../bridge/treasury.md#bridge_treasury">bridge::treasury</a>;
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
<b>use</b> <a href="../sui/clock.md#sui_clock">sui::clock</a>;
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
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="bridge_limiter_ExternalTransferRecordsKey"></a>

## Struct `ExternalTransferRecordsKey`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="bridge_limiter_ExternalTransfer24hLimitsKey"></a>

## Struct `ExternalTransfer24hLimitsKey`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="bridge_limiter_TransferLimiter"></a>

## Struct `TransferLimiter`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>transfer_limits: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>transfer_records: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">bridge::limiter::TransferRecord</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_mint_busd_limit: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_limiter_ExternalLimiter"></a>

## Struct `ExternalLimiter`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>transfer_out_limits: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>external: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_limiter_TransferRecord"></a>

## Struct `TransferRecord`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">TransferRecord</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>hour_head: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>hour_tail: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>per_hour_amounts: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>total_amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_limiter_UpdateRouteLimitEvent"></a>

## Struct `UpdateRouteLimitEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter.md#bridge_limiter_UpdateRouteLimitEvent">UpdateRouteLimitEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sending_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>receiving_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>new_limit: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_limiter_ELimitNotFoundForRoute"></a>



<pre><code><b>const</b> <a href="../bridge/limiter.md#bridge_limiter_ELimitNotFoundForRoute">ELimitNotFoundForRoute</a>: u64 = 0;
</code></pre>



<a name="bridge_limiter_EExternalLimitKeyExist"></a>



<pre><code><b>const</b> <a href="../bridge/limiter.md#bridge_limiter_EExternalLimitKeyExist">EExternalLimitKeyExist</a>: u64 = 1;
</code></pre>



<a name="bridge_limiter_EExternalLimitNotFoundForRoute"></a>



<pre><code><b>const</b> <a href="../bridge/limiter.md#bridge_limiter_EExternalLimitNotFoundForRoute">EExternalLimitNotFoundForRoute</a>: u64 = 3;
</code></pre>



<a name="bridge_limiter_MAX_TRANSFER_LIMIT"></a>



<pre><code><b>const</b> <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>: u64 = 18446744073709551615;
</code></pre>



<a name="bridge_limiter_USD_VALUE_MULTIPLIER"></a>



<pre><code><b>const</b> <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>: u64 = 100000000;
</code></pre>



<a name="bridge_limiter_DEFAULT_MAX_MINT_BUSD_LIMIT"></a>



<pre><code><b>const</b> <a href="../bridge/limiter.md#bridge_limiter_DEFAULT_MAX_MINT_BUSD_LIMIT">DEFAULT_MAX_MINT_BUSD_LIMIT</a>: u64 = 500000000000000;
</code></pre>



<a name="bridge_limiter_EXTERNAL_LIMITS_KEY"></a>



<pre><code><b>const</b> <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>: vector&lt;u8&gt; = vector[98, 114, 105, 100, 103, 101, 95, 101, 120, 116, 101, 114, 110, 97, 108, 95, 108, 105, 109, 105, 116, 115];
</code></pre>



<a name="bridge_limiter_get_route_limit"></a>

## Function `get_route_limit`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_route_limit">get_route_limit</a>(self: &<a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>, route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_route_limit">get_route_limit</a>(self: &<a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a>, route: &BridgeRoute): u64 {
    self.transfer_limits[route]
}
</code></pre>



</details>

<a name="bridge_limiter_get_mint_busd_max_limit"></a>

## Function `get_mint_busd_max_limit`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_mint_busd_max_limit">get_mint_busd_max_limit</a>(self: &<a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_mint_busd_max_limit">get_mint_busd_max_limit</a>(self: &<a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a>): u64 {
    self.max_mint_busd_limit
}
</code></pre>



</details>

<a name="bridge_limiter_set_mint_busd_max_limit"></a>

## Function `set_mint_busd_max_limit`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_set_mint_busd_max_limit">set_mint_busd_max_limit</a>(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>, new_limit: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_set_mint_busd_max_limit">set_mint_busd_max_limit</a>(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a>, new_limit: u64) {
    self.max_mint_busd_limit = new_limit;
}
</code></pre>



</details>

<a name="bridge_limiter_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_new">new</a>(): <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_new">new</a>(): <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a> {
    // hardcoded limit <b>for</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a> genesis
    <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a> {
        transfer_limits: <a href="../bridge/limiter.md#bridge_limiter_initial_transfer_limits">initial_transfer_limits</a>(),
        transfer_records: vec_map::empty(),
        max_mint_busd_limit: <a href="../bridge/limiter.md#bridge_limiter_DEFAULT_MAX_MINT_BUSD_LIMIT">DEFAULT_MAX_MINT_BUSD_LIMIT</a>,
    }
}
</code></pre>



</details>

<a name="bridge_limiter_new_external_limits"></a>

## Function `new_external_limits`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_new_external_limits">new_external_limits</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_new_external_limits">new_external_limits</a>(parent_id: &<b>mut</b> UID, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(
        !dynamic_field::exists_(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>),
        <a href="../bridge/limiter.md#bridge_limiter_EExternalLimitKeyExist">EExternalLimitKeyExist</a>
    );
    dynamic_field::add(
        parent_id,
        <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>,
        <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a> {
            transfer_out_limits: vec_map::empty(),
            external: bag::new(ctx),
        },
    );
    <a href="../bridge/limiter.md#bridge_limiter_initial_external_limits">initial_external_limits</a>(parent_id);
}
</code></pre>



</details>

<a name="bridge_limiter_initial_external_limits"></a>

## Function `initial_external_limits`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_initial_external_limits">initial_external_limits</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_initial_external_limits">initial_external_limits</a>(
    parent_id: &<b>mut</b> UID,
) {
    // <b>assert</b>!(dynamic_field::exists_(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>), 9999);
    <b>let</b> external_limiter = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a>&gt;(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>);
    // Initialize the external limits with the default values
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">chain_ids::tron_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_mainnet">chain_ids::ltc_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_mainnet">chain_ids::doge_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_mainnet">chain_ids::sui_official_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_mainnet">chain_ids::aptos_mainnet</a>()),
        100_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    // Testnet and custom chains
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_testnet">chain_ids::ltc_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_testnet">chain_ids::doge_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">chain_ids::sui_official_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">chain_ids::aptos_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    //custom chains
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_testnet">chain_ids::ltc_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_testnet">chain_ids::doge_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">chain_ids::sui_official_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
        external_limiter,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>( <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">chain_ids::aptos_testnet</a>()),
        100 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
}
</code></pre>



</details>

<a name="bridge_limiter_initial_external_24h_limits"></a>

## Function `initial_external_24h_limits`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_initial_external_24h_limits">initial_external_24h_limits</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_initial_external_24h_limits">initial_external_24h_limits</a>(
    parent_id: &<b>mut</b> UID,
) {
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>()),
        // 10 BTC, assuming 1 BTC = 80,000 USD
        800_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>()),
        // 100_0000 USD
        100_0000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">chain_ids::tron_mainnet</a>()),
        // 100_0000 USD
        100_0000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    // testnet
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>()),
        // 0.005 BTC, assuming 1 BTC = 100,000 USD
        500 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>()),
        1000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>()),
        1000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    // custom
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>()),
        // 0.005 BTC, assuming 1 BTC = 100,000 USD
        500 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>()),
        1000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
    <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
        parent_id,
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>()),
        1000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>,
    );
}
</code></pre>



</details>

<a name="bridge_limiter_get_external_limiter"></a>

## Function `get_external_limiter`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_external_limiter">get_external_limiter</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">bridge::limiter::ExternalLimiter</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_external_limiter">get_external_limiter</a>(
    parent_id: &UID
): &<a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a> {
    dynamic_field::borrow&lt;vector&lt;u8&gt;, <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a>&gt;(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>)
}
</code></pre>



</details>

<a name="bridge_limiter_update_external_out_limit"></a>

## Function `update_external_out_limit`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_external_out_limit">update_external_out_limit</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, limit: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_external_out_limit">update_external_out_limit</a>(
    parent_id: &<b>mut</b> UID,
    route: &BridgeRoute,
    limit: u64,
) {
    <b>let</b> external_limiter = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a>&gt;(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>);
    external_limiter.<a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(route, limit);
}
</code></pre>



</details>

<a name="bridge_limiter_add_external_out_limit"></a>

## Function `add_external_out_limit`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(external_limits: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">bridge::limiter::ExternalLimiter</a>, route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, limit: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_add_external_out_limit">add_external_out_limit</a>(
    external_limits: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a>,
    route: &BridgeRoute,
    limit: u64,
) {
    <b>if</b> (!external_limits.transfer_out_limits.contains(route)) {
        external_limits.transfer_out_limits.insert(*route, limit);
    } <b>else</b> {
        *&<b>mut</b> external_limits.transfer_out_limits[route] = limit;
    };
}
</code></pre>



</details>

<a name="bridge_limiter_get_external_out_limit"></a>

## Function `get_external_out_limit`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_external_out_limit">get_external_out_limit</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_external_out_limit">get_external_out_limit</a>(
    parent_id: &UID,
    route: &BridgeRoute
): u64 {
    <b>let</b> external_limiter = <a href="../bridge/limiter.md#bridge_limiter_get_external_limiter">get_external_limiter</a>(parent_id);
    <b>let</b> limit = external_limiter.transfer_out_limits.try_get(route);
    <b>assert</b>!(limit.is_some(), <a href="../bridge/limiter.md#bridge_limiter_EExternalLimitNotFoundForRoute">EExternalLimitNotFoundForRoute</a>);
    limit.destroy_some()
}
</code></pre>



</details>

<a name="bridge_limiter_update_external_24h_limit"></a>

## Function `update_external_24h_limit`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, limit: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_external_24h_limit">update_external_24h_limit</a>(
    parent_id: &<b>mut</b> UID,
    route: &BridgeRoute,
    limit: u64,
) {
    <b>let</b> <a href="../bridge/limiter.md#bridge_limiter">limiter</a> = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a>&gt;(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>);
    <b>if</b> (!<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.contains(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> {})) {
         <a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.add(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> {}, vec_map::empty&lt;BridgeRoute, u64&gt;());
    };
    <b>let</b> limits = <a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.borrow_mut&lt;<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a>, VecMap&lt;BridgeRoute, u64&gt;&gt;(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> {});
    <b>if</b> (limits.contains(route)) {
        *limits.get_mut(route) = limit;
    } <b>else</b> {
        limits.insert(*route, limit);
    }
}
</code></pre>



</details>

<a name="bridge_limiter_check_and_record_external_24h_transfer"></a>

## Function `check_and_record_external_24h_transfer`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_check_and_record_external_24h_transfer">check_and_record_external_24h_transfer</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, route: <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_check_and_record_external_24h_transfer">check_and_record_external_24h_transfer</a>(
    parent_id: &<b>mut</b> UID,
    clock: &Clock,
    route: BridgeRoute,
    amount: u64
): bool {
    <b>let</b> <a href="../bridge/limiter.md#bridge_limiter">limiter</a> = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a>&gt;(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>);
    // 1. Check <b>if</b> 24h limit exists <b>for</b> this route
    <b>if</b> (!<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.contains(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> {})) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> limits = <a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.borrow&lt;<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a>, VecMap&lt;BridgeRoute, u64&gt;&gt;(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> {});
    <b>if</b> (!limits.contains(&route)) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> limit = *limits.get(&route);
    // 2. Get/Init records
    <b>if</b> (!<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.contains(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a> {})) {
        <a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.add(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a> {}, vec_map::empty&lt;BridgeRoute, <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">TransferRecord</a>&gt;());
    };
    <b>let</b> records = <a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.borrow_mut&lt;<a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a>, VecMap&lt;BridgeRoute, <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">TransferRecord</a>&gt;&gt;(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a> {});
    <b>if</b> (!records.contains(&route)) {
        records.insert(route, <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">TransferRecord</a> {
            hour_head: 0,
            hour_tail: 0,
            per_hour_amounts: vector[],
            total_amount: 0
        });
    };
    <b>let</b> record = records.get_mut(&route);
    // 3. Adjust window
    <b>let</b> current_hour = <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>(clock);
    record.<a href="../bridge/limiter.md#bridge_limiter_adjust_transfer_records">adjust_transfer_records</a>(current_hour);
    // 4. Check limit
    <b>if</b> (record.total_amount + amount &gt; limit) {
        <b>return</b> <b>false</b>
    };
    // 5. Update record
    <b>let</b> new_amount = record.per_hour_amounts.pop_back() + amount;
    record.per_hour_amounts.push_back(new_amount);
    record.total_amount = record.total_amount + amount;
    <b>true</b>
}
</code></pre>



</details>

<a name="bridge_limiter_get_external_available_transfer_amount"></a>

## Function `get_external_available_transfer_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_external_available_transfer_amount">get_external_available_transfer_amount</a>&lt;T&gt;(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, route: <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_external_available_transfer_amount">get_external_available_transfer_amount</a>&lt;T&gt;(
    parent_id: &UID,
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &BridgeTreasury,
    route: BridgeRoute
): u128 {
    <b>let</b> <a href="../bridge/limiter.md#bridge_limiter">limiter</a> = dynamic_field::borrow&lt;vector&lt;u8&gt;, <a href="../bridge/limiter.md#bridge_limiter_ExternalLimiter">ExternalLimiter</a>&gt;(parent_id, <a href="../bridge/limiter.md#bridge_limiter_EXTERNAL_LIMITS_KEY">EXTERNAL_LIMITS_KEY</a>);
    // 1. Check <b>if</b> 24h limit exists <b>for</b> this route
    <b>if</b> (!<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.contains(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> {})) {
        <b>abort</b> <a href="../bridge/limiter.md#bridge_limiter_EExternalLimitNotFoundForRoute">EExternalLimitNotFoundForRoute</a>
    };
    <b>let</b> limits = <a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.borrow&lt;<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a>, VecMap&lt;BridgeRoute, u64&gt;&gt;(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransfer24hLimitsKey">ExternalTransfer24hLimitsKey</a> {});
    <b>if</b> (!limits.contains(&route)) {
        <b>abort</b> <a href="../bridge/limiter.md#bridge_limiter_EExternalLimitNotFoundForRoute">EExternalLimitNotFoundForRoute</a>
    };
    <b>let</b> limit = *limits.get(&route);
    // 2. Get records
    <b>let</b> total_used = <b>if</b> (<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.contains(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a> {})) {
        <b>let</b> records = <a href="../bridge/limiter.md#bridge_limiter">limiter</a>.external.borrow&lt;<a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a>, VecMap&lt;BridgeRoute, <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">TransferRecord</a>&gt;&gt;(<a href="../bridge/limiter.md#bridge_limiter_ExternalTransferRecordsKey">ExternalTransferRecordsKey</a> {});
        <b>if</b> (records.contains(&route)) {
            records.get(&route).total_amount
        } <b>else</b> {
            0
        }
    } <b>else</b> {
        0
    };
    <b>if</b> (total_used &gt;= limit) {
        <b>return</b> 0
    };
    <b>let</b> remaining_usd_limit = limit - total_used;
    // 3. Convert USD limit to Token Amount
    <b>let</b> price = (<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.notional_value&lt;T&gt;() <b>as</b> u128);
    <b>if</b> (price == 0) {
        <b>return</b> 0
    };
    <b>let</b> remaining_adjusted = (remaining_usd_limit <b>as</b> u128) * (<a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a> <b>as</b> u128);
    <b>let</b> available = remaining_adjusted / price;
    <b>return</b> available
}
</code></pre>



</details>

<a name="bridge_limiter_get_available_claim_amount"></a>

## Function `get_available_claim_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(self: &<a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>, <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, route: <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(
    self: &<a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a>,
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &BridgeTreasury,
    route: BridgeRoute,
): u128{
    <b>let</b> route_limit = self.transfer_limits.try_get(&route);
    <b>assert</b>!(route_limit.is_some(), <a href="../bridge/limiter.md#bridge_limiter_ELimitNotFoundForRoute">ELimitNotFoundForRoute</a>);
    <b>let</b> route_limit = route_limit.destroy_some();
    <b>let</b> price = (<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.notional_value&lt;T&gt;() <b>as</b> u128);
    <b>if</b> (price == 0) {
        <b>return</b> 0
    };
    <b>let</b> route_limit_adjusted =
        (route_limit <b>as</b> u128) * (<a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a> <b>as</b> u128);
    <b>if</b> (!self.transfer_records.contains(&route)) {
        <b>return</b> (route_limit_adjusted / price)
    };
    <b>let</b> record=self.transfer_records.get(&route);
    <b>let</b> total_adjusted= (record.total_amount <b>as</b> u128 ) * (<a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a> <b>as</b> u128);
    <b>if</b> (total_adjusted &gt;= route_limit_adjusted){
        <b>return</b> 0
    };
    <b>let</b> available_amount=((route_limit_adjusted-total_adjusted) / price);
    available_amount
}
</code></pre>



</details>

<a name="bridge_limiter_check_and_record_sending_transfer"></a>

## Function `check_and_record_sending_transfer`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_check_and_record_sending_transfer">check_and_record_sending_transfer</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>, <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, route: <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_check_and_record_sending_transfer">check_and_record_sending_transfer</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a>,
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &BridgeTreasury,
    clock: &Clock,
    route: BridgeRoute,
    amount: u64
): bool {
    // Create record <b>for</b> route <b>if</b> not exists
    <b>if</b> (!self.transfer_records.contains(&route)) {
        self.transfer_records.insert(route, <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">TransferRecord</a> {
            hour_head: 0,
            hour_tail: 0,
            per_hour_amounts: vector[],
            total_amount: 0
        })
    };
    <b>let</b> record = self.transfer_records.get_mut(&route);
    <b>let</b> <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a> = <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>(clock);
    record.<a href="../bridge/limiter.md#bridge_limiter_adjust_transfer_records">adjust_transfer_records</a>(<a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>);
    // Get limit <b>for</b> the route
    <b>let</b> route_limit = self.transfer_limits.try_get(&route);
    <b>assert</b>!(route_limit.is_some(), <a href="../bridge/limiter.md#bridge_limiter_ELimitNotFoundForRoute">ELimitNotFoundForRoute</a>);
    <b>let</b> route_limit = route_limit.destroy_some();
    <b>let</b> route_limit_adjusted =
        (route_limit <b>as</b> u128) * (<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.decimal_multiplier&lt;T&gt;() <b>as</b> u128);
    // Compute notional amount
    // Upcast to u128 to prevent overflow, to not miss out on small amounts.
    <b>let</b> value = (<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.notional_value&lt;T&gt;() <b>as</b> u128);
    <b>let</b> notional_amount_with_token_multiplier = value * (amount <b>as</b> u128);
    // Check <b>if</b> transfer amount exceed limit
    // Upscale them to the token's decimal.
    <b>if</b> ((record.total_amount <b>as</b> u128)
        * (<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.decimal_multiplier&lt;T&gt;() <b>as</b> u128)
        + notional_amount_with_token_multiplier &gt; route_limit_adjusted
    ) {
        <b>return</b> <b>false</b>
    };
    // Now scale down to notional value
    <b>let</b> notional_amount = notional_amount_with_token_multiplier
        / (<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.decimal_multiplier&lt;T&gt;() <b>as</b> u128);
    // Should be safe to downcast to u64 after dividing by the decimals
    <b>let</b> notional_amount = (notional_amount <b>as</b> u64);
    // Record transfer value
    <b>let</b> new_amount = record.per_hour_amounts.pop_back() + notional_amount;
    record.per_hour_amounts.push_back(new_amount);
    record.total_amount = record.total_amount + notional_amount;
    <b>true</b>
}
</code></pre>



</details>

<a name="bridge_limiter_update_route_limit"></a>

## Function `update_route_limit`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>, route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, new_usd_limit: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
    self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a>,
    route: &BridgeRoute,
    new_usd_limit: u64
) {
    <b>let</b> receiving_chain = *route.destination();
    <b>if</b> (!self.transfer_limits.contains(route)) {
        self.transfer_limits.insert(*route, new_usd_limit);
    } <b>else</b> {
        *&<b>mut</b> self.transfer_limits[route] = new_usd_limit;
    };
    emit(<a href="../bridge/limiter.md#bridge_limiter_UpdateRouteLimitEvent">UpdateRouteLimitEvent</a> {
        sending_chain: *route.source(),
        receiving_chain,
        new_limit: new_usd_limit,
    })
}
</code></pre>



</details>

<a name="bridge_limiter_current_hour_since_epoch"></a>

## Function `current_hour_since_epoch`



<pre><code><b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>(clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>(clock: &Clock): u64 {
    clock::timestamp_ms(clock) / 3600000
}
</code></pre>



</details>

<a name="bridge_limiter_adjust_transfer_records"></a>

## Function `adjust_transfer_records`



<pre><code><b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_adjust_transfer_records">adjust_transfer_records</a>(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">bridge::limiter::TransferRecord</a>, <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_adjust_transfer_records">adjust_transfer_records</a>(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferRecord">TransferRecord</a>, <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>: u64) {
    <b>if</b> (self.hour_head == <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>) {
        <b>if</b>(self.per_hour_amounts.length() == 0) {
            self.per_hour_amounts.push_back(0);
        };
        <b>return</b> // nothing to backfill
    };
    <b>let</b> target_tail = <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a> - 23;
    // If `hour_head` is even older than 24 hours ago, it means all items in
    // `per_hour_amounts` are to be evicted.
    <b>if</b> (self.hour_head &lt; target_tail) {
        self.per_hour_amounts = vector[];
        self.total_amount = 0;
        self.hour_tail = target_tail;
        self.hour_head = target_tail;
        // Don't forget to insert this hour's record
        self.per_hour_amounts.push_back(0);
    } <b>else</b> {
        // self.hour_head is within 24 hour range.
        // some items in `per_hour_amounts` are still valid, we remove stale hours.
        <b>while</b> (self.hour_tail &lt; target_tail) {
            self.total_amount = self.total_amount - self.per_hour_amounts.remove(0);
            self.hour_tail = self.hour_tail + 1;
        }
    };
    // Backfill from hour_head to current hour
    <b>while</b> (self.hour_head &lt; <a href="../bridge/limiter.md#bridge_limiter_current_hour_since_epoch">current_hour_since_epoch</a>) {
        self.per_hour_amounts.push_back(0);
        self.hour_head = self.hour_head + 1;
    }
}
</code></pre>



</details>

<a name="bridge_limiter_initial_transfer_limits"></a>

## Function `initial_transfer_limits`



<pre><code><b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_initial_transfer_limits">initial_transfer_limits</a>(): <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_initial_transfer_limits">initial_transfer_limits</a>(): VecMap&lt;BridgeRoute, u64&gt; {
    <b>let</b> <b>mut</b> transfer_limits = vec_map::empty();
    // 5M limit on Sui -&gt; Ethereum mainnet
    transfer_limits.insert(
        <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        500_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    // MAX limit <b>for</b> testnet and devnet
    transfer_limits.insert(
        <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    transfer_limits.insert(
        <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    transfer_limits.insert(
        <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    transfer_limits.insert(
        <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    transfer_limits
}
</code></pre>



</details>

<a name="bridge_limiter_update_transfer_limits"></a>

## Function `update_transfer_limits`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_transfer_limits">update_transfer_limits</a>(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter.md#bridge_limiter_update_transfer_limits">update_transfer_limits</a>(self: &<b>mut</b> <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">TransferLimiter</a>)
{
    // 1B limit on Sui -&gt; BSC mainnet
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    // 1B limit on Sui -&gt; Base mainnet
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>()),
        1_000_000_000 * <a href="../bridge/limiter.md#bridge_limiter_USD_VALUE_MULTIPLIER">USD_VALUE_MULTIPLIER</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
    self.<a href="../bridge/limiter.md#bridge_limiter_update_route_limit">update_route_limit</a>(
        &<a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>(), <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()),
        <a href="../bridge/limiter.md#bridge_limiter_MAX_TRANSFER_LIMIT">MAX_TRANSFER_LIMIT</a>
    );
}
</code></pre>



</details>

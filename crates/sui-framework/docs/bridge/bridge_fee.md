---
title: Module `bridge::bridge_fee`
---



-  [Struct `WithdrawBridgeFeeCap`](#bridge_bridge_fee_WithdrawBridgeFeeCap)
-  [Struct `BridgeFee`](#bridge_bridge_fee_BridgeFee)
-  [Struct `FeeInfo`](#bridge_bridge_fee_FeeInfo)
-  [Constants](#@Constants_0)
-  [Function `new_bridge_fee_registry`](#bridge_bridge_fee_new_bridge_fee_registry)
-  [Function `new`](#bridge_bridge_fee_new)
-  [Function `empty`](#bridge_bridge_fee_empty)
-  [Function `initial_bridge_fee`](#bridge_bridge_fee_initial_bridge_fee)
-  [Function `borrow`](#bridge_bridge_fee_borrow)
-  [Function `borrow_mut`](#bridge_bridge_fee_borrow_mut)
-  [Function `set_fee_in_cross_in`](#bridge_bridge_fee_set_fee_in_cross_in)
-  [Function `set_fee_in_cross_out`](#bridge_bridge_fee_set_fee_in_cross_out)
-  [Function `get_cross_out_amount_after_fee`](#bridge_bridge_fee_get_cross_out_amount_after_fee)
-  [Function `get_cross_in_amount_after_fee`](#bridge_bridge_fee_get_cross_in_amount_after_fee)
-  [Function `get_fee_info_cross_in`](#bridge_bridge_fee_get_fee_info_cross_in)
-  [Function `get_fee_info_cross_out`](#bridge_bridge_fee_get_fee_info_cross_out)
-  [Function `calculate_cross_out_fee_amount`](#bridge_bridge_fee_calculate_cross_out_fee_amount)
-  [Function `calculate_cross_in_fee_amount`](#bridge_bridge_fee_calculate_cross_in_fee_amount)
-  [Function `deposit_fee`](#bridge_bridge_fee_deposit_fee)
-  [Function `withdraw_fee`](#bridge_bridge_fee_withdraw_fee)
-  [Function `create_withdraw_fee_cap`](#bridge_bridge_fee_create_withdraw_fee_cap)
-  [Function `get_withdraw_cap_coin_type`](#bridge_bridge_fee_get_withdraw_cap_coin_type)
-  [Function `get_withdraw_cap_amount`](#bridge_bridge_fee_get_withdraw_cap_amount)
-  [Function `get_unclaimed_bridge_fee`](#bridge_bridge_fee_get_unclaimed_bridge_fee)
-  [Function `is_asset_supported`](#bridge_bridge_fee_is_asset_supported)
-  [Function `is_token_id_supported_in_cross_out_internal`](#bridge_bridge_fee_is_token_id_supported_in_cross_out_internal)
-  [Function `is_token_id_supported_in_cross_in_internal`](#bridge_bridge_fee_is_token_id_supported_in_cross_in_internal)
-  [Function `get_fee_info_from_benfen`](#bridge_bridge_fee_get_fee_info_from_benfen)
-  [Function `get_fee_info_to_benfen`](#bridge_bridge_fee_get_fee_info_to_benfen)
-  [Function `default_fee_info`](#bridge_bridge_fee_default_fee_info)
-  [Function `new_fee_info`](#bridge_bridge_fee_new_fee_info)
-  [Function `calculate_fee`](#bridge_bridge_fee_calculate_fee)


<pre><code><b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../bridge/tokenlist.md#bridge_tokenlist">bridge::tokenlist</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="bridge_bridge_fee_WithdrawBridgeFeeCap"></a>

## Struct `WithdrawBridgeFeeCap`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">WithdrawBridgeFeeCap</a> <b>has</b> key, store
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
<code>coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_fee_BridgeFee"></a>

## Struct `BridgeFee`

Bridge fee configuration struct, manages fee settings for cross-chain in and out operations,
as well as a funds pool to hold coin balances.


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>from_benfen: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>&gt;&gt;</code>
</dt>
<dd>
 Cross-chain out fee configuration:
 Maps chain_id -> token_id -> FeeInfo
</dd>
<dt>
<code>to_benfen: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>&gt;&gt;</code>
</dt>
<dd>
 Cross-chain in fee configuration:
 Maps chain_id -> token_id -> FeeInfo
</dd>
<dt>
<code>funds: <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a></code>
</dt>
<dd>
 Funds pool to store balances of various coin types,
 e.g., funds[TypeName] = Balance<T>
</dd>
</dl>


</details>

<a name="bridge_bridge_fee_FeeInfo"></a>

## Struct `FeeInfo`

Fee information struct supports fixed and percentage fee modes.


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>mode: u64</code>
</dt>
<dd>
 Fee mode, either Fixed or Percentage.
</dd>
<dt>
<code>value: u64</code>
</dt>
<dd>
 Fee value:
 - If mode == Fixed(0), this is the fixed fee amount (same unit as token amount).
 - If mode == Percentage(1), this is the fee rate in parts per million (ppm).
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_bridge_fee_KEY"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_KEY">KEY</a>: vector&lt;u8&gt; = vector[98, 114, 105, 100, 103, 101, 95, 102, 101, 101];
</code></pre>



<a name="bridge_bridge_fee_EBridgeFeeRegistryAlreadyExists"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeRegistryAlreadyExists">EBridgeFeeRegistryAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="bridge_bridge_fee_EBridgeFeeTypeNotSupport"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeTypeNotSupport">EBridgeFeeTypeNotSupport</a>: u64 = 1;
</code></pre>



<a name="bridge_bridge_fee_EBridgeFeeChainIDAndTokenIDNotExpect"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeChainIDAndTokenIDNotExpect">EBridgeFeeChainIDAndTokenIDNotExpect</a>: u64 = 3;
</code></pre>



<a name="bridge_bridge_fee_EBridgeFeeWithdrawCoinTypeNotMatch"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeWithdrawCoinTypeNotMatch">EBridgeFeeWithdrawCoinTypeNotMatch</a>: u64 = 4;
</code></pre>



<a name="bridge_bridge_fee_EBridgeFeeSettingWrong"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeSettingWrong">EBridgeFeeSettingWrong</a>: u64 = 4;
</code></pre>



<a name="bridge_bridge_fee_FEE_RATE_PRECISION"></a>

Fee rates are expressed in millionths (1e6 precision)
e.g. 1% = 10000; 0.01% = 100; 0.0001% = 1


<pre><code><b>const</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FEE_RATE_PRECISION">FEE_RATE_PRECISION</a>: u64 = 1000000;
</code></pre>



<a name="bridge_bridge_fee_new_bridge_fee_registry"></a>

## Function `new_bridge_fee_registry`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_bridge_fee_registry">new_bridge_fee_registry</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_bridge_fee_registry">new_bridge_fee_registry</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext) {
   <b>assert</b>!(
       !dynamic_field::exists_(parent_id, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_KEY">KEY</a>),
       <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeRegistryAlreadyExists">EBridgeFeeRegistryAlreadyExists</a> // TODO - add custom error type
   );
   dynamic_field::add(
       parent_id,
       <a href="../bridge/bridge_fee.md#bridge_bridge_fee_KEY">KEY</a>,
       <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new">new</a>(ctx),
   );
   <a href="../bridge/bridge_fee.md#bridge_bridge_fee_initial_bridge_fee">initial_bridge_fee</a>(parent_id,ctx);
}
</code></pre>



</details>

<a name="bridge_bridge_fee_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new">new</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a> {
    <b>let</b> (from_benfen, to_benfen, funds) = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_empty">empty</a>(ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a> {
        from_benfen,
        to_benfen,
        funds,
    }
}
</code></pre>



</details>

<a name="bridge_bridge_fee_empty"></a>

## Function `empty`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_empty">empty</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>&gt;&gt;, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>&gt;&gt;, <a href="../sui/bag.md#sui_bag_Bag">sui::bag::Bag</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_empty">empty</a>(
    ctx: &<b>mut</b> TxContext
): (
    Table&lt;u64, Table&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>&gt;&gt;,
    Table&lt;u64, Table&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>&gt;&gt;,
    Bag,
) {
    <b>let</b> from_benfen = table::new&lt;u64, Table&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>&gt;&gt;(ctx);
    <b>let</b> to_benfen = table::new&lt;u64, Table&lt;u64, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>&gt;&gt;(ctx);
    <b>let</b> funds = bag::new(ctx);
    (from_benfen, to_benfen, funds)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_initial_bridge_fee"></a>

## Function `initial_bridge_fee`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_initial_bridge_fee">initial_bridge_fee</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_initial_bridge_fee">initial_bridge_fee</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext) {
    //todo btc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_btc_token_id">tokenlist::get_btc_token_id</a>() ,0,43202,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_btc_token_id">tokenlist::get_btc_token_id</a>() ,0,43202,ctx);
    // tron usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">chain_ids::tron_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,0,5_000_000_000,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,0,5_000_000_000,ctx);
    //ETH usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //ETH usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    //bsc usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //bsc usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    //base usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //base usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    //op usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //op usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    //polygon usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //polygon usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    //arb usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //arb usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    //avax usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //avax usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    //sol usdc
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>()<b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>() ,1,500,ctx);
    //sol usdt
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>() ,1,500,ctx);
}
</code></pre>



</details>

<a name="bridge_bridge_fee_borrow"></a>

## Function `borrow`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id: &UID): &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>{
    dynamic_field::borrow&lt;vector&lt;u8&gt;,<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>&gt;(parent_id, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<b>mut</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> UID): &<b>mut</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>{
    dynamic_field::borrow_mut&lt;vector&lt;u8&gt;,<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>&gt;(parent_id, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_set_fee_in_cross_in"></a>

## Function `set_fee_in_cross_in`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_in">set_fee_in_cross_in</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, mode: u64, value: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_in">set_fee_in_cross_in</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    mode: u64,
    value: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(mode &lt; 2,<a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeTypeNotSupport">EBridgeFeeTypeNotSupport</a>);
    <b>let</b> self=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.to_benfen.contains(chain_id)) {
        self.to_benfen.add(chain_id, table::new(ctx));
    };
    <b>let</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>(mode,value);
    <b>assert</b>!(<a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>.mode &lt; 2,<a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeTypeNotSupport">EBridgeFeeTypeNotSupport</a>);
    <b>if</b> (!self.to_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(chain_id).contains(token_id)){
        self.to_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>);
    }<b>else</b>{
       *self.to_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(chain_id).<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(token_id)=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>
    }
}
</code></pre>



</details>

<a name="bridge_bridge_fee_set_fee_in_cross_out"></a>

## Function `set_fee_in_cross_out`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, mode: u64, value: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">set_fee_in_cross_out</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    mode: u64,
    value: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(mode &lt; 2,<a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeTypeNotSupport">EBridgeFeeTypeNotSupport</a>);
    <b>let</b> self=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.from_benfen.contains(chain_id)) {
        self.from_benfen.add(chain_id, table::new(ctx));
    };
    <b>let</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>(mode,value);
    <b>assert</b>!(<a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>.mode &lt; 2,<a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeTypeNotSupport">EBridgeFeeTypeNotSupport</a>);
    <b>if</b> (!self.from_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(chain_id).contains(token_id)){
        self.from_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>);
    }<b>else</b>{
       *self.from_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(chain_id).<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(token_id)=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>
    }
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_cross_out_amount_after_fee"></a>

## Function `get_cross_out_amount_after_fee`

Compute the net amount after applying the fee


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_cross_out_amount_after_fee">get_cross_out_amount_after_fee</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_cross_out_amount_after_fee">get_cross_out_amount_after_fee</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64,
    amount: u64
): u64 {
    <b>if</b> (!<a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_token_id_supported_in_cross_out_internal">is_token_id_supported_in_cross_out_internal</a>(<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id), chain_id, token_id)) {
        <b>abort</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeChainIDAndTokenIDNotExpect">EBridgeFeeChainIDAndTokenIDNotExpect</a>
    };
    <b>let</b> fee = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">calculate_cross_out_fee_amount</a>(parent_id, chain_id, token_id, amount);
    <b>if</b> (amount &gt; fee) {
        amount - fee
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_cross_in_amount_after_fee"></a>

## Function `get_cross_in_amount_after_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_cross_in_amount_after_fee">get_cross_in_amount_after_fee</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_cross_in_amount_after_fee">get_cross_in_amount_after_fee</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64,
    amount: u64
): u64 {
    <b>if</b> (!<a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_token_id_supported_in_cross_in_internal">is_token_id_supported_in_cross_in_internal</a>(<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id), chain_id, token_id)) {
        <b>abort</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeChainIDAndTokenIDNotExpect">EBridgeFeeChainIDAndTokenIDNotExpect</a>
    };
    <b>let</b> fee = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">calculate_cross_in_fee_amount</a>(parent_id, chain_id, token_id, amount);
    <b>if</b> (amount &gt; fee) {
        amount - fee
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_fee_info_cross_in"></a>

## Function `get_fee_info_cross_in`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_cross_in">get_fee_info_cross_in</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_cross_in">get_fee_info_cross_in</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64,
): (u64,u64){
    <b>let</b> self = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id);
    <b>let</b> fee_info= <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_to_benfen">get_fee_info_to_benfen</a>(self, chain_id, token_id);
    (fee_info.mode,fee_info.value)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_fee_info_cross_out"></a>

## Function `get_fee_info_cross_out`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_cross_out">get_fee_info_cross_out</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_cross_out">get_fee_info_cross_out</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64,
): (u64,u64){
    <b>let</b> self = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id);
    <b>let</b> fee_info=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_from_benfen">get_fee_info_from_benfen</a>(self, chain_id, token_id);
    (fee_info.mode,fee_info.value)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_calculate_cross_out_fee_amount"></a>

## Function `calculate_cross_out_fee_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">calculate_cross_out_fee_amount</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">calculate_cross_out_fee_amount</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64,
    amount: u64
): u64 {
    <b>let</b> self = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id);
    <b>let</b> fee_info = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_from_benfen">get_fee_info_from_benfen</a>(self, chain_id, token_id);
    // Fixed fee mode
    <b>if</b> (fee_info.mode == 0) {
        <b>return</b> fee_info.value
    };
    // Percentage-based fee mode
    <b>if</b> (fee_info.mode == 1) {
        <b>return</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_fee">calculate_fee</a>(amount, fee_info.value)
    };
    0
}
</code></pre>



</details>

<a name="bridge_bridge_fee_calculate_cross_in_fee_amount"></a>

## Function `calculate_cross_in_fee_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">calculate_cross_in_fee_amount</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">calculate_cross_in_fee_amount</a>(
   parent_id: &UID,
   chain_id: u64,
   token_id: u64,
   amount: u64
): u64 {
   <b>let</b> self = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id);
   <b>let</b> fee_info = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_to_benfen">get_fee_info_to_benfen</a>(self, chain_id, token_id);
   // Fixed fee mode
   <b>if</b> (fee_info.mode == 0) {
       <b>return</b> fee_info.value
   };
   // Percentage-based fee mode
   <b>if</b> (fee_info.mode == 1) {
       <b>return</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_fee">calculate_fee</a>(amount, fee_info.value)
   };
   0
}
</code></pre>



</details>

<a name="bridge_bridge_fee_deposit_fee"></a>

## Function `deposit_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">deposit_fee</a>&lt;T&gt;(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, coin: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">deposit_fee</a>&lt;T&gt;(parent_id: &<b>mut</b> UID,coin: Coin&lt;T&gt;){
    <b>let</b> coin_type=type_name::get&lt;T&gt;();
    <b>let</b> self=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_asset_supported">is_asset_supported</a>(coin_type)){
        //create funds store
        bag::add(&<b>mut</b> self.funds, coin_type, balance::zero&lt;T&gt;())
    };
    <b>let</b> bal=bag::borrow_mut&lt;TypeName,Balance&lt;T&gt;&gt;(&<b>mut</b> self.funds, coin_type);
    bal.join(coin.into_balance());
}
</code></pre>



</details>

<a name="bridge_bridge_fee_withdraw_fee"></a>

## Function `withdraw_fee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_withdraw_fee">withdraw_fee</a>&lt;T&gt;(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, cap: <a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">bridge::bridge_fee::WithdrawBridgeFeeCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_withdraw_fee">withdraw_fee</a>&lt;T&gt;(parent_id: &<b>mut</b> UID, cap: <a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">WithdrawBridgeFeeCap</a>,ctx: &<b>mut</b> TxContext): Coin&lt;T&gt;{
    <b>let</b> input_coin_type=type_name::get&lt;T&gt;();
    <b>let</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">WithdrawBridgeFeeCap</a>{
        id,
        coin_type,
        amount
    }=cap;
    <b>let</b> input_coin_type_str =input_coin_type.into_string();
    <b>assert</b>!(input_coin_type_str==coin_type,<a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeWithdrawCoinTypeNotMatch">EBridgeFeeWithdrawCoinTypeNotMatch</a>);
    object::delete(id);
    <b>let</b> self=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_asset_supported">is_asset_supported</a>(input_coin_type)){
        <b>return</b> coin::zero&lt;T&gt;(ctx)
    };
    <b>let</b> bal=bag::borrow_mut&lt;TypeName,Balance&lt;T&gt;&gt;(&<b>mut</b> self.funds, input_coin_type);
    bal.split(amount).into_coin(ctx)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_create_withdraw_fee_cap"></a>

## Function `create_withdraw_fee_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_create_withdraw_fee_cap">create_withdraw_fee_cap</a>(coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">bridge::bridge_fee::WithdrawBridgeFeeCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_create_withdraw_fee_cap">create_withdraw_fee_cap</a>(coin_type: String,amount: u64,ctx: &<b>mut</b> TxContext):<a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">WithdrawBridgeFeeCap</a>{
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">WithdrawBridgeFeeCap</a> {
        id: object::new(ctx),
        coin_type,
        amount
    }
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_withdraw_cap_coin_type"></a>

## Function `get_withdraw_cap_coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_withdraw_cap_coin_type">get_withdraw_cap_coin_type</a>(cap: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">bridge::bridge_fee::WithdrawBridgeFeeCap</a>): <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_withdraw_cap_coin_type">get_withdraw_cap_coin_type</a>(cap: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">WithdrawBridgeFeeCap</a>): String{
    cap.coin_type
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_withdraw_cap_amount"></a>

## Function `get_withdraw_cap_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_withdraw_cap_amount">get_withdraw_cap_amount</a>(cap: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">bridge::bridge_fee::WithdrawBridgeFeeCap</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_withdraw_cap_amount">get_withdraw_cap_amount</a>(cap: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_WithdrawBridgeFeeCap">WithdrawBridgeFeeCap</a>): u64{
    cap.amount
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_unclaimed_bridge_fee"></a>

## Function `get_unclaimed_bridge_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_unclaimed_bridge_fee">get_unclaimed_bridge_fee</a>&lt;T&gt;(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_unclaimed_bridge_fee">get_unclaimed_bridge_fee</a>&lt;T&gt;(parent_id: &UID): u64{
    <b>let</b> coin_type=type_name::get&lt;T&gt;();
    <b>let</b> self=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(parent_id);
    <b>if</b> (!self.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_asset_supported">is_asset_supported</a>(coin_type)){
        <b>return</b> 0
    };
    bag::borrow&lt;TypeName,Balance&lt;T&gt;&gt;(&self.funds, coin_type).value()
}
</code></pre>



</details>

<a name="bridge_bridge_fee_is_asset_supported"></a>

## Function `is_asset_supported`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_asset_supported">is_asset_supported</a>(self: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>, coin_type: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_asset_supported">is_asset_supported</a>(self :&<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>,coin_type: TypeName):bool{
    bag::contains(&self.funds, coin_type)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_is_token_id_supported_in_cross_out_internal"></a>

## Function `is_token_id_supported_in_cross_out_internal`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_token_id_supported_in_cross_out_internal">is_token_id_supported_in_cross_out_internal</a>(self: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_token_id_supported_in_cross_out_internal">is_token_id_supported_in_cross_out_internal</a>(self :&<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>,chain_id: u64, token_id: u64):bool{
    <b>if</b> (!self.from_benfen.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> inner = self.from_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> <b>false</b>
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="bridge_bridge_fee_is_token_id_supported_in_cross_in_internal"></a>

## Function `is_token_id_supported_in_cross_in_internal`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_token_id_supported_in_cross_in_internal">is_token_id_supported_in_cross_in_internal</a>(self: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_is_token_id_supported_in_cross_in_internal">is_token_id_supported_in_cross_in_internal</a>(self :&<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>,chain_id: u64, token_id: u64):bool{
    <b>if</b> (!self.to_benfen.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> inner = self.to_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> <b>false</b>
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_fee_info_from_benfen"></a>

## Function `get_fee_info_from_benfen`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_from_benfen">get_fee_info_from_benfen</a>(self: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>, chain_id: u64, token_id: u64): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_from_benfen">get_fee_info_from_benfen</a>(self :&<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>,chain_id: u64, token_id: u64): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>{
     <b>if</b> (!self.from_benfen.contains(chain_id)) {
       <b>return</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_default_fee_info">default_fee_info</a>()
    };
    <b>let</b> inner = self.from_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
      <b>return</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_default_fee_info">default_fee_info</a>()
    };
    *inner.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_get_fee_info_to_benfen"></a>

## Function `get_fee_info_to_benfen`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_to_benfen">get_fee_info_to_benfen</a>(self: &<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">bridge::bridge_fee::BridgeFee</a>, chain_id: u64, token_id: u64): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_fee_info_to_benfen">get_fee_info_to_benfen</a>(self :&<a href="../bridge/bridge_fee.md#bridge_bridge_fee_BridgeFee">BridgeFee</a>,chain_id: u64, token_id: u64): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>{
    <b>if</b> (!self.to_benfen.contains(chain_id)) {
      <b>return</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_default_fee_info">default_fee_info</a>()
   };
   <b>let</b> inner = self.to_benfen.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(chain_id);
   <b>if</b> (!inner.contains(token_id)) {
     <b>return</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_default_fee_info">default_fee_info</a>()
   };
   *inner.<a href="../bridge/bridge_fee.md#bridge_bridge_fee_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

<a name="bridge_bridge_fee_default_fee_info"></a>

## Function `default_fee_info`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_default_fee_info">default_fee_info</a>(): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_default_fee_info">default_fee_info</a>():<a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>{
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a> { mode:2, value:0 }
}
</code></pre>



</details>

<a name="bridge_bridge_fee_new_fee_info"></a>

## Function `new_fee_info`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>(mode: u64, value: u64): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">bridge::bridge_fee::FeeInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_fee_info">new_fee_info</a>(
    mode: u64,
    value: u64
): <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a>{
     <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FeeInfo">FeeInfo</a> { mode, value}
}
</code></pre>



</details>

<a name="bridge_bridge_fee_calculate_fee"></a>

## Function `calculate_fee`



<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_fee">calculate_fee</a>(amount: u64, fee_rate: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_fee">calculate_fee</a>(amount: u64,fee_rate: u64) : u64 {
    <b>assert</b>!(fee_rate &lt; <a href="../bridge/bridge_fee.md#bridge_bridge_fee_FEE_RATE_PRECISION">FEE_RATE_PRECISION</a>, <a href="../bridge/bridge_fee.md#bridge_bridge_fee_EBridgeFeeSettingWrong">EBridgeFeeSettingWrong</a>);
    (((amount <b>as</b> u128)*( fee_rate <b>as</b> u128)) /(<a href="../bridge/bridge_fee.md#bridge_bridge_fee_FEE_RATE_PRECISION">FEE_RATE_PRECISION</a> <b>as</b> u128)) <b>as</b> u64
}
</code></pre>



</details>

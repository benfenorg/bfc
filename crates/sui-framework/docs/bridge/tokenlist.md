---
title: Module `bridge::tokenlist`
---



-  [Struct `TokenInfo`](#bridge_tokenlist_TokenInfo)
-  [Struct `BridgeTokenList`](#bridge_tokenlist_BridgeTokenList)
-  [Constants](#@Constants_0)
-  [Function `new_tokenlist_registry`](#bridge_tokenlist_new_tokenlist_registry)
-  [Function `new`](#bridge_tokenlist_new)
-  [Function `initial_token_list`](#bridge_tokenlist_initial_token_list)
-  [Function `add_center_token_list`](#bridge_tokenlist_add_center_token_list)
-  [Function `borrow`](#bridge_tokenlist_borrow)
-  [Function `borrow_mut`](#bridge_tokenlist_borrow_mut)
-  [Function `empty`](#bridge_tokenlist_empty)
-  [Function `add_token_to_benfen`](#bridge_tokenlist_add_token_to_benfen)
-  [Function `add_token_from_benfen`](#bridge_tokenlist_add_token_from_benfen)
-  [Function `remove_token_to_benfen`](#bridge_tokenlist_remove_token_to_benfen)
-  [Function `remove_token_from_benfen`](#bridge_tokenlist_remove_token_from_benfen)
-  [Function `add_token_info`](#bridge_tokenlist_add_token_info)
-  [Function `remove_token_info`](#bridge_tokenlist_remove_token_info)
-  [Function `get_token_info`](#bridge_tokenlist_get_token_info)
-  [Function `is_supported_to_benfen`](#bridge_tokenlist_is_supported_to_benfen)
-  [Function `is_supported_from_benfen`](#bridge_tokenlist_is_supported_from_benfen)
-  [Function `is_exist_token_info`](#bridge_tokenlist_is_exist_token_info)
-  [Function `is_exist_token_info_internal`](#bridge_tokenlist_is_exist_token_info_internal)
-  [Function `is_supported_to_benfen_internal`](#bridge_tokenlist_is_supported_to_benfen_internal)
-  [Function `is_supported_from_benfen_internal`](#bridge_tokenlist_is_supported_from_benfen_internal)
-  [Function `get_sui_token_id`](#bridge_tokenlist_get_sui_token_id)
-  [Function `get_btc_token_id`](#bridge_tokenlist_get_btc_token_id)
-  [Function `get_eth_token_id`](#bridge_tokenlist_get_eth_token_id)
-  [Function `get_usdc_token_id`](#bridge_tokenlist_get_usdc_token_id)
-  [Function `get_usdt_token_id`](#bridge_tokenlist_get_usdt_token_id)
-  [Function `get_busd_token_id`](#bridge_tokenlist_get_busd_token_id)
-  [Function `get_bnb_token_id`](#bridge_tokenlist_get_bnb_token_id)
-  [Function `get_op_token_id`](#bridge_tokenlist_get_op_token_id)
-  [Function `get_pol_token_id`](#bridge_tokenlist_get_pol_token_id)


<pre><code><b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="bridge_tokenlist_TokenInfo"></a>

## Struct `TokenInfo`

Metadata for a bridged token.


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">TokenInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>chain_id: u64</code>
</dt>
<dd>
 The originating chain's ID.
</dd>
<dt>
<code>token_id: u64</code>
</dt>
<dd>
 The token's unique identifier on the originating chain.
</dd>
<dt>
<code>decimal: u64</code>
</dt>
<dd>
 Number of decimal places used by the token.
</dd>
</dl>


</details>

<a name="bridge_tokenlist_BridgeTokenList"></a>

## Struct `BridgeTokenList`

including those that can be transferred from or to the Benfen chain.


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>from_benfen: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, bool&gt;&gt;</code>
</dt>
<dd>
 Tokens supported for bridging **from** Benfen to external chains.
 Mapping: chain_id => (token_id => true)
</dd>
<dt>
<code>to_benfen: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, bool&gt;&gt;</code>
</dt>
<dd>
 Tokens supported for bridging **to** Benfen from external chains.
 Mapping: chain_id => (token_id => true)
</dd>
<dt>
<code>tokens: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">bridge::tokenlist::TokenInfo</a>&gt;&gt;</code>
</dt>
<dd>
 Metadata for supported tokens.
 Mapping: chain_id => (token_id => TokenInfo)
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_tokenlist_EBridgeCenterTokenLisAlreadyExists"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_EBridgeCenterTokenLisAlreadyExists">EBridgeCenterTokenLisAlreadyExists</a>: u64 = 2;
</code></pre>



<a name="bridge_tokenlist_EBridgeTokenListRegistryAlreadyExists"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_EBridgeTokenListRegistryAlreadyExists">EBridgeTokenListRegistryAlreadyExists</a>: u64 = 1;
</code></pre>



<a name="bridge_tokenlist_EChainIDAndTokenIDNotExist"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_EChainIDAndTokenIDNotExist">EChainIDAndTokenIDNotExist</a>: u64 = 0;
</code></pre>



<a name="bridge_tokenlist_KEY"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_KEY">KEY</a>: vector&lt;u8&gt; = vector[98, 114, 105, 100, 103, 101, 95, 116, 111, 107, 101, 110, 95, 108, 105, 115, 116];
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_APTOS"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_APTOS">TOKEN_ID_APTOS</a>: u64 = 11;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_BNB"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>: u64 = 6;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_BTC"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BTC">TOKEN_ID_BTC</a>: u64 = 1;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_BUSD"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a>: u64 = 5;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_DOGE"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_DOGE">TOKEN_ID_DOGE</a>: u64 = 9;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_ETH"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>: u64 = 2;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_LTC"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_LTC">TOKEN_ID_LTC</a>: u64 = 10;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_OP"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_OP">TOKEN_ID_OP</a>: u64 = 7;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_POL"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_POL">TOKEN_ID_POL</a>: u64 = 8;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_SUI"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_SUI">TOKEN_ID_SUI</a>: u64 = 0;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_USDC"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>: u64 = 3;
</code></pre>



<a name="bridge_tokenlist_TOKEN_ID_USDT"></a>



<pre><code><b>const</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>: u64 = 4;
</code></pre>



<a name="bridge_tokenlist_new_tokenlist_registry"></a>

## Function `new_tokenlist_registry`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_new_tokenlist_registry">new_tokenlist_registry</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_new_tokenlist_registry">new_tokenlist_registry</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext) {
   <b>assert</b>!(
       !dynamic_field::exists_(parent_id, <a href="../bridge/tokenlist.md#bridge_tokenlist_KEY">KEY</a>),
       <a href="../bridge/tokenlist.md#bridge_tokenlist_EBridgeTokenListRegistryAlreadyExists">EBridgeTokenListRegistryAlreadyExists</a> // TODO - add custom error type
   );
   dynamic_field::add(
       parent_id,
       <a href="../bridge/tokenlist.md#bridge_tokenlist_KEY">KEY</a>,
       <a href="../bridge/tokenlist.md#bridge_tokenlist_new">new</a>(ctx),
   );
   <a href="../bridge/tokenlist.md#bridge_tokenlist_initial_token_list">initial_token_list</a>(parent_id,ctx);
}
</code></pre>



</details>

<a name="bridge_tokenlist_new"></a>

## Function `new`

Initializes an empty <code><a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a></code> with all tables.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_new">new</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">bridge::tokenlist::BridgeTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a> {
    <b>let</b> (from_benfen, to_benfen, tokens) = <a href="../bridge/tokenlist.md#bridge_tokenlist_empty">empty</a>(ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a> {
        from_benfen,
        to_benfen,
        tokens,
    }
}
</code></pre>



</details>

<a name="bridge_tokenlist_initial_token_list"></a>

## Function `initial_token_list`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_initial_token_list">initial_token_list</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_initial_token_list">initial_token_list</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext){
    //btc cross in benfen
    //bitcoin
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BTC">TOKEN_ID_BTC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BTC">TOKEN_ID_BTC</a>, ctx);
    //eth cross in benfen
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //bsc cross in  benfen
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //bnb
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>,ctx);
    //base cross in benfen
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //op cross in  benfen
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    // polygon cross in  benfen
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //arbitrum cross in benfen
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //avalanche cross in benfen
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //solana cross in benfen
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
    //cross out  bitcoin
    //bitcoin
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BTC">TOKEN_ID_BTC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BTC">TOKEN_ID_BTC</a>,ctx);
    //cross out eth
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //cross out bsc
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //bnb
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>,ctx);
    //cross out base
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //cross out op
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>,ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //cross out polygon
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //cross out arbitrum
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //cross out avalanche
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>,ctx);
    //eth
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>, ctx);
    //cross out solana
    //usdc
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
    //usdt
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64,<a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
}
</code></pre>



</details>

<a name="bridge_tokenlist_add_center_token_list"></a>

## Function `add_center_token_list`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_center_token_list">add_center_token_list</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_center_token_list">add_center_token_list</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext){
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(<a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">chain_ids::tron_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>)){
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">chain_ids::tron_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_mainnet">chain_ids::doge_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_DOGE">TOKEN_ID_DOGE</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_testnet">chain_ids::doge_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_DOGE">TOKEN_ID_DOGE</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_mainnet">chain_ids::ltc_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_LTC">TOKEN_ID_LTC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_testnet">chain_ids::ltc_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_LTC">TOKEN_ID_LTC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_mainnet">chain_ids::sui_official_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">chain_ids::sui_official_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_mainnet">chain_ids::sui_official_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">chain_ids::sui_official_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_mainnet">chain_ids::aptos_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">chain_ids::aptos_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_mainnet">chain_ids::aptos_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">chain_ids::aptos_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">chain_ids::tron_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_mainnet">chain_ids::doge_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_DOGE">TOKEN_ID_DOGE</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_testnet">chain_ids::doge_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_DOGE">TOKEN_ID_DOGE</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_mainnet">chain_ids::ltc_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_LTC">TOKEN_ID_LTC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_testnet">chain_ids::ltc_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_LTC">TOKEN_ID_LTC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_mainnet">chain_ids::sui_official_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">chain_ids::sui_official_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_mainnet">chain_ids::sui_official_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">chain_ids::sui_official_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_mainnet">chain_ids::aptos_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">chain_ids::aptos_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_mainnet">chain_ids::aptos_mainnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">chain_ids::aptos_testnet</a>() <b>as</b> u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, ctx);
    }<b>else</b>{
        <b>abort</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_EBridgeCenterTokenLisAlreadyExists">EBridgeCenterTokenLisAlreadyExists</a>
    }
}
</code></pre>



</details>

<a name="bridge_tokenlist_borrow"></a>

## Function `borrow`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">bridge::tokenlist::BridgeTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(parent_id: &UID): &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a>{
    dynamic_field::borrow&lt;vector&lt;u8&gt;,<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a>&gt;(parent_id, <a href="../bridge/tokenlist.md#bridge_tokenlist_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_tokenlist_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<b>mut</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">bridge::tokenlist::BridgeTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> UID): &<b>mut</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a>{
    dynamic_field::borrow_mut&lt;vector&lt;u8&gt;,<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a>&gt;(parent_id, <a href="../bridge/tokenlist.md#bridge_tokenlist_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_tokenlist_empty"></a>

## Function `empty`

Internal helper to initialize nested tables.


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_empty">empty</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, bool&gt;&gt;, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, bool&gt;&gt;, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">bridge::tokenlist::TokenInfo</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_empty">empty</a>(
    ctx: &<b>mut</b> TxContext
): (
    Table&lt;u64, Table&lt;u64, bool&gt;&gt;,
    Table&lt;u64, Table&lt;u64, bool&gt;&gt;,
    Table&lt;u64, Table&lt;u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">TokenInfo</a>&gt;&gt;,
) {
    <b>let</b> from_benfen = table::new&lt;u64, Table&lt;u64, bool&gt;&gt;(ctx);
    <b>let</b> to_benfen = table::new&lt;u64, Table&lt;u64, bool&gt;&gt;(ctx);
    <b>let</b> tokens = table::new&lt;u64, Table&lt;u64, <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">TokenInfo</a>&gt;&gt;(ctx);
    (from_benfen, to_benfen, tokens)
}
</code></pre>



</details>

<a name="bridge_tokenlist_add_token_to_benfen"></a>

## Function `add_token_to_benfen`

Adds a token to the list of those supported **to** Benfen.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    <b>if</b> (!self.to_benfen.contains(chain_id)) {
        self.to_benfen.add(chain_id, table::new(ctx));
    };
    self.to_benfen.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <b>true</b>);
}
</code></pre>



</details>

<a name="bridge_tokenlist_add_token_from_benfen"></a>

## Function `add_token_from_benfen`

Adds a token to the list of those supported **from** Benfen.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    <b>if</b> (!self.from_benfen.contains(chain_id)) {
        self.from_benfen.add(chain_id, table::new(ctx));
    };
    self.from_benfen.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <b>true</b>);
}
</code></pre>



</details>

<a name="bridge_tokenlist_remove_token_to_benfen"></a>

## Function `remove_token_to_benfen`

Removes a token from the **to Benfen** supported list.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_to_benfen">remove_token_to_benfen</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_to_benfen">remove_token_to_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64
) {
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    self.to_benfen.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(chain_id).remove(token_id);
}
</code></pre>



</details>

<a name="bridge_tokenlist_remove_token_from_benfen"></a>

## Function `remove_token_from_benfen`

Removes a token from the **from Benfen** supported list.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_from_benfen">remove_token_from_benfen</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_from_benfen">remove_token_from_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64
) {
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    self.from_benfen.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(chain_id).remove(token_id);
}
</code></pre>



</details>

<a name="bridge_tokenlist_add_token_info"></a>

## Function `add_token_info`

Adds token metadata if not already present.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_info">add_token_info</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, decimal: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_info">add_token_info</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    decimal: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.tokens.contains(chain_id)) {
        self.tokens.add(chain_id, table::new(ctx));
    };
    <b>if</b> (!self.tokens.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(chain_id).contains(token_id)) {
        <b>let</b> info = <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">TokenInfo</a> {
            chain_id,
            token_id,
            decimal,
        };
        self.tokens.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(chain_id).add(token_id, info);
    };
}
</code></pre>



</details>

<a name="bridge_tokenlist_remove_token_info"></a>

## Function `remove_token_info`

Removes token metadata if exists.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_info">remove_token_info</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_info">remove_token_info</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64
) {
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    self.tokens.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow_mut">borrow_mut</a>(chain_id).remove(token_id);
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_token_info"></a>

## Function `get_token_info`

Fetches metadata of a specific token. Panics if not found.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_token_info">get_token_info</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">bridge::tokenlist::TokenInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_token_info">get_token_info</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64
): <a href="../bridge/tokenlist.md#bridge_tokenlist_TokenInfo">TokenInfo</a> {
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(parent_id);
    <b>assert</b>!(self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(chain_id, token_id), <a href="../bridge/tokenlist.md#bridge_tokenlist_EChainIDAndTokenIDNotExist">EChainIDAndTokenIDNotExist</a>);
    *self.tokens.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(chain_id).<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

<a name="bridge_tokenlist_is_supported_to_benfen"></a>

## Function `is_supported_to_benfen`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen">is_supported_to_benfen</a>(parnet_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen">is_supported_to_benfen</a>(
    parnet_id: &UID,
    chain_id: u64,
    token_id: u64
):bool{
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(parnet_id);
    self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(chain_id,token_id)
}
</code></pre>



</details>

<a name="bridge_tokenlist_is_supported_from_benfen"></a>

## Function `is_supported_from_benfen`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen">is_supported_from_benfen</a>(parnet_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen">is_supported_from_benfen</a>(
   parnet_id: &UID,
   chain_id: u64,
   token_id: u64
):bool{
   <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(parnet_id);
   self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(chain_id,token_id)
}
</code></pre>



</details>

<a name="bridge_tokenlist_is_exist_token_info"></a>

## Function `is_exist_token_info`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_exist_token_info">is_exist_token_info</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_exist_token_info">is_exist_token_info</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64
):bool{
    <b>let</b> self=<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(parent_id);
    self.<a href="../bridge/tokenlist.md#bridge_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(chain_id,token_id)
}
</code></pre>



</details>

<a name="bridge_tokenlist_is_exist_token_info_internal"></a>

## Function `is_exist_token_info_internal`

Returns whether token metadata exists.


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(self: &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">bridge::tokenlist::BridgeTokenList</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(
    self: &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a>,
    chain_id: u64,
    token_id: u64
): bool {
    <b>if</b> (!self.tokens.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    self.tokens.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(chain_id).contains(token_id)
}
</code></pre>



</details>

<a name="bridge_tokenlist_is_supported_to_benfen_internal"></a>

## Function `is_supported_to_benfen_internal`

Checks whether a token is supported for bridging **into** Benfen.


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(self: &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">bridge::tokenlist::BridgeTokenList</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(
    self: &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a>,
    chain_id: u64,
    token_id: u64
): bool {
    <b>if</b> (!self.to_benfen.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> inner = self.to_benfen.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> <b>false</b>
    };
    *inner.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

<a name="bridge_tokenlist_is_supported_from_benfen_internal"></a>

## Function `is_supported_from_benfen_internal`

Checks whether a token is supported for bridging **from** Benfen.


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(self: &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">bridge::tokenlist::BridgeTokenList</a>, chain_id: u64, token_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(
    self: &<a href="../bridge/tokenlist.md#bridge_tokenlist_BridgeTokenList">BridgeTokenList</a>,
    chain_id: u64,
    token_id: u64
): bool {
    <b>if</b> (!self.from_benfen.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> inner = self.from_benfen.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> <b>false</b>
    };
    *inner.<a href="../bridge/tokenlist.md#bridge_tokenlist_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_sui_token_id"></a>

## Function `get_sui_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_sui_token_id">get_sui_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_sui_token_id">get_sui_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_SUI">TOKEN_ID_SUI</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_btc_token_id"></a>

## Function `get_btc_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_btc_token_id">get_btc_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_btc_token_id">get_btc_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BTC">TOKEN_ID_BTC</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_eth_token_id"></a>

## Function `get_eth_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_eth_token_id">get_eth_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_eth_token_id">get_eth_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_ETH">TOKEN_ID_ETH</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_usdc_token_id"></a>

## Function `get_usdc_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">get_usdc_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">get_usdc_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDC">TOKEN_ID_USDC</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_usdt_token_id"></a>

## Function `get_usdt_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">get_usdt_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">get_usdt_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_USDT">TOKEN_ID_USDT</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_busd_token_id"></a>

## Function `get_busd_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_busd_token_id">get_busd_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_busd_token_id">get_busd_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_bnb_token_id"></a>

## Function `get_bnb_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_bnb_token_id">get_bnb_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_bnb_token_id">get_bnb_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_BNB">TOKEN_ID_BNB</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_op_token_id"></a>

## Function `get_op_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_op_token_id">get_op_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_op_token_id">get_op_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_OP">TOKEN_ID_OP</a>
}
</code></pre>



</details>

<a name="bridge_tokenlist_get_pol_token_id"></a>

## Function `get_pol_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_pol_token_id">get_pol_token_id</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/tokenlist.md#bridge_tokenlist_get_pol_token_id">get_pol_token_id</a>(): u64{
    <a href="../bridge/tokenlist.md#bridge_tokenlist_TOKEN_ID_POL">TOKEN_ID_POL</a>
}
</code></pre>



</details>

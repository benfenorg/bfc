---
title: Module `0xb::tokenlist`
---



-  [Struct `TokenInfo`](#0xb_tokenlist_TokenInfo)
-  [Struct `BridgeTokenList`](#0xb_tokenlist_BridgeTokenList)
-  [Constants](#@Constants_0)
-  [Function `new_tokenlist_registry`](#0xb_tokenlist_new_tokenlist_registry)
-  [Function `new`](#0xb_tokenlist_new)
-  [Function `initial_token_list`](#0xb_tokenlist_initial_token_list)
-  [Function `borrow`](#0xb_tokenlist_borrow)
-  [Function `borrow_mut`](#0xb_tokenlist_borrow_mut)
-  [Function `empty`](#0xb_tokenlist_empty)
-  [Function `add_token_to_benfen`](#0xb_tokenlist_add_token_to_benfen)
-  [Function `add_token_from_benfen`](#0xb_tokenlist_add_token_from_benfen)
-  [Function `remove_token_to_benfen`](#0xb_tokenlist_remove_token_to_benfen)
-  [Function `remove_token_from_benfen`](#0xb_tokenlist_remove_token_from_benfen)
-  [Function `add_token_info`](#0xb_tokenlist_add_token_info)
-  [Function `remove_token_info`](#0xb_tokenlist_remove_token_info)
-  [Function `get_token_info`](#0xb_tokenlist_get_token_info)
-  [Function `is_supported_to_benfen`](#0xb_tokenlist_is_supported_to_benfen)
-  [Function `is_supported_from_benfen`](#0xb_tokenlist_is_supported_from_benfen)
-  [Function `is_exist_token_info`](#0xb_tokenlist_is_exist_token_info)
-  [Function `is_exist_token_info_internal`](#0xb_tokenlist_is_exist_token_info_internal)
-  [Function `is_supported_to_benfen_internal`](#0xb_tokenlist_is_supported_to_benfen_internal)
-  [Function `is_supported_from_benfen_internal`](#0xb_tokenlist_is_supported_from_benfen_internal)


<pre><code><b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="chain_ids.md#0xb_chain_ids">0xb::chain_ids</a>;
</code></pre>



<a name="0xb_tokenlist_TokenInfo"></a>

## Struct `TokenInfo`

Metadata for a bridged token.


<pre><code><b>struct</b> <a href="tokenlist.md#0xb_tokenlist_TokenInfo">TokenInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The originating chain's ID.
</dd>
<dt>
<code>token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The token's unique identifier on the originating chain.
</dd>
<dt>
<code>decimal: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Number of decimal places used by the token.
</dd>
</dl>


</details>

<a name="0xb_tokenlist_BridgeTokenList"></a>

## Struct `BridgeTokenList`

including those that can be transferred from or to the Benfen chain.


<pre><code><b>struct</b> <a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>from_benfen: <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;</code>
</dt>
<dd>
 Tokens supported for bridging **from** Benfen to external chains.
 Mapping: chain_id => (token_id => true)
</dd>
<dt>
<code>to_benfen: <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;</code>
</dt>
<dd>
 Tokens supported for bridging **to** Benfen from external chains.
 Mapping: chain_id => (token_id => true)
</dd>
<dt>
<code>tokens: <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="tokenlist.md#0xb_tokenlist_TokenInfo">tokenlist::TokenInfo</a>&gt;&gt;</code>
</dt>
<dd>
 Metadata for supported tokens.
 Mapping: chain_id => (token_id => TokenInfo)
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xb_tokenlist_EBridgeTokenListRegistryAlreadyExists"></a>



<pre><code><b>const</b> <a href="tokenlist.md#0xb_tokenlist_EBridgeTokenListRegistryAlreadyExists">EBridgeTokenListRegistryAlreadyExists</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xb_tokenlist_EChainIDAndTokenIDNotExist"></a>



<pre><code><b>const</b> <a href="tokenlist.md#0xb_tokenlist_EChainIDAndTokenIDNotExist">EChainIDAndTokenIDNotExist</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xb_tokenlist_KEY"></a>



<pre><code><b>const</b> <a href="tokenlist.md#0xb_tokenlist_KEY">KEY</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [98, 114, 105, 100, 103, 101, 95, 116, 111, 107, 101, 110, 95, 108, 105, 115, 116];
</code></pre>



<a name="0xb_tokenlist_new_tokenlist_registry"></a>

## Function `new_tokenlist_registry`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_new_tokenlist_registry">new_tokenlist_registry</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_new_tokenlist_registry">new_tokenlist_registry</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext) {
   <b>assert</b>!(
       !<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(parent_id, <a href="tokenlist.md#0xb_tokenlist_KEY">KEY</a>),
       <a href="tokenlist.md#0xb_tokenlist_EBridgeTokenListRegistryAlreadyExists">EBridgeTokenListRegistryAlreadyExists</a> // TODO - add custom error type
   );
   <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(
       parent_id,
       <a href="tokenlist.md#0xb_tokenlist_KEY">KEY</a>,
       <a href="tokenlist.md#0xb_tokenlist_new">new</a>(ctx),
   );
   <a href="tokenlist.md#0xb_tokenlist_initial_token_list">initial_token_list</a>(parent_id,ctx);
}
</code></pre>



</details>

<a name="0xb_tokenlist_new"></a>

## Function `new`

Initializes an empty <code><a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a></code> with all tables.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_new">new</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">tokenlist::BridgeTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a> {
    <b>let</b> (from_benfen, to_benfen, tokens) = <a href="tokenlist.md#0xb_tokenlist_empty">empty</a>(ctx);
    <a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a> {
        from_benfen,
        to_benfen,
        tokens,
    }
}
</code></pre>



</details>

<a name="0xb_tokenlist_initial_token_list"></a>

## Function `initial_token_list`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_initial_token_list">initial_token_list</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_initial_token_list">initial_token_list</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext){
    //btc cross in benfen
    //bitcoin
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,1,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,1,ctx);

    //eth cross in benfen
    //eth
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);

    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);



    //bsc cross in  benfen
    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);

    //bnb
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,6,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,6,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,6,ctx);


    //base cross in benfen
    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);

    //eth
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);



    //op cross in  benfen
    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);

    //eth
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);

    //op
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,7,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,7,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,7,ctx);


    //cross out  bitcoin
    //bitcoin
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,1,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,1,ctx);


    //cross out eth
    //eth
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);

    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_eth_custom">chain_ids::eth_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);


    //cross out bsc
    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);

    //bnb
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,6,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,6,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_bsc_custom">chain_ids::bsc_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,6,ctx);


    //cross out base
    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);

    //eth
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_base_custom">chain_ids::base_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);


    //cross out op
    //usdc
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,3,ctx);

    //usdt
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,4,ctx);

    //eth
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,2,ctx);

    //op
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,7,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,7,ctx);
    <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id,<a href="chain_ids.md#0xb_chain_ids_op_custom">chain_ids::op_custom</a>() <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,7,ctx);

}
</code></pre>



</details>

<a name="0xb_tokenlist_borrow"></a>

## Function `borrow`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(parent_id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>): &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">tokenlist::BridgeTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(parent_id: &UID): &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a>{
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a>&gt;(parent_id, <a href="tokenlist.md#0xb_tokenlist_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="0xb_tokenlist_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>): &<b>mut</b> <a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">tokenlist::BridgeTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> UID): &<b>mut</b> <a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a>{
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a>&gt;(parent_id, <a href="tokenlist.md#0xb_tokenlist_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="0xb_tokenlist_empty"></a>

## Function `empty`

Internal helper to initialize nested tables.


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_empty">empty</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="tokenlist.md#0xb_tokenlist_TokenInfo">tokenlist::TokenInfo</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_empty">empty</a>(
    ctx: &<b>mut</b> TxContext
): (
    Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;,
    Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;,
    Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="tokenlist.md#0xb_tokenlist_TokenInfo">TokenInfo</a>&gt;&gt;,
) {
    <b>let</b> from_benfen = <a href="../sui-framework/table.md#0x2_table_new">table::new</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;(ctx);
    <b>let</b> to_benfen = <a href="../sui-framework/table.md#0x2_table_new">table::new</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, bool&gt;&gt;(ctx);
    <b>let</b> tokens = <a href="../sui-framework/table.md#0x2_table_new">table::new</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="tokenlist.md#0xb_tokenlist_TokenInfo">TokenInfo</a>&gt;&gt;(ctx);
    (from_benfen, to_benfen, tokens)
}
</code></pre>



</details>

<a name="0xb_tokenlist_add_token_to_benfen"></a>

## Function `add_token_to_benfen`

Adds a token to the list of those supported **to** Benfen.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_add_token_to_benfen">add_token_to_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (self.<a href="tokenlist.md#0xb_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    <b>if</b> (!self.to_benfen.contains(chain_id)) {
        self.to_benfen.add(chain_id, <a href="../sui-framework/table.md#0x2_table_new">table::new</a>(ctx));
    };
    self.to_benfen.<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <b>true</b>);
}
</code></pre>



</details>

<a name="0xb_tokenlist_add_token_from_benfen"></a>

## Function `add_token_from_benfen`

Adds a token to the list of those supported **from** Benfen.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_add_token_from_benfen">add_token_from_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (self.<a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    <b>if</b> (!self.from_benfen.contains(chain_id)) {
        self.from_benfen.add(chain_id, <a href="../sui-framework/table.md#0x2_table_new">table::new</a>(ctx));
    };
    self.from_benfen.<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <b>true</b>);
}
</code></pre>



</details>

<a name="0xb_tokenlist_remove_token_to_benfen"></a>

## Function `remove_token_to_benfen`

Removes a token from the **to Benfen** supported list.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_remove_token_to_benfen">remove_token_to_benfen</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_remove_token_to_benfen">remove_token_to_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
) {
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="tokenlist.md#0xb_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    self.to_benfen.<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(chain_id).remove(token_id);
}
</code></pre>



</details>

<a name="0xb_tokenlist_remove_token_from_benfen"></a>

## Function `remove_token_from_benfen`

Removes a token from the **from Benfen** supported list.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_remove_token_from_benfen">remove_token_from_benfen</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_remove_token_from_benfen">remove_token_from_benfen</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
) {
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    self.from_benfen.<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(chain_id).remove(token_id);
}
</code></pre>



</details>

<a name="0xb_tokenlist_add_token_info"></a>

## Function `add_token_info`

Adds token metadata if not already present.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_add_token_info">add_token_info</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, decimal: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_add_token_info">add_token_info</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    decimal: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.tokens.contains(chain_id)) {
        self.tokens.add(chain_id, <a href="../sui-framework/table.md#0x2_table_new">table::new</a>(ctx));
    };
    <b>if</b> (!self.tokens.<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(chain_id).contains(token_id)) {
        <b>let</b> info = <a href="tokenlist.md#0xb_tokenlist_TokenInfo">TokenInfo</a> {
            chain_id,
            token_id,
            decimal,
        };
        self.tokens.<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(chain_id).add(token_id, info);
    };
}
</code></pre>



</details>

<a name="0xb_tokenlist_remove_token_info"></a>

## Function `remove_token_info`

Removes token metadata if exists.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_remove_token_info">remove_token_info</a>(parent_id: &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<a href="../sui-framework/package.md#0x2_package">package</a>) <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_remove_token_info">remove_token_info</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
) {
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.<a href="tokenlist.md#0xb_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(chain_id, token_id)) {
        <b>return</b>
    };
    self.tokens.<a href="tokenlist.md#0xb_tokenlist_borrow_mut">borrow_mut</a>(chain_id).remove(token_id);
}
</code></pre>



</details>

<a name="0xb_tokenlist_get_token_info"></a>

## Function `get_token_info`

Fetches metadata of a specific token. Panics if not found.


<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_get_token_info">get_token_info</a>(parent_id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="tokenlist.md#0xb_tokenlist_TokenInfo">tokenlist::TokenInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_get_token_info">get_token_info</a>(
    parent_id: &UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): <a href="tokenlist.md#0xb_tokenlist_TokenInfo">TokenInfo</a> {
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(parent_id);
    <b>assert</b>!(self.<a href="tokenlist.md#0xb_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(chain_id, token_id), <a href="tokenlist.md#0xb_tokenlist_EChainIDAndTokenIDNotExist">EChainIDAndTokenIDNotExist</a>);
    *self.tokens.<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(chain_id).<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

<a name="0xb_tokenlist_is_supported_to_benfen"></a>

## Function `is_supported_to_benfen`



<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_to_benfen">is_supported_to_benfen</a>(parnet_id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_to_benfen">is_supported_to_benfen</a>(
    parnet_id: &UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
):bool{
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(parnet_id);
    self.<a href="tokenlist.md#0xb_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(chain_id,token_id)

}
</code></pre>



</details>

<a name="0xb_tokenlist_is_supported_from_benfen"></a>

## Function `is_supported_from_benfen`



<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen">is_supported_from_benfen</a>(parnet_id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen">is_supported_from_benfen</a>(
   parnet_id: &UID,
   chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
   token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
):bool{
   <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(parnet_id);
   self.<a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(chain_id,token_id)

}
</code></pre>



</details>

<a name="0xb_tokenlist_is_exist_token_info"></a>

## Function `is_exist_token_info`



<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_exist_token_info">is_exist_token_info</a>(parent_id: &<a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_exist_token_info">is_exist_token_info</a>(
    parent_id: &UID,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
):bool{
    <b>let</b> self=<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(parent_id);
    self.<a href="tokenlist.md#0xb_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(chain_id,token_id)
}
</code></pre>



</details>

<a name="0xb_tokenlist_is_exist_token_info_internal"></a>

## Function `is_exist_token_info_internal`

Returns whether token metadata exists.


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(self: &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">tokenlist::BridgeTokenList</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_exist_token_info_internal">is_exist_token_info_internal</a>(
    self: &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a>,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): bool {
    <b>if</b> (!self.tokens.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    self.tokens.<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(chain_id).contains(token_id)
}
</code></pre>



</details>

<a name="0xb_tokenlist_is_supported_to_benfen_internal"></a>

## Function `is_supported_to_benfen_internal`

Checks whether a token is supported for bridging **into** Benfen.


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(self: &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">tokenlist::BridgeTokenList</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_to_benfen_internal">is_supported_to_benfen_internal</a>(
    self: &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a>,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): bool {
    <b>if</b> (!self.to_benfen.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> inner = self.to_benfen.<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> <b>false</b>
    };
    *inner.<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

<a name="0xb_tokenlist_is_supported_from_benfen_internal"></a>

## Function `is_supported_from_benfen_internal`

Checks whether a token is supported for bridging **from** Benfen.


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(self: &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">tokenlist::BridgeTokenList</a>, chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen_internal">is_supported_from_benfen_internal</a>(
    self: &<a href="tokenlist.md#0xb_tokenlist_BridgeTokenList">BridgeTokenList</a>,
    chain_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): bool {
    <b>if</b> (!self.from_benfen.contains(chain_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> inner = self.from_benfen.<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> <b>false</b>
    };
    *inner.<a href="tokenlist.md#0xb_tokenlist_borrow">borrow</a>(token_id)
}
</code></pre>



</details>

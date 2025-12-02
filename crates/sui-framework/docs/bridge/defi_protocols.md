---
title: Module `bridge::defi_protocols`
---



-  [Struct `DefiProtocolConfig`](#bridge_defi_protocols_DefiProtocolConfig)
-  [Struct `DefiProtocolKey`](#bridge_defi_protocols_DefiProtocolKey)
-  [Struct `DefiProtocolInfo`](#bridge_defi_protocols_DefiProtocolInfo)
-  [Struct `DefiProtocolEvent`](#bridge_defi_protocols_DefiProtocolEvent)
-  [Constants](#@Constants_0)
-  [Function `borrow`](#bridge_defi_protocols_borrow)
-  [Function `borrow_mut`](#bridge_defi_protocols_borrow_mut)
-  [Function `registry`](#bridge_defi_protocols_registry)
-  [Function `initial_defi_protocol`](#bridge_defi_protocols_initial_defi_protocol)
-  [Function `add_defi_protocol`](#bridge_defi_protocols_add_defi_protocol)
-  [Function `delete_defi_protocol`](#bridge_defi_protocols_delete_defi_protocol)
-  [Function `manage_fee`](#bridge_defi_protocols_manage_fee)
-  [Function `manage_fee_v2`](#bridge_defi_protocols_manage_fee_v2)
-  [Function `calculate_withdraw_principal_amount`](#bridge_defi_protocols_calculate_withdraw_principal_amount)
-  [Function `new`](#bridge_defi_protocols_new)
-  [Function `get_protocol_info`](#bridge_defi_protocols_get_protocol_info)
-  [Function `is_valid_protocol`](#bridge_defi_protocols_is_valid_protocol)
-  [Function `chain_id`](#bridge_defi_protocols_chain_id)
-  [Function `protocol_type`](#bridge_defi_protocols_protocol_type)
-  [Function `protocol_version`](#bridge_defi_protocols_protocol_version)
-  [Function `protocol_token_id`](#bridge_defi_protocols_protocol_token_id)
-  [Function `fee_type`](#bridge_defi_protocols_fee_type)
-  [Function `fee_rate`](#bridge_defi_protocols_fee_rate)
-  [Function `limit_stake_amount`](#bridge_defi_protocols_limit_stake_amount)
-  [Function `limit_unstake_amount`](#bridge_defi_protocols_limit_unstake_amount)


<pre><code><b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="bridge_defi_protocols_DefiProtocolConfig"></a>

## Struct `DefiProtocolConfig`

token id 映射表


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">DefiProtocolConfig</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>protocol_info_map: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">bridge::defi_protocols::DefiProtocolKey</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_defi_protocols_DefiProtocolKey"></a>

## Struct `DefiProtocolKey`

协议信息，存储每个协议的信息


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64</code>
</dt>
<dd>
 协议类型
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64</code>
</dt>
<dd>
 协议版本
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64</code>
</dt>
<dd>
 token id
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8</code>
</dt>
<dd>
 chain id
</dd>
</dl>


</details>

<a name="bridge_defi_protocols_DefiProtocolInfo"></a>

## Struct `DefiProtocolInfo`

协议信息，存储每个协议的信息


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64</code>
</dt>
<dd>
 协议类型
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64</code>
</dt>
<dd>
 协议版本
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64</code>
</dt>
<dd>
 token id
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8</code>
</dt>
<dd>
 chain id
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>: u8</code>
</dt>
<dd>
 fee type, 0: fixed, 1: percentage
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>: u64</code>
</dt>
<dd>
 fee rate,decimal precision is 1e9
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>: u64</code>
</dt>
<dd>
 质押时，单笔最大金额限制
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a>: u64</code>
</dt>
<dd>
 赎回时，单笔最大金额限制
</dd>
</dl>


</details>

<a name="bridge_defi_protocols_DefiProtocolEvent"></a>

## Struct `DefiProtocolEvent`

用户限额使用事件


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolEvent">DefiProtocolEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64</code>
</dt>
<dd>
 协议类型
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64</code>
</dt>
<dd>
 协议版本
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64</code>
</dt>
<dd>
 token id
</dd>
<dt>
<code><a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8</code>
</dt>
<dd>
 chain id
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_defi_protocols_KEY"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_KEY">KEY</a>: vector&lt;u8&gt; = vector[100, 101, 102, 105, 95, 112, 114, 111, 116, 111, 99, 111, 108, 115];
</code></pre>



<a name="bridge_defi_protocols_EDefiProtocolConfigRegistryAlreadyExists"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigRegistryAlreadyExists">EDefiProtocolConfigRegistryAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="bridge_defi_protocols_EDefiProtocolConfigNotFound"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigNotFound">EDefiProtocolConfigNotFound</a>: u64 = 1;
</code></pre>



<a name="bridge_defi_protocols_FEE_TYPE_FIXED"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_FIXED">FEE_TYPE_FIXED</a>: u8 = 1;
</code></pre>



<a name="bridge_defi_protocols_FEE_TYPE_PERCENTAGE"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>: u8 = 0;
</code></pre>



<a name="bridge_defi_protocols_FEE_RATE_15_PERCENTAGE"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>: u64 = 150000000;
</code></pre>



<a name="bridge_defi_protocols_PROTOCOL_TYPE_AAVE"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_AAVE">PROTOCOL_TYPE_AAVE</a>: u64 = 1;
</code></pre>



<a name="bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND">PROTOCOL_TYPE_COMPOUND</a>: u64 = 2;
</code></pre>



<a name="bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC">PROTOCOL_TOKEN_ID_USDC</a>: u64 = 3;
</code></pre>



<a name="bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT">PROTOCOL_TOKEN_ID_USDT</a>: u64 = 4;
</code></pre>



<a name="bridge_defi_protocols_LIMIT_STAKE_AMOUNT"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>: u64 = 2000000000000000;
</code></pre>



<a name="bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT"></a>



<pre><code><b>const</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>: u64 = 2000000000000000;
</code></pre>



<a name="bridge_defi_protocols_borrow"></a>

## Function `borrow`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">bridge::defi_protocols::DefiProtocolConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(parent_id: &UID): &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">DefiProtocolConfig</a>{
    dynamic_field::borrow&lt;vector&lt;u8&gt;,<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">DefiProtocolConfig</a>&gt;(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_defi_protocols_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<b>mut</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">bridge::defi_protocols::DefiProtocolConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> UID): &<b>mut</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">DefiProtocolConfig</a>{
    dynamic_field::borrow_mut&lt;vector&lt;u8&gt;,<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">DefiProtocolConfig</a>&gt;(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_defi_protocols_registry"></a>

## Function `registry`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_registry">registry</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_registry">registry</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(
        !dynamic_field::exists_(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_KEY">KEY</a>),
        <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigRegistryAlreadyExists">EDefiProtocolConfigRegistryAlreadyExists</a>
    );
    dynamic_field::add(
        parent_id,
        <a href="../bridge/defi_protocols.md#bridge_defi_protocols_KEY">KEY</a>,
        <a href="../bridge/defi_protocols.md#bridge_defi_protocols_new">new</a>(ctx),
    );
}
</code></pre>



</details>

<a name="bridge_defi_protocols_initial_defi_protocol"></a>

## Function `initial_defi_protocol`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_initial_defi_protocol">initial_defi_protocol</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_initial_defi_protocol">initial_defi_protocol</a>(parent_id: &<b>mut</b> UID) {
    //aave mainnet
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_AAVE">PROTOCOL_TYPE_AAVE</a>, 3, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC">PROTOCOL_TOKEN_ID_USDC</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">bridge::chain_ids::eth_mainnet</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_AAVE">PROTOCOL_TYPE_AAVE</a>, 3, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT">PROTOCOL_TOKEN_ID_USDT</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">bridge::chain_ids::eth_mainnet</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    //aave sepolia
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_AAVE">PROTOCOL_TYPE_AAVE</a>, 3, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC">PROTOCOL_TOKEN_ID_USDC</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">bridge::chain_ids::eth_sepolia</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_AAVE">PROTOCOL_TYPE_AAVE</a>, 3, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT">PROTOCOL_TOKEN_ID_USDT</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">bridge::chain_ids::eth_sepolia</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    //aave custom
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_AAVE">PROTOCOL_TYPE_AAVE</a>, 3, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC">PROTOCOL_TOKEN_ID_USDC</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">bridge::chain_ids::eth_custom</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_AAVE">PROTOCOL_TYPE_AAVE</a>, 3, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT">PROTOCOL_TOKEN_ID_USDT</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">bridge::chain_ids::eth_custom</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    //compound mainnet
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND">PROTOCOL_TYPE_COMPOUND</a>, 1, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC">PROTOCOL_TOKEN_ID_USDC</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">bridge::chain_ids::eth_mainnet</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND">PROTOCOL_TYPE_COMPOUND</a>, 1, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT">PROTOCOL_TOKEN_ID_USDT</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">bridge::chain_ids::eth_mainnet</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    //compound sepolia
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND">PROTOCOL_TYPE_COMPOUND</a>, 1, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC">PROTOCOL_TOKEN_ID_USDC</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">bridge::chain_ids::eth_sepolia</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND">PROTOCOL_TYPE_COMPOUND</a>, 1, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT">PROTOCOL_TOKEN_ID_USDT</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">bridge::chain_ids::eth_sepolia</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    //compound custom
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND">PROTOCOL_TYPE_COMPOUND</a>, 1, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDC">PROTOCOL_TOKEN_ID_USDC</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">bridge::chain_ids::eth_custom</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TYPE_COMPOUND">PROTOCOL_TYPE_COMPOUND</a>, 1, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_PROTOCOL_TOKEN_ID_USDT">PROTOCOL_TOKEN_ID_USDT</a>, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">bridge::chain_ids::eth_custom</a>(), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_PERCENTAGE">FEE_TYPE_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_RATE_15_PERCENTAGE">FEE_RATE_15_PERCENTAGE</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_STAKE_AMOUNT">LIMIT_STAKE_AMOUNT</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_LIMIT_UNSTAKE_AMOUNT">LIMIT_UNSTAKE_AMOUNT</a>);
}
</code></pre>



</details>

<a name="bridge_defi_protocols_add_defi_protocol"></a>

## Function `add_defi_protocol`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>: u8, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_add_defi_protocol">add_defi_protocol</a>(
    parent_id: &<b>mut</b> UID,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>: u8,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a>: u64,
) {
    <b>let</b> self=<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow_mut">borrow_mut</a>(parent_id);
    <b>let</b> config_key = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> };
    <b>if</b> (!self.protocol_info_map.contains(config_key)) {
        self.protocol_info_map.add(config_key, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a> });
    };
    *self.protocol_info_map.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow_mut">borrow_mut</a>(config_key) = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a> };
    emit(<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolEvent">DefiProtocolEvent</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> });
}
</code></pre>



</details>

<a name="bridge_defi_protocols_delete_defi_protocol"></a>

## Function `delete_defi_protocol`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_delete_defi_protocol">delete_defi_protocol</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_delete_defi_protocol">delete_defi_protocol</a>(
    parent_id: &<b>mut</b> UID,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8,
) {
    <b>let</b> self=<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow_mut">borrow_mut</a>(parent_id);
    <b>let</b> config_key = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> };
    <b>assert</b>!(self.protocol_info_map.contains(config_key), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigNotFound">EDefiProtocolConfigNotFound</a>);
    self.protocol_info_map.remove(config_key);
    emit(<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolEvent">DefiProtocolEvent</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> });
}
</code></pre>



</details>

<a name="bridge_defi_protocols_manage_fee"></a>

## Function `manage_fee`

计算管理费用
返回值：(管理费用, 赎回本金)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_manage_fee">manage_fee</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8, lp_amount_withdraw: u64, amount_withdraw: u64, amount_in_record: u64, lp_amount_in_record: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_manage_fee">manage_fee</a>(
    parent_id: &UID,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8,
    lp_amount_withdraw: u64,
    amount_withdraw: u64,
    amount_in_record: u64,
    lp_amount_in_record: u64,
): (u64, u64) {
    <b>let</b> self=<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(parent_id);
    <b>let</b> config_key = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> };
    <b>assert</b>!(self.protocol_info_map.contains(config_key), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigNotFound">EDefiProtocolConfigNotFound</a>);
    <b>let</b> protocol_info = self.protocol_info_map.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(config_key);
    <b>let</b> lp_amount_withdraw_u256 = lp_amount_withdraw <b>as</b> u256;
    <b>let</b> lp_amount_in_record_u256 = lp_amount_in_record <b>as</b> u256;
    <b>let</b> amount_withdraw_u256 = amount_withdraw <b>as</b> u256;
    <b>let</b> amount_in_record_u256 = amount_in_record <b>as</b> u256;
    <b>let</b> lp_decimal=10000;
    //赎回的LP占比
    <b>let</b> lp_percent=lp_amount_withdraw_u256*lp_decimal/(lp_amount_withdraw_u256+lp_amount_in_record_u256);
    //赎回的本金
    <b>let</b> principal=amount_in_record_u256*lp_percent/lp_decimal;
    //赎回的利息
    <b>let</b> profit = <b>if</b> (amount_withdraw_u256&gt;principal) {
        amount_withdraw_u256-principal
    } <b>else</b> {
        0u256
    };
    //计算管理费用
    <b>let</b> fee: u64 = <b>if</b> (protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a> == <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_FIXED">FEE_TYPE_FIXED</a>) {
        <b>if</b> (profit &gt; (protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a> <b>as</b> u256)) {
            protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>
        } <b>else</b> {
            profit <b>as</b> u64
        }
    } <b>else</b> {
        ((profit * (protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a> <b>as</b> u256)) / 1_000_000_000) <b>as</b> u64
    };
    (fee, principal <b>as</b> u64)
}
</code></pre>



</details>

<a name="bridge_defi_protocols_manage_fee_v2"></a>

## Function `manage_fee_v2`

计算管理费用
返回值：(管理费用, 赎回本金)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_manage_fee_v2">manage_fee_v2</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8, principal_amount: u64, amount_withdraw: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_manage_fee_v2">manage_fee_v2</a>(
    parent_id: &UID,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8,
    principal_amount: u64,
    amount_withdraw: u64,
): u64 {
    <b>let</b> self=<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(parent_id);
    <b>let</b> config_key = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> };
    <b>assert</b>!(self.protocol_info_map.contains(config_key), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigNotFound">EDefiProtocolConfigNotFound</a>);
    <b>let</b> protocol_info = self.protocol_info_map.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(config_key);
    <b>let</b> principal_amount_u256 = principal_amount <b>as</b> u256;
    <b>let</b> amount_withdraw_u256 = amount_withdraw <b>as</b> u256;
    <b>let</b> profit = <b>if</b> (amount_withdraw_u256&gt;principal_amount_u256) {
        amount_withdraw_u256-principal_amount_u256
    } <b>else</b> {
        0u256
    };
    //计算管理费用
    <b>let</b> fee: u64 = <b>if</b> (protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a> == <a href="../bridge/defi_protocols.md#bridge_defi_protocols_FEE_TYPE_FIXED">FEE_TYPE_FIXED</a>) {
        <b>if</b> (profit &gt; (protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a> <b>as</b> u256)) {
            protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>
        } <b>else</b> {
            profit <b>as</b> u64
        }
    } <b>else</b> {
        ((profit * (protocol_info.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a> <b>as</b> u256)) / 1_000_000_000) <b>as</b> u64
    };
    fee
}
</code></pre>



</details>

<a name="bridge_defi_protocols_calculate_withdraw_principal_amount"></a>

## Function `calculate_withdraw_principal_amount`

计算赎回本金
返回值：赎回本金


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_calculate_withdraw_principal_amount">calculate_withdraw_principal_amount</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8, lp_amount_withdraw: u64, lp_amount_in_total: u64, principal_amount_total: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_calculate_withdraw_principal_amount">calculate_withdraw_principal_amount</a>(
    parent_id: &UID,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8,
    lp_amount_withdraw: u64,
    lp_amount_in_total: u64,
    principal_amount_total: u64,
): u64 {
    <b>let</b> self=<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(parent_id);
    <b>let</b> config_key = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> };
    <b>assert</b>!(self.protocol_info_map.contains(config_key), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigNotFound">EDefiProtocolConfigNotFound</a>);
    <b>let</b> lp_amount_withdraw_u256 = lp_amount_withdraw <b>as</b> u256;
    <b>let</b> lp_amount_in_total_u256 = lp_amount_in_total <b>as</b> u256;
    <b>let</b> principal_amount_total_u256 = principal_amount_total <b>as</b> u256;
    <b>let</b> lp_decimal=10000;
    //赎回的LP占比
    <b>let</b> lp_percent=lp_amount_withdraw_u256*lp_decimal/lp_amount_in_total_u256;
    //赎回的本金
    <b>let</b> principal=principal_amount_total_u256*lp_percent/lp_decimal;
    principal <b>as</b> u64
}
</code></pre>



</details>

<a name="bridge_defi_protocols_new"></a>

## Function `new`

创建新的用户限额管理器


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_new">new</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">bridge::defi_protocols::DefiProtocolConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">DefiProtocolConfig</a> {
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolConfig">DefiProtocolConfig</a> {
        protocol_info_map: table::new(ctx),
    }
}
</code></pre>



</details>

<a name="bridge_defi_protocols_get_protocol_info"></a>

## Function `get_protocol_info`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_get_protocol_info">get_protocol_info</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8): <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_get_protocol_info">get_protocol_info</a>(
    parent_id: &<b>mut</b> UID,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8,
): <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a> {
    <b>let</b> self=<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(parent_id);
    <b>let</b> config_key = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> };
    <b>assert</b>!(self.protocol_info_map.contains(config_key), <a href="../bridge/defi_protocols.md#bridge_defi_protocols_EDefiProtocolConfigNotFound">EDefiProtocolConfigNotFound</a>);
    *self.protocol_info_map.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(config_key)
}
</code></pre>



</details>

<a name="bridge_defi_protocols_is_valid_protocol"></a>

## Function `is_valid_protocol`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_is_valid_protocol">is_valid_protocol</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_is_valid_protocol">is_valid_protocol</a>(
    parent_id: &UID,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>: u64,
    <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>: u8,
): bool {
    <b>if</b> (!dynamic_field::exists_(parent_id, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_KEY">KEY</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> self = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_borrow">borrow</a>(parent_id);
    <b>let</b> config_key = <a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolKey">DefiProtocolKey</a> { <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>, <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a> };
    self.protocol_info_map.contains(config_key)
}
</code></pre>



</details>

<a name="bridge_defi_protocols_chain_id"></a>

## Function `chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u8 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_chain_id">chain_id</a>
}
</code></pre>



</details>

<a name="bridge_defi_protocols_protocol_type"></a>

## Function `protocol_type`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u64 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_type">protocol_type</a>
}
</code></pre>



</details>

<a name="bridge_defi_protocols_protocol_version"></a>

## Function `protocol_version`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u64 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_version">protocol_version</a>
}
</code></pre>



</details>

<a name="bridge_defi_protocols_protocol_token_id"></a>

## Function `protocol_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u64 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_protocol_token_id">protocol_token_id</a>
}
</code></pre>



</details>

<a name="bridge_defi_protocols_fee_type"></a>

## Function `fee_type`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u8 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_type">fee_type</a>
}
</code></pre>



</details>

<a name="bridge_defi_protocols_fee_rate"></a>

## Function `fee_rate`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u64 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_fee_rate">fee_rate</a>
}
</code></pre>



</details>

<a name="bridge_defi_protocols_limit_stake_amount"></a>

## Function `limit_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u64 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_stake_amount">limit_stake_amount</a>
}
</code></pre>



</details>

<a name="bridge_defi_protocols_limit_unstake_amount"></a>

## Function `limit_unstake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">bridge::defi_protocols::DefiProtocolInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a>(self: &<a href="../bridge/defi_protocols.md#bridge_defi_protocols_DefiProtocolInfo">DefiProtocolInfo</a>): u64 {
    self.<a href="../bridge/defi_protocols.md#bridge_defi_protocols_limit_unstake_amount">limit_unstake_amount</a>
}
</code></pre>



</details>

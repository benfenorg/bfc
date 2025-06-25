---
title: Module `0xb::message`
---



-  [Struct `BridgeMessage`](#0xb_message_BridgeMessage)
-  [Struct `BitcoinMessage`](#0xb_message_BitcoinMessage)
-  [Struct `BridgeMessageKey`](#0xb_message_BridgeMessageKey)
-  [Struct `RefundMessageKey`](#0xb_message_RefundMessageKey)
-  [Struct `TokenTransferPayload`](#0xb_message_TokenTransferPayload)
-  [Struct `EmergencyOp`](#0xb_message_EmergencyOp)
-  [Struct `Blocklist`](#0xb_message_Blocklist)
-  [Struct `RefundAdmin`](#0xb_message_RefundAdmin)
-  [Struct `UpdateBridgeLimit`](#0xb_message_UpdateBridgeLimit)
-  [Struct `UpdateAssetPrice`](#0xb_message_UpdateAssetPrice)
-  [Struct `AddExternalCoinAdmin`](#0xb_message_AddExternalCoinAdmin)
-  [Struct `RemoveExternalCoinAdmin`](#0xb_message_RemoveExternalCoinAdmin)
-  [Struct `AddExternalCoinWitness`](#0xb_message_AddExternalCoinWitness)
-  [Struct `RemoveExternalCoinWitness`](#0xb_message_RemoveExternalCoinWitness)
-  [Struct `AddExternalCoinTarget`](#0xb_message_AddExternalCoinTarget)
-  [Struct `RemoveExternalCoinTarget`](#0xb_message_RemoveExternalCoinTarget)
-  [Struct `AddTokenOnSui`](#0xb_message_AddTokenOnSui)
-  [Struct `ParsedTokenTransferMessage`](#0xb_message_ParsedTokenTransferMessage)
-  [Struct `AddTokenOnTokenList`](#0xb_message_AddTokenOnTokenList)
-  [Struct `RemoveTokenOnTokenList`](#0xb_message_RemoveTokenOnTokenList)
-  [Constants](#@Constants_0)
-  [Function `extract_token_bridge_payload`](#0xb_message_extract_token_bridge_payload)
-  [Function `extract_add_witness_poyload`](#0xb_message_extract_add_witness_poyload)
-  [Function `extract_remove_witness_poyload`](#0xb_message_extract_remove_witness_poyload)
-  [Function `extract_add_external_target_address_poyload`](#0xb_message_extract_add_external_target_address_poyload)
-  [Function `extract_remove_external_target_address_poyload`](#0xb_message_extract_remove_external_target_address_poyload)
-  [Function `extract_add_token_on_token_list_poyload`](#0xb_message_extract_add_token_on_token_list_poyload)
-  [Function `extract_remove_token_on_token_list_poyload`](#0xb_message_extract_remove_token_on_token_list_poyload)
-  [Function `extract_emergency_op_payload`](#0xb_message_extract_emergency_op_payload)
-  [Function `extract_blocklist_payload`](#0xb_message_extract_blocklist_payload)
-  [Function `extract_refund_admin_payload`](#0xb_message_extract_refund_admin_payload)
-  [Function `extract_update_bridge_limit`](#0xb_message_extract_update_bridge_limit)
-  [Function `extract_add_external_coin_admin`](#0xb_message_extract_add_external_coin_admin)
-  [Function `extract_remove_external_coin_admin`](#0xb_message_extract_remove_external_coin_admin)
-  [Function `extract_update_asset_price`](#0xb_message_extract_update_asset_price)
-  [Function `extract_add_tokens_on_sui`](#0xb_message_extract_add_tokens_on_sui)
-  [Function `serialize_message`](#0xb_message_serialize_message)
-  [Function `serialize_bitcoin_message`](#0xb_message_serialize_bitcoin_message)
-  [Function `create_bitcoin_message`](#0xb_message_create_bitcoin_message)
-  [Function `create_token_bridge_message`](#0xb_message_create_token_bridge_message)
-  [Function `create_emergency_op_message`](#0xb_message_create_emergency_op_message)
-  [Function `create_blocklist_message`](#0xb_message_create_blocklist_message)
-  [Function `create_refund_admin_message`](#0xb_message_create_refund_admin_message)
-  [Function `create_update_bridge_limit_message`](#0xb_message_create_update_bridge_limit_message)
-  [Function `create_update_asset_price_message`](#0xb_message_create_update_asset_price_message)
-  [Function `create_add_external_coin_admin_message`](#0xb_message_create_add_external_coin_admin_message)
-  [Function `create_add_external_coin_witness_message`](#0xb_message_create_add_external_coin_witness_message)
-  [Function `create_remove_external_coin_witness_message`](#0xb_message_create_remove_external_coin_witness_message)
-  [Function `create_add_external_coin_target_message`](#0xb_message_create_add_external_coin_target_message)
-  [Function `create_remove_external_coin_target_message`](#0xb_message_create_remove_external_coin_target_message)
-  [Function `create_remove_external_coin_admin_message`](#0xb_message_create_remove_external_coin_admin_message)
-  [Function `create_add_token_on_token_list`](#0xb_message_create_add_token_on_token_list)
-  [Function `create_remove_token_on_token_list`](#0xb_message_create_remove_token_on_token_list)
-  [Function `create_add_tokens_on_sui_message`](#0xb_message_create_add_tokens_on_sui_message)
-  [Function `create_key`](#0xb_message_create_key)
-  [Function `key`](#0xb_message_key)
-  [Function `key_refund`](#0xb_message_key_refund)
-  [Function `message_version`](#0xb_message_message_version)
-  [Function `message_type`](#0xb_message_message_type)
-  [Function `seq_num`](#0xb_message_seq_num)
-  [Function `source_chain`](#0xb_message_source_chain)
-  [Function `payload`](#0xb_message_payload)
-  [Function `token_sender_address`](#0xb_message_token_sender_address)
-  [Function `token_target_chain`](#0xb_message_token_target_chain)
-  [Function `token_target_address`](#0xb_message_token_target_address)
-  [Function `token_type`](#0xb_message_token_type)
-  [Function `token_amount`](#0xb_message_token_amount)
-  [Function `token_tx_hash`](#0xb_message_token_tx_hash)
-  [Function `token_event_idx`](#0xb_message_token_event_idx)
-  [Function `emergency_op_type`](#0xb_message_emergency_op_type)
-  [Function `blocklist_type`](#0xb_message_blocklist_type)
-  [Function `blocklist_validator_addresses`](#0xb_message_blocklist_validator_addresses)
-  [Function `refund_admin_op_type`](#0xb_message_refund_admin_op_type)
-  [Function `refund_admin_add`](#0xb_message_refund_admin_add)
-  [Function `refund_admin_remove`](#0xb_message_refund_admin_remove)
-  [Function `refund_admin_sui_address`](#0xb_message_refund_admin_sui_address)
-  [Function `update_bridge_limit_payload_sending_chain`](#0xb_message_update_bridge_limit_payload_sending_chain)
-  [Function `update_bridge_limit_payload_receiving_chain`](#0xb_message_update_bridge_limit_payload_receiving_chain)
-  [Function `update_bridge_limit_payload_limit`](#0xb_message_update_bridge_limit_payload_limit)
-  [Function `update_asset_price_payload_token_id`](#0xb_message_update_asset_price_payload_token_id)
-  [Function `update_asset_price_payload_new_price`](#0xb_message_update_asset_price_payload_new_price)
-  [Function `add_external_coin_witness_payload_coin_type`](#0xb_message_add_external_coin_witness_payload_coin_type)
-  [Function `add_external_coin_witness_payload_witness_address`](#0xb_message_add_external_coin_witness_payload_witness_address)
-  [Function `remove_external_coin_witness_payload_coin_type`](#0xb_message_remove_external_coin_witness_payload_coin_type)
-  [Function `remove_external_coin_witness_payload_witness_address`](#0xb_message_remove_external_coin_witness_payload_witness_address)
-  [Function `add_external_coin_target_payload_coin_type`](#0xb_message_add_external_coin_target_payload_coin_type)
-  [Function `add_external_coin_target_payload_target_address`](#0xb_message_add_external_coin_target_payload_target_address)
-  [Function `remove_external_coin_target_payload_coin_type`](#0xb_message_remove_external_coin_target_payload_coin_type)
-  [Function `remove_external_coin_target_payload_target_address`](#0xb_message_remove_external_coin_target_payload_target_address)
-  [Function `add_external_coin_admin_payload_coin_type`](#0xb_message_add_external_coin_admin_payload_coin_type)
-  [Function `add_external_coin_admin_payload_admin_address`](#0xb_message_add_external_coin_admin_payload_admin_address)
-  [Function `remove_external_coin_admin_payload_coin_type`](#0xb_message_remove_external_coin_admin_payload_coin_type)
-  [Function `remove_external_coin_admin_payload_admin_address`](#0xb_message_remove_external_coin_admin_payload_admin_address)
-  [Function `add_token_on_token_list_payload_from_chain_id`](#0xb_message_add_token_on_token_list_payload_from_chain_id)
-  [Function `add_token_on_token_list_payload_to_chain_id`](#0xb_message_add_token_on_token_list_payload_to_chain_id)
-  [Function `add_token_on_token_list_payload_token_id`](#0xb_message_add_token_on_token_list_payload_token_id)
-  [Function `remove_token_on_token_list_payload_from_chain_id`](#0xb_message_remove_token_on_token_list_payload_from_chain_id)
-  [Function `remove_token_on_token_list_payload_to_chain_id`](#0xb_message_remove_token_on_token_list_payload_to_chain_id)
-  [Function `remove_token_on_token_list_payload_token_id`](#0xb_message_remove_token_on_token_list_payload_token_id)
-  [Function `is_native`](#0xb_message_is_native)
-  [Function `token_ids`](#0xb_message_token_ids)
-  [Function `token_type_names`](#0xb_message_token_type_names)
-  [Function `token_prices`](#0xb_message_token_prices)
-  [Function `emergency_op_pause`](#0xb_message_emergency_op_pause)
-  [Function `emergency_op_unpause`](#0xb_message_emergency_op_unpause)
-  [Function `required_voting_power`](#0xb_message_required_voting_power)
-  [Function `to_parsed_token_transfer_message`](#0xb_message_to_parsed_token_transfer_message)
-  [Function `reverse_bytes`](#0xb_message_reverse_bytes)
-  [Function `peel_u64_be`](#0xb_message_peel_u64_be)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/bcs.md#0x2_bcs">0x2::bcs</a>;
<b>use</b> <a href="chain_ids.md#0xb_chain_ids">0xb::chain_ids</a>;
<b>use</b> <a href="message_types.md#0xb_message_types">0xb::message_types</a>;
</code></pre>



<a name="0xb_message_BridgeMessage"></a>

## Struct `BridgeMessage`



<pre><code><b>struct</b> <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>message_version: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>payload: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_BitcoinMessage"></a>

## Struct `BitcoinMessage`



<pre><code><b>struct</b> <a href="message.md#0xb_message_BitcoinMessage">BitcoinMessage</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_BridgeMessageKey"></a>

## Struct `BridgeMessageKey`



<pre><code><b>struct</b> <a href="message.md#0xb_message_BridgeMessageKey">BridgeMessageKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>message_type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_RefundMessageKey"></a>

## Struct `RefundMessageKey`



<pre><code><b>struct</b> <a href="message.md#0xb_message_RefundMessageKey">RefundMessageKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_TokenTransferPayload"></a>

## Struct `TokenTransferPayload`



<pre><code><b>struct</b> <a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>target_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>token_type: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>event_idx: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_EmergencyOp"></a>

## Struct `EmergencyOp`



<pre><code><b>struct</b> <a href="message.md#0xb_message_EmergencyOp">EmergencyOp</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>op_type: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_Blocklist"></a>

## Struct `Blocklist`



<pre><code><b>struct</b> <a href="message.md#0xb_message_Blocklist">Blocklist</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>blocklist_type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>validator_eth_addresses: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_RefundAdmin"></a>

## Struct `RefundAdmin`



<pre><code><b>struct</b> <a href="message.md#0xb_message_RefundAdmin">RefundAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>op_type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>sui_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_UpdateBridgeLimit"></a>

## Struct `UpdateBridgeLimit`



<pre><code><b>struct</b> <a href="message.md#0xb_message_UpdateBridgeLimit">UpdateBridgeLimit</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>receiving_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>sending_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>limit: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_UpdateAssetPrice"></a>

## Struct `UpdateAssetPrice`



<pre><code><b>struct</b> <a href="message.md#0xb_message_UpdateAssetPrice">UpdateAssetPrice</a> <b>has</b> drop
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

<a name="0xb_message_AddExternalCoinAdmin"></a>

## Struct `AddExternalCoinAdmin`



<pre><code><b>struct</b> <a href="message.md#0xb_message_AddExternalCoinAdmin">AddExternalCoinAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>admin_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_RemoveExternalCoinAdmin"></a>

## Struct `RemoveExternalCoinAdmin`



<pre><code><b>struct</b> <a href="message.md#0xb_message_RemoveExternalCoinAdmin">RemoveExternalCoinAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>admin_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_AddExternalCoinWitness"></a>

## Struct `AddExternalCoinWitness`



<pre><code><b>struct</b> <a href="message.md#0xb_message_AddExternalCoinWitness">AddExternalCoinWitness</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>witness_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_RemoveExternalCoinWitness"></a>

## Struct `RemoveExternalCoinWitness`



<pre><code><b>struct</b> <a href="message.md#0xb_message_RemoveExternalCoinWitness">RemoveExternalCoinWitness</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>witness_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_AddExternalCoinTarget"></a>

## Struct `AddExternalCoinTarget`



<pre><code><b>struct</b> <a href="message.md#0xb_message_AddExternalCoinTarget">AddExternalCoinTarget</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>target_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_RemoveExternalCoinTarget"></a>

## Struct `RemoveExternalCoinTarget`



<pre><code><b>struct</b> <a href="message.md#0xb_message_RemoveExternalCoinTarget">RemoveExternalCoinTarget</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>target_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_AddTokenOnSui"></a>

## Struct `AddTokenOnSui`



<pre><code><b>struct</b> <a href="message.md#0xb_message_AddTokenOnSui">AddTokenOnSui</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>native_token: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>token_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>token_type_names: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>token_prices: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_ParsedTokenTransferMessage"></a>

## Struct `ParsedTokenTransferMessage`



<pre><code><b>struct</b> <a href="message.md#0xb_message_ParsedTokenTransferMessage">ParsedTokenTransferMessage</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_version: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>payload: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>parsed_payload: <a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_AddTokenOnTokenList"></a>

## Struct `AddTokenOnTokenList`



<pre><code><b>struct</b> <a href="message.md#0xb_message_AddTokenOnTokenList">AddTokenOnTokenList</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>from_chain_id: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>to_chain_id: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_message_RemoveTokenOnTokenList"></a>

## Struct `RemoveTokenOnTokenList`



<pre><code><b>struct</b> <a href="message.md#0xb_message_RemoveTokenOnTokenList">RemoveTokenOnTokenList</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>from_chain_id: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>to_chain_id: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xb_message_ADD"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_ADD">ADD</a>: u8 = 0;
</code></pre>



<a name="0xb_message_CURRENT_MESSAGE_VERSION"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>: u8 = 1;
</code></pre>



<a name="0xb_message_ECDSA_ADDRESS_LENGTH"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_ECDSA_ADDRESS_LENGTH">ECDSA_ADDRESS_LENGTH</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 20;
</code></pre>



<a name="0xb_message_EEmptyList"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_EEmptyList">EEmptyList</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0xb_message_EInvalidAddressLength"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_EInvalidAddressLength">EInvalidAddressLength</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xb_message_EInvalidEmergencyOpType"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_EInvalidEmergencyOpType">EInvalidEmergencyOpType</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0xb_message_EInvalidMessageType"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_EInvalidMessageType">EInvalidMessageType</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0xb_message_EInvalidOperationType"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_EInvalidOperationType">EInvalidOperationType</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 7;
</code></pre>



<a name="0xb_message_EMustBeTokenMessage"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_EMustBeTokenMessage">EMustBeTokenMessage</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 6;
</code></pre>



<a name="0xb_message_ETrailingBytes"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xb_message_PAUSE"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_PAUSE">PAUSE</a>: u8 = 0;
</code></pre>



<a name="0xb_message_REMOVE"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_REMOVE">REMOVE</a>: u8 = 1;
</code></pre>



<a name="0xb_message_UNPAUSE"></a>



<pre><code><b>const</b> <a href="message.md#0xb_message_UNPAUSE">UNPAUSE</a>: u8 = 1;
</code></pre>



<a name="0xb_message_extract_token_bridge_payload"></a>

## Function `extract_token_bridge_payload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_token_bridge_payload">extract_token_bridge_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_token_bridge_payload">extract_token_bridge_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> sender_address = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8();
    <b>let</b> target_chain = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
    <b>let</b> target_address = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8();
    <b>let</b> token_type = <a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(&<b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>);
    <b>let</b> amount = <a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(&<b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>);
    <b>let</b> tx_hash = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8();
    <b>let</b> event_idx = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(target_chain);
    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);

    <a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a> {
        sender_address,
        target_chain,
        target_address,
        token_type,
        amount,
        tx_hash,
        event_idx
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_add_witness_poyload"></a>

## Function `extract_add_witness_poyload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_witness_poyload">extract_add_witness_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_AddExternalCoinWitness">message::AddExternalCoinWitness</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_witness_poyload">extract_add_witness_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_AddExternalCoinWitness">AddExternalCoinWitness</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> coin_type = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());
    <b>let</b> (<b>mut</b> witness_address, <b>mut</b> i) = (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>[], 0);
    <b>while</b> (i &lt; <a href="message.md#0xb_message_ECDSA_ADDRESS_LENGTH">ECDSA_ADDRESS_LENGTH</a>) {
            witness_address.push_back(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8());
            i = i + 1;
    };
    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);
     <a href="message.md#0xb_message_AddExternalCoinWitness">AddExternalCoinWitness</a> {
        coin_type,
        witness_address
     }
}
</code></pre>



</details>

<a name="0xb_message_extract_remove_witness_poyload"></a>

## Function `extract_remove_witness_poyload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_witness_poyload">extract_remove_witness_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_RemoveExternalCoinWitness">message::RemoveExternalCoinWitness</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_witness_poyload">extract_remove_witness_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_RemoveExternalCoinWitness">RemoveExternalCoinWitness</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> coin_type = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());
    <b>let</b> (<b>mut</b> witness_address, <b>mut</b> i) = (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>[], 0);
     <b>while</b> (i &lt; <a href="message.md#0xb_message_ECDSA_ADDRESS_LENGTH">ECDSA_ADDRESS_LENGTH</a>) {
            witness_address.push_back(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8());
            i = i + 1;
    };
    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);
    <a href="message.md#0xb_message_RemoveExternalCoinWitness">RemoveExternalCoinWitness</a> {
        coin_type,
        witness_address
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_add_external_target_address_poyload"></a>

## Function `extract_add_external_target_address_poyload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_external_target_address_poyload">extract_add_external_target_address_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_AddExternalCoinTarget">message::AddExternalCoinTarget</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_external_target_address_poyload">extract_add_external_target_address_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_AddExternalCoinTarget">AddExternalCoinTarget</a> {
  <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
  <b>let</b> coin_type = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());
  <b>let</b> length=<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();

  <b>let</b> (<b>mut</b> target_address, <b>mut</b> i) = (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>[], 0);
  <b>while</b> (i &lt; length) {
          target_address.push_back(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8());
          i = i + 1;
  };
  <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);

  <b>let</b> target_address = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(target_address);

   <a href="message.md#0xb_message_AddExternalCoinTarget">AddExternalCoinTarget</a> {
      coin_type,
      target_address
   }
}
</code></pre>



</details>

<a name="0xb_message_extract_remove_external_target_address_poyload"></a>

## Function `extract_remove_external_target_address_poyload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_external_target_address_poyload">extract_remove_external_target_address_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_RemoveExternalCoinTarget">message::RemoveExternalCoinTarget</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_external_target_address_poyload">extract_remove_external_target_address_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_RemoveExternalCoinTarget">RemoveExternalCoinTarget</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> coin_type = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());
    <b>let</b> length=<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
    <b>let</b> (<b>mut</b> target_address, <b>mut</b> i) = (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>[], 0);

    <b>while</b> (i &lt; length) {
            target_address.push_back(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8());
            i = i + 1;
    };
    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);
    <b>let</b> target_address = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(target_address);
    <a href="message.md#0xb_message_RemoveExternalCoinTarget">RemoveExternalCoinTarget</a> {
        coin_type,
        target_address
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_add_token_on_token_list_poyload"></a>

## Function `extract_add_token_on_token_list_poyload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_token_on_token_list_poyload">extract_add_token_on_token_list_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_AddTokenOnTokenList">message::AddTokenOnTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_token_on_token_list_poyload">extract_add_token_on_token_list_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_AddTokenOnTokenList">AddTokenOnTokenList</a>{
       <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
       <b>let</b> from_chain_id=<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
       <b>let</b> to_chain_id=<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
       <b>let</b> token_id=<a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(&<b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>);

       <a href="message.md#0xb_message_AddTokenOnTokenList">AddTokenOnTokenList</a>{
        from_chain_id,
        to_chain_id,
        token_id
       }
}
</code></pre>



</details>

<a name="0xb_message_extract_remove_token_on_token_list_poyload"></a>

## Function `extract_remove_token_on_token_list_poyload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_token_on_token_list_poyload">extract_remove_token_on_token_list_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_RemoveTokenOnTokenList">message::RemoveTokenOnTokenList</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_token_on_token_list_poyload">extract_remove_token_on_token_list_poyload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_RemoveTokenOnTokenList">RemoveTokenOnTokenList</a>{
       <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
       <b>let</b> from_chain_id=<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
       <b>let</b> to_chain_id=<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
       <b>let</b> token_id=<a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(&<b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>);

       <a href="message.md#0xb_message_RemoveTokenOnTokenList">RemoveTokenOnTokenList</a>{
        from_chain_id,
        to_chain_id,
        token_id
       }
}
</code></pre>



</details>

<a name="0xb_message_extract_emergency_op_payload"></a>

## Function `extract_emergency_op_payload`

Emergency op payload is just a single byte


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_emergency_op_payload">extract_emergency_op_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_EmergencyOp">message::EmergencyOp</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_emergency_op_payload">extract_emergency_op_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_EmergencyOp">EmergencyOp</a> {
    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.payload.length() == 1, <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);
    <a href="message.md#0xb_message_EmergencyOp">EmergencyOp</a> { op_type: <a href="message.md#0xb_message">message</a>.payload[0] }
}
</code></pre>



</details>

<a name="0xb_message_extract_blocklist_payload"></a>

## Function `extract_blocklist_payload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_blocklist_payload">extract_blocklist_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_Blocklist">message::Blocklist</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_blocklist_payload">extract_blocklist_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_Blocklist">Blocklist</a> {
    // blocklist payload should consist of one byte blocklist type, and list of 20 bytes evm addresses
    // derived from ECDSA <b>public</b> keys
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> blocklist_type = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
    <b>let</b> <b>mut</b> address_count = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();

    <b>assert</b>!(address_count != 0, <a href="message.md#0xb_message_EEmptyList">EEmptyList</a>);

    <b>let</b> <b>mut</b> validator_eth_addresses = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (address_count &gt; 0) {
        <b>let</b> (<b>mut</b> <b>address</b>, <b>mut</b> i) = (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>[], 0);
        <b>while</b> (i &lt; <a href="message.md#0xb_message_ECDSA_ADDRESS_LENGTH">ECDSA_ADDRESS_LENGTH</a>) {
            <b>address</b>.push_back(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8());
            i = i + 1;
        };
        validator_eth_addresses.push_back(<b>address</b>);
        address_count = address_count - 1;
    };

    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);

    <a href="message.md#0xb_message_Blocklist">Blocklist</a> {
        blocklist_type,
        validator_eth_addresses
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_refund_admin_payload"></a>

## Function `extract_refund_admin_payload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_refund_admin_payload">extract_refund_admin_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_RefundAdmin">message::RefundAdmin</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_refund_admin_payload">extract_refund_admin_payload</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_RefundAdmin">RefundAdmin</a> {
    // blocklist payload should consist of one byte blocklist type, and list of 20 bytes evm addresses
    // derived from ECDSA <b>public</b> keys
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> op_type = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
    <b>let</b> sui_address = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());

    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);

    <b>assert</b>!(op_type == <a href="message.md#0xb_message_ADD">ADD</a> || op_type == <a href="message.md#0xb_message_REMOVE">REMOVE</a>, <a href="message.md#0xb_message_EInvalidOperationType">EInvalidOperationType</a>);

    <a href="message.md#0xb_message_RefundAdmin">RefundAdmin</a> {
        op_type,
        sui_address
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_update_bridge_limit"></a>

## Function `extract_update_bridge_limit`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_update_bridge_limit">extract_update_bridge_limit</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_UpdateBridgeLimit">message::UpdateBridgeLimit</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_update_bridge_limit">extract_update_bridge_limit</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_UpdateBridgeLimit">UpdateBridgeLimit</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> sending_chain = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_u8();
    <b>let</b> limit = <a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(&<b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>);

    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(sending_chain);
    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);

    <a href="message.md#0xb_message_UpdateBridgeLimit">UpdateBridgeLimit</a> {
        receiving_chain: <a href="message.md#0xb_message">message</a>.source_chain,
        sending_chain,
        limit
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_add_external_coin_admin"></a>

## Function `extract_add_external_coin_admin`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_external_coin_admin">extract_add_external_coin_admin</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_AddExternalCoinAdmin">message::AddExternalCoinAdmin</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_external_coin_admin">extract_add_external_coin_admin</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_AddExternalCoinAdmin">AddExternalCoinAdmin</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> coin_type = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());
    <b>let</b> admin_address = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());

    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);

    <a href="message.md#0xb_message_AddExternalCoinAdmin">AddExternalCoinAdmin</a> {
        coin_type,
        admin_address
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_remove_external_coin_admin"></a>

## Function `extract_remove_external_coin_admin`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_external_coin_admin">extract_remove_external_coin_admin</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_RemoveExternalCoinAdmin">message::RemoveExternalCoinAdmin</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_remove_external_coin_admin">extract_remove_external_coin_admin</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_RemoveExternalCoinAdmin">RemoveExternalCoinAdmin</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> coin_type = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());
    <b>let</b> admin_address = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u8());
    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);
    <a href="message.md#0xb_message_RemoveExternalCoinAdmin">RemoveExternalCoinAdmin</a> {
        coin_type,
        admin_address
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_update_asset_price"></a>

## Function `extract_update_asset_price`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_update_asset_price">extract_update_asset_price</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_UpdateAssetPrice">message::UpdateAssetPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_update_asset_price">extract_update_asset_price</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_UpdateAssetPrice">UpdateAssetPrice</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> token_id = <a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(&<b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>);
    <b>let</b> new_price = <a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(&<b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>);

    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);

    <a href="message.md#0xb_message_UpdateAssetPrice">UpdateAssetPrice</a> {
        token_id,
        new_price
    }
}
</code></pre>



</details>

<a name="0xb_message_extract_add_tokens_on_sui"></a>

## Function `extract_add_tokens_on_sui`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_tokens_on_sui">extract_add_tokens_on_sui</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_AddTokenOnSui">message::AddTokenOnSui</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_extract_add_tokens_on_sui">extract_add_tokens_on_sui</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_AddTokenOnSui">AddTokenOnSui</a> {
    <b>let</b> <b>mut</b> <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> = bcs::new(<a href="message.md#0xb_message">message</a>.payload);
    <b>let</b> native_token = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_bool();
    <b>let</b> token_ids = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u64();
    <b>let</b> token_type_names_bytes = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_vec_u8();
    <b>let</b> token_prices = <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.peel_vec_u64();

    <b>let</b> <b>mut</b> n = 0;
    <b>let</b> <b>mut</b> token_type_names = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (n &lt; token_type_names_bytes.length()){
        token_type_names.push_back(<a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(*token_type_names_bytes.borrow(n)));
        n = n + 1;
    };
    <b>assert</b>!(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>.into_remainder_bytes().is_empty(), <a href="message.md#0xb_message_ETrailingBytes">ETrailingBytes</a>);
    <a href="message.md#0xb_message_AddTokenOnSui">AddTokenOnSui</a> {
        native_token,
        token_ids,
        token_type_names,
        token_prices
    }
}
</code></pre>



</details>

<a name="0xb_message_serialize_message"></a>

## Function `serialize_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_serialize_message">serialize_message</a>(<a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_serialize_message">serialize_message</a>(<a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type,
        message_version,
        seq_num,
        source_chain,
        payload
    } = <a href="message.md#0xb_message">message</a>;

    <b>let</b> <b>mut</b> <a href="message.md#0xb_message">message</a> = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[
        message_type,
        message_version,
    ];

    // <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> serializes <a href="../move-stdlib/u64.md#0x1_u64">u64</a> <b>as</b> 8 bytes
    <a href="message.md#0xb_message">message</a>.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&seq_num)));
    <a href="message.md#0xb_message">message</a>.push_back(source_chain);
    <a href="message.md#0xb_message">message</a>.append(payload);
    <a href="message.md#0xb_message">message</a>
}
</code></pre>



</details>

<a name="0xb_message_serialize_bitcoin_message"></a>

## Function `serialize_bitcoin_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_serialize_bitcoin_message">serialize_bitcoin_message</a>(<a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BitcoinMessage">message::BitcoinMessage</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_serialize_bitcoin_message">serialize_bitcoin_message</a>(<a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BitcoinMessage">BitcoinMessage</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;{
    <b>let</b> <a href="message.md#0xb_message_BitcoinMessage">BitcoinMessage</a> {
    source_chain,
    source_address,
    target_address,
    amount,
    tx_hash,
    coin_type,
    } = <a href="message.md#0xb_message">message</a>;
     <b>let</b> <b>mut</b> <a href="message.md#0xb_message">message</a>=<a href="../move-stdlib/vector.md#0x1_vector">vector</a>[
        source_chain,
     ];
    <a href="message.md#0xb_message">message</a>.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&amount)));
    <a href="message.md#0xb_message">message</a>.append(source_address);
    <a href="message.md#0xb_message">message</a>.append(target_address);
    <a href="message.md#0xb_message">message</a>.append(tx_hash);
    <a href="message.md#0xb_message">message</a>.append(coin_type);
    <a href="message.md#0xb_message">message</a>
}
</code></pre>



</details>

<a name="0xb_message_create_bitcoin_message"></a>

## Function `create_bitcoin_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_bitcoin_message">create_bitcoin_message</a>(source_chain: u8, source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, coin_type: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="message.md#0xb_message_BitcoinMessage">message::BitcoinMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_bitcoin_message">create_bitcoin_message</a> (
    source_chain: u8,
    source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    coin_type: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
):<a href="message.md#0xb_message_BitcoinMessage">BitcoinMessage</a>{
    <a href="message.md#0xb_message_BitcoinMessage">BitcoinMessage</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
        coin_type,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_token_bridge_message"></a>

## Function `create_token_bridge_message`

Token Transfer Message Format:
[message_type: u8]
[version:u8]
[nonce:u64]
[source_chain: u8]
[sender_address_length:u8]
[sender_address: byte[]]
[target_chain:u8]
[target_address_length:u8]
[target_address: byte[]]
[token_type:u8]
[amount:u64]


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_token_bridge_message">create_token_bridge_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, sender_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, target_chain: u8, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, token_type: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, event_idx: u8): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_token_bridge_message">create_token_bridge_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    sender_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    target_chain: u8,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    token_type: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    event_idx: u8,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(target_chain);

    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[];

    // sender <b>address</b> should be less than 255 bytes so can fit into u8
    payload.push_back((<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&sender_address) <b>as</b> u8));
    payload.append(sender_address);
    payload.push_back(target_chain);
    // target <b>address</b> should be less than 255 bytes so can fit into u8
    payload.push_back((<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&target_address) <b>as</b> u8));
    payload.append(target_address);
    // <a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a> serialzies <a href="../move-stdlib/u64.md#0x1_u64">u64</a> <b>as</b> 8 bytes
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&token_type)));
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&amount)));

    // btc <b>address</b> len is different from eth <b>address</b> len, so we can't <b>assert</b> palyload length
    // <b>assert</b>!(<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&payload) == 71, EInvalidPayloadLength);
    payload.push_back((<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&tx_hash) <b>as</b> u8));
    payload.append(tx_hash);
    payload.push_back(event_idx);
    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_token">message_types::token</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_emergency_op_message"></a>

## Function `create_emergency_op_message`

Emergency Op Message Format:
[message_type: u8]
[version:u8]
[nonce:u64]
[chain_id: u8]
[op_type: u8]


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_emergency_op_message">create_emergency_op_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, op_type: u8): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_emergency_op_message">create_emergency_op_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    op_type: u8,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_emergency_op">message_types::emergency_op</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[op_type],
    }
}
</code></pre>



</details>

<a name="0xb_message_create_blocklist_message"></a>

## Function `create_blocklist_message`

Blocklist Message Format:
[message_type: u8]
[version:u8]
[nonce:u64]
[chain_id: u8]
[blocklist_type: u8]
[validator_length: u8]
[validator_ecdsa_addresses: byte[][]]


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_blocklist_message">create_blocklist_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, blocklist_type: u8, validator_ecdsa_addresses: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_blocklist_message">create_blocklist_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    // 0: block, 1: unblock
    blocklist_type: u8,
    validator_ecdsa_addresses: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);

    <b>let</b> address_length = validator_ecdsa_addresses.length();
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[blocklist_type, (address_length <b>as</b> u8)];
    <b>let</b> <b>mut</b> i = 0;

    <b>while</b> (i &lt; address_length) {
        <b>let</b> <b>address</b> = validator_ecdsa_addresses[i];
        <b>assert</b>!(<b>address</b>.length() == <a href="message.md#0xb_message_ECDSA_ADDRESS_LENGTH">ECDSA_ADDRESS_LENGTH</a>, <a href="message.md#0xb_message_EInvalidAddressLength">EInvalidAddressLength</a>);
        payload.append(<b>address</b>);

        i = i + 1;
    };

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_committee_blocklist">message_types::committee_blocklist</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_refund_admin_message"></a>

## Function `create_refund_admin_message`

Blocklist Message Format:
[message_type: u8]
[version:u8]
[nonce:u64]
[chain_id: u8]
[op_type: u8]
[address_admin: byte[][]]


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_refund_admin_message">create_refund_admin_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, op_type: u8, admin_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_refund_admin_message">create_refund_admin_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    // 0: add, 1: del
    op_type: u8,
    admin_address: String,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&op_type);
    payload.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&admin_address));

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_refund_admin_operate">message_types::refund_admin_operate</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_update_bridge_limit_message"></a>

## Function `create_update_bridge_limit_message`

Update bridge limit Message Format:
[message_type: u8]
[version:u8]
[nonce:u64]
[receiving_chain_id: u8]
[sending_chain_id: u8]
[new_limit: u64]


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_update_bridge_limit_message">create_update_bridge_limit_message</a>(receiving_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, sending_chain: u8, new_limit: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_update_bridge_limit_message">create_update_bridge_limit_message</a>(
    receiving_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    sending_chain: u8,
    new_limit: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(receiving_chain);
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(sending_chain);

    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[sending_chain];
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&new_limit)));

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_update_bridge_limit">message_types::update_bridge_limit</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain: receiving_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_update_asset_price_message"></a>

## Function `create_update_asset_price_message`

Update asset price message
[message_type: u8]
[version:u8]
[nonce:u64]
[chain_id: u8]
[token_id: u64]
[new_price:u64]


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_update_asset_price_message">create_update_asset_price_message</a>(token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, new_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_update_asset_price_message">create_update_asset_price_message</a>(
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    new_price: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);

    <b>let</b> <b>mut</b> payload = <a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&token_id));
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&new_price)));
    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_update_asset_price">message_types::update_asset_price</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_add_external_coin_admin_message"></a>

## Function `create_add_external_coin_admin_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_external_coin_admin_message">create_add_external_coin_admin_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, admin_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_external_coin_admin_message">create_add_external_coin_admin_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    coin_type: String,
    admin_address: String,
)   : <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&coin_type);
    payload.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&admin_address));

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_add_external_coin_admin">message_types::add_external_coin_admin</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_add_external_coin_witness_message"></a>

## Function `create_add_external_coin_witness_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_external_coin_witness_message">create_add_external_coin_witness_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, witness_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_external_coin_witness_message">create_add_external_coin_witness_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    coin_type: String,
    witness_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>{
     <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&coin_type);
    payload.append(witness_address);

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_add_external_coin_witness">message_types::add_external_coin_witness</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }

}
</code></pre>



</details>

<a name="0xb_message_create_remove_external_coin_witness_message"></a>

## Function `create_remove_external_coin_witness_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_external_coin_witness_message">create_remove_external_coin_witness_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, witness_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_external_coin_witness_message">create_remove_external_coin_witness_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    coin_type: String,
    witness_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
):<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>{
     <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&coin_type);
    payload.append(witness_address);

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_remove_external_coin_witness">message_types::remove_external_coin_witness</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }

}
</code></pre>



</details>

<a name="0xb_message_create_add_external_coin_target_message"></a>

## Function `create_add_external_coin_target_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_external_coin_target_message">create_add_external_coin_target_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_external_coin_target_message">create_add_external_coin_target_message</a>(
   source_chain: u8,
   seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
   coin_type: String,
   target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>{
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
   <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&coin_type);
   payload.push_back(target_address.length() <b>as</b> u8);
   payload.append(target_address);

   <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
       message_type: <a href="message_types.md#0xb_message_types_add_external_coin_target">message_types::add_external_coin_target</a>(),
       message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
       seq_num,
       source_chain,
       payload,
   }

}
</code></pre>



</details>

<a name="0xb_message_create_remove_external_coin_target_message"></a>

## Function `create_remove_external_coin_target_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_external_coin_target_message">create_remove_external_coin_target_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_external_coin_target_message">create_remove_external_coin_target_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    coin_type: String,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
):<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>{
     <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&coin_type);
    payload.push_back(target_address.length() <b>as</b> u8);
    payload.append(target_address);

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_remove_external_coin_target">message_types::remove_external_coin_target</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }

}
</code></pre>



</details>

<a name="0xb_message_create_remove_external_coin_admin_message"></a>

## Function `create_remove_external_coin_admin_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_external_coin_admin_message">create_remove_external_coin_admin_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, admin_address: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_external_coin_admin_message">create_remove_external_coin_admin_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    coin_type: String,
    admin_address: String,
)   : <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&coin_type);
    payload.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&admin_address));

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_remove_external_coin_admin">message_types::remove_external_coin_admin</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_add_token_on_token_list"></a>

## Function `create_add_token_on_token_list`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_token_on_token_list">create_add_token_on_token_list</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, from_chain: u8, target_chain: u8, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_token_on_token_list">create_add_token_on_token_list</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    from_chain: u8,
    target_chain: u8,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>{
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(from_chain);
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(target_chain);
    <b>let</b> <b>mut</b> payload = <a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&from_chain));
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&target_chain)));
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&token_id)));

    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_add_token_on_token_list">message_types::add_token_on_token_list</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_remove_token_on_token_list"></a>

## Function `create_remove_token_on_token_list`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_token_on_token_list">create_remove_token_on_token_list</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, from_chain: u8, target_chain: u8, token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_remove_token_on_token_list">create_remove_token_on_token_list</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    from_chain: u8,
    target_chain: u8,
    token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>{
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(from_chain);
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(target_chain);
    <b>let</b> <b>mut</b> payload = <a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&from_chain));
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&target_chain)));
    payload.append(<a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&token_id)));
    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_remove_token_on_token_list">message_types::remove_token_on_token_list</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }

}
</code></pre>



</details>

<a name="0xb_message_create_add_tokens_on_sui_message"></a>

## Function `create_add_tokens_on_sui_message`

Update Sui token message
[message_type:u8]
[version:u8]
[nonce:u64]
[chain_id: u8]
[native_token:bool]
[token_ids:vector<u64>]
[token_type_name:vector<String>]
[token_prices:vector<u64>]


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_tokens_on_sui_message">create_add_tokens_on_sui_message</a>(source_chain: u8, seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, native_token: bool, token_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, type_names: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, token_prices: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;): <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_add_tokens_on_sui_message">create_add_tokens_on_sui_message</a>(
    source_chain: u8,
    seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    native_token: bool,
    token_ids: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    type_names: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    token_prices: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
): <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
    <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">chain_ids::assert_valid_chain_id</a>(source_chain);
    <b>let</b> <b>mut</b> payload = <a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&native_token);
    payload.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&token_ids));
    payload.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&type_names));
    payload.append(<a href="../move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&token_prices));
    <a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a> {
        message_type: <a href="message_types.md#0xb_message_types_add_tokens_on_sui">message_types::add_tokens_on_sui</a>(),
        message_version: <a href="message.md#0xb_message_CURRENT_MESSAGE_VERSION">CURRENT_MESSAGE_VERSION</a>,
        seq_num,
        source_chain,
        payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_create_key"></a>

## Function `create_key`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_key">create_key</a>(source_chain: u8, message_type: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_create_key">create_key</a>(source_chain: u8, message_type: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="message.md#0xb_message_BridgeMessageKey">BridgeMessageKey</a> {
    <a href="message.md#0xb_message_BridgeMessageKey">BridgeMessageKey</a> { source_chain, message_type, bridge_seq_num }
}
</code></pre>



</details>

<a name="0xb_message_key"></a>

## Function `key`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_key">key</a>(self: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_key">key</a>(self: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="message.md#0xb_message_BridgeMessageKey">BridgeMessageKey</a> {
    <a href="message.md#0xb_message_create_key">create_key</a>(self.source_chain, self.message_type, self.seq_num)
}
</code></pre>



</details>

<a name="0xb_message_key_refund"></a>

## Function `key_refund`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_key_refund">key_refund</a>(tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="message.md#0xb_message_RefundMessageKey">message::RefundMessageKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_key_refund">key_refund</a>(tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="message.md#0xb_message_RefundMessageKey">RefundMessageKey</a> {
    <a href="message.md#0xb_message_RefundMessageKey">RefundMessageKey</a> { tx_hash }
}
</code></pre>



</details>

<a name="0xb_message_message_version"></a>

## Function `message_version`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_message_version">message_version</a>(self: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_message_version">message_version</a>(self: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): u8 {
    self.message_version
}
</code></pre>



</details>

<a name="0xb_message_message_type"></a>

## Function `message_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_message_type">message_type</a>(self: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_message_type">message_type</a>(self: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): u8 {
    self.message_type
}
</code></pre>



</details>

<a name="0xb_message_seq_num"></a>

## Function `seq_num`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_seq_num">seq_num</a>(self: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_seq_num">seq_num</a>(self: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.seq_num
}
</code></pre>



</details>

<a name="0xb_message_source_chain"></a>

## Function `source_chain`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_source_chain">source_chain</a>(self: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_source_chain">source_chain</a>(self: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): u8 {
    self.source_chain
}
</code></pre>



</details>

<a name="0xb_message_payload"></a>

## Function `payload`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_payload">payload</a>(self: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_payload">payload</a>(self: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    self.payload
}
</code></pre>



</details>

<a name="0xb_message_token_sender_address"></a>

## Function `token_sender_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_sender_address">token_sender_address</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_sender_address">token_sender_address</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    self.sender_address
}
</code></pre>



</details>

<a name="0xb_message_token_target_chain"></a>

## Function `token_target_chain`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_target_chain">token_target_chain</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_target_chain">token_target_chain</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a>): u8 {
    self.target_chain
}
</code></pre>



</details>

<a name="0xb_message_token_target_address"></a>

## Function `token_target_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_target_address">token_target_address</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_target_address">token_target_address</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    self.target_address
}
</code></pre>



</details>

<a name="0xb_message_token_type"></a>

## Function `token_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_type">token_type</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_type">token_type</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.token_type
}
</code></pre>



</details>

<a name="0xb_message_token_amount"></a>

## Function `token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_amount">token_amount</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_amount">token_amount</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.amount
}
</code></pre>



</details>

<a name="0xb_message_token_tx_hash"></a>

## Function `token_tx_hash`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_tx_hash">token_tx_hash</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_tx_hash">token_tx_hash</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    self.tx_hash
}
</code></pre>



</details>

<a name="0xb_message_token_event_idx"></a>

## Function `token_event_idx`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_event_idx">token_event_idx</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">message::TokenTransferPayload</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_event_idx">token_event_idx</a>(self: &<a href="message.md#0xb_message_TokenTransferPayload">TokenTransferPayload</a>): u8 {
    self.event_idx
}
</code></pre>



</details>

<a name="0xb_message_emergency_op_type"></a>

## Function `emergency_op_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_emergency_op_type">emergency_op_type</a>(self: &<a href="message.md#0xb_message_EmergencyOp">message::EmergencyOp</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_emergency_op_type">emergency_op_type</a>(self: &<a href="message.md#0xb_message_EmergencyOp">EmergencyOp</a>): u8 {
    self.op_type
}
</code></pre>



</details>

<a name="0xb_message_blocklist_type"></a>

## Function `blocklist_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_blocklist_type">blocklist_type</a>(self: &<a href="message.md#0xb_message_Blocklist">message::Blocklist</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_blocklist_type">blocklist_type</a>(self: &<a href="message.md#0xb_message_Blocklist">Blocklist</a>): u8 {
    self.blocklist_type
}
</code></pre>



</details>

<a name="0xb_message_blocklist_validator_addresses"></a>

## Function `blocklist_validator_addresses`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_blocklist_validator_addresses">blocklist_validator_addresses</a>(self: &<a href="message.md#0xb_message_Blocklist">message::Blocklist</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_blocklist_validator_addresses">blocklist_validator_addresses</a>(self: &<a href="message.md#0xb_message_Blocklist">Blocklist</a>): &<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; {
    &self.validator_eth_addresses
}
</code></pre>



</details>

<a name="0xb_message_refund_admin_op_type"></a>

## Function `refund_admin_op_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_op_type">refund_admin_op_type</a>(self: &<a href="message.md#0xb_message_RefundAdmin">message::RefundAdmin</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_op_type">refund_admin_op_type</a>(self: &<a href="message.md#0xb_message_RefundAdmin">RefundAdmin</a>): u8 {
    self.op_type
}
</code></pre>



</details>

<a name="0xb_message_refund_admin_add"></a>

## Function `refund_admin_add`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_add">refund_admin_add</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_add">refund_admin_add</a>(): u8 {
    <a href="message.md#0xb_message_ADD">ADD</a>
}
</code></pre>



</details>

<a name="0xb_message_refund_admin_remove"></a>

## Function `refund_admin_remove`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_remove">refund_admin_remove</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_remove">refund_admin_remove</a>(): u8 {
    <a href="message.md#0xb_message_REMOVE">REMOVE</a>
}
</code></pre>



</details>

<a name="0xb_message_refund_admin_sui_address"></a>

## Function `refund_admin_sui_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_sui_address">refund_admin_sui_address</a>(self: &<a href="message.md#0xb_message_RefundAdmin">message::RefundAdmin</a>): &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_refund_admin_sui_address">refund_admin_sui_address</a>(self: &<a href="message.md#0xb_message_RefundAdmin">RefundAdmin</a>): &String {
    &self.sui_address
}
</code></pre>



</details>

<a name="0xb_message_update_bridge_limit_payload_sending_chain"></a>

## Function `update_bridge_limit_payload_sending_chain`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_bridge_limit_payload_sending_chain">update_bridge_limit_payload_sending_chain</a>(self: &<a href="message.md#0xb_message_UpdateBridgeLimit">message::UpdateBridgeLimit</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_bridge_limit_payload_sending_chain">update_bridge_limit_payload_sending_chain</a>(self: &<a href="message.md#0xb_message_UpdateBridgeLimit">UpdateBridgeLimit</a>): u8 {
    self.sending_chain
}
</code></pre>



</details>

<a name="0xb_message_update_bridge_limit_payload_receiving_chain"></a>

## Function `update_bridge_limit_payload_receiving_chain`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_bridge_limit_payload_receiving_chain">update_bridge_limit_payload_receiving_chain</a>(self: &<a href="message.md#0xb_message_UpdateBridgeLimit">message::UpdateBridgeLimit</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_bridge_limit_payload_receiving_chain">update_bridge_limit_payload_receiving_chain</a>(self: &<a href="message.md#0xb_message_UpdateBridgeLimit">UpdateBridgeLimit</a>): u8 {
    self.receiving_chain
}
</code></pre>



</details>

<a name="0xb_message_update_bridge_limit_payload_limit"></a>

## Function `update_bridge_limit_payload_limit`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_bridge_limit_payload_limit">update_bridge_limit_payload_limit</a>(self: &<a href="message.md#0xb_message_UpdateBridgeLimit">message::UpdateBridgeLimit</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_bridge_limit_payload_limit">update_bridge_limit_payload_limit</a>(self: &<a href="message.md#0xb_message_UpdateBridgeLimit">UpdateBridgeLimit</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.limit
}
</code></pre>



</details>

<a name="0xb_message_update_asset_price_payload_token_id"></a>

## Function `update_asset_price_payload_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_asset_price_payload_token_id">update_asset_price_payload_token_id</a>(self: &<a href="message.md#0xb_message_UpdateAssetPrice">message::UpdateAssetPrice</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_asset_price_payload_token_id">update_asset_price_payload_token_id</a>(self: &<a href="message.md#0xb_message_UpdateAssetPrice">UpdateAssetPrice</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.token_id
}
</code></pre>



</details>

<a name="0xb_message_update_asset_price_payload_new_price"></a>

## Function `update_asset_price_payload_new_price`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_asset_price_payload_new_price">update_asset_price_payload_new_price</a>(self: &<a href="message.md#0xb_message_UpdateAssetPrice">message::UpdateAssetPrice</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_update_asset_price_payload_new_price">update_asset_price_payload_new_price</a>(self: &<a href="message.md#0xb_message_UpdateAssetPrice">UpdateAssetPrice</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.new_price
}
</code></pre>



</details>

<a name="0xb_message_add_external_coin_witness_payload_coin_type"></a>

## Function `add_external_coin_witness_payload_coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_witness_payload_coin_type">add_external_coin_witness_payload_coin_type</a>(self: &<a href="message.md#0xb_message_AddExternalCoinWitness">message::AddExternalCoinWitness</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_witness_payload_coin_type">add_external_coin_witness_payload_coin_type</a>(self: &<a href="message.md#0xb_message_AddExternalCoinWitness">AddExternalCoinWitness</a>): String {
    self.coin_type
}
</code></pre>



</details>

<a name="0xb_message_add_external_coin_witness_payload_witness_address"></a>

## Function `add_external_coin_witness_payload_witness_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_witness_payload_witness_address">add_external_coin_witness_payload_witness_address</a>(self: &<a href="message.md#0xb_message_AddExternalCoinWitness">message::AddExternalCoinWitness</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_witness_payload_witness_address">add_external_coin_witness_payload_witness_address</a>(self: &<a href="message.md#0xb_message_AddExternalCoinWitness">AddExternalCoinWitness</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    self.witness_address
}
</code></pre>



</details>

<a name="0xb_message_remove_external_coin_witness_payload_coin_type"></a>

## Function `remove_external_coin_witness_payload_coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_witness_payload_coin_type">remove_external_coin_witness_payload_coin_type</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinWitness">message::RemoveExternalCoinWitness</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_witness_payload_coin_type">remove_external_coin_witness_payload_coin_type</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinWitness">RemoveExternalCoinWitness</a>): String {
    self.coin_type
}
</code></pre>



</details>

<a name="0xb_message_remove_external_coin_witness_payload_witness_address"></a>

## Function `remove_external_coin_witness_payload_witness_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_witness_payload_witness_address">remove_external_coin_witness_payload_witness_address</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinWitness">message::RemoveExternalCoinWitness</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_witness_payload_witness_address">remove_external_coin_witness_payload_witness_address</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinWitness">RemoveExternalCoinWitness</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    self.witness_address
}
</code></pre>



</details>

<a name="0xb_message_add_external_coin_target_payload_coin_type"></a>

## Function `add_external_coin_target_payload_coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_target_payload_coin_type">add_external_coin_target_payload_coin_type</a>(self: &<a href="message.md#0xb_message_AddExternalCoinTarget">message::AddExternalCoinTarget</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_target_payload_coin_type">add_external_coin_target_payload_coin_type</a>(self: &<a href="message.md#0xb_message_AddExternalCoinTarget">AddExternalCoinTarget</a>): String {
    self.coin_type
}
</code></pre>



</details>

<a name="0xb_message_add_external_coin_target_payload_target_address"></a>

## Function `add_external_coin_target_payload_target_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_target_payload_target_address">add_external_coin_target_payload_target_address</a>(self: &<a href="message.md#0xb_message_AddExternalCoinTarget">message::AddExternalCoinTarget</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_target_payload_target_address">add_external_coin_target_payload_target_address</a>(self: &<a href="message.md#0xb_message_AddExternalCoinTarget">AddExternalCoinTarget</a>): String {
    self.target_address
}
</code></pre>



</details>

<a name="0xb_message_remove_external_coin_target_payload_coin_type"></a>

## Function `remove_external_coin_target_payload_coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_target_payload_coin_type">remove_external_coin_target_payload_coin_type</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinTarget">message::RemoveExternalCoinTarget</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_target_payload_coin_type">remove_external_coin_target_payload_coin_type</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinTarget">RemoveExternalCoinTarget</a>): String {
    self.coin_type
}
</code></pre>



</details>

<a name="0xb_message_remove_external_coin_target_payload_target_address"></a>

## Function `remove_external_coin_target_payload_target_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_target_payload_target_address">remove_external_coin_target_payload_target_address</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinTarget">message::RemoveExternalCoinTarget</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_target_payload_target_address">remove_external_coin_target_payload_target_address</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinTarget">RemoveExternalCoinTarget</a>): String {
    self.target_address
}
</code></pre>



</details>

<a name="0xb_message_add_external_coin_admin_payload_coin_type"></a>

## Function `add_external_coin_admin_payload_coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_admin_payload_coin_type">add_external_coin_admin_payload_coin_type</a>(self: &<a href="message.md#0xb_message_AddExternalCoinAdmin">message::AddExternalCoinAdmin</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_admin_payload_coin_type">add_external_coin_admin_payload_coin_type</a>(self: &<a href="message.md#0xb_message_AddExternalCoinAdmin">AddExternalCoinAdmin</a>): String {
    self.coin_type
}
</code></pre>



</details>

<a name="0xb_message_add_external_coin_admin_payload_admin_address"></a>

## Function `add_external_coin_admin_payload_admin_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_admin_payload_admin_address">add_external_coin_admin_payload_admin_address</a>(self: &<a href="message.md#0xb_message_AddExternalCoinAdmin">message::AddExternalCoinAdmin</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_external_coin_admin_payload_admin_address">add_external_coin_admin_payload_admin_address</a>(self: &<a href="message.md#0xb_message_AddExternalCoinAdmin">AddExternalCoinAdmin</a>): String {
    self.admin_address
}
</code></pre>



</details>

<a name="0xb_message_remove_external_coin_admin_payload_coin_type"></a>

## Function `remove_external_coin_admin_payload_coin_type`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_admin_payload_coin_type">remove_external_coin_admin_payload_coin_type</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinAdmin">message::RemoveExternalCoinAdmin</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_admin_payload_coin_type">remove_external_coin_admin_payload_coin_type</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinAdmin">RemoveExternalCoinAdmin</a>): String {
    self.coin_type
}
</code></pre>



</details>

<a name="0xb_message_remove_external_coin_admin_payload_admin_address"></a>

## Function `remove_external_coin_admin_payload_admin_address`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_admin_payload_admin_address">remove_external_coin_admin_payload_admin_address</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinAdmin">message::RemoveExternalCoinAdmin</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_external_coin_admin_payload_admin_address">remove_external_coin_admin_payload_admin_address</a>(self: &<a href="message.md#0xb_message_RemoveExternalCoinAdmin">RemoveExternalCoinAdmin</a>): String {
    self.admin_address
}
</code></pre>



</details>

<a name="0xb_message_add_token_on_token_list_payload_from_chain_id"></a>

## Function `add_token_on_token_list_payload_from_chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_token_on_token_list_payload_from_chain_id">add_token_on_token_list_payload_from_chain_id</a>(self: &<a href="message.md#0xb_message_AddTokenOnTokenList">message::AddTokenOnTokenList</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_token_on_token_list_payload_from_chain_id">add_token_on_token_list_payload_from_chain_id</a>(self: &<a href="message.md#0xb_message_AddTokenOnTokenList">AddTokenOnTokenList</a>): u8 {
    self.from_chain_id
}
</code></pre>



</details>

<a name="0xb_message_add_token_on_token_list_payload_to_chain_id"></a>

## Function `add_token_on_token_list_payload_to_chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_token_on_token_list_payload_to_chain_id">add_token_on_token_list_payload_to_chain_id</a>(self: &<a href="message.md#0xb_message_AddTokenOnTokenList">message::AddTokenOnTokenList</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_token_on_token_list_payload_to_chain_id">add_token_on_token_list_payload_to_chain_id</a>(self: &<a href="message.md#0xb_message_AddTokenOnTokenList">AddTokenOnTokenList</a>): u8 {
    self.to_chain_id
}
</code></pre>



</details>

<a name="0xb_message_add_token_on_token_list_payload_token_id"></a>

## Function `add_token_on_token_list_payload_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_token_on_token_list_payload_token_id">add_token_on_token_list_payload_token_id</a>(self: &<a href="message.md#0xb_message_AddTokenOnTokenList">message::AddTokenOnTokenList</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_add_token_on_token_list_payload_token_id">add_token_on_token_list_payload_token_id</a>(self: &<a href="message.md#0xb_message_AddTokenOnTokenList">AddTokenOnTokenList</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.token_id
}
</code></pre>



</details>

<a name="0xb_message_remove_token_on_token_list_payload_from_chain_id"></a>

## Function `remove_token_on_token_list_payload_from_chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_token_on_token_list_payload_from_chain_id">remove_token_on_token_list_payload_from_chain_id</a>(self: &<a href="message.md#0xb_message_RemoveTokenOnTokenList">message::RemoveTokenOnTokenList</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_token_on_token_list_payload_from_chain_id">remove_token_on_token_list_payload_from_chain_id</a>(self: &<a href="message.md#0xb_message_RemoveTokenOnTokenList">RemoveTokenOnTokenList</a>): u8 {
   self.from_chain_id
}
</code></pre>



</details>

<a name="0xb_message_remove_token_on_token_list_payload_to_chain_id"></a>

## Function `remove_token_on_token_list_payload_to_chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_token_on_token_list_payload_to_chain_id">remove_token_on_token_list_payload_to_chain_id</a>(self: &<a href="message.md#0xb_message_RemoveTokenOnTokenList">message::RemoveTokenOnTokenList</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_token_on_token_list_payload_to_chain_id">remove_token_on_token_list_payload_to_chain_id</a>(self: &<a href="message.md#0xb_message_RemoveTokenOnTokenList">RemoveTokenOnTokenList</a>): u8 {
    self.to_chain_id
}
</code></pre>



</details>

<a name="0xb_message_remove_token_on_token_list_payload_token_id"></a>

## Function `remove_token_on_token_list_payload_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_token_on_token_list_payload_token_id">remove_token_on_token_list_payload_token_id</a>(self: &<a href="message.md#0xb_message_RemoveTokenOnTokenList">message::RemoveTokenOnTokenList</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_remove_token_on_token_list_payload_token_id">remove_token_on_token_list_payload_token_id</a>(self: &<a href="message.md#0xb_message_RemoveTokenOnTokenList">RemoveTokenOnTokenList</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    self.token_id
}
</code></pre>



</details>

<a name="0xb_message_is_native"></a>

## Function `is_native`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_is_native">is_native</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">message::AddTokenOnSui</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_is_native">is_native</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">AddTokenOnSui</a>): bool {
    self.native_token
}
</code></pre>



</details>

<a name="0xb_message_token_ids"></a>

## Function `token_ids`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_ids">token_ids</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">message::AddTokenOnSui</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_ids">token_ids</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">AddTokenOnSui</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    self.token_ids
}
</code></pre>



</details>

<a name="0xb_message_token_type_names"></a>

## Function `token_type_names`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_type_names">token_type_names</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">message::AddTokenOnSui</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_type_names">token_type_names</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">AddTokenOnSui</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;String&gt; {
    self.token_type_names
}
</code></pre>



</details>

<a name="0xb_message_token_prices"></a>

## Function `token_prices`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_prices">token_prices</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">message::AddTokenOnSui</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_token_prices">token_prices</a>(self: &<a href="message.md#0xb_message_AddTokenOnSui">AddTokenOnSui</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    self.token_prices
}
</code></pre>



</details>

<a name="0xb_message_emergency_op_pause"></a>

## Function `emergency_op_pause`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_emergency_op_pause">emergency_op_pause</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_emergency_op_pause">emergency_op_pause</a>(): u8 {
    <a href="message.md#0xb_message_PAUSE">PAUSE</a>
}
</code></pre>



</details>

<a name="0xb_message_emergency_op_unpause"></a>

## Function `emergency_op_unpause`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_emergency_op_unpause">emergency_op_unpause</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_emergency_op_unpause">emergency_op_unpause</a>(): u8 {
    <a href="message.md#0xb_message_UNPAUSE">UNPAUSE</a>
}
</code></pre>



</details>

<a name="0xb_message_required_voting_power"></a>

## Function `required_voting_power`

Return the required signature threshold for the message, values are voting power in the scale of 10000


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_required_voting_power">required_voting_power</a>(self: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_required_voting_power">required_voting_power</a>(self: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> message_type = <a href="message.md#0xb_message_message_type">message_type</a>(self);

    <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_token">message_types::token</a>()) {
        3334
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_emergency_op">message_types::emergency_op</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message_extract_emergency_op_payload">extract_emergency_op_payload</a>(self);
        <b>if</b> (payload.op_type == <a href="message.md#0xb_message_PAUSE">PAUSE</a>) {
            450
        } <b>else</b> <b>if</b> (payload.op_type == <a href="message.md#0xb_message_UNPAUSE">UNPAUSE</a>) {
            5001
        } <b>else</b> {
            <b>abort</b> <a href="message.md#0xb_message_EInvalidEmergencyOpType">EInvalidEmergencyOpType</a>
        }
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_committee_blocklist">message_types::committee_blocklist</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_update_asset_price">message_types::update_asset_price</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_update_bridge_limit">message_types::update_bridge_limit</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_add_tokens_on_sui">message_types::add_tokens_on_sui</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_add_external_coin_admin">message_types::add_external_coin_admin</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_remove_external_coin_admin">message_types::remove_external_coin_admin</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_refund_admin_operate">message_types::refund_admin_operate</a>()) {
        5001
    }<b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_add_external_coin_witness">message_types::add_external_coin_witness</a>()) {
        5001
    }<b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_remove_external_coin_witness">message_types::remove_external_coin_witness</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_add_external_coin_target">message_types::add_external_coin_target</a>()) {
        5001
    }<b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_remove_external_coin_target">message_types::remove_external_coin_target</a>()) {
        5001
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_add_token_on_token_list">message_types::add_token_on_token_list</a>()) {
        5001
    }<b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_remove_token_on_token_list">message_types::remove_token_on_token_list</a>()) {
        5001
    }<b>else</b> {
        <b>abort</b> <a href="message.md#0xb_message_EInvalidMessageType">EInvalidMessageType</a>
    }
}
</code></pre>



</details>

<a name="0xb_message_to_parsed_token_transfer_message"></a>

## Function `to_parsed_token_transfer_message`



<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_to_parsed_token_transfer_message">to_parsed_token_transfer_message</a>(<a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>): <a href="message.md#0xb_message_ParsedTokenTransferMessage">message::ParsedTokenTransferMessage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="message.md#0xb_message_to_parsed_token_transfer_message">to_parsed_token_transfer_message</a>(
    <a href="message.md#0xb_message">message</a>: &<a href="message.md#0xb_message_BridgeMessage">BridgeMessage</a>,
): <a href="message.md#0xb_message_ParsedTokenTransferMessage">ParsedTokenTransferMessage</a> {
    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.<a href="message.md#0xb_message_message_type">message_type</a>() == <a href="message_types.md#0xb_message_types_token">message_types::token</a>(), <a href="message.md#0xb_message_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>let</b> payload = <a href="message.md#0xb_message">message</a>.<a href="message.md#0xb_message_extract_token_bridge_payload">extract_token_bridge_payload</a>();
    <a href="message.md#0xb_message_ParsedTokenTransferMessage">ParsedTokenTransferMessage</a> {
        message_version: <a href="message.md#0xb_message">message</a>.<a href="message.md#0xb_message_message_version">message_version</a>(),
        seq_num: <a href="message.md#0xb_message">message</a>.<a href="message.md#0xb_message_seq_num">seq_num</a>(),
        source_chain: <a href="message.md#0xb_message">message</a>.<a href="message.md#0xb_message_source_chain">source_chain</a>(),
        payload: <a href="message.md#0xb_message">message</a>.<a href="message.md#0xb_message_payload">payload</a>(),
        parsed_payload: payload,
    }
}
</code></pre>



</details>

<a name="0xb_message_reverse_bytes"></a>

## Function `reverse_bytes`



<pre><code><b>fun</b> <a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="message.md#0xb_message_reverse_bytes">reverse_bytes</a>(<b>mut</b> bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="../move-stdlib/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> bytes);
    bytes
}
</code></pre>



</details>

<a name="0xb_message_peel_u64_be"></a>

## Function `peel_u64_be`



<pre><code><b>fun</b> <a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> bcs::BCS): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="message.md#0xb_message_peel_u64_be">peel_u64_be</a>(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>: &<b>mut</b> BCS): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> (<b>mut</b> value, <b>mut</b> i) = (0u64, 64u8);
    <b>while</b> (i &gt; 0) {
        i = i - 8;
        <b>let</b> byte = (bcs::peel_u8(<a href="../move-stdlib/bcs.md#0x1_bcs">bcs</a>) <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>);
        value = value + (byte &lt;&lt; i);
    };
    value
}
</code></pre>



</details>

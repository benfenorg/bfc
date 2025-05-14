---
title: Module `0xb::bridge`
---



-  [Resource `Bridge`](#0xb_bridge_Bridge)
-  [Struct `BridgeInner`](#0xb_bridge_BridgeInner)
-  [Struct `TokenDepositedEvent`](#0xb_bridge_TokenDepositedEvent)
-  [Struct `TokenSendBackEvent`](#0xb_bridge_TokenSendBackEvent)
-  [Struct `EmergencyOpEvent`](#0xb_bridge_EmergencyOpEvent)
-  [Struct `BridgeRecord`](#0xb_bridge_BridgeRecord)
-  [Struct `TokenTransferApproved`](#0xb_bridge_TokenTransferApproved)
-  [Struct `TokenTransferClaimed`](#0xb_bridge_TokenTransferClaimed)
-  [Struct `TokenTransferAlreadyApproved`](#0xb_bridge_TokenTransferAlreadyApproved)
-  [Struct `TokenTransferAlreadyClaimed`](#0xb_bridge_TokenTransferAlreadyClaimed)
-  [Struct `TokenTransferLimitExceed`](#0xb_bridge_TokenTransferLimitExceed)
-  [Struct `ExternalPreDepositedEvent`](#0xb_bridge_ExternalPreDepositedEvent)
-  [Struct `ExternalPreDepositedDoneEvent`](#0xb_bridge_ExternalPreDepositedDoneEvent)
-  [Struct `ExternalDepositedApprovedEvent`](#0xb_bridge_ExternalDepositedApprovedEvent)
-  [Struct `ExternalDepositStartEvent`](#0xb_bridge_ExternalDepositStartEvent)
-  [Struct `ExternalDepositedEvent`](#0xb_bridge_ExternalDepositedEvent)
-  [Struct `ExternalWithdrawEvent`](#0xb_bridge_ExternalWithdrawEvent)
-  [Struct `ExternalBridgeMessageKey`](#0xb_bridge_ExternalBridgeMessageKey)
-  [Struct `ExternalBridgeRecord`](#0xb_bridge_ExternalBridgeRecord)
-  [Constants](#@Constants_0)
-  [Function `create`](#0xb_bridge_create)
-  [Function `init_bridge_committee`](#0xb_bridge_init_bridge_committee)
-  [Function `migrate`](#0xb_bridge_migrate)
-  [Function `committee_registration`](#0xb_bridge_committee_registration)
-  [Function `update_node_url`](#0xb_bridge_update_node_url)
-  [Function `register_foreign_token`](#0xb_bridge_register_foreign_token)
-  [Function `send_token`](#0xb_bridge_send_token)
-  [Function `send_busd`](#0xb_bridge_send_busd)
-  [Function `send_back_token`](#0xb_bridge_send_back_token)
-  [Function `is_refund_admin`](#0xb_bridge_is_refund_admin)
-  [Function `approve_token_transfer`](#0xb_bridge_approve_token_transfer)
-  [Function `get_max_mint_busd_amount`](#0xb_bridge_get_max_mint_busd_amount)
-  [Function `set_max_mint_busd_amount`](#0xb_bridge_set_max_mint_busd_amount)
-  [Function `claim_token`](#0xb_bridge_claim_token)
-  [Function `claim_and_transfer_token`](#0xb_bridge_claim_and_transfer_token)
-  [Function `claim_and_transfer_busd`](#0xb_bridge_claim_and_transfer_busd)
-  [Function `execute_system_message`](#0xb_bridge_execute_system_message)
-  [Function `get_available_claim_amount`](#0xb_bridge_get_available_claim_amount)
-  [Function `pre_deposit_external_coin`](#0xb_bridge_pre_deposit_external_coin)
-  [Function `deposit_external_coin`](#0xb_bridge_deposit_external_coin)
-  [Function `approval_and_claimed_external_coin`](#0xb_bridge_approval_and_claimed_external_coin)
-  [Function `withdraw_external_coin`](#0xb_bridge_withdraw_external_coin)
-  [Function `get_token_transfer_action_status`](#0xb_bridge_get_token_transfer_action_status)
-  [Function `get_external_token_transfer_action_status`](#0xb_bridge_get_external_token_transfer_action_status)
-  [Function `get_send_back_status`](#0xb_bridge_get_send_back_status)
-  [Function `get_token_transfer_action_signatures`](#0xb_bridge_get_token_transfer_action_signatures)
-  [Function `multi_signature_passed`](#0xb_bridge_multi_signature_passed)
-  [Function `load_inner`](#0xb_bridge_load_inner)
-  [Function `load_inner_mut`](#0xb_bridge_load_inner_mut)
-  [Function `load_inner_mut_and_uid`](#0xb_bridge_load_inner_mut_and_uid)
-  [Function `claim_token_internal`](#0xb_bridge_claim_token_internal)
-  [Function `claim_stable_token_internal`](#0xb_bridge_claim_stable_token_internal)
-  [Function `execute_emergency_op`](#0xb_bridge_execute_emergency_op)
-  [Function `execute_refund_admin_operate`](#0xb_bridge_execute_refund_admin_operate)
-  [Function `add_refund_admin`](#0xb_bridge_add_refund_admin)
-  [Function `remove_refund_admin`](#0xb_bridge_remove_refund_admin)
-  [Function `execute_update_bridge_limit`](#0xb_bridge_execute_update_bridge_limit)
-  [Function `execute_update_asset_price`](#0xb_bridge_execute_update_asset_price)
-  [Function `execute_add_external_coin_admin`](#0xb_bridge_execute_add_external_coin_admin)
-  [Function `execute_remove_external_coin_admin`](#0xb_bridge_execute_remove_external_coin_admin)
-  [Function `execute_add_external_coin_target_payload`](#0xb_bridge_execute_add_external_coin_target_payload)
-  [Function `execute_remove_external_coin_target_payload`](#0xb_bridge_execute_remove_external_coin_target_payload)
-  [Function `execute_add_external_coin_witness`](#0xb_bridge_execute_add_external_coin_witness)
-  [Function `execute_remove_external_coin_witness`](#0xb_bridge_execute_remove_external_coin_witness)
-  [Function `execute_add_tokens_on_sui`](#0xb_bridge_execute_add_tokens_on_sui)
-  [Function `get_current_seq_num_and_increment`](#0xb_bridge_get_current_seq_num_and_increment)
-  [Function `get_parsed_token_transfer_message`](#0xb_bridge_get_parsed_token_transfer_message)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../sui-framework/address.md#0x2_address">0x2::address</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/hex.md#0x2_hex">0x2::hex</a>;
<b>use</b> <a href="../sui-framework/linked_table.md#0x2_linked_table">0x2::linked_table</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
<b>use</b> <a href="../sui-framework/versioned.md#0x2_versioned">0x2::versioned</a>;
<b>use</b> <a href="../sui-system/sui_system.md#0x3_sui_system">0x3::sui_system</a>;
<b>use</b> <a href="chain_ids.md#0xb_chain_ids">0xb::chain_ids</a>;
<b>use</b> <a href="committee.md#0xb_committee">0xb::committee</a>;
<b>use</b> <a href="limiter.md#0xb_limiter">0xb::limiter</a>;
<b>use</b> <a href="message.md#0xb_message">0xb::message</a>;
<b>use</b> <a href="message_types.md#0xb_message_types">0xb::message_types</a>;
<b>use</b> <a href="tokenlist.md#0xb_tokenlist">0xb::tokenlist</a>;
<b>use</b> <a href="treasury.md#0xb_treasury">0xb::treasury</a>;
<b>use</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system">0xc8::bfc_system</a>;
<b>use</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner">0xc8::bfc_system_state_inner</a>;
<b>use</b> <a href="../bfc-system/busd.md#0xc8_busd">0xc8::busd</a>;
</code></pre>



<a name="0xb_bridge_Bridge"></a>

## Resource `Bridge`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>inner: <a href="../sui-framework/versioned.md#0x2_versioned_Versioned">versioned::Versioned</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_BridgeInner"></a>

## Struct `BridgeInner`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>message_version: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>chain_id: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>sequence_nums: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u8, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="committee.md#0xb_committee">committee</a>: <a href="committee.md#0xb_committee_BridgeCommittee">committee::BridgeCommittee</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>: treasury::BridgeTreasury</code>
</dt>
<dd>

</dd>
<dt>
<code>token_transfer_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;<a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a>, <a href="bridge.md#0xb_bridge_BridgeRecord">bridge::BridgeRecord</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>external_bridge_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;<a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">bridge::ExternalBridgeMessageKey</a>, <a href="bridge.md#0xb_bridge_ExternalBridgeRecord">bridge::ExternalBridgeRecord</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>pre_deposit_multi_signature_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;<a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">bridge::ExternalBridgeMessageKey</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="limiter.md#0xb_limiter">limiter</a>: <a href="limiter.md#0xb_limiter_TransferLimiter">limiter::TransferLimiter</a></code>
</dt>
<dd>

</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>refund_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;<a href="message.md#0xb_message_RefundMessageKey">message::RefundMessageKey</a>, <a href="bridge.md#0xb_bridge_BridgeRecord">bridge::BridgeRecord</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>refund_admins: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_TokenDepositedEvent"></a>

## Struct `TokenDepositedEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_TokenDepositedEvent">TokenDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>benfen_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_TokenSendBackEvent"></a>

## Struct `TokenSendBackEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_TokenSendBackEvent">TokenSendBackEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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

<a name="0xb_bridge_EmergencyOpEvent"></a>

## Struct `EmergencyOpEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_EmergencyOpEvent">EmergencyOpEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>frozen: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_BridgeRecord"></a>

## Struct `BridgeRecord`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_BridgeRecord">BridgeRecord</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a></code>
</dt>
<dd>

</dd>
<dt>
<code>verified_signatures: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>claimed: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_TokenTransferApproved"></a>

## Struct `TokenTransferApproved`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_TokenTransferApproved">TokenTransferApproved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_TokenTransferClaimed"></a>

## Struct `TokenTransferClaimed`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_TokenTransferClaimed">TokenTransferClaimed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_TokenTransferAlreadyApproved"></a>

## Struct `TokenTransferAlreadyApproved`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_TokenTransferAlreadyClaimed"></a>

## Struct `TokenTransferAlreadyClaimed`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_TokenTransferAlreadyClaimed">TokenTransferAlreadyClaimed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_TokenTransferLimitExceed"></a>

## Struct `TokenTransferLimitExceed`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_TokenTransferLimitExceed">TokenTransferLimitExceed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="message.md#0xb_message_BridgeMessageKey">message::BridgeMessageKey</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_ExternalPreDepositedEvent"></a>

## Struct `ExternalPreDepositedEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalPreDepositedEvent">ExternalPreDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>target_chain: u8</code>
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
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_ExternalPreDepositedDoneEvent"></a>

## Struct `ExternalPreDepositedDoneEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalPreDepositedDoneEvent">ExternalPreDepositedDoneEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>target_chain: u8</code>
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
</dl>


</details>

<a name="0xb_bridge_ExternalDepositedApprovedEvent"></a>

## Struct `ExternalDepositedApprovedEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalDepositedApprovedEvent">ExternalDepositedApprovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>target_chain: u8</code>
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
</dl>


</details>

<a name="0xb_bridge_ExternalDepositStartEvent"></a>

## Struct `ExternalDepositStartEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalDepositStartEvent">ExternalDepositStartEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>target_chain: u8</code>
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
</dl>


</details>

<a name="0xb_bridge_ExternalDepositedEvent"></a>

## Struct `ExternalDepositedEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalDepositedEvent">ExternalDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>target_chain: u8</code>
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
</dl>


</details>

<a name="0xb_bridge_ExternalWithdrawEvent"></a>

## Struct `ExternalWithdrawEvent`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalWithdrawEvent">ExternalWithdrawEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>source_chain: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>target_chain: u8</code>
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
</dl>


</details>

<a name="0xb_bridge_ExternalBridgeMessageKey"></a>

## Struct `ExternalBridgeMessageKey`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a> <b>has</b> <b>copy</b>, drop, store
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
<code>tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xb_bridge_ExternalBridgeRecord"></a>

## Struct `ExternalBridgeRecord`



<pre><code><b>struct</b> <a href="bridge.md#0xb_bridge_ExternalBridgeRecord">ExternalBridgeRecord</a> <b>has</b> drop, store
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
<code>target_chain: u8</code>
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
<code>verified_signatures: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>claimed: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xb_bridge_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0xb_bridge_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EWrongInnerVersion">EWrongInnerVersion</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 7;
</code></pre>



<a name="0xb_bridge_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_CURRENT_VERSION">CURRENT_VERSION</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xb_bridge_EInvalidBridgeRoute"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 16;
</code></pre>



<a name="0xb_bridge_EMustBeTokenMessage"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 17;
</code></pre>



<a name="0xb_bridge_EBridgeAlreadyPaused"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EBridgeAlreadyPaused">EBridgeAlreadyPaused</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 13;
</code></pre>



<a name="0xb_bridge_EBridgeNotPaused"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EBridgeNotPaused">EBridgeNotPaused</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 14;
</code></pre>



<a name="0xb_bridge_EBridgeUnavailable"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 8;
</code></pre>



<a name="0xb_bridge_EDuplicateRefund"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EDuplicateRefund">EDuplicateRefund</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 22;
</code></pre>



<a name="0xb_bridge_EDuplicatedMessage"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EDuplicatedMessage">EDuplicatedMessage</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 30;
</code></pre>



<a name="0xb_bridge_EInvalidChainIDAndTokenIDExpect"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 34;
</code></pre>



<a name="0xb_bridge_EInvalidEvmAddress"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 18;
</code></pre>



<a name="0xb_bridge_EInvalidMinStakeParticipationPercentage"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidMinStakeParticipationPercentage">EInvalidMinStakeParticipationPercentage</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 50;
</code></pre>



<a name="0xb_bridge_EInvalidMintAmount"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidMintAmount">EInvalidMintAmount</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 41;
</code></pre>



<a name="0xb_bridge_EInvalidSender"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidSender">EInvalidSender</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 20;
</code></pre>



<a name="0xb_bridge_EInvalidTokenIdExpect"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidTokenIdExpect">EInvalidTokenIdExpect</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 23;
</code></pre>



<a name="0xb_bridge_EInvalidTxHash"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvalidTxHash">EInvalidTxHash</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 21;
</code></pre>



<a name="0xb_bridge_EInvariantSuiInitializedTokenTransferShouldNotBeClaimed"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EInvariantSuiInitializedTokenTransferShouldNotBeClaimed">EInvariantSuiInitializedTokenTransferShouldNotBeClaimed</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10;
</code></pre>



<a name="0xb_bridge_EMalformedMessageError"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EMalformedMessageError">EMalformedMessageError</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0xb_bridge_EMessageNotFoundInRecords"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EMessageNotFoundInRecords">EMessageNotFoundInRecords</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 11;
</code></pre>



<a name="0xb_bridge_EOnlySupportBusd"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EOnlySupportBusd">EOnlySupportBusd</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 24;
</code></pre>



<a name="0xb_bridge_ETokenAlreadyClaimedOrHitLimit"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_ETokenAlreadyClaimedOrHitLimit">ETokenAlreadyClaimedOrHitLimit</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 15;
</code></pre>



<a name="0xb_bridge_ETokenValueIsZero"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_ETokenValueIsZero">ETokenValueIsZero</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 19;
</code></pre>



<a name="0xb_bridge_EUnauthorisedClaim"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xb_bridge_EUnauthorisedUpdateLimit"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnauthorisedUpdateLimit">EUnauthorisedUpdateLimit</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 40;
</code></pre>



<a name="0xb_bridge_EUnexpectedChainID"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnexpectedChainID">EUnexpectedChainID</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0xb_bridge_EUnexpectedMessageType"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xb_bridge_EUnexpectedMessageVersion"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 12;
</code></pre>



<a name="0xb_bridge_EUnexpectedOperation"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnexpectedOperation">EUnexpectedOperation</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 9;
</code></pre>



<a name="0xb_bridge_EUnexpectedSeqNum"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnexpectedSeqNum">EUnexpectedSeqNum</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 6;
</code></pre>



<a name="0xb_bridge_EUnexpectedTokenType"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnexpectedTokenType">EUnexpectedTokenType</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0xb_bridge_EUnknownExternalCoinOrSender"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnknownExternalCoinOrSender">EUnknownExternalCoinOrSender</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 31;
</code></pre>



<a name="0xb_bridge_EUnpassedMultiSignature"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnpassedMultiSignature">EUnpassedMultiSignature</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 32;
</code></pre>



<a name="0xb_bridge_EUnpassedWitnessSignature"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUnpassedWitnessSignature">EUnpassedWitnessSignature</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 33;
</code></pre>



<a name="0xb_bridge_EUseClaimBusd"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUseClaimBusd">EUseClaimBusd</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 26;
</code></pre>



<a name="0xb_bridge_EUseSendBusd"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EUseSendBusd">EUseSendBusd</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 25;
</code></pre>



<a name="0xb_bridge_EVM_ADDRESS_LENGTH"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 20;
</code></pre>



<a name="0xb_bridge_MESSAGE_VERSION"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>: u8 = 1;
</code></pre>



<a name="0xb_bridge_TRANSFER_STATUS_APPROVED"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_APPROVED">TRANSFER_STATUS_APPROVED</a>: u8 = 1;
</code></pre>



<a name="0xb_bridge_TRANSFER_STATUS_CLAIMED"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>: u8 = 2;
</code></pre>



<a name="0xb_bridge_TRANSFER_STATUS_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>: u8 = 3;
</code></pre>



<a name="0xb_bridge_TRANSFER_STATUS_PENDING"></a>



<pre><code><b>const</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>: u8 = 0;
</code></pre>



<a name="0xb_bridge_create"></a>

## Function `create`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_create">create</a>(id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>, chain_id: u8, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_create">create</a>(id: UID, chain_id: u8, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="bridge.md#0xb_bridge_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> bridge_inner = <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> {
        bridge_version: <a href="bridge.md#0xb_bridge_CURRENT_VERSION">CURRENT_VERSION</a>,
        message_version: <a href="bridge.md#0xb_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>,
        chain_id,
        sequence_nums: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        <a href="committee.md#0xb_committee">committee</a>: <a href="committee.md#0xb_committee_create">committee::create</a>(ctx),
        <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>: treasury::create(ctx),
        token_transfer_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_new">linked_table::new</a>(ctx),
        external_bridge_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_new">linked_table::new</a>(ctx),
        pre_deposit_multi_signature_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_new">linked_table::new</a>(ctx),
        <a href="limiter.md#0xb_limiter">limiter</a>: <a href="limiter.md#0xb_limiter_new">limiter::new</a>(),
        paused: <b>false</b>,
        refund_records: <a href="../sui-framework/linked_table.md#0x2_linked_table_new">linked_table::new</a>(ctx),
        refund_admins: <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
    };
    <b>let</b> <a href="bridge.md#0xb_bridge">bridge</a> = <a href="bridge.md#0xb_bridge_Bridge">Bridge</a> {
        id,
        inner: <a href="../sui-framework/versioned.md#0x2_versioned_create">versioned::create</a>(<a href="bridge.md#0xb_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, bridge_inner, ctx)
    };
    <a href="../sui-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
}
</code></pre>



</details>

<a name="0xb_bridge_init_bridge_committee"></a>

## Function `init_bridge_committee`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_init_bridge_committee">init_bridge_committee</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, active_validator_voting_power: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<b>address</b>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;, min_stake_participation_percentage: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_init_bridge_committee">init_bridge_committee</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    active_validator_voting_power: VecMap&lt;<b>address</b>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
    min_stake_participation_percentage: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &TxContext
) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="bridge.md#0xb_bridge_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(min_stake_participation_percentage&gt;=7500, <a href="bridge.md#0xb_bridge_EInvalidMinStakeParticipationPercentage">EInvalidMinStakeParticipationPercentage</a>);
    <b>if</b> (inner.<a href="committee.md#0xb_committee">committee</a>.committee_members().is_empty()) {
        inner.<a href="committee.md#0xb_committee">committee</a>.try_create_next_committee(
            active_validator_voting_power,
            min_stake_participation_percentage,
            ctx,
        )
    }
}
</code></pre>



</details>

<a name="0xb_bridge_migrate"></a>

## Function `migrate`



<pre><code><b>public</b> entry <b>fun</b> <a href="bridge.md#0xb_bridge_migrate">migrate</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_migrate">migrate</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    ctx: &<b>mut</b> TxContext
){
    <a href="tokenlist.md#0xb_tokenlist_new_tokenlist_registry">tokenlist::new_tokenlist_registry</a>(&<b>mut</b> <a href="bridge.md#0xb_bridge">bridge</a>.id, ctx)
}
</code></pre>



</details>

<a name="0xb_bridge_committee_registration"></a>

## Function `committee_registration`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_committee_registration">committee_registration</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, system_state: &<b>mut</b> <a href="../sui-system/sui_system.md#0x3_sui_system_SuiSystemState">sui_system::SuiSystemState</a>, bridge_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, http_rest_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_committee_registration">committee_registration</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    system_state: &<b>mut</b> SuiSystemState,
    bridge_pubkey_bytes: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    http_rest_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext
) {
    <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>)
        .<a href="committee.md#0xb_committee">committee</a>
        .register(system_state, bridge_pubkey_bytes, http_rest_url, ctx);
}
</code></pre>



</details>

<a name="0xb_bridge_update_node_url"></a>

## Function `update_node_url`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_update_node_url">update_node_url</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, new_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_update_node_url">update_node_url</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>, new_url: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &TxContext) {
    <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>).<a href="committee.md#0xb_committee">committee</a>.<a href="bridge.md#0xb_bridge_update_node_url">update_node_url</a>(new_url, ctx);
}
</code></pre>



</details>

<a name="0xb_bridge_register_foreign_token"></a>

## Function `register_foreign_token`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_register_foreign_token">register_foreign_token</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, tc: <a href="../sui-framework/coin.md#0x2_coin_TreasuryCap">coin::TreasuryCap</a>&lt;T&gt;, uc: <a href="../sui-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>, metadata: &<a href="../sui-framework/coin.md#0x2_coin_CoinMetadata">coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_register_foreign_token">register_foreign_token</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    tc: TreasuryCap&lt;T&gt;,
    uc: UpgradeCap,
    metadata: &CoinMetadata&lt;T&gt;,
) {
    <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>)
        .<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>
        .<a href="bridge.md#0xb_bridge_register_foreign_token">register_foreign_token</a>&lt;T&gt;(tc, uc, metadata)
}
</code></pre>



</details>

<a name="0xb_bridge_send_token"></a>

## Function `send_token`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_send_token">send_token</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, target_chain: u8, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, token: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_send_token">send_token</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    token: Coin&lt;T&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner,parent_id) = <a href="bridge.md#0xb_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="chain_ids.md#0xb_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="bridge.md#0xb_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>assert</b>!(target_address.length() == <a href="bridge.md#0xb_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>, <a href="bridge.md#0xb_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>);

    <b>let</b> bridge_seq_num = inner.<a href="bridge.md#0xb_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="message_types.md#0xb_message_types_token">message_types::token</a>());
    <b>let</b> token_id = inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.token_id&lt;T&gt;();
    <b>let</b> token_amount = token.<a href="../sui-framework/balance.md#0x2_balance">balance</a>().value();
    <b>assert</b>!(token_amount &gt; 0, <a href="bridge.md#0xb_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>assert</b>!(token_id != 5, <a href="bridge.md#0xb_bridge_EUseSendBusd">EUseSendBusd</a>);

    <b>assert</b>!(<a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen">tokenlist::is_supported_from_benfen</a>(parent_id, target_chain <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id),<a href="bridge.md#0xb_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>);

    // create <a href="bridge.md#0xb_bridge">bridge</a> <a href="message.md#0xb_message">message</a>
    <b>let</b> <a href="message.md#0xb_message">message</a> = <a href="message.md#0xb_message_create_token_bridge_message">message::create_token_bridge_message</a>(
        inner.chain_id,
        bridge_seq_num,
        address::to_bytes(ctx.sender()),
        target_chain,
        target_address,
        token_id,
        token_amount,
        <a href="../sui-framework/hex.md#0x2_hex_decode">hex::decode</a>(b""),
        0u8, // event_idx
    );

    // burn / escrow token, unsupported coins will fail in this step
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.burn(token);

    // Store pending <a href="bridge.md#0xb_bridge">bridge</a> request
    inner.token_transfer_records.push_back(
        <a href="message.md#0xb_message">message</a>.key(),
        <a href="bridge.md#0xb_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="message.md#0xb_message">message</a>,
            verified_signatures: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
            claimed: <b>false</b>,
        },
    );

    // emit <a href="../sui-framework/event.md#0x2_event">event</a>
    emit(
        <a href="bridge.md#0xb_bridge_TokenDepositedEvent">TokenDepositedEvent</a> {
            seq_num: bridge_seq_num,
            source_chain: inner.chain_id,
            sender_address: address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_type: token_id,
            amount: token_amount,
            benfen_amount: token_amount,

        },
    );
}
</code></pre>



</details>

<a name="0xb_bridge_send_busd"></a>

## Function `send_busd`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_send_busd">send_busd</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, bfc_system_state: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, target_chain: u8, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, token: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;, token_id_expect: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_send_busd">send_busd</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    target_chain: u8,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    token: Coin&lt;T&gt;,
    token_id_expect: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="tokenlist.md#0xb_tokenlist_is_supported_from_benfen">tokenlist::is_supported_from_benfen</a>(&<a href="bridge.md#0xb_bridge">bridge</a>.id, target_chain <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_id_expect),<a href="bridge.md#0xb_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>);

    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="chain_ids.md#0xb_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="bridge.md#0xb_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>assert</b>!(target_address.length() == <a href="bridge.md#0xb_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>, <a href="bridge.md#0xb_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>);
    <b>let</b> is_busd = <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;() == <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;BUSD&gt;();
    <b>assert</b>!(is_busd, <a href="bridge.md#0xb_bridge_EOnlySupportBusd">EOnlySupportBusd</a>);
    <b>assert</b>!(token_id_expect == 3 || token_id_expect == 4, <a href="bridge.md#0xb_bridge_EInvalidTokenIdExpect">EInvalidTokenIdExpect</a>);

    <b>let</b> bridge_seq_num = inner.<a href="bridge.md#0xb_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="message_types.md#0xb_message_types_token">message_types::token</a>());
    // <b>let</b> token_id_origin = inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.token_id&lt;T&gt;();
    // <b>assert</b>!(token_id_origin == 5, <a href="bridge.md#0xb_bridge_EOnlySupportBusd">EOnlySupportBusd</a>);
    <b>let</b> token_id = token_id_expect;
    <b>let</b> benfen_amount=token.<a href="../sui-framework/balance.md#0x2_balance">balance</a>().value();

    <b>let</b> token_amount=<b>if</b> (target_chain==<a href="chain_ids.md#0xb_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() || target_chain==<a href="chain_ids.md#0xb_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() || target_chain==<a href="chain_ids.md#0xb_chain_ids_eth_custom">chain_ids::eth_custom</a>()) {
         token.<a href="../sui-framework/balance.md#0x2_balance">balance</a>().value()/1000u64
    }<b>else</b>{
         token.<a href="../sui-framework/balance.md#0x2_balance">balance</a>().value()
    };

    <b>assert</b>!(token_amount &gt; 0, <a href="bridge.md#0xb_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);

    // create <a href="bridge.md#0xb_bridge">bridge</a> <a href="message.md#0xb_message">message</a>
    <b>let</b> <a href="message.md#0xb_message">message</a> = <a href="message.md#0xb_message_create_token_bridge_message">message::create_token_bridge_message</a>(
        inner.chain_id,
        bridge_seq_num,
        address::to_bytes(ctx.sender()),
        target_chain,
        target_address,
        token_id,
        token_amount,
        <a href="../sui-framework/hex.md#0x2_hex_decode">hex::decode</a>(b""),
        0u8, // event_idx
    );

    // burn / escrow token, unsupported coins will fail in this step
    bfc_system_state.burn_stable(token, ctx);

    // Store pending <a href="bridge.md#0xb_bridge">bridge</a> request
    inner.token_transfer_records.push_back(
        <a href="message.md#0xb_message">message</a>.key(),
        <a href="bridge.md#0xb_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="message.md#0xb_message">message</a>,
            verified_signatures: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
            claimed: <b>false</b>,
        },
    );

    // emit <a href="../sui-framework/event.md#0x2_event">event</a>
    emit(
        <a href="bridge.md#0xb_bridge_TokenDepositedEvent">TokenDepositedEvent</a> {
            seq_num: bridge_seq_num,
            source_chain: inner.chain_id,
            sender_address: address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_type: token_id,
            amount: token_amount,
            benfen_amount: benfen_amount,
        },
    );
}
</code></pre>



</details>

<a name="0xb_bridge_send_back_token"></a>

## Function `send_back_token`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_send_back_token">send_back_token</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, target_chain: u8, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, token_type: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, event_idx: u8, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_send_back_token">send_back_token</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    token_type: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    event_idx: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="chain_ids.md#0xb_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="bridge.md#0xb_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>assert</b>!(!inner.refund_records.contains(<a href="message.md#0xb_message_key_refund">message::key_refund</a>(tx_hash)), <a href="bridge.md#0xb_bridge_EDuplicateRefund">EDuplicateRefund</a>);
    <b>assert</b>!(target_address.length() == <a href="bridge.md#0xb_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>, <a href="bridge.md#0xb_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>);
    <b>assert</b>!(token_amount &gt; 0, <a href="bridge.md#0xb_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>assert</b>!(tx_hash.length() &gt;= 1, <a href="bridge.md#0xb_bridge_EInvalidTxHash">EInvalidTxHash</a>);
    <b>assert</b>!(inner.<a href="bridge.md#0xb_bridge_is_refund_admin">is_refund_admin</a>(ctx.sender().to_ascii_string()), <a href="bridge.md#0xb_bridge_EInvalidSender">EInvalidSender</a>);
    <b>let</b> bridge_seq_num = inner.<a href="bridge.md#0xb_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="message_types.md#0xb_message_types_token">message_types::token</a>());
    // create <a href="bridge.md#0xb_bridge">bridge</a> <a href="message.md#0xb_message">message</a>
    <b>let</b> <a href="message.md#0xb_message">message</a> = <a href="message.md#0xb_message_create_token_bridge_message">message::create_token_bridge_message</a>(
        inner.chain_id,
        bridge_seq_num,
        address::to_bytes(ctx.sender()),
        target_chain,
        target_address,
        token_type,
        token_amount,
        tx_hash,
        event_idx,
    );
    // Store pending <a href="bridge.md#0xb_bridge">bridge</a> request
    inner.token_transfer_records.push_back(
        <a href="message.md#0xb_message">message</a>.key(),
        <a href="bridge.md#0xb_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="message.md#0xb_message">message</a>,
            verified_signatures: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
            claimed: <b>false</b>,
        },
    );
    //store for idempotency
    inner.refund_records.push_back(
        <a href="message.md#0xb_message_key_refund">message::key_refund</a>(tx_hash),
        <a href="bridge.md#0xb_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="message.md#0xb_message">message</a>,
            verified_signatures: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
            claimed: <b>false</b>,
        },
    );

    // emit <a href="../sui-framework/event.md#0x2_event">event</a>
    emit(
        <a href="bridge.md#0xb_bridge_TokenSendBackEvent">TokenSendBackEvent</a> {
            seq_num: bridge_seq_num,
            source_chain: inner.chain_id,
            sender_address: address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_type: token_type,
            amount: token_amount,
            tx_hash,
            event_idx,
        },
    );
}
</code></pre>



</details>

<a name="0xb_bridge_is_refund_admin"></a>

## Function `is_refund_admin`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_is_refund_admin">is_refund_admin</a>(inner: &<a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, <b>address</b>: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_is_refund_admin">is_refund_admin</a>(inner: &<a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, <b>address</b>: String): bool {
    inner.refund_admins.contains(&<b>address</b>)
}
</code></pre>



</details>

<a name="0xb_bridge_approve_token_transfer"></a>

## Function `approve_token_transfer`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_approve_token_transfer">approve_token_transfer</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, <a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_approve_token_transfer">approve_token_transfer</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    <a href="message.md#0xb_message">message</a>: BridgeMessage,
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
) {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    // verify signatures
    inner.<a href="committee.md#0xb_committee">committee</a>.verify_signatures(<a href="message.md#0xb_message">message</a>, signatures);

    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.message_type() == <a href="message_types.md#0xb_message_types_token">message_types::token</a>(), <a href="bridge.md#0xb_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.message_version() == <a href="bridge.md#0xb_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="bridge.md#0xb_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> token_payload = <a href="message.md#0xb_message">message</a>.extract_token_bridge_payload();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>assert</b>!(
        <a href="message.md#0xb_message">message</a>.source_chain() == inner.chain_id || target_chain == inner.chain_id,
        <a href="bridge.md#0xb_bridge_EUnexpectedChainID">EUnexpectedChainID</a>,
    );

    <b>let</b> message_key = <a href="message.md#0xb_message">message</a>.key();
    // retrieve pending <a href="message.md#0xb_message">message</a> <b>if</b> source chain is Sui, the initial <a href="message.md#0xb_message">message</a>
    // must exist on chain
    <b>if</b> (<a href="message.md#0xb_message">message</a>.source_chain() == inner.chain_id) {
        <b>let</b> record = &<b>mut</b> inner.token_transfer_records[message_key];

        <b>assert</b>!(record.<a href="message.md#0xb_message">message</a> == <a href="message.md#0xb_message">message</a>, <a href="bridge.md#0xb_bridge_EMalformedMessageError">EMalformedMessageError</a>);
        <b>assert</b>!(!record.claimed, <a href="bridge.md#0xb_bridge_EInvariantSuiInitializedTokenTransferShouldNotBeClaimed">EInvariantSuiInitializedTokenTransferShouldNotBeClaimed</a>);

        // If record already <b>has</b> verified signatures, it means the <a href="message.md#0xb_message">message</a> <b>has</b> been approved
        // Then we exit early.
        <b>if</b> (record.verified_signatures.is_some()) {
            emit(<a href="bridge.md#0xb_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        // Store approval
        record.verified_signatures = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(signatures)
    } <b>else</b> {
        // At this point, <b>if</b> this <a href="message.md#0xb_message">message</a> is in token_transfer_records, we know
        // it's already approved because we only add a <a href="message.md#0xb_message">message</a> <b>to</b> token_transfer_records
        // after verifying the signatures
        <b>if</b> (inner.token_transfer_records.contains(message_key)) {
            emit(<a href="bridge.md#0xb_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        //idempotency for SendBack and ETHToSui
        <b>let</b> tx_hash = token_payload.token_tx_hash();
        <b>if</b> (inner.refund_records.contains(<a href="message.md#0xb_message_key_refund">message::key_refund</a>(tx_hash))) {
            emit(<a href="bridge.md#0xb_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        // Store <a href="message.md#0xb_message">message</a> and approval
        inner.token_transfer_records.push_back(
            message_key,
            <a href="bridge.md#0xb_bridge_BridgeRecord">BridgeRecord</a> {
                <a href="message.md#0xb_message">message</a>,
                verified_signatures: <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(signatures),
                claimed: <b>false</b>
            },
        );
    };

    emit(<a href="bridge.md#0xb_bridge_TokenTransferApproved">TokenTransferApproved</a> { message_key });
}
</code></pre>



</details>

<a name="0xb_bridge_get_max_mint_busd_amount"></a>

## Function `get_max_mint_busd_amount`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_get_max_mint_busd_amount">get_max_mint_busd_amount</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_get_max_mint_busd_amount">get_max_mint_busd_amount</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    inner.<a href="limiter.md#0xb_limiter">limiter</a>.get_mint_busd_max_limit()
}
</code></pre>



</details>

<a name="0xb_bridge_set_max_mint_busd_amount"></a>

## Function `set_max_mint_busd_amount`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_set_max_mint_busd_amount">set_max_mint_busd_amount</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, bfc_system_state: &<a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>, new_limit: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_set_max_mint_busd_amount">set_max_mint_busd_amount</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    bfc_system_state: &BfcSystemState,
    cap: &BfcSystemModifyCap,
    new_limit: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
    ) {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(bfc_system_state.verify_capability(cap, ctx), <a href="bridge.md#0xb_bridge_EUnauthorisedUpdateLimit">EUnauthorisedUpdateLimit</a>);
    inner.<a href="limiter.md#0xb_limiter">limiter</a>.set_mint_busd_max_limit(new_limit);
}
</code></pre>



</details>

<a name="0xb_bridge_claim_token"></a>

## Function `claim_token`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_claim_token">claim_token</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_claim_token">claim_token</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> (maybe_token, owner) = <a href="bridge.md#0xb_bridge">bridge</a>.<a href="bridge.md#0xb_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(
        <a href="../sui-framework/clock.md#0x2_clock">clock</a>,
        source_chain,
        bridge_seq_num,
        ctx,
    );
    // Only token owner can claim the token
    <b>assert</b>!(ctx.sender() == owner, <a href="bridge.md#0xb_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>);
    <b>assert</b>!(maybe_token.is_some(), <a href="bridge.md#0xb_bridge_ETokenAlreadyClaimedOrHitLimit">ETokenAlreadyClaimedOrHitLimit</a>);
    maybe_token.destroy_some()
}
</code></pre>



</details>

<a name="0xb_bridge_claim_and_transfer_token"></a>

## Function `claim_and_transfer_token`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_claim_and_transfer_token">claim_and_transfer_token</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_claim_and_transfer_token">claim_and_transfer_token</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (token, owner) = <a href="bridge.md#0xb_bridge">bridge</a>.<a href="bridge.md#0xb_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(<a href="../sui-framework/clock.md#0x2_clock">clock</a>, source_chain, bridge_seq_num, ctx);
    <b>if</b> (token.is_some()) {
        <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(token.destroy_some(), owner)
    } <b>else</b> {
        token.destroy_none();
    };
}
</code></pre>



</details>

<a name="0xb_bridge_claim_and_transfer_busd"></a>

## Function `claim_and_transfer_busd`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_claim_and_transfer_busd">claim_and_transfer_busd</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, bfc_system_state: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_claim_and_transfer_busd">claim_and_transfer_busd</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    cap: &BfcSystemModifyCap,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (token, owner) = <a href="bridge.md#0xb_bridge">bridge</a>.<a href="bridge.md#0xb_bridge_claim_stable_token_internal">claim_stable_token_internal</a>&lt;T&gt;(bfc_system_state, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, source_chain, bridge_seq_num, cap, ctx);
    <b>if</b> (token.is_some()) {
        <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(token.destroy_some(), owner)
    } <b>else</b> {
        token.destroy_none();
    };
}
</code></pre>



</details>

<a name="0xb_bridge_execute_system_message"></a>

## Function `execute_system_message`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_execute_system_message">execute_system_message</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, <a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_execute_system_message">execute_system_message</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    <a href="message.md#0xb_message">message</a>: BridgeMessage,
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
) {
    <b>let</b> message_type = <a href="message.md#0xb_message">message</a>.message_type();

    // TODO: test version mismatch
    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.message_version() == <a href="bridge.md#0xb_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="bridge.md#0xb_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);

    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.source_chain() == inner.chain_id, <a href="bridge.md#0xb_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);

    // check system ops seq number and increment it
    <b>let</b> expected_seq_num = inner.<a href="bridge.md#0xb_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(message_type);
    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.seq_num() == expected_seq_num, <a href="bridge.md#0xb_bridge_EUnexpectedSeqNum">EUnexpectedSeqNum</a>);

    inner.<a href="committee.md#0xb_committee">committee</a>.verify_signatures(<a href="message.md#0xb_message">message</a>, signatures);

    <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_emergency_op">message_types::emergency_op</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_emergency_op_payload();
        inner.<a href="bridge.md#0xb_bridge_execute_emergency_op">execute_emergency_op</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_committee_blocklist">message_types::committee_blocklist</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_blocklist_payload();
        inner.<a href="committee.md#0xb_committee">committee</a>.execute_blocklist(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_update_bridge_limit">message_types::update_bridge_limit</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_update_bridge_limit();
        inner.<a href="bridge.md#0xb_bridge_execute_update_bridge_limit">execute_update_bridge_limit</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_update_asset_price">message_types::update_asset_price</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_update_asset_price();
        inner.<a href="bridge.md#0xb_bridge_execute_update_asset_price">execute_update_asset_price</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_add_external_coin_admin">message_types::add_external_coin_admin</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_add_external_coin_admin();
        inner.<a href="bridge.md#0xb_bridge_execute_add_external_coin_admin">execute_add_external_coin_admin</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_remove_external_coin_admin">message_types::remove_external_coin_admin</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_remove_external_coin_admin();
        inner.<a href="bridge.md#0xb_bridge_execute_remove_external_coin_admin">execute_remove_external_coin_admin</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_add_tokens_on_sui">message_types::add_tokens_on_sui</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_add_tokens_on_sui();
        inner.<a href="bridge.md#0xb_bridge_execute_add_tokens_on_sui">execute_add_tokens_on_sui</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="message_types.md#0xb_message_types_refund_admin_operate">message_types::refund_admin_operate</a>()) {
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_refund_admin_payload();
        inner.<a href="bridge.md#0xb_bridge_execute_refund_admin_operate">execute_refund_admin_operate</a>(payload);
    } <b>else</b> <b>if</b>  (message_type == <a href="message_types.md#0xb_message_types_add_external_coin_witness">message_types::add_external_coin_witness</a>()){
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_add_witness_poyload();
        inner.<a href="bridge.md#0xb_bridge_execute_add_external_coin_witness">execute_add_external_coin_witness</a>(payload);

    }<b>else</b> <b>if</b>  (message_type == <a href="message_types.md#0xb_message_types_remove_external_coin_witness">message_types::remove_external_coin_witness</a>()){
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_remove_witness_poyload();
        inner.<a href="bridge.md#0xb_bridge_execute_remove_external_coin_witness">execute_remove_external_coin_witness</a>(payload);

    }<b>else</b> <b>if</b>  (message_type == <a href="message_types.md#0xb_message_types_add_external_coin_target">message_types::add_external_coin_target</a>()){
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_add_external_target_address_poyload();
        inner.<a href="bridge.md#0xb_bridge_execute_add_external_coin_target_payload">execute_add_external_coin_target_payload</a>(payload);
    }<b>else</b> <b>if</b>  (message_type == <a href="message_types.md#0xb_message_types_remove_external_coin_target">message_types::remove_external_coin_target</a>()){
        <b>let</b> payload = <a href="message.md#0xb_message">message</a>.extract_remove_external_target_address_poyload();
        inner.<a href="bridge.md#0xb_bridge_execute_remove_external_coin_target_payload">execute_remove_external_coin_target_payload</a>(payload);

    }<b>else</b> {
        <b>abort</b> <a href="bridge.md#0xb_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>
    };
}
</code></pre>



</details>

<a name="0xb_bridge_get_available_claim_amount"></a>

## Function `get_available_claim_amount`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, source_chain: u8): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(
      <a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
      source_chain: u8,
): u128 {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>let</b> route = <a href="chain_ids.md#0xb_chain_ids_get_route">chain_ids::get_route</a>(source_chain, inner.chain_id);
    inner.<a href="limiter.md#0xb_limiter">limiter</a>.<a href="bridge.md#0xb_bridge_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(&inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, route)
}
</code></pre>



</details>

<a name="0xb_bridge_pre_deposit_external_coin"></a>

## Function `pre_deposit_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_pre_deposit_external_coin">pre_deposit_external_coin</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, source_chain: u8, source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_pre_deposit_external_coin">pre_deposit_external_coin</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>,
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = ctx.sender();
    <b>let</b> sender_str = sender.to_ascii_string();
    <b>let</b> coin_type = <a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;());

    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.verify_bitcoin_signatures&lt;T&gt;(source_chain, source_address, target_address, amount, tx_hash, signatures),<a href="bridge.md#0xb_bridge_EUnpassedWitnessSignature">EUnpassedWitnessSignature</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="chain_ids.md#0xb_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(source_chain, inner.chain_id), <a href="bridge.md#0xb_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>if</b> (!inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.is_external_coin_admin(coin_type, sender_str)) {
        <b>abort</b> <a href="bridge.md#0xb_bridge_EUnknownExternalCoinOrSender">EUnknownExternalCoinOrSender</a>
    };

    // check then add <b>to</b> pre_deposit_multi_signature_records
    <b>let</b> key = <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (inner.pre_deposit_multi_signature_records.contains(key)) {
        <b>let</b> records = &<b>mut</b> inner.pre_deposit_multi_signature_records[key];
        <b>if</b> (records.contains(&sender_str)) {
            <b>return</b>
        };

        records.insert(sender_str);
    } <b>else</b> {
        <b>let</b> <b>mut</b> records = <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>();
        records.insert(sender_str);
        inner.pre_deposit_multi_signature_records.push_back(key, records);
    };

    emit(
        <a href="bridge.md#0xb_bridge_ExternalPreDepositedEvent">ExternalPreDepositedEvent</a> {
            tx_hash,
            coin_type,
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount,
            sender,
            signatures,
        }
    );

    <b>if</b> (inner.<a href="bridge.md#0xb_bridge_multi_signature_passed">multi_signature_passed</a>(key, coin_type)) {
        emit(
            <a href="bridge.md#0xb_bridge_ExternalPreDepositedDoneEvent">ExternalPreDepositedDoneEvent</a> {
                tx_hash,
                coin_type,
                source_chain,
                target_chain: inner.chain_id,
                source_address,
                target_address,
                amount,
            },
        )
    }
}
</code></pre>



</details>

<a name="0xb_bridge_deposit_external_coin"></a>

## Function `deposit_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_deposit_external_coin">deposit_external_coin</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, source_chain: u8, source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_deposit_external_coin">deposit_external_coin</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>,
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = ctx.sender();
    <b>let</b> coin_type = <a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;());

    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);

    <b>assert</b>!(inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.verify_bitcoin_signatures&lt;T&gt;(source_chain, source_address, target_address, amount, tx_hash, signatures),<a href="bridge.md#0xb_bridge_EUnpassedWitnessSignature">EUnpassedWitnessSignature</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="chain_ids.md#0xb_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(source_chain, inner.chain_id), <a href="bridge.md#0xb_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>if</b> (!inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.is_external_coin_admin(coin_type, sender.to_ascii_string())) {
        <b>abort</b> <a href="bridge.md#0xb_bridge_EUnknownExternalCoinOrSender">EUnknownExternalCoinOrSender</a>
    };

    <b>let</b> key = <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (!inner.<a href="bridge.md#0xb_bridge_multi_signature_passed">multi_signature_passed</a>(key, coin_type)) {
        <b>abort</b> <a href="bridge.md#0xb_bridge_EUnpassedMultiSignature">EUnpassedMultiSignature</a>
    };

    // check records
    <b>let</b> key = <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (inner.external_bridge_records.contains(key)) {
        <b>abort</b> <a href="bridge.md#0xb_bridge_EDuplicatedMessage">EDuplicatedMessage</a>
    };

    // // v1
    // <b>let</b> token = inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.mint&lt;T&gt;(amount, ctx);
    // <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(token, address::from_bytes(target_address));

    // inner.external_bridge_records.push_back(
    //     key,
    //     <a href="bridge.md#0xb_bridge_ExternalBridgeRecord">ExternalBridgeRecord</a> {
    //         source_chain,
    //         target_chain: inner.chain_id,
    //         source_address,
    //         target_address,
    //         amount,
    //         verified_signatures: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
    //         claimed: <b>true</b>,
    //     },
    // );

    // emit(
    //     <a href="bridge.md#0xb_bridge_ExternalDepositedEvent">ExternalDepositedEvent</a> {
    //         tx_hash,
    //         coin_type,
    //         source_chain,
    //         target_chain: inner.chain_id,
    //         source_address,
    //         target_address,
    //         amount,
    //     },
    // )

    // v2
    <b>let</b> seq_num = inner.<a href="bridge.md#0xb_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="message_types.md#0xb_message_types_token">message_types::token</a>());
    <b>let</b> token_id = inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.token_id&lt;T&gt;();

    emit(
        <a href="bridge.md#0xb_bridge_ExternalDepositStartEvent">ExternalDepositStartEvent</a> {
            seq_num,
            tx_hash,
            token_id,
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount,
        },
    )
}
</code></pre>



</details>

<a name="0xb_bridge_approval_and_claimed_external_coin"></a>

## Function `approval_and_claimed_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_approval_and_claimed_external_coin">approval_and_claimed_external_coin</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, <a href="message.md#0xb_message">message</a>: <a href="message.md#0xb_message_BridgeMessage">message::BridgeMessage</a>, signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_approval_and_claimed_external_coin">approval_and_claimed_external_coin</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    <a href="message.md#0xb_message">message</a>: BridgeMessage,
    signatures: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);

    // verify signatures
    inner.<a href="committee.md#0xb_committee">committee</a>.verify_signatures(<a href="message.md#0xb_message">message</a>, signatures);

    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.message_type() == <a href="message_types.md#0xb_message_types_token">message_types::token</a>(), <a href="bridge.md#0xb_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>assert</b>!(<a href="message.md#0xb_message">message</a>.message_version() == <a href="bridge.md#0xb_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="bridge.md#0xb_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> token_payload = <a href="message.md#0xb_message">message</a>.extract_token_bridge_payload();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>assert</b>!(
        <a href="message.md#0xb_message">message</a>.source_chain() == inner.chain_id || target_chain == inner.chain_id,
        <a href="bridge.md#0xb_bridge_EUnexpectedChainID">EUnexpectedChainID</a>,
    );

    <b>let</b> coin_type = <a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;());
    // check records
    <b>let</b> tx_hash = <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(token_payload.token_tx_hash());
    <b>let</b> source_chain = <a href="message.md#0xb_message">message</a>.source_chain();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>let</b> source_address = token_payload.token_sender_address();
    <b>let</b> target_address = token_payload.token_target_address();
    <b>let</b> amount = token_payload.token_amount();
    <b>let</b> key = <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (inner.external_bridge_records.contains(key)) {
        emit(<a href="bridge.md#0xb_bridge_ExternalDepositedApprovedEvent">ExternalDepositedApprovedEvent</a>{
            tx_hash,
            coin_type,
            source_chain: source_chain,
            target_chain: target_chain,
            source_address: source_address,
            target_address: target_address,
            amount: token_payload.token_amount(),
           });

        <b>return</b>
    };

    <b>let</b> token = inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.mint&lt;T&gt;(amount, ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(token, address::from_bytes(target_address));

    inner.external_bridge_records.push_back(
        key,
        <a href="bridge.md#0xb_bridge_ExternalBridgeRecord">ExternalBridgeRecord</a> {
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount,
            verified_signatures: <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(signatures),
            claimed: <b>true</b>,
        },
    );

    emit(
        <a href="bridge.md#0xb_bridge_ExternalDepositedEvent">ExternalDepositedEvent</a> {
            tx_hash,
            coin_type,
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount,
        },
    )
}
</code></pre>



</details>

<a name="0xb_bridge_withdraw_external_coin"></a>

## Function `withdraw_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_withdraw_external_coin">withdraw_external_coin</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, target_chain: u8, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, token: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bridge.md#0xb_bridge_withdraw_external_coin">withdraw_external_coin</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    token: Coin&lt;T&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="chain_ids.md#0xb_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="bridge.md#0xb_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);

    <b>let</b> coin_type = <a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;());
    <b>let</b> amount = token.<a href="../sui-framework/balance.md#0x2_balance">balance</a>().value();
    <b>assert</b>!(amount &gt; 0, <a href="bridge.md#0xb_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);

    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.burn(token);

    // emit <a href="../sui-framework/event.md#0x2_event">event</a>
    emit(
        <a href="bridge.md#0xb_bridge_ExternalWithdrawEvent">ExternalWithdrawEvent</a> {
            coin_type,
            source_chain: inner.chain_id,
            target_chain,
            source_address: address::to_bytes(ctx.sender()),
            target_address,
            amount,
        },
    );
}
</code></pre>



</details>

<a name="0xb_bridge_get_token_transfer_action_status"></a>

## Function `get_token_transfer_action_status`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_token_transfer_action_status">get_token_transfer_action_status</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_token_transfer_action_status">get_token_transfer_action_status</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): u8 {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>let</b> key = <a href="message.md#0xb_message_create_key">message::create_key</a>(
        source_chain,
        <a href="message_types.md#0xb_message_types_token">message_types::token</a>(),
        bridge_seq_num
    );

    <b>if</b> (!inner.token_transfer_records.contains(key)) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>
    };

    <b>let</b> record = &inner.token_transfer_records[key];
    <b>if</b> (record.claimed) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>
    };

    <b>if</b> (record.verified_signatures.is_some()) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_APPROVED">TRANSFER_STATUS_APPROVED</a>
    };

    <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>
}
</code></pre>



</details>

<a name="0xb_bridge_get_external_token_transfer_action_status"></a>

## Function `get_external_token_transfer_action_status`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_external_token_transfer_action_status">get_external_token_transfer_action_status</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, source_chain: u8, source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_external_token_transfer_action_status">get_external_token_transfer_action_status</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    source_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    target_address: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    tx_hash: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>,
): u8 {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>);

     <b>let</b> key = <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };

    <b>if</b> (!inner.external_bridge_records.contains(key)) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>
    };

    <b>let</b> record = &inner.external_bridge_records[key];
    <b>if</b> (record.claimed) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>
    };

    <b>if</b> (record.verified_signatures.is_some()) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_APPROVED">TRANSFER_STATUS_APPROVED</a>
    };

    <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>
}
</code></pre>



</details>

<a name="0xb_bridge_get_send_back_status"></a>

## Function `get_send_back_status`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_send_back_status">get_send_back_status</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_send_back_status">get_send_back_status</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    tx_hash: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): u8 {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>let</b> key = <a href="message.md#0xb_message_key_refund">message::key_refund</a>(tx_hash);

    <b>if</b> (!inner.refund_records.contains(key)) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>
    };

    <b>let</b> record = &inner.refund_records[key];
    <b>if</b> (record.claimed) {
        <b>return</b> <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>
    };
    <a href="bridge.md#0xb_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>
}
</code></pre>



</details>

<a name="0xb_bridge_get_token_transfer_action_signatures"></a>

## Function `get_token_transfer_action_signatures`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_token_transfer_action_signatures">get_token_transfer_action_signatures</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_token_transfer_action_signatures">get_token_transfer_action_signatures</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): Option&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt; {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>let</b> key = <a href="message.md#0xb_message_create_key">message::create_key</a>(
        source_chain,
        <a href="message_types.md#0xb_message_types_token">message_types::token</a>(),
        bridge_seq_num
    );

    <b>if</b> (!inner.token_transfer_records.contains(key)) {
        <b>return</b> <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>()
    };

    <b>let</b> record = &inner.token_transfer_records[key];
    record.verified_signatures
}
</code></pre>



</details>

<a name="0xb_bridge_multi_signature_passed"></a>

## Function `multi_signature_passed`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_multi_signature_passed">multi_signature_passed</a>(inner: &<a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, key: <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">bridge::ExternalBridgeMessageKey</a>, coin_type: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_multi_signature_passed">multi_signature_passed</a>(
    inner: & <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>,
    key: <a href="bridge.md#0xb_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>,
    coin_type: String,
): bool {
    // check then add <b>to</b> pre_deposit_multi_signature_records
    <b>if</b> (inner.pre_deposit_multi_signature_records.contains(key)) {
        <b>let</b> records = inner.pre_deposit_multi_signature_records[key];
        // <b>if</b> pre_deposit_multi_signature_records &gt; 50%
        <b>let</b> signed = records.size();
        <b>let</b> len = inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.external_coin_admin_count(coin_type);
        <b>if</b> (signed * 2 &gt; len) {
            <b>return</b> <b>true</b>
        };
    };

    <b>false</b>
}
</code></pre>



</details>

<a name="0xb_bridge_load_inner"></a>

## Function `load_inner`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>): &<a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
): &<a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> {
    <b>let</b> version = <a href="bridge.md#0xb_bridge">bridge</a>.inner.version();

    // TODO: Replace this <b>with</b> a lazy <b>update</b> function when we add a new version of the inner <a href="../sui-framework/object.md#0x2_object">object</a>.
    <b>assert</b>!(version == <a href="bridge.md#0xb_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="bridge.md#0xb_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> = <a href="bridge.md#0xb_bridge">bridge</a>.inner.load_value();
    <b>assert</b>!(inner.bridge_version == version, <a href="bridge.md#0xb_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="0xb_bridge_load_inner_mut"></a>

## Function `load_inner_mut`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>): &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>): &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> {
    <b>let</b> version = <a href="bridge.md#0xb_bridge">bridge</a>.inner.version();
    // TODO: Replace this <b>with</b> a lazy <b>update</b> function when we add a new version of the inner <a href="../sui-framework/object.md#0x2_object">object</a>.
    <b>assert</b>!(version == <a href="bridge.md#0xb_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="bridge.md#0xb_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> = <a href="bridge.md#0xb_bridge">bridge</a>.inner.load_value_mut();
    <b>assert</b>!(inner.bridge_version == version, <a href="bridge.md#0xb_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="0xb_bridge_load_inner_mut_and_uid"></a>

## Function `load_inner_mut_and_uid`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>): (&<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, &<b>mut</b> <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>): (&<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> ,&<b>mut</b> UID){
    <b>let</b> version = <a href="bridge.md#0xb_bridge">bridge</a>.inner.version();
    // TODO: Replace this <b>with</b> a lazy <b>update</b> function when we add a new version of the inner <a href="../sui-framework/object.md#0x2_object">object</a>.
    <b>assert</b>!(version == <a href="bridge.md#0xb_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="bridge.md#0xb_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a> = <a href="bridge.md#0xb_bridge">bridge</a>.inner.load_value_mut();
    <b>assert</b>!(inner.bridge_version == version, <a href="bridge.md#0xb_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    (inner,&<b>mut</b> <a href="bridge.md#0xb_bridge">bridge</a>.id)
}
</code></pre>



</details>

<a name="0xb_bridge_claim_token_internal"></a>

## Function `claim_token_internal`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;&gt;, <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): (Option&lt;Coin&lt;T&gt;&gt;, <b>address</b>) {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>let</b> is_busd = <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;T&gt;() == <a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;BUSD&gt;();
    <b>assert</b>!(!is_busd, <a href="bridge.md#0xb_bridge_EUseClaimBusd">EUseClaimBusd</a>);
    <b>let</b> key = <a href="message.md#0xb_message_create_key">message::create_key</a>(source_chain, <a href="message_types.md#0xb_message_types_token">message_types::token</a>(), bridge_seq_num);
    <b>assert</b>!(inner.token_transfer_records.contains(key), <a href="bridge.md#0xb_bridge_EMessageNotFoundInRecords">EMessageNotFoundInRecords</a>);

    // retrieve approved <a href="bridge.md#0xb_bridge">bridge</a> <a href="message.md#0xb_message">message</a>
    <b>let</b> record = &<b>mut</b> inner.token_transfer_records[key];
    // ensure this is a token <a href="bridge.md#0xb_bridge">bridge</a> <a href="message.md#0xb_message">message</a>
    <b>assert</b>!(
        &record.<a href="message.md#0xb_message">message</a>.message_type() == <a href="message_types.md#0xb_message_types_token">message_types::token</a>(),
        <a href="bridge.md#0xb_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>,
    );
    // Ensure it's signed
    <b>assert</b>!(record.verified_signatures.is_some(), <a href="bridge.md#0xb_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>);

    // extract token <a href="message.md#0xb_message">message</a>
    <b>let</b> token_payload = record.<a href="message.md#0xb_message">message</a>.extract_token_bridge_payload();
    // get owner <b>address</b>
    <b>let</b> owner = address::from_bytes(token_payload.token_target_address());

    // If already claimed, exit early
    <b>if</b> (record.claimed) {
        emit(<a href="bridge.md#0xb_bridge_TokenTransferAlreadyClaimed">TokenTransferAlreadyClaimed</a> { message_key: key });
        <b>return</b> (<a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), owner)
    };

    <b>let</b> target_chain = token_payload.token_target_chain();
    // ensure target chain matches <a href="bridge.md#0xb_bridge">bridge</a>.chain_id
    <b>assert</b>!(target_chain == inner.chain_id, <a href="bridge.md#0xb_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);

    // TODO: why do we check validity of the route here? what <b>if</b> inconsistency?
    // Ensure route is valid
    // TODO: add unit tests
    // `get_route` <b>abort</b> <b>if</b> route is invalid
    <b>let</b> route = <a href="chain_ids.md#0xb_chain_ids_get_route">chain_ids::get_route</a>(source_chain, target_chain);
    // check token type
    <b>assert</b>!(
        treasury::token_id&lt;T&gt;(&inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>) == token_payload.token_type(),
        <a href="bridge.md#0xb_bridge_EUnexpectedTokenType">EUnexpectedTokenType</a>,
    );

    <b>let</b> amount = token_payload.token_amount();
    // Make sure <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a> is within limit.
    <b>if</b> (!inner
        .<a href="limiter.md#0xb_limiter">limiter</a>
        .check_and_record_sending_transfer&lt;T&gt;(
        &inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>,
        <a href="../sui-framework/clock.md#0x2_clock">clock</a>,
        route,
        amount,
    )
    ) {
        emit(<a href="bridge.md#0xb_bridge_TokenTransferLimitExceed">TokenTransferLimitExceed</a> { message_key: key });
        <b>return</b> (<a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), owner)
    };

    <b>let</b> token = inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.mint&lt;T&gt;(amount, ctx);

    // Record changes
    record.claimed = <b>true</b>;
    emit(<a href="bridge.md#0xb_bridge_TokenTransferClaimed">TokenTransferClaimed</a> { message_key: key });

    (<a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(token), owner)
}
</code></pre>



</details>

<a name="0xb_bridge_claim_stable_token_internal"></a>

## Function `claim_stable_token_internal`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_claim_stable_token_internal">claim_stable_token_internal</a>&lt;T&gt;(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, bfc_system_state: &<b>mut</b> <a href="../bfc-system/bfc_system.md#0xc8_bfc_system_BfcSystemState">bfc_system::BfcSystemState</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, cap: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;&gt;, <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_claim_stable_token_internal">claim_stable_token_internal</a>&lt;T&gt;(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    cap: &BfcSystemModifyCap,
    ctx: &<b>mut</b> TxContext,
): (Option&lt;Coin&lt;T&gt;&gt;, <b>address</b>) {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner_mut">load_inner_mut</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);

    <b>let</b> key = <a href="message.md#0xb_message_create_key">message::create_key</a>(source_chain, <a href="message_types.md#0xb_message_types_token">message_types::token</a>(), bridge_seq_num);
    <b>assert</b>!(inner.token_transfer_records.contains(key), <a href="bridge.md#0xb_bridge_EMessageNotFoundInRecords">EMessageNotFoundInRecords</a>);

    // retrieve approved <a href="bridge.md#0xb_bridge">bridge</a> <a href="message.md#0xb_message">message</a>
    <b>let</b> record = &<b>mut</b> inner.token_transfer_records[key];
    // ensure this is a token <a href="bridge.md#0xb_bridge">bridge</a> <a href="message.md#0xb_message">message</a>
    <b>assert</b>!(
        &record.<a href="message.md#0xb_message">message</a>.message_type() == <a href="message_types.md#0xb_message_types_token">message_types::token</a>(),
        <a href="bridge.md#0xb_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>,
    );
    // Ensure it's signed
    <b>assert</b>!(record.verified_signatures.is_some(), <a href="bridge.md#0xb_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>);

    // extract token <a href="message.md#0xb_message">message</a>
    <b>let</b> token_payload = record.<a href="message.md#0xb_message">message</a>.extract_token_bridge_payload();
    // get owner <b>address</b>
    <b>let</b> owner = address::from_bytes(token_payload.token_target_address());
    // get token type
    <b>let</b> token_id = token_payload.token_type();
    <b>assert</b>!(token_id == 5, <a href="bridge.md#0xb_bridge_EOnlySupportBusd">EOnlySupportBusd</a>);

    // If already claimed, exit early
    <b>if</b> (record.claimed) {
        emit(<a href="bridge.md#0xb_bridge_TokenTransferAlreadyClaimed">TokenTransferAlreadyClaimed</a> { message_key: key });
        <b>return</b> (<a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), owner)
    };

    <b>let</b> target_chain = token_payload.token_target_chain();
    // ensure target chain matches <a href="bridge.md#0xb_bridge">bridge</a>.chain_id
    <b>assert</b>!(target_chain == inner.chain_id, <a href="bridge.md#0xb_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);

    // `get_route` <b>abort</b> <b>if</b> route is invalid
    <b>let</b> route = <a href="chain_ids.md#0xb_chain_ids_get_route">chain_ids::get_route</a>(source_chain, target_chain);
    // check token type
    <b>assert</b>!(
        treasury::token_id&lt;T&gt;(&inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>) == token_payload.token_type(),
        <a href="bridge.md#0xb_bridge_EUnexpectedTokenType">EUnexpectedTokenType</a>,
    );

    <b>let</b> amount = token_payload.token_amount();
    <b>assert</b>!(amount &lt; inner.<a href="limiter.md#0xb_limiter">limiter</a>.get_mint_busd_max_limit(), <a href="bridge.md#0xb_bridge_EInvalidMintAmount">EInvalidMintAmount</a>);
    // Make sure <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a> is within limit.
    <b>if</b> (!inner
        .<a href="limiter.md#0xb_limiter">limiter</a>
        .check_and_record_sending_transfer&lt;T&gt;(
        &inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>,
        <a href="../sui-framework/clock.md#0x2_clock">clock</a>,
        route,
        amount,
    )
    ) {
        emit(<a href="bridge.md#0xb_bridge_TokenTransferLimitExceed">TokenTransferLimitExceed</a> { message_key: key });
        <b>return</b> (<a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), owner)
    };

    // claim from <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>
    //<a href="../sui-framework/transfer.md#0x2_transfer">transfer</a> <a href="../bfc-system/busd.md#0xc8_busd">busd</a> <b>to</b> owner
    bfc_system_state.mint_stable_entry_to_address&lt;BUSD&gt;(amount, cap, owner, ctx);
    record.claimed = <b>true</b>;
    emit(<a href="bridge.md#0xb_bridge_TokenTransferClaimed">TokenTransferClaimed</a> { message_key: key });
    (<a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), owner)
}
</code></pre>



</details>

<a name="0xb_bridge_execute_emergency_op"></a>

## Function `execute_emergency_op`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_emergency_op">execute_emergency_op</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_EmergencyOp">message::EmergencyOp</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_emergency_op">execute_emergency_op</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: EmergencyOp) {
    <b>let</b> op = payload.emergency_op_type();
    <b>if</b> (op == <a href="message.md#0xb_message_emergency_op_pause">message::emergency_op_pause</a>()) {
        <b>assert</b>!(!inner.paused, <a href="bridge.md#0xb_bridge_EBridgeAlreadyPaused">EBridgeAlreadyPaused</a>);
        inner.paused = <b>true</b>;
        emit(<a href="bridge.md#0xb_bridge_EmergencyOpEvent">EmergencyOpEvent</a> { frozen: <b>true</b> });
    } <b>else</b> <b>if</b> (op == <a href="message.md#0xb_message_emergency_op_unpause">message::emergency_op_unpause</a>()) {
        <b>assert</b>!(inner.paused, <a href="bridge.md#0xb_bridge_EBridgeNotPaused">EBridgeNotPaused</a>);
        inner.paused = <b>false</b>;
        emit(<a href="bridge.md#0xb_bridge_EmergencyOpEvent">EmergencyOpEvent</a> { frozen: <b>false</b> });
    } <b>else</b> {
        <b>abort</b> <a href="bridge.md#0xb_bridge_EUnexpectedOperation">EUnexpectedOperation</a>
    };
}
</code></pre>



</details>

<a name="0xb_bridge_execute_refund_admin_operate"></a>

## Function `execute_refund_admin_operate`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_refund_admin_operate">execute_refund_admin_operate</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_RefundAdmin">message::RefundAdmin</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_refund_admin_operate">execute_refund_admin_operate</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: <a href="message.md#0xb_message_RefundAdmin">message::RefundAdmin</a>) {
    <b>let</b> op = payload.refund_admin_op_type();
    <b>if</b> (op == <a href="message.md#0xb_message_refund_admin_add">message::refund_admin_add</a>()) {
        <b>let</b> sui_address = payload.refund_admin_sui_address();
        inner.<a href="bridge.md#0xb_bridge_add_refund_admin">add_refund_admin</a>(sui_address);
    } <b>else</b> <b>if</b> (op == <a href="message.md#0xb_message_refund_admin_remove">message::refund_admin_remove</a>()) {
        <b>let</b> sui_address = payload.refund_admin_sui_address();
        inner.<a href="bridge.md#0xb_bridge_remove_refund_admin">remove_refund_admin</a>(sui_address);
    } <b>else</b> {
        <b>abort</b> <a href="bridge.md#0xb_bridge_EUnexpectedOperation">EUnexpectedOperation</a>
    };
}
</code></pre>



</details>

<a name="0xb_bridge_add_refund_admin"></a>

## Function `add_refund_admin`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_add_refund_admin">add_refund_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, <b>address</b>: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_add_refund_admin">add_refund_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, <b>address</b>: &String) {
    <b>if</b> (!inner.refund_admins.contains(<b>address</b>)) {
        inner.refund_admins.insert(*<b>address</b>);
    }
}
</code></pre>



</details>

<a name="0xb_bridge_remove_refund_admin"></a>

## Function `remove_refund_admin`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_remove_refund_admin">remove_refund_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, <b>address</b>: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_remove_refund_admin">remove_refund_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, <b>address</b>: &String) {
    <b>if</b> (inner.refund_admins.contains(<b>address</b>)) {
        inner.refund_admins.remove(<b>address</b>);
    }
}
</code></pre>



</details>

<a name="0xb_bridge_execute_update_bridge_limit"></a>

## Function `execute_update_bridge_limit`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_update_bridge_limit">execute_update_bridge_limit</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_UpdateBridgeLimit">message::UpdateBridgeLimit</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_update_bridge_limit">execute_update_bridge_limit</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: UpdateBridgeLimit) {
    <b>let</b> receiving_chain = payload.update_bridge_limit_payload_receiving_chain();
    <b>assert</b>!(receiving_chain == inner.chain_id, <a href="bridge.md#0xb_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);
    <b>let</b> route = <a href="chain_ids.md#0xb_chain_ids_get_route">chain_ids::get_route</a>(
        payload.update_bridge_limit_payload_sending_chain(),
        receiving_chain
    );

    inner.<a href="limiter.md#0xb_limiter">limiter</a>.update_route_limit(
        &route,
        payload.update_bridge_limit_payload_limit()
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_update_asset_price"></a>

## Function `execute_update_asset_price`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_update_asset_price">execute_update_asset_price</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_UpdateAssetPrice">message::UpdateAssetPrice</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_update_asset_price">execute_update_asset_price</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: UpdateAssetPrice) {
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.update_asset_notional_price(
        payload.update_asset_price_payload_token_id(),
        payload.update_asset_price_payload_new_price()
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_add_external_coin_admin"></a>

## Function `execute_add_external_coin_admin`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_external_coin_admin">execute_add_external_coin_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_AddExternalCoinAdmin">message::AddExternalCoinAdmin</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_external_coin_admin">execute_add_external_coin_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: AddExternalCoinAdmin) {
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.add_external_coin_admin(
        payload.add_external_coin_admin_payload_coin_type(),
        payload.add_external_coin_admin_payload_admin_address(),
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_remove_external_coin_admin"></a>

## Function `execute_remove_external_coin_admin`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_remove_external_coin_admin">execute_remove_external_coin_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_RemoveExternalCoinAdmin">message::RemoveExternalCoinAdmin</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_remove_external_coin_admin">execute_remove_external_coin_admin</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: RemoveExternalCoinAdmin) {
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.remove_external_coin_admin(
        payload.remove_external_coin_admin_payload_coin_type(),
        payload.remove_external_coin_admin_payload_admin_address(),
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_add_external_coin_target_payload"></a>

## Function `execute_add_external_coin_target_payload`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_external_coin_target_payload">execute_add_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_AddExternalCoinTarget">message::AddExternalCoinTarget</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_external_coin_target_payload">execute_add_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: AddExternalCoinTarget) {
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.add_external_coin_target(
        payload.add_external_coin_target_payload_coin_type(),
        payload.add_external_coin_target_payload_target_address(),
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_remove_external_coin_target_payload"></a>

## Function `execute_remove_external_coin_target_payload`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_remove_external_coin_target_payload">execute_remove_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_RemoveExternalCoinTarget">message::RemoveExternalCoinTarget</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_remove_external_coin_target_payload">execute_remove_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: RemoveExternalCoinTarget) {
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.remove_external_coin_target(
        payload.remove_external_coin_target_payload_coin_type(),
        payload.remove_external_coin_target_payload_target_address(),
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_add_external_coin_witness"></a>

## Function `execute_add_external_coin_witness`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_external_coin_witness">execute_add_external_coin_witness</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_AddExternalCoinWitness">message::AddExternalCoinWitness</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_external_coin_witness">execute_add_external_coin_witness</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: AddExternalCoinWitness) {
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.add_external_coin_witness(
        payload.add_external_coin_witness_payload_coin_type(),
        payload.add_external_coin_witness_payload_witness_address(),
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_remove_external_coin_witness"></a>

## Function `execute_remove_external_coin_witness`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_remove_external_coin_witness">execute_remove_external_coin_witness</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_RemoveExternalCoinWitness">message::RemoveExternalCoinWitness</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_remove_external_coin_witness">execute_remove_external_coin_witness</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: RemoveExternalCoinWitness) {
    inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.remove_external_coin_witness(
        payload.remove_external_coin_witness_payload_coin_type(),
        payload.remove_external_coin_witness_payload_witness_address(),
    )
}
</code></pre>



</details>

<a name="0xb_bridge_execute_add_tokens_on_sui"></a>

## Function `execute_add_tokens_on_sui`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_tokens_on_sui">execute_add_tokens_on_sui</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, payload: <a href="message.md#0xb_message_AddTokenOnSui">message::AddTokenOnSui</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_execute_add_tokens_on_sui">execute_add_tokens_on_sui</a>(inner: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, payload: AddTokenOnSui) {
    // FIXME: <b>assert</b> native_token <b>to</b> be <b>false</b> and add test
    <b>let</b> native_token = payload.is_native();
    <b>let</b> <b>mut</b> token_ids = payload.token_ids();
    <b>let</b> <b>mut</b> token_type_names = payload.token_type_names();
    <b>let</b> <b>mut</b> token_prices = payload.token_prices();

    // Make sure token data is consistent
    <b>assert</b>!(token_ids.length() == token_type_names.length(), <a href="bridge.md#0xb_bridge_EMalformedMessageError">EMalformedMessageError</a>);
    <b>assert</b>!(token_ids.length() == token_prices.length(), <a href="bridge.md#0xb_bridge_EMalformedMessageError">EMalformedMessageError</a>);

    <b>while</b> (token_ids.length() &gt; 0) {
        <b>let</b> token_id = token_ids.pop_back();
        <b>let</b> token_type_name = token_type_names.pop_back();
        <b>let</b> token_price = token_prices.pop_back();
        inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>.add_new_token(token_type_name, token_id, native_token, token_price)
    }
}
</code></pre>



</details>

<a name="0xb_bridge_get_current_seq_num_and_increment"></a>

## Function `get_current_seq_num_and_increment`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">bridge::BridgeInner</a>, msg_type: u8): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<b>mut</b> <a href="bridge.md#0xb_bridge_BridgeInner">BridgeInner</a>, msg_type: u8): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>if</b> (!<a href="bridge.md#0xb_bridge">bridge</a>.sequence_nums.contains(&msg_type)) {
        <a href="bridge.md#0xb_bridge">bridge</a>.sequence_nums.insert(msg_type, 1);
        <b>return</b> 0
    };

    <b>let</b> entry = &<b>mut</b> <a href="bridge.md#0xb_bridge">bridge</a>.sequence_nums[&msg_type];
    <b>let</b> seq_num = *entry;
    *entry = seq_num + 1;
    seq_num
}
</code></pre>



</details>

<a name="0xb_bridge_get_parsed_token_transfer_message"></a>

## Function `get_parsed_token_transfer_message`



<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_parsed_token_transfer_message">get_parsed_token_transfer_message</a>(<a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">bridge::Bridge</a>, source_chain: u8, bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="message.md#0xb_message_ParsedTokenTransferMessage">message::ParsedTokenTransferMessage</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bridge.md#0xb_bridge_get_parsed_token_transfer_message">get_parsed_token_transfer_message</a>(
    <a href="bridge.md#0xb_bridge">bridge</a>: &<a href="bridge.md#0xb_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    bridge_seq_num: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
): Option&lt;ParsedTokenTransferMessage&gt; {
    <b>let</b> inner = <a href="bridge.md#0xb_bridge_load_inner">load_inner</a>(<a href="bridge.md#0xb_bridge">bridge</a>);
    <b>let</b> key = <a href="message.md#0xb_message_create_key">message::create_key</a>(
        source_chain,
        <a href="message_types.md#0xb_message_types_token">message_types::token</a>(),
        bridge_seq_num
    );

    <b>if</b> (!inner.token_transfer_records.contains(key)) {
        <b>return</b> <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>()
    };

    <b>let</b> record = &inner.token_transfer_records[key];
    <b>let</b> <a href="message.md#0xb_message">message</a> = &record.<a href="message.md#0xb_message">message</a>;
    <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(to_parsed_token_transfer_message(<a href="message.md#0xb_message">message</a>))
}
</code></pre>



</details>

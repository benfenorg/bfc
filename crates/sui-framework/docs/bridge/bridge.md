---
title: Module `bridge::bridge`
---



-  [Struct `Bridge`](#bridge_bridge_Bridge)
-  [Struct `BridgeInner`](#bridge_bridge_BridgeInner)
-  [Struct `TokenDepositedEvent`](#bridge_bridge_TokenDepositedEvent)
-  [Struct `TokenDepositedEventV2`](#bridge_bridge_TokenDepositedEventV2)
-  [Struct `TokenSendBackEvent`](#bridge_bridge_TokenSendBackEvent)
-  [Struct `TokenSendBackEventV2`](#bridge_bridge_TokenSendBackEventV2)
-  [Struct `EmergencyOpEvent`](#bridge_bridge_EmergencyOpEvent)
-  [Struct `BridgeRecord`](#bridge_bridge_BridgeRecord)
-  [Struct `TokenTransferApproved`](#bridge_bridge_TokenTransferApproved)
-  [Struct `TokenTransferClaimed`](#bridge_bridge_TokenTransferClaimed)
-  [Struct `TokenTransferAlreadyApproved`](#bridge_bridge_TokenTransferAlreadyApproved)
-  [Struct `TokenTransferAlreadyClaimed`](#bridge_bridge_TokenTransferAlreadyClaimed)
-  [Struct `TokenTransferLimitExceed`](#bridge_bridge_TokenTransferLimitExceed)
-  [Struct `ExternalPreDepositedEvent`](#bridge_bridge_ExternalPreDepositedEvent)
-  [Struct `ExternalPreDepositedDoneEvent`](#bridge_bridge_ExternalPreDepositedDoneEvent)
-  [Struct `ExternalDepositedApprovedEvent`](#bridge_bridge_ExternalDepositedApprovedEvent)
-  [Struct `ExternalDepositStartEvent`](#bridge_bridge_ExternalDepositStartEvent)
-  [Struct `ExternalDepositedEvent`](#bridge_bridge_ExternalDepositedEvent)
-  [Struct `ExternalDepositedEventV2`](#bridge_bridge_ExternalDepositedEventV2)
-  [Struct `ExternalWithdrawEvent`](#bridge_bridge_ExternalWithdrawEvent)
-  [Struct `ExternalWithdrawEventV2`](#bridge_bridge_ExternalWithdrawEventV2)
-  [Struct `ExternalBridgeMessageKey`](#bridge_bridge_ExternalBridgeMessageKey)
-  [Struct `ExternalBridgeRecord`](#bridge_bridge_ExternalBridgeRecord)
-  [Constants](#@Constants_0)
-  [Function `create`](#bridge_bridge_create)
-  [Function `init_bridge_committee`](#bridge_bridge_init_bridge_committee)
-  [Function `migrate`](#bridge_bridge_migrate)
-  [Function `init_token_list`](#bridge_bridge_init_token_list)
-  [Function `committee_registration`](#bridge_bridge_committee_registration)
-  [Function `update_node_url`](#bridge_bridge_update_node_url)
-  [Function `register_foreign_token`](#bridge_bridge_register_foreign_token)
-  [Function `send_token`](#bridge_bridge_send_token)
-  [Function `send_busd`](#bridge_bridge_send_busd)
-  [Function `send_back_token`](#bridge_bridge_send_back_token)
-  [Function `send_back_token_v2`](#bridge_bridge_send_back_token_v2)
-  [Function `is_refund_admin`](#bridge_bridge_is_refund_admin)
-  [Function `approve_token_transfer`](#bridge_bridge_approve_token_transfer)
-  [Function `approve_token_transfer_v2`](#bridge_bridge_approve_token_transfer_v2)
-  [Function `approve_token_transfer_in`](#bridge_bridge_approve_token_transfer_in)
-  [Function `get_max_mint_busd_amount`](#bridge_bridge_get_max_mint_busd_amount)
-  [Function `set_max_mint_busd_amount`](#bridge_bridge_set_max_mint_busd_amount)
-  [Function `claim_token`](#bridge_bridge_claim_token)
-  [Function `claim_and_transfer_token`](#bridge_bridge_claim_and_transfer_token)
-  [Function `claim_and_transfer_busd`](#bridge_bridge_claim_and_transfer_busd)
-  [Function `execute_system_message_with_ctx`](#bridge_bridge_execute_system_message_with_ctx)
-  [Function `execute_system_message`](#bridge_bridge_execute_system_message)
-  [Function `get_available_claim_amount`](#bridge_bridge_get_available_claim_amount)
-  [Function `pre_deposit_external_coin`](#bridge_bridge_pre_deposit_external_coin)
-  [Function `deposit_external_coin`](#bridge_bridge_deposit_external_coin)
-  [Function `approval_and_claimed_external_coin`](#bridge_bridge_approval_and_claimed_external_coin)
-  [Function `approval_and_claimed_external_busd_coin`](#bridge_bridge_approval_and_claimed_external_busd_coin)
-  [Function `withdraw_external_busd_coin`](#bridge_bridge_withdraw_external_busd_coin)
-  [Function `withdraw_external_coin`](#bridge_bridge_withdraw_external_coin)
-  [Function `get_unclaimed_bridge_fee`](#bridge_bridge_get_unclaimed_bridge_fee)
-  [Function `get_cross_out_fee_amount`](#bridge_bridge_get_cross_out_fee_amount)
-  [Function `get_cross_in_fee_amount`](#bridge_bridge_get_cross_in_fee_amount)
-  [Function `get_token_transfer_action_status`](#bridge_bridge_get_token_transfer_action_status)
-  [Function `get_external_token_transfer_action_status`](#bridge_bridge_get_external_token_transfer_action_status)
-  [Function `get_send_back_status`](#bridge_bridge_get_send_back_status)
-  [Function `get_token_transfer_action_signatures`](#bridge_bridge_get_token_transfer_action_signatures)
-  [Function `multi_signature_passed`](#bridge_bridge_multi_signature_passed)
-  [Function `load_inner`](#bridge_bridge_load_inner)
-  [Function `load_inner_mut`](#bridge_bridge_load_inner_mut)
-  [Function `load_inner_mut_and_uid`](#bridge_bridge_load_inner_mut_and_uid)
-  [Function `load_inner_and_uid`](#bridge_bridge_load_inner_and_uid)
-  [Function `claim_token_internal`](#bridge_bridge_claim_token_internal)
-  [Function `check_fast_path_limit`](#bridge_bridge_check_fast_path_limit)
-  [Function `claim_stable_token_internal`](#bridge_bridge_claim_stable_token_internal)
-  [Function `execute_emergency_op`](#bridge_bridge_execute_emergency_op)
-  [Function `execute_refund_admin_operate`](#bridge_bridge_execute_refund_admin_operate)
-  [Function `add_refund_admin`](#bridge_bridge_add_refund_admin)
-  [Function `remove_refund_admin`](#bridge_bridge_remove_refund_admin)
-  [Function `execute_update_bridge_limit`](#bridge_bridge_execute_update_bridge_limit)
-  [Function `execute_update_asset_price`](#bridge_bridge_execute_update_asset_price)
-  [Function `execute_add_external_coin_admin`](#bridge_bridge_execute_add_external_coin_admin)
-  [Function `execute_remove_external_coin_admin`](#bridge_bridge_execute_remove_external_coin_admin)
-  [Function `execute_add_external_coin_target_payload`](#bridge_bridge_execute_add_external_coin_target_payload)
-  [Function `execute_remove_external_coin_target_payload`](#bridge_bridge_execute_remove_external_coin_target_payload)
-  [Function `execute_add_external_coin_witness`](#bridge_bridge_execute_add_external_coin_witness)
-  [Function `execute_remove_external_coin_witness`](#bridge_bridge_execute_remove_external_coin_witness)
-  [Function `execute_set_cross_in`](#bridge_bridge_execute_set_cross_in)
-  [Function `execute_set_cross_out`](#bridge_bridge_execute_set_cross_out)
-  [Function `execute_withdraw_bridge_fee`](#bridge_bridge_execute_withdraw_bridge_fee)
-  [Function `execute_add_token_on_token_list`](#bridge_bridge_execute_add_token_on_token_list)
-  [Function `execute_remove_token_on_token_list`](#bridge_bridge_execute_remove_token_on_token_list)
-  [Function `execute_add_tokens_on_sui`](#bridge_bridge_execute_add_tokens_on_sui)
-  [Function `get_current_seq_num_and_increment`](#bridge_bridge_get_current_seq_num_and_increment)
-  [Function `get_parsed_token_transfer_message`](#bridge_bridge_get_parsed_token_transfer_message)
-  [Function `get_parsed_token_transfer_message_v2`](#bridge_bridge_get_parsed_token_transfer_message_v2)


<pre><code><b>use</b> <a href="../bfc_system/auth_utils.md#bfc_system_auth_utils">bfc_system::auth_utils</a>;
<b>use</b> <a href="../bfc_system/bars.md#bfc_system_bars">bfc_system::bars</a>;
<b>use</b> <a href="../bfc_system/baud.md#bfc_system_baud">bfc_system::baud</a>;
<b>use</b> <a href="../bfc_system/bbrl.md#bfc_system_bbrl">bfc_system::bbrl</a>;
<b>use</b> <a href="../bfc_system/bcad.md#bfc_system_bcad">bfc_system::bcad</a>;
<b>use</b> <a href="../bfc_system/beur.md#bfc_system_beur">bfc_system::beur</a>;
<b>use</b> <a href="../bfc_system/bfc_dao.md#bfc_system_bfc_dao">bfc_system::bfc_dao</a>;
<b>use</b> <a href="../bfc_system/bfc_dao_manager.md#bfc_system_bfc_dao_manager">bfc_system::bfc_dao_manager</a>;
<b>use</b> <a href="../bfc_system/bfc_system.md#bfc_system_bfc_system">bfc_system::bfc_system</a>;
<b>use</b> <a href="../bfc_system/bfc_system_state_inner.md#bfc_system_bfc_system_state_inner">bfc_system::bfc_system_state_inner</a>;
<b>use</b> <a href="../bfc_system/bgbp.md#bfc_system_bgbp">bfc_system::bgbp</a>;
<b>use</b> <a href="../bfc_system/bidr.md#bfc_system_bidr">bfc_system::bidr</a>;
<b>use</b> <a href="../bfc_system/binr.md#bfc_system_binr">bfc_system::binr</a>;
<b>use</b> <a href="../bfc_system/bjpy.md#bfc_system_bjpy">bfc_system::bjpy</a>;
<b>use</b> <a href="../bfc_system/bkrw.md#bfc_system_bkrw">bfc_system::bkrw</a>;
<b>use</b> <a href="../bfc_system/bmxn.md#bfc_system_bmxn">bfc_system::bmxn</a>;
<b>use</b> <a href="../bfc_system/brub.md#bfc_system_brub">bfc_system::brub</a>;
<b>use</b> <a href="../bfc_system/bsar.md#bfc_system_bsar">bfc_system::bsar</a>;
<b>use</b> <a href="../bfc_system/btry.md#bfc_system_btry">bfc_system::btry</a>;
<b>use</b> <a href="../bfc_system/busd.md#bfc_system_busd">bfc_system::busd</a>;
<b>use</b> <a href="../bfc_system/bzar.md#bfc_system_bzar">bfc_system::bzar</a>;
<b>use</b> <a href="../bfc_system/clmm_math.md#bfc_system_clmm_math">bfc_system::clmm_math</a>;
<b>use</b> <a href="../bfc_system/comparator.md#bfc_system_comparator">bfc_system::comparator</a>;
<b>use</b> <a href="../bfc_system/event.md#bfc_system_event">bfc_system::event</a>;
<b>use</b> <a href="../bfc_system/full_math_u128.md#bfc_system_full_math_u128">bfc_system::full_math_u128</a>;
<b>use</b> <a href="../bfc_system/i128.md#bfc_system_i128">bfc_system::i128</a>;
<b>use</b> <a href="../bfc_system/i32.md#bfc_system_i32">bfc_system::i32</a>;
<b>use</b> <a href="../bfc_system/i64.md#bfc_system_i64">bfc_system::i64</a>;
<b>use</b> <a href="../bfc_system/linked_table.md#bfc_system_linked_table">bfc_system::linked_table</a>;
<b>use</b> <a href="../bfc_system/math_u128.md#bfc_system_math_u128">bfc_system::math_u128</a>;
<b>use</b> <a href="../bfc_system/math_u256.md#bfc_system_math_u256">bfc_system::math_u256</a>;
<b>use</b> <a href="../bfc_system/math_u64.md#bfc_system_math_u64">bfc_system::math_u64</a>;
<b>use</b> <a href="../bfc_system/mgg.md#bfc_system_mgg">bfc_system::mgg</a>;
<b>use</b> <a href="../bfc_system/option_u64.md#bfc_system_option_u64">bfc_system::option_u64</a>;
<b>use</b> <a href="../bfc_system/position.md#bfc_system_position">bfc_system::position</a>;
<b>use</b> <a href="../bfc_system/random.md#bfc_system_random">bfc_system::random</a>;
<b>use</b> <a href="../bfc_system/skip_list.md#bfc_system_skip_list">bfc_system::skip_list</a>;
<b>use</b> <a href="../bfc_system/tick.md#bfc_system_tick">bfc_system::tick</a>;
<b>use</b> <a href="../bfc_system/tick_math.md#bfc_system_tick_math">bfc_system::tick_math</a>;
<b>use</b> <a href="../bfc_system/treasury.md#bfc_system_treasury">bfc_system::treasury</a>;
<b>use</b> <a href="../bfc_system/treasury_pool.md#bfc_system_treasury_pool">bfc_system::treasury_pool</a>;
<b>use</b> <a href="../bfc_system/utils.md#bfc_system_utils">bfc_system::utils</a>;
<b>use</b> <a href="../bfc_system/vault.md#bfc_system_vault">bfc_system::vault</a>;
<b>use</b> <a href="../bfc_system/bfc_dao_voting_pool.md#bfc_system_voting_pool">bfc_system::voting_pool</a>;
<b>use</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee">bridge::bridge_fee</a>;
<b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../bridge/committee.md#bridge_committee">bridge::committee</a>;
<b>use</b> <a href="../bridge/crypto.md#bridge_crypto">bridge::crypto</a>;
<b>use</b> <a href="../bridge/limiter.md#bridge_limiter">bridge::limiter</a>;
<b>use</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path">bridge::limiter_fast_path</a>;
<b>use</b> <a href="../bridge/message.md#bridge_message">bridge::message</a>;
<b>use</b> <a href="../bridge/message_types.md#bridge_message_types">bridge::message_types</a>;
<b>use</b> <a href="../bridge/tokenlist.md#bridge_tokenlist">bridge::tokenlist</a>;
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
<b>use</b> <a href="../sui/bfc.md#sui_bfc">sui::bfc</a>;
<b>use</b> <a href="../sui/clock.md#sui_clock">sui::clock</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/curve.md#sui_curve">sui::curve</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/ecdsa_k1.md#sui_ecdsa_k1">sui::ecdsa_k1</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/linked_table.md#sui_linked_table">sui::linked_table</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_bag.md#sui_object_bag">sui::object_bag</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
<b>use</b> <a href="../sui/pay.md#sui_pay">sui::pay</a>;
<b>use</b> <a href="../sui/priority_queue.md#sui_priority_queue">sui::priority_queue</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/table_vec.md#sui_table_vec">sui::table_vec</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
<b>use</b> <a href="../sui/versioned.md#sui_versioned">sui::versioned</a>;
<b>use</b> <a href="../sui_system/stable_pool.md#sui_system_stable_pool">sui_system::stable_pool</a>;
<b>use</b> <a href="../sui_system/stake_subsidy.md#sui_system_stake_subsidy">sui_system::stake_subsidy</a>;
<b>use</b> <a href="../sui_system/staking_pool.md#sui_system_staking_pool">sui_system::staking_pool</a>;
<b>use</b> <a href="../sui_system/storage_fund.md#sui_system_storage_fund">sui_system::storage_fund</a>;
<b>use</b> <a href="../sui_system/sui_system.md#sui_system_sui_system">sui_system::sui_system</a>;
<b>use</b> <a href="../sui_system/sui_system_state_inner.md#sui_system_sui_system_state_inner">sui_system::sui_system_state_inner</a>;
<b>use</b> <a href="../sui_system/validator.md#sui_system_validator">sui_system::validator</a>;
<b>use</b> <a href="../sui_system/validator_cap.md#sui_system_validator_cap">sui_system::validator_cap</a>;
<b>use</b> <a href="../sui_system/validator_set.md#sui_system_validator_set">sui_system::validator_set</a>;
<b>use</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper">sui_system::validator_wrapper</a>;
<b>use</b> <a href="../sui_system/voting_power.md#sui_system_voting_power">sui_system::voting_power</a>;
</code></pre>



<a name="bridge_bridge_Bridge"></a>

## Struct `Bridge`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a> <b>has</b> key
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
<code>inner: <a href="../sui/versioned.md#sui_versioned_Versioned">sui::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_BridgeInner"></a>

## Struct `BridgeInner`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_version: u64</code>
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
<code>sequence_nums: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;u8, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/committee.md#bridge_committee">committee</a>: <a href="../bridge/committee.md#bridge_committee_BridgeCommittee">bridge::committee::BridgeCommittee</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury">treasury</a>: <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a></code>
</dt>
<dd>
</dd>
<dt>
<code>token_transfer_records: <a href="../sui/linked_table.md#sui_linked_table_LinkedTable">sui::linked_table::LinkedTable</a>&lt;<a href="../bridge/message.md#bridge_message_BridgeMessageKey">bridge::message::BridgeMessageKey</a>, <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">bridge::bridge::BridgeRecord</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>external_bridge_records: <a href="../sui/linked_table.md#sui_linked_table_LinkedTable">sui::linked_table::LinkedTable</a>&lt;<a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">bridge::bridge::ExternalBridgeMessageKey</a>, <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeRecord">bridge::bridge::ExternalBridgeRecord</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>pre_deposit_multi_signature_records: <a href="../sui/linked_table.md#sui_linked_table_LinkedTable">sui::linked_table::LinkedTable</a>&lt;<a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">bridge::bridge::ExternalBridgeMessageKey</a>, <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/limiter.md#bridge_limiter">limiter</a>: <a href="../bridge/limiter.md#bridge_limiter_TransferLimiter">bridge::limiter::TransferLimiter</a></code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>refund_records: <a href="../sui/linked_table.md#sui_linked_table_LinkedTable">sui::linked_table::LinkedTable</a>&lt;<a href="../bridge/message.md#bridge_message_RefundMessageKey">bridge::message::RefundMessageKey</a>, <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">bridge::bridge::BridgeRecord</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>refund_admins: <a href="../sui/vec_set.md#sui_vec_set_VecSet">sui::vec_set::VecSet</a>&lt;<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_TokenDepositedEvent"></a>

## Struct `TokenDepositedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenDepositedEvent">TokenDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seq_num: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u64</code>
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

<a name="bridge_bridge_TokenDepositedEventV2"></a>

## Struct `TokenDepositedEventV2`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenDepositedEventV2">TokenDepositedEventV2</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seq_num: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_before_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_after_fee: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_TokenSendBackEvent"></a>

## Struct `TokenSendBackEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenSendBackEvent">TokenSendBackEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seq_num: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tx_hash: vector&lt;u8&gt;</code>
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

<a name="bridge_bridge_TokenSendBackEventV2"></a>

## Struct `TokenSendBackEventV2`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenSendBackEventV2">TokenSendBackEventV2</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seq_num: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tx_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>event_idx: u16</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_EmergencyOpEvent"></a>

## Struct `EmergencyOpEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_EmergencyOpEvent">EmergencyOpEvent</a> <b>has</b> <b>copy</b>, drop
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

<a name="bridge_bridge_BridgeRecord"></a>

## Struct `BridgeRecord`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a></code>
</dt>
<dd>
</dd>
<dt>
<code>verified_signatures: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;</code>
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

<a name="bridge_bridge_TokenTransferApproved"></a>

## Struct `TokenTransferApproved`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenTransferApproved">TokenTransferApproved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="../bridge/message.md#bridge_message_BridgeMessageKey">bridge::message::BridgeMessageKey</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_TokenTransferClaimed"></a>

## Struct `TokenTransferClaimed`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenTransferClaimed">TokenTransferClaimed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="../bridge/message.md#bridge_message_BridgeMessageKey">bridge::message::BridgeMessageKey</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_TokenTransferAlreadyApproved"></a>

## Struct `TokenTransferAlreadyApproved`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="../bridge/message.md#bridge_message_BridgeMessageKey">bridge::message::BridgeMessageKey</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_TokenTransferAlreadyClaimed"></a>

## Struct `TokenTransferAlreadyClaimed`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyClaimed">TokenTransferAlreadyClaimed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="../bridge/message.md#bridge_message_BridgeMessageKey">bridge::message::BridgeMessageKey</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_TokenTransferLimitExceed"></a>

## Struct `TokenTransferLimitExceed`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_TokenTransferLimitExceed">TokenTransferLimitExceed</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key: <a href="../bridge/message.md#bridge_message_BridgeMessageKey">bridge::message::BridgeMessageKey</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_ExternalPreDepositedEvent"></a>

## Struct `ExternalPreDepositedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalPreDepositedEvent">ExternalPreDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>signatures: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_ExternalPreDepositedDoneEvent"></a>

## Struct `ExternalPreDepositedDoneEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalPreDepositedDoneEvent">ExternalPreDepositedDoneEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
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

<a name="bridge_bridge_ExternalDepositedApprovedEvent"></a>

## Struct `ExternalDepositedApprovedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedApprovedEvent">ExternalDepositedApprovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
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

<a name="bridge_bridge_ExternalDepositStartEvent"></a>

## Struct `ExternalDepositStartEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositStartEvent">ExternalDepositStartEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seq_num: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>token_id: u64</code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
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

<a name="bridge_bridge_ExternalDepositedEvent"></a>

## Struct `ExternalDepositedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedEvent">ExternalDepositedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
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

<a name="bridge_bridge_ExternalDepositedEventV2"></a>

## Struct `ExternalDepositedEventV2`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedEventV2">ExternalDepositedEventV2</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>token_type: u64</code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_before_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_after_fee: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_ExternalWithdrawEvent"></a>

## Struct `ExternalWithdrawEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalWithdrawEvent">ExternalWithdrawEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
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

<a name="bridge_bridge_ExternalWithdrawEventV2"></a>

## Struct `ExternalWithdrawEventV2`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalWithdrawEventV2">ExternalWithdrawEventV2</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>token_type: u64</code>
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_before_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>amount_after_fee: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_ExternalBridgeMessageKey"></a>

## Struct `ExternalBridgeMessageKey`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a> <b>has</b> <b>copy</b>, drop, store
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_ExternalBridgeRecord"></a>

## Struct `ExternalBridgeRecord`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeRecord">ExternalBridgeRecord</a> <b>has</b> drop, store
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
<code>source_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>target_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>verified_signatures: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;</code>
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


<a name="bridge_bridge_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_CURRENT_VERSION">CURRENT_VERSION</a>: u64 = 1;
</code></pre>



<a name="bridge_bridge_EBridgeAlreadyPaused"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EBridgeAlreadyPaused">EBridgeAlreadyPaused</a>: u64 = 13;
</code></pre>



<a name="bridge_bridge_EBridgeNotPaused"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EBridgeNotPaused">EBridgeNotPaused</a>: u64 = 14;
</code></pre>



<a name="bridge_bridge_EBridgeUnavailable"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>: u64 = 8;
</code></pre>



<a name="bridge_bridge_EDuplicateRefund"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EDuplicateRefund">EDuplicateRefund</a>: u64 = 22;
</code></pre>



<a name="bridge_bridge_EDuplicatedMessage"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EDuplicatedMessage">EDuplicatedMessage</a>: u64 = 30;
</code></pre>



<a name="bridge_bridge_EFastPathLimitError"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EFastPathLimitError">EFastPathLimitError</a>: u64 = 51;
</code></pre>



<a name="bridge_bridge_EInputAmountLteBridgeFee"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>: u64 = 36;
</code></pre>



<a name="bridge_bridge_EInvalidBridgeRoute"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>: u64 = 16;
</code></pre>



<a name="bridge_bridge_EInvalidChainIDAndTokenIDExpect"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>: u64 = 34;
</code></pre>



<a name="bridge_bridge_EInvalidChainIDOnTokenList"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDOnTokenList">EInvalidChainIDOnTokenList</a>: u64 = 35;
</code></pre>



<a name="bridge_bridge_EInvalidEvmAddress"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>: u64 = 18;
</code></pre>



<a name="bridge_bridge_EInvalidMinStakeParticipationPercentage"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidMinStakeParticipationPercentage">EInvalidMinStakeParticipationPercentage</a>: u64 = 50;
</code></pre>



<a name="bridge_bridge_EInvalidMintAmount"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidMintAmount">EInvalidMintAmount</a>: u64 = 41;
</code></pre>



<a name="bridge_bridge_EInvalidSender"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidSender">EInvalidSender</a>: u64 = 20;
</code></pre>



<a name="bridge_bridge_EInvalidTokenIdExpect"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidTokenIdExpect">EInvalidTokenIdExpect</a>: u64 = 23;
</code></pre>



<a name="bridge_bridge_EInvalidTxHash"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidTxHash">EInvalidTxHash</a>: u64 = 21;
</code></pre>



<a name="bridge_bridge_EInvariantSuiInitializedTokenTransferShouldNotBeClaimed"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EInvariantSuiInitializedTokenTransferShouldNotBeClaimed">EInvariantSuiInitializedTokenTransferShouldNotBeClaimed</a>: u64 = 10;
</code></pre>



<a name="bridge_bridge_EMalformedMessageError"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EMalformedMessageError">EMalformedMessageError</a>: u64 = 2;
</code></pre>



<a name="bridge_bridge_EMessageNotFoundInRecords"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EMessageNotFoundInRecords">EMessageNotFoundInRecords</a>: u64 = 11;
</code></pre>



<a name="bridge_bridge_EMustBeTokenMessage"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>: u64 = 17;
</code></pre>



<a name="bridge_bridge_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_ENotSystemAddress">ENotSystemAddress</a>: u64 = 5;
</code></pre>



<a name="bridge_bridge_EOnlySupportBusd"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EOnlySupportBusd">EOnlySupportBusd</a>: u64 = 24;
</code></pre>



<a name="bridge_bridge_EOnlySupportTokenTransferIn"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EOnlySupportTokenTransferIn">EOnlySupportTokenTransferIn</a>: u64 = 52;
</code></pre>



<a name="bridge_bridge_ETokenAlreadyClaimedOrHitLimit"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_ETokenAlreadyClaimedOrHitLimit">ETokenAlreadyClaimedOrHitLimit</a>: u64 = 15;
</code></pre>



<a name="bridge_bridge_ETokenValueIsZero"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>: u64 = 19;
</code></pre>



<a name="bridge_bridge_ETransferLimit"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_ETransferLimit">ETransferLimit</a>: u64 = 55;
</code></pre>



<a name="bridge_bridge_EUnauthorisedClaim"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>: u64 = 1;
</code></pre>



<a name="bridge_bridge_EUnauthorisedUpdateLimit"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnauthorisedUpdateLimit">EUnauthorisedUpdateLimit</a>: u64 = 40;
</code></pre>



<a name="bridge_bridge_EUnexpectedChainID"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>: u64 = 4;
</code></pre>



<a name="bridge_bridge_EUnexpectedMessageType"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>: u64 = 0;
</code></pre>



<a name="bridge_bridge_EUnexpectedMessageVersion"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>: u64 = 12;
</code></pre>



<a name="bridge_bridge_EUnexpectedOperation"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedOperation">EUnexpectedOperation</a>: u64 = 9;
</code></pre>



<a name="bridge_bridge_EUnexpectedSeqNum"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedSeqNum">EUnexpectedSeqNum</a>: u64 = 6;
</code></pre>



<a name="bridge_bridge_EUnexpectedTokenType"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedTokenType">EUnexpectedTokenType</a>: u64 = 3;
</code></pre>



<a name="bridge_bridge_EUnknownExternalCoinOrSender"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnknownExternalCoinOrSender">EUnknownExternalCoinOrSender</a>: u64 = 31;
</code></pre>



<a name="bridge_bridge_EUnpassedMultiSignature"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnpassedMultiSignature">EUnpassedMultiSignature</a>: u64 = 32;
</code></pre>



<a name="bridge_bridge_EUnpassedWitnessSignature"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUnpassedWitnessSignature">EUnpassedWitnessSignature</a>: u64 = 33;
</code></pre>



<a name="bridge_bridge_EUseClaimBusd"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUseClaimBusd">EUseClaimBusd</a>: u64 = 26;
</code></pre>



<a name="bridge_bridge_EUseSendBusd"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EUseSendBusd">EUseSendBusd</a>: u64 = 25;
</code></pre>



<a name="bridge_bridge_EVM_ADDRESS_LENGTH"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>: u64 = 20;
</code></pre>



<a name="bridge_bridge_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 7;
</code></pre>



<a name="bridge_bridge_MESSAGE_VERSION"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>: u8 = 1;
</code></pre>



<a name="bridge_bridge_MESSAGE_VERSION_V2"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION_V2">MESSAGE_VERSION_V2</a>: u8 = 2;
</code></pre>



<a name="bridge_bridge_TOKEN_ID_USDC"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_TOKEN_ID_USDC">TOKEN_ID_USDC</a>: u64 = 3;
</code></pre>



<a name="bridge_bridge_TOKEN_ID_USDT"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_TOKEN_ID_USDT">TOKEN_ID_USDT</a>: u64 = 4;
</code></pre>



<a name="bridge_bridge_TRANSFER_STATUS_APPROVED"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_APPROVED">TRANSFER_STATUS_APPROVED</a>: u8 = 1;
</code></pre>



<a name="bridge_bridge_TRANSFER_STATUS_CLAIMED"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>: u8 = 2;
</code></pre>



<a name="bridge_bridge_TRANSFER_STATUS_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>: u8 = 3;
</code></pre>



<a name="bridge_bridge_TRANSFER_STATUS_PENDING"></a>



<pre><code><b>const</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>: u8 = 0;
</code></pre>



<a name="bridge_bridge_create"></a>

## Function `create`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_create">create</a>(id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u8, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_create">create</a>(id: UID, chain_id: u8, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../bridge/bridge.md#bridge_bridge_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> bridge_inner = <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> {
        bridge_version: <a href="../bridge/bridge.md#bridge_bridge_CURRENT_VERSION">CURRENT_VERSION</a>,
        message_version: <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>,
        chain_id,
        sequence_nums: vec_map::empty(),
        <a href="../bridge/committee.md#bridge_committee">committee</a>: <a href="../bridge/committee.md#bridge_committee_create">committee::create</a>(ctx),
        <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: <a href="../bridge/treasury.md#bridge_treasury_create">treasury::create</a>(ctx),
        token_transfer_records: linked_table::new(ctx),
        external_bridge_records: linked_table::new(ctx),
        pre_deposit_multi_signature_records: linked_table::new(ctx),
        <a href="../bridge/limiter.md#bridge_limiter">limiter</a>: <a href="../bridge/limiter.md#bridge_limiter_new">limiter::new</a>(),
        paused: <b>false</b>,
        refund_records: linked_table::new(ctx),
        refund_admins: vec_set::empty(),
    };
    <b>let</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a> = <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a> {
        id,
        inner: versioned::create(<a href="../bridge/bridge.md#bridge_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, bridge_inner, ctx)
    };
    transfer::share_object(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
}
</code></pre>



</details>

<a name="bridge_bridge_init_bridge_committee"></a>

## Function `init_bridge_committee`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_init_bridge_committee">init_bridge_committee</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, active_validator_voting_power: <a href="../sui/vec_map.md#sui_vec_map_VecMap">sui::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;, min_stake_participation_percentage: u64, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_init_bridge_committee">init_bridge_committee</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    active_validator_voting_power: VecMap&lt;<b>address</b>, u64&gt;,
    min_stake_participation_percentage: u64,
    ctx: &TxContext
) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../bridge/bridge.md#bridge_bridge_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(min_stake_participation_percentage&gt;=7500, <a href="../bridge/bridge.md#bridge_bridge_EInvalidMinStakeParticipationPercentage">EInvalidMinStakeParticipationPercentage</a>);
    <b>if</b> (inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.committee_members().is_empty()) {
        inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.try_create_next_committee(
            active_validator_voting_power,
            min_stake_participation_percentage,
            ctx,
        )
    }
}
</code></pre>



</details>

<a name="bridge_bridge_migrate"></a>

## Function `migrate`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_migrate">migrate</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_migrate">migrate</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    ctx: &<b>mut</b> TxContext
){
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_new_bridge_fee_registry">bridge_fee::new_bridge_fee_registry</a>(&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id, ctx);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_registry">limiter_fast_path::registry</a>(&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id, ctx);
    <a href="../bridge/limiter.md#bridge_limiter_new_external_limits">limiter::new_external_limits</a>(&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id, ctx);
}
</code></pre>



</details>

<a name="bridge_bridge_init_token_list"></a>

## Function `init_token_list`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_init_token_list">init_token_list</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_init_token_list">init_token_list</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    ctx: &<b>mut</b> TxContext
){
    <a href="../bridge/tokenlist.md#bridge_tokenlist_new_tokenlist_registry">tokenlist::new_tokenlist_registry</a>(&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id, ctx);
    <a href="../bridge/tokenlist.md#bridge_tokenlist_add_center_token_list">tokenlist::add_center_token_list</a>(&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id, ctx);
    <a href="../bridge/limiter.md#bridge_limiter_update_transfer_limits">limiter::update_transfer_limits</a>(&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>).<a href="../bridge/limiter.md#bridge_limiter">limiter</a>);
}
</code></pre>



</details>

<a name="bridge_bridge_committee_registration"></a>

## Function `committee_registration`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_committee_registration">committee_registration</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, system_state: &<b>mut</b> <a href="../sui_system/sui_system.md#sui_system_sui_system_SuiSystemState">sui_system::sui_system::SuiSystemState</a>, bridge_pubkey_bytes: vector&lt;u8&gt;, http_rest_url: vector&lt;u8&gt;, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_committee_registration">committee_registration</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    system_state: &<b>mut</b> SuiSystemState,
    bridge_pubkey_bytes: vector&lt;u8&gt;,
    http_rest_url: vector&lt;u8&gt;,
    ctx: &TxContext
) {
    <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>)
        .<a href="../bridge/committee.md#bridge_committee">committee</a>
        .register(system_state, bridge_pubkey_bytes, http_rest_url, ctx);
}
</code></pre>



</details>

<a name="bridge_bridge_update_node_url"></a>

## Function `update_node_url`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_update_node_url">update_node_url</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, new_url: vector&lt;u8&gt;, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_update_node_url">update_node_url</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>, new_url: vector&lt;u8&gt;, ctx: &TxContext) {
    <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>).<a href="../bridge/committee.md#bridge_committee">committee</a>.<a href="../bridge/bridge.md#bridge_bridge_update_node_url">update_node_url</a>(new_url, ctx);
}
</code></pre>



</details>

<a name="bridge_bridge_register_foreign_token"></a>

## Function `register_foreign_token`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_register_foreign_token">register_foreign_token</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, tc: <a href="../sui/coin.md#sui_coin_TreasuryCap">sui::coin::TreasuryCap</a>&lt;T&gt;, uc: <a href="../sui/package.md#sui_package_UpgradeCap">sui::package::UpgradeCap</a>, metadata: &<a href="../sui/coin.md#sui_coin_CoinMetadata">sui::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_register_foreign_token">register_foreign_token</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    tc: TreasuryCap&lt;T&gt;,
    uc: UpgradeCap,
    metadata: &CoinMetadata&lt;T&gt;,
) {
    <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>)
        .<a href="../bridge/treasury.md#bridge_treasury">treasury</a>
        .<a href="../bridge/bridge.md#bridge_bridge_register_foreign_token">register_foreign_token</a>&lt;T&gt;(tc, uc, metadata)
}
</code></pre>



</details>

<a name="bridge_bridge_send_token"></a>

## Function `send_token`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_token">send_token</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, target_chain: u8, target_address: vector&lt;u8&gt;, token: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_token">send_token</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: vector&lt;u8&gt;,
    <b>mut</b> token: Coin&lt;T&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>assert</b>!(target_address.length() == <a href="../bridge/bridge.md#bridge_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>, <a href="../bridge/bridge.md#bridge_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>);
    <b>let</b> bridge_seq_num = inner.<a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>());
    <b>let</b> token_id = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.token_id&lt;T&gt;();
    <b>let</b> token_amount = token.balance().value();
    <b>assert</b>!(token_amount &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>assert</b>!(token_id != 5, <a href="../bridge/bridge.md#bridge_bridge_EUseSendBusd">EUseSendBusd</a>);
    <b>assert</b>!(<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen">tokenlist::is_supported_from_benfen</a>(parent_id, target_chain <b>as</b> u64, token_id),<a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>);
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">bridge_fee::calculate_cross_out_fee_amount</a>(parent_id,target_chain <b>as</b> u64,token_id,token_amount);
    <b>assert</b>!(token_amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>let</b> fee_coin=token.split&lt;T&gt;(fee, ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(parent_id, fee_coin);
    <b>let</b> amount_after_fee=token_amount-fee;
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(inner.chain_id, target_chain);
    <b>let</b> amount_in_usd = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.calculate_amount_in_usd&lt;T&gt;(amount_after_fee);
    <b>assert</b>!(amount_in_usd &lt; <a href="../bridge/limiter.md#bridge_limiter_get_external_out_limit">limiter::get_external_out_limit</a>(parent_id, &route), <a href="../bridge/bridge.md#bridge_bridge_ETransferLimit">ETransferLimit</a>);
    // <a href="../bridge/bridge.md#bridge_bridge_create">create</a> <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> <a href="../bridge/message.md#bridge_message">message</a> = <a href="../bridge/message.md#bridge_message_create_token_bridge_message_v2">message::create_token_bridge_message_v2</a>(
        inner.chain_id,
        bridge_seq_num,
        address::to_bytes(ctx.sender()),
        target_chain,
        target_address,
        token_id,
        amount_after_fee,
        hex::decode(b""),
        0u16, // event_idx
    );
    // burn / escrow token, unsupported coins will fail in this step
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.burn(token);
    // Store pending <a href="../bridge/bridge.md#bridge_bridge">bridge</a> request
    inner.token_transfer_records.push_back(
        <a href="../bridge/message.md#bridge_message">message</a>.key(),
        <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="../bridge/message.md#bridge_message">message</a>,
            verified_signatures: option::none(),
            claimed: <b>false</b>,
        },
    );
    // emit event
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_TokenDepositedEventV2">TokenDepositedEventV2</a> {
            seq_num: bridge_seq_num,
            source_chain: inner.chain_id,
            sender_address: address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_type: token_id,
            amount_before_fee: token_amount,
            amount_after_fee,
        },
    );
}
</code></pre>



</details>

<a name="bridge_bridge_send_busd"></a>

## Function `send_busd`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_busd">send_busd</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, bfc_system_state: &<b>mut</b> <a href="../bfc_system/bfc_system.md#bfc_system_bfc_system_BfcSystemState">bfc_system::bfc_system::BfcSystemState</a>, target_chain: u8, target_address: vector&lt;u8&gt;, token: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;, token_id_expect: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_busd">send_busd</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    target_chain: u8,
    target_address: vector&lt;u8&gt;,
    <b>mut</b> token: Coin&lt;T&gt;,
    token_id_expect: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner,bridge_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen">tokenlist::is_supported_from_benfen</a>(bridge_id, target_chain <b>as</b> u64, token_id_expect),<a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>assert</b>!(target_address.length() == <a href="../bridge/bridge.md#bridge_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>, <a href="../bridge/bridge.md#bridge_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>);
    <b>let</b> is_busd = type_name::get&lt;T&gt;() == type_name::get&lt;BUSD&gt;();
    <b>assert</b>!(is_busd, <a href="../bridge/bridge.md#bridge_bridge_EOnlySupportBusd">EOnlySupportBusd</a>);
    <b>assert</b>!(token_id_expect == 3 || token_id_expect == 4, <a href="../bridge/bridge.md#bridge_bridge_EInvalidTokenIdExpect">EInvalidTokenIdExpect</a>);
    <b>let</b> bridge_seq_num = inner.<a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>());
    // <b>let</b> token_id_origin = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.token_id&lt;T&gt;();
    // <b>assert</b>!(token_id_origin == 5, <a href="../bridge/bridge.md#bridge_bridge_EOnlySupportBusd">EOnlySupportBusd</a>);
    <b>let</b> token_id = token_id_expect;
    <b>let</b> token_amount=<b>if</b> (target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() || target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() || target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>()) {
         token.balance().value()/1000u64
    }<b>else</b>{
         token.balance().value()
    };
    <b>assert</b>!(token_amount &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">bridge_fee::calculate_cross_out_fee_amount</a>(bridge_id,target_chain <b>as</b> u64,token_id,token_amount);
    <b>assert</b>!(token_amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>let</b> amount_after_fee=token_amount-fee;
    <b>let</b> fee_coin=token.split&lt;T&gt;(fee, ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(bridge_id, fee_coin);
    // <a href="../bridge/bridge.md#bridge_bridge_create">create</a> <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> <a href="../bridge/message.md#bridge_message">message</a> = <a href="../bridge/message.md#bridge_message_create_token_bridge_message_v2">message::create_token_bridge_message_v2</a>(
        inner.chain_id,
        bridge_seq_num,
        address::to_bytes(ctx.sender()),
        target_chain,
        target_address,
        token_id,
        amount_after_fee,
        hex::decode(b""),
        0u16, // event_idx
    );
    // burn / escrow token, unsupported coins will fail in this step
    bfc_system_state.burn_stable(token, ctx);
    // Store pending <a href="../bridge/bridge.md#bridge_bridge">bridge</a> request
    inner.token_transfer_records.push_back(
        <a href="../bridge/message.md#bridge_message">message</a>.key(),
        <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="../bridge/message.md#bridge_message">message</a>,
            verified_signatures: option::none(),
            claimed: <b>false</b>,
        },
    );
    // emit event
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_TokenDepositedEventV2">TokenDepositedEventV2</a> {
            seq_num: bridge_seq_num,
            source_chain: inner.chain_id,
            sender_address: address::to_bytes(ctx.sender()),
            target_chain,
            target_address,
            token_type: token_id,
            amount_before_fee: token_amount,
            amount_after_fee,
        },
    );
}
</code></pre>



</details>

<a name="bridge_bridge_send_back_token"></a>

## Function `send_back_token`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_back_token">send_back_token</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, target_chain: u8, target_address: vector&lt;u8&gt;, token_type: u64, token_amount: u64, tx_hash: vector&lt;u8&gt;, event_idx: u8, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_back_token">send_back_token</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: vector&lt;u8&gt;,
    token_type: u64,
    token_amount: u64,
    tx_hash: vector&lt;u8&gt;,
    event_idx: u8,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>assert</b>!(!inner.refund_records.contains(<a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash)), <a href="../bridge/bridge.md#bridge_bridge_EDuplicateRefund">EDuplicateRefund</a>);
    <b>assert</b>!(target_address.length() == <a href="../bridge/bridge.md#bridge_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>, <a href="../bridge/bridge.md#bridge_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>);
    <b>assert</b>!(token_amount &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>assert</b>!(tx_hash.length() &gt;= 1, <a href="../bridge/bridge.md#bridge_bridge_EInvalidTxHash">EInvalidTxHash</a>);
    <b>assert</b>!(inner.<a href="../bridge/bridge.md#bridge_bridge_is_refund_admin">is_refund_admin</a>(ctx.sender().to_ascii_string()), <a href="../bridge/bridge.md#bridge_bridge_EInvalidSender">EInvalidSender</a>);
    <b>let</b> bridge_seq_num = inner.<a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>());
    // <a href="../bridge/bridge.md#bridge_bridge_create">create</a> <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> <a href="../bridge/message.md#bridge_message">message</a> = <a href="../bridge/message.md#bridge_message_create_token_bridge_message">message::create_token_bridge_message</a>(
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
    // Store pending <a href="../bridge/bridge.md#bridge_bridge">bridge</a> request
    inner.token_transfer_records.push_back(
        <a href="../bridge/message.md#bridge_message">message</a>.key(),
        <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="../bridge/message.md#bridge_message">message</a>,
            verified_signatures: option::none(),
            claimed: <b>false</b>,
        },
    );
    //store <b>for</b> idempotency
    inner.refund_records.push_back(
        <a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash),
        <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="../bridge/message.md#bridge_message">message</a>,
            verified_signatures: option::none(),
            claimed: <b>false</b>,
        },
    );
    // emit event
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_TokenSendBackEvent">TokenSendBackEvent</a> {
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

<a name="bridge_bridge_send_back_token_v2"></a>

## Function `send_back_token_v2`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_back_token_v2">send_back_token_v2</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, target_chain: u8, target_address: vector&lt;u8&gt;, token_type: u64, token_amount: u64, tx_hash: vector&lt;u8&gt;, event_idx: u16, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_send_back_token_v2">send_back_token_v2</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: vector&lt;u8&gt;,
    token_type: u64,
    token_amount: u64,
    tx_hash: vector&lt;u8&gt;,
    event_idx: u16,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>assert</b>!(!inner.refund_records.contains(<a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash)), <a href="../bridge/bridge.md#bridge_bridge_EDuplicateRefund">EDuplicateRefund</a>);
    <b>assert</b>!(target_address.length() == <a href="../bridge/bridge.md#bridge_bridge_EVM_ADDRESS_LENGTH">EVM_ADDRESS_LENGTH</a>, <a href="../bridge/bridge.md#bridge_bridge_EInvalidEvmAddress">EInvalidEvmAddress</a>);
    <b>assert</b>!(token_amount &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>assert</b>!(tx_hash.length() &gt;= 1, <a href="../bridge/bridge.md#bridge_bridge_EInvalidTxHash">EInvalidTxHash</a>);
    <b>assert</b>!(inner.<a href="../bridge/bridge.md#bridge_bridge_is_refund_admin">is_refund_admin</a>(ctx.sender().to_ascii_string()), <a href="../bridge/bridge.md#bridge_bridge_EInvalidSender">EInvalidSender</a>);
    <b>let</b> bridge_seq_num = inner.<a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>());
    // <a href="../bridge/bridge.md#bridge_bridge_create">create</a> <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> <a href="../bridge/message.md#bridge_message">message</a> = <a href="../bridge/message.md#bridge_message_create_token_bridge_message_v2">message::create_token_bridge_message_v2</a>(
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
    // Store pending <a href="../bridge/bridge.md#bridge_bridge">bridge</a> request
    inner.token_transfer_records.push_back(
        <a href="../bridge/message.md#bridge_message">message</a>.key(),
        <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="../bridge/message.md#bridge_message">message</a>,
            verified_signatures: option::none(),
            claimed: <b>false</b>,
        },
    );
    //store <b>for</b> idempotency
    inner.refund_records.push_back(
        <a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash),
        <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="../bridge/message.md#bridge_message">message</a>,
            verified_signatures: option::none(),
            claimed: <b>false</b>,
        },
    );
    // emit event
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_TokenSendBackEventV2">TokenSendBackEventV2</a> {
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

<a name="bridge_bridge_is_refund_admin"></a>

## Function `is_refund_admin`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_is_refund_admin">is_refund_admin</a>(inner: &<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, <b>address</b>: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_is_refund_admin">is_refund_admin</a>(inner: &<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, <b>address</b>: String): bool {
    inner.refund_admins.contains(&<b>address</b>)
}
</code></pre>



</details>

<a name="bridge_bridge_approve_token_transfer"></a>

## Function `approve_token_transfer`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approve_token_transfer">approve_token_transfer</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approve_token_transfer">approve_token_transfer</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
) {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    // verify signatures
    inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.verify_signatures(<a href="../bridge/message.md#bridge_message">message</a>, signatures);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_type() == <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(), <a href="../bridge/bridge.md#bridge_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_version() == <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> token_payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_token_bridge_payload();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>assert</b>!(
        <a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id || target_chain == inner.chain_id,
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>,
    );
    <b>let</b> message_key = <a href="../bridge/message.md#bridge_message">message</a>.key();
    // retrieve pending <a href="../bridge/message.md#bridge_message">message</a> <b>if</b> source chain is Sui, the initial <a href="../bridge/message.md#bridge_message">message</a>
    // must exist on chain
    <b>if</b> (<a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id) {
        <b>let</b> record = &<b>mut</b> inner.token_transfer_records[message_key];
        <b>assert</b>!(record.<a href="../bridge/message.md#bridge_message">message</a> == <a href="../bridge/message.md#bridge_message">message</a>, <a href="../bridge/bridge.md#bridge_bridge_EMalformedMessageError">EMalformedMessageError</a>);
        <b>assert</b>!(!record.claimed, <a href="../bridge/bridge.md#bridge_bridge_EInvariantSuiInitializedTokenTransferShouldNotBeClaimed">EInvariantSuiInitializedTokenTransferShouldNotBeClaimed</a>);
        // If record already <b>has</b> verified signatures, it means the <a href="../bridge/message.md#bridge_message">message</a> <b>has</b> been approved
        // Then we exit early.
        <b>if</b> (record.verified_signatures.is_some()) {
            emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        // Store approval
        record.verified_signatures = option::some(signatures)
    } <b>else</b> {
        // At this point, <b>if</b> this <a href="../bridge/message.md#bridge_message">message</a> is in token_transfer_records, we know
        // it's already approved because we only add a <a href="../bridge/message.md#bridge_message">message</a> to token_transfer_records
        // after verifying the signatures
        <b>if</b> (inner.token_transfer_records.contains(message_key)) {
            emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        //idempotency <b>for</b> SendBack and ETHToSui
        <b>let</b> tx_hash = token_payload.token_tx_hash();
        <b>if</b> (inner.refund_records.contains(<a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash))) {
            emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        // Store <a href="../bridge/message.md#bridge_message">message</a> and approval
        inner.token_transfer_records.push_back(
            message_key,
            <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
                <a href="../bridge/message.md#bridge_message">message</a>,
                verified_signatures: option::some(signatures),
                claimed: <b>false</b>
            },
        );
    };
    emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferApproved">TokenTransferApproved</a> { message_key });
}
</code></pre>



</details>

<a name="bridge_bridge_approve_token_transfer_v2"></a>

## Function `approve_token_transfer_v2`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approve_token_transfer_v2">approve_token_transfer_v2</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approve_token_transfer_v2">approve_token_transfer_v2</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
) {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    // verify signatures
    inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.verify_signatures(<a href="../bridge/message.md#bridge_message">message</a>, signatures);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_type() == <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(), <a href="../bridge/bridge.md#bridge_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_version() == <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> token_payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_token_bridge_payload_v2();
    <b>let</b> target_chain = token_payload.token_target_chain_v2();
    <b>assert</b>!(
        <a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id || target_chain == inner.chain_id,
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>,
    );
    <b>let</b> message_key = <a href="../bridge/message.md#bridge_message">message</a>.key();
    // retrieve pending <a href="../bridge/message.md#bridge_message">message</a> <b>if</b> source chain is Sui, the initial <a href="../bridge/message.md#bridge_message">message</a>
    // must exist on chain
    <b>if</b> (<a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id) {
        <b>let</b> record = &<b>mut</b> inner.token_transfer_records[message_key];
        <b>assert</b>!(record.<a href="../bridge/message.md#bridge_message">message</a> == <a href="../bridge/message.md#bridge_message">message</a>, <a href="../bridge/bridge.md#bridge_bridge_EMalformedMessageError">EMalformedMessageError</a>);
        <b>assert</b>!(!record.claimed, <a href="../bridge/bridge.md#bridge_bridge_EInvariantSuiInitializedTokenTransferShouldNotBeClaimed">EInvariantSuiInitializedTokenTransferShouldNotBeClaimed</a>);
        // If record already <b>has</b> verified signatures, it means the <a href="../bridge/message.md#bridge_message">message</a> <b>has</b> been approved
        // Then we exit early.
        <b>if</b> (record.verified_signatures.is_some()) {
            emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        // Store approval
        record.verified_signatures = option::some(signatures)
    } <b>else</b> {
        // At this point, <b>if</b> this <a href="../bridge/message.md#bridge_message">message</a> is in token_transfer_records, we know
        // it's already approved because we only add a <a href="../bridge/message.md#bridge_message">message</a> to token_transfer_records
        // after verifying the signatures
        <b>if</b> (inner.token_transfer_records.contains(message_key)) {
            emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        //idempotency <b>for</b> SendBack and ETHToSui
        <b>let</b> tx_hash = token_payload.token_tx_hash_v2();
        <b>if</b> (inner.refund_records.contains(<a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash))) {
            emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
        };
        // Store <a href="../bridge/message.md#bridge_message">message</a> and approval
        inner.token_transfer_records.push_back(
            message_key,
            <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
                <a href="../bridge/message.md#bridge_message">message</a>,
                verified_signatures: option::some(signatures),
                claimed: <b>false</b>
            },
        );
    };
    emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferApproved">TokenTransferApproved</a> { message_key });
}
</code></pre>



</details>

<a name="bridge_bridge_approve_token_transfer_in"></a>

## Function `approve_token_transfer_in`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approve_token_transfer_in">approve_token_transfer_in</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approve_token_transfer_in">approve_token_transfer_in</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
) {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    // verify signatures
    inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.verify_signatures(<a href="../bridge/message.md#bridge_message">message</a>, signatures);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_type() == <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(), <a href="../bridge/bridge.md#bridge_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_version() == <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION_V2">MESSAGE_VERSION_V2</a>, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> token_payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_token_bridge_in_payload();
    <b>let</b> target_chain = token_payload.token_target_chain_in();
    <b>assert</b>!(
        <a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id || target_chain == inner.chain_id,
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>,
    );
    <b>let</b> message_key = <a href="../bridge/message.md#bridge_message">message</a>.key();
    // retrieve pending <a href="../bridge/message.md#bridge_message">message</a> <b>if</b> source chain is Sui, the initial <a href="../bridge/message.md#bridge_message">message</a>
    // must exist on chain
    //only support token transfer in
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.source_chain() != inner.chain_id, <a href="../bridge/bridge.md#bridge_bridge_EOnlySupportTokenTransferIn">EOnlySupportTokenTransferIn</a>);
    // At this point, <b>if</b> this <a href="../bridge/message.md#bridge_message">message</a> is in token_transfer_records, we know
    // it's already approved because we only add a <a href="../bridge/message.md#bridge_message">message</a> to token_transfer_records
    // after verifying the signatures
    <b>if</b> (inner.token_transfer_records.contains(message_key)) {
        emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
        <b>return</b>
    };
    //idempotency <b>for</b> SendBack and ETHToSui
    <b>let</b> tx_hash = token_payload.token_tx_hash_in();
    <b>if</b> (inner.refund_records.contains(<a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash))) {
            emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyApproved">TokenTransferAlreadyApproved</a> { message_key });
            <b>return</b>
    };
    // Store <a href="../bridge/message.md#bridge_message">message</a> and approval
    inner.token_transfer_records.push_back(
        message_key,
        <a href="../bridge/bridge.md#bridge_bridge_BridgeRecord">BridgeRecord</a> {
            <a href="../bridge/message.md#bridge_message">message</a>,
            verified_signatures: option::some(signatures),
            claimed: <b>false</b>
        },
    );
    emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferApproved">TokenTransferApproved</a> { message_key });
}
</code></pre>



</details>

<a name="bridge_bridge_get_max_mint_busd_amount"></a>

## Function `get_max_mint_busd_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_max_mint_busd_amount">get_max_mint_busd_amount</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_max_mint_busd_amount">get_max_mint_busd_amount</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>): u64 {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    inner.<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.get_mint_busd_max_limit()
}
</code></pre>



</details>

<a name="bridge_bridge_set_max_mint_busd_amount"></a>

## Function `set_max_mint_busd_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_set_max_mint_busd_amount">set_max_mint_busd_amount</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, bfc_system_state: &<a href="../bfc_system/bfc_system.md#bfc_system_bfc_system_BfcSystemState">bfc_system::bfc_system::BfcSystemState</a>, cap: &<a href="../bfc_system/bfc_system_state_inner.md#bfc_system_bfc_system_state_inner_BfcSystemModifyCap">bfc_system::bfc_system_state_inner::BfcSystemModifyCap</a>, new_limit: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_set_max_mint_busd_amount">set_max_mint_busd_amount</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    bfc_system_state: &BfcSystemState,
    cap: &BfcSystemModifyCap,
    new_limit: u64,
    ctx: &<b>mut</b> TxContext,
    ) {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(bfc_system_state.verify_capability(cap, ctx), <a href="../bridge/bridge.md#bridge_bridge_EUnauthorisedUpdateLimit">EUnauthorisedUpdateLimit</a>);
    inner.<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.set_mint_busd_max_limit(new_limit);
}
</code></pre>



</details>

<a name="bridge_bridge_claim_token"></a>

## Function `claim_token`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_token">claim_token</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_token">claim_token</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;T&gt; {
    <b>let</b> (maybe_token, owner) = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.<a href="../bridge/bridge.md#bridge_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(
        clock,
        source_chain,
        bridge_seq_num,
        ctx,
    );
    // Only token owner can claim the token
    <b>assert</b>!(ctx.sender() == owner, <a href="../bridge/bridge.md#bridge_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>);
    <b>assert</b>!(maybe_token.is_some(), <a href="../bridge/bridge.md#bridge_bridge_ETokenAlreadyClaimedOrHitLimit">ETokenAlreadyClaimedOrHitLimit</a>);
    maybe_token.destroy_some()
}
</code></pre>



</details>

<a name="bridge_bridge_claim_and_transfer_token"></a>

## Function `claim_and_transfer_token`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_and_transfer_token">claim_and_transfer_token</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_and_transfer_token">claim_and_transfer_token</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (token, owner) = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.<a href="../bridge/bridge.md#bridge_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(clock, source_chain, bridge_seq_num, ctx);
    <b>if</b> (token.is_some()) {
        transfer::public_transfer(token.destroy_some(), owner)
    } <b>else</b> {
        token.destroy_none();
    };
}
</code></pre>



</details>

<a name="bridge_bridge_claim_and_transfer_busd"></a>

## Function `claim_and_transfer_busd`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_and_transfer_busd">claim_and_transfer_busd</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, bfc_system_state: &<b>mut</b> <a href="../bfc_system/bfc_system.md#bfc_system_bfc_system_BfcSystemState">bfc_system::bfc_system::BfcSystemState</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, cap: &<a href="../bfc_system/bfc_system_state_inner.md#bfc_system_bfc_system_state_inner_BfcSystemModifyCap">bfc_system::bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_and_transfer_busd">claim_and_transfer_busd</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    cap: &BfcSystemModifyCap,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> (token, owner) = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.<a href="../bridge/bridge.md#bridge_bridge_claim_stable_token_internal">claim_stable_token_internal</a>&lt;T&gt;(bfc_system_state, clock, source_chain, bridge_seq_num, cap, ctx);
    <b>if</b> (token.is_some()) {
        transfer::public_transfer(token.destroy_some(), owner)
    } <b>else</b> {
        token.destroy_none();
    };
}
</code></pre>



</details>

<a name="bridge_bridge_execute_system_message_with_ctx"></a>

## Function `execute_system_message_with_ctx`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_system_message_with_ctx">execute_system_message_with_ctx</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_system_message_with_ctx">execute_system_message_with_ctx</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
    ctx: &<b>mut</b> TxContext,
){
     <b>let</b> message_type = <a href="../bridge/message.md#bridge_message">message</a>.message_type();
    // TODO: test version mismatch
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_version() == <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> (inner,bridge_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);
    // check system ops seq number and increment it
    <b>let</b> expected_seq_num = inner.<a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(message_type);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.seq_num() == expected_seq_num, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedSeqNum">EUnexpectedSeqNum</a>);
    inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.verify_signatures(<a href="../bridge/message.md#bridge_message">message</a>, signatures);
    <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_add_token_on_token_list">message_types::add_token_on_token_list</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_add_token_on_token_list_poyload();
        <a href="../bridge/bridge.md#bridge_bridge_execute_add_token_on_token_list">execute_add_token_on_token_list</a>(bridge_id,payload,ctx);
    }<b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_remove_token_on_token_list">message_types::remove_token_on_token_list</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_remove_token_on_token_list_poyload();
        <a href="../bridge/bridge.md#bridge_bridge_execute_remove_token_on_token_list">execute_remove_token_on_token_list</a>(bridge_id,payload);
    }<b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_set_cross_out_bridge_fee">message_types::set_cross_out_bridge_fee</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_set_cross_out_bridge_fee_poyload();
        <a href="../bridge/bridge.md#bridge_bridge_execute_set_cross_out">execute_set_cross_out</a>(bridge_id, payload, ctx)
    }<b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_set_cross_in_bridge_fee">message_types::set_cross_in_bridge_fee</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_set_cross_in_bridge_fee_poyload();
        <a href="../bridge/bridge.md#bridge_bridge_execute_set_cross_in">execute_set_cross_in</a>(bridge_id, payload, ctx)
    }<b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_withdraw_bridge_fee">message_types::withdraw_bridge_fee</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_withdraw_bridge_fee();
        <a href="../bridge/bridge.md#bridge_bridge_execute_withdraw_bridge_fee">execute_withdraw_bridge_fee</a>(payload, ctx)
    }<b>else</b>{
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>
    };
}
</code></pre>



</details>

<a name="bridge_bridge_execute_system_message"></a>

## Function `execute_system_message`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_system_message">execute_system_message</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_system_message">execute_system_message</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
) {
    <b>let</b> message_type = <a href="../bridge/message.md#bridge_message">message</a>.message_type();
    // TODO: test version mismatch
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_version() == <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> (inner, uid) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);
    // check system ops seq number and increment it
    <b>let</b> expected_seq_num = inner.<a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(message_type);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.seq_num() == expected_seq_num, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedSeqNum">EUnexpectedSeqNum</a>);
    inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.verify_signatures(<a href="../bridge/message.md#bridge_message">message</a>, signatures);
    <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_emergency_op">message_types::emergency_op</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_emergency_op_payload();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_emergency_op">execute_emergency_op</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_committee_blocklist">message_types::committee_blocklist</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_blocklist_payload();
        inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.execute_blocklist(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_update_bridge_limit">message_types::update_bridge_limit</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_update_bridge_limit();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_update_bridge_limit">execute_update_bridge_limit</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_update_asset_price">message_types::update_asset_price</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_update_asset_price();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_update_asset_price">execute_update_asset_price</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_admin">message_types::add_external_coin_admin</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_add_external_coin_admin();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_admin">execute_add_external_coin_admin</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_admin">message_types::remove_external_coin_admin</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_remove_external_coin_admin();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_admin">execute_remove_external_coin_admin</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_add_tokens_on_sui">message_types::add_tokens_on_sui</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_add_tokens_on_sui();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_add_tokens_on_sui">execute_add_tokens_on_sui</a>(payload);
    } <b>else</b> <b>if</b> (message_type == <a href="../bridge/message_types.md#bridge_message_types_refund_admin_operate">message_types::refund_admin_operate</a>()) {
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_refund_admin_payload();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_refund_admin_operate">execute_refund_admin_operate</a>(payload);
    } <b>else</b> <b>if</b>  (message_type == <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_witness">message_types::add_external_coin_witness</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_add_witness_poyload();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_witness">execute_add_external_coin_witness</a>(payload);
    }<b>else</b> <b>if</b>  (message_type == <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_witness">message_types::remove_external_coin_witness</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_remove_witness_poyload();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_witness">execute_remove_external_coin_witness</a>(payload);
    }<b>else</b> <b>if</b>  (message_type == <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_target">message_types::add_external_coin_target</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_add_external_target_address_poyload();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_target_payload">execute_add_external_coin_target_payload</a>(payload);
    }<b>else</b> <b>if</b>  (message_type == <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_target">message_types::remove_external_coin_target</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_remove_external_target_address_poyload();
        inner.<a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_target_payload">execute_remove_external_coin_target_payload</a>(payload);
    }<b>else</b> <b>if</b>  (message_type == <a href="../bridge/message_types.md#bridge_message_types_update_bridge_limit_fast_path">message_types::update_bridge_limit_fast_path</a>()){
        <b>let</b> payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_fast_path_limit_payload();
        <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">limiter_fast_path::add_limiter</a>(uid, payload.chain_id(),payload.token_id(),payload.amount());
    }<b>else</b> {
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>
    };
}
</code></pre>



</details>

<a name="bridge_bridge_get_available_claim_amount"></a>

## Function `get_available_claim_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(
      <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
      source_chain: u8,
): u128 {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(source_chain, inner.chain_id);
    inner.<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.<a href="../bridge/bridge.md#bridge_bridge_get_available_claim_amount">get_available_claim_amount</a>&lt;T&gt;(&inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, route)
}
</code></pre>



</details>

<a name="bridge_bridge_pre_deposit_external_coin"></a>

## Function `pre_deposit_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_pre_deposit_external_coin">pre_deposit_external_coin</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, source_address: vector&lt;u8&gt;, target_address: vector&lt;u8&gt;, amount: u64, tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, signatures: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_pre_deposit_external_coin">pre_deposit_external_coin</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    source_address: vector&lt;u8&gt;,
    target_address: vector&lt;u8&gt;,
    amount: u64,
    tx_hash: ascii::String,
    signatures: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = ctx.sender();
    <b>let</b> sender_str = sender.to_ascii_string();
    <b>let</b> coin_type = type_name::into_string(type_name::get&lt;T&gt;());
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.verify_bitcoin_signatures&lt;T&gt;(source_chain, source_address, target_address, amount, tx_hash, signatures),<a href="../bridge/bridge.md#bridge_bridge_EUnpassedWitnessSignature">EUnpassedWitnessSignature</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(source_chain, inner.chain_id), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>if</b> (!inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.is_external_coin_admin(coin_type, sender_str)) {
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EUnknownExternalCoinOrSender">EUnknownExternalCoinOrSender</a>
    };
    // check then add to pre_deposit_multi_signature_records
    <b>let</b> key = <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
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
        <b>let</b> <b>mut</b> records = vec_set::empty();
        records.insert(sender_str);
        inner.pre_deposit_multi_signature_records.push_back(key, records);
    };
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_ExternalPreDepositedEvent">ExternalPreDepositedEvent</a> {
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
    <b>if</b> (inner.<a href="../bridge/bridge.md#bridge_bridge_multi_signature_passed">multi_signature_passed</a>(key, coin_type)) {
        emit(
            <a href="../bridge/bridge.md#bridge_bridge_ExternalPreDepositedDoneEvent">ExternalPreDepositedDoneEvent</a> {
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

<a name="bridge_bridge_deposit_external_coin"></a>

## Function `deposit_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_deposit_external_coin">deposit_external_coin</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, source_address: vector&lt;u8&gt;, target_address: vector&lt;u8&gt;, amount: u64, tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, signatures: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_deposit_external_coin">deposit_external_coin</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    source_address: vector&lt;u8&gt;,
    target_address: vector&lt;u8&gt;,
    amount: u64,
    tx_hash: ascii::String,
    signatures: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> sender = ctx.sender();
    <b>let</b> coin_type = type_name::into_string(type_name::get&lt;T&gt;());
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.verify_bitcoin_signatures&lt;T&gt;(source_chain, source_address, target_address, amount, tx_hash, signatures),<a href="../bridge/bridge.md#bridge_bridge_EUnpassedWitnessSignature">EUnpassedWitnessSignature</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(source_chain, inner.chain_id), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>if</b> (!inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.is_external_coin_admin(coin_type, sender.to_ascii_string())) {
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EUnknownExternalCoinOrSender">EUnknownExternalCoinOrSender</a>
    };
    <b>let</b> key = <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (!inner.<a href="../bridge/bridge.md#bridge_bridge_multi_signature_passed">multi_signature_passed</a>(key, coin_type)) {
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EUnpassedMultiSignature">EUnpassedMultiSignature</a>
    };
    // check records
    <b>let</b> key = <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (inner.external_bridge_records.contains(key)) {
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EDuplicatedMessage">EDuplicatedMessage</a>
    };
    // // v1
    // <b>let</b> token = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.mint&lt;T&gt;(amount, ctx);
    // transfer::public_transfer(token, address::from_bytes(target_address));
    // inner.external_bridge_records.push_back(
    //     key,
    //     <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeRecord">ExternalBridgeRecord</a> {
    //         source_chain,
    //         target_chain: inner.chain_id,
    //         source_address,
    //         target_address,
    //         amount,
    //         verified_signatures: option::none(),
    //         claimed: <b>true</b>,
    //     },
    // );
    // emit(
    //     <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedEvent">ExternalDepositedEvent</a> {
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
    <b>let</b> seq_num = inner.<a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>());
    <b>let</b> token_id = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.token_id&lt;T&gt;();
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositStartEvent">ExternalDepositStartEvent</a> {
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

<a name="bridge_bridge_approval_and_claimed_external_coin"></a>

## Function `approval_and_claimed_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approval_and_claimed_external_coin">approval_and_claimed_external_coin</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approval_and_claimed_external_coin">approval_and_claimed_external_coin</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    // verify signatures
    inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.verify_signatures(<a href="../bridge/message.md#bridge_message">message</a>, signatures);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_type() == <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(), <a href="../bridge/bridge.md#bridge_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_version() == <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> token_payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_token_bridge_payload();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>assert</b>!(
        <a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id || target_chain == inner.chain_id,
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>,
    );
    <b>let</b> coin_type = type_name::into_string(type_name::get&lt;T&gt;());
    <b>let</b> token_id=<a href="../bridge/treasury.md#bridge_treasury_token_id">treasury::token_id</a>&lt;T&gt;(&inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>);
    // check records
    <b>let</b> tx_hash = ascii::string(token_payload.token_tx_hash());
    <b>let</b> source_chain = <a href="../bridge/message.md#bridge_message">message</a>.source_chain();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>let</b> source_address = token_payload.token_sender_address();
    <b>let</b> target_address = token_payload.token_target_address();
    <b>let</b> amount = token_payload.token_amount();
    <b>let</b> key = <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (inner.external_bridge_records.contains(key)) {
        emit(<a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedApprovedEvent">ExternalDepositedApprovedEvent</a>{
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
    <b>assert</b>!(token_payload.token_amount() &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>let</b> <b>mut</b> token = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.mint&lt;T&gt;(amount, ctx);
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">bridge_fee::calculate_cross_in_fee_amount</a>(parent_id,source_chain <b>as</b> u64,token_id,amount);
    <b>assert</b>!(amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>if</b> (fee != 0){
          <b>let</b> fee_coin=token.split&lt;T&gt;(fee, ctx);
          <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(parent_id, fee_coin);
    };
    transfer::public_transfer(token, address::from_bytes(target_address));
    inner.external_bridge_records.push_back(
        key,
        <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeRecord">ExternalBridgeRecord</a> {
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount,
            verified_signatures: option::some(signatures),
            claimed: <b>true</b>,
        },
    );
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedEventV2">ExternalDepositedEventV2</a> {
            tx_hash,
            token_type: token_id,
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount_before_fee: amount,
            amount_after_fee: amount - fee
        },
    )
}
</code></pre>



</details>

<a name="bridge_bridge_approval_and_claimed_external_busd_coin"></a>

## Function `approval_and_claimed_external_busd_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approval_and_claimed_external_busd_coin">approval_and_claimed_external_busd_coin</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, <a href="../bridge/message.md#bridge_message">message</a>: <a href="../bridge/message.md#bridge_message_BridgeMessage">bridge::message::BridgeMessage</a>, signatures: vector&lt;vector&lt;u8&gt;&gt;, bfc_system_state: &<b>mut</b> <a href="../bfc_system/bfc_system.md#bfc_system_bfc_system_BfcSystemState">bfc_system::bfc_system::BfcSystemState</a>, cap: &<a href="../bfc_system/bfc_system_state_inner.md#bfc_system_bfc_system_state_inner_BfcSystemModifyCap">bfc_system::bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_approval_and_claimed_external_busd_coin">approval_and_claimed_external_busd_coin</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    <a href="../bridge/message.md#bridge_message">message</a>: BridgeMessage,
    signatures: vector&lt;vector&lt;u8&gt;&gt;,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    cap: &BfcSystemModifyCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    // verify signatures
    inner.<a href="../bridge/committee.md#bridge_committee">committee</a>.verify_signatures(<a href="../bridge/message.md#bridge_message">message</a>, signatures);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_type() == <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(), <a href="../bridge/bridge.md#bridge_bridge_EMustBeTokenMessage">EMustBeTokenMessage</a>);
    <b>assert</b>!(<a href="../bridge/message.md#bridge_message">message</a>.message_version() == <a href="../bridge/bridge.md#bridge_bridge_MESSAGE_VERSION">MESSAGE_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageVersion">EUnexpectedMessageVersion</a>);
    <b>let</b> token_payload = <a href="../bridge/message.md#bridge_message">message</a>.extract_token_bridge_payload();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>assert</b>!(
        <a href="../bridge/message.md#bridge_message">message</a>.source_chain() == inner.chain_id || target_chain == inner.chain_id,
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>,
    );
    <b>let</b> coin_type = type_name::into_string(type_name::get&lt;T&gt;());
    <b>let</b> token_id=<a href="../bridge/treasury.md#bridge_treasury_token_id">treasury::token_id</a>&lt;T&gt;(&inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>);
    // check records
    <b>let</b> tx_hash = ascii::string(token_payload.token_tx_hash());
    <b>let</b> source_chain = <a href="../bridge/message.md#bridge_message">message</a>.source_chain();
    <b>let</b> target_chain = token_payload.token_target_chain();
    <b>let</b> source_address = token_payload.token_sender_address();
    <b>let</b> target_address = token_payload.token_target_address();
    <b>let</b> amount = token_payload.token_amount();
    <b>let</b> key = <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (inner.external_bridge_records.contains(key)) {
        emit(<a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedApprovedEvent">ExternalDepositedApprovedEvent</a>{
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
    <b>assert</b>!(token_payload.token_amount() &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>let</b> <b>mut</b> token =bfc_system_state.mint_stable&lt;BUSD&gt;(amount, cap,  ctx);
    //address::from_bytes(target_address),
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">bridge_fee::calculate_cross_in_fee_amount</a>(parent_id,source_chain <b>as</b> u64,token_id,amount);
    <b>assert</b>!(amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>if</b> (fee != 0){
          <b>let</b> fee_coin=token.split&lt;BUSD&gt;(fee, ctx);
          <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(parent_id, fee_coin);
    };
    transfer::public_transfer(token, address::from_bytes(target_address));
    inner.external_bridge_records.push_back(
        key,
        <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeRecord">ExternalBridgeRecord</a> {
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount,
            verified_signatures: option::some(signatures),
            claimed: <b>true</b>,
        },
    );
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_ExternalDepositedEventV2">ExternalDepositedEventV2</a> {
            tx_hash,
            token_type: token_id,
            source_chain,
            target_chain: inner.chain_id,
            source_address,
            target_address,
            amount_before_fee: amount,
            amount_after_fee: amount - fee
        },
    )
}
</code></pre>



</details>

<a name="bridge_bridge_withdraw_external_busd_coin"></a>

## Function `withdraw_external_busd_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_withdraw_external_busd_coin">withdraw_external_busd_coin</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, target_chain: u8, target_address: vector&lt;u8&gt;, token: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;, token_id_expect: u64, bfc_system_state: &<b>mut</b> <a href="../bfc_system/bfc_system.md#bfc_system_bfc_system_BfcSystemState">bfc_system::bfc_system::BfcSystemState</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_withdraw_external_busd_coin">withdraw_external_busd_coin</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: vector&lt;u8&gt;,
    <b>mut</b> token: Coin&lt;T&gt;,
    token_id_expect: u64,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen">tokenlist::is_supported_from_benfen</a>(
        parent_id, target_chain <b>as</b> u64, token_id_expect),<a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>);
    <b>assert</b>!(token_id_expect == <a href="../bridge/bridge.md#bridge_bridge_TOKEN_ID_USDC">TOKEN_ID_USDC</a> || token_id_expect == <a href="../bridge/bridge.md#bridge_bridge_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, <a href="../bridge/bridge.md#bridge_bridge_EInvalidTokenIdExpect">EInvalidTokenIdExpect</a>);
    <b>assert</b>!(type_name::get&lt;T&gt;() == type_name::get&lt;BUSD&gt;(), <a href="../bridge/bridge.md#bridge_bridge_EOnlySupportBusd">EOnlySupportBusd</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>let</b> amount = token.balance().value();
    <b>assert</b>!(amount &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">bridge_fee::calculate_cross_out_fee_amount</a>(parent_id,target_chain <b>as</b> u64,token_id_expect,amount);
    <b>assert</b>!(amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>let</b> fee_coin=token.split&lt;T&gt;(fee, ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(parent_id, fee_coin);
    <b>let</b> amount_after_fee=amount-fee;
    bfc_system_state.burn_stable(token, ctx);
    // emit event
   emit(
        <a href="../bridge/bridge.md#bridge_bridge_ExternalWithdrawEventV2">ExternalWithdrawEventV2</a> {
            token_type: token_id_expect,
            source_chain: inner.chain_id,
            target_chain,
            source_address: address::to_bytes(ctx.sender()),
            target_address,
            amount_before_fee: amount,
            amount_after_fee,
        },
    );
}
</code></pre>



</details>

<a name="bridge_bridge_withdraw_external_coin"></a>

## Function `withdraw_external_coin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_withdraw_external_coin">withdraw_external_coin</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, target_chain: u8, target_address: vector&lt;u8&gt;, token: <a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_withdraw_external_coin">withdraw_external_coin</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    target_chain: u8,
    target_address: vector&lt;u8&gt;,
    <b>mut</b> token: Coin&lt;T&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> token_id=<a href="../bridge/treasury.md#bridge_treasury_token_id">treasury::token_id</a>&lt;T&gt;(&inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>);
    <b>assert</b>!(<a href="../bridge/tokenlist.md#bridge_tokenlist_is_supported_from_benfen">tokenlist::is_supported_from_benfen</a>(
        parent_id, target_chain <b>as</b> u64, token_id),<a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDAndTokenIDExpect">EInvalidChainIDAndTokenIDExpect</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">chain_ids::is_valid_route</a>(inner.chain_id, target_chain), <a href="../bridge/bridge.md#bridge_bridge_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    <b>let</b> amount = token.balance().value();
    <b>assert</b>!(amount &gt; 0, <a href="../bridge/bridge.md#bridge_bridge_ETokenValueIsZero">ETokenValueIsZero</a>);
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">bridge_fee::calculate_cross_out_fee_amount</a>(parent_id,target_chain <b>as</b> u64,token_id,amount);
    <b>assert</b>!(amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>let</b> fee_coin=token.split&lt;T&gt;(fee, ctx);
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(parent_id, fee_coin);
    <b>let</b> amount_after_fee=amount-fee;
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.burn(token);
    // emit event
    emit(
        <a href="../bridge/bridge.md#bridge_bridge_ExternalWithdrawEventV2">ExternalWithdrawEventV2</a> {
            token_type: token_id,
            source_chain: inner.chain_id,
            target_chain,
            source_address: address::to_bytes(ctx.sender()),
            target_address,
            amount_before_fee: amount,
            amount_after_fee,
        },
    );
}
</code></pre>



</details>

<a name="bridge_bridge_get_unclaimed_bridge_fee"></a>

## Function `get_unclaimed_bridge_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_unclaimed_bridge_fee">get_unclaimed_bridge_fee</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_unclaimed_bridge_fee">get_unclaimed_bridge_fee</a>&lt;T&gt;(
     <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
):u64{
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_get_unclaimed_bridge_fee">bridge_fee::get_unclaimed_bridge_fee</a>&lt;T&gt;(&<a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id)
}
</code></pre>



</details>

<a name="bridge_bridge_get_cross_out_fee_amount"></a>

## Function `get_cross_out_fee_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_cross_out_fee_amount">get_cross_out_fee_amount</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, chain_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_cross_out_fee_amount">get_cross_out_fee_amount</a>&lt;T&gt;(
     <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
     chain_id: u64,
     amount: u64,
):u64{
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_and_uid">load_inner_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> token_id = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.token_id&lt;T&gt;();
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">bridge_fee::calculate_cross_out_fee_amount</a>(parent_id,chain_id,token_id,amount)
}
</code></pre>



</details>

<a name="bridge_bridge_get_cross_in_fee_amount"></a>

## Function `get_cross_in_fee_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_cross_in_fee_amount">get_cross_in_fee_amount</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, chain_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_cross_in_fee_amount">get_cross_in_fee_amount</a>&lt;T&gt;(
     <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
     chain_id: u64,
     amount: u64,
):u64{
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_and_uid">load_inner_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> token_id = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.token_id&lt;T&gt;();
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">bridge_fee::calculate_cross_in_fee_amount</a>(parent_id,chain_id,token_id,amount)
}
</code></pre>



</details>

<a name="bridge_bridge_get_token_transfer_action_status"></a>

## Function `get_token_transfer_action_status`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_token_transfer_action_status">get_token_transfer_action_status</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, bridge_seq_num: u64): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_token_transfer_action_status">get_token_transfer_action_status</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    bridge_seq_num: u64,
): u8 {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> key = <a href="../bridge/message.md#bridge_message_create_key">message::create_key</a>(
        source_chain,
        <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(),
        bridge_seq_num
    );
    <b>if</b> (!inner.token_transfer_records.contains(key)) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>
    };
    <b>let</b> record = &inner.token_transfer_records[key];
    <b>if</b> (record.claimed) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>
    };
    <b>if</b> (record.verified_signatures.is_some()) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_APPROVED">TRANSFER_STATUS_APPROVED</a>
    };
    <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>
}
</code></pre>



</details>

<a name="bridge_bridge_get_external_token_transfer_action_status"></a>

## Function `get_external_token_transfer_action_status`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_external_token_transfer_action_status">get_external_token_transfer_action_status</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, source_address: vector&lt;u8&gt;, target_address: vector&lt;u8&gt;, amount: u64, tx_hash: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_external_token_transfer_action_status">get_external_token_transfer_action_status</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    source_address: vector&lt;u8&gt;,
    target_address: vector&lt;u8&gt;,
    amount: u64,
    tx_hash: ascii::String,
): u8 {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
     <b>let</b> key = <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>{
        source_chain,
        source_address,
        target_address,
        amount,
        tx_hash,
    };
    <b>if</b> (!inner.external_bridge_records.contains(key)) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>
    };
    <b>let</b> record = &inner.external_bridge_records[key];
    <b>if</b> (record.claimed) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>
    };
    <b>if</b> (record.verified_signatures.is_some()) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_APPROVED">TRANSFER_STATUS_APPROVED</a>
    };
    <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>
}
</code></pre>



</details>

<a name="bridge_bridge_get_send_back_status"></a>

## Function `get_send_back_status`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_send_back_status">get_send_back_status</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, tx_hash: vector&lt;u8&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_send_back_status">get_send_back_status</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    tx_hash: vector&lt;u8&gt;,
): u8 {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> key = <a href="../bridge/message.md#bridge_message_key_refund">message::key_refund</a>(tx_hash);
    <b>if</b> (!inner.refund_records.contains(key)) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_NOT_FOUND">TRANSFER_STATUS_NOT_FOUND</a>
    };
    <b>let</b> record = &inner.refund_records[key];
    <b>if</b> (record.claimed) {
        <b>return</b> <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_CLAIMED">TRANSFER_STATUS_CLAIMED</a>
    };
    <a href="../bridge/bridge.md#bridge_bridge_TRANSFER_STATUS_PENDING">TRANSFER_STATUS_PENDING</a>
}
</code></pre>



</details>

<a name="bridge_bridge_get_token_transfer_action_signatures"></a>

## Function `get_token_transfer_action_signatures`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_token_transfer_action_signatures">get_token_transfer_action_signatures</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, bridge_seq_num: u64): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;vector&lt;u8&gt;&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_token_transfer_action_signatures">get_token_transfer_action_signatures</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    bridge_seq_num: u64,
): Option&lt;vector&lt;vector&lt;u8&gt;&gt;&gt; {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> key = <a href="../bridge/message.md#bridge_message_create_key">message::create_key</a>(
        source_chain,
        <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(),
        bridge_seq_num
    );
    <b>if</b> (!inner.token_transfer_records.contains(key)) {
        <b>return</b> option::none()
    };
    <b>let</b> record = &inner.token_transfer_records[key];
    record.verified_signatures
}
</code></pre>



</details>

<a name="bridge_bridge_multi_signature_passed"></a>

## Function `multi_signature_passed`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_multi_signature_passed">multi_signature_passed</a>(inner: &<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, key: <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">bridge::bridge::ExternalBridgeMessageKey</a>, coin_type: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_multi_signature_passed">multi_signature_passed</a>(
    inner: & <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>,
    key: <a href="../bridge/bridge.md#bridge_bridge_ExternalBridgeMessageKey">ExternalBridgeMessageKey</a>,
    coin_type: String,
): bool {
    // check then add to pre_deposit_multi_signature_records
    <b>if</b> (inner.pre_deposit_multi_signature_records.contains(key)) {
        <b>let</b> records = inner.pre_deposit_multi_signature_records[key];
        // <b>if</b> pre_deposit_multi_signature_records &gt; 50%
        <b>let</b> signed = records.size();
        <b>let</b> len = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.external_coin_admin_count(coin_type);
        <b>if</b> (signed * 2 &gt; len) {
            <b>return</b> <b>true</b>
        };
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="bridge_bridge_load_inner"></a>

## Function `load_inner`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>): &<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
): &<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> {
    <b>let</b> version = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.version();
    // TODO: Replace this with a lazy update function when we add a new version of the inner object.
    <b>assert</b>!(version == <a href="../bridge/bridge.md#bridge_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.load_value();
    <b>assert</b>!(inner.bridge_version == version, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="bridge_bridge_load_inner_mut"></a>

## Function `load_inner_mut`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>): &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut">load_inner_mut</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>): &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> {
    <b>let</b> version = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.version();
    // TODO: Replace this with a lazy update function when we add a new version of the inner object.
    <b>assert</b>!(version == <a href="../bridge/bridge.md#bridge_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.load_value_mut();
    <b>assert</b>!(inner.bridge_version == version, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="bridge_bridge_load_inner_mut_and_uid"></a>

## Function `load_inner_mut_and_uid`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>): (&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>): (&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> ,&<b>mut</b> UID){
    <b>let</b> version = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.version();
    // TODO: Replace this with a lazy update function when we add a new version of the inner object.
    <b>assert</b>!(version == <a href="../bridge/bridge.md#bridge_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.load_value_mut();
    <b>assert</b>!(inner.bridge_version == version, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    (inner,&<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id)
}
</code></pre>



</details>

<a name="bridge_bridge_load_inner_and_uid"></a>

## Function `load_inner_and_uid`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner_and_uid">load_inner_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>): (&<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_load_inner_and_uid">load_inner_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>): (&<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a> ,&UID){
    <b>let</b> version = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.version();
    // TODO: Replace this with a lazy update function when we add a new version of the inner object.
    <b>assert</b>!(version == <a href="../bridge/bridge.md#bridge_bridge_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.inner.load_value&lt;<a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>&gt;();
    <b>assert</b>!(inner.bridge_version == version, <a href="../bridge/bridge.md#bridge_bridge_EWrongInnerVersion">EWrongInnerVersion</a>);
    (inner,&<a href="../bridge/bridge.md#bridge_bridge">bridge</a>.id)
}
</code></pre>



</details>

<a name="bridge_bridge_claim_token_internal"></a>

## Function `claim_token_internal`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;&gt;, <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_token_internal">claim_token_internal</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
): (Option&lt;Coin&lt;T&gt;&gt;, <b>address</b>) {
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>let</b> is_busd = type_name::get&lt;T&gt;() == type_name::get&lt;BUSD&gt;();
    <b>assert</b>!(!is_busd, <a href="../bridge/bridge.md#bridge_bridge_EUseClaimBusd">EUseClaimBusd</a>);
    <b>let</b> key = <a href="../bridge/message.md#bridge_message_create_key">message::create_key</a>(source_chain, <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(), bridge_seq_num);
    <b>assert</b>!(inner.token_transfer_records.contains(key), <a href="../bridge/bridge.md#bridge_bridge_EMessageNotFoundInRecords">EMessageNotFoundInRecords</a>);
    // retrieve approved <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> record = &<b>mut</b> inner.token_transfer_records[key];
    // ensure this is a token <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>assert</b>!(
        &record.<a href="../bridge/message.md#bridge_message">message</a>.message_type() == <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(),
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>,
    );
    // Ensure it's signed
    <b>assert</b>!(record.verified_signatures.is_some(), <a href="../bridge/bridge.md#bridge_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>);
    // extract token <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> token_payload = record.<a href="../bridge/message.md#bridge_message">message</a>.extract_token_bridge_payload_v2();
    // get owner <b>address</b>
    <b>let</b> owner = address::from_bytes(token_payload.token_target_address_v2());
    // If already claimed, exit early
    <b>if</b> (record.claimed) {
        emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyClaimed">TokenTransferAlreadyClaimed</a> { message_key: key });
        <b>return</b> (option::none(), owner)
    };
    <b>let</b> target_chain = token_payload.token_target_chain_v2();
    // ensure target chain matches <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.chain_id
    <b>assert</b>!(target_chain == inner.chain_id, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);
    // TODO: why do we check validity of the route here? what <b>if</b> inconsistency?
    // Ensure route is valid
    // TODO: add unit tests
    // `get_route` <b>abort</b> <b>if</b> route is invalid
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(source_chain, target_chain);
    // check token type
    <b>assert</b>!(
        <a href="../bridge/treasury.md#bridge_treasury_token_id">treasury::token_id</a>&lt;T&gt;(&inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>) == token_payload.token_type_v2(),
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedTokenType">EUnexpectedTokenType</a>,
    );
    <b>let</b> amount = token_payload.token_amount_v2();
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">bridge_fee::calculate_cross_in_fee_amount</a>(parent_id,source_chain <b>as</b> u64,token_payload.token_type_v2(),amount);
    <b>assert</b>!(amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>let</b> amount_in_usd = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.calculate_amount_in_usd&lt;T&gt;(amount);
    <b>assert</b>!(amount_in_usd &lt; <a href="../bridge/limiter.md#bridge_limiter_get_external_in_limit">limiter::get_external_in_limit</a>(parent_id, &route), <a href="../bridge/bridge.md#bridge_bridge_ETransferLimit">ETransferLimit</a>);
    // Make sure transfer is within limit.
    <b>if</b> (!inner
        .<a href="../bridge/limiter.md#bridge_limiter">limiter</a>
        .check_and_record_sending_transfer&lt;T&gt;(
        &inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>,
        clock,
        route,
        amount,
    )
    ) {
        emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferLimitExceed">TokenTransferLimitExceed</a> { message_key: key });
        <b>return</b> (option::none(), owner)
    };
    <b>let</b> <b>mut</b> token = inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.mint&lt;T&gt;(amount, ctx);
    <b>if</b> (fee!=0){
          <b>let</b> fee_coin=token.split&lt;T&gt;(fee, ctx);
          <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(parent_id, fee_coin);
    };
    // Record changes
    record.claimed = <b>true</b>;
    emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferClaimed">TokenTransferClaimed</a> { message_key: key });
    (option::some(token), owner)
}
</code></pre>



</details>

<a name="bridge_bridge_check_fast_path_limit"></a>

## Function `check_fast_path_limit`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_check_fast_path_limit">check_fast_path_limit</a>(bridge_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, token_payload: <a href="../bridge/message.md#bridge_message_TokenTransferInPayload">bridge::message::TokenTransferInPayload</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_check_fast_path_limit">check_fast_path_limit</a>(
    bridge_id: &<b>mut</b> UID,
    clock: &Clock,
    token_payload: TokenTransferInPayload,
) {
    //fast path checker
    <b>if</b> (token_payload.token_fast_path_selector_in() != 2) { // 2 is finalized,0 and 1 is fast path
        <b>let</b> amount = token_payload.token_amount_in();
        <b>let</b> chain_id = token_payload.token_target_chain_in();
        <b>let</b> token_id = token_payload.token_type_in();
        <b>let</b> sender_address = token_payload.token_sender_address_in();
        <b>let</b> remaining_limit = <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_check_and_record_user_limit">limiter_fast_path::check_and_record_user_limit</a>(bridge_id, sender_address, chain_id, token_id, amount, clock);
        <b>assert</b>!(remaining_limit, <a href="../bridge/bridge.md#bridge_bridge_EFastPathLimitError">EFastPathLimitError</a>);
    };
}
</code></pre>



</details>

<a name="bridge_bridge_claim_stable_token_internal"></a>

## Function `claim_stable_token_internal`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_stable_token_internal">claim_stable_token_internal</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, bfc_system_state: &<b>mut</b> <a href="../bfc_system/bfc_system.md#bfc_system_bfc_system_BfcSystemState">bfc_system::bfc_system::BfcSystemState</a>, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, cap: &<a href="../bfc_system/bfc_system_state_inner.md#bfc_system_bfc_system_state_inner_BfcSystemModifyCap">bfc_system::bfc_system_state_inner::BfcSystemModifyCap</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../sui/coin.md#sui_coin_Coin">sui::coin::Coin</a>&lt;T&gt;&gt;, <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_claim_stable_token_internal">claim_stable_token_internal</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    bfc_system_state: &<b>mut</b> BfcSystemState,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    cap: &BfcSystemModifyCap,
    ctx: &<b>mut</b> TxContext,
): (Option&lt;Coin&lt;T&gt;&gt;, <b>address</b>) {
    <b>let</b> (inner,parent_id) = <a href="../bridge/bridge.md#bridge_bridge_load_inner_mut_and_uid">load_inner_mut_and_uid</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeUnavailable">EBridgeUnavailable</a>);
    <b>let</b> key = <a href="../bridge/message.md#bridge_message_create_key">message::create_key</a>(source_chain, <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(), bridge_seq_num);
    <b>assert</b>!(inner.token_transfer_records.contains(key), <a href="../bridge/bridge.md#bridge_bridge_EMessageNotFoundInRecords">EMessageNotFoundInRecords</a>);
    // retrieve approved <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> record = &<b>mut</b> inner.token_transfer_records[key];
    // ensure this is a token <a href="../bridge/bridge.md#bridge_bridge">bridge</a> <a href="../bridge/message.md#bridge_message">message</a>
    <b>assert</b>!(
        &record.<a href="../bridge/message.md#bridge_message">message</a>.message_type() == <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(),
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedMessageType">EUnexpectedMessageType</a>,
    );
    // Ensure it's signed
    <b>assert</b>!(record.verified_signatures.is_some(), <a href="../bridge/bridge.md#bridge_bridge_EUnauthorisedClaim">EUnauthorisedClaim</a>);
    // extract token <a href="../bridge/message.md#bridge_message">message</a>
    <b>let</b> token_payload = record.<a href="../bridge/message.md#bridge_message">message</a>.extract_token_bridge_in_payload();
    // get owner <b>address</b>
    <b>let</b> owner = address::from_bytes(token_payload.token_target_address_in());
    // get token type
    <b>let</b> token_id = token_payload.token_type_in();
    <b>assert</b>!(token_id == 5, <a href="../bridge/bridge.md#bridge_bridge_EOnlySupportBusd">EOnlySupportBusd</a>);
    // If already claimed, exit early
    <b>if</b> (record.claimed) {
        emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferAlreadyClaimed">TokenTransferAlreadyClaimed</a> { message_key: key });
        <b>return</b> (option::none(), owner)
    };
    <b>let</b> target_chain = token_payload.token_target_chain_in();
    // ensure target chain matches <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.chain_id
    <b>assert</b>!(target_chain == inner.chain_id, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);
    // `get_route` <b>abort</b> <b>if</b> route is invalid
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(source_chain, target_chain);
    // check token type
    <b>assert</b>!(
        <a href="../bridge/treasury.md#bridge_treasury_token_id">treasury::token_id</a>&lt;T&gt;(&inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>) == token_payload.token_type_in(),
        <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedTokenType">EUnexpectedTokenType</a>,
    );
    <b>let</b> amount = token_payload.token_amount_in();
    <b>assert</b>!(amount &lt; inner.<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.get_mint_busd_max_limit(), <a href="../bridge/bridge.md#bridge_bridge_EInvalidMintAmount">EInvalidMintAmount</a>);
    // Make sure transfer is within limit.
    <b>if</b> (!inner
        .<a href="../bridge/limiter.md#bridge_limiter">limiter</a>
        .check_and_record_sending_transfer&lt;T&gt;(
        &inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>,
        clock,
        route,
        amount,
    )
    ) {
        emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferLimitExceed">TokenTransferLimitExceed</a> { message_key: key });
        <b>return</b> (option::none(), owner)
    };
    <b>let</b> token_id=token_payload.token_type_in();
    <b>let</b> fee=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">bridge_fee::calculate_cross_in_fee_amount</a>(parent_id,target_chain <b>as</b> u64,token_id,amount);
    <b>assert</b>!(amount&gt;fee,<a href="../bridge/bridge.md#bridge_bridge_EInputAmountLteBridgeFee">EInputAmountLteBridgeFee</a>);
    <b>let</b> amount_after_fee=amount-fee;
    <a href="../bridge/bridge.md#bridge_bridge_check_fast_path_limit">check_fast_path_limit</a>(parent_id, clock, token_payload);
    // claim from <a href="../bridge/treasury.md#bridge_treasury">treasury</a>
    //transfer busd to owner
    bfc_system_state.mint_stable_entry_to_address&lt;BUSD&gt;(amount_after_fee, cap, owner, ctx);
    <b>if</b> (fee != 0){
        <b>let</b> fee_coin=bfc_system_state.mint_stable&lt;BUSD&gt;(fee,cap, ctx);
        <a href="../bridge/bridge_fee.md#bridge_bridge_fee_deposit_fee">bridge_fee::deposit_fee</a>(parent_id, fee_coin);
    };
    record.claimed = <b>true</b>;
    emit(<a href="../bridge/bridge.md#bridge_bridge_TokenTransferClaimed">TokenTransferClaimed</a> { message_key: key });
    (option::none(), owner)
}
</code></pre>



</details>

<a name="bridge_bridge_execute_emergency_op"></a>

## Function `execute_emergency_op`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_emergency_op">execute_emergency_op</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_EmergencyOp">bridge::message::EmergencyOp</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_emergency_op">execute_emergency_op</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: EmergencyOp) {
    <b>let</b> op = payload.emergency_op_type();
    <b>if</b> (op == <a href="../bridge/message.md#bridge_message_emergency_op_pause">message::emergency_op_pause</a>()) {
        <b>assert</b>!(!inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeAlreadyPaused">EBridgeAlreadyPaused</a>);
        inner.paused = <b>true</b>;
        emit(<a href="../bridge/bridge.md#bridge_bridge_EmergencyOpEvent">EmergencyOpEvent</a> { frozen: <b>true</b> });
    } <b>else</b> <b>if</b> (op == <a href="../bridge/message.md#bridge_message_emergency_op_unpause">message::emergency_op_unpause</a>()) {
        <b>assert</b>!(inner.paused, <a href="../bridge/bridge.md#bridge_bridge_EBridgeNotPaused">EBridgeNotPaused</a>);
        inner.paused = <b>false</b>;
        emit(<a href="../bridge/bridge.md#bridge_bridge_EmergencyOpEvent">EmergencyOpEvent</a> { frozen: <b>false</b> });
    } <b>else</b> {
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedOperation">EUnexpectedOperation</a>
    };
}
</code></pre>



</details>

<a name="bridge_bridge_execute_refund_admin_operate"></a>

## Function `execute_refund_admin_operate`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_refund_admin_operate">execute_refund_admin_operate</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_RefundAdmin">bridge::message::RefundAdmin</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_refund_admin_operate">execute_refund_admin_operate</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_RefundAdmin">message::RefundAdmin</a>) {
    <b>let</b> op = payload.refund_admin_op_type();
    <b>if</b> (op == <a href="../bridge/message.md#bridge_message_refund_admin_add">message::refund_admin_add</a>()) {
        <b>let</b> sui_address = payload.refund_admin_sui_address();
        inner.<a href="../bridge/bridge.md#bridge_bridge_add_refund_admin">add_refund_admin</a>(sui_address);
    } <b>else</b> <b>if</b> (op == <a href="../bridge/message.md#bridge_message_refund_admin_remove">message::refund_admin_remove</a>()) {
        <b>let</b> sui_address = payload.refund_admin_sui_address();
        inner.<a href="../bridge/bridge.md#bridge_bridge_remove_refund_admin">remove_refund_admin</a>(sui_address);
    } <b>else</b> {
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedOperation">EUnexpectedOperation</a>
    };
}
</code></pre>



</details>

<a name="bridge_bridge_add_refund_admin"></a>

## Function `add_refund_admin`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_add_refund_admin">add_refund_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, <b>address</b>: &<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_add_refund_admin">add_refund_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, <b>address</b>: &String) {
    <b>if</b> (!inner.refund_admins.contains(<b>address</b>)) {
        inner.refund_admins.insert(*<b>address</b>);
    }
}
</code></pre>



</details>

<a name="bridge_bridge_remove_refund_admin"></a>

## Function `remove_refund_admin`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_remove_refund_admin">remove_refund_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, <b>address</b>: &<a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_remove_refund_admin">remove_refund_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, <b>address</b>: &String) {
    <b>if</b> (inner.refund_admins.contains(<b>address</b>)) {
        inner.refund_admins.remove(<b>address</b>);
    }
}
</code></pre>



</details>

<a name="bridge_bridge_execute_update_bridge_limit"></a>

## Function `execute_update_bridge_limit`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_update_bridge_limit">execute_update_bridge_limit</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_UpdateBridgeLimit">bridge::message::UpdateBridgeLimit</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_update_bridge_limit">execute_update_bridge_limit</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: UpdateBridgeLimit) {
    <b>let</b> receiving_chain = payload.update_bridge_limit_payload_receiving_chain();
    <b>assert</b>!(receiving_chain == inner.chain_id, <a href="../bridge/bridge.md#bridge_bridge_EUnexpectedChainID">EUnexpectedChainID</a>);
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">chain_ids::get_route</a>(
        payload.update_bridge_limit_payload_sending_chain(),
        receiving_chain
    );
    inner.<a href="../bridge/limiter.md#bridge_limiter">limiter</a>.update_route_limit(
        &route,
        payload.update_bridge_limit_payload_limit()
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_update_asset_price"></a>

## Function `execute_update_asset_price`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_update_asset_price">execute_update_asset_price</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_UpdateAssetPrice">bridge::message::UpdateAssetPrice</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_update_asset_price">execute_update_asset_price</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: UpdateAssetPrice) {
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.update_asset_notional_price(
        payload.update_asset_price_payload_token_id(),
        payload.update_asset_price_payload_new_price()
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_add_external_coin_admin"></a>

## Function `execute_add_external_coin_admin`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_admin">execute_add_external_coin_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_AddExternalCoinAdmin">bridge::message::AddExternalCoinAdmin</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_admin">execute_add_external_coin_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: AddExternalCoinAdmin) {
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.add_external_coin_admin(
        payload.add_external_coin_admin_payload_coin_type(),
        payload.add_external_coin_admin_payload_admin_address(),
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_remove_external_coin_admin"></a>

## Function `execute_remove_external_coin_admin`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_admin">execute_remove_external_coin_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_RemoveExternalCoinAdmin">bridge::message::RemoveExternalCoinAdmin</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_admin">execute_remove_external_coin_admin</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: RemoveExternalCoinAdmin) {
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.remove_external_coin_admin(
        payload.remove_external_coin_admin_payload_coin_type(),
        payload.remove_external_coin_admin_payload_admin_address(),
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_add_external_coin_target_payload"></a>

## Function `execute_add_external_coin_target_payload`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_target_payload">execute_add_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_AddExternalCoinTarget">bridge::message::AddExternalCoinTarget</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_target_payload">execute_add_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: AddExternalCoinTarget) {
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.add_external_coin_target(
        payload.add_external_coin_target_payload_coin_type(),
        payload.add_external_coin_target_payload_target_address(),
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_remove_external_coin_target_payload"></a>

## Function `execute_remove_external_coin_target_payload`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_target_payload">execute_remove_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_RemoveExternalCoinTarget">bridge::message::RemoveExternalCoinTarget</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_target_payload">execute_remove_external_coin_target_payload</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: RemoveExternalCoinTarget) {
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.remove_external_coin_target(
        payload.remove_external_coin_target_payload_coin_type(),
        payload.remove_external_coin_target_payload_target_address(),
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_add_external_coin_witness"></a>

## Function `execute_add_external_coin_witness`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_witness">execute_add_external_coin_witness</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_AddExternalCoinWitness">bridge::message::AddExternalCoinWitness</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_external_coin_witness">execute_add_external_coin_witness</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: AddExternalCoinWitness) {
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.add_external_coin_witness(
        payload.add_external_coin_witness_payload_coin_type(),
        payload.add_external_coin_witness_payload_witness_address(),
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_remove_external_coin_witness"></a>

## Function `execute_remove_external_coin_witness`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_witness">execute_remove_external_coin_witness</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_RemoveExternalCoinWitness">bridge::message::RemoveExternalCoinWitness</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_external_coin_witness">execute_remove_external_coin_witness</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: RemoveExternalCoinWitness) {
    inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.remove_external_coin_witness(
        payload.remove_external_coin_witness_payload_coin_type(),
        payload.remove_external_coin_witness_payload_witness_address(),
    )
}
</code></pre>



</details>

<a name="bridge_bridge_execute_set_cross_in"></a>

## Function `execute_set_cross_in`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_set_cross_in">execute_set_cross_in</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, payload: <a href="../bridge/message.md#bridge_message_SetCrossInBridgeFee">bridge::message::SetCrossInBridgeFee</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_set_cross_in">execute_set_cross_in</a>(parent_id: &<b>mut</b> UID,payload:SetCrossInBridgeFee,ctx: &<b>mut</b> TxContext){
    <b>let</b> (chain_id,token_id,mode,amount)=payload.set_cross_in_bridge_fee_poyload();
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_in">bridge_fee::set_fee_in_cross_in</a>(parent_id,chain_id <b>as</b> u64,token_id,mode,amount,ctx);
}
</code></pre>



</details>

<a name="bridge_bridge_execute_set_cross_out"></a>

## Function `execute_set_cross_out`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_set_cross_out">execute_set_cross_out</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, payload: <a href="../bridge/message.md#bridge_message_SetCrossOutBridgeFee">bridge::message::SetCrossOutBridgeFee</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_set_cross_out">execute_set_cross_out</a>(parent_id: &<b>mut</b> UID,payload:SetCrossOutBridgeFee,ctx: &<b>mut</b> TxContext){
    <b>let</b> (chain_id,token_id,mode,amount)=payload.set_cross_out_bridge_fee_poyload();
    <a href="../bridge/bridge_fee.md#bridge_bridge_fee_set_fee_in_cross_out">bridge_fee::set_fee_in_cross_out</a>(parent_id,chain_id <b>as</b> u64,token_id,mode,amount,ctx);
}
</code></pre>



</details>

<a name="bridge_bridge_execute_withdraw_bridge_fee"></a>

## Function `execute_withdraw_bridge_fee`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_withdraw_bridge_fee">execute_withdraw_bridge_fee</a>(payload: <a href="../bridge/message.md#bridge_message_WithdrawBridgeFee">bridge::message::WithdrawBridgeFee</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_withdraw_bridge_fee">execute_withdraw_bridge_fee</a>(payload: WithdrawBridgeFee,ctx: &<b>mut</b> TxContext){
   <b>let</b>(recipient,coin_type,amount)=payload.withdraw_bridge_fee_polyload();
   <b>let</b> cap=<a href="../bridge/bridge_fee.md#bridge_bridge_fee_create_withdraw_fee_cap">bridge_fee::create_withdraw_fee_cap</a>(coin_type, amount, ctx);
   transfer::public_transfer(cap,recipient);
}
</code></pre>



</details>

<a name="bridge_bridge_execute_add_token_on_token_list"></a>

## Function `execute_add_token_on_token_list`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_token_on_token_list">execute_add_token_on_token_list</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, payload: <a href="../bridge/message.md#bridge_message_AddTokenOnTokenList">bridge::message::AddTokenOnTokenList</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_token_on_token_list">execute_add_token_on_token_list</a>(parent_id: &<b>mut</b> UID,payload: AddTokenOnTokenList,ctx: &<b>mut</b> TxContext){
    <b>let</b> source_chain=payload.add_token_on_token_list_payload_from_chain_id();
    <b>let</b> target_chain=payload.add_token_on_token_list_payload_to_chain_id();
    <b>let</b> token_id=payload.add_token_on_token_list_payload_token_id();
    <b>if</b> (target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>() || target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>() || target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()) {
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_to_benfen">tokenlist::add_token_to_benfen</a>(parent_id,source_chain <b>as</b> u64,token_id,ctx);
    }<b>else</b> <b>if</b> (source_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>() || source_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>() || source_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>())  {
        <a href="../bridge/tokenlist.md#bridge_tokenlist_add_token_from_benfen">tokenlist::add_token_from_benfen</a>(parent_id,target_chain <b>as</b> u64,token_id,ctx);
    }<b>else</b>{
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDOnTokenList">EInvalidChainIDOnTokenList</a>
    }
}
</code></pre>



</details>

<a name="bridge_bridge_execute_remove_token_on_token_list"></a>

## Function `execute_remove_token_on_token_list`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_token_on_token_list">execute_remove_token_on_token_list</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, payload: <a href="../bridge/message.md#bridge_message_RemoveTokenOnTokenList">bridge::message::RemoveTokenOnTokenList</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_remove_token_on_token_list">execute_remove_token_on_token_list</a>(parent_id: &<b>mut</b> UID,payload: RemoveTokenOnTokenList){
    <b>let</b> source_chain=payload.remove_token_on_token_list_payload_from_chain_id();
    <b>let</b> target_chain=payload.remove_token_on_token_list_payload_to_chain_id();
    <b>let</b> token_id=payload.remove_token_on_token_list_payload_token_id();
    <b>if</b> (target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>() || target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>() || target_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>()) {
        <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_to_benfen">tokenlist::remove_token_to_benfen</a>(parent_id,source_chain <b>as</b> u64,token_id);
    }<b>else</b> <b>if</b> (source_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">chain_ids::sui_mainnet</a>() || source_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">chain_ids::sui_testnet</a>() || source_chain==<a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">chain_ids::sui_custom</a>())  {
        <a href="../bridge/tokenlist.md#bridge_tokenlist_remove_token_from_benfen">tokenlist::remove_token_from_benfen</a>(parent_id,target_chain <b>as</b> u64,token_id);
    }<b>else</b>{
        <b>abort</b> <a href="../bridge/bridge.md#bridge_bridge_EInvalidChainIDOnTokenList">EInvalidChainIDOnTokenList</a>
    }
}
</code></pre>



</details>

<a name="bridge_bridge_execute_add_tokens_on_sui"></a>

## Function `execute_add_tokens_on_sui`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_tokens_on_sui">execute_add_tokens_on_sui</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, payload: <a href="../bridge/message.md#bridge_message_AddTokenOnSui">bridge::message::AddTokenOnSui</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_execute_add_tokens_on_sui">execute_add_tokens_on_sui</a>(inner: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, payload: AddTokenOnSui) {
    // FIXME: <b>assert</b> native_token to be <b>false</b> and add test
    <b>let</b> native_token = payload.is_native();
    <b>let</b> <b>mut</b> token_ids = payload.token_ids();
    <b>let</b> <b>mut</b> token_type_names = payload.token_type_names();
    <b>let</b> <b>mut</b> token_prices = payload.token_prices();
    // Make sure token data is consistent
    <b>assert</b>!(token_ids.length() == token_type_names.length(), <a href="../bridge/bridge.md#bridge_bridge_EMalformedMessageError">EMalformedMessageError</a>);
    <b>assert</b>!(token_ids.length() == token_prices.length(), <a href="../bridge/bridge.md#bridge_bridge_EMalformedMessageError">EMalformedMessageError</a>);
    <b>while</b> (token_ids.length() &gt; 0) {
        <b>let</b> token_id = token_ids.pop_back();
        <b>let</b> token_type_name = token_type_names.pop_back();
        <b>let</b> token_price = token_prices.pop_back();
        inner.<a href="../bridge/treasury.md#bridge_treasury">treasury</a>.add_new_token(token_type_name, token_id, native_token, token_price)
    }
}
</code></pre>



</details>

<a name="bridge_bridge_get_current_seq_num_and_increment"></a>

## Function `get_current_seq_num_and_increment`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">bridge::bridge::BridgeInner</a>, msg_type: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_current_seq_num_and_increment">get_current_seq_num_and_increment</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_BridgeInner">BridgeInner</a>, msg_type: u8): u64 {
    <b>if</b> (!<a href="../bridge/bridge.md#bridge_bridge">bridge</a>.sequence_nums.contains(&msg_type)) {
        <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.sequence_nums.insert(msg_type, 1);
        <b>return</b> 0
    };
    <b>let</b> <b>entry</b> = &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge">bridge</a>.sequence_nums[&msg_type];
    <b>let</b> seq_num = *<b>entry</b>;
    *<b>entry</b> = seq_num + 1;
    seq_num
}
</code></pre>



</details>

<a name="bridge_bridge_get_parsed_token_transfer_message"></a>

## Function `get_parsed_token_transfer_message`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_parsed_token_transfer_message">get_parsed_token_transfer_message</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, bridge_seq_num: u64): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../bridge/message.md#bridge_message_ParsedTokenTransferMessage">bridge::message::ParsedTokenTransferMessage</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_parsed_token_transfer_message">get_parsed_token_transfer_message</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    bridge_seq_num: u64,
): Option&lt;ParsedTokenTransferMessage&gt; {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> key = <a href="../bridge/message.md#bridge_message_create_key">message::create_key</a>(
        source_chain,
        <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(),
        bridge_seq_num
    );
    <b>if</b> (!inner.token_transfer_records.contains(key)) {
        <b>return</b> option::none()
    };
    <b>let</b> record = &inner.token_transfer_records[key];
    <b>let</b> <a href="../bridge/message.md#bridge_message">message</a> = &record.<a href="../bridge/message.md#bridge_message">message</a>;
    option::some(to_parsed_token_transfer_message(<a href="../bridge/message.md#bridge_message">message</a>))
}
</code></pre>



</details>

<a name="bridge_bridge_get_parsed_token_transfer_message_v2"></a>

## Function `get_parsed_token_transfer_message_v2`



<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_parsed_token_transfer_message_v2">get_parsed_token_transfer_message_v2</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, bridge_seq_num: u64): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../bridge/message.md#bridge_message_ParsedTokenTransferMessageV2">bridge::message::ParsedTokenTransferMessageV2</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge.md#bridge_bridge_get_parsed_token_transfer_message_v2">get_parsed_token_transfer_message_v2</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">Bridge</a>,
    source_chain: u8,
    bridge_seq_num: u64,
): Option&lt;ParsedTokenTransferMessageV2&gt; {
    <b>let</b> inner = <a href="../bridge/bridge.md#bridge_bridge_load_inner">load_inner</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>);
    <b>let</b> key = <a href="../bridge/message.md#bridge_message_create_key">message::create_key</a>(
        source_chain,
        <a href="../bridge/message_types.md#bridge_message_types_token">message_types::token</a>(),
        bridge_seq_num
    );
    <b>if</b> (!inner.token_transfer_records.contains(key)) {
        <b>return</b> option::none()
    };
    <b>let</b> record = &inner.token_transfer_records[key];
    <b>let</b> <a href="../bridge/message.md#bridge_message">message</a> = &record.<a href="../bridge/message.md#bridge_message">message</a>;
    option::some(to_parsed_token_transfer_message_v2(<a href="../bridge/message.md#bridge_message">message</a>))
}
</code></pre>



</details>

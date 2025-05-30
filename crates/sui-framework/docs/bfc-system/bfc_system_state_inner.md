---
title: Module `0xc8::bfc_system_state_inner`
---



-  [Struct `BfcSystemStateInner`](#0xc8_bfc_system_state_inner_BfcSystemStateInner)
-  [Struct `BfcSystemStateInnerV2`](#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2)
-  [Resource `BfcSystemAdminCap`](#0xc8_bfc_system_state_inner_BfcSystemAdminCap)
-  [Resource `BfcSystemModifyCap`](#0xc8_bfc_system_state_inner_BfcSystemModifyCap)
-  [Struct `TreasuryParameters`](#0xc8_bfc_system_state_inner_TreasuryParameters)
-  [Struct `BfcSystemParameters`](#0xc8_bfc_system_state_inner_BfcSystemParameters)
-  [Constants](#@Constants_0)
-  [Function `create_inner_state`](#0xc8_bfc_system_state_inner_create_inner_state)
-  [Function `create_stake_manager_key`](#0xc8_bfc_system_state_inner_create_stake_manager_key)
-  [Function `unstake_manager_key`](#0xc8_bfc_system_state_inner_unstake_manager_key)
-  [Function `update_round`](#0xc8_bfc_system_state_inner_update_round)
-  [Function `update_round_v2`](#0xc8_bfc_system_state_inner_update_round_v2)
-  [Function `init_vault_with_positions`](#0xc8_bfc_system_state_inner_init_vault_with_positions)
-  [Function `create_treasury`](#0xc8_bfc_system_state_inner_create_treasury)
-  [Function `get_rate_map`](#0xc8_bfc_system_state_inner_get_rate_map)
-  [Function `swap_bfc_to_stablecoin`](#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin)
-  [Function `swap_bfc_to_stablecoin_balance`](#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance)
-  [Function `swap_stablecoin_to_bfc`](#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc)
-  [Function `swap_stablecoin_to_bfc_balance`](#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance)
-  [Function `get_stablecoin_by_bfc`](#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc)
-  [Function `get_bfc_by_stablecoin`](#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin)
-  [Function `get_bfc_exchange_rate`](#0xc8_bfc_system_state_inner_get_bfc_exchange_rate)
-  [Function `get_stablecoin_exchange_rate`](#0xc8_bfc_system_state_inner_get_stablecoin_exchange_rate)
-  [Function `next_epoch_bfc_required`](#0xc8_bfc_system_state_inner_next_epoch_bfc_required)
-  [Function `next_epoch_bfc_required_v2`](#0xc8_bfc_system_state_inner_next_epoch_bfc_required_v2)
-  [Function `bfc_required`](#0xc8_bfc_system_state_inner_bfc_required)
-  [Function `bfc_required_with_one_stablecoin`](#0xc8_bfc_system_state_inner_bfc_required_with_one_stablecoin)
-  [Function `treasury_balance`](#0xc8_bfc_system_state_inner_treasury_balance)
-  [Function `treasury_balance_v2`](#0xc8_bfc_system_state_inner_treasury_balance_v2)
-  [Function `deposit_to_treasury`](#0xc8_bfc_system_state_inner_deposit_to_treasury)
-  [Function `deposit_to_treasury_pool`](#0xc8_bfc_system_state_inner_deposit_to_treasury_pool)
-  [Function `rebalance`](#0xc8_bfc_system_state_inner_rebalance)
-  [Function `rebalance_with_one_stablecoin`](#0xc8_bfc_system_state_inner_rebalance_with_one_stablecoin)
-  [Function `request_gas_balance`](#0xc8_bfc_system_state_inner_request_gas_balance)
-  [Function `mint_stable`](#0xc8_bfc_system_state_inner_mint_stable)
-  [Function `burn_stable`](#0xc8_bfc_system_state_inner_burn_stable)
-  [Function `get_all_stable_rate`](#0xc8_bfc_system_state_inner_get_all_stable_rate)
-  [Function `vault_info`](#0xc8_bfc_system_state_inner_vault_info)
-  [Function `vault_ticks`](#0xc8_bfc_system_state_inner_vault_ticks)
-  [Function `vault_positions`](#0xc8_bfc_system_state_inner_vault_positions)
-  [Function `get_total_supply`](#0xc8_bfc_system_state_inner_get_total_supply)
-  [Function `vault_set_pause`](#0xc8_bfc_system_state_inner_vault_set_pause)
-  [Function `vault_info_v2`](#0xc8_bfc_system_state_inner_vault_info_v2)
-  [Function `vault_ticks_v2`](#0xc8_bfc_system_state_inner_vault_ticks_v2)
-  [Function `vault_positions_v2`](#0xc8_bfc_system_state_inner_vault_positions_v2)
-  [Function `get_total_supply_v2`](#0xc8_bfc_system_state_inner_get_total_supply_v2)
-  [Function `vault_set_pause_v2`](#0xc8_bfc_system_state_inner_vault_set_pause_v2)
-  [Function `bfc_system_parameters`](#0xc8_bfc_system_state_inner_bfc_system_parameters)
-  [Function `bfc_system_treasury_parameters`](#0xc8_bfc_system_state_inner_bfc_system_treasury_parameters)
-  [Function `create_bfcdao_action`](#0xc8_bfc_system_state_inner_create_bfcdao_action)
-  [Function `propose`](#0xc8_bfc_system_state_inner_propose)
-  [Function `remove_proposal`](#0xc8_bfc_system_state_inner_remove_proposal)
-  [Function `remove_action`](#0xc8_bfc_system_state_inner_remove_action)
-  [Function `set_voting_delay`](#0xc8_bfc_system_state_inner_set_voting_delay)
-  [Function `set_voting_period`](#0xc8_bfc_system_state_inner_set_voting_period)
-  [Function `set_voting_quorum_rate`](#0xc8_bfc_system_state_inner_set_voting_quorum_rate)
-  [Function `set_min_action_delay`](#0xc8_bfc_system_state_inner_set_min_action_delay)
-  [Function `destroy_terminated_proposal`](#0xc8_bfc_system_state_inner_destroy_terminated_proposal)
-  [Function `judge_proposal_state`](#0xc8_bfc_system_state_inner_judge_proposal_state)
-  [Function `modify_proposal`](#0xc8_bfc_system_state_inner_modify_proposal)
-  [Function `cast_vote`](#0xc8_bfc_system_state_inner_cast_vote)
-  [Function `change_vote`](#0xc8_bfc_system_state_inner_change_vote)
-  [Function `queue_proposal_action`](#0xc8_bfc_system_state_inner_queue_proposal_action)
-  [Function `revoke_vote`](#0xc8_bfc_system_state_inner_revoke_vote)
-  [Function `withdraw_voting`](#0xc8_bfc_system_state_inner_withdraw_voting)
-  [Function `create_voting_bfc`](#0xc8_bfc_system_state_inner_create_voting_bfc)
-  [Function `v1_to_v2`](#0xc8_bfc_system_state_inner_v1_to_v2)
-  [Function `init_bfc_system_state_v2`](#0xc8_bfc_system_state_inner_init_bfc_system_state_v2)
-  [Function `transfer_bfc_from_vault_to_treasury_pool`](#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool)
-  [Function `get_operation_capability`](#0xc8_bfc_system_state_inner_get_operation_capability)
-  [Function `get_operation_capability_by_key`](#0xc8_bfc_system_state_inner_get_operation_capability_by_key)
-  [Function `set_operation_capability`](#0xc8_bfc_system_state_inner_set_operation_capability)
-  [Function `add_operation_capability`](#0xc8_bfc_system_state_inner_add_operation_capability)
-  [Function `remove_operation_capability`](#0xc8_bfc_system_state_inner_remove_operation_capability)
-  [Function `verify_operation_capability`](#0xc8_bfc_system_state_inner_verify_operation_capability)
-  [Function `set_oracle_address`](#0xc8_bfc_system_state_inner_set_oracle_address)
-  [Function `get_oracle_address`](#0xc8_bfc_system_state_inner_get_oracle_address)
-  [Function `withdraw_balance`](#0xc8_bfc_system_state_inner_withdraw_balance)
-  [Function `add_balance_to_vault`](#0xc8_bfc_system_state_inner_add_balance_to_vault)
-  [Function `get_bfc_system_modify_cap_key`](#0xc8_bfc_system_state_inner_get_bfc_system_modify_cap_key)
-  [Function `create_bfc_system_admin_cap`](#0xc8_bfc_system_state_inner_create_bfc_system_admin_cap)
-  [Function `verify_admin_capability`](#0xc8_bfc_system_state_inner_verify_admin_capability)
-  [Function `init_bfc_system_admins`](#0xc8_bfc_system_state_inner_init_bfc_system_admins)
-  [Function `add_bfc_system_admin_cap`](#0xc8_bfc_system_state_inner_add_bfc_system_admin_cap)
-  [Function `remove_bfc_system_admin_cap`](#0xc8_bfc_system_state_inner_remove_bfc_system_admin_cap)
-  [Function `create_bfc_system_modify_cap`](#0xc8_bfc_system_state_inner_create_bfc_system_modify_cap)
-  [Function `get_extra_fields`](#0xc8_bfc_system_state_inner_get_extra_fields)
-  [Function `in_external_stable_gas_coin_list`](#0xc8_bfc_system_state_inner_in_external_stable_gas_coin_list)
-  [Function `add_external_stable_gas_coin`](#0xc8_bfc_system_state_inner_add_external_stable_gas_coin)
-  [Function `delete_external_stable_gas_coin`](#0xc8_bfc_system_state_inner_delete_external_stable_gas_coin)
-  [Function `clear_to_delete_external_stable_gas_coin_list`](#0xc8_bfc_system_state_inner_clear_to_delete_external_stable_gas_coin_list)
-  [Function `delete_from_list`](#0xc8_bfc_system_state_inner_delete_from_list)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="../sui-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
<b>use</b> <a href="../bfc-system/auth_utils.md#0xc8_auth_utils">0xc8::auth_utils</a>;
<b>use</b> <a href="../bfc-system/bars.md#0xc8_bars">0xc8::bars</a>;
<b>use</b> <a href="../bfc-system/baud.md#0xc8_baud">0xc8::baud</a>;
<b>use</b> <a href="../bfc-system/bbrl.md#0xc8_bbrl">0xc8::bbrl</a>;
<b>use</b> <a href="../bfc-system/bcad.md#0xc8_bcad">0xc8::bcad</a>;
<b>use</b> <a href="../bfc-system/beur.md#0xc8_beur">0xc8::beur</a>;
<b>use</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao">0xc8::bfc_dao</a>;
<b>use</b> <a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager">0xc8::bfc_dao_manager</a>;
<b>use</b> <a href="../bfc-system/bgbp.md#0xc8_bgbp">0xc8::bgbp</a>;
<b>use</b> <a href="../bfc-system/bidr.md#0xc8_bidr">0xc8::bidr</a>;
<b>use</b> <a href="../bfc-system/binr.md#0xc8_binr">0xc8::binr</a>;
<b>use</b> <a href="../bfc-system/bjpy.md#0xc8_bjpy">0xc8::bjpy</a>;
<b>use</b> <a href="../bfc-system/bkrw.md#0xc8_bkrw">0xc8::bkrw</a>;
<b>use</b> <a href="../bfc-system/bmxn.md#0xc8_bmxn">0xc8::bmxn</a>;
<b>use</b> <a href="../bfc-system/brub.md#0xc8_brub">0xc8::brub</a>;
<b>use</b> <a href="../bfc-system/bsar.md#0xc8_bsar">0xc8::bsar</a>;
<b>use</b> <a href="../bfc-system/btry.md#0xc8_btry">0xc8::btry</a>;
<b>use</b> <a href="../bfc-system/busd.md#0xc8_busd">0xc8::busd</a>;
<b>use</b> <a href="../bfc-system/bzar.md#0xc8_bzar">0xc8::bzar</a>;
<b>use</b> <a href="../bfc-system/math_u64.md#0xc8_math_u64">0xc8::math_u64</a>;
<b>use</b> <a href="../bfc-system/mgg.md#0xc8_mgg">0xc8::mgg</a>;
<b>use</b> <a href="../bfc-system/position.md#0xc8_position">0xc8::position</a>;
<b>use</b> <a href="../bfc-system/tick.md#0xc8_tick">0xc8::tick</a>;
<b>use</b> <a href="../bfc-system/treasury.md#0xc8_treasury">0xc8::treasury</a>;
<b>use</b> <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">0xc8::treasury_pool</a>;
<b>use</b> <a href="../bfc-system/vault.md#0xc8_vault">0xc8::vault</a>;
<b>use</b> <a href="../bfc-system/bfc_dao_voting_pool.md#0xc8_voting_pool">0xc8::voting_pool</a>;
</code></pre>



<a name="0xc8_bfc_system_state_inner_BfcSystemStateInner"></a>

## Struct `BfcSystemStateInner`



<pre><code><b>struct</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stable_base_points: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_rate: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dao: <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>: <a href="../bfc-system/treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>: <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_TreasuryPool">treasury_pool::TreasuryPool</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stable_rate: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_system_state_inner_BfcSystemStateInnerV2"></a>

## Struct `BfcSystemStateInnerV2`



<pre><code><b>struct</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stable_base_points: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_rate: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>dao: <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>: <a href="../bfc-system/treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>: <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_TreasuryPool">treasury_pool::TreasuryPool</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stable_rate: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>operation_capability: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>admin_capability_addresses: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>admin_init: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>oracle_address: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_system_state_inner_BfcSystemAdminCap"></a>

## Resource `BfcSystemAdminCap`



<pre><code><b>struct</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">BfcSystemAdminCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_system_state_inner_BfcSystemModifyCap"></a>

## Resource `BfcSystemModifyCap`



<pre><code><b>struct</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">BfcSystemModifyCap</a> <b>has</b> store, key
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
<code>key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_system_state_inner_TreasuryParameters"></a>

## Struct `TreasuryParameters`



<pre><code><b>struct</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">TreasuryParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>position_number: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>tick_spacing: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>spacing_times: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>max_counter_times: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>base_point: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>initialize_price: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_system_state_inner_BfcSystemParameters"></a>

## Struct `BfcSystemParameters`



<pre><code><b>struct</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>chain_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>time_interval: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>treasury_parameters: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">bfc_system_state_inner::TreasuryParameters</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_bfc_system_state_inner_BFC_SYSTEM_STATE_START_ROUND"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BFC_SYSTEM_STATE_START_ROUND">BFC_SYSTEM_STATE_START_ROUND</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xc8_bfc_system_state_inner_BFC_SYSTEM_TREASURY_KEY"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BFC_SYSTEM_TREASURY_KEY">BFC_SYSTEM_TREASURY_KEY</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xc8_bfc_system_state_inner_DEFAULT_ADMIN_ADDRESSES"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; = [0];
</code></pre>



<a name="0xc8_bfc_system_state_inner_DEFAULT_REWARD_RATE"></a>

Default reward rate 50% ,base point is 100


<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_REWARD_RATE">DEFAULT_REWARD_RATE</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 50;
</code></pre>



<a name="0xc8_bfc_system_state_inner_DEFAULT_STABLE_BASE_POINTS"></a>

Default stable base points


<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_STABLE_BASE_POINTS">DEFAULT_STABLE_BASE_POINTS</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10;
</code></pre>



<a name="0xc8_bfc_system_state_inner_DEFAULT_STABLE_RATE"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_STABLE_RATE">DEFAULT_STABLE_RATE</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000;
</code></pre>



<a name="0xc8_bfc_system_state_inner_DEFAULT_TREASURY_ADMIN"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_TREASURY_ADMIN">DEFAULT_TREASURY_ADMIN</a>: <b>address</b> = 0;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERROR_MINT_COIN_TYPE"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERROR_MINT_COIN_TYPE">ERROR_MINT_COIN_TYPE</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1013;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_ADD_ADMIN_COUNT_ZERO"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_ADD_ADMIN_COUNT_ZERO">ERR_ADD_ADMIN_COUNT_ZERO</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1009;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_ADMIN_ALREADY_INITED"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_ADMIN_ALREADY_INITED">ERR_ADMIN_ALREADY_INITED</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1012;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_ADMIN_COUNT_ZERO"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_ADMIN_COUNT_ZERO">ERR_ADMIN_COUNT_ZERO</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1010;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_INNER_STABLECOIN_TO_BFC_LIMIT"></a>

Errors


<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_INNER_STABLECOIN_TO_BFC_LIMIT">ERR_INNER_STABLECOIN_TO_BFC_LIMIT</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_MINT_AMOUNT_ZERO"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_MINT_AMOUNT_ZERO">ERR_MINT_AMOUNT_ZERO</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1007;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_MINT_OPERATION_UNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_MINT_OPERATION_UNAUTHORIZED">ERR_MINT_OPERATION_UNAUTHORIZED</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1008;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_MINT_UNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_MINT_UNAUTHORIZED">ERR_MINT_UNAUTHORIZED</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1004;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_NOT_SYSTEM_ADDRESS"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_NOT_SYSTEM_ADDRESS">ERR_NOT_SYSTEM_ADDRESS</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1001;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_REBALANCE_NOT_BUSD"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_REBALANCE_NOT_BUSD">ERR_REBALANCE_NOT_BUSD</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1006;
</code></pre>



<a name="0xc8_bfc_system_state_inner_ERR_SET_CONFIG_UNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_SET_CONFIG_UNAUTHORIZED">ERR_SET_CONFIG_UNAUTHORIZED</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1011;
</code></pre>



<a name="0xc8_bfc_system_state_inner_INNER_STABLECOIN_TO_BFC_LIMIT"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_INNER_STABLECOIN_TO_BFC_LIMIT">INNER_STABLECOIN_TO_BFC_LIMIT</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000000000000;
</code></pre>



<a name="0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [69, 120, 116, 101, 114, 110, 97, 108, 83, 116, 97, 98, 108, 101, 67, 111, 105, 110, 76, 105, 115, 116];
</code></pre>



<a name="0xc8_bfc_system_state_inner_KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST"></a>



<pre><code><b>const</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST</a>: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [84, 111, 68, 101, 108, 101, 116, 101, 69, 120, 116, 101, 114, 110, 97, 108, 83, 116, 97, 98, 108, 101, 67, 111, 105, 110, 76, 105, 115, 116];
</code></pre>



<a name="0xc8_bfc_system_state_inner_create_inner_state"></a>

## Function `create_inner_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_inner_state">create_inner_state</a>(bfc_balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, usd_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;, jpy_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bjpy.md#0xc8_bjpy_BJPY">bjpy::BJPY</a>&gt;, krw_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bkrw.md#0xc8_bkrw_BKRW">bkrw::BKRW</a>&gt;, aud_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/baud.md#0xc8_baud_BAUD">baud::BAUD</a>&gt;, ars_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bars.md#0xc8_bars_BARS">bars::BARS</a>&gt;, brl_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bbrl.md#0xc8_bbrl_BBRL">bbrl::BBRL</a>&gt;, cad_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bcad.md#0xc8_bcad_BCAD">bcad::BCAD</a>&gt;, eur_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/beur.md#0xc8_beur_BEUR">beur::BEUR</a>&gt;, gbp_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bgbp.md#0xc8_bgbp_BGBP">bgbp::BGBP</a>&gt;, idr_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bidr.md#0xc8_bidr_BIDR">bidr::BIDR</a>&gt;, inr_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/binr.md#0xc8_binr_BINR">binr::BINR</a>&gt;, rub_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/brub.md#0xc8_brub_BRUB">brub::BRUB</a>&gt;, sar_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bsar.md#0xc8_bsar_BSAR">bsar::BSAR</a>&gt;, try_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/btry.md#0xc8_btry_BTRY">btry::BTRY</a>&gt;, zar_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bzar.md#0xc8_bzar_BZAR">bzar::BZAR</a>&gt;, mxn_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bmxn.md#0xc8_bmxn_BMXN">bmxn::BMXN</a>&gt;, mgg_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/mgg.md#0xc8_mgg_MGG">mgg::MGG</a>&gt;, parameters: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>, bfc_skip_init_vault: u32, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_inner_state">create_inner_state</a>(
    bfc_balance: Balance&lt;BFC&gt;,
    usd_supply: Supply&lt;BUSD&gt;,
    jpy_supply: Supply&lt;BJPY&gt;,
    krw_supply: Supply&lt;BKRW&gt;,
    aud_supply: Supply&lt;BAUD&gt;,
    ars_supply: Supply&lt;BARS&gt;,
    brl_supply: Supply&lt;BBRL&gt;,
    cad_supply: Supply&lt;BCAD&gt;,
    eur_supply: Supply&lt;BEUR&gt;,
    gbp_supply: Supply&lt;BGBP&gt;,
    idr_supply: Supply&lt;BIDR&gt;,
    inr_supply: Supply&lt;BINR&gt;,
    rub_supply: Supply&lt;BRUB&gt;,
    sar_supply: Supply&lt;BSAR&gt;,
    try_supply: Supply&lt;BTRY&gt;,
    zar_supply: Supply&lt;BZAR&gt;,
    mxn_supply: Supply&lt;BMXN&gt;,
    mgg_supply: Supply&lt;MGG&gt;,
    parameters: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a>,
    bfc_skip_init_vault:u32,
    ctx: &<b>mut</b> TxContext,
): <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a> {
    <b>let</b> dao = <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_create_dao">bfc_dao::create_dao</a>(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>, ctx);
    <a href="../bfc-system/treasury.md#0xc8_treasury_create_treasury_pause_cap">treasury::create_treasury_pause_cap</a>(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_TREASURY_ADMIN">DEFAULT_TREASURY_ADMIN</a>, ctx);
    <b>let</b> (t, remain_balance, rate_map) = <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_treasury">create_treasury</a>(
        bfc_balance,
        usd_supply,
        jpy_supply,
        krw_supply,
        aud_supply,
        ars_supply,
        brl_supply,
        cad_supply,
        eur_supply,
        gbp_supply,
        idr_supply,
        inr_supply,
        rub_supply,
        sar_supply,
        try_supply,
        zar_supply,
        mxn_supply,
        mgg_supply,
        parameters,
        bfc_skip_init_vault,
        ctx);
    <b>let</b> tp = <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_create_treasury_pool">treasury_pool::create_treasury_pool</a>(remain_balance, ctx);

    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a> {
        round: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BFC_SYSTEM_STATE_START_ROUND">BFC_SYSTEM_STATE_START_ROUND</a>,
        stable_base_points: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_STABLE_BASE_POINTS">DEFAULT_STABLE_BASE_POINTS</a>,
        reward_rate: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_REWARD_RATE">DEFAULT_REWARD_RATE</a>,
        dao,
        <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>: t,
        <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>: tp,
        stable_rate: rate_map,
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_stake_manager_key"></a>

## Function `create_stake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_stake_manager_key">create_stake_manager_key</a>(payment: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_stake_manager_key">create_stake_manager_key</a>(payment: Coin&lt;BFC&gt;,
                                             ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_create_stake_manager_key">bfc_dao::create_stake_manager_key</a>(payment, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_unstake_manager_key"></a>

## Function `unstake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_unstake_manager_key">unstake_manager_key</a>(key: <a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, token: <a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_ManagerKeyBfc">bfc_dao_manager::ManagerKeyBfc</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_unstake_manager_key">unstake_manager_key</a>(key: BFCDaoManageKey,
                                        token: ManagerKeyBfc,
                                        ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_unstake_manager_key">bfc_dao::unstake_manager_key</a>(key, token, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_update_round"></a>

## Function `update_round`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round">update_round</a>(inner: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round">update_round</a>(
    inner: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    _ = round;
    inner.stable_rate = <a href="../bfc-system/treasury.md#0xc8_treasury_get_exchange_rates">treasury::get_exchange_rates</a>(&inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_update_round_v2"></a>

## Function `update_round_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round_v2">update_round_v2</a>(inner: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, stable_type_name_vector: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, stable_rate_vector: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round_v2">update_round_v2</a>(
    inner: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    round: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    stable_type_name_vector: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;,
    stable_rate_vector: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;,
) {
    // check
    <b>if</b> (<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&stable_type_name_vector) != <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&stable_rate_vector)) {
        <b>return</b>
    };

    // delete external stable <a href="../sui-framework/coin.md#0x2_coin">coin</a> in <b>to</b> delete list
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_clear_to_delete_external_stable_gas_coin_list">clear_to_delete_external_stable_gas_coin_list</a>(inner);

    _ = round;
    <b>let</b> stable_rate_map = <a href="../bfc-system/treasury.md#0xc8_treasury_get_exchange_rates">treasury::get_exchange_rates</a>(&inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>);
    <b>let</b> busd_vault_key = <a href="../bfc-system/treasury.md#0xc8_treasury_get_vault_key">treasury::get_vault_key</a>&lt;BUSD&gt;();
    <b>let</b> <b>mut</b> busd_rate_some = stable_rate_map.try_get(&busd_vault_key);

    <b>if</b> (busd_rate_some.is_none()) <b>return</b>;

    <b>let</b> busd_rate: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = busd_rate_some.extract();
    // <b>update</b> <a href="../bfc-system/busd.md#0xc8_busd">busd</a> rate
    <b>if</b> (inner.stable_rate.contains(&busd_vault_key)) {
        inner.stable_rate.remove(&busd_vault_key);
        inner.stable_rate.insert(busd_vault_key, busd_rate);
    };

    // <b>update</b> other stable rate
    <b>let</b> len = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&stable_type_name_vector);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> stable_type_name = stable_type_name_vector[i];
        <b>let</b> rate_against_busd = stable_rate_vector[i];
        <b>if</b> ((<a href="../bfc-system/treasury.md#0xc8_treasury_has_vault">treasury::has_vault</a>(&inner.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, stable_type_name) ||
            inner.<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_in_external_stable_gas_coin_list">in_external_stable_gas_coin_list</a>(stable_type_name)) &&
            stable_type_name != busd_vault_key && rate_against_busd &gt; 0) {
            <b>if</b> (inner.stable_rate.contains(&stable_type_name)) {
                inner.stable_rate.remove(&stable_type_name);
            };

            // oracle price decimal = 1_000_000_000
            <b>let</b> <b>mut</b> _rate_against_bfc = 0;
            <b>let</b> (temp_rate, overflowing) = <a href="../bfc-system/math_u64.md#0xc8_math_u64_overflowing_mul">math_u64::overflowing_mul</a>(busd_rate, rate_against_busd);
            <b>if</b> (overflowing) {
                _rate_against_bfc = <a href="../bfc-system/math_u64.md#0xc8_math_u64_wrapping_mul">math_u64::wrapping_mul</a>(busd_rate / 1_000_000_000, rate_against_busd);
            } <b>else</b> {
                _rate_against_bfc = temp_rate / 1_000_000_000;
            };

            inner.stable_rate.insert(stable_type_name, _rate_against_bfc);
        };
        i = i + 1;
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_init_vault_with_positions"></a>

## Function `init_vault_with_positions`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="../bfc-system/treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, _supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _parameters: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> Treasury,
    _key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>,
    _supply: Supply&lt;StableCoinType&gt;,
    _parameters: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> p = <a href="../sui-framework/vec_map.md#0x2_vec_map_get">vec_map::get</a>(&_parameters.treasury_parameters, &_key);
    <a href="../bfc-system/treasury.md#0xc8_treasury_init_vault_with_positions">treasury::init_vault_with_positions</a>&lt;StableCoinType&gt;(
        _treasury,
        _supply,
        p.initialize_price,
        p.base_point,
        p.position_number,
        p.tick_spacing,
        p.spacing_times,
        p.max_counter_times,
        _parameters.chain_start_timestamp_ms,
        ctx,
    );
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_treasury"></a>

## Function `create_treasury`

X treasury  init treasury


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_treasury">create_treasury</a>(bfc_balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, usd_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;, jpy_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bjpy.md#0xc8_bjpy_BJPY">bjpy::BJPY</a>&gt;, krw_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bkrw.md#0xc8_bkrw_BKRW">bkrw::BKRW</a>&gt;, aud_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/baud.md#0xc8_baud_BAUD">baud::BAUD</a>&gt;, ars_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bars.md#0xc8_bars_BARS">bars::BARS</a>&gt;, brl_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bbrl.md#0xc8_bbrl_BBRL">bbrl::BBRL</a>&gt;, cad_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bcad.md#0xc8_bcad_BCAD">bcad::BCAD</a>&gt;, eur_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/beur.md#0xc8_beur_BEUR">beur::BEUR</a>&gt;, gbp_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bgbp.md#0xc8_bgbp_BGBP">bgbp::BGBP</a>&gt;, idr_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bidr.md#0xc8_bidr_BIDR">bidr::BIDR</a>&gt;, inr_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/binr.md#0xc8_binr_BINR">binr::BINR</a>&gt;, rub_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/brub.md#0xc8_brub_BRUB">brub::BRUB</a>&gt;, sar_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bsar.md#0xc8_bsar_BSAR">bsar::BSAR</a>&gt;, try_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/btry.md#0xc8_btry_BTRY">btry::BTRY</a>&gt;, zar_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bzar.md#0xc8_bzar_BZAR">bzar::BZAR</a>&gt;, mxn_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/bmxn.md#0xc8_bmxn_BMXN">bmxn::BMXN</a>&gt;, mgg_supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="../bfc-system/mgg.md#0xc8_mgg_MGG">mgg::MGG</a>&gt;, parameters: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>, bfc_skip_init_vault: u32, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../bfc-system/treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_treasury">create_treasury</a>(
    <b>mut</b> bfc_balance: Balance&lt;BFC&gt;,
    usd_supply: Supply&lt;BUSD&gt;,
    jpy_supply: Supply&lt;BJPY&gt;,
    krw_supply: Supply&lt;BKRW&gt;,
    aud_supply: Supply&lt;BAUD&gt;,
    ars_supply: Supply&lt;BARS&gt;,
    brl_supply: Supply&lt;BBRL&gt;,
    cad_supply: Supply&lt;BCAD&gt;,
    eur_supply: Supply&lt;BEUR&gt;,
    gbp_supply: Supply&lt;BGBP&gt;,
    idr_supply: Supply&lt;BIDR&gt;,
    inr_supply: Supply&lt;BINR&gt;,
    rub_supply: Supply&lt;BRUB&gt;,
    sar_supply: Supply&lt;BSAR&gt;,
    try_supply: Supply&lt;BTRY&gt;,
    zar_supply: Supply&lt;BZAR&gt;,
    mxn_supply: Supply&lt;BMXN&gt;,
    mgg_supply: Supply&lt;MGG&gt;,
    parameters: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a>,
    bfc_skip_init_vault:u32,
    ctx: &<b>mut</b> TxContext
): (Treasury, Balance&lt;BFC&gt;, VecMap&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;) {
    <b>let</b> <b>mut</b> t = <a href="../bfc-system/treasury.md#0xc8_treasury_create_treasury">treasury::create_treasury</a>(parameters.time_interval, <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&bfc_balance), ctx);
    <b>if</b> (bfc_skip_init_vault == 0) {
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BUSD&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BUSD"), usd_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BJPY&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BJPY"), jpy_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BKRW&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BKRW"), krw_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BAUD&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BAUD"), aud_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BARS&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BARS"), ars_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BBRL&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BBRL"), brl_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BCAD&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BCAD"), cad_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BEUR&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BEUR"), eur_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BGBP&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BGBP"), gbp_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BIDR&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BIDR"), idr_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BINR&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BINR"), inr_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BRUB&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BRUB"), rub_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BSAR&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BSAR"), sar_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BTRY&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BTRY"), try_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BZAR&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BZAR"), zar_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BMXN&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BMXN"), mxn_supply, parameters, ctx);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;MGG&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"MGG"), mgg_supply, parameters, ctx);
    }<b>else</b>{
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_vault_with_positions">init_vault_with_positions</a>&lt;BUSD&gt;(&<b>mut</b> t, <a href="../move-stdlib/ascii.md#0x1_ascii_string">ascii::string</a>(b"BUSD"), usd_supply, parameters, ctx);
    };
    <b>let</b> <b>mut</b> rate_map = <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;();
    <b>if</b> (<a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>&lt;BFC&gt;(&bfc_balance) &gt; 0) {
        <b>let</b> deposit_balance = <a href="../sui-framework/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> bfc_balance, <a href="../bfc-system/treasury.md#0xc8_treasury_bfc_required">treasury::bfc_required</a>(&t));
        <a href="../bfc-system/treasury.md#0xc8_treasury_deposit">treasury::deposit</a>(&<b>mut</b> t, <a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(deposit_balance, ctx));
        <a href="../bfc-system/treasury.md#0xc8_treasury_rebalance_internal">treasury::rebalance_internal</a>(&<b>mut</b> t, <b>false</b>, ctx);
        rate_map = <a href="../bfc-system/treasury.md#0xc8_treasury_get_exchange_rates">treasury::get_exchange_rates</a>(&t);
    };
    (t, bfc_balance, rate_map)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_rate_map"></a>

## Function `get_rate_map`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_rate_map">get_rate_map</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_rate_map">get_rate_map</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): VecMap&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    self.stable_rate
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin"></a>

## Function `swap_bfc_to_stablecoin`

swap bfc to stablecoin


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, coin_bfc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    coin_bfc: Coin&lt;BFC&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/treasury.md#0xc8_treasury_mint">treasury::mint</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, coin_bfc, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, amount, min_amount, deadline, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance"></a>

## Function `swap_bfc_to_stablecoin_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance">swap_bfc_to_stablecoin_balance</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, coin_bfc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance">swap_bfc_to_stablecoin_balance</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    coin_bfc: Coin&lt;BFC&gt;,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;StableCoinType&gt; {
    <a href="../bfc-system/treasury.md#0xc8_treasury_mint_internal">treasury::mint_internal</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, coin_bfc, amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc"></a>

## Function `swap_stablecoin_to_bfc`

swap stablecoin to bfc


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, coin_sc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    min_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    deadline: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/treasury.md#0xc8_treasury_redeem">treasury::redeem</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, coin_sc, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, amount, min_amount, deadline, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance"></a>

## Function `swap_stablecoin_to_bfc_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance">swap_stablecoin_to_bfc_balance</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, coin_sc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, expected_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance">swap_stablecoin_to_bfc_balance</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    expected_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <b>let</b> amount = <a href="../sui-framework/coin.md#0x2_coin_value">coin::value</a>(&coin_sc);
    <b>assert</b>!(amount &lt;= <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_INNER_STABLECOIN_TO_BFC_LIMIT">INNER_STABLECOIN_TO_BFC_LIMIT</a>, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_INNER_STABLECOIN_TO_BFC_LIMIT">ERR_INNER_STABLECOIN_TO_BFC_LIMIT</a>);
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_NOT_SYSTEM_ADDRESS">ERR_NOT_SYSTEM_ADDRESS</a>);
    <b>let</b> <b>mut</b> result_balance = <a href="../bfc-system/treasury.md#0xc8_treasury_redeem_internal">treasury::redeem_internal</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, coin_sc, amount, ctx);
    <b>if</b> (expected_amount == 0 || <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&result_balance) == expected_amount) {
        result_balance
    }
    <b>else</b> <b>if</b> (<a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&result_balance) &gt; expected_amount) {
        <b>let</b> result = <a href="../sui-framework/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> result_balance, expected_amount);
        <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_deposit_to_treasury_pool">treasury_pool::deposit_to_treasury_pool</a>(&<b>mut</b> self.<a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>, <a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(result_balance, ctx));
        result
    } <b>else</b> {
        <b>let</b> amount = expected_amount - <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&result_balance) ;
        <b>let</b> <b>mut</b> result = <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_gas_balance">request_gas_balance</a>(self, amount, ctx);
        <a href="../sui-framework/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> result, result_balance);
        result
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_stablecoin_by_bfc"></a>

## Function `get_stablecoin_by_bfc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(
    self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
{
    <a href="../bfc-system/treasury.md#0xc8_treasury_calculate_swap_result">treasury::calculate_swap_result</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, <b>false</b>, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_bfc_by_stablecoin"></a>

## Function `get_bfc_by_stablecoin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(
    self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): <a href="../bfc-system/vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
{
    <a href="../bfc-system/treasury.md#0xc8_treasury_calculate_swap_result">treasury::calculate_swap_result</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, <b>true</b>, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_bfc_exchange_rate"></a>

## Function `get_bfc_exchange_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_exchange_rate">get_bfc_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_exchange_rate">get_bfc_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/vault.md#0xc8_vault_calculated_swap_result_amount_out">vault::calculated_swap_result_amount_out</a>(&<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;CoinType&gt;(
        self,
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_STABLE_RATE">DEFAULT_STABLE_RATE</a>,
    ))
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_stablecoin_exchange_rate"></a>

## Function `get_stablecoin_exchange_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_exchange_rate">get_stablecoin_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_exchange_rate">get_stablecoin_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/vault.md#0xc8_vault_calculated_swap_result_amount_out">vault::calculated_swap_result_amount_out</a>(&<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;CoinType&gt;(
        self,
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_STABLE_RATE">DEFAULT_STABLE_RATE</a>,
    ))
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_next_epoch_bfc_required"></a>

## Function `next_epoch_bfc_required`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_next_epoch_bfc_required">next_epoch_bfc_required</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_next_epoch_bfc_required">next_epoch_bfc_required</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/treasury.md#0xc8_treasury_bfc_required">treasury::bfc_required</a>(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_next_epoch_bfc_required_v2"></a>

## Function `next_epoch_bfc_required_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_next_epoch_bfc_required_v2">next_epoch_bfc_required_v2</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_next_epoch_bfc_required_v2">next_epoch_bfc_required_v2</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/treasury.md#0xc8_treasury_bfc_required_v2">treasury::bfc_required_v2</a>(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_bfc_required"></a>

## Function `bfc_required`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_required">bfc_required</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_required">bfc_required</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    1
    //todo:<a href="../bfc-system/treasury.md#0xc8_treasury_bfc_required">treasury::bfc_required</a>(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_bfc_required_with_one_stablecoin"></a>

## Function `bfc_required_with_one_stablecoin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_required_with_one_stablecoin">bfc_required_with_one_stablecoin</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_required_with_one_stablecoin">bfc_required_with_one_stablecoin</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/treasury.md#0xc8_treasury_bfc_required_with_one_stablecoin">treasury::bfc_required_with_one_stablecoin</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_treasury_balance"></a>

## Function `treasury_balance`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/treasury.md#0xc8_treasury_get_balance">treasury::get_balance</a>(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_treasury_balance_v2"></a>

## Function `treasury_balance_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance_v2">treasury_balance_v2</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance_v2">treasury_balance_v2</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/treasury.md#0xc8_treasury_get_balance">treasury::get_balance</a>(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_deposit_to_treasury"></a>

## Function `deposit_to_treasury`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, coin_bfc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, coin_bfc: Coin&lt;BFC&gt;) {
    <a href="../bfc-system/treasury.md#0xc8_treasury_deposit_v2">treasury::deposit_v2</a>(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, coin_bfc);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_deposit_to_treasury_pool"></a>

## Function `deposit_to_treasury_pool`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, coin_bfc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, coin_bfc: Coin&lt;BFC&gt;) {
    <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_deposit_to_treasury_pool">treasury_pool::deposit_to_treasury_pool</a>(&<b>mut</b> self.<a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>, coin_bfc);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance">rebalance</a>(_self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, _clock: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance">rebalance</a>(
    _self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    _clock: &Clock,
    _ctx: &<b>mut</b> TxContext,
) {
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_rebalance_with_one_stablecoin"></a>

## Function `rebalance_with_one_stablecoin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance_with_one_stablecoin">rebalance_with_one_stablecoin</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance_with_one_stablecoin">rebalance_with_one_stablecoin</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> vault_key = <a href="../bfc-system/treasury.md#0xc8_treasury_get_vault_key">treasury::get_vault_key</a>&lt;StableCoinType&gt;();
    <b>assert</b>!(vault_key == <a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;BUSD&gt;()), <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_REBALANCE_NOT_BUSD">ERR_REBALANCE_NOT_BUSD</a>);

    <b>let</b> amount = <a href="../bfc-system/treasury.md#0xc8_treasury_bfc_required_with_one_stablecoin">treasury::bfc_required_with_one_stablecoin</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>);
    <b>if</b> (amount &gt; 0) {
        <b>let</b> withdraw_balance = <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_withdraw_to_treasury">treasury_pool::withdraw_to_treasury</a>(&<b>mut</b> self.<a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>, amount, ctx);
        <b>if</b> (<a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&withdraw_balance) &gt; 0) {
            <a href="../bfc-system/treasury.md#0xc8_treasury_deposit_with_one_stablecoin">treasury::deposit_with_one_stablecoin</a>&lt;StableCoinType&gt;(
                &<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>,
                <a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(withdraw_balance, ctx)
            );
        } <b>else</b> {
            <a href="../sui-framework/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(withdraw_balance);
        };
    };
    <b>let</b> pool_balance = <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_get_balance">treasury_pool::get_balance</a>(&self.<a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>);
    <a href="../bfc-system/treasury.md#0xc8_treasury_rebalance_with_one_stablecoin">treasury::rebalance_with_one_stablecoin</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, pool_balance, <b>true</b>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_request_gas_balance"></a>

## Function `request_gas_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_gas_balance">request_gas_balance</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_gas_balance">request_gas_balance</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_withdraw_to_treasury">treasury_pool::withdraw_to_treasury</a>(&<b>mut</b> self.<a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>, amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_mint_stable"></a>

## Function `mint_stable`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_mint_stable">mint_stable</a>&lt;StableCoinType&gt;(inner_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, key: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_mint_stable">mint_stable</a>&lt;StableCoinType&gt;(
    inner_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    key: &String,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;StableCoinType&gt; {
    <b>assert</b>!(std::type_name::get&lt;StableCoinType&gt;() == std::type_name::get&lt;BUSD&gt;(), <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERROR_MINT_COIN_TYPE">ERROR_MINT_COIN_TYPE</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_MINT_AMOUNT_ZERO">ERR_MINT_AMOUNT_ZERO</a>);
    <b>assert</b>!(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_operation_capability">verify_operation_capability</a>(inner_state, key, ctx.sender()), <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_MINT_UNAUTHORIZED">ERR_MINT_UNAUTHORIZED</a>);
    <b>let</b> vault_key = <a href="../bfc-system/treasury.md#0xc8_treasury_get_vault_key">treasury::get_vault_key</a>&lt;StableCoinType&gt;();
    <b>let</b> busd_key = <a href="../bfc-system/treasury.md#0xc8_treasury_get_vault_key">treasury::get_vault_key</a>&lt;BUSD&gt;();
    <b>if</b> (vault_key == busd_key) {
        <b>assert</b>!(<a href="../bfc-system/auth_utils.md#0xc8_auth_utils_has_mint_busd">auth_utils::has_mint_busd</a>(key), <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_MINT_OPERATION_UNAUTHORIZED">ERR_MINT_OPERATION_UNAUTHORIZED</a>);
        <b>return</b> <a href="../bfc-system/treasury.md#0xc8_treasury_mint_stable">treasury::mint_stable</a>&lt;StableCoinType&gt;(&<b>mut</b> inner_state.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, amount, ctx)
    };
    <b>assert</b>!(<a href="../bfc-system/auth_utils.md#0xc8_auth_utils_has_mint_other_stablecoin">auth_utils::has_mint_other_stablecoin</a>(key), <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_MINT_OPERATION_UNAUTHORIZED">ERR_MINT_OPERATION_UNAUTHORIZED</a>);
    <b>let</b> vault_mut = <a href="../bfc-system/treasury.md#0xc8_treasury_borrow_mut_vault">treasury::borrow_mut_vault</a>&lt;StableCoinType&gt;(
        &<b>mut</b> inner_state.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>,
        <a href="../bfc-system/treasury.md#0xc8_treasury_get_vault_key">treasury::get_vault_key</a>&lt;StableCoinType&gt;()
    );
    <b>let</b> (balance_stable, _balance_bfc) = <a href="../bfc-system/vault.md#0xc8_vault_balances">vault::balances</a>&lt;StableCoinType&gt;(vault_mut);
    <b>if</b> (balance_stable &gt;= amount) {
        <b>return</b> <a href="../bfc-system/vault.md#0xc8_vault_decrease_coin_a">vault::decrease_coin_a</a>(vault_mut, amount, ctx)
    };

    <a href="../bfc-system/treasury.md#0xc8_treasury_mint_stable">treasury::mint_stable</a>&lt;StableCoinType&gt;(&<b>mut</b> inner_state.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_burn_stable"></a>

## Function `burn_stable`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_burn_stable">burn_stable</a>&lt;StableCoinType&gt;(inner_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, token: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_burn_stable">burn_stable</a>&lt;StableCoinType&gt;(
    inner_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    token: Coin&lt;StableCoinType&gt;,
){
    <b>let</b> vault_key = <a href="../bfc-system/treasury.md#0xc8_treasury_get_vault_key">treasury::get_vault_key</a>&lt;StableCoinType&gt;();
    <a href="../bfc-system/treasury.md#0xc8_treasury_check_vault">treasury::check_vault</a>(&inner_state.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, vault_key);
    <a href="../bfc-system/treasury.md#0xc8_treasury_burn_stable">treasury::burn_stable</a>(&<b>mut</b> inner_state.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, token)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_all_stable_rate"></a>

## Function `get_all_stable_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_all_stable_rate">get_all_stable_rate</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_all_stable_rate">get_all_stable_rate</a>(self: & <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): VecMap&lt;String, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt; {
    self.stable_rate
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_info"></a>

## Function `vault_info`

X-vault
deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info">vault_info</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="../bfc-system/vault.md#0xc8_vault_VaultInfo">vault::VaultInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info">vault_info</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): VaultInfo {
    <a href="../bfc-system/treasury.md#0xc8_treasury_vault_info">treasury::vault_info</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_ticks"></a>

## Function `vault_ticks`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_ticks">vault_ticks</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_ticks">vault_ticks</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Tick&gt; {
    <a href="../bfc-system/treasury.md#0xc8_treasury_fetch_ticks">treasury::fetch_ticks</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_positions"></a>

## Function `vault_positions`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_positions">vault_positions</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/position.md#0xc8_position_Position">position::Position</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_positions">vault_positions</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Position&gt; {
    <a href="../bfc-system/treasury.md#0xc8_treasury_fetch_positions">treasury::fetch_positions</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_total_supply"></a>

## Function `get_total_supply`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_total_supply">get_total_supply</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_total_supply">get_total_supply</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/treasury.md#0xc8_treasury_get_total_supply">treasury::get_total_supply</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_set_pause"></a>

## Function `vault_set_pause`

deprecated


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(cap: &<a href="../bfc-system/treasury.md#0xc8_treasury_TreasuryPauseCap">treasury::TreasuryPauseCap</a>, self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, pause: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(cap: &TreasuryPauseCap, self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>, pause: bool) {
    <a href="../bfc-system/treasury.md#0xc8_treasury_vault_set_pause">treasury::vault_set_pause</a>&lt;StableCoinType&gt;(cap, &<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, pause)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_info_v2"></a>

## Function `vault_info_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info_v2">vault_info_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../bfc-system/vault.md#0xc8_vault_VaultInfo">vault::VaultInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info_v2">vault_info_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): VaultInfo {
    <a href="../bfc-system/treasury.md#0xc8_treasury_vault_info">treasury::vault_info</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_ticks_v2"></a>

## Function `vault_ticks_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_ticks_v2">vault_ticks_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_ticks_v2">vault_ticks_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Tick&gt; {
    <a href="../bfc-system/treasury.md#0xc8_treasury_fetch_ticks">treasury::fetch_ticks</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_positions_v2"></a>

## Function `vault_positions_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_positions_v2">vault_positions_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/position.md#0xc8_position_Position">position::Position</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_positions_v2">vault_positions_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Position&gt; {
    <a href="../bfc-system/treasury.md#0xc8_treasury_fetch_positions">treasury::fetch_positions</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_total_supply_v2"></a>

## Function `get_total_supply_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_total_supply_v2">get_total_supply_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_total_supply_v2">get_total_supply_v2</a>&lt;StableCoinType&gt;(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="../bfc-system/treasury.md#0xc8_treasury_get_total_supply">treasury::get_total_supply</a>&lt;StableCoinType&gt;(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_set_pause_v2"></a>

## Function `vault_set_pause_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_set_pause_v2">vault_set_pause_v2</a>&lt;StableCoinType&gt;(cap: &<a href="../bfc-system/treasury.md#0xc8_treasury_TreasuryPauseCap">treasury::TreasuryPauseCap</a>, self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, pause: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_set_pause_v2">vault_set_pause_v2</a>&lt;StableCoinType&gt;(
    cap: &TreasuryPauseCap,
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    pause: bool
) {
    <a href="../bfc-system/treasury.md#0xc8_treasury_vault_set_pause">treasury::vault_set_pause</a>&lt;StableCoinType&gt;(cap, &<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, pause)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_bfc_system_parameters"></a>

## Function `bfc_system_parameters`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_system_parameters">bfc_system_parameters</a>(time_interval: u32, chain_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, treasury_parameters: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">bfc_system_state_inner::TreasuryParameters</a>&gt;): <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_system_parameters">bfc_system_parameters</a>(
    time_interval: u32,
    chain_start_timestamp_ms: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    treasury_parameters: VecMap&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">TreasuryParameters</a>&gt;,
): <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a> {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a> {
        time_interval,
        chain_start_timestamp_ms,
        treasury_parameters,
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_bfc_system_treasury_parameters"></a>

## Function `bfc_system_treasury_parameters`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_system_treasury_parameters">bfc_system_treasury_parameters</a>(position_number: u32, tick_spacing: u32, spacing_times: u32, initialize_price: u128, base_point: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, max_counter_times: u32): <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">bfc_system_state_inner::TreasuryParameters</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_system_treasury_parameters">bfc_system_treasury_parameters</a>(
    position_number: u32,
    tick_spacing: u32,
    spacing_times: u32,
    initialize_price: u128,
    base_point: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    max_counter_times: u32,
): <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">TreasuryParameters</a> {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">TreasuryParameters</a> {
        position_number,
        tick_spacing,
        spacing_times,
        initialize_price,
        base_point,
        max_counter_times,
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_bfcdao_action"></a>

## Function `create_bfcdao_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfcdao_action">create_bfcdao_action</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, payment: &<b>mut</b> <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, actionName: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfcdao_action">create_bfcdao_action</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    actionName: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_create_bfcdao_action">bfc_dao::create_bfcdao_action</a>(&<b>mut</b> self.dao, payment, actionName, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_propose"></a>

## Function `propose`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_propose">propose</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, version_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, payment: &<b>mut</b> <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, action_delay: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_propose">propose</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    version_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    action_delay: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_propose">bfc_dao::propose</a>(&<b>mut</b> self.dao, version_id, payment, action_id, action_delay, description, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_remove_proposal"></a>

## Function `remove_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_proposal">remove_proposal</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_proposal">remove_proposal</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, key: &BFCDaoManageKey, proposal_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_remove_proposal">bfc_dao::remove_proposal</a>(&<b>mut</b> self.dao, key, proposal_id);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_remove_action"></a>

## Function `remove_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_action">remove_action</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_action">remove_action</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, key: &BFCDaoManageKey, action_id: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_remove_action">bfc_dao::remove_action</a>(&<b>mut</b> self.dao, key, action_id);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_voting_delay"></a>

## Function `set_voting_delay`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, manager_key: &BFCDaoManageKey, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_set_voting_delay">bfc_dao::set_voting_delay</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_voting_period"></a>

## Function `set_voting_period`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_period">set_voting_period</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_period">set_voting_period</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    manager_key: &BFCDaoManageKey,
    value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_set_voting_period">bfc_dao::set_voting_period</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_voting_quorum_rate"></a>

## Function `set_voting_quorum_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_quorum_rate">set_voting_quorum_rate</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_quorum_rate">set_voting_quorum_rate</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    manager_key: &BFCDaoManageKey,
    value: u8,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_set_voting_quorum_rate">bfc_dao::set_voting_quorum_rate</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_min_action_delay"></a>

## Function `set_min_action_delay`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_min_action_delay">set_min_action_delay</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_min_action_delay">set_min_action_delay</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    manager_key: &BFCDaoManageKey,
    value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_set_min_action_delay">bfc_dao::set_min_action_delay</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_destroy_terminated_proposal">bfc_dao::destroy_terminated_proposal</a>(&<b>mut</b> self.dao, manager_key, proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, current_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, current_time: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> proposal_record = <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_getProposalRecord">bfc_dao::getProposalRecord</a>(&<b>mut</b> wrapper.dao);
    <b>let</b> size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = <a href="../sui-framework/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&proposal_record);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; size) {
        <b>let</b> (_, proposalInfo) = <a href="../sui-framework/vec_map.md#0x2_vec_map_get_entry_by_idx">vec_map::get_entry_by_idx</a>(&proposal_record, i);
        <b>let</b> cur_status = <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_judge_proposal_state">bfc_dao::judge_proposal_state</a>(proposalInfo, current_time);
        <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_set_current_status_into_dao">bfc_dao::set_current_status_into_dao</a>(&<b>mut</b> wrapper.dao, proposalInfo, cur_status);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_modify_proposal"></a>

## Function `modify_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_modify_proposal">modify_proposal</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, proposal_obj: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, index: u8, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_modify_proposal">modify_proposal</a>(
    system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    proposal_obj: &<b>mut</b> Proposal,
    index: u8,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_modify_proposal_obj">bfc_dao::modify_proposal_obj</a>(&<b>mut</b> system_state.dao, proposal_obj, index, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_cast_vote"></a>

## Function `cast_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_cast_vote">cast_vote</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/coin.md#0x2_coin">coin</a>: <a href="../bfc-system/bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, agreeInt: u8, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_cast_vote">cast_vote</a>(
    system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    proposal: &<b>mut</b> Proposal,
    <a href="../sui-framework/coin.md#0x2_coin">coin</a>: VotingBfc,
    agreeInt: u8,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_cast_vote">bfc_dao::cast_vote</a>(&<b>mut</b> system_state.dao, proposal, <a href="../sui-framework/coin.md#0x2_coin">coin</a>, agreeInt, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_change_vote"></a>

## Function `change_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_change_vote">change_vote</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, my_vote: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, agree: bool, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_change_vote">change_vote</a>(
    system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    my_vote: &<b>mut</b> Vote,
    proposal: &<b>mut</b> Proposal,
    agree: bool,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_change_vote">bfc_dao::change_vote</a>(&<b>mut</b> system_state.dao, my_vote, proposal, agree, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_queue_proposal_action"></a>

## Function `queue_proposal_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_queue_proposal_action">queue_proposal_action</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, manager_key: &<a href="../bfc-system/bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_queue_proposal_action">queue_proposal_action</a>(
    system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_queue_proposal_action">bfc_dao::queue_proposal_action</a>(&<b>mut</b> system_state.dao, manager_key, proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_revoke_vote"></a>

## Function `revoke_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_revoke_vote">revoke_vote</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, proposal: &<b>mut</b> <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, my_vote: <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, <a href="../../voting_power.md#0x3_voting_power">voting_power</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_revoke_vote">revoke_vote</a>(
    system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    proposal: &<b>mut</b> Proposal,
    my_vote: Vote,
    <a href="../../voting_power.md#0x3_voting_power">voting_power</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_revoke_vote">bfc_dao::revoke_vote</a>(&<b>mut</b> system_state.dao, proposal, my_vote, <a href="../../voting_power.md#0x3_voting_power">voting_power</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_withdraw_voting"></a>

## Function `withdraw_voting`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_voting">withdraw_voting</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, voting_bfc: <a href="../bfc-system/bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_voting">withdraw_voting</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
                                    voting_bfc: VotingBfc,
                                    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
                                    ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_withdraw_voting">bfc_dao::withdraw_voting</a>(&<b>mut</b> system_state.dao, voting_bfc, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_voting_bfc"></a>

## Function `create_voting_bfc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_voting_bfc">create_voting_bfc</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, <a href="../sui-framework/coin.md#0x2_coin">coin</a>: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_voting_bfc">create_voting_bfc</a>(system_state: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
                                      <a href="../sui-framework/coin.md#0x2_coin">coin</a>: Coin&lt;BFC&gt;,
                                      <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
                                      ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_dao.md#0xc8_bfc_dao_create_voting_bfc">bfc_dao::create_voting_bfc</a>(&<b>mut</b> system_state.dao, <a href="../sui-framework/coin.md#0x2_coin">coin</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_v1_to_v2"></a>

## Function `v1_to_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_v1_to_v2">v1_to_v2</a>(self: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_v1_to_v2">v1_to_v2</a>(
    self: <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    _ctx: &<b>mut</b> TxContext
): (<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, &<b>mut</b> TxContext) {
    <b>let</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a> {
        round,
        stable_base_points,
        reward_rate,
        dao,
        <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>,
        <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>,
        stable_rate,
    } = self;
    (<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a> {
        round,
        stable_base_points,
        reward_rate,
        dao,
        <a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>,
        <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>,
        stable_rate,
        operation_capability: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        admin_capability_addresses: <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
        admin_init: <b>false</b>,
        oracle_address: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(_ctx),
    }, _ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_init_bfc_system_state_v2"></a>

## Function `init_bfc_system_state_v2`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_bfc_system_state_v2">init_bfc_system_state_v2</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_bfc_system_state_v2">init_bfc_system_state_v2</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, _ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;MGG&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BJPY&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BKRW&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BAUD&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BARS&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BBRL&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BCAD&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BEUR&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BGBP&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BIDR&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BINR&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BRUB&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BSAR&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BTRY&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BZAR&gt;(self);
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;BMXN&gt;(self);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool"></a>

## Function `transfer_bfc_from_vault_to_treasury_pool`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_transfer_bfc_from_vault_to_treasury_pool">transfer_bfc_from_vault_to_treasury_pool</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>) {
    <b>let</b> vault_key = <a href="../bfc-system/treasury.md#0xc8_treasury_get_vault_key">treasury::get_vault_key</a>&lt;StableCoinType&gt;();
    <b>if</b> (<a href="../bfc-system/treasury.md#0xc8_treasury_has_vault">treasury::has_vault</a>(&self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, vault_key)) {
        <b>let</b> <a href="../bfc-system/vault.md#0xc8_vault">vault</a> = <a href="../bfc-system/treasury.md#0xc8_treasury_borrow_mut_vault">treasury::borrow_mut_vault</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, vault_key);
        <b>let</b> bfc_balance = <a href="../bfc-system/vault.md#0xc8_vault_clear_coin_b">vault::clear_coin_b</a>(<a href="../bfc-system/vault.md#0xc8_vault">vault</a>);
        <a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool_increase_balance">treasury_pool::increase_balance</a>(&<b>mut</b> self.<a href="../bfc-system/treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>, bfc_balance, vault_key);
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_operation_capability"></a>

## Function `get_operation_capability`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_operation_capability">get_operation_capability</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_operation_capability">get_operation_capability</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): VecMap&lt;String, VecSet&lt;<b>address</b>&gt;&gt; {
    self.operation_capability
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_operation_capability_by_key"></a>

## Function `get_operation_capability_by_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_operation_capability_by_key">get_operation_capability_by_key</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, key: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_operation_capability_by_key">get_operation_capability_by_key</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, key: &String): VecSet&lt;<b>address</b>&gt; {
    <b>let</b> result: Option&lt;VecSet&lt;<b>address</b>&gt;&gt; = <a href="../sui-framework/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&self.operation_capability, key);

    <b>if</b> (<a href="../move-stdlib/option.md#0x1_option_is_some">option::is_some</a>(&result)) {
        *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&result)
    } <b>else</b> {
        <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>()
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_operation_capability"></a>

## Function `set_operation_capability`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_operation_capability">set_operation_capability</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, value: <a href="../sui-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_operation_capability">set_operation_capability</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    key: String,
    value: VecSet&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self, sender(ctx));
    <b>let</b> source_contents = <a href="../sui-framework/vec_set.md#0x2_vec_set_keys">vec_set::keys</a>(&value);
    <b>if</b> (<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&self.operation_capability, &key)) {
        <b>let</b> new_capability = <a href="../sui-framework/vec_map.md#0x2_vec_map_get_mut">vec_map::get_mut</a>(&<b>mut</b> self.operation_capability, &key);
        <b>let</b> <b>mut</b> i = 0;
        <b>while</b> (i &lt; <a href="../sui-framework/vec_set.md#0x2_vec_set_size">vec_set::size</a>(&value)) {
            <b>let</b> addr = &source_contents[i];
            <b>if</b> (!<a href="../sui-framework/vec_set.md#0x2_vec_set_contains">vec_set::contains</a>(new_capability, addr)) {
                <a href="../sui-framework/vec_set.md#0x2_vec_set_insert">vec_set::insert</a>(new_capability, *addr);
                <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_modify_cap">create_bfc_system_modify_cap</a>(ctx, *addr, key);
            };
            i = i + 1;
        };
    } <b>else</b> {
        <a href="../sui-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> self.operation_capability, key, value);
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> length = <a href="../sui-framework/vec_set.md#0x2_vec_set_size">vec_set::size</a>(&value);
        <b>while</b> (i &lt; length) {
            <b>let</b> addr = &source_contents[i];
            <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_modify_cap">create_bfc_system_modify_cap</a>(ctx, *addr, key);
            i = i + 1;
        };
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_add_operation_capability"></a>

## Function `add_operation_capability`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_operation_capability">add_operation_capability</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, value: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_operation_capability">add_operation_capability</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    key: String,
    value: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self, sender(ctx));
    <b>if</b> (<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&self.operation_capability, &key)) {
        <b>let</b> new_capability = <a href="../sui-framework/vec_map.md#0x2_vec_map_get_mut">vec_map::get_mut</a>(&<b>mut</b> self.operation_capability, &key);
        <a href="../sui-framework/vec_set.md#0x2_vec_set_insert">vec_set::insert</a>(new_capability, value);
    } <b>else</b> {
        <b>let</b> <b>mut</b> new_set = <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>();
        <a href="../sui-framework/vec_set.md#0x2_vec_set_insert">vec_set::insert</a>(&<b>mut</b> new_set, value);
        <a href="../sui-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> self.operation_capability, key, new_set);
    };
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_modify_cap">create_bfc_system_modify_cap</a>(ctx, value, key);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_remove_operation_capability"></a>

## Function `remove_operation_capability`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_operation_capability">remove_operation_capability</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, key: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, value: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_operation_capability">remove_operation_capability</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    key: &String,
    value: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self, sender(ctx));
    <b>if</b> (<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&self.operation_capability, key)) {
        <b>let</b> new_capability = <a href="../sui-framework/vec_map.md#0x2_vec_map_get_mut">vec_map::get_mut</a>(&<b>mut</b> self.operation_capability, key);
        <b>if</b> (<a href="../sui-framework/vec_set.md#0x2_vec_set_contains">vec_set::contains</a>(new_capability, &value)) {
            <a href="../sui-framework/vec_set.md#0x2_vec_set_remove">vec_set::remove</a>(new_capability, &value);
        }
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_verify_operation_capability"></a>

## Function `verify_operation_capability`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_operation_capability">verify_operation_capability</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, key: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, value: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_operation_capability">verify_operation_capability</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, key: &String, value: <b>address</b>): bool {
    <b>if</b> (<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&self.operation_capability, key)) {
        <b>let</b> new_capability = <a href="../sui-framework/vec_map.md#0x2_vec_map_get">vec_map::get</a>(&self.operation_capability, key);
        <a href="../sui-framework/vec_set.md#0x2_vec_set_contains">vec_set::contains</a>(new_capability, &value)
    } <b>else</b> {
        <b>false</b>
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_oracle_address"></a>

## Function `set_oracle_address`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_oracle_address">set_oracle_address</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_oracle_address">set_oracle_address</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, <b>address</b>: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self, sender(ctx));
    self.oracle_address = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(<b>address</b>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_oracle_address"></a>

## Function `get_oracle_address`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_oracle_address">get_oracle_address</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_oracle_address">get_oracle_address</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, _ctx: &<b>mut</b> TxContext): Option&lt;<b>address</b>&gt; {
    self.oracle_address
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_withdraw_balance"></a>

## Function `withdraw_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_balance">withdraw_balance</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_balance">withdraw_balance</a>(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
): Balance&lt;BFC&gt; {
    <a href="../bfc-system/treasury.md#0xc8_treasury_withdraw_balance">treasury::withdraw_balance</a>(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_add_balance_to_vault"></a>

## Function `add_balance_to_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_balance_to_vault">add_balance_to_vault</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, <a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_balance_to_vault">add_balance_to_vault</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>,
    <a href="../sui-framework/balance.md#0x2_balance">balance</a>: Balance&lt;StableCoinType&gt;,
    _ctx: &<b>mut</b> TxContext
) {
    <a href="../bfc-system/treasury.md#0xc8_treasury_increase_other_stablecoin_balance">treasury::increase_other_stablecoin_balance</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="../bfc-system/treasury.md#0xc8_treasury">treasury</a>, <a href="../sui-framework/balance.md#0x2_balance">balance</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_bfc_system_modify_cap_key"></a>

## Function `get_bfc_system_modify_cap_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_system_modify_cap_key">get_bfc_system_modify_cap_key</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">bfc_system_state_inner::BfcSystemModifyCap</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_system_modify_cap_key">get_bfc_system_modify_cap_key</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">BfcSystemModifyCap</a>) : String {
    self.key
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_bfc_system_admin_cap"></a>

## Function `create_bfc_system_admin_cap`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_admin_cap">create_bfc_system_admin_cap</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_admin_cap">create_bfc_system_admin_cap</a>(ctx: &<b>mut</b> TxContext, recipient: <b>address</b>) {
    <b>let</b> cap = <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemAdminCap">BfcSystemAdminCap</a> {
        id : <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
    };
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(cap, recipient);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_verify_admin_capability"></a>

## Function `verify_admin_capability`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, addr: <b>address</b>) {
    <b>assert</b>!(<a href="../sui-framework/vec_set.md#0x2_vec_set_contains">vec_set::contains</a>(&self.admin_capability_addresses, &addr), <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_SET_CONFIG_UNAUTHORIZED">ERR_SET_CONFIG_UNAUTHORIZED</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_init_bfc_system_admins"></a>

## Function `init_bfc_system_admins`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_bfc_system_admins">init_bfc_system_admins</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>, admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_bfc_system_admins">init_bfc_system_admins</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, ctx: &<b>mut</b> TxContext, admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;) {
    <b>assert</b>!(!self.admin_init, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_ADMIN_ALREADY_INITED">ERR_ADMIN_ALREADY_INITED</a>);
    self.admin_capability_addresses = <a href="../sui-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>();
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_bfc_system_admin_cap">add_bfc_system_admin_cap</a>(self, ctx, admins);
    self.admin_init = <b>true</b>;
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_add_bfc_system_admin_cap"></a>

## Function `add_bfc_system_admin_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_bfc_system_admin_cap">add_bfc_system_admin_cap</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>, admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_bfc_system_admin_cap">add_bfc_system_admin_cap</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, ctx: &<b>mut</b> TxContext, admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;) {
    <b>let</b> count = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&admins);
    <b>assert</b>!(count &gt; 0, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_ADD_ADMIN_COUNT_ZERO">ERR_ADD_ADMIN_COUNT_ZERO</a>);

    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        <b>let</b> admin = <a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&admins, i);
        self.admin_capability_addresses.insert(*admin);
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_admin_cap">create_bfc_system_admin_cap</a>(ctx, *admin);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_remove_bfc_system_admin_cap"></a>

## Function `remove_bfc_system_admin_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_bfc_system_admin_cap">remove_bfc_system_admin_cap</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, addr: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_remove_bfc_system_admin_cap">remove_bfc_system_admin_cap</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, addr: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self, ctx.sender());
    <b>if</b> (self.admin_capability_addresses.contains(&addr)) {
        self.admin_capability_addresses.remove(&addr);
    };

    <b>assert</b>!(self.admin_capability_addresses.size() &gt; 0, <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_ERR_ADMIN_COUNT_ZERO">ERR_ADMIN_COUNT_ZERO</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_bfc_system_modify_cap"></a>

## Function `create_bfc_system_modify_cap`



<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_modify_cap">create_bfc_system_modify_cap</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>, recipient: <b>address</b>, key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfc_system_modify_cap">create_bfc_system_modify_cap</a>(ctx: &<b>mut</b> TxContext, recipient: <b>address</b>, key: std::ascii::String) {
    <b>let</b> cap = <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemModifyCap">BfcSystemModifyCap</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        key,
    };
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(cap, recipient);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_extra_fields"></a>

## Function `get_extra_fields`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_extra_fields">get_extra_fields</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>): &<a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_extra_fields">get_extra_fields</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>): &Bag {
    &self.extra_fields
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_in_external_stable_gas_coin_list"></a>

## Function `in_external_stable_gas_coin_list`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_in_external_stable_gas_coin_list">in_external_stable_gas_coin_list</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_in_external_stable_gas_coin_list">in_external_stable_gas_coin_list</a>(self: &<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): bool {
    <b>if</b> (self.extra_fields.contains(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>)) {
        <b>let</b> list = self.extra_fields.borrow&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>);

        <b>return</b> list.any!(|x| x == &value);
    };

    <b>false</b>
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_add_external_stable_gas_coin"></a>

## Function `add_external_stable_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_external_stable_gas_coin">add_external_stable_gas_coin</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, value: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_add_external_stable_gas_coin">add_external_stable_gas_coin</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, value: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self, sender(ctx));

    <b>if</b> (self.extra_fields.contains(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>)) {
        <b>let</b> list = self.extra_fields.borrow_mut&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>);

        <b>let</b> <b>mut</b> allow_list = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;();
        <b>let</b> <b>mut</b> i = 0;
        <b>while</b> (i &lt; value.length()) {
            <b>if</b> (!list.any!(|x| x == &value[i])) {
               allow_list.insert(value[i], 0);
            };

            i = i + 1;
        };

        list.append(allow_list);
        <b>return</b>;
    };

    <b>let</b> <b>mut</b> list = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;();
    list.append(value);
    self.extra_fields.add(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>, list);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_delete_external_stable_gas_coin"></a>

## Function `delete_external_stable_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_delete_external_stable_gas_coin">delete_external_stable_gas_coin</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_delete_external_stable_gas_coin">delete_external_stable_gas_coin</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, ctx: &<b>mut</b> TxContext) {
    <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_verify_admin_capability">verify_admin_capability</a>(self, sender(ctx));

    // add <b>to</b> delete list, delete it from rate map after next epoch
    <b>if</b> (!self.extra_fields.contains(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST</a>)) {
        <b>let</b> <b>mut</b> list = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;();
        list.insert(value, 0);
        self.extra_fields.add(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST</a>, list);
    } <b>else</b> {
        <b>let</b> list = self.extra_fields.borrow_mut&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST</a>);
        <b>if</b> (!list.any!(|x| x == &value)) {
            list.insert(value, 0);
        };
    };
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_clear_to_delete_external_stable_gas_coin_list"></a>

## Function `clear_to_delete_external_stable_gas_coin_list`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_clear_to_delete_external_stable_gas_coin_list">clear_to_delete_external_stable_gas_coin_list</a>(self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">bfc_system_state_inner::BfcSystemStateInnerV2</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_clear_to_delete_external_stable_gas_coin_list">clear_to_delete_external_stable_gas_coin_list</a> (self: &<b>mut</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInnerV2">BfcSystemStateInnerV2</a>) {
    <b>if</b> (!self.extra_fields.contains(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST</a>)) {
       <b>return</b>
    };

    <b>let</b> to_delete_list = self.extra_fields.remove&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;( <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_TO_DELETE_EXTERNAL_STABLE_GAS_COIN_LIST</a>);

    <b>let</b> stable_gas_list: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;;
    <b>if</b> (self.extra_fields.contains(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>)) {
        stable_gas_list = self.extra_fields.borrow_mut&lt;<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;&gt;(<a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_KEY_EXTERNAL_STABLE_GAS_COIN_LIST">KEY_EXTERNAL_STABLE_GAS_COIN_LIST</a>);
    } <b>else</b> {
        stable_gas_list = &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;();
    };
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; to_delete_list.length()) {
        <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_delete_from_list">delete_from_list</a>(stable_gas_list, to_delete_list[i]);
        // remove from stable rate map
        <b>if</b> (self.stable_rate.contains(&to_delete_list[i])) {
            self.stable_rate.remove(&to_delete_list[i]);
        };

        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_delete_from_list"></a>

## Function `delete_from_list`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_delete_from_list">delete_from_list</a>(list: &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bfc-system/bfc_system_state_inner.md#0xc8_bfc_system_state_inner_delete_from_list">delete_from_list</a>(list:  &<b>mut</b> <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>&gt;, value: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>) {
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; list.length()) {
        <b>if</b> (list[i] == value) {
            list.remove(i);
            <b>return</b>
        };

        i = i + 1;
    };
}
</code></pre>



</details>

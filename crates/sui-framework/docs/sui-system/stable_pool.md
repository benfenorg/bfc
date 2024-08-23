---
title: Module `0x3::stable_pool`
---



-  [Resource `StablePool`](#0x3_stable_pool_StablePool)
-  [Struct `PoolStableTokenExchangeRate`](#0x3_stable_pool_PoolStableTokenExchangeRate)
-  [Resource `StakedStable`](#0x3_stable_pool_StakedStable)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x3_stable_pool_new)
-  [Function `request_add_stake`](#0x3_stable_pool_request_add_stake)
-  [Function `request_withdraw_stake`](#0x3_stable_pool_request_withdraw_stake)
-  [Function `withdraw_from_principal`](#0x3_stable_pool_withdraw_from_principal)
-  [Function `unwrap_staked_sui`](#0x3_stable_pool_unwrap_staked_sui)
-  [Function `deposit_rewards`](#0x3_stable_pool_deposit_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#0x3_stable_pool_process_pending_stakes_and_withdraws)
-  [Function `process_pending_stake_withdraw`](#0x3_stable_pool_process_pending_stake_withdraw)
-  [Function `process_pending_stake`](#0x3_stable_pool_process_pending_stake)
-  [Function `withdraw_rewards`](#0x3_stable_pool_withdraw_rewards)
-  [Function `activate_stable_pool`](#0x3_stable_pool_activate_stable_pool)
-  [Function `deactivate_stable_pool`](#0x3_stable_pool_deactivate_stable_pool)
-  [Function `stable_balance`](#0x3_stable_pool_stable_balance)
-  [Function `rewards_pool`](#0x3_stable_pool_rewards_pool)
-  [Function `pool_id`](#0x3_stable_pool_pool_id)
-  [Function `staked_sui_amount`](#0x3_stable_pool_staked_sui_amount)
-  [Function `stake_activation_epoch`](#0x3_stable_pool_stake_activation_epoch)
-  [Function `is_preactive`](#0x3_stable_pool_is_preactive)
-  [Function `is_inactive`](#0x3_stable_pool_is_inactive)
-  [Function `split`](#0x3_stable_pool_split)
-  [Function `split_staked_sui`](#0x3_stable_pool_split_staked_sui)
-  [Function `join_staked_sui`](#0x3_stable_pool_join_staked_sui)
-  [Function `is_equal_staking_metadata`](#0x3_stable_pool_is_equal_staking_metadata)
-  [Function `pool_token_exchange_rate_at_epoch`](#0x3_stable_pool_pool_token_exchange_rate_at_epoch)
-  [Function `pending_stake_amount`](#0x3_stable_pool_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#0x3_stable_pool_pending_stake_withdraw_amount)
-  [Function `exchange_rates`](#0x3_stable_pool_exchange_rates)
-  [Function `sui_amount`](#0x3_stable_pool_sui_amount)
-  [Function `pool_token_amount`](#0x3_stable_pool_pool_token_amount)
-  [Function `is_preactive_at_epoch`](#0x3_stable_pool_is_preactive_at_epoch)
-  [Function `get_sui_amount`](#0x3_stable_pool_get_sui_amount)
-  [Function `get_token_amount`](#0x3_stable_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#0x3_stable_pool_initial_exchange_rate)
-  [Function `check_balance_invariants`](#0x3_stable_pool_check_balance_invariants)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/u64.md#0x1_u64">0x1::u64</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0x3_stable_pool_StablePool"></a>

## Resource `StablePool`

A stable pool embedded in each validator struct in the system state object.


<pre><code><b>struct</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt; <b>has</b> store, key
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
<code>activation_epoch: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is None if the pool is pre-active and Some(<epoch_number>) if active or inactive.
</dd>
<dt>
<code>deactivation_epoch: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>&gt;</code>
</dt>
<dd>
 The epoch at which this stable pool ceased to be active. None = {pre-active, active},
 Some(<epoch_number>) if in-active, and it was de-activated at epoch <epoch_number>.
</dd>
<dt>
<code>stable_balance: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The total number of STABLE tokens in this pool, including the SUI in the rewards_pool, as well as in all the principal
 in the StakedSTABLE object, updated at epoch boundaries.
</dd>
<dt>
<code>rewards_pool: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>pool_token_balance: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
<dt>
<code>exchange_rates: <a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>&gt;</code>
</dt>
<dd>
 Exchange rate history of previous epochs. Key is the epoch number.
 The entries start from the activation_epoch of this pool and contains exchange rates at the beginning of each epoch,
 i.e., right after the rewards for the previous epoch have been deposited into the pool.
</dd>
<dt>
<code>pending_stake: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Pending stake amount for this epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>pending_total_sui_withdraw: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Pending stake withdrawn during the current epoch, emptied at epoch boundaries.
 This includes both the principal and rewards SUI withdrawn.
</dd>
<dt>
<code>pending_pool_token_withdraw: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Pending pool token withdrawn during the current epoch, emptied at epoch boundaries.
</dd>
<dt>
<code>extra_fields: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x3_stable_pool_PoolStableTokenExchangeRate"></a>

## Struct `PoolStableTokenExchangeRate`

Struct representing the exchange rate of the stake pool token to SUI.


<pre><code><b>struct</b> <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sui_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x3_stable_pool_StakedStable"></a>

## Resource `StakedStable`

A self-custodial object holding the staked SUI tokens.


<pre><code><b>struct</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt; <b>has</b> store, key
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
<code>pool_id: <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 ID of the stable pool we are stable with.
</dd>
<dt>
<code>stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The epoch at which the stake becomes active.
</dd>
<dt>
<code>principal: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE&gt;</code>
</dt>
<dd>
 The staked SUI tokens.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_stable_pool_EActivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 16;
</code></pre>



<a name="0x3_stable_pool_EDeactivationOfInactivePool"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 11;
</code></pre>



<a name="0x3_stable_pool_EDelegationOfZeroSui"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EDelegationOfZeroSui">EDelegationOfZeroSui</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 17;
</code></pre>



<a name="0x3_stable_pool_EDelegationToInactivePool"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 10;
</code></pre>



<a name="0x3_stable_pool_EDestroyNonzeroBalance"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EDestroyNonzeroBalance">EDestroyNonzeroBalance</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 5;
</code></pre>



<a name="0x3_stable_pool_EIncompatibleStakedSui"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EIncompatibleStakedSui">EIncompatibleStakedSui</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 12;
</code></pre>



<a name="0x3_stable_pool_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x3_stable_pool_EInsufficientRewardsPoolBalance"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EInsufficientRewardsPoolBalance">EInsufficientRewardsPoolBalance</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0x3_stable_pool_EInsufficientSuiTokenBalance"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EInsufficientSuiTokenBalance">EInsufficientSuiTokenBalance</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0x3_stable_pool_EPendingDelegationDoesNotExist"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EPendingDelegationDoesNotExist">EPendingDelegationDoesNotExist</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 8;
</code></pre>



<a name="0x3_stable_pool_EPoolAlreadyActive"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 14;
</code></pre>



<a name="0x3_stable_pool_EPoolNotPreactive"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EPoolNotPreactive">EPoolNotPreactive</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 15;
</code></pre>



<a name="0x3_stable_pool_EStakedSuiBelowThreshold"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EStakedSuiBelowThreshold">EStakedSuiBelowThreshold</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 18;
</code></pre>



<a name="0x3_stable_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 9;
</code></pre>



<a name="0x3_stable_pool_ETokenTimeLockIsSome"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_ETokenTimeLockIsSome">ETokenTimeLockIsSome</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 6;
</code></pre>



<a name="0x3_stable_pool_EWithdrawAmountCannotBeZero"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EWithdrawAmountCannotBeZero">EWithdrawAmountCannotBeZero</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 2;
</code></pre>



<a name="0x3_stable_pool_EWithdrawalInSameEpoch"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EWithdrawalInSameEpoch">EWithdrawalInSameEpoch</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 13;
</code></pre>



<a name="0x3_stable_pool_EWrongDelegation"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EWrongDelegation">EWrongDelegation</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 7;
</code></pre>



<a name="0x3_stable_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_EWrongPool">EWrongPool</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x3_stable_pool_MIN_STAKING_THRESHOLD"></a>

StakedSui objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="stable_pool.md#0x3_stable_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1000000000;
</code></pre>



<a name="0x3_stable_pool_new"></a>

## Function `new`

Create a new, empty stable pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_new">new</a>&lt;STABLE&gt;(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_new">new</a>&lt;STABLE&gt;(ctx: &<b>mut</b> TxContext) : <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt; {
    <b>let</b> exchange_rates = <a href="../sui-framework/table.md#0x2_table_new">table::new</a>(ctx);
    <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        activation_epoch: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        deactivation_epoch: <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(),
        stable_balance: 0,
        rewards_pool: <a href="../sui-framework/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;(),
        pool_token_balance: 0,
        exchange_rates,
        pending_stake: 0,
        pending_total_sui_withdraw: 0,
        pending_pool_token_withdraw: 0,
        extra_fields: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x3_stable_pool_request_add_stake"></a>

## Function `request_add_stake`

Request to stake to a stable pool. The stake starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_request_add_stake">request_add_stake</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, stake: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE&gt;, stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_request_add_stake">request_add_stake</a>&lt;STABLE&gt;(
    pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;,
    stake: Balance&lt;STABLE&gt;,
    stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) : <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt; {
    <b>let</b> sui_amount = <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&stake);
    <b>assert</b>!(!<a href="stable_pool.md#0x3_stable_pool_is_inactive">is_inactive</a>(pool), <a href="stable_pool.md#0x3_stable_pool_EDelegationToInactivePool">EDelegationToInactivePool</a>);
    <b>assert</b>!(sui_amount &gt; 0, <a href="stable_pool.md#0x3_stable_pool_EDelegationOfZeroSui">EDelegationOfZeroSui</a>);
    <b>let</b> staked_sui = <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt; {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(pool),
        stake_activation_epoch,
        principal: stake,
    };
    pool.pending_stake = pool.pending_stake + sui_amount;
    staked_sui
}
</code></pre>



</details>

<a name="0x3_stable_pool_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw the given stake plus rewards from a stable pool.
Both the principal and corresponding rewards in SUI are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_request_withdraw_stake">request_withdraw_stake</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, staked_sui: <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;, rate: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE&gt;, <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_request_withdraw_stake">request_withdraw_stake</a>&lt;STABLE&gt;(
    pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;,
    staked_sui: <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;,
    rate: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &<b>mut</b> TxContext
) : (Balance&lt;STABLE&gt;, Balance&lt;BFC&gt;) {
    <b>let</b> staked_epoch = staked_sui.stake_activation_epoch;
    <b>let</b> (pool_token_withdraw_amount, principal_withdraw) =
        <a href="stable_pool.md#0x3_stable_pool_withdraw_from_principal">withdraw_from_principal</a>(pool, staked_sui);
    <b>let</b> principal_withdraw_amount = <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&principal_withdraw);

    <b>let</b> (rewards_withdraw, stable_reward_amount) = <a href="stable_pool.md#0x3_stable_pool_withdraw_rewards">withdraw_rewards</a>(
        pool, staked_epoch, principal_withdraw_amount, pool_token_withdraw_amount, <a href="../sui-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx), rate
    );
    <b>let</b> total_sui_withdraw_amount = principal_withdraw_amount + stable_reward_amount;

    pool.pending_total_sui_withdraw = pool.pending_total_sui_withdraw + total_sui_withdraw_amount;
    pool.pending_pool_token_withdraw = pool.pending_pool_token_withdraw + pool_token_withdraw_amount;

    // If the pool is inactive, we immediately process the withdrawal.
    <b>if</b> (<a href="stable_pool.md#0x3_stable_pool_is_inactive">is_inactive</a>(pool)) <a href="stable_pool.md#0x3_stable_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);

    (principal_withdraw, rewards_withdraw)
}
</code></pre>



</details>

<a name="0x3_stable_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal SUI stored in the StakedSui object, and calculate the corresponding amount of pool
tokens using exchange rate at stable epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of SUI.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_withdraw_from_principal">withdraw_from_principal</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, staked_sui: <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_withdraw_from_principal">withdraw_from_principal</a>&lt;STABLE&gt;(
    pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;,
    staked_sui: <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;,
) : (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, Balance&lt;STABLE&gt;) {

    // Check that the stake information matches the pool.
    <b>assert</b>!(staked_sui.pool_id == <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(pool), <a href="stable_pool.md#0x3_stable_pool_EWrongPool">EWrongPool</a>);

    <b>let</b> exchange_rate_at_staking_epoch = <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, staked_sui.stake_activation_epoch);
    <b>let</b> principal_withdraw = <a href="stable_pool.md#0x3_stable_pool_unwrap_staked_sui">unwrap_staked_sui</a>(staked_sui);
    <b>let</b> pool_token_withdraw_amount = <a href="stable_pool.md#0x3_stable_pool_get_token_amount">get_token_amount</a>(&exchange_rate_at_staking_epoch, <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&principal_withdraw));

    (
        pool_token_withdraw_amount,
        principal_withdraw,
    )
}
</code></pre>



</details>

<a name="0x3_stable_pool_unwrap_staked_sui"></a>

## Function `unwrap_staked_sui`



<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_unwrap_staked_sui">unwrap_staked_sui</a>&lt;STABLE&gt;(staked_sui: <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_unwrap_staked_sui">unwrap_staked_sui</a>&lt;STABLE&gt;(staked_sui: <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;): Balance&lt;STABLE&gt; {
    <b>let</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a> {
        id,
        pool_id: _,
        stake_activation_epoch: _,
        principal,
    } = staked_sui;
    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
    principal
}
</code></pre>



</details>

<a name="0x3_stable_pool_deposit_rewards"></a>

## Function `deposit_rewards`

Called at epoch advancement times to add rewards (in SUI) to the stable pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_deposit_rewards">deposit_rewards</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, rewards: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_deposit_rewards">deposit_rewards</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;,
                                            rewards: Balance&lt;BFC&gt;,
                                            amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    pool.stable_balance = pool.stable_balance + amount;
    <a href="../sui-framework/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> pool.rewards_pool, rewards);
}
</code></pre>



</details>

<a name="0x3_stable_pool_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;, ctx: &<b>mut</b> TxContext) {
    <b>let</b> new_epoch = <a href="../sui-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) + 1;
    <a href="stable_pool.md#0x3_stable_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>(pool);
    <a href="stable_pool.md#0x3_stable_pool_process_pending_stake">process_pending_stake</a>(pool);
    <a href="../sui-framework/table.md#0x2_table_add">table::add</a>(
        &<b>mut</b> pool.exchange_rates,
        new_epoch,
        <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a> { sui_amount: pool.stable_balance, pool_token_amount: pool.pool_token_balance },
    );
    <a href="stable_pool.md#0x3_stable_pool_check_balance_invariants">check_balance_invariants</a>(pool, new_epoch);
}
</code></pre>



</details>

<a name="0x3_stable_pool_process_pending_stake_withdraw"></a>

## Function `process_pending_stake_withdraw`

Called at epoch boundaries to process pending stake withdraws requested during the epoch.
Also called immediately upon withdrawal if the pool is inactive.


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_process_pending_stake_withdraw">process_pending_stake_withdraw</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;) {
    pool.stable_balance = pool.stable_balance - pool.pending_total_sui_withdraw;
    pool.pool_token_balance = pool.pool_token_balance - pool.pending_pool_token_withdraw;
    pool.pending_total_sui_withdraw = 0;
    pool.pending_pool_token_withdraw = 0;
}
</code></pre>



</details>

<a name="0x3_stable_pool_process_pending_stake"></a>

## Function `process_pending_stake`

Called at epoch boundaries to process the pending stake.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_process_pending_stake">process_pending_stake</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_process_pending_stake">process_pending_stake</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;) {
    // Use the most up <b>to</b> date exchange rate <b>with</b> the rewards deposited and withdraws effectuated.
    <b>let</b> latest_exchange_rate =
        <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a> { sui_amount: pool.stable_balance, pool_token_amount: pool.pool_token_balance };
    pool.stable_balance = pool.stable_balance + pool.pending_stake;
    pool.pool_token_balance = <a href="stable_pool.md#0x3_stable_pool_get_token_amount">get_token_amount</a>(&latest_exchange_rate, pool.stable_balance);
    pool.pending_stake = 0;
}
</code></pre>



</details>

<a name="0x3_stable_pool_withdraw_rewards"></a>

## Function `withdraw_rewards`

This function does the following:
1. Calculates the total amount of SUI (including principal and rewards) that the provided pool tokens represent
at the current exchange rate.
2. Using the above number and the given principal_withdraw_amount, calculates the rewards portion of the
stake we should withdraw.
3. Withdraws the rewards portion from the rewards pool at the current exchange rate. We only withdraw the rewards
portion because the principal portion was already taken out of the staker's self custodied StakedSui.


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_withdraw_rewards">withdraw_rewards</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, principal_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, pool_token_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, rate: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): (<a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_withdraw_rewards">withdraw_rewards</a>&lt;STABLE&gt;(
    pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;,
    stake_activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    principal_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    pool_token_withdraw_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    rate: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
) : (Balance&lt;BFC&gt;, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>if</b> (stake_activation_epoch == epoch) {
        <b>return</b> (<a href="../sui-framework/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;(), 0)
    };

    <b>let</b> exchange_rate = <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    <b>let</b> total_sui_withdraw_amount = <a href="stable_pool.md#0x3_stable_pool_get_sui_amount">get_sui_amount</a>(&exchange_rate, pool_token_withdraw_amount);
    <b>let</b> <b>mut</b> reward_withdraw_amount =
        <b>if</b> (total_sui_withdraw_amount &gt;= principal_withdraw_amount)
            total_sui_withdraw_amount - principal_withdraw_amount
        <b>else</b> 0;
    <b>let</b> stable_reward_amount = reward_withdraw_amount;
    // This may happen when we are withdrawing everything from the pool and
    // the rewards pool <a href="../sui-framework/balance.md#0x2_balance">balance</a> may be less than reward_withdraw_amount.
    // TODO: FIGURE OUT EXACTLY WHY THIS CAN HAPPEN.
    <b>let</b> reward_bfc = (reward_withdraw_amount <b>as</b> u128) * (rate <b>as</b> u128) / (1000000000 <b>as</b> u128);
    reward_withdraw_amount = std::u64::min((reward_bfc <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>),  <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&pool.rewards_pool));
    (<a href="../sui-framework/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> pool.rewards_pool, reward_withdraw_amount), stable_reward_amount)
}
</code></pre>



</details>

<a name="0x3_stable_pool_activate_stable_pool"></a>

## Function `activate_stable_pool`

Called by validator module to activate a stable pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_activate_stable_pool">activate_stable_pool</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_activate_stable_pool">activate_stable_pool</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;, activation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    // Add the initial exchange rate <b>to</b> the <a href="../sui-framework/table.md#0x2_table">table</a>.
    <a href="../sui-framework/table.md#0x2_table_add">table::add</a>(
        &<b>mut</b> pool.exchange_rates,
        activation_epoch,
        <a href="stable_pool.md#0x3_stable_pool_initial_exchange_rate">initial_exchange_rate</a>()
    );
    // Check that the pool is preactive and not inactive.
    <b>assert</b>!(<a href="stable_pool.md#0x3_stable_pool_is_preactive">is_preactive</a>(pool), <a href="stable_pool.md#0x3_stable_pool_EPoolAlreadyActive">EPoolAlreadyActive</a>);
    <b>assert</b>!(!<a href="stable_pool.md#0x3_stable_pool_is_inactive">is_inactive</a>(pool), <a href="stable_pool.md#0x3_stable_pool_EActivationOfInactivePool">EActivationOfInactivePool</a>);
    // Fill in the active epoch.
    <a href="../move-stdlib/option.md#0x1_option_fill">option::fill</a>(&<b>mut</b> pool.activation_epoch, activation_epoch);
}
</code></pre>



</details>

<a name="0x3_stable_pool_deactivate_stable_pool"></a>

## Function `deactivate_stable_pool`

Deactivate a stable pool by setting the deactivation_epoch. After
this pool deactivation, the pool stops earning rewards. Only stake
withdraws can be made to the pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_deactivate_stable_pool">deactivate_stable_pool</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, deactivation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_deactivate_stable_pool">deactivate_stable_pool</a>&lt;STABLE&gt;(pool: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;, deactivation_epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    // We can't deactivate an already deactivated pool.
    <b>assert</b>!(!<a href="stable_pool.md#0x3_stable_pool_is_inactive">is_inactive</a>(pool), <a href="stable_pool.md#0x3_stable_pool_EDeactivationOfInactivePool">EDeactivationOfInactivePool</a>);
    pool.deactivation_epoch = <a href="../move-stdlib/option.md#0x1_option_some">option::some</a>(deactivation_epoch);
}
</code></pre>



</details>

<a name="0x3_stable_pool_stable_balance"></a>

## Function `stable_balance`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_stable_balance">stable_balance</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_stable_balance">stable_balance</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> { pool.stable_balance }
</code></pre>



</details>

<a name="0x3_stable_pool_rewards_pool"></a>

## Function `rewards_pool`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_rewards_pool">rewards_pool</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_rewards_pool">rewards_pool</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> { <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&pool.rewards_pool) }
</code></pre>



</details>

<a name="0x3_stable_pool_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pool_id">pool_id</a>&lt;STABLE&gt;(staked_sui: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;): <a href="../sui-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pool_id">pool_id</a>&lt;STABLE&gt;(staked_sui: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;): ID { staked_sui.pool_id }
</code></pre>



</details>

<a name="0x3_stable_pool_staked_sui_amount"></a>

## Function `staked_sui_amount`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_staked_sui_amount">staked_sui_amount</a>&lt;STABLE&gt;(staked_sui: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_staked_sui_amount">staked_sui_amount</a>&lt;STABLE&gt;(staked_sui: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> { <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&staked_sui.principal) }
</code></pre>



</details>

<a name="0x3_stable_pool_stake_activation_epoch"></a>

## Function `stake_activation_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_stake_activation_epoch">stake_activation_epoch</a>&lt;STABLE&gt;(staked_sui: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_stake_activation_epoch">stake_activation_epoch</a>&lt;STABLE&gt;(staked_sui: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    staked_sui.stake_activation_epoch
}
</code></pre>



</details>

<a name="0x3_stable_pool_is_preactive"></a>

## Function `is_preactive`

Returns true if the input stable pool is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_preactive">is_preactive</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_preactive">is_preactive</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;): bool{
    <a href="../move-stdlib/option.md#0x1_option_is_none">option::is_none</a>(&pool.activation_epoch)
}
</code></pre>



</details>

<a name="0x3_stable_pool_is_inactive"></a>

## Function `is_inactive`

Returns true if the input stable pool is inactive.


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_inactive">is_inactive</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_inactive">is_inactive</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;): bool {
    <a href="../move-stdlib/option.md#0x1_option_is_some">option::is_some</a>(&pool.deactivation_epoch)
}
</code></pre>



</details>

<a name="0x3_stable_pool_split"></a>

## Function `split`

Split StakedSui self to two parts, one with principal split_amount,
and the remaining principal is left in self.
All the other parameters of the StakedSui like stake_activation_epoch or pool_id remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_split">split</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_split">split</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> TxContext): <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt; {
    <b>let</b> original_amount = <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&self.principal);
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="stable_pool.md#0x3_stable_pool_EInsufficientSuiTokenBalance">EInsufficientSuiTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="stable_pool.md#0x3_stable_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="stable_pool.md#0x3_stable_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="stable_pool.md#0x3_stable_pool_EStakedSuiBelowThreshold">EStakedSuiBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="stable_pool.md#0x3_stable_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="stable_pool.md#0x3_stable_pool_EStakedSuiBelowThreshold">EStakedSuiBelowThreshold</a>);
    <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: self.pool_id,
        stake_activation_epoch: self.stake_activation_epoch,
        principal: <a href="../sui-framework/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> self.principal, split_amount),
    }
}
</code></pre>



</details>

<a name="0x3_stable_pool_split_staked_sui"></a>

## Function `split_staked_sui`

Split the given StakedSui to the two parts, one with principal split_amount,
transfer the newly split part to the sender address.


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_split_staked_sui">split_staked_sui</a>&lt;STABLE&gt;(stake: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_split_staked_sui">split_staked_sui</a>&lt;STABLE&gt;(stake: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;, split_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> TxContext) {
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="stable_pool.md#0x3_stable_pool_split">split</a>(stake, split_amount, ctx), <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x3_stable_pool_join_staked_sui"></a>

## Function `join_staked_sui`

Consume the staked sui other and add its value to self.
Aborts if some of the stable parameters are incompatible (pool id, stake activation epoch, etc.)


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_join_staked_sui">join_staked_sui</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;, other: <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_join_staked_sui">join_staked_sui</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;, other: <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;) {
    <b>assert</b>!(<a href="stable_pool.md#0x3_stable_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="stable_pool.md#0x3_stable_pool_EIncompatibleStakedSui">EIncompatibleStakedSui</a>);
    <b>let</b> <a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a> {
        id,
        pool_id: _,
        stake_activation_epoch: _,
        principal,
    } = other;

    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
    <a href="../sui-framework/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> self.principal, principal);
}
</code></pre>



</details>

<a name="0x3_stable_pool_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the stable parameters of the staked sui except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>&lt;STABLE&gt;(self: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;, other: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>&lt;STABLE&gt;(self: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;, other: &<a href="stable_pool.md#0x3_stable_pool_StakedStable">StakedStable</a>&lt;STABLE&gt;): bool {
    (self.pool_id == other.pool_id) &&
        (self.stake_activation_epoch == other.stake_activation_epoch)
}
</code></pre>



</details>

<a name="0x3_stable_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a> {
    // If the pool is preactive then the exchange rate is always 1:1.
    <b>if</b> (<a href="stable_pool.md#0x3_stable_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>(pool, epoch)) {
        <b>return</b> <a href="stable_pool.md#0x3_stable_pool_initial_exchange_rate">initial_exchange_rate</a>()
    };
    <b>let</b> clamped_epoch = <a href="../move-stdlib/option.md#0x1_option_get_with_default">option::get_with_default</a>(&pool.deactivation_epoch, epoch);
    <b>let</b> <b>mut</b> epoch = std::u64::min(clamped_epoch, epoch);
    <b>let</b> activation_epoch = *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&pool.activation_epoch);

    // Find the latest epoch that's earlier than the given epoch <b>with</b> an entry in the <a href="../sui-framework/table.md#0x2_table">table</a>
    <b>while</b> (epoch &gt;= activation_epoch) {
        <b>if</b> (<a href="../sui-framework/table.md#0x2_table_contains">table::contains</a>(&pool.exchange_rates, epoch)) {
            <b>return</b> *<a href="../sui-framework/table.md#0x2_table_borrow">table::borrow</a>(&pool.exchange_rates, epoch)
        };
        epoch = epoch - 1;
    };
    // This line really should be unreachable. Do we want an <b>assert</b> <b>false</b> here?
    <a href="stable_pool.md#0x3_stable_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="0x3_stable_pool_pending_stake_amount"></a>

## Function `pending_stake_amount`

Returns the total value of the pending stable requests for this stable pool.


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pending_stake_amount">pending_stake_amount</a>&lt;STABLE&gt;(<a href="stable_pool.md#0x3_stable_pool">stable_pool</a>: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pending_stake_amount">pending_stake_amount</a>&lt;STABLE&gt;(<a href="stable_pool.md#0x3_stable_pool">stable_pool</a>: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="stable_pool.md#0x3_stable_pool">stable_pool</a>.pending_stake
}
</code></pre>



</details>

<a name="0x3_stable_pool_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`

Returns the total withdrawal from the stable pool this epoch.


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>&lt;STABLE&gt;(<a href="stable_pool.md#0x3_stable_pool">stable_pool</a>: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>&lt;STABLE&gt;(<a href="stable_pool.md#0x3_stable_pool">stable_pool</a>: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <a href="stable_pool.md#0x3_stable_pool">stable_pool</a>.pending_total_sui_withdraw
}
</code></pre>



</details>

<a name="0x3_stable_pool_exchange_rates"></a>

## Function `exchange_rates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_exchange_rates">exchange_rates</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;): &<a href="../sui-framework/table.md#0x2_table_Table">table::Table</a>&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_exchange_rates">exchange_rates</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;): &Table&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a>&gt; {
    &pool.exchange_rates
}
</code></pre>



</details>

<a name="0x3_stable_pool_sui_amount"></a>

## Function `sui_amount`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_sui_amount">sui_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_sui_amount">sui_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    exchange_rate.sui_amount
}
</code></pre>



</details>

<a name="0x3_stable_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    exchange_rate.pool_token_amount
}
</code></pre>



</details>

<a name="0x3_stable_pool_is_preactive_at_epoch"></a>

## Function `is_preactive_at_epoch`

Returns true if the provided stable pool is preactive at the provided epoch.


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_is_preactive_at_epoch">is_preactive_at_epoch</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool{
    // Either the pool is currently preactive or the pool's starting epoch is later than the provided epoch.
    <a href="stable_pool.md#0x3_stable_pool_is_preactive">is_preactive</a>(pool) || (*<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&pool.activation_epoch) &gt; epoch)
}
</code></pre>



</details>

<a name="0x3_stable_pool_get_sui_amount"></a>

## Function `get_sui_amount`



<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_get_sui_amount">get_sui_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>, token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_get_sui_amount">get_sui_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a>, token_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    // When either amount is 0, that means we have no stakes <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.sui_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> token_amount
    };
    <b>let</b> res = (exchange_rate.sui_amount <b>as</b> u128)
        * (token_amount <b>as</b> u128)
        / (exchange_rate.pool_token_amount <b>as</b> u128);
    (res <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
}
</code></pre>



</details>

<a name="0x3_stable_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>, sui_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a>, sui_amount: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    // When either amount is 0, that means we have no stakes <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.sui_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> sui_amount
    };
    <b>let</b> res = (exchange_rate.pool_token_amount <b>as</b> u128)
        * (sui_amount <b>as</b> u128)
        / (exchange_rate.sui_amount <b>as</b> u128);
    (res <b>as</b> <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
}
</code></pre>



</details>

<a name="0x3_stable_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a> {
    <a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">PoolStableTokenExchangeRate</a> { sui_amount: 0, pool_token_amount: 0 }
}
</code></pre>



</details>

<a name="0x3_stable_pool_check_balance_invariants"></a>

## Function `check_balance_invariants`



<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_check_balance_invariants">check_balance_invariants</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_pool.md#0x3_stable_pool_check_balance_invariants">check_balance_invariants</a>&lt;STABLE&gt;(pool: &<a href="stable_pool.md#0x3_stable_pool_StablePool">StablePool</a>&lt;STABLE&gt;, epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    <b>let</b> exchange_rate = <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(pool, epoch);
    // check that the pool token <a href="../sui-framework/balance.md#0x2_balance">balance</a> and sui <a href="../sui-framework/balance.md#0x2_balance">balance</a> ratio matches the exchange rate stored.
    <b>let</b> expected = <a href="stable_pool.md#0x3_stable_pool_get_token_amount">get_token_amount</a>(&exchange_rate, pool.stable_balance);
    <b>let</b> actual = pool.pool_token_balance;
    <b>assert</b>!(expected == actual, <a href="stable_pool.md#0x3_stable_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>)
}
</code></pre>



</details>

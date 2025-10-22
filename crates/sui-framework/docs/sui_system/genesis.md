---
title: Module `sui_system::genesis`
---



-  [Struct `GenesisValidatorMetadata`](#sui_system_genesis_GenesisValidatorMetadata)
-  [Struct `GenesisChainParameters`](#sui_system_genesis_GenesisChainParameters)
-  [Struct `TokenDistributionSchedule`](#sui_system_genesis_TokenDistributionSchedule)
-  [Struct `TokenAllocation`](#sui_system_genesis_TokenAllocation)
-  [Constants](#@Constants_0)
-  [Function `create`](#sui_system_genesis_create)
-  [Function `allocate_tokens`](#sui_system_genesis_allocate_tokens)
-  [Function `activate_validators`](#sui_system_genesis_activate_validators)


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
<b>use</b> <a href="../sui/bfc.md#sui_bfc">sui::bfc</a>;
<b>use</b> <a href="../sui/clock.md#sui_clock">sui::clock</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/curve.md#sui_curve">sui::curve</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
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



<a name="sui_system_genesis_GenesisValidatorMetadata"></a>

## Struct `GenesisValidatorMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../sui_system/genesis.md#sui_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>name: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>description: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>image_url: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>project_url: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>sui_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>gas_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>commission_rate: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>protocol_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proof_of_possession: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>network_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>worker_public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>network_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>p2p_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>primary_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>worker_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_system_genesis_GenesisChainParameters"></a>

## Struct `GenesisChainParameters`



<pre><code><b>public</b> <b>struct</b> <a href="../sui_system/genesis.md#sui_system_genesis_GenesisChainParameters">GenesisChainParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>protocol_version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>chain_start_timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>epoch_duration_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_start_epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_initial_distribution_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_subsidy_decrease_rate: u16</code>
</dt>
<dd>
</dd>
<dt>
<code>max_validator_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>min_validator_joining_stake: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_low_stake_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_very_low_stake_threshold: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>validator_low_stake_grace_period: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_system_genesis_TokenDistributionSchedule"></a>

## Struct `TokenDistributionSchedule`



<pre><code><b>public</b> <b>struct</b> <a href="../sui_system/genesis.md#sui_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>stake_subsidy_fund_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>allocations: vector&lt;<a href="../sui_system/genesis.md#sui_system_genesis_TokenAllocation">sui_system::genesis::TokenAllocation</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_system_genesis_TokenAllocation"></a>

## Struct `TokenAllocation`



<pre><code><b>public</b> <b>struct</b> <a href="../sui_system/genesis.md#sui_system_genesis_TokenAllocation">TokenAllocation</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>recipient_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount_mist: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>staked_with_validator: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 Indicates if this allocation should be staked at genesis and with which validator
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="sui_system_genesis_ENotCalledAtGenesis"></a>

The <code><a href="../sui_system/genesis.md#sui_system_genesis_create">create</a></code> function was called at a non-genesis epoch.


<pre><code><b>const</b> <a href="../sui_system/genesis.md#sui_system_genesis_ENotCalledAtGenesis">ENotCalledAtGenesis</a>: u64 = 0;
</code></pre>



<a name="sui_system_genesis_EDuplicateValidator"></a>

The <code><a href="../sui_system/genesis.md#sui_system_genesis_create">create</a></code> function was called with duplicate validators.


<pre><code><b>const</b> <a href="../sui_system/genesis.md#sui_system_genesis_EDuplicateValidator">EDuplicateValidator</a>: u64 = 1;
</code></pre>



<a name="sui_system_genesis_create"></a>

## Function `create`

This function will be explicitly called once at genesis.
It will create a singleton SuiSystemState object, which contains
all the information we need in the system.


<pre><code><b>fun</b> <a href="../sui_system/genesis.md#sui_system_genesis_create">create</a>(sui_system_state_id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, bfc_system_state_id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, sui_supply: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/bfc.md#sui_bfc_BFC">sui::bfc::BFC</a>&gt;, genesis_chain_parameters: <a href="../sui_system/genesis.md#sui_system_genesis_GenesisChainParameters">sui_system::genesis::GenesisChainParameters</a>, genesis_validators: vector&lt;<a href="../sui_system/genesis.md#sui_system_genesis_GenesisValidatorMetadata">sui_system::genesis::GenesisValidatorMetadata</a>&gt;, token_distribution_schedule: <a href="../sui_system/genesis.md#sui_system_genesis_TokenDistributionSchedule">sui_system::genesis::TokenDistributionSchedule</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui_system/genesis.md#sui_system_genesis_create">create</a>(
    sui_system_state_id: UID,
    bfc_system_state_id: UID,
    <b>mut</b> sui_supply: Balance&lt;BFC&gt;,
    genesis_chain_parameters: <a href="../sui_system/genesis.md#sui_system_genesis_GenesisChainParameters">GenesisChainParameters</a>,
    genesis_validators: vector&lt;<a href="../sui_system/genesis.md#sui_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a>&gt;,
    token_distribution_schedule: <a href="../sui_system/genesis.md#sui_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a>,
    ctx: &<b>mut</b> TxContext,
) {
    // Ensure this is only called at <a href="../sui_system/genesis.md#sui_system_genesis">genesis</a>
    <b>assert</b>!(ctx.epoch() == 0, <a href="../sui_system/genesis.md#sui_system_genesis_ENotCalledAtGenesis">ENotCalledAtGenesis</a>);
// Create all the `Validator` structs
<b>let</b> <b>mut</b> validators = vector[];
genesis_validators.do!(|genesis_validator| {
    <b>let</b> <a href="../sui_system/genesis.md#sui_system_genesis_GenesisValidatorMetadata">GenesisValidatorMetadata</a> {
        name,
        description,
        image_url,
        project_url,
        sui_address,
        gas_price,
        commission_rate,
        protocol_public_key,
        proof_of_possession,
        network_public_key,
        worker_public_key,
        network_address,
        p2p_address,
        primary_address,
        worker_address,
    } = genesis_validator;
    <b>let</b> <a href="../sui_system/validator.md#sui_system_validator">validator</a> = <a href="../sui_system/validator.md#sui_system_validator_new">validator::new</a>(
        sui_address,
        protocol_public_key,
        network_public_key,
        worker_public_key,
        proof_of_possession,
        name,
        description,
        image_url,
        project_url,
        network_address,
        p2p_address,
        primary_address,
        worker_address,
        gas_price,
        commission_rate,
        ctx,
    );
    // Ensure that each <a href="../sui_system/validator.md#sui_system_validator">validator</a> is unique
    <b>assert</b>!(
        !<a href="../sui_system/validator_set.md#sui_system_validator_set_is_duplicate_validator">validator_set::is_duplicate_validator</a>(&validators, &<a href="../sui_system/validator.md#sui_system_validator">validator</a>),
        <a href="../sui_system/genesis.md#sui_system_genesis_EDuplicateValidator">EDuplicateValidator</a>,
    );
    validators.push_back(<a href="../sui_system/validator.md#sui_system_validator">validator</a>);
});
<b>let</b> <a href="../sui_system/genesis.md#sui_system_genesis_TokenDistributionSchedule">TokenDistributionSchedule</a> {
    stake_subsidy_fund_mist,
    allocations,
} = token_distribution_schedule;
<b>let</b> subsidy_fund = sui_supply.split(stake_subsidy_fund_mist);
<b>let</b> <a href="../sui_system/storage_fund.md#sui_system_storage_fund">storage_fund</a> = balance::zero();
// Allocate tokens and staking operations
<a href="../sui_system/genesis.md#sui_system_genesis_allocate_tokens">allocate_tokens</a>(sui_supply, allocations, &<b>mut</b> validators, ctx);
// Activate all validators
<a href="../sui_system/genesis.md#sui_system_genesis_activate_validators">activate_validators</a>(&<b>mut</b> validators);
<b>let</b> system_parameters = <a href="../sui_system/sui_system_state_inner.md#sui_system_sui_system_state_inner_create_system_parameters">sui_system_state_inner::create_system_parameters</a>(
    genesis_chain_parameters.epoch_duration_ms,
    genesis_chain_parameters.stake_subsidy_start_epoch,
    // Validator committee parameters
    genesis_chain_parameters.max_validator_count,
    genesis_chain_parameters.min_validator_joining_stake,
    genesis_chain_parameters.validator_low_stake_threshold,
    genesis_chain_parameters.validator_very_low_stake_threshold,
    genesis_chain_parameters.validator_low_stake_grace_period,
    ctx,
);
<b>let</b> <a href="../sui_system/stake_subsidy.md#sui_system_stake_subsidy">stake_subsidy</a> = <a href="../sui_system/stake_subsidy.md#sui_system_stake_subsidy_create">stake_subsidy::create</a>(
    subsidy_fund,
    genesis_chain_parameters.stake_subsidy_initial_distribution_amount,
    genesis_chain_parameters.stake_subsidy_period_length,
    genesis_chain_parameters.stake_subsidy_decrease_rate,
    ctx,
);
sui_system::create(
    sui_system_state_id,
    bfc_system_state_id,
    validators,
    <a href="../sui_system/storage_fund.md#sui_system_storage_fund">storage_fund</a>,
    genesis_chain_parameters.protocol_version,
    genesis_chain_parameters.chain_start_timestamp_ms,
    system_parameters,
    <a href="../sui_system/stake_subsidy.md#sui_system_stake_subsidy">stake_subsidy</a>,
    ctx,
);
}
</code></pre>



</details>

<a name="sui_system_genesis_allocate_tokens"></a>

## Function `allocate_tokens`



<pre><code><b>fun</b> <a href="../sui_system/genesis.md#sui_system_genesis_allocate_tokens">allocate_tokens</a>(sui_supply: <a href="../sui/balance.md#sui_balance_Balance">sui::balance::Balance</a>&lt;<a href="../sui/bfc.md#sui_bfc_BFC">sui::bfc::BFC</a>&gt;, allocations: vector&lt;<a href="../sui_system/genesis.md#sui_system_genesis_TokenAllocation">sui_system::genesis::TokenAllocation</a>&gt;, validators: &<b>mut</b> vector&lt;<a href="../sui_system/validator.md#sui_system_validator_Validator">sui_system::validator::Validator</a>&gt;, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui_system/genesis.md#sui_system_genesis_allocate_tokens">allocate_tokens</a>(
    <b>mut</b> sui_supply: Balance&lt;BFC&gt;,
    allocations: vector&lt;<a href="../sui_system/genesis.md#sui_system_genesis_TokenAllocation">TokenAllocation</a>&gt;,
    validators: &<b>mut</b> vector&lt;Validator&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    allocations.destroy!(
        |<a href="../sui_system/genesis.md#sui_system_genesis_TokenAllocation">TokenAllocation</a> { recipient_address, amount_mist, staked_with_validator }| {
            <b>let</b> allocation_balance = sui_supply.split(amount_mist);
            <b>if</b> (staked_with_validator.is_some()) {
                <b>let</b> validator_address = staked_with_validator.destroy_some();
                <b>let</b> <a href="../sui_system/validator.md#sui_system_validator">validator</a> = <a href="../sui_system/validator_set.md#sui_system_validator_set_get_validator_mut">validator_set::get_validator_mut</a>(validators, validator_address);
                <a href="../sui_system/validator.md#sui_system_validator">validator</a>.request_add_stake_at_genesis(
                    allocation_balance,
                    recipient_address,
                    ctx,
                );
            } <b>else</b> {
                bfc::transfer(
                    allocation_balance.into_coin(ctx),
                    recipient_address,
                );
            };
        },
    );
    // should be none left at this point.
    // Provided allocations must fully allocate the sui_supply and there
    sui_supply.destroy_zero();
}
</code></pre>



</details>

<a name="sui_system_genesis_activate_validators"></a>

## Function `activate_validators`



<pre><code><b>fun</b> <a href="../sui_system/genesis.md#sui_system_genesis_activate_validators">activate_validators</a>(validators: &<b>mut</b> vector&lt;<a href="../sui_system/validator.md#sui_system_validator_Validator">sui_system::validator::Validator</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui_system/genesis.md#sui_system_genesis_activate_validators">activate_validators</a>(validators: &<b>mut</b> vector&lt;Validator&gt;) {
    // Activate all <a href="../sui_system/genesis.md#sui_system_genesis">genesis</a> validators
    <b>let</b> count = validators.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        <b>let</b> <a href="../sui_system/validator.md#sui_system_validator">validator</a> =  &<b>mut</b> validators[i];
        <a href="../sui_system/validator.md#sui_system_validator">validator</a>.activate(0);
        <a href="../sui_system/validator.md#sui_system_validator_activate_stable">validator::activate_stable</a>(<a href="../sui_system/validator.md#sui_system_validator">validator</a>, 0);
        i = i + 1;
    };
}
</code></pre>



</details>

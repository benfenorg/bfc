
<a name="0x3_validator"></a>

# Module `0x3::validator`



-  [Struct `ValidatorMetadata`](#0x3_validator_ValidatorMetadata)
-  [Struct `Validator`](#0x3_validator_Validator)
-  [Struct `StakingRequestEvent`](#0x3_validator_StakingRequestEvent)
-  [Struct `UnstakingRequestEvent`](#0x3_validator_UnstakingRequestEvent)
-  [Constants](#@Constants_0)
-  [Function `new_metadata`](#0x3_validator_new_metadata)
-  [Function `new`](#0x3_validator_new)
-  [Function `deactivate`](#0x3_validator_deactivate)
-  [Function `deactivate_stable`](#0x3_validator_deactivate_stable)
-  [Function `activate`](#0x3_validator_activate)
-  [Function `activate_stable`](#0x3_validator_activate_stable)
-  [Function `activate_stable_`](#0x3_validator_activate_stable_)
-  [Function `adjust_stake_and_gas_price`](#0x3_validator_adjust_stake_and_gas_price)
-  [Function `request_add_stake`](#0x3_validator_request_add_stake)
-  [Function `get_stable_pool_mut`](#0x3_validator_get_stable_pool_mut)
-  [Function `get_stable_pool`](#0x3_validator_get_stable_pool)
-  [Function `request_add_stable_stake`](#0x3_validator_request_add_stable_stake)
-  [Function `request_add_stake_at_genesis`](#0x3_validator_request_add_stake_at_genesis)
-  [Function `request_withdraw_stake`](#0x3_validator_request_withdraw_stake)
-  [Function `request_withdraw_stable_stake`](#0x3_validator_request_withdraw_stable_stake)
-  [Function `request_set_gas_price`](#0x3_validator_request_set_gas_price)
-  [Function `set_candidate_gas_price`](#0x3_validator_set_candidate_gas_price)
-  [Function `request_set_commission_rate`](#0x3_validator_request_set_commission_rate)
-  [Function `set_candidate_commission_rate`](#0x3_validator_set_candidate_commission_rate)
-  [Function `deposit_stake_rewards`](#0x3_validator_deposit_stake_rewards)
-  [Function `distribute_stable_pool_reward`](#0x3_validator_distribute_stable_pool_reward)
-  [Function `get_stable_staking_total`](#0x3_validator_get_stable_staking_total)
-  [Function `deposit_stable_stake_rewards`](#0x3_validator_deposit_stable_stake_rewards)
-  [Function `process_pending_stakes_and_withdraws`](#0x3_validator_process_pending_stakes_and_withdraws)
-  [Function `process_pending_all_stable_stakes_and_withdraws`](#0x3_validator_process_pending_all_stable_stakes_and_withdraws)
-  [Function `process_pending_stable_stakes_and_withdraws`](#0x3_validator_process_pending_stable_stakes_and_withdraws)
-  [Function `is_preactive`](#0x3_validator_is_preactive)
-  [Function `metadata`](#0x3_validator_metadata)
-  [Function `sui_address`](#0x3_validator_sui_address)
-  [Function `name`](#0x3_validator_name)
-  [Function `description`](#0x3_validator_description)
-  [Function `image_url`](#0x3_validator_image_url)
-  [Function `project_url`](#0x3_validator_project_url)
-  [Function `network_address`](#0x3_validator_network_address)
-  [Function `p2p_address`](#0x3_validator_p2p_address)
-  [Function `primary_address`](#0x3_validator_primary_address)
-  [Function `worker_address`](#0x3_validator_worker_address)
-  [Function `protocol_pubkey_bytes`](#0x3_validator_protocol_pubkey_bytes)
-  [Function `proof_of_possession`](#0x3_validator_proof_of_possession)
-  [Function `network_pubkey_bytes`](#0x3_validator_network_pubkey_bytes)
-  [Function `worker_pubkey_bytes`](#0x3_validator_worker_pubkey_bytes)
-  [Function `next_epoch_network_address`](#0x3_validator_next_epoch_network_address)
-  [Function `next_epoch_p2p_address`](#0x3_validator_next_epoch_p2p_address)
-  [Function `next_epoch_primary_address`](#0x3_validator_next_epoch_primary_address)
-  [Function `next_epoch_worker_address`](#0x3_validator_next_epoch_worker_address)
-  [Function `next_epoch_protocol_pubkey_bytes`](#0x3_validator_next_epoch_protocol_pubkey_bytes)
-  [Function `next_epoch_proof_of_possession`](#0x3_validator_next_epoch_proof_of_possession)
-  [Function `next_epoch_network_pubkey_bytes`](#0x3_validator_next_epoch_network_pubkey_bytes)
-  [Function `next_epoch_worker_pubkey_bytes`](#0x3_validator_next_epoch_worker_pubkey_bytes)
-  [Function `operation_cap_id`](#0x3_validator_operation_cap_id)
-  [Function `next_epoch_gas_price`](#0x3_validator_next_epoch_gas_price)
-  [Function `total_stake_amount`](#0x3_validator_total_stake_amount)
-  [Function `stake_amount`](#0x3_validator_stake_amount)
-  [Function `stable_stake_amount`](#0x3_validator_stable_stake_amount)
-  [Function `total_stake`](#0x3_validator_total_stake)
-  [Function `total_stake_with_all_stable`](#0x3_validator_total_stake_with_all_stable)
-  [Function `total_stake_for_reward`](#0x3_validator_total_stake_for_reward)
-  [Function `total_stake_of_stable`](#0x3_validator_total_stake_of_stable)
-  [Function `voting_power`](#0x3_validator_voting_power)
-  [Function `set_voting_power`](#0x3_validator_set_voting_power)
-  [Function `pending_stake_amount`](#0x3_validator_pending_stake_amount)
-  [Function `pending_stake_withdraw_amount`](#0x3_validator_pending_stake_withdraw_amount)
-  [Function `gas_price`](#0x3_validator_gas_price)
-  [Function `commission_rate`](#0x3_validator_commission_rate)
-  [Function `pool_token_exchange_rate_at_epoch`](#0x3_validator_pool_token_exchange_rate_at_epoch)
-  [Function `pool_stable_token_exchange_rate_at_epoch`](#0x3_validator_pool_stable_token_exchange_rate_at_epoch)
-  [Function `staking_pool_id`](#0x3_validator_staking_pool_id)
-  [Function `stable_pool_id`](#0x3_validator_stable_pool_id)
-  [Function `stable_pool`](#0x3_validator_stable_pool)
-  [Function `all_stable_pool_id`](#0x3_validator_all_stable_pool_id)
-  [Function `is_duplicate`](#0x3_validator_is_duplicate)
-  [Function `is_equal_some_and_value`](#0x3_validator_is_equal_some_and_value)
-  [Function `is_equal_some`](#0x3_validator_is_equal_some)
-  [Function `new_unverified_validator_operation_cap_and_transfer`](#0x3_validator_new_unverified_validator_operation_cap_and_transfer)
-  [Function `update_name`](#0x3_validator_update_name)
-  [Function `update_description`](#0x3_validator_update_description)
-  [Function `update_image_url`](#0x3_validator_update_image_url)
-  [Function `update_project_url`](#0x3_validator_update_project_url)
-  [Function `update_next_epoch_network_address`](#0x3_validator_update_next_epoch_network_address)
-  [Function `update_candidate_network_address`](#0x3_validator_update_candidate_network_address)
-  [Function `update_next_epoch_p2p_address`](#0x3_validator_update_next_epoch_p2p_address)
-  [Function `update_candidate_p2p_address`](#0x3_validator_update_candidate_p2p_address)
-  [Function `update_next_epoch_primary_address`](#0x3_validator_update_next_epoch_primary_address)
-  [Function `update_candidate_primary_address`](#0x3_validator_update_candidate_primary_address)
-  [Function `update_next_epoch_worker_address`](#0x3_validator_update_next_epoch_worker_address)
-  [Function `update_candidate_worker_address`](#0x3_validator_update_candidate_worker_address)
-  [Function `update_next_epoch_protocol_pubkey`](#0x3_validator_update_next_epoch_protocol_pubkey)
-  [Function `update_candidate_protocol_pubkey`](#0x3_validator_update_candidate_protocol_pubkey)
-  [Function `update_next_epoch_network_pubkey`](#0x3_validator_update_next_epoch_network_pubkey)
-  [Function `update_candidate_network_pubkey`](#0x3_validator_update_candidate_network_pubkey)
-  [Function `update_next_epoch_worker_pubkey`](#0x3_validator_update_next_epoch_worker_pubkey)
-  [Function `update_candidate_worker_pubkey`](#0x3_validator_update_candidate_worker_pubkey)
-  [Function `effectuate_staged_metadata`](#0x3_validator_effectuate_staged_metadata)
-  [Function `validate_metadata`](#0x3_validator_validate_metadata)
-  [Function `validate_metadata_bcs`](#0x3_validator_validate_metadata_bcs)
-  [Function `get_staking_pool_ref`](#0x3_validator_get_staking_pool_ref)
-  [Function `get_stable_pool_ref`](#0x3_validator_get_stable_pool_ref)
-  [Function `new_from_metadata`](#0x3_validator_new_from_metadata)
-  [Function `rate_vec_map`](#0x3_validator_rate_vec_map)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="">0x1::string</a>;
<b>use</b> <a href="">0x1::type_name</a>;
<b>use</b> <a href="">0x1::vector</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/url.md#0x2_url">0x2::url</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="stable_pool.md#0x3_stable_pool">0x3::stable_pool</a>;
<b>use</b> <a href="staking_pool.md#0x3_staking_pool">0x3::staking_pool</a>;
<b>use</b> <a href="validator_cap.md#0x3_validator_cap">0x3::validator_cap</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bars.md#0xc8_bars">0xc8::bars</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/baud.md#0xc8_baud">0xc8::baud</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bbrl.md#0xc8_bbrl">0xc8::bbrl</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bcad.md#0xc8_bcad">0xc8::bcad</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/beur.md#0xc8_beur">0xc8::beur</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bgbp.md#0xc8_bgbp">0xc8::bgbp</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bidr.md#0xc8_bidr">0xc8::bidr</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/binr.md#0xc8_binr">0xc8::binr</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bjpy.md#0xc8_bjpy">0xc8::bjpy</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bkrw.md#0xc8_bkrw">0xc8::bkrw</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bmxn.md#0xc8_bmxn">0xc8::bmxn</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/brub.md#0xc8_brub">0xc8::brub</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bsar.md#0xc8_bsar">0xc8::bsar</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/btry.md#0xc8_btry">0xc8::btry</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/busd.md#0xc8_busd">0xc8::busd</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/bzar.md#0xc8_bzar">0xc8::bzar</a>;
<b>use</b> <a href="../../../.././build/BfcSystem/docs/mgg.md#0xc8_mgg">0xc8::mgg</a>;
</code></pre>



<a name="0x3_validator_ValidatorMetadata"></a>

## Struct `ValidatorMetadata`



<pre><code><b>struct</b> <a href="validator.md#0x3_validator_ValidatorMetadata">ValidatorMetadata</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sui_address: <b>address</b></code>
</dt>
<dd>
 The Sui Address of the validator. This is the sender that created the Validator object,
 and also the address to send validator/coins to during withdraws.
</dd>
<dt>
<code>protocol_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 holds to sign transactions. For now, this is the same as AuthorityName.
</dd>
<dt>
<code>network_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes corresponding to the private key that the validator
 uses to establish TLS connections
</dd>
<dt>
<code>worker_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 The public key bytes correstponding to the Narwhal Worker
</dd>
<dt>
<code>proof_of_possession: <a href="">vector</a>&lt;u8&gt;</code>
</dt>
<dd>
 This is a proof that the validator has ownership of the private key
</dd>
<dt>
<code>name: <a href="_String">string::String</a></code>
</dt>
<dd>
 A unique human-readable name of this validator.
</dd>
<dt>
<code>description: <a href="_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>image_url: <a href="../../../.././build/Sui/docs/url.md#0x2_url_Url">url::Url</a></code>
</dt>
<dd>

</dd>
<dt>
<code>project_url: <a href="../../../.././build/Sui/docs/url.md#0x2_url_Url">url::Url</a></code>
</dt>
<dd>

</dd>
<dt>
<code>net_address: <a href="_String">string::String</a></code>
</dt>
<dd>
 The network address of the validator (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code>p2p_address: <a href="_String">string::String</a></code>
</dt>
<dd>
 The address of the validator used for p2p activities such as state sync (could also contain extra info such as port, DNS and etc.).
</dd>
<dt>
<code>primary_address: <a href="_String">string::String</a></code>
</dt>
<dd>
 The address of the narwhal primary
</dd>
<dt>
<code>worker_address: <a href="_String">string::String</a></code>
</dt>
<dd>
 The address of the narwhal worker
</dd>
<dt>
<code>next_epoch_protocol_pubkey_bytes: <a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>
 "next_epoch" metadata only takes effects in the next epoch.
 If none, current value will stay unchanged.
</dd>
<dt>
<code>next_epoch_proof_of_possession: <a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_network_pubkey_bytes: <a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_worker_pubkey_bytes: <a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_net_address: <a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_p2p_address: <a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_primary_address: <a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next_epoch_worker_address: <a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>extra_fields: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x3_validator_Validator"></a>

## Struct `Validator`



<pre><code><b>struct</b> <a href="validator.md#0x3_validator_Validator">Validator</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="validator.md#0x3_validator_ValidatorMetadata">validator::ValidatorMetadata</a></code>
</dt>
<dd>
 Summary of the validator.
</dd>
<dt>
<code><a href="voting_power.md#0x3_voting_power">voting_power</a>: u64</code>
</dt>
<dd>
 The voting power of this validator, which might be different from its
 stake amount.
</dd>
<dt>
<code>operation_cap_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 The ID of this validator's current valid <code>UnverifiedValidatorOperationCap</code>
</dd>
<dt>
<code>gas_price: u64</code>
</dt>
<dd>
 Gas price quote, updated only at end of epoch.
</dd>
<dt>
<code><a href="staking_pool.md#0x3_staking_pool">staking_pool</a>: <a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a></code>
</dt>
<dd>
 Staking pool for this validator.
</dd>
<dt>
<code>stable_pools: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 stable pool for this validator.
</dd>
<dt>
<code>commission_rate: u64</code>
</dt>
<dd>
 Commission rate of the validator, in basis point.
</dd>
<dt>
<code>next_epoch_stake: u64</code>
</dt>
<dd>
 Total amount of stake that would be active in the next epoch.
</dd>
<dt>
<code>next_epoch_stable_stake: <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;</code>
</dt>
<dd>
 Total amount of stable stake that would be active in the next epoch.
</dd>
<dt>
<code>next_epoch_gas_price: u64</code>
</dt>
<dd>
 This validator's gas price quote for the next epoch.
</dd>
<dt>
<code>next_epoch_commission_rate: u64</code>
</dt>
<dd>
 The commission rate of the validator starting the next epoch, in basis point.
</dd>
<dt>
<code>extra_fields: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="0x3_validator_StakingRequestEvent"></a>

## Struct `StakingRequestEvent`

Event emitted when a new stake request is received.


<pre><code><b>struct</b> <a href="validator.md#0x3_validator_StakingRequestEvent">StakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staker_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>epoch: u64</code>
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

<a name="0x3_validator_UnstakingRequestEvent"></a>

## Struct `UnstakingRequestEvent`

Event emitted when a new unstake request is received.


<pre><code><b>struct</b> <a href="validator.md#0x3_validator_UnstakingRequestEvent">UnstakingRequestEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>validator_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staker_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>stake_activation_epoch: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>unstaking_epoch: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>principal_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_validator_MAX_U64"></a>



<pre><code><b>const</b> <a href="validator.md#0x3_validator_MAX_U64">MAX_U64</a>: u128 = 18446744073709551615;
</code></pre>



<a name="0x3_validator_ECalledDuringNonGenesis"></a>

Function called during non-genesis times.


<pre><code><b>const</b> <a href="validator.md#0x3_validator_ECalledDuringNonGenesis">ECalledDuringNonGenesis</a>: u64 = 12;
</code></pre>



<a name="0x3_validator_ECommissionRateTooHigh"></a>

Commission rate set by the validator is higher than the threshold


<pre><code><b>const</b> <a href="validator.md#0x3_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>: u64 = 8;
</code></pre>



<a name="0x3_validator_EGasPriceHigherThanThreshold"></a>

Validator trying to set gas price higher than threshold.


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>: u64 = 102;
</code></pre>



<a name="0x3_validator_EInvalidCap"></a>

Capability code is not valid


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EInvalidCap">EInvalidCap</a>: u64 = 101;
</code></pre>



<a name="0x3_validator_EInvalidProofOfPossession"></a>

Invalid proof_of_possession field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EInvalidProofOfPossession">EInvalidProofOfPossession</a>: u64 = 0;
</code></pre>



<a name="0x3_validator_EInvalidStakeAmount"></a>

Stake amount is invalid or wrong.


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>: u64 = 11;
</code></pre>



<a name="0x3_validator_EMetadataInvalidNetAddr"></a>

Invalid net_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EMetadataInvalidNetAddr">EMetadataInvalidNetAddr</a>: u64 = 4;
</code></pre>



<a name="0x3_validator_EMetadataInvalidNetPubkey"></a>

Invalid network_pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EMetadataInvalidNetPubkey">EMetadataInvalidNetPubkey</a>: u64 = 2;
</code></pre>



<a name="0x3_validator_EMetadataInvalidP2pAddr"></a>

Invalid p2p_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EMetadataInvalidP2pAddr">EMetadataInvalidP2pAddr</a>: u64 = 5;
</code></pre>



<a name="0x3_validator_EMetadataInvalidPrimaryAddr"></a>

Invalid primary_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EMetadataInvalidPrimaryAddr">EMetadataInvalidPrimaryAddr</a>: u64 = 6;
</code></pre>



<a name="0x3_validator_EMetadataInvalidPubkey"></a>

Invalid pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EMetadataInvalidPubkey">EMetadataInvalidPubkey</a>: u64 = 1;
</code></pre>



<a name="0x3_validator_EMetadataInvalidWorkerAddr"></a>

Invalidworker_address field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EMetadataInvalidWorkerAddr">EMetadataInvalidWorkerAddr</a>: u64 = 7;
</code></pre>



<a name="0x3_validator_EMetadataInvalidWorkerPubkey"></a>

Invalid worker_pubkey_bytes field in ValidatorMetadata


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EMetadataInvalidWorkerPubkey">EMetadataInvalidWorkerPubkey</a>: u64 = 3;
</code></pre>



<a name="0x3_validator_ENewCapNotCreatedByValidatorItself"></a>

New Capability is not created by the validator itself


<pre><code><b>const</b> <a href="validator.md#0x3_validator_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>: u64 = 100;
</code></pre>



<a name="0x3_validator_ENotValidatorCandidate"></a>

Intended validator is not a candidate one.


<pre><code><b>const</b> <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>: u64 = 10;
</code></pre>



<a name="0x3_validator_EValidatorMetadataExceedingLengthLimit"></a>

Validator Metadata is too long


<pre><code><b>const</b> <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>: u64 = 9;
</code></pre>



<a name="0x3_validator_MAX_COMMISSION_RATE"></a>



<pre><code><b>const</b> <a href="validator.md#0x3_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>: u64 = 2000;
</code></pre>



<a name="0x3_validator_MAX_VALIDATOR_GAS_PRICE"></a>

Max gas price a validator can set is 100K MIST.


<pre><code><b>const</b> <a href="validator.md#0x3_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>: u64 = 100000;
</code></pre>



<a name="0x3_validator_MAX_VALIDATOR_METADATA_LENGTH"></a>



<pre><code><b>const</b> <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>: u64 = 256;
</code></pre>



<a name="0x3_validator_new_metadata"></a>

## Function `new_metadata`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_new_metadata">new_metadata</a>(sui_address: <b>address</b>, protocol_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;, worker_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;, proof_of_possession: <a href="">vector</a>&lt;u8&gt;, name: <a href="_String">string::String</a>, description: <a href="_String">string::String</a>, image_url: <a href="../../../.././build/Sui/docs/url.md#0x2_url_Url">url::Url</a>, project_url: <a href="../../../.././build/Sui/docs/url.md#0x2_url_Url">url::Url</a>, net_address: <a href="_String">string::String</a>, p2p_address: <a href="_String">string::String</a>, primary_address: <a href="_String">string::String</a>, worker_address: <a href="_String">string::String</a>, extra_fields: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_Bag">bag::Bag</a>): <a href="validator.md#0x3_validator_ValidatorMetadata">validator::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_new_metadata">new_metadata</a>(
    sui_address: <b>address</b>,
    protocol_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;,
    network_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;,
    worker_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;,
    proof_of_possession: <a href="">vector</a>&lt;u8&gt;,
    name: String,
    description: String,
    image_url: Url,
    project_url: Url,
    net_address: String,
    p2p_address: String,
    primary_address: String,
    worker_address: String,
    extra_fields: Bag,
): <a href="validator.md#0x3_validator_ValidatorMetadata">ValidatorMetadata</a> {
    <b>let</b> metadata = <a href="validator.md#0x3_validator_ValidatorMetadata">ValidatorMetadata</a> {
        sui_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        worker_pubkey_bytes,
        proof_of_possession,
        name,
        description,
        image_url,
        project_url,
        net_address,
        p2p_address,
        primary_address,
        worker_address,
        next_epoch_protocol_pubkey_bytes: <a href="_none">option::none</a>(),
        next_epoch_network_pubkey_bytes: <a href="_none">option::none</a>(),
        next_epoch_worker_pubkey_bytes: <a href="_none">option::none</a>(),
        next_epoch_proof_of_possession: <a href="_none">option::none</a>(),
        next_epoch_net_address: <a href="_none">option::none</a>(),
        next_epoch_p2p_address: <a href="_none">option::none</a>(),
        next_epoch_primary_address: <a href="_none">option::none</a>(),
        next_epoch_worker_address: <a href="_none">option::none</a>(),
        extra_fields,
    };
    metadata
}
</code></pre>



</details>

<a name="0x3_validator_new"></a>

## Function `new`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_new">new</a>(sui_address: <b>address</b>, protocol_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;, worker_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;, proof_of_possession: <a href="">vector</a>&lt;u8&gt;, name: <a href="">vector</a>&lt;u8&gt;, description: <a href="">vector</a>&lt;u8&gt;, image_url: <a href="">vector</a>&lt;u8&gt;, project_url: <a href="">vector</a>&lt;u8&gt;, net_address: <a href="">vector</a>&lt;u8&gt;, p2p_address: <a href="">vector</a>&lt;u8&gt;, primary_address: <a href="">vector</a>&lt;u8&gt;, worker_address: <a href="">vector</a>&lt;u8&gt;, gas_price: u64, commission_rate: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator.md#0x3_validator_Validator">validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_new">new</a>(
    sui_address: <b>address</b>,
    protocol_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;,
    network_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;,
    worker_pubkey_bytes: <a href="">vector</a>&lt;u8&gt;,
    proof_of_possession: <a href="">vector</a>&lt;u8&gt;,
    name: <a href="">vector</a>&lt;u8&gt;,
    description: <a href="">vector</a>&lt;u8&gt;,
    image_url: <a href="">vector</a>&lt;u8&gt;,
    project_url: <a href="">vector</a>&lt;u8&gt;,
    net_address: <a href="">vector</a>&lt;u8&gt;,
    p2p_address: <a href="">vector</a>&lt;u8&gt;,
    primary_address: <a href="">vector</a>&lt;u8&gt;,
    worker_address: <a href="">vector</a>&lt;u8&gt;,
    gas_price: u64,
    commission_rate: u64,
    ctx: &<b>mut</b> TxContext
): <a href="validator.md#0x3_validator_Validator">Validator</a> {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&net_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="_length">vector::length</a>(&p2p_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="_length">vector::length</a>(&primary_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="_length">vector::length</a>(&worker_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="_length">vector::length</a>(&name) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="_length">vector::length</a>(&description) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="_length">vector::length</a>(&image_url) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>
            && <a href="_length">vector::length</a>(&project_url) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>assert</b>!(<a href="validator.md#0x3_validator_commission_rate">commission_rate</a> &lt;= <a href="validator.md#0x3_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="validator.md#0x3_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    <b>assert</b>!(<a href="validator.md#0x3_validator_gas_price">gas_price</a> &lt; <a href="validator.md#0x3_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="validator.md#0x3_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);

    <b>let</b> metadata = <a href="validator.md#0x3_validator_new_metadata">new_metadata</a>(
        sui_address,
        protocol_pubkey_bytes,
        network_pubkey_bytes,
        worker_pubkey_bytes,
        proof_of_possession,
        <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(name)),
        <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(description)),
        <a href="../../../.././build/Sui/docs/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(image_url),
        <a href="../../../.././build/Sui/docs/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(project_url),
        <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(net_address)),
        <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(p2p_address)),
        <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(primary_address)),
        <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(worker_address)),
        <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_new">bag::new</a>(ctx),
    );

    // Checks that the keys & addresses & PoP are valid.
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&metadata);

    <a href="validator.md#0x3_validator_new_from_metadata">new_from_metadata</a>(
        metadata,
        gas_price,
        commission_rate,
        ctx
    )
}
</code></pre>



</details>

<a name="0x3_validator_deactivate"></a>

## Function `deactivate`

Deactivate this validator's staking pool


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deactivate">deactivate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deactivate">deactivate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, deactivation_epoch: u64) {
    <a href="staking_pool.md#0x3_staking_pool_deactivate_staking_pool">staking_pool::deactivate_staking_pool</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, deactivation_epoch);
}
</code></pre>



</details>

<a name="0x3_validator_deactivate_stable"></a>

## Function `deactivate_stable`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deactivate_stable">deactivate_stable</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, deactivation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deactivate_stable">deactivate_stable</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, deactivation_epoch: u64) {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <b>let</b> pool = <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;STABLE&gt;&gt;(&<b>mut</b> self.stable_pools, pool_key);
    <a href="stable_pool.md#0x3_stable_pool_deactivate_stable_pool">stable_pool::deactivate_stable_pool</a>(pool, deactivation_epoch);
}
</code></pre>



</details>

<a name="0x3_validator_activate"></a>

## Function `activate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_activate">activate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_activate">activate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, activation_epoch: u64) {
    <a href="staking_pool.md#0x3_staking_pool_activate_staking_pool">staking_pool::activate_staking_pool</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, activation_epoch);
}
</code></pre>



</details>

<a name="0x3_validator_activate_stable"></a>

## Function `activate_stable`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_activate_stable">activate_stable</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_activate_stable">activate_stable</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, activation_epoch: u64) {
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BUSD&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BARS&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BAUD&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BBRL&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BCAD&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BEUR&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BGBP&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BIDR&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BINR&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BKRW&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BMXN&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BRUB&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BSAR&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BTRY&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BZAR&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;BJPY&gt;(self, activation_epoch);
    <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;MGG&gt;(self, activation_epoch);
}
</code></pre>



</details>

<a name="0x3_validator_activate_stable_"></a>

## Function `activate_stable_`



<pre><code><b>fun</b> <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, activation_epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_activate_stable_">activate_stable_</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, activation_epoch: u64) {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <b>let</b> pool = <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;STABLE&gt;&gt;(&<b>mut</b> self.stable_pools, pool_key);
    <a href="stable_pool.md#0x3_stable_pool_activate_stable_pool">stable_pool::activate_stable_pool</a>(pool, activation_epoch);
}
</code></pre>



</details>

<a name="0x3_validator_adjust_stake_and_gas_price"></a>

## Function `adjust_stake_and_gas_price`

Process pending stake and pending withdraws, and update the gas price.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_adjust_stake_and_gas_price">adjust_stake_and_gas_price</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_adjust_stake_and_gas_price">adjust_stake_and_gas_price</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>) {
    self.gas_price = self.next_epoch_gas_price;
    self.commission_rate = self.next_epoch_commission_rate;
}
</code></pre>



</details>

<a name="0x3_validator_request_add_stake"></a>

## Function `request_add_stake`

Request to add stake to the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_add_stake">request_add_stake</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, stake: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, staker_address: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staking_pool.md#0x3_staking_pool_StakedBfc">staking_pool::StakedBfc</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_add_stake">request_add_stake</a>(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    stake: Balance&lt;BFC&gt;,
    staker_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) : StakedBfc {
    <b>let</b> stake_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&stake);
    <b>assert</b>!(stake_amount &gt; 0, <a href="validator.md#0x3_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    <b>let</b> stake_epoch = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) + 1;
    <b>let</b> staked_sui = <a href="staking_pool.md#0x3_staking_pool_request_add_stake">staking_pool::request_add_stake</a>(
        &<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, stake, stake_epoch, ctx
    );
    // Process stake right away <b>if</b> staking pool is preactive.
    <b>if</b> (<a href="staking_pool.md#0x3_staking_pool_is_preactive">staking_pool::is_preactive</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>)) {
        <a href="staking_pool.md#0x3_staking_pool_process_pending_stake">staking_pool::process_pending_stake</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>);
    };
    self.next_epoch_stake = self.next_epoch_stake + stake_amount;
    <a href="../../../.././build/BfcSystem/docs/event.md#0x2_event_emit">event::emit</a>(
        <a href="validator.md#0x3_validator_StakingRequestEvent">StakingRequestEvent</a> {
            pool_id: <a href="validator.md#0x3_validator_staking_pool_id">staking_pool_id</a>(self),
            validator_address: self.metadata.sui_address,
            staker_address,
            epoch: <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx),
            amount: stake_amount,
        }
    );
    staked_sui
}
</code></pre>



</details>

<a name="0x3_validator_get_stable_pool_mut"></a>

## Function `get_stable_pool_mut`



<pre><code><b>fun</b> <a href="validator.md#0x3_validator_get_stable_pool_mut">get_stable_pool_mut</a>&lt;STABLE&gt;(<a href="../../../.././build/Sui/docs/bag.md#0x2_bag">bag</a>: &<b>mut</b> <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_Bag">bag::Bag</a>): &<b>mut</b> <a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_get_stable_pool_mut">get_stable_pool_mut</a>&lt;STABLE&gt;(
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag">bag</a>: &<b>mut</b> Bag,
) :&<b>mut</b> StablePool&lt;STABLE&gt; {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;STABLE&gt;&gt;(<a href="../../../.././build/Sui/docs/bag.md#0x2_bag">bag</a>, pool_key)
}
</code></pre>



</details>

<a name="0x3_validator_get_stable_pool"></a>

## Function `get_stable_pool`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>&lt;STABLE&gt;(<a href="../../../.././build/Sui/docs/bag.md#0x2_bag">bag</a>: &<a href="../../../.././build/Sui/docs/bag.md#0x2_bag_Bag">bag::Bag</a>): &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>&lt;STABLE&gt;(<a href="../../../.././build/Sui/docs/bag.md#0x2_bag">bag</a>: &Bag) :&StablePool&lt;STABLE&gt; {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow">bag::borrow</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;STABLE&gt;&gt;(<a href="../../../.././build/Sui/docs/bag.md#0x2_bag">bag</a>, pool_key)
}
</code></pre>



</details>

<a name="0x3_validator_request_add_stable_stake"></a>

## Function `request_add_stable_stake`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_add_stable_stake">request_add_stable_stake</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, stake: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE&gt;, staker_address: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_add_stable_stake">request_add_stable_stake</a>&lt;STABLE&gt;(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    stake: Balance&lt;STABLE&gt;,
    staker_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) : StakedStable&lt;STABLE&gt; {
    <b>let</b> stake_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&stake);
    <b>assert</b>!(stake_amount &gt; 0, <a href="validator.md#0x3_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
    <b>let</b> stake_epoch = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) + 1;
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <b>let</b> pool = <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;STABLE&gt;&gt;(&<b>mut</b> self.stable_pools, pool_key);
    <b>let</b> staked_sui = <a href="stable_pool.md#0x3_stable_pool_request_add_stake">stable_pool::request_add_stake</a>&lt;STABLE&gt;(
        pool, stake, stake_epoch, ctx
    );
    // Process stake right away <b>if</b> stable pool is preactive.
    <b>if</b> (<a href="stable_pool.md#0x3_stable_pool_is_preactive">stable_pool::is_preactive</a>&lt;STABLE&gt;(pool)) {
        <a href="stable_pool.md#0x3_stable_pool_process_pending_stake">stable_pool::process_pending_stake</a>&lt;STABLE&gt;(pool);
    };
    <b>let</b> next_stable_stake = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&<b>mut</b> self.next_epoch_stable_stake, &pool_key);
    <b>if</b> (<a href="_is_none">option::is_none</a>(&next_stable_stake)) {
        <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> self.next_epoch_stable_stake, pool_key, stake_amount);
    } <b>else</b> {
        <b>let</b> (_, next_stable_stake) = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>(&<b>mut</b> self.next_epoch_stable_stake, &pool_key);
        <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> self.next_epoch_stable_stake, pool_key, stake_amount + next_stable_stake);
    };
    <a href="../../../.././build/BfcSystem/docs/event.md#0x2_event_emit">event::emit</a>(
        <a href="validator.md#0x3_validator_StakingRequestEvent">StakingRequestEvent</a> {
            pool_id: <a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;STABLE&gt;(self),
            validator_address: self.metadata.sui_address,
            staker_address,
            epoch: <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx),
            amount: stake_amount,
        }
    );
    staked_sui
}
</code></pre>



</details>

<a name="0x3_validator_request_add_stake_at_genesis"></a>

## Function `request_add_stake_at_genesis`

Request to add stake to the validator's staking pool at genesis


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_add_stake_at_genesis">request_add_stake_at_genesis</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, stake: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, staker_address: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_add_stake_at_genesis">request_add_stake_at_genesis</a>(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    stake: Balance&lt;BFC&gt;,
    staker_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="validator.md#0x3_validator_ECalledDuringNonGenesis">ECalledDuringNonGenesis</a>);
    <b>let</b> stake_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&stake);
    <b>assert</b>!(stake_amount &gt; 0, <a href="validator.md#0x3_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);

    <b>let</b> staked_sui = <a href="staking_pool.md#0x3_staking_pool_request_add_stake">staking_pool::request_add_stake</a>(
        &<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>,
        stake,
        0, // epoch 0 -- <a href="genesis.md#0x3_genesis">genesis</a>
        ctx
    );

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(staked_sui, staker_address);

    // Process stake right away
    <a href="staking_pool.md#0x3_staking_pool_process_pending_stake">staking_pool::process_pending_stake</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>);
    self.next_epoch_stake = self.next_epoch_stake + stake_amount;
}
</code></pre>



</details>

<a name="0x3_validator_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Request to withdraw stake from the validator's staking pool, processed at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_withdraw_stake">request_withdraw_stake</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, staked_sui: <a href="staking_pool.md#0x3_staking_pool_StakedBfc">staking_pool::StakedBfc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_withdraw_stake">request_withdraw_stake</a>(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    staked_sui: StakedBfc,
    ctx: &<b>mut</b> TxContext,
) : Balance&lt;BFC&gt; {
    <b>let</b> principal_amount = <a href="staking_pool.md#0x3_staking_pool_staked_sui_amount">staking_pool::staked_sui_amount</a>(&staked_sui);
    <b>let</b> stake_activation_epoch = <a href="staking_pool.md#0x3_staking_pool_stake_activation_epoch">staking_pool::stake_activation_epoch</a>(&staked_sui);
    <b>let</b> withdrawn_stake = <a href="staking_pool.md#0x3_staking_pool_request_withdraw_stake">staking_pool::request_withdraw_stake</a>(
            &<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, staked_sui, ctx);
    <b>let</b> withdraw_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&withdrawn_stake);
    <b>let</b> reward_amount = withdraw_amount - principal_amount;
    self.next_epoch_stake = self.next_epoch_stake - withdraw_amount;
    <a href="../../../.././build/BfcSystem/docs/event.md#0x2_event_emit">event::emit</a>(
        <a href="validator.md#0x3_validator_UnstakingRequestEvent">UnstakingRequestEvent</a> {
            pool_id: <a href="validator.md#0x3_validator_staking_pool_id">staking_pool_id</a>(self),
            validator_address: self.metadata.sui_address,
            staker_address: <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            stake_activation_epoch,
            unstaking_epoch: <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx),
            principal_amount,
            reward_amount,
        }
    );
    withdrawn_stake
}
</code></pre>



</details>

<a name="0x3_validator_request_withdraw_stable_stake"></a>

## Function `request_withdraw_stable_stake`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_withdraw_stable_stake">request_withdraw_stable_stake</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, staked_sui: <a href="stable_pool.md#0x3_stable_pool_StakedStable">stable_pool::StakedStable</a>&lt;STABLE&gt;, rate: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE&gt;, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_withdraw_stable_stake">request_withdraw_stable_stake</a>&lt;STABLE&gt;(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    staked_sui: StakedStable&lt;STABLE&gt;,
    rate: u64,
    ctx: &<b>mut</b> TxContext,
) : (Balance&lt;STABLE&gt;, Balance&lt;BFC&gt;) {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <b>let</b> pool = <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;STABLE&gt;&gt;(&<b>mut</b> self.stable_pools, pool_key);
    <b>let</b> principal_amount = <a href="stable_pool.md#0x3_stable_pool_staked_sui_amount">stable_pool::staked_sui_amount</a>(&staked_sui);
    <b>let</b> stake_activation_epoch = <a href="stable_pool.md#0x3_stable_pool_stake_activation_epoch">stable_pool::stake_activation_epoch</a>(&staked_sui);
    <b>let</b> (withdrawn_stake, reward) = <a href="stable_pool.md#0x3_stable_pool_request_withdraw_stake">stable_pool::request_withdraw_stake</a>(pool, staked_sui, rate, ctx);
    <b>let</b> withdraw_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&withdrawn_stake);
    <b>let</b> reward_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&reward);
    <b>let</b> next_stable_stake = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&<b>mut</b> self.next_epoch_stable_stake, &pool_key);
    <b>if</b> (<a href="_is_some">option::is_some</a>(&next_stable_stake)) {
        <b>let</b> (_, next_stable) = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>(&<b>mut</b> self.next_epoch_stable_stake, &pool_key);
        <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> self.next_epoch_stable_stake, pool_key,next_stable - withdraw_amount);
    };
    <a href="../../../.././build/BfcSystem/docs/event.md#0x2_event_emit">event::emit</a>(
        <a href="validator.md#0x3_validator_UnstakingRequestEvent">UnstakingRequestEvent</a> {
            pool_id: <a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;STABLE&gt;(self),
            validator_address: self.metadata.sui_address,
            staker_address: <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            stake_activation_epoch,
            unstaking_epoch: <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx),
            principal_amount,
            reward_amount,
        }
    );
    (withdrawn_stake, reward)
}
</code></pre>



</details>

<a name="0x3_validator_request_set_gas_price"></a>

## Function `request_set_gas_price`

Request to set new gas price for the next epoch.
Need to present a <code>ValidatorOperationCap</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_set_gas_price">request_set_gas_price</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, verified_cap: <a href="validator_cap.md#0x3_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_set_gas_price">request_set_gas_price</a>(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    verified_cap: ValidatorOperationCap,
    new_price: u64,
) {
    <b>assert</b>!(new_price &lt; <a href="validator.md#0x3_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="validator.md#0x3_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_address = *<a href="validator_cap.md#0x3_validator_cap_verified_operation_cap_address">validator_cap::verified_operation_cap_address</a>(&verified_cap);
    <b>assert</b>!(validator_address == self.metadata.sui_address, <a href="validator.md#0x3_validator_EInvalidCap">EInvalidCap</a>);
    self.next_epoch_gas_price = new_price;
}
</code></pre>



</details>

<a name="0x3_validator_set_candidate_gas_price"></a>

## Function `set_candidate_gas_price`

Set new gas price for the candidate validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_set_candidate_gas_price">set_candidate_gas_price</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, verified_cap: <a href="validator_cap.md#0x3_validator_cap_ValidatorOperationCap">validator_cap::ValidatorOperationCap</a>, new_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_set_candidate_gas_price">set_candidate_gas_price</a>(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    verified_cap: ValidatorOperationCap,
    new_price: u64
) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_price &lt; <a href="validator.md#0x3_validator_MAX_VALIDATOR_GAS_PRICE">MAX_VALIDATOR_GAS_PRICE</a>, <a href="validator.md#0x3_validator_EGasPriceHigherThanThreshold">EGasPriceHigherThanThreshold</a>);
    <b>let</b> validator_address = *<a href="validator_cap.md#0x3_validator_cap_verified_operation_cap_address">validator_cap::verified_operation_cap_address</a>(&verified_cap);
    <b>assert</b>!(validator_address == self.metadata.sui_address, <a href="validator.md#0x3_validator_EInvalidCap">EInvalidCap</a>);
    self.next_epoch_gas_price = new_price;
    self.gas_price = new_price;
}
</code></pre>



</details>

<a name="0x3_validator_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

Request to set new commission rate for the next epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, new_commission_rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_request_set_commission_rate">request_set_commission_rate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, new_commission_rate: u64) {
    <b>assert</b>!(new_commission_rate &lt;= <a href="validator.md#0x3_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="validator.md#0x3_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.next_epoch_commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="0x3_validator_set_candidate_commission_rate"></a>

## Function `set_candidate_commission_rate`

Set new commission rate for the candidate validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, new_commission_rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_set_candidate_commission_rate">set_candidate_commission_rate</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, new_commission_rate: u64) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(new_commission_rate &lt;= <a href="validator.md#0x3_validator_MAX_COMMISSION_RATE">MAX_COMMISSION_RATE</a>, <a href="validator.md#0x3_validator_ECommissionRateTooHigh">ECommissionRateTooHigh</a>);
    self.commission_rate = new_commission_rate;
}
</code></pre>



</details>

<a name="0x3_validator_deposit_stake_rewards"></a>

## Function `deposit_stake_rewards`

Deposit stakes rewards into the validator's staking pool, called at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, reward: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, stable_rate: &<a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deposit_stake_rewards">deposit_stake_rewards</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, reward: Balance&lt;BFC&gt;, stable_rate: &VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;) {
    <b>let</b> total_reward = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&reward);
    <b>let</b> bfc_reward = 0;
    <b>let</b> stable_total_stake = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>();
    <b>let</b> all_stable_total_stake = <a href="validator.md#0x3_validator_get_stable_staking_total">get_stable_staking_total</a>(self, &<b>mut</b> stable_total_stake, stable_rate);
    <b>if</b> (all_stable_total_stake &gt; 0) {
        //distribute for <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a> pool
        <b>let</b> bfc_total_stake = <a href="validator.md#0x3_validator_stake_amount">stake_amount</a>(self);
        <b>let</b> all_total_stake = all_stable_total_stake + bfc_total_stake;
        <b>let</b> bfc_dis_reward = (total_reward <b>as</b> u128) * (bfc_total_stake <b>as</b> u128) / (all_total_stake <b>as</b> u128);
        <a href="staking_pool.md#0x3_staking_pool_deposit_rewards">staking_pool::deposit_rewards</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, (bfc_dis_reward <b>as</b> u64)));
        bfc_reward = (bfc_dis_reward <b>as</b> u64);

        //distribute for stable pool
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BUSD&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b> (stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BUSD&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BARS&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BARS&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BAUD&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BAUD&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BBRL&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BBRL&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BCAD&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BCAD&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BEUR&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BEUR&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BGBP&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BGBP&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BIDR&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BIDR&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BINR&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BINR&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BKRW&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BKRW&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BMXN&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BMXN&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BRUB&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BRUB&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BSAR&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BSAR&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BTRY&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BTRY&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BZAR&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BZAR&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;BJPY&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;BJPY&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };
        <b>let</b> stable_reward = <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;MGG&gt;(&stable_total_stake,
            total_reward, all_total_stake);
        <b>if</b>(stable_reward &gt; 0) {
            <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;MGG&gt;(self, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> reward, stable_reward), stable_reward, stable_rate);
        };

        <b>let</b> remainder = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&reward);
        <b>if</b> (remainder &gt; 0) {
            <a href="staking_pool.md#0x3_staking_pool_deposit_rewards">staking_pool::deposit_rewards</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, reward);
            bfc_reward = bfc_reward + remainder;
        } <b>else</b> {
            <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(reward);
        };
    }<b>else</b> {
        <a href="staking_pool.md#0x3_staking_pool_deposit_rewards">staking_pool::deposit_rewards</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, reward);
        bfc_reward = total_reward;
    };

    self.next_epoch_stake = self.next_epoch_stake + bfc_reward;
}
</code></pre>



</details>

<a name="0x3_validator_distribute_stable_pool_reward"></a>

## Function `distribute_stable_pool_reward`



<pre><code><b>fun</b> <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;STABLE&gt;(stable_pool_total: &<a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;, reward_count: u64, all_total: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_distribute_stable_pool_reward">distribute_stable_pool_reward</a>&lt;STABLE&gt;(
    stable_pool_total: &VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;,
    reward_count: u64,
    all_total: u64,
): u64 {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <b>let</b> total_option = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(stable_pool_total, &pool_key);
    <b>if</b> (<a href="_is_some">option::is_some</a>(&total_option)) {
        <b>let</b> reward = (reward_count <b>as</b> u128) * (*<a href="_borrow">option::borrow</a>(&total_option) <b>as</b> u128) / (all_total <b>as</b> u128);
        (reward <b>as</b> u64)
    }
    <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="0x3_validator_get_stable_staking_total"></a>

## Function `get_stable_staking_total`



<pre><code><b>fun</b> <a href="validator.md#0x3_validator_get_stable_staking_total">get_stable_staking_total</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>, stable_total: &<b>mut</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;, stable_rate: &<a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_get_stable_staking_total">get_stable_staking_total</a>(
    self: &<a href="validator.md#0x3_validator_Validator">Validator</a>,
    stable_total: &<b>mut</b> VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;,
    stable_rate: &VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;
): u64 {
    <b>let</b> all_pool_total: u64 = 0;
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BUSD&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BARS&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BAUD&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BBRL&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BCAD&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BEUR&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BGBP&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BIDR&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BINR&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BKRW&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BMXN&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BRUB&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BSAR&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BTRY&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BZAR&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;BJPY&gt;(self, stable_total, stable_rate);
    all_pool_total = all_pool_total + <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;MGG&gt;(self, stable_total, stable_rate);
    all_pool_total
}
</code></pre>



</details>

<a name="0x3_validator_deposit_stable_stake_rewards"></a>

## Function `deposit_stable_stake_rewards`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, reward: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, reward_amount: u64, stable_rate: &<a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_deposit_stable_stake_rewards">deposit_stable_stake_rewards</a>&lt;STABLE&gt;(
    self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>,
    reward: Balance&lt;BFC&gt;,
    reward_amount: u64,
    stable_rate: &VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;,
) {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <b>let</b> rate = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get">vec_map::get</a>(stable_rate, &pool_key);
    //todo rate is zero?
    <b>let</b> stable_amount = (reward_amount <b>as</b> u128) *  (1000000000 <b>as</b> u128) / (*rate <b>as</b> u128);
    <b>if</b> (<a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&self.next_epoch_stable_stake, &pool_key)) {
        <b>let</b> next_stake = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get_mut">vec_map::get_mut</a>(&<b>mut</b> self.next_epoch_stable_stake, &pool_key);
        *next_stake = *next_stake + (stable_amount <b>as</b> u64);
    }<b>else</b> {
        <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> self.next_epoch_stable_stake, pool_key, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&reward));
    };
    <b>let</b> pool = <a href="validator.md#0x3_validator_get_stable_pool_mut">get_stable_pool_mut</a>&lt;STABLE&gt;(&<b>mut</b> self.stable_pools);

    <a href="stable_pool.md#0x3_stable_pool_deposit_rewards">stable_pool::deposit_rewards</a>&lt;STABLE&gt;(pool, reward, (stable_amount <b>as</b> u64));
}
</code></pre>



</details>

<a name="0x3_validator_process_pending_stakes_and_withdraws"></a>

## Function `process_pending_stakes_and_withdraws`

Process pending stakes and withdraws, called at the end of the epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_process_pending_stakes_and_withdraws">process_pending_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, ctx: &<b>mut</b> TxContext) {
    <a href="staking_pool.md#0x3_staking_pool_process_pending_stakes_and_withdraws">staking_pool::process_pending_stakes_and_withdraws</a>(&<b>mut</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, ctx);
    <b>assert</b>!(<a href="validator.md#0x3_validator_stake_amount">stake_amount</a>(self) == self.next_epoch_stake, <a href="validator.md#0x3_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
}
</code></pre>



</details>

<a name="0x3_validator_process_pending_all_stable_stakes_and_withdraws"></a>

## Function `process_pending_all_stable_stakes_and_withdraws`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_process_pending_all_stable_stakes_and_withdraws">process_pending_all_stable_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_process_pending_all_stable_stakes_and_withdraws">process_pending_all_stable_stakes_and_withdraws</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, ctx: &<b>mut</b> TxContext) {
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BUSD&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BARS&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BAUD&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BBRL&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BCAD&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BEUR&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BGBP&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BIDR&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BINR&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BKRW&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BMXN&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BRUB&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BSAR&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BTRY&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BZAR&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;BJPY&gt;(self, ctx);
    <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;MGG&gt;(self, ctx);
}
</code></pre>



</details>

<a name="0x3_validator_process_pending_stable_stakes_and_withdraws"></a>

## Function `process_pending_stable_stakes_and_withdraws`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_process_pending_stable_stakes_and_withdraws">process_pending_stable_stakes_and_withdraws</a>&lt;STABLE&gt;(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
    <b>let</b> pool = <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;STABLE&gt;&gt;(&<b>mut</b> self.stable_pools, pool_key);
    <a href="stable_pool.md#0x3_stable_pool_process_pending_stakes_and_withdraws">stable_pool::process_pending_stakes_and_withdraws</a>&lt;STABLE&gt;(pool, ctx);
    //todo add muiti stable pool
    // <b>assert</b>!(<a href="validator.md#0x3_validator_stable_stake_amount">stable_stake_amount</a>&lt;STABLE&gt;(self) == self.next_epoch_stable_stake, <a href="validator.md#0x3_validator_EInvalidStakeAmount">EInvalidStakeAmount</a>);
}
</code></pre>



</details>

<a name="0x3_validator_is_preactive"></a>

## Function `is_preactive`

Returns true if the validator is preactive.


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): bool {
    <a href="staking_pool.md#0x3_staking_pool_is_preactive">staking_pool::is_preactive</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<a name="0x3_validator_metadata"></a>

## Function `metadata`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_metadata">metadata</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="validator.md#0x3_validator_ValidatorMetadata">validator::ValidatorMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_metadata">metadata</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &<a href="validator.md#0x3_validator_ValidatorMetadata">ValidatorMetadata</a> {
    &self.metadata
}
</code></pre>



</details>

<a name="0x3_validator_sui_address"></a>

## Function `sui_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_sui_address">sui_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_sui_address">sui_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): <b>address</b> {
    self.metadata.sui_address
}
</code></pre>



</details>

<a name="0x3_validator_name"></a>

## Function `name`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_name">name</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_name">name</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &String {
    &self.metadata.name
}
</code></pre>



</details>

<a name="0x3_validator_description"></a>

## Function `description`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_description">description</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_description">description</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &String {
    &self.metadata.description
}
</code></pre>



</details>

<a name="0x3_validator_image_url"></a>

## Function `image_url`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_image_url">image_url</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="../../../.././build/Sui/docs/url.md#0x2_url_Url">url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_image_url">image_url</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Url {
    &self.metadata.image_url
}
</code></pre>



</details>

<a name="0x3_validator_project_url"></a>

## Function `project_url`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_project_url">project_url</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="../../../.././build/Sui/docs/url.md#0x2_url_Url">url::Url</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_project_url">project_url</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Url {
    &self.metadata.project_url
}
</code></pre>



</details>

<a name="0x3_validator_network_address"></a>

## Function `network_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_network_address">network_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_network_address">network_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &String {
    &self.metadata.net_address
}
</code></pre>



</details>

<a name="0x3_validator_p2p_address"></a>

## Function `p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_p2p_address">p2p_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_p2p_address">p2p_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &String {
    &self.metadata.p2p_address
}
</code></pre>



</details>

<a name="0x3_validator_primary_address"></a>

## Function `primary_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_primary_address">primary_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_primary_address">primary_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &String {
    &self.metadata.primary_address
}
</code></pre>



</details>

<a name="0x3_validator_worker_address"></a>

## Function `worker_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_worker_address">worker_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_worker_address">worker_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &String {
    &self.metadata.worker_address
}
</code></pre>



</details>

<a name="0x3_validator_protocol_pubkey_bytes"></a>

## Function `protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_protocol_pubkey_bytes">protocol_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &<a href="">vector</a>&lt;u8&gt; {
    &self.metadata.protocol_pubkey_bytes
}
</code></pre>



</details>

<a name="0x3_validator_proof_of_possession"></a>

## Function `proof_of_possession`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_proof_of_possession">proof_of_possession</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_proof_of_possession">proof_of_possession</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &<a href="">vector</a>&lt;u8&gt; {
    &self.metadata.proof_of_possession
}
</code></pre>



</details>

<a name="0x3_validator_network_pubkey_bytes"></a>

## Function `network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_network_pubkey_bytes">network_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &<a href="">vector</a>&lt;u8&gt; {
    &self.metadata.network_pubkey_bytes
}
</code></pre>



</details>

<a name="0x3_validator_worker_pubkey_bytes"></a>

## Function `worker_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_worker_pubkey_bytes">worker_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &<a href="">vector</a>&lt;u8&gt; {
    &self.metadata.worker_pubkey_bytes
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_network_address"></a>

## Function `next_epoch_network_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_network_address">next_epoch_network_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.metadata.next_epoch_net_address
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_p2p_address"></a>

## Function `next_epoch_p2p_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.metadata.next_epoch_p2p_address
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_primary_address"></a>

## Function `next_epoch_primary_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_primary_address">next_epoch_primary_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_primary_address">next_epoch_primary_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.metadata.next_epoch_primary_address
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_worker_address"></a>

## Function `next_epoch_worker_address`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_worker_address">next_epoch_worker_address</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_worker_address">next_epoch_worker_address</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;String&gt; {
    &self.metadata.next_epoch_worker_address
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_protocol_pubkey_bytes"></a>

## Function `next_epoch_protocol_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;<a href="">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_protocol_pubkey_bytes
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_proof_of_possession"></a>

## Function `next_epoch_proof_of_possession`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_proof_of_possession">next_epoch_proof_of_possession</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;<a href="">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_proof_of_possession
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_network_pubkey_bytes"></a>

## Function `next_epoch_network_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;<a href="">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_network_pubkey_bytes
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_worker_pubkey_bytes"></a>

## Function `next_epoch_worker_pubkey_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="_Option">option::Option</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &Option&lt;<a href="">vector</a>&lt;u8&gt;&gt; {
    &self.metadata.next_epoch_worker_pubkey_bytes
}
</code></pre>



</details>

<a name="0x3_validator_operation_cap_id"></a>

## Function `operation_cap_id`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_operation_cap_id">operation_cap_id</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_operation_cap_id">operation_cap_id</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &ID {
    &self.operation_cap_id
}
</code></pre>



</details>

<a name="0x3_validator_next_epoch_gas_price"></a>

## Function `next_epoch_gas_price`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_gas_price">next_epoch_gas_price</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_next_epoch_gas_price">next_epoch_gas_price</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    self.next_epoch_gas_price
}
</code></pre>



</details>

<a name="0x3_validator_total_stake_amount"></a>

## Function `total_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake_amount">total_stake_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake_amount">total_stake_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    <b>spec</b> {
        // TODO: this should be provable rather than assumed
        <b>assume</b> self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>.sui_balance &lt;= <a href="validator.md#0x3_validator_MAX_U64">MAX_U64</a>;
    };
    <a href="staking_pool.md#0x3_staking_pool_sui_balance">staking_pool::sui_balance</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0x3_validator_stake_amount"></a>

## Function `stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_stake_amount">stake_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_stake_amount">stake_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    <a href="staking_pool.md#0x3_staking_pool_sui_balance">staking_pool::sui_balance</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<a name="0x3_validator_stable_stake_amount"></a>

## Function `stable_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_stable_stake_amount">stable_stake_amount</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_stable_stake_amount">stable_stake_amount</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    <a href="stable_pool.md#0x3_stable_pool_stable_balance">stable_pool::stable_balance</a>(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>&lt;STABLE&gt;(&self.stable_pools))
}
</code></pre>



</details>

<a name="0x3_validator_total_stake"></a>

## Function `total_stake`

Return the total amount staked with this validator


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake">total_stake</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake">total_stake</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    <a href="validator.md#0x3_validator_stake_amount">stake_amount</a>(self)
}
</code></pre>



</details>

<a name="0x3_validator_total_stake_with_all_stable"></a>

## Function `total_stake_with_all_stable`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake_with_all_stable">total_stake_with_all_stable</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>, stable_rate: <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake_with_all_stable">total_stake_with_all_stable</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>, stable_rate: VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;): u64 {
    <b>let</b> total_stake = <a href="validator.md#0x3_validator_total_stake">total_stake</a>(self);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BUSD&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BARS&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BAUD&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BBRL&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BCAD&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BEUR&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BGBP&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BIDR&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BINR&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BKRW&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BMXN&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BRUB&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BSAR&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BTRY&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BZAR&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;BJPY&gt;(self, stable_rate);
    total_stake = total_stake + <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;MGG&gt;(self, stable_rate);
    total_stake
}
</code></pre>



</details>

<a name="0x3_validator_total_stake_for_reward"></a>

## Function `total_stake_for_reward`



<pre><code><b>fun</b> <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>, stable_stake: &<b>mut</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;, stable_rate: &<a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_total_stake_for_reward">total_stake_for_reward</a>&lt;STABLE&gt;(
    self: &<a href="validator.md#0x3_validator_Validator">Validator</a>,
    stable_stake: &<b>mut</b> VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;,
    stable_rate: &VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;
):u64  {
    <b>let</b> total =  <a href="validator.md#0x3_validator_stable_stake_amount">stable_stake_amount</a>&lt;STABLE&gt;(self);
    <b>if</b> (total &gt; 0) {
        <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
        <b>let</b> rate = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get">vec_map::get</a>(stable_rate, &pool_key);
        <b>let</b> total_stake = (total <b>as</b> u128) * (*rate <b>as</b> u128) / (1000000000 <b>as</b> u128);
        <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(stable_stake, pool_key, (total_stake <b>as</b> u64));
        (total_stake <b>as</b> u64)
    }<b>else</b> {
     0
    }
}
</code></pre>



</details>

<a name="0x3_validator_total_stake_of_stable"></a>

## Function `total_stake_of_stable`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>, stable_rate: <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_total_stake_of_stable">total_stake_of_stable</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>, stable_rate: VecMap&lt;<a href="_String">ascii::String</a>, u64&gt;): u64 {
    <b>let</b> stable_stake =  <a href="validator.md#0x3_validator_stable_stake_amount">stable_stake_amount</a>&lt;STABLE&gt;(self);
    <b>if</b> (stable_stake &gt; 0) {
        <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;STABLE&gt;());
        <b>let</b> rate = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get">vec_map::get</a>(&stable_rate, &pool_key);
        <b>let</b> total_stake = (stable_stake <b>as</b> u128) *  (*rate <b>as</b> u128) / (1000000000 <b>as</b> u128);
        (total_stake <b>as</b> u64)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="0x3_validator_voting_power"></a>

## Function `voting_power`

Return the voting power of this validator.


<pre><code><b>public</b> <b>fun</b> <a href="voting_power.md#0x3_voting_power">voting_power</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="voting_power.md#0x3_voting_power">voting_power</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    self.<a href="voting_power.md#0x3_voting_power">voting_power</a>
}
</code></pre>



</details>

<a name="0x3_validator_set_voting_power"></a>

## Function `set_voting_power`

Set the voting power of this validator, called only from validator_set.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_set_voting_power">set_voting_power</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, new_voting_power: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_set_voting_power">set_voting_power</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, new_voting_power: u64) {
    self.<a href="voting_power.md#0x3_voting_power">voting_power</a> = new_voting_power;
}
</code></pre>



</details>

<a name="0x3_validator_pending_stake_amount"></a>

## Function `pending_stake_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pending_stake_amount">pending_stake_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pending_stake_amount">pending_stake_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    <a href="staking_pool.md#0x3_staking_pool_pending_stake_amount">staking_pool::pending_stake_amount</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<a name="0x3_validator_pending_stake_withdraw_amount"></a>

## Function `pending_stake_withdraw_amount`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pending_stake_withdraw_amount">pending_stake_withdraw_amount</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    <a href="staking_pool.md#0x3_staking_pool_pending_stake_withdraw_amount">staking_pool::pending_stake_withdraw_amount</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<a name="0x3_validator_gas_price"></a>

## Function `gas_price`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_gas_price">gas_price</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_gas_price">gas_price</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    self.gas_price
}
</code></pre>



</details>

<a name="0x3_validator_commission_rate"></a>

## Function `commission_rate`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_commission_rate">commission_rate</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_commission_rate">commission_rate</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): u64 {
    self.commission_rate
}
</code></pre>



</details>

<a name="0x3_validator_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>, epoch: u64): <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>, epoch: u64): PoolTokenExchangeRate {
    <a href="staking_pool.md#0x3_staking_pool_pool_token_exchange_rate_at_epoch">staking_pool::pool_token_exchange_rate_at_epoch</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>, epoch)
}
</code></pre>



</details>

<a name="0x3_validator_pool_stable_token_exchange_rate_at_epoch"></a>

## Function `pool_stable_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pool_stable_token_exchange_rate_at_epoch">pool_stable_token_exchange_rate_at_epoch</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>, epoch: u64): <a href="">vector</a>&lt;<a href="stable_pool.md#0x3_stable_pool_PoolStableTokenExchangeRate">stable_pool::PoolStableTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_pool_stable_token_exchange_rate_at_epoch">pool_stable_token_exchange_rate_at_epoch</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>, epoch: u64): <a href="">vector</a>&lt;PoolStableTokenExchangeRate&gt; {
    <b>let</b> vec_rate = <a href="_empty">vector::empty</a>&lt;PoolStableTokenExchangeRate&gt;();
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BUSD&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 0);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BARS&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 1);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BAUD&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 2);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BBRL&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 3);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BCAD&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 4);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BEUR&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 5);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BGBP&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 6);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BIDR&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 7);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BINR&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 8);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BJPY&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 9);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BKRW&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 10);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BMXN&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 11);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BRUB&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 12);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BSAR&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 13);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BTRY&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 14);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;BZAR&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 15);
    <a href="_insert">vector::insert</a>(&<b>mut</b> vec_rate, <a href="stable_pool.md#0x3_stable_pool_pool_token_exchange_rate_at_epoch">stable_pool::pool_token_exchange_rate_at_epoch</a>&lt;MGG&gt;(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>(&self.stable_pools), epoch), 16);
    vec_rate
}
</code></pre>



</details>

<a name="0x3_validator_staking_pool_id"></a>

## Function `staking_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_staking_pool_id">staking_pool_id</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_staking_pool_id">staking_pool_id</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): ID {
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(&self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>)
}
</code></pre>



</details>

<a name="0x3_validator_stable_pool_id"></a>

## Function `stable_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): ID {
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(<a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>&lt;STABLE&gt;(&self.stable_pools))
}
</code></pre>



</details>

<a name="0x3_validator_stable_pool"></a>

## Function `stable_pool`



<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool">stable_pool</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_pool.md#0x3_stable_pool">stable_pool</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>): &StablePool&lt;STABLE&gt; {
    <a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>&lt;STABLE&gt;(&self.stable_pools)
}
</code></pre>



</details>

<a name="0x3_validator_all_stable_pool_id"></a>

## Function `all_stable_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_all_stable_pool_id">all_stable_pool_id</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): <a href="">vector</a>&lt;<a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_all_stable_pool_id">all_stable_pool_id</a>(self:&<a href="validator.md#0x3_validator_Validator">Validator</a>): <a href="">vector</a>&lt;ID&gt; {
    <b>let</b> id_vec = <a href="">vector</a>[];
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BUSD&gt;(self), 0);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BARS&gt;(self), 1);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BAUD&gt;(self), 2);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BBRL&gt;(self), 3);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BCAD&gt;(self), 4);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BEUR&gt;(self), 5);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BGBP&gt;(self), 6);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BIDR&gt;(self), 7);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BINR&gt;(self), 8);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BJPY&gt;(self), 9);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BKRW&gt;(self), 10);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BMXN&gt;(self), 11);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BRUB&gt;(self), 12);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BSAR&gt;(self), 13);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BTRY&gt;(self), 14);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;BZAR&gt;(self), 15);
    <a href="_insert">vector::insert</a>(&<b>mut</b> id_vec ,<a href="validator.md#0x3_validator_stable_pool_id">stable_pool_id</a>&lt;MGG&gt;(self), 16);
    id_vec
}
</code></pre>



</details>

<a name="0x3_validator_is_duplicate"></a>

## Function `is_duplicate`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_is_duplicate">is_duplicate</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>, other: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_is_duplicate">is_duplicate</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>, other: &<a href="validator.md#0x3_validator_Validator">Validator</a>): bool {
     self.metadata.sui_address == other.metadata.sui_address
        || self.metadata.name == other.metadata.name
        || self.metadata.net_address == other.metadata.net_address
        || self.metadata.p2p_address == other.metadata.p2p_address
        || self.metadata.protocol_pubkey_bytes == other.metadata.protocol_pubkey_bytes
        || self.metadata.network_pubkey_bytes == other.metadata.network_pubkey_bytes
        || self.metadata.network_pubkey_bytes == other.metadata.worker_pubkey_bytes
        || self.metadata.worker_pubkey_bytes == other.metadata.worker_pubkey_bytes
        || self.metadata.worker_pubkey_bytes == other.metadata.network_pubkey_bytes
        // All next epoch parameters.
        || <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_net_address, &other.metadata.next_epoch_net_address)
        || <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_p2p_address, &other.metadata.next_epoch_p2p_address)
        || <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_protocol_pubkey_bytes, &other.metadata.next_epoch_protocol_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.next_epoch_network_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.next_epoch_worker_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_worker_pubkey_bytes, &other.metadata.next_epoch_worker_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>(&self.metadata.next_epoch_worker_pubkey_bytes, &other.metadata.next_epoch_network_pubkey_bytes)
        // My next epoch parameters <b>with</b> other current epoch parameters.
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_net_address, &other.metadata.net_address)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_p2p_address, &other.metadata.p2p_address)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_protocol_pubkey_bytes, &other.metadata.protocol_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.network_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_network_pubkey_bytes, &other.metadata.worker_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_worker_pubkey_bytes, &other.metadata.worker_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&self.metadata.next_epoch_worker_pubkey_bytes, &other.metadata.network_pubkey_bytes)
        // Other next epoch parameters <b>with</b> my current epoch parameters.
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_net_address, &self.metadata.net_address)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_p2p_address, &self.metadata.p2p_address)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_protocol_pubkey_bytes, &self.metadata.protocol_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_network_pubkey_bytes, &self.metadata.network_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_network_pubkey_bytes, &self.metadata.worker_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_worker_pubkey_bytes, &self.metadata.worker_pubkey_bytes)
        || <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>(&other.metadata.next_epoch_worker_pubkey_bytes, &self.metadata.network_pubkey_bytes)
}
</code></pre>



</details>

<a name="0x3_validator_is_equal_some_and_value"></a>

## Function `is_equal_some_and_value`



<pre><code><b>fun</b> <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>&lt;T&gt;(a: &<a href="_Option">option::Option</a>&lt;T&gt;, b: &T): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_is_equal_some_and_value">is_equal_some_and_value</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &T): bool {
    <b>if</b> (<a href="_is_none">option::is_none</a>(a)) {
        <b>false</b>
    } <b>else</b> {
        <a href="_borrow">option::borrow</a>(a) == b
    }
}
</code></pre>



</details>

<a name="0x3_validator_is_equal_some"></a>

## Function `is_equal_some`



<pre><code><b>fun</b> <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>&lt;T&gt;(a: &<a href="_Option">option::Option</a>&lt;T&gt;, b: &<a href="_Option">option::Option</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_is_equal_some">is_equal_some</a>&lt;T&gt;(a: &Option&lt;T&gt;, b: &Option&lt;T&gt;): bool {
    <b>if</b> (<a href="_is_none">option::is_none</a>(a) || <a href="_is_none">option::is_none</a>(b)) {
        <b>false</b>
    } <b>else</b> {
        <a href="_borrow">option::borrow</a>(a) == <a href="_borrow">option::borrow</a>(b)
    }
}
</code></pre>



</details>

<a name="0x3_validator_new_unverified_validator_operation_cap_and_transfer"></a>

## Function `new_unverified_validator_operation_cap_and_transfer`

Create a new <code>UnverifiedValidatorOperationCap</code>, transfer to the validator,
and registers it, thus revoking the previous cap's permission.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_new_unverified_validator_operation_cap_and_transfer">new_unverified_validator_operation_cap_and_transfer</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_new_unverified_validator_operation_cap_and_transfer">new_unverified_validator_operation_cap_and_transfer</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> <b>address</b> = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>assert</b>!(<b>address</b> == self.metadata.sui_address, <a href="validator.md#0x3_validator_ENewCapNotCreatedByValidatorItself">ENewCapNotCreatedByValidatorItself</a>);
    <b>let</b> new_id = <a href="validator_cap.md#0x3_validator_cap_new_unverified_validator_operation_cap_and_transfer">validator_cap::new_unverified_validator_operation_cap_and_transfer</a>(<b>address</b>, ctx);
    self.operation_cap_id = new_id;
}
</code></pre>



</details>

<a name="0x3_validator_update_name"></a>

## Function `update_name`

Update name of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_name">update_name</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, name: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_name">update_name</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, name: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&name) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    self.metadata.name = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(name));
}
</code></pre>



</details>

<a name="0x3_validator_update_description"></a>

## Function `update_description`

Update description of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_description">update_description</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, description: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_description">update_description</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, description: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&description) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    self.metadata.description = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(description));
}
</code></pre>



</details>

<a name="0x3_validator_update_image_url"></a>

## Function `update_image_url`

Update image url of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, image_url: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_image_url">update_image_url</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, image_url: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&image_url) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    self.metadata.image_url = <a href="../../../.././build/Sui/docs/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(image_url);
}
</code></pre>



</details>

<a name="0x3_validator_update_project_url"></a>

## Function `update_project_url`

Update project url of the validator.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, project_url: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_project_url">update_project_url</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, project_url: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&project_url) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    self.metadata.project_url = <a href="../../../.././build/Sui/docs/url.md#0x2_url_new_unsafe_from_bytes">url::new_unsafe_from_bytes</a>(project_url);
}
</code></pre>



</details>

<a name="0x3_validator_update_next_epoch_network_address"></a>

## Function `update_next_epoch_network_address`

Update network address of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_network_address">update_next_epoch_network_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, net_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_network_address">update_next_epoch_network_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, net_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&net_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> net_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(net_address));
    self.metadata.next_epoch_net_address = <a href="_some">option::some</a>(net_address);
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_candidate_network_address"></a>

## Function `update_candidate_network_address`

Update network address of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_network_address">update_candidate_network_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, net_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_network_address">update_candidate_network_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, net_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&net_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> net_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(net_address));
    self.metadata.net_address = net_address;
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_next_epoch_p2p_address"></a>

## Function `update_next_epoch_p2p_address`

Update p2p address of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, p2p_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_p2p_address">update_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, p2p_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&p2p_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> p2p_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(p2p_address));
    self.metadata.next_epoch_p2p_address = <a href="_some">option::some</a>(p2p_address);
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_candidate_p2p_address"></a>

## Function `update_candidate_p2p_address`

Update p2p address of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, p2p_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_p2p_address">update_candidate_p2p_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, p2p_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&p2p_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> p2p_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(p2p_address));
    self.metadata.p2p_address = p2p_address;
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_next_epoch_primary_address"></a>

## Function `update_next_epoch_primary_address`

Update primary address of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_primary_address">update_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, primary_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_primary_address">update_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, primary_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&primary_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> primary_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(primary_address));
    self.metadata.next_epoch_primary_address = <a href="_some">option::some</a>(primary_address);
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_candidate_primary_address"></a>

## Function `update_candidate_primary_address`

Update primary address of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_primary_address">update_candidate_primary_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, primary_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_primary_address">update_candidate_primary_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, primary_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&primary_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> primary_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(primary_address));
    self.metadata.primary_address = primary_address;
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_next_epoch_worker_address"></a>

## Function `update_next_epoch_worker_address`

Update worker address of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_worker_address">update_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, worker_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_worker_address">update_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, worker_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&worker_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> worker_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(worker_address));
    self.metadata.next_epoch_worker_address = <a href="_some">option::some</a>(worker_address);
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_candidate_worker_address"></a>

## Function `update_candidate_worker_address`

Update worker address of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_worker_address">update_candidate_worker_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, worker_address: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_worker_address">update_candidate_worker_address</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, worker_address: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    <b>assert</b>!(
        <a href="_length">vector::length</a>(&worker_address) &lt;= <a href="validator.md#0x3_validator_MAX_VALIDATOR_METADATA_LENGTH">MAX_VALIDATOR_METADATA_LENGTH</a>,
        <a href="validator.md#0x3_validator_EValidatorMetadataExceedingLengthLimit">EValidatorMetadataExceedingLengthLimit</a>
    );
    <b>let</b> worker_address = <a href="_from_ascii">string::from_ascii</a>(<a href="_string">ascii::string</a>(worker_address));
    self.metadata.worker_address = worker_address;
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_next_epoch_protocol_pubkey"></a>

## Function `update_next_epoch_protocol_pubkey`

Update protocol public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_protocol_pubkey">update_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, protocol_pubkey: <a href="">vector</a>&lt;u8&gt;, proof_of_possession: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_protocol_pubkey">update_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, protocol_pubkey: <a href="">vector</a>&lt;u8&gt;, proof_of_possession: <a href="">vector</a>&lt;u8&gt;) {
    self.metadata.next_epoch_protocol_pubkey_bytes = <a href="_some">option::some</a>(protocol_pubkey);
    self.metadata.next_epoch_proof_of_possession = <a href="_some">option::some</a>(proof_of_possession);
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_candidate_protocol_pubkey"></a>

## Function `update_candidate_protocol_pubkey`

Update protocol public key of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_protocol_pubkey">update_candidate_protocol_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, protocol_pubkey: <a href="">vector</a>&lt;u8&gt;, proof_of_possession: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_protocol_pubkey">update_candidate_protocol_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, protocol_pubkey: <a href="">vector</a>&lt;u8&gt;, proof_of_possession: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.metadata.protocol_pubkey_bytes = protocol_pubkey;
    self.metadata.proof_of_possession = proof_of_possession;
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_next_epoch_network_pubkey"></a>

## Function `update_next_epoch_network_pubkey`

Update network public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_network_pubkey">update_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, network_pubkey: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_network_pubkey">update_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, network_pubkey: <a href="">vector</a>&lt;u8&gt;) {
    self.metadata.next_epoch_network_pubkey_bytes = <a href="_some">option::some</a>(network_pubkey);
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_candidate_network_pubkey"></a>

## Function `update_candidate_network_pubkey`

Update network public key of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_network_pubkey">update_candidate_network_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, network_pubkey: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_network_pubkey">update_candidate_network_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, network_pubkey: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.metadata.network_pubkey_bytes = network_pubkey;
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_next_epoch_worker_pubkey"></a>

## Function `update_next_epoch_worker_pubkey`

Update Narwhal worker public key of this validator, taking effects from next epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_worker_pubkey">update_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, worker_pubkey: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_next_epoch_worker_pubkey">update_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, worker_pubkey: <a href="">vector</a>&lt;u8&gt;) {
    self.metadata.next_epoch_worker_pubkey_bytes = <a href="_some">option::some</a>(worker_pubkey);
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_update_candidate_worker_pubkey"></a>

## Function `update_candidate_worker_pubkey`

Update Narwhal worker public key of this candidate validator


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_worker_pubkey">update_candidate_worker_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>, worker_pubkey: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_update_candidate_worker_pubkey">update_candidate_worker_pubkey</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>, worker_pubkey: <a href="">vector</a>&lt;u8&gt;) {
    <b>assert</b>!(<a href="validator.md#0x3_validator_is_preactive">is_preactive</a>(self), <a href="validator.md#0x3_validator_ENotValidatorCandidate">ENotValidatorCandidate</a>);
    self.metadata.worker_pubkey_bytes = worker_pubkey;
    <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(&self.metadata);
}
</code></pre>



</details>

<a name="0x3_validator_effectuate_staged_metadata"></a>

## Function `effectuate_staged_metadata`

Effectutate all staged next epoch metadata for this validator.
NOTE: this function SHOULD ONLY be called by validator_set when
advancing an epoch.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">validator::Validator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_effectuate_staged_metadata">effectuate_staged_metadata</a>(self: &<b>mut</b> <a href="validator.md#0x3_validator_Validator">Validator</a>) {
    <b>if</b> (<a href="_is_some">option::is_some</a>(<a href="validator.md#0x3_validator_next_epoch_network_address">next_epoch_network_address</a>(self))) {
        self.metadata.net_address = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_net_address);
        self.metadata.next_epoch_net_address = <a href="_none">option::none</a>();
    };

    <b>if</b> (<a href="_is_some">option::is_some</a>(<a href="validator.md#0x3_validator_next_epoch_p2p_address">next_epoch_p2p_address</a>(self))) {
        self.metadata.p2p_address = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_p2p_address);
        self.metadata.next_epoch_p2p_address = <a href="_none">option::none</a>();
    };

    <b>if</b> (<a href="_is_some">option::is_some</a>(<a href="validator.md#0x3_validator_next_epoch_primary_address">next_epoch_primary_address</a>(self))) {
        self.metadata.primary_address = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_primary_address);
        self.metadata.next_epoch_primary_address = <a href="_none">option::none</a>();
    };

    <b>if</b> (<a href="_is_some">option::is_some</a>(<a href="validator.md#0x3_validator_next_epoch_worker_address">next_epoch_worker_address</a>(self))) {
        self.metadata.worker_address = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_worker_address);
        self.metadata.next_epoch_worker_address = <a href="_none">option::none</a>();
    };

    <b>if</b> (<a href="_is_some">option::is_some</a>(<a href="validator.md#0x3_validator_next_epoch_protocol_pubkey_bytes">next_epoch_protocol_pubkey_bytes</a>(self))) {
        self.metadata.protocol_pubkey_bytes = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_protocol_pubkey_bytes);
        self.metadata.next_epoch_protocol_pubkey_bytes = <a href="_none">option::none</a>();
        self.metadata.proof_of_possession = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_proof_of_possession);
        self.metadata.next_epoch_proof_of_possession = <a href="_none">option::none</a>();
    };

    <b>if</b> (<a href="_is_some">option::is_some</a>(<a href="validator.md#0x3_validator_next_epoch_network_pubkey_bytes">next_epoch_network_pubkey_bytes</a>(self))) {
        self.metadata.network_pubkey_bytes = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_network_pubkey_bytes);
        self.metadata.next_epoch_network_pubkey_bytes = <a href="_none">option::none</a>();
    };

    <b>if</b> (<a href="_is_some">option::is_some</a>(<a href="validator.md#0x3_validator_next_epoch_worker_pubkey_bytes">next_epoch_worker_pubkey_bytes</a>(self))) {
        self.metadata.worker_pubkey_bytes = <a href="_extract">option::extract</a>(&<b>mut</b> self.metadata.next_epoch_worker_pubkey_bytes);
        self.metadata.next_epoch_worker_pubkey_bytes = <a href="_none">option::none</a>();
    };
}
</code></pre>



</details>

<a name="0x3_validator_validate_metadata"></a>

## Function `validate_metadata`

Aborts if validator metadata is valid


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(metadata: &<a href="validator.md#0x3_validator_ValidatorMetadata">validator::ValidatorMetadata</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_validate_metadata">validate_metadata</a>(metadata: &<a href="validator.md#0x3_validator_ValidatorMetadata">ValidatorMetadata</a>) {
    <a href="validator.md#0x3_validator_validate_metadata_bcs">validate_metadata_bcs</a>(<a href="../../../.././build/Sui/docs/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(metadata));
}
</code></pre>



</details>

<a name="0x3_validator_validate_metadata_bcs"></a>

## Function `validate_metadata_bcs`



<pre><code><b>public</b> <b>fun</b> <a href="validator.md#0x3_validator_validate_metadata_bcs">validate_metadata_bcs</a>(metadata: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="validator.md#0x3_validator_validate_metadata_bcs">validate_metadata_bcs</a>(metadata: <a href="">vector</a>&lt;u8&gt;);
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>pragma</b> opaque;
<b>aborts_if</b> [abstract] <b>true</b>;
</code></pre>



</details>

<a name="0x3_validator_get_staking_pool_ref"></a>

## Function `get_staking_pool_ref`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="staking_pool.md#0x3_staking_pool_StakingPool">staking_pool::StakingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_get_staking_pool_ref">get_staking_pool_ref</a>(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>) : &StakingPool {
    &self.<a href="staking_pool.md#0x3_staking_pool">staking_pool</a>
}
</code></pre>



</details>

<a name="0x3_validator_get_stable_pool_ref"></a>

## Function `get_stable_pool_ref`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_get_stable_pool_ref">get_stable_pool_ref</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">validator::Validator</a>): &<a href="stable_pool.md#0x3_stable_pool_StablePool">stable_pool::StablePool</a>&lt;STABLE&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_get_stable_pool_ref">get_stable_pool_ref</a>&lt;STABLE&gt;(self: &<a href="validator.md#0x3_validator_Validator">Validator</a>) : &StablePool&lt;STABLE&gt; {
    <a href="validator.md#0x3_validator_get_stable_pool">get_stable_pool</a>&lt;STABLE&gt;(&self.stable_pools)
}
</code></pre>



</details>

<a name="0x3_validator_new_from_metadata"></a>

## Function `new_from_metadata`

Create a new validator from the given <code><a href="validator.md#0x3_validator_ValidatorMetadata">ValidatorMetadata</a></code>, called by both <code>new</code> and <code>new_for_testing</code>.


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_new_from_metadata">new_from_metadata</a>(metadata: <a href="validator.md#0x3_validator_ValidatorMetadata">validator::ValidatorMetadata</a>, gas_price: u64, commission_rate: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="validator.md#0x3_validator_Validator">validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="validator.md#0x3_validator_new_from_metadata">new_from_metadata</a>(
    metadata: <a href="validator.md#0x3_validator_ValidatorMetadata">ValidatorMetadata</a>,
    gas_price: u64,
    commission_rate: u64,
    ctx: &<b>mut</b> TxContext
): <a href="validator.md#0x3_validator_Validator">Validator</a> {
    <b>let</b> sui_address = metadata.sui_address;

    <b>let</b> <a href="staking_pool.md#0x3_staking_pool">staking_pool</a> = <a href="staking_pool.md#0x3_staking_pool_new">staking_pool::new</a>(ctx);
    <b>let</b> stable_pools = <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_new">bag::new</a>(ctx);

    <b>let</b> pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BUSD&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BUSD&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BUSD&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BARS&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BARS&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BARS&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BAUD&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BAUD&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BAUD&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BBRL&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BBRL&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BBRL&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BCAD&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BCAD&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BCAD&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BEUR&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BEUR&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BEUR&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BGBP&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BGBP&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BGBP&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BIDR&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BIDR&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BIDR&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BINR&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BINR&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BINR&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BJPY&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BJPY&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BJPY&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BKRW&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BKRW&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BKRW&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BMXN&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BMXN&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BMXN&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BRUB&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BRUB&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BRUB&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BSAR&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BSAR&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BSAR&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BTRY&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BTRY&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BTRY&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BZAR&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;BZAR&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;BZAR&gt;(ctx));
    pool_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;MGG&gt;());
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;<a href="_String">ascii::String</a>, StablePool&lt;MGG&gt;&gt;(&<b>mut</b> stable_pools, pool_key,<a href="stable_pool.md#0x3_stable_pool_new">stable_pool::new</a>&lt;MGG&gt;(ctx));


    <b>let</b> operation_cap_id = <a href="validator_cap.md#0x3_validator_cap_new_unverified_validator_operation_cap_and_transfer">validator_cap::new_unverified_validator_operation_cap_and_transfer</a>(sui_address, ctx);
    <a href="validator.md#0x3_validator_Validator">Validator</a> {
        metadata,
        // Initialize the voting power <b>to</b> be 0.
        // At the epoch change <b>where</b> this <a href="validator.md#0x3_validator">validator</a> is actually added <b>to</b> the
        // active <a href="validator.md#0x3_validator">validator</a> set, the voting power will be updated accordingly.
        <a href="voting_power.md#0x3_voting_power">voting_power</a>: 0,
        operation_cap_id,
        gas_price,
        <a href="staking_pool.md#0x3_staking_pool">staking_pool</a>,
        stable_pools,
        commission_rate,
        next_epoch_stake: 0,
        next_epoch_stable_stake: <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        next_epoch_gas_price: gas_price,
        next_epoch_commission_rate: commission_rate,
        extra_fields: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_new">bag::new</a>(ctx),
    }
}
</code></pre>



</details>

<a name="0x3_validator_rate_vec_map"></a>

## Function `rate_vec_map`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_rate_vec_map">rate_vec_map</a>(): <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="_String">ascii::String</a>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="validator.md#0x3_validator_rate_vec_map">rate_vec_map</a>() : VecMap&lt;<a href="_String">ascii::String</a>, u64&gt; {
    <b>let</b> rate_map = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>&lt;<a href="_String">ascii::String</a>, u64&gt;();
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BUSD&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BARS&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BAUD&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BBRL&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BCAD&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BEUR&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BGBP&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BIDR&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BINR&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BJPY&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BKRW&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BMXN&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BRUB&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BSAR&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BTRY&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BZAR&gt;()), 1000000000);
    <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> rate_map, <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;MGG&gt;()), 1000000000);
    rate_map
}
</code></pre>



</details>

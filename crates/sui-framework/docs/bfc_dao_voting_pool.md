
<a name="0xc8_voting_pool"></a>

# Module `0xc8::voting_pool`



-  [Resource `VotingPool`](#0xc8_voting_pool_VotingPool)
-  [Struct `PoolTokenExchangeRate`](#0xc8_voting_pool_PoolTokenExchangeRate)
-  [Resource `VotingBfc`](#0xc8_voting_pool_VotingBfc)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_voting_pool_new)
-  [Function `request_add_voting`](#0xc8_voting_pool_request_add_voting)
-  [Function `request_withdraw_voting`](#0xc8_voting_pool_request_withdraw_voting)
-  [Function `withdraw_from_principal`](#0xc8_voting_pool_withdraw_from_principal)
-  [Function `unwrap_voting_bfc`](#0xc8_voting_pool_unwrap_voting_bfc)
-  [Function `bfc_balance`](#0xc8_voting_pool_bfc_balance)
-  [Function `pool_id`](#0xc8_voting_pool_pool_id)
-  [Function `voting_bfc_amount`](#0xc8_voting_pool_voting_bfc_amount)
-  [Function `split`](#0xc8_voting_pool_split)
-  [Function `is_equal_staking_metadata`](#0xc8_voting_pool_is_equal_staking_metadata)
-  [Function `pool_token_exchange_rate_at_epoch`](#0xc8_voting_pool_pool_token_exchange_rate_at_epoch)
-  [Function `bfc_amount`](#0xc8_voting_pool_bfc_amount)
-  [Function `pool_token_amount`](#0xc8_voting_pool_pool_token_amount)
-  [Function `get_token_amount`](#0xc8_voting_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#0xc8_voting_pool_initial_exchange_rate)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0xc8_voting_pool_VotingPool"></a>

## Resource `VotingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>struct</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>bfc_balance: u64</code>
</dt>
<dd>
 The total number of Bfc tokens in this pool,
</dd>
<dt>
<code>pool_token_balance: u64</code>
</dt>
<dd>
 Total number of pool tokens issued by the pool.
</dd>
</dl>


</details>

<a name="0xc8_voting_pool_PoolTokenExchangeRate"></a>

## Struct `PoolTokenExchangeRate`

Struct representing the exchange rate of the voting pool token to BFC.


<pre><code><b>struct</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bfc_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>pool_token_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_voting_pool_VotingBfc"></a>

## Resource `VotingBfc`

A self-custodial object holding the Voting bfc tokens.


<pre><code><b>struct</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>
 ID of the staking pool we are staking with.
</dd>
<dt>
<code>principal: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>
 The voting BFC tokens.
</dd>
<dt>
<code>stake_end_time: u64</code>
</dt>
<dd>
 when voting stake ends.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_voting_pool_DEFAULT_VOTE_END_TIME"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_DEFAULT_VOTE_END_TIME">DEFAULT_VOTE_END_TIME</a>: u64 = 3600000;
</code></pre>



<a name="0xc8_voting_pool_EDelegationOfZeroBfc"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EDelegationOfZeroBfc">EDelegationOfZeroBfc</a>: u64 = 17;
</code></pre>



<a name="0xc8_voting_pool_EIncompatibleVotingBfc"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EIncompatibleVotingBfc">EIncompatibleVotingBfc</a>: u64 = 12;
</code></pre>



<a name="0xc8_voting_pool_EInsufficientBfcTokenBalance"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EInsufficientBfcTokenBalance">EInsufficientBfcTokenBalance</a>: u64 = 3;
</code></pre>



<a name="0xc8_voting_pool_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: u64 = 0;
</code></pre>



<a name="0xc8_voting_pool_ENotEndOfStakingTime"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_ENotEndOfStakingTime">ENotEndOfStakingTime</a>: u64 = 19;
</code></pre>



<a name="0xc8_voting_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: u64 = 9;
</code></pre>



<a name="0xc8_voting_pool_EVotingBfcBelowThreshold"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EVotingBfcBelowThreshold">EVotingBfcBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="0xc8_voting_pool_EWithdrawAmountCannotBeZero"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EWithdrawAmountCannotBeZero">EWithdrawAmountCannotBeZero</a>: u64 = 2;
</code></pre>



<a name="0xc8_voting_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="0xc8_voting_pool_MIN_STAKING_THRESHOLD"></a>

votingBfc objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="0xc8_voting_pool_new"></a>

## Function `new`

Create a new, empty voting pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_new">new</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_new">new</a>(ctx: &<b>mut</b> TxContext) : <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a> {
    <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        bfc_balance: 0,
        pool_token_balance: 0,
    }
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
<b>aborts_if</b> ctx.ids_created + 1 &gt; MAX_U64;
</code></pre>



</details>

<a name="0xc8_voting_pool_request_add_voting"></a>

## Function `request_add_voting`

Request to voting to a staking pool. The voting starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_request_add_voting">request_add_voting</a>(pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>, voting: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_request_add_voting">request_add_voting</a>(
    pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>,
    voting: Balance&lt;BFC&gt;,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext
) : <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a> {
    <b>let</b> bfc_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&voting);
    <b>assert</b>!(bfc_amount &gt;= <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EDelegationOfZeroBfc">EDelegationOfZeroBfc</a>);
    <b>let</b> votingbfc = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(pool),
        principal: voting,
        stake_end_time: <a href="../../../.././build/Sui/docs/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>) + <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_DEFAULT_VOTE_END_TIME">DEFAULT_VOTE_END_TIME</a>,
    };
    votingbfc
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
<b>aborts_if</b> voting.value &lt; <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>;
<b>aborts_if</b> ctx.ids_created + 1 &gt; MAX_U64;
</code></pre>



</details>

<a name="0xc8_voting_pool_request_withdraw_voting"></a>

## Function `request_withdraw_voting`

Request to withdraw the given voting plus rewards from a staking pool.
Both the principal and corresponding rewards in BFC are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_request_withdraw_voting">request_withdraw_voting</a>(pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>, voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_request_withdraw_voting">request_withdraw_voting</a>(
    pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>,
    voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
) : Balance&lt;BFC&gt; {
    <b>let</b> (_, principal_withdraw) =
        <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_withdraw_from_principal">withdraw_from_principal</a>(pool, voting_bfc, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
    principal_withdraw
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
<b>aborts_if</b> voting_bfc.pool_id != <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(pool);
</code></pre>



</details>

<a name="0xc8_voting_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal BFC stored in the votingdBfc object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of BFC.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>, voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>): (u64, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>,
    voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
) : (u64, Balance&lt;BFC&gt;) {

    // Check that the voting information matches the pool.
    <b>assert</b>!(voting_bfc.pool_id == <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(pool), <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EWrongPool">EWrongPool</a>);
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>) &gt; voting_bfc.stake_end_time, <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_ENotEndOfStakingTime">ENotEndOfStakingTime</a>);


    <b>let</b> exchange_rate_at_staking_epoch = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>();
    <b>let</b> principal_withdraw = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_unwrap_voting_bfc">unwrap_voting_bfc</a>(voting_bfc);
    <b>let</b> pool_token_withdraw_amount = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_get_token_amount">get_token_amount</a>(&exchange_rate_at_staking_epoch, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&principal_withdraw));

    (
        pool_token_withdraw_amount,
        principal_withdraw,
    )
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
<b>aborts_if</b> voting_bfc.pool_id != <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(pool);
</code></pre>



</details>

<a name="0xc8_voting_pool_unwrap_voting_bfc"></a>

## Function `unwrap_voting_bfc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_unwrap_voting_bfc">unwrap_voting_bfc</a>(voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_unwrap_voting_bfc">unwrap_voting_bfc</a>(voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>): Balance&lt;BFC&gt; {
    <b>let</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a> {
        id,
        pool_id: _,
        principal,
        stake_end_time: _,
    } = voting_bfc;
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_delete">object::delete</a>(id);
    principal
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_bfc_balance"></a>

## Function `bfc_balance`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_bfc_balance">bfc_balance</a>(pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_bfc_balance">bfc_balance</a>(pool: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>): u64 { pool.bfc_balance }
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_pool_id">pool_id</a>(voting_bfc: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_pool_id">pool_id</a>(voting_bfc: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>): ID { voting_bfc.pool_id }
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_voting_bfc_amount"></a>

## Function `voting_bfc_amount`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_voting_bfc_amount">voting_bfc_amount</a>(voting_bfc: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_voting_bfc_amount">voting_bfc_amount</a>(voting_bfc: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>): u64 { <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&voting_bfc.principal) }
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>aborts_if</b> split_amount &gt; votingBfc.principal.value;
<b>let</b> remaining_amount = votingBfc.principal.value - split_amount;
<b>aborts_if</b> remaining_amount &lt; <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>;
<b>aborts_if</b> split_amount &lt; <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>;
<b>aborts_if</b> ctx.ids_created + 1 &gt; MAX_U64;
</code></pre>




<pre><code><b>aborts_if</b> <b>false</b>;
<b>aborts_if</b> other.pool_id != self.pool_id;
<b>aborts_if</b> self.principal.value + other.principal.value &gt; MAX_U64;
</code></pre>



</details>

<a name="0xc8_voting_pool_split"></a>

## Function `split`

Split votingBfc <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the votingBfc like <code>voting</code> or <code>pool_id</code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_split">split</a>(self: &<b>mut</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_split">split</a>(self: &<b>mut</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a> {
    <b>let</b> original_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&self.principal);
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EInsufficientBfcTokenBalance">EInsufficientBfcTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EVotingBfcBelowThreshold">EVotingBfcBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_EVotingBfcBelowThreshold">EVotingBfcBelowThreshold</a>);
    <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: self.pool_id,
        principal: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> self.principal, split_amount),
        stake_end_time: self.stake_end_time,
    }
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
<b>let</b> original_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(self.principal);
<b>aborts_if</b> split_amount &gt; original_amount;
<b>let</b> remaining_amount = original_amount - split_amount;
<b>aborts_if</b> remaining_amount &lt; <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>;
<b>aborts_if</b> split_amount &lt; <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>;
<b>aborts_if</b> ctx.ids_created + 1 &gt; MAX_U64;
</code></pre>



</details>

<a name="0xc8_voting_pool_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the voting bfc except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, other: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>, other: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">VotingBfc</a>): bool {
    (self.pool_id == other.pool_id)
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_bfc_amount"></a>

## Function `bfc_amount`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_bfc_amount">bfc_amount</a>(exchange_rate: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_bfc_amount">bfc_amount</a>(exchange_rate: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.bfc_amount
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.pool_token_amount
}
</code></pre>



</details>

<a name="0xc8_voting_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>, bfc_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, bfc_amount: u64): u64 {
    // When either amount is 0, that means we have no voting <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.bfc_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> bfc_amount
    };
    <b>let</b> res = (exchange_rate.pool_token_amount <b>as</b> u128)
        * (bfc_amount <b>as</b> u128)
        / (exchange_rate.bfc_amount <b>as</b> u128);
    (res <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_voting_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { bfc_amount: 0, pool_token_amount: 0 }
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify;
</code></pre>

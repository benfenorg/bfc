
<a name="0xc8_voting_pool"></a>

# Module `0xc8::voting_pool`



-  [Resource `VotingPool`](#0xc8_voting_pool_VotingPool)
-  [Struct `PoolTokenExchangeRate`](#0xc8_voting_pool_PoolTokenExchangeRate)
-  [Resource `VotingObc`](#0xc8_voting_pool_VotingObc)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_voting_pool_new)
-  [Function `request_add_voting`](#0xc8_voting_pool_request_add_voting)
-  [Function `request_withdraw_voting`](#0xc8_voting_pool_request_withdraw_voting)
-  [Function `withdraw_from_principal`](#0xc8_voting_pool_withdraw_from_principal)
-  [Function `unwrap_voting_obc`](#0xc8_voting_pool_unwrap_voting_obc)
-  [Function `obc_balance`](#0xc8_voting_pool_obc_balance)
-  [Function `pool_id`](#0xc8_voting_pool_pool_id)
-  [Function `voting_obc_amount`](#0xc8_voting_pool_voting_obc_amount)
-  [Function `split`](#0xc8_voting_pool_split)
-  [Function `split_voting_obc`](#0xc8_voting_pool_split_voting_obc)
-  [Function `join_voting_obc`](#0xc8_voting_pool_join_voting_obc)
-  [Function `is_equal_staking_metadata`](#0xc8_voting_pool_is_equal_staking_metadata)
-  [Function `pool_token_exchange_rate_at_epoch`](#0xc8_voting_pool_pool_token_exchange_rate_at_epoch)
-  [Function `obc_amount`](#0xc8_voting_pool_obc_amount)
-  [Function `pool_token_amount`](#0xc8_voting_pool_pool_token_amount)
-  [Function `get_token_amount`](#0xc8_voting_pool_get_token_amount)
-  [Function `initial_exchange_rate`](#0xc8_voting_pool_initial_exchange_rate)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0xc8_voting_pool_VotingPool"></a>

## Resource `VotingPool`

A staking pool embedded in each validator struct in the system state object.


<pre><code><b>struct</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a> <b>has</b> store, key
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
<code>obc_balance: u64</code>
</dt>
<dd>
 The total number of OBC tokens in this pool,
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

Struct representing the exchange rate of the voting pool token to OBC.


<pre><code><b>struct</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>obc_amount: u64</code>
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

<a name="0xc8_voting_pool_VotingObc"></a>

## Resource `VotingObc`

A self-custodial object holding the Voting Obc tokens.


<pre><code><b>struct</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a> <b>has</b> store, key
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
<code>principal: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;</code>
</dt>
<dd>
 The voting OBC tokens.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_voting_pool_EDelegationOfZeroObc"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EDelegationOfZeroObc">EDelegationOfZeroObc</a>: u64 = 17;
</code></pre>



<a name="0xc8_voting_pool_EIncompatibleVotingObc"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EIncompatibleVotingObc">EIncompatibleVotingObc</a>: u64 = 12;
</code></pre>



<a name="0xc8_voting_pool_EInsufficientObcTokenBalance"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EInsufficientObcTokenBalance">EInsufficientObcTokenBalance</a>: u64 = 3;
</code></pre>



<a name="0xc8_voting_pool_EInsufficientPoolTokenBalance"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EInsufficientPoolTokenBalance">EInsufficientPoolTokenBalance</a>: u64 = 0;
</code></pre>



<a name="0xc8_voting_pool_ETokenBalancesDoNotMatchExchangeRate"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_ETokenBalancesDoNotMatchExchangeRate">ETokenBalancesDoNotMatchExchangeRate</a>: u64 = 9;
</code></pre>



<a name="0xc8_voting_pool_EVotingObcBelowThreshold"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EVotingObcBelowThreshold">EVotingObcBelowThreshold</a>: u64 = 18;
</code></pre>



<a name="0xc8_voting_pool_EWithdrawAmountCannotBeZero"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EWithdrawAmountCannotBeZero">EWithdrawAmountCannotBeZero</a>: u64 = 2;
</code></pre>



<a name="0xc8_voting_pool_EWrongPool"></a>



<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EWrongPool">EWrongPool</a>: u64 = 1;
</code></pre>



<a name="0xc8_voting_pool_MIN_STAKING_THRESHOLD"></a>

votingObc objects cannot be split to below this amount.


<pre><code><b>const</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="0xc8_voting_pool_new"></a>

## Function `new`

Create a new, empty voting pool.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_new">new</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_new">new</a>(ctx: &<b>mut</b> TxContext) : <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a> {
    <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        obc_balance: 0,
        pool_token_balance: 0,
    }
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_request_add_voting"></a>

## Function `request_add_voting`

Request to voting to a staking pool. The voting starts counting at the beginning of the next epoch,


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_request_add_voting">request_add_voting</a>(pool: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>, voting: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_request_add_voting">request_add_voting</a>(
    pool: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>,
    voting: Balance&lt;OBC&gt;,
    ctx: &<b>mut</b> TxContext
) : <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a> {
    <b>let</b> obc_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&voting);
    <b>assert</b>!(obc_amount &gt; 0, <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EDelegationOfZeroObc">EDelegationOfZeroObc</a>);
    <b>let</b> votingobc = <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(pool),
        principal: voting,
    };
    votingobc
}
</code></pre>



</details>

<a name="0xc8_voting_pool_request_withdraw_voting"></a>

## Function `request_withdraw_voting`

Request to withdraw the given voting plus rewards from a staking pool.
Both the principal and corresponding rewards in OBC are withdrawn.
A proportional amount of pool token withdraw is recorded and processed at epoch change time.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_request_withdraw_voting">request_withdraw_voting</a>(pool: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>, voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_request_withdraw_voting">request_withdraw_voting</a>(
    pool: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>,
    voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>,
) : Balance&lt;OBC&gt; {
    <b>let</b> (_, principal_withdraw) =
        <a href="obc_dao_voting_pool.md#0xc8_voting_pool_withdraw_from_principal">withdraw_from_principal</a>(pool, voting_obc);
    <b>let</b> principal_withdraw_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&principal_withdraw);


    <b>let</b> _ = principal_withdraw_amount;

    // TODO: implement withdraw bonding period here.
    principal_withdraw
}
</code></pre>



</details>

<a name="0xc8_voting_pool_withdraw_from_principal"></a>

## Function `withdraw_from_principal`

Withdraw the principal OBC stored in the votingdObc object, and calculate the corresponding amount of pool
tokens using exchange rate at staking epoch.
Returns values are amount of pool tokens withdrawn and withdrawn principal portion of OBC.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_withdraw_from_principal">withdraw_from_principal</a>(pool: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>, voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>): (u64, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_withdraw_from_principal">withdraw_from_principal</a>(
    pool: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>,
    voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>,
) : (u64, Balance&lt;OBC&gt;) {

    // Check that the voting information matches the pool.
    <b>assert</b>!(voting_obc.pool_id == <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(pool), <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EWrongPool">EWrongPool</a>);

    <b>let</b> exchange_rate_at_staking_epoch = <a href="obc_dao_voting_pool.md#0xc8_voting_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>();
    <b>let</b> principal_withdraw = <a href="obc_dao_voting_pool.md#0xc8_voting_pool_unwrap_voting_obc">unwrap_voting_obc</a>(voting_obc);
    <b>let</b> pool_token_withdraw_amount = <a href="obc_dao_voting_pool.md#0xc8_voting_pool_get_token_amount">get_token_amount</a>(&exchange_rate_at_staking_epoch, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&principal_withdraw));

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
</code></pre>



</details>

<a name="0xc8_voting_pool_unwrap_voting_obc"></a>

## Function `unwrap_voting_obc`



<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_unwrap_voting_obc">unwrap_voting_obc</a>(voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_unwrap_voting_obc">unwrap_voting_obc</a>(voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>): Balance&lt;OBC&gt; {
    <b>let</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a> {
        id,
        pool_id: _,
        principal,
    } = voting_obc;
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_delete">object::delete</a>(id);
    principal
}
</code></pre>



</details>

<a name="0xc8_voting_pool_obc_balance"></a>

## Function `obc_balance`



<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_obc_balance">obc_balance</a>(pool: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_obc_balance">obc_balance</a>(pool: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">VotingPool</a>): u64 { pool.obc_balance }
</code></pre>



</details>

<a name="0xc8_voting_pool_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_pool_id">pool_id</a>(voting_obc: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_pool_id">pool_id</a>(voting_obc: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>): ID { voting_obc.pool_id }
</code></pre>



</details>

<a name="0xc8_voting_pool_voting_obc_amount"></a>

## Function `voting_obc_amount`



<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_voting_obc_amount">voting_obc_amount</a>(voting_obc: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_voting_obc_amount">voting_obc_amount</a>(voting_obc: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>): u64 { <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&voting_obc.principal) }
</code></pre>



</details>

<a name="0xc8_voting_pool_split"></a>

## Function `split`

Split votingObc <code>self</code> to two parts, one with principal <code>split_amount</code>,
and the remaining principal is left in <code>self</code>.
All the other parameters of the votingObc like <code>voting</code> or <code>pool_id</code> remain the same.


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_split">split</a>(self: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_split">split</a>(self: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>, split_amount: u64, ctx: &<b>mut</b> TxContext): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a> {
    <b>let</b> original_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&self.principal);
    <b>assert</b>!(split_amount &lt;= original_amount, <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EInsufficientObcTokenBalance">EInsufficientObcTokenBalance</a>);
    <b>let</b> remaining_amount = original_amount - split_amount;
    // Both resulting parts should have at least <a href="obc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>.
    <b>assert</b>!(remaining_amount &gt;= <a href="obc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EVotingObcBelowThreshold">EVotingObcBelowThreshold</a>);
    <b>assert</b>!(split_amount &gt;= <a href="obc_dao_voting_pool.md#0xc8_voting_pool_MIN_STAKING_THRESHOLD">MIN_STAKING_THRESHOLD</a>, <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EVotingObcBelowThreshold">EVotingObcBelowThreshold</a>);
    <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        pool_id: self.pool_id,
        principal: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> self.principal, split_amount),
    }
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
</code></pre>



</details>

<a name="0xc8_voting_pool_split_voting_obc"></a>

## Function `split_voting_obc`

Split the given votingObc to the two parts, one with principal <code>split_amount</code>,
transfer the newly split part to the sender address.


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_split_voting_obc">split_voting_obc</a>(votingObc: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, split_amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_split_voting_obc">split_voting_obc</a>(votingObc: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="obc_dao_voting_pool.md#0xc8_voting_pool_split">split</a>(votingObc, split_amount, ctx), <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0xc8_voting_pool_join_voting_obc"></a>

## Function `join_voting_obc`

Consume the voting obc <code>other</code> and add its value to <code>self</code>.
Aborts if some of the staking parameters are incompatible (pool id,  activation epoch, etc.)


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_join_voting_obc">join_voting_obc</a>(self: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, other: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_join_voting_obc">join_voting_obc</a>(self: &<b>mut</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>, other: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>) {
    <b>assert</b>!(<a href="obc_dao_voting_pool.md#0xc8_voting_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self, &other), <a href="obc_dao_voting_pool.md#0xc8_voting_pool_EIncompatibleVotingObc">EIncompatibleVotingObc</a>);
    <b>let</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a> {
        id,
        pool_id: _,
        principal,
    } = other;

    <a href="../../../.././build/Sui/docs/object.md#0x2_object_delete">object::delete</a>(id);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> self.principal, principal);
}
</code></pre>



</details>

<a name="0xc8_voting_pool_is_equal_staking_metadata"></a>

## Function `is_equal_staking_metadata`

Returns true if all the staking parameters of the voting obc except the principal are identical


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, other: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_is_equal_staking_metadata">is_equal_staking_metadata</a>(self: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>, other: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">VotingObc</a>): bool {
    (self.pool_id == other.pool_id)
}
</code></pre>



</details>

<a name="0xc8_voting_pool_pool_token_exchange_rate_at_epoch"></a>

## Function `pool_token_exchange_rate_at_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_pool_token_exchange_rate_at_epoch">pool_token_exchange_rate_at_epoch</a>(): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="obc_dao_voting_pool.md#0xc8_voting_pool_initial_exchange_rate">initial_exchange_rate</a>()
}
</code></pre>



</details>

<a name="0xc8_voting_pool_obc_amount"></a>

## Function `obc_amount`



<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_obc_amount">obc_amount</a>(exchange_rate: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_obc_amount">obc_amount</a>(exchange_rate: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.obc_amount
}
</code></pre>



</details>

<a name="0xc8_voting_pool_pool_token_amount"></a>

## Function `pool_token_amount`



<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_pool_token_amount">pool_token_amount</a>(exchange_rate: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>): u64 {
    exchange_rate.pool_token_amount
}
</code></pre>



</details>

<a name="0xc8_voting_pool_get_token_amount"></a>

## Function `get_token_amount`



<pre><code><b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>, obc_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_get_token_amount">get_token_amount</a>(exchange_rate: &<a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a>, obc_amount: u64): u64 {
    // When either amount is 0, that means we have no voting <b>with</b> this pool.
    // The other amount might be non-zero when there's dust left in the pool.
    <b>if</b> (exchange_rate.obc_amount == 0 || exchange_rate.pool_token_amount == 0) {
        <b>return</b> obc_amount
    };
    <b>let</b> res = (exchange_rate.pool_token_amount <b>as</b> u128)
        * (obc_amount <b>as</b> u128)
        / (exchange_rate.obc_amount <b>as</b> u128);
    (res <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_voting_pool_initial_exchange_rate"></a>

## Function `initial_exchange_rate`



<pre><code><b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">voting_pool::PoolTokenExchangeRate</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool_initial_exchange_rate">initial_exchange_rate</a>(): <a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> {
    <a href="obc_dao_voting_pool.md#0xc8_voting_pool_PoolTokenExchangeRate">PoolTokenExchangeRate</a> { obc_amount: 0, pool_token_amount: 0 }
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify;
<b>pragma</b> aborts_if_is_strict;
</code></pre>

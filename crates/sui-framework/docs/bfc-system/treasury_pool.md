---
title: Module `0xc8::treasury_pool`
---



-  [Resource `TreasuryPool`](#0xc8_treasury_pool_TreasuryPool)
-  [Struct `WithdrawEvent`](#0xc8_treasury_pool_WithdrawEvent)
-  [Struct `DepositEvent`](#0xc8_treasury_pool_DepositEvent)
-  [Constants](#@Constants_0)
-  [Function `create_treasury_pool`](#0xc8_treasury_pool_create_treasury_pool)
-  [Function `deposit_to_treasury_pool`](#0xc8_treasury_pool_deposit_to_treasury_pool)
-  [Function `withdraw_to_treasury`](#0xc8_treasury_pool_withdraw_to_treasury)
-  [Function `get_balance`](#0xc8_treasury_pool_get_balance)


<pre><code><b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/math.md#0x2_math">0x2::math</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="event.md#0xc8_event">0xc8::event</a>;
</code></pre>



<a name="0xc8_treasury_pool_TreasuryPool"></a>

## Resource `TreasuryPool`



<pre><code><b>struct</b> <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">TreasuryPool</a> <b>has</b> store, key
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
<code><a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_treasury_pool_WithdrawEvent"></a>

## Struct `WithdrawEvent`



<pre><code><b>struct</b> <a href="treasury_pool.md#0xc8_treasury_pool_WithdrawEvent">WithdrawEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../sui-framework/balance.md#0x2_balance">balance</a>: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>request_amount: u64</code>
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

<a name="0xc8_treasury_pool_DepositEvent"></a>

## Struct `DepositEvent`



<pre><code><b>struct</b> <a href="treasury_pool.md#0xc8_treasury_pool_DepositEvent">DepositEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../sui-framework/balance.md#0x2_balance">balance</a>: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>deposit_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_treasury_pool_ERR_NOT_ZERO_ADDRESS"></a>

The <code>withdraw</code> function only called by 0x0 address.


<pre><code><b>const</b> <a href="treasury_pool.md#0xc8_treasury_pool_ERR_NOT_ZERO_ADDRESS">ERR_NOT_ZERO_ADDRESS</a>: u64 = 900;
</code></pre>



<a name="0xc8_treasury_pool_create_treasury_pool"></a>

## Function `create_treasury_pool`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_create_treasury_pool">create_treasury_pool</a>(<a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">treasury_pool::TreasuryPool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_create_treasury_pool">create_treasury_pool</a>(
    <a href="../sui-framework/balance.md#0x2_balance">balance</a>: Balance&lt;BFC&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">TreasuryPool</a>
{
    <b>let</b> <a href="treasury_pool.md#0xc8_treasury_pool">treasury_pool</a> = <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">TreasuryPool</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        <a href="../sui-framework/balance.md#0x2_balance">balance</a>: <a href="../sui-framework/balance.md#0x2_balance">balance</a>,
    };
    <b>let</b> treasury_pool_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&<a href="treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>);
    event::init_treasury_pool(treasury_pool_id);
    <a href="treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>
}
</code></pre>



</details>

<a name="0xc8_treasury_pool_deposit_to_treasury_pool"></a>

## Function `deposit_to_treasury_pool`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(self: &<b>mut</b> <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">treasury_pool::TreasuryPool</a>, bfc_coin: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_deposit_to_treasury_pool">deposit_to_treasury_pool</a>(
    self: &<b>mut</b> <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">TreasuryPool</a>,
    bfc_coin: Coin&lt;BFC&gt;
)
{
    <b>let</b> origin_amount = <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&self.<a href="../sui-framework/balance.md#0x2_balance">balance</a>);
    <b>let</b> deposit_amount = <a href="../sui-framework/coin.md#0x2_coin_value">coin::value</a>(&bfc_coin);
    <a href="../sui-framework/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> self.<a href="../sui-framework/balance.md#0x2_balance">balance</a>, <a href="../sui-framework/coin.md#0x2_coin_into_balance">coin::into_balance</a>(bfc_coin));
    emit(<a href="treasury_pool.md#0xc8_treasury_pool_DepositEvent">DepositEvent</a> {
        <a href="../sui-framework/balance.md#0x2_balance">balance</a>: origin_amount,
        deposit_amount
    });
}
</code></pre>



</details>

<a name="0xc8_treasury_pool_withdraw_to_treasury"></a>

## Function `withdraw_to_treasury`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_withdraw_to_treasury">withdraw_to_treasury</a>(self: &<b>mut</b> <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">treasury_pool::TreasuryPool</a>, amount: u64, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_withdraw_to_treasury">withdraw_to_treasury</a>(
    self: &<b>mut</b> <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">TreasuryPool</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
): Balance&lt;BFC&gt;
{
    <b>assert</b>!(<a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="treasury_pool.md#0xc8_treasury_pool_ERR_NOT_ZERO_ADDRESS">ERR_NOT_ZERO_ADDRESS</a>);
    // Take the minimum of the amount and the remaining <a href="../sui-framework/balance.md#0x2_balance">balance</a> in
    // order <b>to</b> ensure we don't overdraft the remaining <a href="../sui-framework/balance.md#0x2_balance">balance</a>
    <b>let</b> current_balance = <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&self.<a href="../sui-framework/balance.md#0x2_balance">balance</a>);
    <b>let</b> to_withdraw = <a href="../sui-framework/math.md#0x2_math_min">math::min</a>(amount, current_balance);

    <b>let</b> withdraw_balance = <a href="../sui-framework/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> self.<a href="../sui-framework/balance.md#0x2_balance">balance</a>, to_withdraw);
    emit(<a href="treasury_pool.md#0xc8_treasury_pool_WithdrawEvent">WithdrawEvent</a> {
        <a href="../sui-framework/balance.md#0x2_balance">balance</a>: current_balance,
        request_amount: amount,
        amount: to_withdraw,
    });
    withdraw_balance
}
</code></pre>



</details>

<a name="0xc8_treasury_pool_get_balance"></a>

## Function `get_balance`



<pre><code><b>public</b> <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_get_balance">get_balance</a>(self: &<a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">treasury_pool::TreasuryPool</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury_pool.md#0xc8_treasury_pool_get_balance">get_balance</a>(self: &<a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">TreasuryPool</a>): u64 {
    <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&self.<a href="../sui-framework/balance.md#0x2_balance">balance</a>)
}
</code></pre>



</details>

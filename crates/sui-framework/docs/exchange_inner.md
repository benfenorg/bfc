
<a name="0xc8_exchange_inner"></a>

# Module `0xc8::exchange_inner`



-  [Resource `ExchangePool`](#0xc8_exchange_inner_ExchangePool)
-  [Constants](#@Constants_0)
-  [Function `new_exchange_pool`](#0xc8_exchange_inner_new_exchange_pool)
-  [Function `pool_id`](#0xc8_exchange_inner_pool_id)
-  [Function `add_obc_to_pool`](#0xc8_exchange_inner_add_obc_to_pool)
-  [Function `is_active`](#0xc8_exchange_inner_is_active)
-  [Function `dis_activate`](#0xc8_exchange_inner_dis_activate)
-  [Function `activate`](#0xc8_exchange_inner_activate)
-  [Function `get_obc_amount`](#0xc8_exchange_inner_get_obc_amount)
-  [Function `get_stable_amount`](#0xc8_exchange_inner_get_stable_amount)
-  [Function `exchange_obc_amount`](#0xc8_exchange_inner_exchange_obc_amount)
-  [Function `request_exchange_stable`](#0xc8_exchange_inner_request_exchange_stable)
-  [Function `get_obc_for_exchange_all`](#0xc8_exchange_inner_get_obc_for_exchange_all)
-  [Function `request_exchange_all`](#0xc8_exchange_inner_request_exchange_all)
-  [Function `request_withdraw_all_stable`](#0xc8_exchange_inner_request_withdraw_all_stable)
-  [Function `request_deposit_obc_balance`](#0xc8_exchange_inner_request_deposit_obc_balance)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0xc8_exchange_inner_ExchangePool"></a>

## Resource `ExchangePool`



<pre><code><b>struct</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt; <b>has</b> store, key
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
<code>activation_epoch: <a href="_Option">option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 The epoch at which this pool became active.
 The value is <code>None</code> if the pool is pre-active and <code>Some(&lt;epoch_number&gt;)</code> if active or inactive.
</dd>
<dt>
<code>obc_balance: u64</code>
</dt>
<dd>
 The total number of SUI coins in this pool
</dd>
<dt>
<code>obc_pool: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>
 The epoch stake rewards will be added here at the end of each epoch.
</dd>
<dt>
<code>stable_token_balance: u64</code>
</dt>
<dd>
 Total number of pool stable coins issued by the pool.
</dd>
<dt>
<code>stable_pool: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE_COIN&gt;</code>
</dt>
<dd>
 The epoch stable gas coins
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_exchange_inner_ELackOfOBC"></a>



<pre><code><b>const</b> <a href="exchange_inner.md#0xc8_exchange_inner_ELackOfOBC">ELackOfOBC</a>: u64 = 4;
</code></pre>



<a name="0xc8_exchange_inner_ENotActivePool"></a>



<pre><code><b>const</b> <a href="exchange_inner.md#0xc8_exchange_inner_ENotActivePool">ENotActivePool</a>: u64 = 1;
</code></pre>



<a name="0xc8_exchange_inner_ENotAllowDeposit"></a>



<pre><code><b>const</b> <a href="exchange_inner.md#0xc8_exchange_inner_ENotAllowDeposit">ENotAllowDeposit</a>: u64 = 6;
</code></pre>



<a name="0xc8_exchange_inner_ENotAllowWithdraw"></a>



<pre><code><b>const</b> <a href="exchange_inner.md#0xc8_exchange_inner_ENotAllowWithdraw">ENotAllowWithdraw</a>: u64 = 5;
</code></pre>



<a name="0xc8_exchange_inner_EOBCZeroAmount"></a>



<pre><code><b>const</b> <a href="exchange_inner.md#0xc8_exchange_inner_EOBCZeroAmount">EOBCZeroAmount</a>: u64 = 3;
</code></pre>



<a name="0xc8_exchange_inner_EZeroAmount"></a>



<pre><code><b>const</b> <a href="exchange_inner.md#0xc8_exchange_inner_EZeroAmount">EZeroAmount</a>: u64 = 2;
</code></pre>



<a name="0xc8_exchange_inner_new_exchange_pool"></a>

## Function `new_exchange_pool`

Init exchange pool for gas coin exchange.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_new_exchange_pool">new_exchange_pool</a>&lt;STABLE_COIN&gt;(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>, epoch: u64): <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_new_exchange_pool">new_exchange_pool</a>&lt;STABLE_COIN&gt;(ctx: &<b>mut</b> TxContext, epoch: u64) : <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt; {
    <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        activation_epoch: <a href="_some">option::some</a>(epoch),
        obc_balance: 0,
        obc_pool: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>(),
        stable_token_balance: 0,
        stable_pool: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;STABLE_COIN&gt;(),
    }
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_pool_id"></a>

## Function `pool_id`

Get pool id.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_pool_id">pool_id</a>&lt;STABLE_COIN&gt;(pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): &<a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_pool_id">pool_id</a>&lt;STABLE_COIN&gt;(
    pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
): &UID {
    &pool.id
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_add_obc_to_pool"></a>

## Function `add_obc_to_pool`

Add obc to pool for gas exchange.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_add_obc_to_pool">add_obc_to_pool</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_add_obc_to_pool">add_obc_to_pool</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;BFC&gt;) {
    <b>let</b> amount = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>);
    <b>assert</b>!( amount &gt; 0, <a href="exchange_inner.md#0xc8_exchange_inner_EZeroAmount">EZeroAmount</a>);
    pool.obc_balance = pool.obc_balance + amount;
    <b>let</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a> = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> pool.obc_pool, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>);
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_is_active"></a>

## Function `is_active`

Returns true if the input exchange pool is active.


<pre><code><b>public</b> <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>&lt;STABLE_COIN&gt;(pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>&lt;STABLE_COIN&gt;(pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;): bool {
    <a href="_is_some">option::is_some</a>(&pool.activation_epoch)
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_dis_activate"></a>

## Function `dis_activate`

Disable activation of pool and return current epoch


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_dis_activate">dis_activate</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_dis_activate">dis_activate</a>&lt;STABLE_COIN&gt;(
    pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;
): u64 {
    <a href="_destroy_some">option::destroy_some</a>(pool.activation_epoch)
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_activate"></a>

## Function `activate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_activate">activate</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, epoch: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_activate">activate</a>&lt;STABLE_COIN&gt;(
    pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
    epoch: u64) {
    pool.activation_epoch = <a href="_some">option::some</a>(epoch);
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_get_obc_amount"></a>

## Function `get_obc_amount`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_obc_amount">get_obc_amount</a>&lt;STABLE_COIN&gt;(pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_obc_amount">get_obc_amount</a>&lt;STABLE_COIN&gt;(pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;): u64 {
    pool.obc_balance
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_get_stable_amount"></a>

## Function `get_stable_amount`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_stable_amount">get_stable_amount</a>&lt;STABLE_COIN&gt;(pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_stable_amount">get_stable_amount</a>&lt;STABLE_COIN&gt;(pool: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;): u64 {
    pool.stable_token_balance
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_exchange_obc_amount"></a>

## Function `exchange_obc_amount`

Get obc amount by exchange rate.


<pre><code><b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_exchange_obc_amount">exchange_obc_amount</a>(exchange_rate: u64, token_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_exchange_obc_amount">exchange_obc_amount</a>(exchange_rate: u64, token_amount: u64): u64 {
    <b>let</b> res = (token_amount <b>as</b> u128) / (exchange_rate <b>as</b> u128);
    (res <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_request_exchange_stable"></a>

## Function `request_exchange_stable`

Request for exchange gas coin to default coin.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_stable">request_exchange_stable</a>&lt;STABLE_COIN&gt;(exchange_rate: u64, pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, stable_coin: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;STABLE_COIN&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_stable">request_exchange_stable</a>&lt;STABLE_COIN&gt;(
    exchange_rate: u64,
    pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
    stable_coin: Coin&lt;STABLE_COIN&gt;,
    ctx: &<b>mut</b> TxContext
): Balance&lt;BFC&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&stable_coin) &gt; 0, <a href="exchange_inner.md#0xc8_exchange_inner_EZeroAmount">EZeroAmount</a>);
    <b>let</b> tok_balance = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(stable_coin);
    <b>let</b> stable_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&tok_balance);
    <b>let</b> obc_amount= <a href="exchange_inner.md#0xc8_exchange_inner_exchange_obc_amount">exchange_obc_amount</a>(exchange_rate, stable_amount);
    <b>assert</b>!(obc_amount &gt; 0, <a href="exchange_inner.md#0xc8_exchange_inner_EOBCZeroAmount">EOBCZeroAmount</a>);
    <b>assert</b>!(pool.obc_balance &gt; obc_amount, <a href="exchange_inner.md#0xc8_exchange_inner_ELackOfOBC">ELackOfOBC</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> pool.stable_pool, tok_balance);
    <b>let</b> result = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_take">coin::take</a>(&<b>mut</b> pool.obc_pool, obc_amount, ctx);
    pool.obc_balance = pool.obc_balance - obc_amount;
    pool.stable_token_balance = pool.stable_token_balance + stable_amount;
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(result)
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_get_obc_for_exchange_all"></a>

## Function `get_obc_for_exchange_all`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_obc_for_exchange_all">get_obc_for_exchange_all</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_obc_for_exchange_all">get_obc_for_exchange_all</a>&lt;STABLE_COIN&gt;(
    pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
): Balance&lt;BFC&gt; {
    <b>if</b>(pool.obc_balance &gt; 0) {
        //set pool active is <b>false</b>
        pool.obc_balance = 0;
       <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_withdraw_all">balance::withdraw_all</a>(&<b>mut</b> pool.obc_pool)
    }<b>else</b> {
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;()
    }
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_request_exchange_all"></a>

## Function `request_exchange_all`

Exchange all stable gas coins to default coins


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_all">request_exchange_all</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE_COIN&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_all">request_exchange_all</a>&lt;STABLE_COIN&gt;(
    pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: Balance&lt;STABLE_COIN&gt;,
) {
    <b>assert</b>!(<a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>(pool), <a href="exchange_inner.md#0xc8_exchange_inner_ENotActivePool">ENotActivePool</a>);
    pool.stable_token_balance = pool.stable_token_balance + <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> pool.stable_pool, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>);
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_request_withdraw_all_stable"></a>

## Function `request_withdraw_all_stable`

Withdraw the stable gas coins.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_all_stable">request_withdraw_all_stable</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE_COIN&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_all_stable">request_withdraw_all_stable</a>&lt;STABLE_COIN&gt;(
    pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
): Balance&lt;STABLE_COIN&gt; {
    <b>assert</b>!(!<a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>(pool), <a href="exchange_inner.md#0xc8_exchange_inner_ENotAllowWithdraw">ENotAllowWithdraw</a>);
    pool.stable_token_balance = 0;
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_withdraw_all">balance::withdraw_all</a>&lt;STABLE_COIN&gt;(&<b>mut</b> pool.stable_pool)
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_request_deposit_obc_balance"></a>

## Function `request_deposit_obc_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_deposit_obc_balance">request_deposit_obc_balance</a>&lt;STABLE_COIN&gt;(pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, obc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_deposit_obc_balance">request_deposit_obc_balance</a>&lt;STABLE_COIN&gt;(
    pool: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
    obc_balance: Balance&lt;BFC&gt;,
) {
    <b>assert</b>!(!<a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>(pool), <a href="exchange_inner.md#0xc8_exchange_inner_ENotAllowDeposit">ENotAllowDeposit</a>);
    pool.obc_balance = pool.obc_balance + <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&obc_balance);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> pool.obc_pool, obc_balance);
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

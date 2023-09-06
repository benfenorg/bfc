
<a name="0xc8_exchange_inner"></a>

# Module `0xc8::exchange_inner`



-  [Resource `ExchangePool`](#0xc8_exchange_inner_ExchangePool)
-  [Constants](#@Constants_0)
-  [Function `new_exchange_pool`](#0xc8_exchange_inner_new_exchange_pool)
-  [Function `pool_id`](#0xc8_exchange_inner_pool_id)
-  [Function `add_obc_to_pool`](#0xc8_exchange_inner_add_obc_to_pool)
-  [Function `is_active`](#0xc8_exchange_inner_is_active)
-  [Function `get_obc_amount`](#0xc8_exchange_inner_get_obc_amount)
-  [Function `get_stable_amount`](#0xc8_exchange_inner_get_stable_amount)
-  [Function `exchange_obc_amount`](#0xc8_exchange_inner_exchange_obc_amount)
-  [Function `request_exchange_stable`](#0xc8_exchange_inner_request_exchange_stable)
-  [Function `request_exchange_all`](#0xc8_exchange_inner_request_exchange_all)
-  [Function `request_withdraw_stable`](#0xc8_exchange_inner_request_withdraw_stable)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
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
<code>obc_pool: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;obc::OBC&gt;</code>
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


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_pool_id">pool_id</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): &<a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_pool_id">pool_id</a>&lt;STABLE_COIN&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
): &UID {
    &<a href="pool.md#0xc8_pool">pool</a>.id
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_add_obc_to_pool"></a>

## Function `add_obc_to_pool`

Add obc to pool for gas exchange.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_add_obc_to_pool">add_obc_to_pool</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;obc::OBC&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_add_obc_to_pool">add_obc_to_pool</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;OBC&gt;) {
    <b>let</b> amount = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>);
    <b>assert</b>!( amount &gt; 0, <a href="exchange_inner.md#0xc8_exchange_inner_EZeroAmount">EZeroAmount</a>);
    <a href="pool.md#0xc8_pool">pool</a>.obc_balance = <a href="pool.md#0xc8_pool">pool</a>.obc_balance + amount;
    <b>let</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a> = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.obc_pool, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>);
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_is_active"></a>

## Function `is_active`

Returns true if the input exchange pool is active.


<pre><code><b>public</b> <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;): bool {
    <a href="_is_some">option::is_some</a>(&<a href="pool.md#0xc8_pool">pool</a>.activation_epoch)
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_get_obc_amount"></a>

## Function `get_obc_amount`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_obc_amount">get_obc_amount</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_obc_amount">get_obc_amount</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;): u64 {
    <a href="pool.md#0xc8_pool">pool</a>.obc_balance
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_get_stable_amount"></a>

## Function `get_stable_amount`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_stable_amount">get_stable_amount</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_get_stable_amount">get_stable_amount</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;): u64 {
    <a href="pool.md#0xc8_pool">pool</a>.stable_token_balance
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


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_stable">request_exchange_stable</a>&lt;STABLE_COIN&gt;(exchange_rate: u64, <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, <a href="stable_coin.md#0xc8_stable_coin">stable_coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;STABLE_COIN&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;obc::OBC&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_stable">request_exchange_stable</a>&lt;STABLE_COIN&gt;(
    exchange_rate: u64,
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
    <a href="stable_coin.md#0xc8_stable_coin">stable_coin</a>: Coin&lt;STABLE_COIN&gt;,
    ctx: &<b>mut</b> TxContext
): Balance&lt;OBC&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&<a href="stable_coin.md#0xc8_stable_coin">stable_coin</a>) &gt; 0, <a href="exchange_inner.md#0xc8_exchange_inner_EZeroAmount">EZeroAmount</a>);
    <b>let</b> tok_balance = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="stable_coin.md#0xc8_stable_coin">stable_coin</a>);
    <b>let</b> stable_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&tok_balance);
    <b>let</b> obc_amount= <a href="exchange_inner.md#0xc8_exchange_inner_exchange_obc_amount">exchange_obc_amount</a>(exchange_rate, stable_amount);
    <b>assert</b>!(obc_amount &gt; 0, <a href="exchange_inner.md#0xc8_exchange_inner_EOBCZeroAmount">EOBCZeroAmount</a>);
    <b>assert</b>!(<a href="pool.md#0xc8_pool">pool</a>.obc_balance &gt; obc_amount, <a href="exchange_inner.md#0xc8_exchange_inner_ELackOfOBC">ELackOfOBC</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.stable_pool, tok_balance);
    <b>let</b> result = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_take">coin::take</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.obc_pool, obc_amount, ctx);
    <a href="pool.md#0xc8_pool">pool</a>.obc_balance = <a href="pool.md#0xc8_pool">pool</a>.obc_balance - obc_amount;
    <a href="pool.md#0xc8_pool">pool</a>.stable_token_balance = <a href="pool.md#0xc8_pool">pool</a>.stable_token_balance + stable_amount;
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(result)
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_request_exchange_all"></a>

## Function `request_exchange_all`

Exchange all stable gas coins to default coins


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_all">request_exchange_all</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_all">request_exchange_all</a>&lt;STABLE_COIN&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="exchange_inner.md#0xc8_exchange_inner_is_active">is_active</a>(<a href="pool.md#0xc8_pool">pool</a>), <a href="exchange_inner.md#0xc8_exchange_inner_ENotActivePool">ENotActivePool</a>);
    <b>if</b>(<a href="pool.md#0xc8_pool">pool</a>.stable_token_balance &gt; 0) {
        // call <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> <a href="swap.md#0xc8_swap">swap</a> interface
        // <b>let</b> <a href="obc.md#0xc8_obc">obc</a> = stable_coin::request_swap_obc&lt;CoinType&gt;(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>&lt;CoinType&gt;(<a href="pool.md#0xc8_pool">pool</a>.stable_pool, ctx), ctx);
        <b>let</b> <a href="obc.md#0xc8_obc">obc</a> = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;OBC&gt;(ctx);//<a href="pool.md#0xc8_pool_swap_token">pool::swap_token</a>&lt;P,T&gt;(<a href="stable_coin.md#0xc8_stable_coin_new_dummy">stable_coin::new_dummy</a>&lt;T&gt;(ctx), ctx);
        // store <a href="obc.md#0xc8_obc">obc</a> <b>to</b> exchange <a href="pool.md#0xc8_pool">pool</a>
        <a href="pool.md#0xc8_pool">pool</a>.obc_balance = <a href="pool.md#0xc8_pool">pool</a>.obc_balance + <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&<a href="obc.md#0xc8_obc">obc</a>);
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.obc_pool, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="obc.md#0xc8_obc">obc</a>));
        <a href="pool.md#0xc8_pool">pool</a>.stable_token_balance = 0;
    }
}
</code></pre>



</details>

<a name="0xc8_exchange_inner_request_withdraw_stable"></a>

## Function `request_withdraw_stable`

Withdraw the stable gas coins.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_stable">request_withdraw_stable</a>&lt;STABLE_COIN&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;STABLE_COIN&gt;): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;STABLE_COIN&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_stable">request_withdraw_stable</a>&lt;STABLE_COIN&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">ExchangePool</a>&lt;STABLE_COIN&gt;,
): Balance&lt;STABLE_COIN&gt; {
    <a href="pool.md#0xc8_pool">pool</a>.stable_token_balance = 0;
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_withdraw_all">balance::withdraw_all</a>&lt;STABLE_COIN&gt;(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.stable_pool)
}
</code></pre>



</details>

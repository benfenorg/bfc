
<a name="0xc8_pool"></a>

# Module `0xc8::pool`



-  [Struct `LSP`](#0xc8_pool_LSP)
-  [Resource `Pool`](#0xc8_pool_Pool)
-  [Constants](#@Constants_0)
-  [Function `init`](#0xc8_pool_init)
-  [Function `create_pool`](#0xc8_pool_create_pool)
-  [Function `swap_obc_`](#0xc8_pool_swap_obc_)
-  [Function `swap_obc`](#0xc8_pool_swap_obc)
-  [Function `swap_token`](#0xc8_pool_swap_token)
-  [Function `add_liquidity_`](#0xc8_pool_add_liquidity_)
-  [Function `add_liquidity`](#0xc8_pool_add_liquidity)
-  [Function `remove_liquidity_`](#0xc8_pool_remove_liquidity_)
-  [Function `remove_liquidity`](#0xc8_pool_remove_liquidity)
-  [Function `sui_price`](#0xc8_pool_sui_price)
-  [Function `token_price`](#0xc8_pool_token_price)
-  [Function `get_amounts`](#0xc8_pool_get_amounts)
-  [Function `get_input_price`](#0xc8_pool_get_input_price)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/math.md#0x2_math">0x2::math</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="stable_coin.md#0xc8_stable_coin">0xc8::stable_coin</a>;
</code></pre>



<a name="0xc8_pool_LSP"></a>

## Struct `LSP`

The Pool token that will be used to mark the pool share
of a liquidity provider. The first type parameter stands
for the witness type of a pool. The seconds is for the
coin held in the pool.


<pre><code><b>struct</b> <a href="pool.md#0xc8_pool_LSP">LSP</a>&lt;P, T&gt; <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_pool_Pool"></a>

## Resource `Pool`

The pool with exchange.

- <code>fee_percent</code> should be in the range: [0-10000), meaning
that 1000 is 100% and 1 is 0.1%


<pre><code><b>struct</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt; <b>has</b> key
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
<code>sui: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>token: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;T&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>lsp_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="pool.md#0xc8_pool_LSP">pool::LSP</a>&lt;P, T&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>fee_percent: u64</code>
</dt>
<dd>
 Fee Percent is denominated in basis points.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_pool_EZeroAmount"></a>

For when supplied Coin is zero.


<pre><code><b>const</b> <a href="pool.md#0xc8_pool_EZeroAmount">EZeroAmount</a>: u64 = 0;
</code></pre>



<a name="0xc8_pool_EPoolFull"></a>

For when someone attempts to add more liquidity than u128 Math allows.


<pre><code><b>const</b> <a href="pool.md#0xc8_pool_EPoolFull">EPoolFull</a>: u64 = 4;
</code></pre>



<a name="0xc8_pool_EReservesEmpty"></a>

For when someone tries to swap in an empty pool.


<pre><code><b>const</b> <a href="pool.md#0xc8_pool_EReservesEmpty">EReservesEmpty</a>: u64 = 2;
</code></pre>



<a name="0xc8_pool_EShareEmpty"></a>

For when initial LSP amount is zero.


<pre><code><b>const</b> <a href="pool.md#0xc8_pool_EShareEmpty">EShareEmpty</a>: u64 = 3;
</code></pre>



<a name="0xc8_pool_EWrongFee"></a>

For when pool fee is set incorrectly.
Allowed values are: [0-10000).


<pre><code><b>const</b> <a href="pool.md#0xc8_pool_EWrongFee">EWrongFee</a>: u64 = 1;
</code></pre>



<a name="0xc8_pool_FEE_SCALING"></a>

The integer scaling setting for fees calculation.


<pre><code><b>const</b> <a href="pool.md#0xc8_pool_FEE_SCALING">FEE_SCALING</a>: u128 = 10000;
</code></pre>



<a name="0xc8_pool_MAX_POOL_VALUE"></a>

The max value that can be held in one of the Balances of
a Pool. U64 MAX / FEE_SCALING


<pre><code><b>const</b> <a href="pool.md#0xc8_pool_MAX_POOL_VALUE">MAX_POOL_VALUE</a>: u64 = 1844674407370955;
</code></pre>



<a name="0xc8_pool_init"></a>

## Function `init`

Module initializer is empty - to publish a new Pool one has
to create a type which will mark LSPs.


<pre><code><b>fun</b> <a href="pool.md#0xc8_pool_init">init</a>(_: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="pool.md#0xc8_pool_init">init</a>(_: &<b>mut</b> TxContext) {}
</code></pre>



</details>

<a name="0xc8_pool_create_pool"></a>

## Function `create_pool`

Create new <code><a href="pool.md#0xc8_pool_Pool">Pool</a></code> for token <code>T</code>. Each Pool holds a <code>Coin&lt;T&gt;</code>
and a <code>Coin&lt;SUI&gt;</code>. Swaps are available in both directions.

Share is calculated based on Uniswap's constant product formula:
liquidity = sqrt( X * Y )


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_create_pool">create_pool</a>&lt;P: drop, T&gt;(_: P, token: <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;T&gt;, sui: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, fee_percent: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="pool.md#0xc8_pool_LSP">pool::LSP</a>&lt;P, T&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_create_pool">create_pool</a>&lt;P: drop, T&gt;(
    _: P,
    token: DummyCoin&lt;T&gt;,
    sui: Coin&lt;OBC&gt;,
    fee_percent: u64,
    ctx: &<b>mut</b> TxContext
): Coin&lt;<a href="pool.md#0xc8_pool_LSP">LSP</a>&lt;P, T&gt;&gt; {
    <b>let</b> sui_amt = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&sui);
    <b>let</b> tok_amt = <a href="stable_coin.md#0xc8_stable_coin_value">stable_coin::value</a>(&token);

    <b>assert</b>!(sui_amt &gt; 0 && tok_amt &gt; 0, <a href="pool.md#0xc8_pool_EZeroAmount">EZeroAmount</a>);
    <b>assert</b>!(sui_amt &lt; <a href="pool.md#0xc8_pool_MAX_POOL_VALUE">MAX_POOL_VALUE</a> && tok_amt &lt; <a href="pool.md#0xc8_pool_MAX_POOL_VALUE">MAX_POOL_VALUE</a>, <a href="pool.md#0xc8_pool_EPoolFull">EPoolFull</a>);
    <b>assert</b>!(fee_percent &gt;= 0 && fee_percent &lt; 10000, <a href="pool.md#0xc8_pool_EWrongFee">EWrongFee</a>);

    // Initial share of <a href="pool.md#0xc8_pool_LSP">LSP</a> is the sqrt(a) * sqrt(b)
    <b>let</b> share = <a href="../../../.././build/Sui/docs/math.md#0x2_math_sqrt">math::sqrt</a>(sui_amt) * <a href="../../../.././build/Sui/docs/math.md#0x2_math_sqrt">math::sqrt</a>(tok_amt);
    <b>let</b> lsp_supply = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_create_supply">balance::create_supply</a>(<a href="pool.md#0xc8_pool_LSP">LSP</a>&lt;P, T&gt; {});
    <b>let</b> lsp = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_increase_supply">balance::increase_supply</a>(&<b>mut</b> lsp_supply, share);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(<a href="pool.md#0xc8_pool_Pool">Pool</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        token: <a href="stable_coin.md#0xc8_stable_coin_into_balance">stable_coin::into_balance</a>(token),
        sui: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(sui),
        lsp_supply,
        fee_percent
    });

    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(lsp, ctx)
}
</code></pre>



</details>

<a name="0xc8_pool_swap_obc_"></a>

## Function `swap_obc_`

Entrypoint for the <code>swap_sui</code> method. Sends swapped token
to sender.


<pre><code>entry <b>fun</b> <a href="pool.md#0xc8_pool_swap_obc_">swap_obc_</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, sui: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>fun</b> <a href="pool.md#0xc8_pool_swap_obc_">swap_obc_</a>&lt;P, T&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;, sui: Coin&lt;OBC&gt;, ctx: &<b>mut</b> TxContext
) {
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(
        <a href="pool.md#0xc8_pool_swap_obc">swap_obc</a>(<a href="pool.md#0xc8_pool">pool</a>, sui, ctx),
        <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    )
}
</code></pre>



</details>

<a name="0xc8_pool_swap_obc"></a>

## Function `swap_obc`

Swap <code>Coin&lt;SUI&gt;</code> for the <code>Coin&lt;T&gt;</code>.
Returns Coin<T>.


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_swap_obc">swap_obc</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, sui: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_swap_obc">swap_obc</a>&lt;P, T&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;, sui: Coin&lt;OBC&gt;, ctx: &<b>mut</b> TxContext
): Coin&lt;T&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&sui) &gt; 0, <a href="pool.md#0xc8_pool_EZeroAmount">EZeroAmount</a>);

    <b>let</b> sui_balance = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(sui);

    // Calculate the output amount - fee
    <b>let</b> (sui_reserve, token_reserve, _) = <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>(<a href="pool.md#0xc8_pool">pool</a>);

    <b>assert</b>!(sui_reserve &gt; 0 && token_reserve &gt; 0, <a href="pool.md#0xc8_pool_EReservesEmpty">EReservesEmpty</a>);

    <b>let</b> output_amount = <a href="pool.md#0xc8_pool_get_input_price">get_input_price</a>(
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&sui_balance),
        sui_reserve,
        token_reserve,
        <a href="pool.md#0xc8_pool">pool</a>.fee_percent
    );

    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.sui, sui_balance);
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_take">coin::take</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.token, output_amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_pool_swap_token"></a>

## Function `swap_token`

Swap <code>Coin&lt;T&gt;</code> for the <code>Coin&lt;SUI&gt;</code>.
Returns the swapped <code>Coin&lt;SUI&gt;</code>.


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_swap_token">swap_token</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, token: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_swap_token">swap_token</a>&lt;P, T&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;, token: Coin&lt;T&gt;, ctx: &<b>mut</b> TxContext
): Coin&lt;OBC&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&token) &gt; 0, <a href="pool.md#0xc8_pool_EZeroAmount">EZeroAmount</a>);

    <b>let</b> tok_balance = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(token);
    <b>let</b> (sui_reserve, token_reserve, _) = <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>(<a href="pool.md#0xc8_pool">pool</a>);

    <b>assert</b>!(sui_reserve &gt; 0 && token_reserve &gt; 0, <a href="pool.md#0xc8_pool_EReservesEmpty">EReservesEmpty</a>);

    <b>let</b> output_amount = <a href="pool.md#0xc8_pool_get_input_price">get_input_price</a>(
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&tok_balance),
        token_reserve,
        sui_reserve,
        <a href="pool.md#0xc8_pool">pool</a>.fee_percent
    );

    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.token, tok_balance);
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_take">coin::take</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.sui, output_amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_pool_add_liquidity_"></a>

## Function `add_liquidity_`

Entrypoint for the <code>add_liquidity</code> method. Sends <code>Coin&lt;<a href="pool.md#0xc8_pool_LSP">LSP</a>&gt;</code> to
the transaction sender.


<pre><code>entry <b>fun</b> <a href="pool.md#0xc8_pool_add_liquidity_">add_liquidity_</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, sui: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, token: <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>fun</b> <a href="pool.md#0xc8_pool_add_liquidity_">add_liquidity_</a>&lt;P, T&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;, sui: Coin&lt;OBC&gt;, token: DummyCoin&lt;T&gt;, ctx: &<b>mut</b> TxContext
) {
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(
        <a href="pool.md#0xc8_pool_add_liquidity">add_liquidity</a>(<a href="pool.md#0xc8_pool">pool</a>, sui, token, ctx),
        <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx)
    );
}
</code></pre>



</details>

<a name="0xc8_pool_add_liquidity"></a>

## Function `add_liquidity`

Add liquidity to the <code><a href="pool.md#0xc8_pool_Pool">Pool</a></code>. Sender needs to provide both
<code>Coin&lt;SUI&gt;</code> and <code>Coin&lt;T&gt;</code>, and in exchange he gets <code>Coin&lt;<a href="pool.md#0xc8_pool_LSP">LSP</a>&gt;</code> -
liquidity provider tokens.


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_add_liquidity">add_liquidity</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, sui: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, token: <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;T&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="pool.md#0xc8_pool_LSP">pool::LSP</a>&lt;P, T&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_add_liquidity">add_liquidity</a>&lt;P, T&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;, sui: Coin&lt;OBC&gt;, token: DummyCoin&lt;T&gt;, ctx: &<b>mut</b> TxContext
): Coin&lt;<a href="pool.md#0xc8_pool_LSP">LSP</a>&lt;P, T&gt;&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&sui) &gt; 0, <a href="pool.md#0xc8_pool_EZeroAmount">EZeroAmount</a>);
    <b>assert</b>!(<a href="stable_coin.md#0xc8_stable_coin_value">stable_coin::value</a>(&token) &gt; 0, <a href="pool.md#0xc8_pool_EZeroAmount">EZeroAmount</a>);

    <b>let</b> sui_balance = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(sui);
    <b>let</b> tok_balance = <a href="stable_coin.md#0xc8_stable_coin_into_balance">stable_coin::into_balance</a>(token);

    <b>let</b> (sui_amount, tok_amount, lsp_supply) = <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>(<a href="pool.md#0xc8_pool">pool</a>);

    <b>let</b> sui_added = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&sui_balance);
    <b>let</b> tok_added = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&tok_balance);
    <b>let</b> share_minted = <a href="../../../.././build/Sui/docs/math.md#0x2_math_min">math::min</a>(
        (sui_added * lsp_supply) / sui_amount,
        (tok_added * lsp_supply) / tok_amount
    );

    <b>let</b> sui_amt = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.sui, sui_balance);
    <b>let</b> tok_amt = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.token, tok_balance);

    <b>assert</b>!(sui_amt &lt; <a href="pool.md#0xc8_pool_MAX_POOL_VALUE">MAX_POOL_VALUE</a>, <a href="pool.md#0xc8_pool_EPoolFull">EPoolFull</a>);
    <b>assert</b>!(tok_amt &lt; <a href="pool.md#0xc8_pool_MAX_POOL_VALUE">MAX_POOL_VALUE</a>, <a href="pool.md#0xc8_pool_EPoolFull">EPoolFull</a>);

    <b>let</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a> = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_increase_supply">balance::increase_supply</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.lsp_supply, share_minted);
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>, ctx)
}
</code></pre>



</details>

<a name="0xc8_pool_remove_liquidity_"></a>

## Function `remove_liquidity_`

Entrypoint for the <code>remove_liquidity</code> method. Transfers
withdrawn assets to the sender.


<pre><code>entry <b>fun</b> <a href="pool.md#0xc8_pool_remove_liquidity_">remove_liquidity_</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, lsp: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="pool.md#0xc8_pool_LSP">pool::LSP</a>&lt;P, T&gt;&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>fun</b> <a href="pool.md#0xc8_pool_remove_liquidity_">remove_liquidity_</a>&lt;P, T&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;,
    lsp: Coin&lt;<a href="pool.md#0xc8_pool_LSP">LSP</a>&lt;P, T&gt;&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> (sui, token) = <a href="pool.md#0xc8_pool_remove_liquidity">remove_liquidity</a>(<a href="pool.md#0xc8_pool">pool</a>, lsp, ctx);
    <b>let</b> sender = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(sui, sender);
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(token, sender);
}
</code></pre>



</details>

<a name="0xc8_pool_remove_liquidity"></a>

## Function `remove_liquidity`

Remove liquidity from the <code><a href="pool.md#0xc8_pool_Pool">Pool</a></code> by burning <code>Coin&lt;<a href="pool.md#0xc8_pool_LSP">LSP</a>&gt;</code>.
Returns <code>Coin&lt;T&gt;</code> and <code>Coin&lt;SUI&gt;</code>.


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_remove_liquidity">remove_liquidity</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, lsp: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="pool.md#0xc8_pool_LSP">pool::LSP</a>&lt;P, T&gt;&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_remove_liquidity">remove_liquidity</a>&lt;P, T&gt;(
    <a href="pool.md#0xc8_pool">pool</a>: &<b>mut</b> <a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;,
    lsp: Coin&lt;<a href="pool.md#0xc8_pool_LSP">LSP</a>&lt;P, T&gt;&gt;,
    ctx: &<b>mut</b> TxContext
): (Coin&lt;OBC&gt;, DummyCoin&lt;T&gt;) {
    <b>let</b> lsp_amount = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&lsp);

    // If there's a non-empty <a href="pool.md#0xc8_pool_LSP">LSP</a>, we can
    <b>assert</b>!(lsp_amount &gt; 0, <a href="pool.md#0xc8_pool_EZeroAmount">EZeroAmount</a>);

    <b>let</b> (sui_amt, tok_amt, lsp_supply) = <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>(<a href="pool.md#0xc8_pool">pool</a>);
    <b>let</b> sui_removed = (sui_amt * lsp_amount) / lsp_supply;
    <b>let</b> tok_removed = (tok_amt * lsp_amount) / lsp_supply;

    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_decrease_supply">balance::decrease_supply</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.lsp_supply, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(lsp));

    (
        <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_take">coin::take</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.sui, sui_removed, ctx),
        <a href="stable_coin.md#0xc8_stable_coin_take">stable_coin::take</a>(&<b>mut</b> <a href="pool.md#0xc8_pool">pool</a>.token, tok_removed, ctx)
    )
}
</code></pre>



</details>

<a name="0xc8_pool_sui_price"></a>

## Function `sui_price`

Public getter for the price of SUI in token T.
- How much SUI one will get if they send <code>to_sell</code> amount of T;


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_sui_price">sui_price</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, to_sell: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_sui_price">sui_price</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;, to_sell: u64): u64 {
    <b>let</b> (sui_amt, tok_amt, _) = <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>(<a href="pool.md#0xc8_pool">pool</a>);
    <a href="pool.md#0xc8_pool_get_input_price">get_input_price</a>(to_sell, tok_amt, sui_amt, <a href="pool.md#0xc8_pool">pool</a>.fee_percent)
}
</code></pre>



</details>

<a name="0xc8_pool_token_price"></a>

## Function `token_price`

Public getter for the price of token T in SUI.
- How much T one will get if they send <code>to_sell</code> amount of SUI;


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_token_price">token_price</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;, to_sell: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_token_price">token_price</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;, to_sell: u64): u64 {
    <b>let</b> (sui_amt, tok_amt, _) = <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>(<a href="pool.md#0xc8_pool">pool</a>);
    <a href="pool.md#0xc8_pool_get_input_price">get_input_price</a>(to_sell, sui_amt, tok_amt, <a href="pool.md#0xc8_pool">pool</a>.fee_percent)
}
</code></pre>



</details>

<a name="0xc8_pool_get_amounts"></a>

## Function `get_amounts`

Get most used values in a handy way:
- amount of SUI
- amount of token
- total supply of LSP


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="pool.md#0xc8_pool_Pool">pool::Pool</a>&lt;P, T&gt;): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_get_amounts">get_amounts</a>&lt;P, T&gt;(<a href="pool.md#0xc8_pool">pool</a>: &<a href="pool.md#0xc8_pool_Pool">Pool</a>&lt;P, T&gt;): (u64, u64, u64) {
    (
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&<a href="pool.md#0xc8_pool">pool</a>.sui),
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&<a href="pool.md#0xc8_pool">pool</a>.token),
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_supply_value">balance::supply_value</a>(&<a href="pool.md#0xc8_pool">pool</a>.lsp_supply)
    )
}
</code></pre>



</details>

<a name="0xc8_pool_get_input_price"></a>

## Function `get_input_price`

Calculate the output amount minus the fee - 0.3%


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_get_input_price">get_input_price</a>(input_amount: u64, input_reserve: u64, output_reserve: u64, fee_percent: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="pool.md#0xc8_pool_get_input_price">get_input_price</a>(
    input_amount: u64, input_reserve: u64, output_reserve: u64, fee_percent: u64
): u64 {
    // up casts
    <b>let</b> (
        input_amount,
        input_reserve,
        output_reserve,
        fee_percent
    ) = (
        (input_amount <b>as</b> u128),
        (input_reserve <b>as</b> u128),
        (output_reserve <b>as</b> u128),
        (fee_percent <b>as</b> u128)
    );

    <b>let</b> input_amount_with_fee = input_amount * (<a href="pool.md#0xc8_pool_FEE_SCALING">FEE_SCALING</a> - fee_percent);
    <b>let</b> numerator = input_amount_with_fee * output_reserve;
    <b>let</b> denominator = (input_reserve * <a href="pool.md#0xc8_pool_FEE_SCALING">FEE_SCALING</a>) + input_amount_with_fee;

    (numerator / denominator <b>as</b> u64)
}
</code></pre>



</details>

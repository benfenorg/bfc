
<a name="0xc8_swap"></a>

# Module `0xc8::swap`



-  [Constants](#@Constants_0)
-  [Function `mint`](#0xc8_swap_mint)
-  [Function `burn`](#0xc8_swap_burn)
-  [Function `transfer_or_delete`](#0xc8_swap_transfer_or_delete)
-  [Function `swap_internal`](#0xc8_swap_swap_internal)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="treasury.md#0xc8_treasury">0xc8::treasury</a>;
<b>use</b> <a href="utils.md#0xc8_utils">0xc8::utils</a>;
<b>use</b> <a href="vault.md#0xc8_vault">0xc8::vault</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0xc8_swap_ERR_ZERO_AMOUNT"></a>

Errors


<pre><code><b>const</b> <a href="swap.md#0xc8_swap_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>: u64 = 0;
</code></pre>



<a name="0xc8_swap_TickSpacing"></a>

Constants


<pre><code><b>const</b> <a href="swap.md#0xc8_swap_TickSpacing">TickSpacing</a>: u32 = 60;
</code></pre>



<a name="0xc8_swap_mint"></a>

## Function `mint`

Mint swap obc to stablecoin


<pre><code><b>public</b> entry <b>fun</b> <a href="swap.md#0xc8_swap_mint">mint</a>&lt;StableCoinType&gt;(<a href="treasury.md#0xc8_treasury">treasury</a>: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, coin_obc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;obc::OBC&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="swap.md#0xc8_swap_mint">mint</a>&lt;StableCoinType&gt;(
    <a href="treasury.md#0xc8_treasury">treasury</a>: &<b>mut</b> Treasury,
    coin_obc: Coin&lt;OBC&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>&lt;OBC&gt;(&coin_obc) &gt; 0, <a href="swap.md#0xc8_swap_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>if</b> (<a href="utils.md#0xc8_utils_cmp">utils::cmp</a>&lt;OBC, StableCoinType&gt;() &lt; 1) {
        <a href="swap.md#0xc8_swap_swap_internal">swap_internal</a>&lt;OBC, StableCoinType&gt;(
            <a href="treasury.md#0xc8_treasury">treasury</a>,
            <b>true</b>,
            coin_obc,
            <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;StableCoinType&gt;(ctx),
            amount,
            ctx,
        );
    } <b>else</b> {
        <a href="swap.md#0xc8_swap_swap_internal">swap_internal</a>&lt;StableCoinType, OBC&gt;(
            <a href="treasury.md#0xc8_treasury">treasury</a>,
            <b>false</b>,
            <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;StableCoinType&gt;(ctx),
            coin_obc,
            amount,
            ctx,
        );
    };
}
</code></pre>



</details>

<a name="0xc8_swap_burn"></a>

## Function `burn`

Burn swap stablecoin to obc


<pre><code><b>public</b> entry <b>fun</b> <a href="swap.md#0xc8_swap_burn">burn</a>&lt;StableCoinType&gt;(<a href="treasury.md#0xc8_treasury">treasury</a>: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="swap.md#0xc8_swap_burn">burn</a>&lt;StableCoinType&gt;(
    <a href="treasury.md#0xc8_treasury">treasury</a>: &<b>mut</b> Treasury,
    coin_sc: Coin&lt;StableCoinType&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>&lt;StableCoinType&gt;(&coin_sc) &gt; 0, <a href="swap.md#0xc8_swap_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>if</b> (<a href="utils.md#0xc8_utils_cmp">utils::cmp</a>&lt;OBC, StableCoinType&gt;() &lt; 1) {
        <a href="swap.md#0xc8_swap_swap_internal">swap_internal</a>&lt;OBC, StableCoinType&gt;(
            <a href="treasury.md#0xc8_treasury">treasury</a>,
            <b>false</b>,
            <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;OBC&gt;(ctx),
            coin_sc,
            amount,
            ctx,
        );
    } <b>else</b> {
        <a href="swap.md#0xc8_swap_swap_internal">swap_internal</a>&lt;StableCoinType, OBC&gt;(
            <a href="treasury.md#0xc8_treasury">treasury</a>,
            <b>true</b>,
            coin_sc,
            <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;OBC&gt;(ctx),
            amount,
            ctx,
        );
    };
}
</code></pre>



</details>

<a name="0xc8_swap_transfer_or_delete"></a>

## Function `transfer_or_delete`



<pre><code><b>fun</b> <a href="swap.md#0xc8_swap_transfer_or_delete">transfer_or_delete</a>&lt;CoinType&gt;(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;CoinType&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="swap.md#0xc8_swap_transfer_or_delete">transfer_or_delete</a>&lt;CoinType&gt;(
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>: Balance&lt;CoinType&gt;,
    ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>) &gt; 0) {
        <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>, ctx), <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
    } <b>else</b> {
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>);
    }
}
</code></pre>



</details>

<a name="0xc8_swap_swap_internal"></a>

## Function `swap_internal`

Internal swap


<pre><code><b>fun</b> <a href="swap.md#0xc8_swap_swap_internal">swap_internal</a>&lt;CoinTypeA, CoinTypeB&gt;(<a href="treasury.md#0xc8_treasury">treasury</a>: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, a2b: bool, coin_a: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinTypeA&gt;, coin_b: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinTypeB&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="swap.md#0xc8_swap_swap_internal">swap_internal</a>&lt;CoinTypeA, CoinTypeB&gt;(
    <a href="treasury.md#0xc8_treasury">treasury</a>: &<b>mut</b> Treasury,
    a2b: bool, // <b>true</b> a-&gt;b , <b>false</b> b-&gt;a
    coin_a: Coin&lt;CoinTypeA&gt;,
    coin_b: Coin&lt;CoinTypeB&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> vault_key = <a href="treasury.md#0xc8_treasury_generate_vault_key">treasury::generate_vault_key</a>&lt;CoinTypeA, CoinTypeB&gt;(<a href="swap.md#0xc8_swap_TickSpacing">TickSpacing</a>);
    <b>let</b> mut_vault = <a href="treasury.md#0xc8_treasury_borrow_mut_vault">treasury::borrow_mut_vault</a>&lt;CoinTypeA, CoinTypeB&gt;(<a href="treasury.md#0xc8_treasury">treasury</a>, vault_key);
    <b>let</b> current_sqrt_price = <a href="vault.md#0xc8_vault_vault_current_sqrt_price">vault::vault_current_sqrt_price</a>(mut_vault);
    <b>let</b> (balance_a, balance_b) = <a href="vault.md#0xc8_vault_swap">vault::swap</a>&lt;CoinTypeA, CoinTypeB&gt;(
        mut_vault,
        coin_a,
        coin_b,
        a2b,
        <b>true</b>,
        amount,
        0, // ? unuse
        current_sqrt_price,
        ctx
    );
    <a href="swap.md#0xc8_swap_transfer_or_delete">transfer_or_delete</a>(balance_a, ctx);
    <a href="swap.md#0xc8_swap_transfer_or_delete">transfer_or_delete</a>(balance_b, ctx);
}
</code></pre>



</details>

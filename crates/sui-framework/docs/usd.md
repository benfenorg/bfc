
<a name="0xc8_usd"></a>

# Module `0xc8::usd`



-  [Struct `USD`](#0xc8_usd_USD)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_usd_new)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0xc8_usd_USD"></a>

## Struct `USD`



<pre><code><b>struct</b> <a href="usd.md#0xc8_usd_USD">USD</a> <b>has</b> drop
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

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_usd_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="usd.md#0xc8_usd_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="0xc8_usd_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="usd.md#0xc8_usd_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="0xc8_usd_new"></a>

## Function `new`



<pre><code><b>fun</b> <a href="usd.md#0xc8_usd_new">new</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="usd.md#0xc8_usd_new">new</a>(ctx: &<b>mut</b> TxContext): Supply&lt;<a href="usd.md#0xc8_usd_USD">USD</a>&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="usd.md#0xc8_usd_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="usd.md#0xc8_usd_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (cap, metadata) = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="usd.md#0xc8_usd_USD">USD</a> {},
        9,
        b"obUSD",
        b"ob <a href="usd.md#0xc8_usd">usd</a>",
        b"",
        <a href="_none">option::none</a>(),
        ctx
    );
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_treasury_into_supply">coin::treasury_into_supply</a>(cap)
}
</code></pre>



</details>

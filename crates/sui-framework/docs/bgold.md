
<a name="0xc8_bgold"></a>

# Module `0xc8::bgold`



-  [Struct `BGOLD`](#0xc8_bgold_BGOLD)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_bgold_new)
-  [Function `transfer`](#0xc8_bgold_transfer)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0xc8_bgold_BGOLD"></a>

## Struct `BGOLD`



<pre><code><b>struct</b> <a href="bgold.md#0xc8_bgold_BGOLD">BGOLD</a> <b>has</b> drop
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


<a name="0xc8_bgold_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="bgold.md#0xc8_bgold_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="0xc8_bgold_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="bgold.md#0xc8_bgold_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="0xc8_bgold_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="bgold.md#0xc8_bgold_new">new</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="bgold.md#0xc8_bgold_BGOLD">bgold::BGOLD</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bgold.md#0xc8_bgold_new">new</a>(ctx: &<b>mut</b> TxContext): Supply&lt;<a href="bgold.md#0xc8_bgold_BGOLD">BGOLD</a>&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="bgold.md#0xc8_bgold_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="bgold.md#0xc8_bgold_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (cap, metadata) = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="bgold.md#0xc8_bgold_BGOLD">BGOLD</a> {},
        9,
        b"<a href="bgold.md#0xc8_bgold_BGOLD">BGOLD</a>",
        b"Benfen GOLD",
        b"",
        <a href="_none">option::none</a>(),
        ctx
    );
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_treasury_into_supply">coin::treasury_into_supply</a>(cap)
}
</code></pre>



</details>

<a name="0xc8_bgold_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">transfer</a>(c: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="bgold.md#0xc8_bgold_BGOLD">bgold::BGOLD</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">transfer</a>(c: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="bgold.md#0xc8_bgold_BGOLD">BGOLD</a>&gt;, recipient: <b>address</b>) {
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

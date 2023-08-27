
<a name="0xc8_usdx"></a>

# Module `0xc8::usdx`



-  [Struct `Usdx`](#0xc8_usdx_Usdx)
-  [Resource `Transfer`](#0xc8_usdx_Transfer)
-  [Resource `Registry`](#0xc8_usdx_Registry)
-  [Resource `AbcTreasuryCap`](#0xc8_usdx_AbcTreasuryCap)
-  [Constants](#@Constants_0)
-  [Function `init`](#0xc8_usdx_init)
-  [Function `swapped_amount`](#0xc8_usdx_swapped_amount)
-  [Function `banned`](#0xc8_usdx_banned)
-  [Function `create`](#0xc8_usdx_create)
-  [Function `mint`](#0xc8_usdx_mint)
-  [Function `burn`](#0xc8_usdx_burn)
-  [Function `ban`](#0xc8_usdx_ban)
-  [Function `transfer`](#0xc8_usdx_transfer)
-  [Function `accept_transfer`](#0xc8_usdx_accept_transfer)
-  [Function `take`](#0xc8_usdx_take)
-  [Function `put_back`](#0xc8_usdx_put_back)
-  [Function `borrow`](#0xc8_usdx_borrow)
-  [Function `borrow_mut`](#0xc8_usdx_borrow_mut)
-  [Function `zero`](#0xc8_usdx_zero)


<pre><code><b>use</b> <a href="">0x1::vector</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="stable_coin.md#0xc8_stable_coin">0xc8::stable_coin</a>;
</code></pre>



<a name="0xc8_usdx_Usdx"></a>

## Struct `Usdx`

The ticker of Abc regulated token


<pre><code><b>struct</b> <a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a> <b>has</b> drop
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

<a name="0xc8_usdx_Transfer"></a>

## Resource `Transfer`

A restricted transfer of Abc to another account.


<pre><code><b>struct</b> <a href="stable_coin.md#0xc8_usdx_Transfer">Transfer</a> <b>has</b> key
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
<code><a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code><b>to</b>: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_usdx_Registry"></a>

## Resource `Registry`

A registry of addresses banned from using the coin.


<pre><code><b>struct</b> <a href="stable_coin.md#0xc8_usdx_Registry">Registry</a> <b>has</b> key
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
<code>banned: <a href="">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>swapped_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_usdx_AbcTreasuryCap"></a>

## Resource `AbcTreasuryCap`

A AbcTreasuryCap for the balance::Supply.


<pre><code><b>struct</b> <a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">AbcTreasuryCap</a> <b>has</b> store, key
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
<code>supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_usdx_EAddressBanned"></a>

For when address has been banned and someone is trying to access the balance


<pre><code><b>const</b> <a href="stable_coin.md#0xc8_usdx_EAddressBanned">EAddressBanned</a>: u64 = 2;
</code></pre>



<a name="0xc8_usdx_ENotOwner"></a>

For when an attempting to interact with another account's RegulatedCoin<Abc>.


<pre><code><b>const</b> <a href="stable_coin.md#0xc8_usdx_ENotOwner">ENotOwner</a>: u64 = 1;
</code></pre>



<a name="0xc8_usdx_init"></a>

## Function `init`

Create the Abc currency and send the AbcTreasuryCap to the creator
as well as the first (and empty) balance of the RegulatedCoin<Abc>.

Also creates a shared Registry which holds banned addresses.


<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_init">init</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_init">init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> treasury_cap = <a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">AbcTreasuryCap</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_create_supply">balance::create_supply</a>(<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a> {})
    };

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="stable_coin.md#0xc8_usdx_zero">zero</a>(sender, ctx), sender);
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(treasury_cap, sender);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(<a href="stable_coin.md#0xc8_usdx_Registry">Registry</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        banned: <a href="_empty">vector::empty</a>(),
        swapped_amount: 0,
    });
}
</code></pre>



</details>

<a name="0xc8_usdx_swapped_amount"></a>

## Function `swapped_amount`

Get total amount of <code>Coin</code> from the <code><a href="stable_coin.md#0xc8_usdx_Registry">Registry</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0xc8_usdx_swapped_amount">swapped_amount</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">usdx::Registry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0xc8_usdx_swapped_amount">swapped_amount</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">Registry</a>): u64 {
    r.swapped_amount
}
</code></pre>



</details>

<a name="0xc8_usdx_banned"></a>

## Function `banned`

Get vector of banned addresses from <code><a href="stable_coin.md#0xc8_usdx_Registry">Registry</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0xc8_usdx_banned">banned</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">usdx::Registry</a>): &<a href="">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stable_coin.md#0xc8_usdx_banned">banned</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">Registry</a>): &<a href="">vector</a>&lt;<b>address</b>&gt; {
    &r.banned
}
</code></pre>



</details>

<a name="0xc8_usdx_create"></a>

## Function `create`

Create an empty <code>RCoin&lt;Abc&gt;</code> instance for account <code>for</code>. AbcTreasuryCap is passed for
authentication purposes - only admin can create new accounts.


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_create">create</a>(_: &<a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">usdx::AbcTreasuryCap</a>, for: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_create">create</a>(_: &<a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">AbcTreasuryCap</a>, for: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="stable_coin.md#0xc8_usdx_zero">zero</a>(for, ctx), for)
}
</code></pre>



</details>

<a name="0xc8_usdx_mint"></a>

## Function `mint`

Mint more Abc. Requires AbcTreasuryCap for authorization, so can only be done by admins.


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_mint">mint</a>(treasury: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">usdx::AbcTreasuryCap</a>, owned: &<b>mut</b> <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_mint">mint</a>(treasury: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">AbcTreasuryCap</a>, owned: &<b>mut</b> DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;, value: u64) {
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(<a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(owned), <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_increase_supply">balance::increase_supply</a>(&<b>mut</b> treasury.supply, value));
}
</code></pre>



</details>

<a name="0xc8_usdx_burn"></a>

## Function `burn`

Burn <code>value</code> amount of <code>RCoin&lt;Abc&gt;</code>. Requires AbcTreasuryCap for authorization, so can only be done by admins.

TODO: Make AbcTreasuryCap a part of Balance module instead of Coin.


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_burn">burn</a>(treasury: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">usdx::AbcTreasuryCap</a>, owned: &<b>mut</b> <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_burn">burn</a>(treasury: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">AbcTreasuryCap</a>, owned: &<b>mut</b> DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;, value: u64) {
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_decrease_supply">balance::decrease_supply</a>(
        &<b>mut</b> treasury.supply,
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(<a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(owned), value)
    );
}
</code></pre>



</details>

<a name="0xc8_usdx_ban"></a>

## Function `ban`

Ban some address and forbid making any transactions from or to this address.
Only owner of the AbcTreasuryCap can perform this action.


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_ban">ban</a>(_cap: &<a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">usdx::AbcTreasuryCap</a>, registry: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_Registry">usdx::Registry</a>, to_ban: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_ban">ban</a>(_cap: &<a href="stable_coin.md#0xc8_usdx_AbcTreasuryCap">AbcTreasuryCap</a>, registry: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_Registry">Registry</a>, to_ban: <b>address</b>) {
    <a href="_push_back">vector::push_back</a>(&<b>mut</b> registry.banned, to_ban)
}
</code></pre>



</details>

<a name="0xc8_usdx_transfer"></a>

## Function `transfer`

Transfer entrypoint - create a restricted <code><a href="stable_coin.md#0xc8_usdx_Transfer">Transfer</a></code> instance and transfer it to the
<code><b>to</b></code> account for being accepted later.
Fails if sender is not an creator of the <code>RegulatedCoin</code> or if any of the parties is in
the ban list in Registry.


<pre><code><b>public</b> entry <b>fun</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">transfer</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">usdx::Registry</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;, value: u64, <b>to</b>: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">transfer</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">Registry</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;, value: u64, <b>to</b>: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

    <b>assert</b>!(rcoin::creator(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>) == sender, <a href="stable_coin.md#0xc8_usdx_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(<a href="_contains">vector::contains</a>(&r.banned, &<b>to</b>) == <b>false</b>, <a href="stable_coin.md#0xc8_usdx_EAddressBanned">EAddressBanned</a>);
    <b>assert</b>!(<a href="_contains">vector::contains</a>(&r.banned, &sender) == <b>false</b>, <a href="stable_coin.md#0xc8_usdx_EAddressBanned">EAddressBanned</a>);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="stable_coin.md#0xc8_usdx_Transfer">Transfer</a> {
        <b>to</b>,
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(<a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>), value),
    }, <b>to</b>)
}
</code></pre>



</details>

<a name="0xc8_usdx_accept_transfer"></a>

## Function `accept_transfer`

Accept an incoming transfer by joining an incoming balance with an owned one.

Fails if:
1. the <code>RegulatedCoin&lt;Abc&gt;.creator</code> does not match <code><a href="stable_coin.md#0xc8_usdx_Transfer">Transfer</a>.<b>to</b></code>;
2. the address of the creator/recipient is banned;


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_accept_transfer">accept_transfer</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">usdx::Registry</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;, <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">transfer</a>: <a href="stable_coin.md#0xc8_usdx_Transfer">usdx::Transfer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_accept_transfer">accept_transfer</a>(r: &<a href="stable_coin.md#0xc8_usdx_Registry">Registry</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;, <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">transfer</a>: <a href="stable_coin.md#0xc8_usdx_Transfer">Transfer</a>) {
    <b>let</b> <a href="stable_coin.md#0xc8_usdx_Transfer">Transfer</a> { id, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>, <b>to</b> } = <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">transfer</a>;

    <b>assert</b>!(rcoin::creator(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>) == <b>to</b>, <a href="stable_coin.md#0xc8_usdx_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(<a href="_contains">vector::contains</a>(&r.banned, &<b>to</b>) == <b>false</b>, <a href="stable_coin.md#0xc8_usdx_EAddressBanned">EAddressBanned</a>);

    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(<a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>), <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>);
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_delete">object::delete</a>(id)
}
</code></pre>



</details>

<a name="0xc8_usdx_take"></a>

## Function `take`

Take <code>value</code> amount of <code>RegulatedCoin</code> and make it freely transferable by wrapping it into
a <code>Coin</code>. Update <code><a href="stable_coin.md#0xc8_usdx_Registry">Registry</a></code> to keep track of the swapped amount.

Fails if:
1. <code>RegulatedCoin&lt;Abc&gt;.creator</code> was banned;
2. <code>RegulatedCoin&lt;Abc&gt;</code> is not owned by the tx sender;


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_take">take</a>(r: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_Registry">usdx::Registry</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;, value: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_take">take</a>(r: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_Registry">Registry</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;, value: u64, ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

    <b>assert</b>!(rcoin::creator(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>) == sender, <a href="stable_coin.md#0xc8_usdx_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(<a href="_contains">vector::contains</a>(&r.banned, &sender) == <b>false</b>, <a href="stable_coin.md#0xc8_usdx_EAddressBanned">EAddressBanned</a>);

    // Update swapped amount for <a href="stable_coin.md#0xc8_usdx_Registry">Registry</a> <b>to</b> keep track of non-regulated amounts.
    r.swapped_amount = r.swapped_amount + value;

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_take">coin::take</a>(<a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>), value, ctx), sender);
}
</code></pre>



</details>

<a name="0xc8_usdx_put_back"></a>

## Function `put_back`

Take <code>Coin</code> and put to the <code>RegulatedCoin</code>'s balance.

Fails if:
1. <code>RegulatedCoin&lt;Abc&gt;.creator</code> was banned;
2. <code>RegulatedCoin&lt;Abc&gt;</code> is not owned by the tx sender;


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_put_back">put_back</a>(r: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_Registry">usdx::Registry</a>, rc_coin: &<b>mut</b> <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;, ctx: &<a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stable_coin.md#0xc8_usdx_put_back">put_back</a>(r: &<b>mut</b> <a href="stable_coin.md#0xc8_usdx_Registry">Registry</a>, rc_coin: &<b>mut</b> DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;, ctx: &TxContext) {
    <b>let</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a> = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>);
    <b>let</b> sender = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

    <b>assert</b>!(rcoin::creator(rc_coin) == sender, <a href="stable_coin.md#0xc8_usdx_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(<a href="_contains">vector::contains</a>(&r.banned, &sender) == <b>false</b>, <a href="stable_coin.md#0xc8_usdx_EAddressBanned">EAddressBanned</a>);

    // Update swapped amount <b>as</b> in `swap_regulated`.
    r.swapped_amount = r.swapped_amount - <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&<a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>);

    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(<a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(rc_coin), <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>);
}
</code></pre>



</details>

<a name="0xc8_usdx_borrow"></a>

## Function `borrow`



<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_borrow">borrow</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;): &<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_borrow">borrow</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;): &Balance&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt; { <a href="stable_coin.md#0xc8_stable_coin_borrow">stable_coin::borrow</a>(<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a> {}, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>) }
</code></pre>



</details>

<a name="0xc8_usdx_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;): &<b>mut</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_borrow_mut">borrow_mut</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: &<b>mut</b> DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt;): &<b>mut</b> Balance&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt; { <a href="stable_coin.md#0xc8_stable_coin_borrow_mut">stable_coin::borrow_mut</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>) }
</code></pre>



</details>

<a name="0xc8_usdx_zero"></a>

## Function `zero`



<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_zero">zero</a>(creator: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="stable_coin.md#0xc8_stable_coin_DummyCoin">stable_coin::DummyCoin</a>&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">usdx::Usdx</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stable_coin.md#0xc8_usdx_zero">zero</a>(creator: <b>address</b>, ctx: &<b>mut</b> TxContext): DummyCoin&lt;<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a>&gt; { <a href="stable_coin.md#0xc8_stable_coin_zero">stable_coin::zero</a>(<a href="stable_coin.md#0xc8_usdx_Usdx">Usdx</a> {}, creator, ctx) }
</code></pre>



</details>

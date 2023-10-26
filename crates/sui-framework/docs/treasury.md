
<a name="0xc8_treasury"></a>

# Module `0xc8::treasury`



-  [Resource `Treasury`](#0xc8_treasury_Treasury)
-  [Constants](#@Constants_0)
-  [Function `create_treasury`](#0xc8_treasury_create_treasury)
-  [Function `index`](#0xc8_treasury_index)
-  [Function `get_balance`](#0xc8_treasury_get_balance)
-  [Function `check_vault`](#0xc8_treasury_check_vault)
-  [Function `get_vault_key`](#0xc8_treasury_get_vault_key)
-  [Function `borrow_vault`](#0xc8_treasury_borrow_vault)
-  [Function `borrow_mut_vault`](#0xc8_treasury_borrow_mut_vault)
-  [Function `vault_info`](#0xc8_treasury_vault_info)
-  [Function `create_vault`](#0xc8_treasury_create_vault)
-  [Function `init_vault_with_positions`](#0xc8_treasury_init_vault_with_positions)
-  [Function `create_vault_internal`](#0xc8_treasury_create_vault_internal)
-  [Function `mint`](#0xc8_treasury_mint)
-  [Function `mint_internal`](#0xc8_treasury_mint_internal)
-  [Function `redeem`](#0xc8_treasury_redeem)
-  [Function `redeem_internal`](#0xc8_treasury_redeem_internal)
-  [Function `calculate_swap_result`](#0xc8_treasury_calculate_swap_result)
-  [Function `transfer_or_delete`](#0xc8_treasury_transfer_or_delete)
-  [Function `swap_internal`](#0xc8_treasury_swap_internal)
-  [Function `next_epoch_bfc_required`](#0xc8_treasury_next_epoch_bfc_required)
-  [Function `deposit`](#0xc8_treasury_deposit)
-  [Function `rebalance`](#0xc8_treasury_rebalance)
-  [Function `rebalance_first_init`](#0xc8_treasury_rebalance_first_init)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="">0x1::type_name</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="busd.md#0xc8_busd">0xc8::busd</a>;
<b>use</b> <a href="event.md#0xc8_event">0xc8::event</a>;
<b>use</b> <a href="i32.md#0xc8_i32">0xc8::i32</a>;
<b>use</b> <a href="tick_math.md#0xc8_tick_math">0xc8::tick_math</a>;
<b>use</b> <a href="vault.md#0xc8_vault">0xc8::vault</a>;
</code></pre>



<a name="0xc8_treasury_Treasury"></a>

## Resource `Treasury`



<pre><code><b>struct</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a> <b>has</b> store, key
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
<code>bfc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>supplies: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_Bag">bag::Bag</a></code>
</dt>
<dd>
 stable coin supplies
</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>
 Vault index
</dd>
<dt>
<code>time_interval: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>updated_at: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>init: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_treasury_ERR_INSUFFICIENT"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_INSUFFICIENT">ERR_INSUFFICIENT</a>: u64 = 103;
</code></pre>



<a name="0xc8_treasury_ERR_POOL_HAS_REGISTERED"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_POOL_HAS_REGISTERED">ERR_POOL_HAS_REGISTERED</a>: u64 = 100;
</code></pre>



<a name="0xc8_treasury_ERR_POOL_NOT_EXISTS"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_POOL_NOT_EXISTS">ERR_POOL_NOT_EXISTS</a>: u64 = 101;
</code></pre>



<a name="0xc8_treasury_ERR_UNINITIALIZE_TREASURY"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_UNINITIALIZE_TREASURY">ERR_UNINITIALIZE_TREASURY</a>: u64 = 104;
</code></pre>



<a name="0xc8_treasury_ERR_ZERO_AMOUNT"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>: u64 = 102;
</code></pre>



<a name="0xc8_treasury_create_treasury"></a>

## Function `create_treasury`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_treasury">create_treasury</a>(time_interval: u32, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_treasury">create_treasury</a>(time_interval: u32, ctx: &<b>mut</b> TxContext): <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a> {
    <b>let</b> <a href="treasury.md#0xc8_treasury">treasury</a> = <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        bfc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;(),
        supplies: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_new">bag::new</a>(ctx),
        index: 0,
        time_interval,
        updated_at: 0,
        init: <b>false</b>,
    };
    <b>let</b> treasury_id = <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(&<a href="treasury.md#0xc8_treasury">treasury</a>);
    event::init_treasury(treasury_id);
    <a href="treasury.md#0xc8_treasury">treasury</a>
}
</code></pre>



</details>

<a name="0xc8_treasury_index"></a>

## Function `index`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_index">index</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_index">index</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>): u64 {
    _treasury.index
}
</code></pre>



</details>

<a name="0xc8_treasury_get_balance"></a>

## Function `get_balance`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_get_balance">get_balance</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_get_balance">get_balance</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>): u64 {
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_treasury.bfc_balance)
}
</code></pre>



</details>

<a name="0xc8_treasury_check_vault"></a>

## Function `check_vault`



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>, _vault_key: String) {
    <b>assert</b>!(
        <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(
            &_treasury.id,
            _vault_key
        ),
        <a href="treasury.md#0xc8_treasury_ERR_POOL_NOT_EXISTS">ERR_POOL_NOT_EXISTS</a>
    );
}
</code></pre>



</details>

<a name="0xc8_treasury_get_vault_key"></a>

## Function `get_vault_key`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;(): <a href="_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;(): String {
    <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;StableCoinType&gt;())
}
</code></pre>



</details>

<a name="0xc8_treasury_borrow_vault"></a>

## Function `borrow_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="_String">ascii::String</a>): &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _vault_key: String
): &Vault&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury, _vault_key);
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>&lt;String, Vault&lt;StableCoinType&gt;&gt;(&_treasury.id, _vault_key)
}
</code></pre>



</details>

<a name="0xc8_treasury_borrow_mut_vault"></a>

## Function `borrow_mut_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="_String">ascii::String</a>): &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _vault_key: String
): &<b>mut</b> Vault&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury, _vault_key);
    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;String, Vault&lt;StableCoinType&gt;&gt;(&<b>mut</b> _treasury.id, _vault_key)
}
</code></pre>



</details>

<a name="0xc8_treasury_vault_info"></a>

## Function `vault_info`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_vault_info">vault_info</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): <a href="vault.md#0xc8_vault_VaultInfo">vault::VaultInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_vault_info">vault_info</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>): VaultInfo {
    <a href="vault.md#0xc8_vault_vault_info">vault::vault_info</a>(
        <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury, <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;())
    )
}
</code></pre>



</details>

<a name="0xc8_treasury_create_vault"></a>

## Function `create_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault">create_vault</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _position_number: u32, _tick_spacing: u32, _spacing_times: u32, _initialize_price: u128, _base_point: u64, _max_counter_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault">create_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _supply: Supply&lt;StableCoinType&gt;,
    _position_number: u32,
    _tick_spacing: u32,
    _spacing_times: u32,
    _initialize_price: u128,
    _base_point: u64,
    _max_counter_times: u32,
    _ts: u64,
    _ctx: &<b>mut</b> TxContext
) {
    <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _supply,
        _tick_spacing,
        _spacing_times,
        _position_number,
        _initialize_price,
        _base_point,
        _max_counter_times,
        _ts,
        _ctx,
    );
}
</code></pre>



</details>

<a name="0xc8_treasury_init_vault_with_positions"></a>

## Function `init_vault_with_positions`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_init_vault_with_positions">init_vault_with_positions</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _initialize_price: u128, _base_point: u64, _position_number: u32, _tick_spacing: u32, _spacing_times: u32, _max_counter_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_init_vault_with_positions">init_vault_with_positions</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _supply: Supply&lt;StableCoinType&gt;,
    _initialize_price: u128,
    _base_point: u64,
    _position_number: u32,
    _tick_spacing: u32,
    _spacing_times: u32,
    _max_counter_times: u32,
    _ts: u64,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> vault_key = <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _supply,
        _tick_spacing,
        _spacing_times,
        _position_number,
        _initialize_price,
        _base_point,
        _max_counter_times,
        _ts,
        _ctx,
    );
    _ = <a href="vault.md#0xc8_vault_init_positions">vault::init_positions</a>&lt;StableCoinType&gt;(
        <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(_treasury, vault_key),
        _spacing_times,
        _ctx,
    );
}
</code></pre>



</details>

<a name="0xc8_treasury_create_vault_internal"></a>

## Function `create_vault_internal`

creat vault for ordered A & B


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _tick_spacing: u32, _spacing_times: u32, _position_number: u32, _initialize_price: u128, _base_point: u64, _max_counter_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _supply: Supply&lt;StableCoinType&gt;,
    _tick_spacing: u32,
    _spacing_times: u32,
    _position_number: u32,
    _initialize_price: u128,
    _base_point: u64,
    _max_counter_times: u32,
    _ts: u64,
    _ctx: &<b>mut</b> TxContext
): String {
    <b>let</b> vault_key = <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;();
    <b>assert</b>!(!<a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>&lt;String&gt;(&_treasury.id, vault_key), <a href="treasury.md#0xc8_treasury_ERR_POOL_HAS_REGISTERED">ERR_POOL_HAS_REGISTERED</a>);

    // index increased
    _treasury.index = _treasury.index + 1;
    <b>let</b> new_vault = <a href="vault.md#0xc8_vault_create_vault">vault::create_vault</a>&lt;StableCoinType&gt;(
        _treasury.index,
        _tick_spacing,
        _spacing_times,
        _position_number,
        _initialize_price,
        _base_point,
        _max_counter_times,
        _ts,
        _ctx,
    );
    <b>let</b> vault_id = <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(&new_vault);

    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(
        &<b>mut</b> _treasury.id,
        vault_key,
        new_vault,
    );
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;String, Supply&lt;StableCoinType&gt;&gt;(&<b>mut</b> _treasury.supplies, vault_key, _supply);

    event::create_vault(
        vault_id,
        vault_key,
        into_string(get&lt;StableCoinType&gt;()),
        into_string(get&lt;BFC&gt;()),
        _tick_spacing,
        _spacing_times,
        _treasury.index,
    );
    vault_key
}
</code></pre>



</details>

<a name="0xc8_treasury_mint"></a>

## Function `mint`

======= Swap
Mint swap bfc to stablecoin


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_mint">mint</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_bfc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _amount: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_mint">mint</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_bfc: Coin&lt;BFC&gt;,
    _amount: u64,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> balance_a = <a href="treasury.md#0xc8_treasury_mint_internal">mint_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _coin_bfc,
        _amount,
        _ctx,
    );
    <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>(balance_a, _ctx);
}
</code></pre>



</details>

<a name="0xc8_treasury_mint_internal"></a>

## Function `mint_internal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_mint_internal">mint_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_bfc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _amount: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_mint_internal">mint_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_bfc: Coin&lt;BFC&gt;,
    _amount: u64,
    _ctx: &<b>mut</b> TxContext,
): Balance&lt;StableCoinType&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>&lt;BFC&gt;(&_coin_bfc) &gt; 0, <a href="treasury.md#0xc8_treasury_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>let</b> (balance_a, balance_b) = <a href="treasury.md#0xc8_treasury_swap_internal">swap_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        <b>false</b>,
        <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;StableCoinType&gt;(_ctx),
        _coin_bfc,
        _amount,
        <b>true</b>,
        _ctx,
    );
    <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>(balance_b, _ctx);
    balance_a
}
</code></pre>



</details>

<a name="0xc8_treasury_redeem"></a>

## Function `redeem`

Burn swap stablecoin to bfc


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem">redeem</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, _amount: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem">redeem</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_sc: Coin&lt;StableCoinType&gt;,
    _amount: u64,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>&lt;StableCoinType&gt;(&_coin_sc) &gt; 0, <a href="treasury.md#0xc8_treasury_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>let</b> balance_b = <a href="treasury.md#0xc8_treasury_redeem_internal">redeem_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _coin_sc,
        _amount,
        _ctx,
    );
    <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>(balance_b, _ctx);
}
</code></pre>



</details>

<a name="0xc8_treasury_redeem_internal"></a>

## Function `redeem_internal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem_internal">redeem_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, _amount: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem_internal">redeem_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_sc: Coin&lt;StableCoinType&gt;,
    _amount: u64,
    _ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>&lt;StableCoinType&gt;(&_coin_sc) &gt; 0, <a href="treasury.md#0xc8_treasury_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>let</b> (balance_a, balance_b) = <a href="treasury.md#0xc8_treasury_swap_internal">swap_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        <b>true</b>,
        _coin_sc,
        <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_zero">coin::zero</a>&lt;BFC&gt;(_ctx),
        _amount,
        <b>true</b>,
        _ctx,
    );
    <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>(balance_a, _ctx);
    balance_b
}
</code></pre>



</details>

<a name="0xc8_treasury_calculate_swap_result"></a>

## Function `calculate_swap_result`

Burn swap stablecoin to bfc


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_calculate_swap_result">calculate_swap_result</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _a2b: bool, _amount: u64): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_calculate_swap_result">calculate_swap_result</a>&lt;StableCoinType&gt;(
    _treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _a2b: bool,
    _amount: u64
): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
{
    <b>let</b> sc_vault = <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury, <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;());
    <a href="vault.md#0xc8_vault_calculate_swap_result">vault::calculate_swap_result</a>(sc_vault, _a2b, <b>true</b>, _amount)
}
</code></pre>



</details>

<a name="0xc8_treasury_transfer_or_delete"></a>

## Function `transfer_or_delete`



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>&lt;CoinType&gt;(_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;CoinType&gt;, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>&lt;CoinType&gt;(
    _balance: Balance&lt;CoinType&gt;,
    _ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_balance) &gt; 0) {
        <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(_balance, _ctx), <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(_ctx));
    } <b>else</b> {
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(_balance);
    }
}
</code></pre>



</details>

<a name="0xc8_treasury_swap_internal"></a>

## Function `swap_internal`

Internal swap


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_swap_internal">swap_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _a2b: bool, _coin_a: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, _coin_b: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _amount: u64, _by_amount_in: bool, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_swap_internal">swap_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _a2b: bool, // <b>true</b> a-&gt;b , <b>false</b> b-&gt;a
    _coin_a: Coin&lt;StableCoinType&gt;,
    _coin_b: Coin&lt;BFC&gt;,
    _amount: u64,
    _by_amount_in: bool,
    _ctx: &<b>mut</b> TxContext,
): (Balance&lt;StableCoinType&gt;, Balance&lt;BFC&gt;) {
    <b>let</b> vault_key = <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;();
    <b>let</b> mut_vault = <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(_treasury, vault_key);
    <b>let</b> sqrt_price_limit = <a href="tick_math.md#0xc8_tick_math_get_default_sqrt_price_limit">tick_math::get_default_sqrt_price_limit</a>(_a2b);
    <a href="vault.md#0xc8_vault_swap">vault::swap</a>&lt;StableCoinType&gt;(
        mut_vault,
        _coin_a,
        _coin_b,
        _a2b,
        _by_amount_in,
        _amount,
        0, // ? unuse
        sqrt_price_limit,
        _ctx
    )
}
</code></pre>



</details>

<a name="0xc8_treasury_next_epoch_bfc_required"></a>

## Function `next_epoch_bfc_required`

Rebalance


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_next_epoch_bfc_required">next_epoch_bfc_required</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_next_epoch_bfc_required">next_epoch_bfc_required</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>): u64 {
    <b>let</b> total = 0;
    <b>let</b> times_per_day = (3600 * 24 / _treasury.time_interval <b>as</b> u64);

    // USD <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a> required
    <b>let</b> usd_vault_key = <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;BUSD&gt;();
    <b>if</b> (<a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(&_treasury.id, usd_vault_key)) {
        <b>let</b> usd_v = <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;BUSD&gt;(_treasury, usd_vault_key);
        <b>let</b> bfc_required_per_time = <a href="vault.md#0xc8_vault_bfc_required">vault::bfc_required</a>(usd_v);
        total = total + bfc_required_per_time * times_per_day;
    };

    total - <a href="treasury.md#0xc8_treasury_get_balance">get_balance</a>(_treasury)
}
</code></pre>



</details>

<a name="0xc8_treasury_deposit"></a>

## Function `deposit`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_deposit">deposit</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_bfc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_deposit">deposit</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>, _coin_bfc: Coin&lt;BFC&gt;) {
    <b>let</b> min_amount = <a href="treasury.md#0xc8_treasury_next_epoch_bfc_required">next_epoch_bfc_required</a>(_treasury);
    <b>let</b> input = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(_coin_bfc);
    <b>let</b> input_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&input);
    <b>assert</b>!(input_amount &gt;= min_amount, <a href="treasury.md#0xc8_treasury_ERR_INSUFFICIENT">ERR_INSUFFICIENT</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> _treasury.bfc_balance, input);
    event::deposit(input_amount);

    <b>if</b> (!_treasury.init) {
        _treasury.init = <b>true</b>
    }
}
</code></pre>



</details>

<a name="0xc8_treasury_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance">rebalance</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance">rebalance</a>(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    _ctx: &<b>mut</b> TxContext,
) {
    // check init
    <b>if</b> (!_treasury.init) {
        <b>return</b>
    };

    // check time_interval
    <b>let</b> current_ts = <a href="../../../.././build/Sui/docs/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>) / 1000;
    <b>if</b> ((current_ts - _treasury.updated_at) &lt; (_treasury.time_interval <b>as</b> u64)) {
        <b>return</b>
    };

    // <b>update</b> updated_at
    _treasury.updated_at = current_ts;
    <b>let</b> usd_mut_v = <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;String, Vault&lt;BUSD&gt;&gt;(
        &<b>mut</b> _treasury.id,
        <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;BUSD&gt;()
    );
    <a href="vault.md#0xc8_vault_update_state">vault::update_state</a>(usd_mut_v);

    <a href="vault.md#0xc8_vault_rebalance">vault::rebalance</a>(
        usd_mut_v,
        &<b>mut</b> _treasury.bfc_balance,
        <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;String, Supply&lt;BUSD&gt;&gt;(&<b>mut</b> _treasury.supplies, <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;BUSD&gt;()),
        _ctx
    );
}
</code></pre>



</details>

<a name="0xc8_treasury_rebalance_first_init"></a>

## Function `rebalance_first_init`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance_first_init">rebalance_first_init</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance_first_init">rebalance_first_init</a>(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _ctx: &<b>mut</b> TxContext
)
{
    <b>let</b> usd_mut_v = <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;String, Vault&lt;BUSD&gt;&gt;(
        &<b>mut</b> _treasury.id,
        <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;BUSD&gt;()
    );
    // first rebalance just place liquidity not change <a href="vault.md#0xc8_vault">vault</a> state
    <a href="vault.md#0xc8_vault_rebalance">vault::rebalance</a>(
        usd_mut_v,
        &<b>mut</b> _treasury.bfc_balance,
        <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;String, Supply&lt;BUSD&gt;&gt;(&<b>mut</b> _treasury.supplies, <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;BUSD&gt;()),
        _ctx
    );
}
</code></pre>



</details>

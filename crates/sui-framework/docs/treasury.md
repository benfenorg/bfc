
<a name="0xc8_treasury"></a>

# Module `0xc8::treasury`



-  [Resource `Treasury`](#0xc8_treasury_Treasury)
-  [Constants](#@Constants_0)
-  [Function `create_treasury`](#0xc8_treasury_create_treasury)
-  [Function `index`](#0xc8_treasury_index)
-  [Function `check_vault`](#0xc8_treasury_check_vault)
-  [Function `borrow_vault`](#0xc8_treasury_borrow_vault)
-  [Function `borrow_mut_vault`](#0xc8_treasury_borrow_mut_vault)
-  [Function `create_vault`](#0xc8_treasury_create_vault)
-  [Function `init_vault_with_positions`](#0xc8_treasury_init_vault_with_positions)
-  [Function `create_vault_internal`](#0xc8_treasury_create_vault_internal)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="">0x1::type_name</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field">0x2::dynamic_object_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="event.md#0xc8_event">0xc8::event</a>;
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
<code>obc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;</code>
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
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_treasury_ERR_INVALID_LIMIT"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_INVALID_LIMIT">ERR_INVALID_LIMIT</a>: u64 = 102;
</code></pre>



<a name="0xc8_treasury_ERR_INVALID_VECTOR_LENGTH"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_INVALID_VECTOR_LENGTH">ERR_INVALID_VECTOR_LENGTH</a>: u64 = 103;
</code></pre>



<a name="0xc8_treasury_ERR_MUST_BE_ORDER"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_MUST_BE_ORDER">ERR_MUST_BE_ORDER</a>: u64 = 104;
</code></pre>



<a name="0xc8_treasury_ERR_POOL_HAS_REGISTERED"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_POOL_HAS_REGISTERED">ERR_POOL_HAS_REGISTERED</a>: u64 = 101;
</code></pre>



<a name="0xc8_treasury_ERR_POOL_NOT_EXISTS"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_POOL_NOT_EXISTS">ERR_POOL_NOT_EXISTS</a>: u64 = 105;
</code></pre>



<a name="0xc8_treasury_ERR_THE_SAME_COIN"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_THE_SAME_COIN">ERR_THE_SAME_COIN</a>: u64 = 100;
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
        obc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;OBC&gt;(),
        supplies: <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_new">bag::new</a>(ctx),
        index: 0,
        time_interval: time_interval,
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

<a name="0xc8_treasury_check_vault"></a>

## Function `check_vault`



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>, _vault_key: String) {
    <b>assert</b>!(
        <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_exists_">dynamic_object_field::exists_</a>(
            &_treasury.id,
            _vault_key
        ),
        <a href="treasury.md#0xc8_treasury_ERR_POOL_NOT_EXISTS">ERR_POOL_NOT_EXISTS</a>
    );
}
</code></pre>



</details>

<a name="0xc8_treasury_borrow_vault"></a>

## Function `borrow_vault`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="_String">ascii::String</a>): &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _vault_key: String
): &Vault&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>&lt;StableCoinType&gt;(_treasury, _vault_key);
    <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_borrow">dynamic_object_field::borrow</a>&lt;String, Vault&lt;StableCoinType&gt;&gt;(&_treasury.id, _vault_key)
}
</code></pre>



</details>

<a name="0xc8_treasury_borrow_mut_vault"></a>

## Function `borrow_mut_vault`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="_String">ascii::String</a>): &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _vault_key: String
): &<b>mut</b> Vault&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>&lt;StableCoinType&gt;(_treasury, _vault_key);
    <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_borrow_mut">dynamic_object_field::borrow_mut</a>&lt;String, Vault&lt;StableCoinType&gt;&gt;(&<b>mut</b> _treasury.id, _vault_key)
}
</code></pre>



</details>

<a name="0xc8_treasury_create_vault"></a>

## Function `create_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault">create_vault</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _position_number: u32, _tick_spacing: u32, _initialize_price: u128, _base_point: u64, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault">create_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _supply: Supply&lt;StableCoinType&gt;,
    _position_number: u32,
    _tick_spacing: u32,
    _initialize_price: u128,
    _base_point: u64,
    _ts: u64,
    _ctx: &<b>mut</b> TxContext
) {
    <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _supply,
        _tick_spacing,
        _position_number,
        _initialize_price,
        _base_point,
        _ts,
        _ctx,
    );
}
</code></pre>



</details>

<a name="0xc8_treasury_init_vault_with_positions"></a>

## Function `init_vault_with_positions`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_init_vault_with_positions">init_vault_with_positions</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _initialize_price: u128, _base_point: u64, _position_number: u32, _tick_spacing: u32, _spacing_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
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
    _ts: u64,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> vault_key = <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _supply,
        _tick_spacing,
        _position_number,
        _initialize_price,
        _base_point,
        _ts,
        _ctx,
    );
    <a href="vault.md#0xc8_vault_init_positions">vault::init_positions</a>&lt;StableCoinType&gt;(
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


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _tick_spacing: u32, _position_number: u32, _initialize_price: u128, _base_point: u64, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _supply: Supply&lt;StableCoinType&gt;,
    _tick_spacing: u32,
    _position_number: u32,
    _initialize_price: u128,
    _base_point: u64,
    _ts: u64,
    _ctx: &<b>mut</b> TxContext
): String {
    <b>let</b> vault_key = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;StableCoinType&gt;());
    <b>assert</b>!(!<a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_exists_">dynamic_object_field::exists_</a>&lt;String&gt;(&_treasury.id, vault_key), <a href="treasury.md#0xc8_treasury_ERR_POOL_HAS_REGISTERED">ERR_POOL_HAS_REGISTERED</a>);

    // index increased
    _treasury.index = _treasury.index + 1;
    <b>let</b> new_vault = <a href="vault.md#0xc8_vault_create_vault">vault::create_vault</a>&lt;StableCoinType&gt;(
        _treasury.index,
        _tick_spacing,
        _position_number,
        _initialize_price,
        _base_point,
        _ts,
        _ctx,
    );
    <b>let</b> vault_id = <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(&new_vault);

    <a href="../../../.././build/Sui/docs/dynamic_object_field.md#0x2_dynamic_object_field_add">dynamic_object_field::add</a>(
        &<b>mut</b> _treasury.id,
        vault_key,
        new_vault,
    );
    <a href="../../../.././build/Sui/docs/bag.md#0x2_bag_add">bag::add</a>&lt;String, Supply&lt;StableCoinType&gt;&gt;(&<b>mut</b> _treasury.supplies, vault_key, _supply);

    event::create_vault(
        vault_id,
        vault_key,
        into_string(get&lt;StableCoinType&gt;()),
        into_string(get&lt;OBC&gt;()),
        _tick_spacing,
        _treasury.index,
    );
    vault_key
}
</code></pre>



</details>

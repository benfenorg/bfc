---
title: Module `0xc8::treasury`
---



-  [Resource `TreasuryPauseCap`](#0xc8_treasury_TreasuryPauseCap)
-  [Resource `Treasury`](#0xc8_treasury_Treasury)
-  [Constants](#@Constants_0)
-  [Function `create_treasury`](#0xc8_treasury_create_treasury)
-  [Function `create_treasury_pause_cap`](#0xc8_treasury_create_treasury_pause_cap)
-  [Function `index`](#0xc8_treasury_index)
-  [Function `get_balance`](#0xc8_treasury_get_balance)
-  [Function `check_vault`](#0xc8_treasury_check_vault)
-  [Function `get_vault_key`](#0xc8_treasury_get_vault_key)
-  [Function `borrow_vault`](#0xc8_treasury_borrow_vault)
-  [Function `borrow_mut_vault`](#0xc8_treasury_borrow_mut_vault)
-  [Function `vault_info`](#0xc8_treasury_vault_info)
-  [Function `vault_set_pause`](#0xc8_treasury_vault_set_pause)
-  [Function `fetch_ticks`](#0xc8_treasury_fetch_ticks)
-  [Function `fetch_positions`](#0xc8_treasury_fetch_positions)
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
-  [Function `deposit`](#0xc8_treasury_deposit)
-  [Function `bfc_required`](#0xc8_treasury_bfc_required)
-  [Function `rebalance`](#0xc8_treasury_rebalance)
-  [Function `rebalance_internal`](#0xc8_treasury_rebalance_internal)
-  [Function `get_exchange_rates`](#0xc8_treasury_get_exchange_rates)
-  [Function `get_total_supply`](#0xc8_treasury_get_total_supply)
-  [Function `one_coin_rebalance_internal`](#0xc8_treasury_one_coin_rebalance_internal)
-  [Function `one_coin_bfc_required`](#0xc8_treasury_one_coin_bfc_required)
-  [Function `one_coin_exchange_rate`](#0xc8_treasury_one_coin_exchange_rate)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../sui-framework/bag.md#0x2_bag">0x2::bag</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="bars.md#0xc8_bars">0xc8::bars</a>;
<b>use</b> <a href="baud.md#0xc8_baud">0xc8::baud</a>;
<b>use</b> <a href="bbrl.md#0xc8_bbrl">0xc8::bbrl</a>;
<b>use</b> <a href="bcad.md#0xc8_bcad">0xc8::bcad</a>;
<b>use</b> <a href="beur.md#0xc8_beur">0xc8::beur</a>;
<b>use</b> <a href="bgbp.md#0xc8_bgbp">0xc8::bgbp</a>;
<b>use</b> <a href="bidr.md#0xc8_bidr">0xc8::bidr</a>;
<b>use</b> <a href="binr.md#0xc8_binr">0xc8::binr</a>;
<b>use</b> <a href="bjpy.md#0xc8_bjpy">0xc8::bjpy</a>;
<b>use</b> <a href="bkrw.md#0xc8_bkrw">0xc8::bkrw</a>;
<b>use</b> <a href="bmxn.md#0xc8_bmxn">0xc8::bmxn</a>;
<b>use</b> <a href="brub.md#0xc8_brub">0xc8::brub</a>;
<b>use</b> <a href="bsar.md#0xc8_bsar">0xc8::bsar</a>;
<b>use</b> <a href="btry.md#0xc8_btry">0xc8::btry</a>;
<b>use</b> <a href="busd.md#0xc8_busd">0xc8::busd</a>;
<b>use</b> <a href="bzar.md#0xc8_bzar">0xc8::bzar</a>;
<b>use</b> <a href="event.md#0xc8_event">0xc8::event</a>;
<b>use</b> <a href="i32.md#0xc8_i32">0xc8::i32</a>;
<b>use</b> <a href="mgg.md#0xc8_mgg">0xc8::mgg</a>;
<b>use</b> <a href="position.md#0xc8_position">0xc8::position</a>;
<b>use</b> <a href="tick.md#0xc8_tick">0xc8::tick</a>;
<b>use</b> <a href="tick_math.md#0xc8_tick_math">0xc8::tick_math</a>;
<b>use</b> <a href="vault.md#0xc8_vault">0xc8::vault</a>;
</code></pre>



<a name="0xc8_treasury_TreasuryPauseCap"></a>

## Resource `TreasuryPauseCap`



<pre><code><b>struct</b> <a href="treasury.md#0xc8_treasury_TreasuryPauseCap">TreasuryPauseCap</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_treasury_Treasury"></a>

## Resource `Treasury`



<pre><code><b>struct</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a> <b>has</b> store, key
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
<code>bfc_balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>supplies: <a href="../sui-framework/bag.md#0x2_bag_Bag">bag::Bag</a></code>
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
<dt>
<code>total_bfc_supply: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_treasury_ERR_DEADLINE_EXCEED"></a>



<pre><code><b>const</b> <a href="treasury.md#0xc8_treasury_ERR_DEADLINE_EXCEED">ERR_DEADLINE_EXCEED</a>: u64 = 105;
</code></pre>



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



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_treasury">create_treasury</a>(time_interval: u32, total_bfc_supply: u64, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_treasury">create_treasury</a>(time_interval: u32, total_bfc_supply: u64, ctx: &<b>mut</b> TxContext): <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a> {
    <b>let</b> <a href="treasury.md#0xc8_treasury">treasury</a> = <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a> {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        bfc_balance: <a href="../sui-framework/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;(),
        supplies: <a href="../sui-framework/bag.md#0x2_bag_new">bag::new</a>(ctx),
        index: 0,
        time_interval,
        updated_at: 0,
        init: <b>false</b>,
        total_bfc_supply: total_bfc_supply,
    };
    <b>let</b> treasury_id = <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&<a href="treasury.md#0xc8_treasury">treasury</a>);
    event::init_treasury(treasury_id);
    <a href="treasury.md#0xc8_treasury">treasury</a>
}
</code></pre>



</details>

<a name="0xc8_treasury_create_treasury_pause_cap"></a>

## Function `create_treasury_pause_cap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_treasury_pause_cap">create_treasury_pause_cap</a>(admin: <b>address</b>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_treasury_pause_cap">create_treasury_pause_cap</a>(admin: <b>address</b>, ctx: &<b>mut</b> TxContext) {
    <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(<a href="treasury.md#0xc8_treasury_TreasuryPauseCap">TreasuryPauseCap</a> { id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx) }, admin);
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
    <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&_treasury.bfc_balance)
}
</code></pre>



</details>

<a name="0xc8_treasury_check_vault"></a>

## Function `check_vault`



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>, _vault_key: String) {
    <b>assert</b>!(
        <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(
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



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;(): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;(): String {
    <a href="../move-stdlib/type_name.md#0x1_type_name_into_string">type_name::into_string</a>(<a href="../move-stdlib/type_name.md#0x1_type_name_get">type_name::get</a>&lt;StableCoinType&gt;())
}
</code></pre>



</details>

<a name="0xc8_treasury_borrow_vault"></a>

## Function `borrow_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _vault_key: String
): &Vault&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury, _vault_key);
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow">dynamic_field::borrow</a>&lt;String, Vault&lt;StableCoinType&gt;&gt;(&_treasury.id, _vault_key)
}
</code></pre>



</details>

<a name="0xc8_treasury_borrow_mut_vault"></a>

## Function `borrow_mut_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _vault_key: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _vault_key: String
): &<b>mut</b> Vault&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_check_vault">check_vault</a>(_treasury, _vault_key);
    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;String, Vault&lt;StableCoinType&gt;&gt;(&<b>mut</b> _treasury.id, _vault_key)
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

<a name="0xc8_treasury_vault_set_pause"></a>

## Function `vault_set_pause`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(_: &<a href="treasury.md#0xc8_treasury_TreasuryPauseCap">treasury::TreasuryPauseCap</a>, _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _pause: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_vault_set_pause">vault_set_pause</a>&lt;StableCoinType&gt;(_: &<a href="treasury.md#0xc8_treasury_TreasuryPauseCap">TreasuryPauseCap</a>, _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>, _pause: bool) {
    <a href="vault.md#0xc8_vault_set_pause">vault::set_pause</a>(
        <a href="treasury.md#0xc8_treasury_borrow_mut_vault">borrow_mut_vault</a>&lt;StableCoinType&gt;(_treasury, <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;()),
        _pause,
    );
}
</code></pre>



</details>

<a name="0xc8_treasury_fetch_ticks"></a>

## Function `fetch_ticks`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_fetch_ticks">fetch_ticks</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_fetch_ticks">fetch_ticks</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Tick&gt; {
    <a href="vault.md#0xc8_vault_fetch_ticks">vault::fetch_ticks</a>(
        <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury, <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;())
    )
}
</code></pre>



</details>

<a name="0xc8_treasury_fetch_positions"></a>

## Function `fetch_positions`



<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_fetch_positions">fetch_positions</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="position.md#0xc8_position_Position">position::Position</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="treasury.md#0xc8_treasury_fetch_positions">fetch_positions</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;Position&gt; {
    <a href="vault.md#0xc8_vault_fetch_positions">vault::fetch_positions</a>(
        <a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury, <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;())
    )
}
</code></pre>



</details>

<a name="0xc8_treasury_create_vault"></a>

## Function `create_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault">create_vault</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _position_number: u32, _tick_spacing: u32, _spacing_times: u32, _initialize_price: u128, _base_point: u64, _max_counter_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault">create_vault</a>&lt;StableCoinType&gt;(
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



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_init_vault_with_positions">init_vault_with_positions</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _initialize_price: u128, _base_point: u64, _position_number: u32, _tick_spacing: u32, _spacing_times: u32, _max_counter_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_init_vault_with_positions">init_vault_with_positions</a>&lt;StableCoinType&gt;(
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


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_create_vault_internal">create_vault_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _supply: <a href="../sui-framework/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _tick_spacing: u32, _spacing_times: u32, _position_number: u32, _initialize_price: u128, _base_point: u64, _max_counter_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
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
    <b>assert</b>!(!<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>&lt;String&gt;(&_treasury.id, vault_key), <a href="treasury.md#0xc8_treasury_ERR_POOL_HAS_REGISTERED">ERR_POOL_HAS_REGISTERED</a>);

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

    <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(
        &<b>mut</b> _treasury.id,
        vault_key,
        new_vault,
    );
    <a href="../sui-framework/bag.md#0x2_bag_add">bag::add</a>&lt;String, Supply&lt;StableCoinType&gt;&gt;(&<b>mut</b> _treasury.supplies, vault_key, _supply);
    vault_key
}
</code></pre>



</details>

<a name="0xc8_treasury_mint"></a>

## Function `mint`

======= Swap
Mint swap bfc to stablecoin


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_mint">mint</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_bfc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _clock: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, _amount: u64, _min_amount: u64, _deadline: u64, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_mint">mint</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_bfc: Coin&lt;BFC&gt;,
    _clock: &Clock,
    _amount: u64,
    _min_amount: u64,
    _deadline: u64,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> balance_a = <a href="treasury.md#0xc8_treasury_mint_internal">mint_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _coin_bfc,
        _amount,
        _ctx,
    );
    <b>assert</b>!(<a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&balance_a) &gt;= _min_amount, <a href="treasury.md#0xc8_treasury_ERR_INSUFFICIENT">ERR_INSUFFICIENT</a>);
    <b>assert</b>!(<a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(_clock) &lt;= _deadline, <a href="treasury.md#0xc8_treasury_ERR_DEADLINE_EXCEED">ERR_DEADLINE_EXCEED</a>);
    <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>(balance_a, _ctx);
}
</code></pre>



</details>

<a name="0xc8_treasury_mint_internal"></a>

## Function `mint_internal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_mint_internal">mint_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_bfc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _amount: u64, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_mint_internal">mint_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_bfc: Coin&lt;BFC&gt;,
    _amount: u64,
    _ctx: &<b>mut</b> TxContext,
): Balance&lt;StableCoinType&gt; {
    <b>assert</b>!(<a href="../sui-framework/coin.md#0x2_coin_value">coin::value</a>&lt;BFC&gt;(&_coin_bfc) &gt; 0, <a href="treasury.md#0xc8_treasury_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>let</b> (balance_a, balance_b) = <a href="treasury.md#0xc8_treasury_swap_internal">swap_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        <b>false</b>,
        <a href="../sui-framework/coin.md#0x2_coin_zero">coin::zero</a>&lt;StableCoinType&gt;(_ctx),
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


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem">redeem</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_sc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, _clock: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, _amount: u64, _min_amount: u64, _deadline: u64, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem">redeem</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_sc: Coin&lt;StableCoinType&gt;,
    _clock: &Clock,
    _amount: u64,
    _min_amount: u64,
    _deadline: u64,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../sui-framework/coin.md#0x2_coin_value">coin::value</a>&lt;StableCoinType&gt;(&_coin_sc) &gt; 0, <a href="treasury.md#0xc8_treasury_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>let</b> balance_b = <a href="treasury.md#0xc8_treasury_redeem_internal">redeem_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        _coin_sc,
        _amount,
        _ctx,
    );
    <b>assert</b>!(<a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&balance_b) &gt;= _min_amount, <a href="treasury.md#0xc8_treasury_ERR_INSUFFICIENT">ERR_INSUFFICIENT</a>);
    <b>assert</b>!(<a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(_clock) &lt;= _deadline, <a href="treasury.md#0xc8_treasury_ERR_DEADLINE_EXCEED">ERR_DEADLINE_EXCEED</a>);
    <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>(balance_b, _ctx);
}
</code></pre>



</details>

<a name="0xc8_treasury_redeem_internal"></a>

## Function `redeem_internal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem_internal">redeem_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_sc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, _amount: u64, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_redeem_internal">redeem_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _coin_sc: Coin&lt;StableCoinType&gt;,
    _amount: u64,
    _ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <b>assert</b>!(<a href="../sui-framework/coin.md#0x2_coin_value">coin::value</a>&lt;StableCoinType&gt;(&_coin_sc) &gt; 0, <a href="treasury.md#0xc8_treasury_ERR_ZERO_AMOUNT">ERR_ZERO_AMOUNT</a>);
    <b>let</b> (balance_a, balance_b) = <a href="treasury.md#0xc8_treasury_swap_internal">swap_internal</a>&lt;StableCoinType&gt;(
        _treasury,
        <b>true</b>,
        _coin_sc,
        <a href="../sui-framework/coin.md#0x2_coin_zero">coin::zero</a>&lt;BFC&gt;(_ctx),
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



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>&lt;CoinType&gt;(_balance: <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;CoinType&gt;, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_transfer_or_delete">transfer_or_delete</a>&lt;CoinType&gt;(
    _balance: Balance&lt;CoinType&gt;,
    _ctx: &<b>mut</b> TxContext
) {
    <b>if</b> (<a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&_balance) &gt; 0) {
        <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(_balance, _ctx), <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(_ctx));
    } <b>else</b> {
        <a href="../sui-framework/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(_balance);
    }
}
</code></pre>



</details>

<a name="0xc8_treasury_swap_internal"></a>

## Function `swap_internal`

Internal swap


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_swap_internal">swap_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _a2b: bool, _coin_a: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, _coin_b: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _amount: u64, _by_amount_in: bool, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, <a href="../sui-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
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

<a name="0xc8_treasury_deposit"></a>

## Function `deposit`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_deposit">deposit</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _coin_bfc: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_deposit">deposit</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>, _coin_bfc: Coin&lt;BFC&gt;) {
    <b>let</b> min_amount = <a href="treasury.md#0xc8_treasury_bfc_required">bfc_required</a>(_treasury);
    <b>let</b> input = <a href="../sui-framework/coin.md#0x2_coin_into_balance">coin::into_balance</a>(_coin_bfc);
    <b>let</b> input_amount = <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&input);
    <b>assert</b>!(input_amount &gt;= min_amount, <a href="treasury.md#0xc8_treasury_ERR_INSUFFICIENT">ERR_INSUFFICIENT</a>);
    <a href="../sui-framework/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> _treasury.bfc_balance, input);

    <b>if</b> (!_treasury.init) {
        _treasury.init = <b>true</b>
    }
}
</code></pre>



</details>

<a name="0xc8_treasury_bfc_required"></a>

## Function `bfc_required`

Rebalance


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_bfc_required">bfc_required</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_bfc_required">bfc_required</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>): u64 {
    <b>let</b> treasury_total_bfc_supply = _treasury.total_bfc_supply;

    <b>let</b> total = <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BUSD&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;MGG&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BJPY&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BAUD&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BKRW&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BBRL&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BCAD&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BEUR&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BGBP&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BIDR&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BINR&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BRUB&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BSAR&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BTRY&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BZAR&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BMXN&gt;(_treasury, treasury_total_bfc_supply) +
        <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;BARS&gt;(_treasury, treasury_total_bfc_supply);

    <b>let</b> get_treasury_balance = <a href="treasury.md#0xc8_treasury_get_balance">get_balance</a>(_treasury);
    <b>if</b> (total &gt; get_treasury_balance) {
        total - get_treasury_balance
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="0xc8_treasury_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance">rebalance</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _pool_balance: u64, _update: bool, _clock: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance">rebalance</a>(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _pool_balance: u64,
    _update: bool,
    _clock: &Clock,
    _ctx: &<b>mut</b> TxContext,
) {
    // check init
    <b>if</b> (!_treasury.init) {
        <b>return</b>
    };

    <b>let</b> current_ts = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(_clock) / 1000;

    <b>if</b> ((current_ts - _treasury.updated_at) &lt; (_treasury.time_interval <b>as</b> u64)) {
        <b>return</b>
    };

    // <b>update</b> updated_at
    _treasury.updated_at = current_ts;
    <b>let</b> bfc_in_vault = <a href="treasury.md#0xc8_treasury_rebalance_internal">rebalance_internal</a>(_treasury, _update, _ctx);
    _treasury.total_bfc_supply = _pool_balance + bfc_in_vault + <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&_treasury.bfc_balance);
}
</code></pre>



</details>

<a name="0xc8_treasury_rebalance_internal"></a>

## Function `rebalance_internal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance_internal">rebalance_internal</a>(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _update: bool, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_rebalance_internal">rebalance_internal</a>(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _update: bool,
    _ctx: &<b>mut</b> TxContext
): u64 {
    <b>let</b> <b>mut</b> bfc_in_vault = 0;
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BUSD&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;MGG&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BJPY&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BKRW&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BAUD&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BARS&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BBRL&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BCAD&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BEUR&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BGBP&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BIDR&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BINR&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BRUB&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BSAR&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BTRY&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BZAR&gt;(_treasury, _update, _ctx);
    bfc_in_vault = bfc_in_vault + <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;BMXN&gt;(_treasury, _update, _ctx);
    bfc_in_vault
}
</code></pre>



</details>

<a name="0xc8_treasury_get_exchange_rates"></a>

## Function `get_exchange_rates`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_get_exchange_rates">get_exchange_rates</a>(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_get_exchange_rates">get_exchange_rates</a>(
    _treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
): VecMap&lt;String, u64&gt; {
    <b>let</b> <b>mut</b> rate_map = <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>&lt;String, u64&gt;();
    <b>let</b> amount = 1_000_000_000;

    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BUSD&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;MGG&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BJPY&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BKRW&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BAUD&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BARS&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BBRL&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BCAD&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BEUR&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BGBP&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BIDR&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BINR&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BRUB&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BSAR&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BTRY&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BZAR&gt;(_treasury, &<b>mut</b> rate_map, amount);
    <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;BMXN&gt;(_treasury, &<b>mut</b> rate_map, amount);

    rate_map
}
</code></pre>



</details>

<a name="0xc8_treasury_get_total_supply"></a>

## Function `get_total_supply`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="treasury.md#0xc8_treasury_get_total_supply">get_total_supply</a>&lt;StableCoinType&gt;(_self: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="treasury.md#0xc8_treasury_get_total_supply">get_total_supply</a>&lt;StableCoinType&gt;(
    _self: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>
): u64
{
    <b>let</b> key = <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;();
    <b>if</b> (!<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(&_self.id, key)) {
        <b>return</b> 0
    };
    <b>let</b> supply = <a href="../sui-framework/bag.md#0x2_bag_borrow">bag::borrow</a>&lt;String, Supply&lt;StableCoinType&gt;&gt;(&_self.supplies, key);
    <a href="../sui-framework/balance.md#0x2_balance_supply_value">balance::supply_value</a>(supply)
}
</code></pre>



</details>

<a name="0xc8_treasury_one_coin_rebalance_internal"></a>

## Function `one_coin_rebalance_internal`



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;StableCoinType&gt;(_treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _update: bool, _ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_one_coin_rebalance_internal">one_coin_rebalance_internal</a>&lt;StableCoinType&gt;(
    _treasury: &<b>mut</b> <a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _update: bool,
    _ctx: &<b>mut</b> TxContext
): u64 {
    <b>let</b> key = <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;();
    <b>if</b> (!<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(&_treasury.id, key)) {
        <b>return</b> 0
    };
    <b>let</b> mut_v = <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>&lt;String, Vault&lt;StableCoinType&gt;&gt;(
        &<b>mut</b> _treasury.id,
        key,
    );
    <b>if</b> (_update) {
        <a href="vault.md#0xc8_vault_update_state">vault::update_state</a>(mut_v);
    };

    // first rebalance just place liquidity not change <a href="vault.md#0xc8_vault">vault</a> state
    <a href="vault.md#0xc8_vault_rebalance">vault::rebalance</a>(
        mut_v,
        &<b>mut</b> _treasury.bfc_balance,
        <a href="../sui-framework/bag.md#0x2_bag_borrow_mut">bag::borrow_mut</a>&lt;String, Supply&lt;StableCoinType&gt;&gt;(&<b>mut</b> _treasury.supplies, key),
        _treasury.total_bfc_supply,
        _ctx
    )
}
</code></pre>



</details>

<a name="0xc8_treasury_one_coin_bfc_required"></a>

## Function `one_coin_bfc_required`



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _treasury_total_bfc_supply: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_one_coin_bfc_required">one_coin_bfc_required</a>&lt;StableCoinType&gt;(
    _treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _treasury_total_bfc_supply: u64
): u64 {
    <b>let</b> key = <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;();
    <b>if</b> (<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(&_treasury.id, key)) {
        <a href="vault.md#0xc8_vault_bfc_required">vault::bfc_required</a>(<a href="treasury.md#0xc8_treasury_borrow_vault">borrow_vault</a>&lt;StableCoinType&gt;(_treasury, key), _treasury_total_bfc_supply)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="0xc8_treasury_one_coin_exchange_rate"></a>

## Function `one_coin_exchange_rate`



<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;StableCoinType&gt;(_treasury: &<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, _rate_map: &<b>mut</b> <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, u64&gt;, _amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="treasury.md#0xc8_treasury_one_coin_exchange_rate">one_coin_exchange_rate</a>&lt;StableCoinType&gt;(
    _treasury: &<a href="treasury.md#0xc8_treasury_Treasury">Treasury</a>,
    _rate_map: &<b>mut</b> VecMap&lt;String, u64&gt;,
    _amount: u64
) {
    <b>let</b> key = <a href="treasury.md#0xc8_treasury_get_vault_key">get_vault_key</a>&lt;StableCoinType&gt;();
    <b>if</b> (!<a href="../sui-framework/dynamic_field.md#0x2_dynamic_field_exists_">dynamic_field::exists_</a>(&_treasury.id, key)) {
        <b>return</b>
    };
    <a href="../sui-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(
        _rate_map,
        key,
        <a href="vault.md#0xc8_vault_calculated_swap_result_amount_out">vault::calculated_swap_result_amount_out</a>(&<a href="treasury.md#0xc8_treasury_calculate_swap_result">calculate_swap_result</a>&lt;StableCoinType&gt;(_treasury, <b>true</b>, _amount)),
    );
}
</code></pre>



</details>

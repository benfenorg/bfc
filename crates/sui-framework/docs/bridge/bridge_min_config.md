---
title: Module `bridge::bridge_min_config`
---



-  [Struct `BridgeMinConfig`](#bridge_bridge_min_config_BridgeMinConfig)
-  [Struct `FeeLimit`](#bridge_bridge_min_config_FeeLimit)
-  [Constants](#@Constants_0)
-  [Function `new_bridge_min_config_registry`](#bridge_bridge_min_config_new_bridge_min_config_registry)
-  [Function `new`](#bridge_bridge_min_config_new)
-  [Function `empty`](#bridge_bridge_min_config_empty)
-  [Function `borrow`](#bridge_bridge_min_config_borrow)
-  [Function `borrow_mut`](#bridge_bridge_min_config_borrow_mut)
-  [Function `exists`](#bridge_bridge_min_config_exists)
-  [Function `set_min_limit_cross_out`](#bridge_bridge_min_config_set_min_limit_cross_out)
-  [Function `set_min_limit_cross_in`](#bridge_bridge_min_config_set_min_limit_cross_in)
-  [Function `set_fee_limit`](#bridge_bridge_min_config_set_fee_limit)
-  [Function `set_min_fee_cross_out`](#bridge_bridge_min_config_set_min_fee_cross_out)
-  [Function `set_min_fee_cross_in`](#bridge_bridge_min_config_set_min_fee_cross_in)
-  [Function `get_min_limit_cross_out`](#bridge_bridge_min_config_get_min_limit_cross_out)
-  [Function `get_min_limit_token_amount_cross_out`](#bridge_bridge_min_config_get_min_limit_token_amount_cross_out)
-  [Function `get_min_limit_cross_in`](#bridge_bridge_min_config_get_min_limit_cross_in)
-  [Function `get_min_limit_token_amount_cross_in`](#bridge_bridge_min_config_get_min_limit_token_amount_cross_in)
-  [Function `get_min_fee_cross_out`](#bridge_bridge_min_config_get_min_fee_cross_out)
-  [Function `get_min_fee_cross_in`](#bridge_bridge_min_config_get_min_fee_cross_in)
-  [Function `check_cross_in_amount_ok`](#bridge_bridge_min_config_check_cross_in_amount_ok)
-  [Function `check_cross_out_amount_ok`](#bridge_bridge_min_config_check_cross_out_amount_ok)
-  [Function `get_effective_cross_out_fee`](#bridge_bridge_min_config_get_effective_cross_out_fee)
-  [Function `get_effective_cross_in_fee`](#bridge_bridge_min_config_get_effective_cross_in_fee)
-  [Function `get_usd_value`](#bridge_bridge_min_config_get_usd_value)
-  [Function `initial_min_fee_limits`](#bridge_bridge_min_config_initial_min_fee_limits)


<pre><code><b>use</b> <a href="../bridge/bridge_fee.md#bridge_bridge_fee">bridge::bridge_fee</a>;
<b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../bridge/crypto.md#bridge_crypto">bridge::crypto</a>;
<b>use</b> <a href="../bridge/message.md#bridge_message">bridge::message</a>;
<b>use</b> <a href="../bridge/message_types.md#bridge_message_types">bridge::message_types</a>;
<b>use</b> <a href="../bridge/tokenlist.md#bridge_tokenlist">bridge::tokenlist</a>;
<b>use</b> <a href="../bridge/treasury.md#bridge_treasury">bridge::treasury</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/bcs.md#sui_bcs">sui::bcs</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/ecdsa_k1.md#sui_ecdsa_k1">sui::ecdsa_k1</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hash.md#sui_hash">sui::hash</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/object_bag.md#sui_object_bag">sui::object_bag</a>;
<b>use</b> <a href="../sui/package.md#sui_package">sui::package</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
</code></pre>



<a name="bridge_bridge_min_config_BridgeMinConfig"></a>

## Struct `BridgeMinConfig`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">BridgeMinConfig</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>min_limit_out: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>min_limit_in: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>fee_limit: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">bridge::bridge_min_config::FeeLimit</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_bridge_min_config_FeeLimit"></a>

## Struct `FeeLimit`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">FeeLimit</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>cross_in_fee_min: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>cross_out_fee_min: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_bridge_min_config_EBridgeMinConfigRegistryAlreadyExists"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_EBridgeMinConfigRegistryAlreadyExists">EBridgeMinConfigRegistryAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="bridge_bridge_min_config_KEY"></a>



<pre><code><b>const</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>: vector&lt;u8&gt; = vector[98, 114, 105, 100, 103, 101, 95, 109, 105, 110, 95, 99, 111, 110, 102, 105, 103];
</code></pre>



<a name="bridge_bridge_min_config_new_bridge_min_config_registry"></a>

## Function `new_bridge_min_config_registry`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_new_bridge_min_config_registry">new_bridge_min_config_registry</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_new_bridge_min_config_registry">new_bridge_min_config_registry</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(
        !dynamic_field::exists_(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>),
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_EBridgeMinConfigRegistryAlreadyExists">EBridgeMinConfigRegistryAlreadyExists</a>
    );
    dynamic_field::add(
        parent_id,
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>,
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_new">new</a>(ctx),
    );
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_new">new</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">bridge::bridge_min_config::BridgeMinConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">BridgeMinConfig</a> {
    <b>let</b> (min_limit_out, min_limit_in, fee_limit) = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_empty">empty</a>(ctx);
    <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">BridgeMinConfig</a> {
        min_limit_out,
        min_limit_in,
        fee_limit,
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_empty"></a>

## Function `empty`



<pre><code><b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_empty">empty</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): (<a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, u64&gt;, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, u64&gt;, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;u64, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">bridge::bridge_min_config::FeeLimit</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_empty">empty</a>(ctx: &<b>mut</b> TxContext): (
    Table&lt;u64, u64&gt;,
    Table&lt;u64, u64&gt;,
    Table&lt;u64, Table&lt;u64, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">FeeLimit</a>&gt;&gt;,
) {
    <b>let</b> min_limit_out = table::new&lt;u64, u64&gt;(ctx);
    <b>let</b> min_limit_in = table::new&lt;u64, u64&gt;(ctx);
    <b>let</b> fee_limit = table::new&lt;u64, Table&lt;u64, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">FeeLimit</a>&gt;&gt;(ctx);
    (min_limit_out, min_limit_in, fee_limit)
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_borrow"></a>

## Function `borrow`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">bridge::bridge_min_config::BridgeMinConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(parent_id: &UID): &<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">BridgeMinConfig</a>{
    dynamic_field::borrow&lt;vector&lt;u8&gt;,<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">BridgeMinConfig</a>&gt;(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<b>mut</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">bridge::bridge_min_config::BridgeMinConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> UID): &<b>mut</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">BridgeMinConfig</a>{
    dynamic_field::borrow_mut&lt;vector&lt;u8&gt;,<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_BridgeMinConfig">BridgeMinConfig</a>&gt;(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_exists"></a>

## Function `exists`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_exists">exists</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_exists">exists</a>(parent_id: &UID): bool {
    dynamic_field::exists_(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_set_min_limit_cross_out"></a>

## Function `set_min_limit_cross_out`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    amount: u64
) {
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.min_limit_out.contains(chain_id)) {
        self.min_limit_out.add(chain_id, amount);
    }<b>else</b>{
        *self.min_limit_out.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id)=amount
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_set_min_limit_cross_in"></a>

## Function `set_min_limit_cross_in`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    amount: u64
) {
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.min_limit_in.contains(chain_id)) {
        self.min_limit_in.add(chain_id, amount);
    }<b>else</b>{
        *self.min_limit_in.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id)=amount
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_set_fee_limit"></a>

## Function `set_fee_limit`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_fee_limit">set_fee_limit</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, cross_in_fee_min: u64, cross_out_fee_min: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_fee_limit">set_fee_limit</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    cross_in_fee_min: u64,
    cross_out_fee_min: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.fee_limit.contains(chain_id)) {
        self.fee_limit.add(chain_id, table::new(ctx));
    };
    <b>let</b> limit = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">FeeLimit</a> { cross_in_fee_min, cross_out_fee_min };
    <b>if</b> (!self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(chain_id).contains(token_id)) {
        self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id).add(token_id, limit);
    }<b>else</b>{
        *self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id).<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(token_id)=limit
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_set_min_fee_cross_out"></a>

## Function `set_min_fee_cross_out`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, fee_amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    fee_amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.fee_limit.contains(chain_id)) {
        self.fee_limit.add(chain_id, table::new(ctx));
    };
    <b>if</b> (!self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(chain_id).contains(token_id)) {
        self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">FeeLimit</a> { cross_in_fee_min: 0, cross_out_fee_min: fee_amount });
    }<b>else</b>{
        <b>let</b> v = self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id).<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(token_id);
        v.cross_out_fee_min = fee_amount
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_set_min_fee_cross_in"></a>

## Function `set_min_fee_cross_in`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, fee_amount: u64, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u64,
    token_id: u64,
    fee_amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.fee_limit.contains(chain_id)) {
        self.fee_limit.add(chain_id, table::new(ctx));
    };
    <b>if</b> (!self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(chain_id).contains(token_id)) {
        self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id).add(token_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_FeeLimit">FeeLimit</a> { cross_in_fee_min: fee_amount, cross_out_fee_min: 0 });
    }<b>else</b>{
        <b>let</b> v = self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(chain_id).<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow_mut">borrow_mut</a>(token_id);
        v.cross_in_fee_min = fee_amount
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_min_limit_cross_out"></a>

## Function `get_min_limit_cross_out`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_out">get_min_limit_cross_out</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_out">get_min_limit_cross_out</a>(parent_id: &UID,chain_id: u64): u64{
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(parent_id);
    <b>if</b> (!self.min_limit_out.contains(chain_id)) {
        <b>return</b> 0
    };
    *self.min_limit_out.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(chain_id)
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_min_limit_token_amount_cross_out"></a>

## Function `get_min_limit_token_amount_cross_out`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_token_amount_cross_out">get_min_limit_token_amount_cross_out</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, chain_id: u64, token_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_token_amount_cross_out">get_min_limit_token_amount_cross_out</a>(
    parent_id: &UID,
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">treasury::BridgeTreasury</a>,
    chain_id: u64,
    token_id: u64): u64 {
    <b>let</b> min_usd = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_out">get_min_limit_cross_out</a>(parent_id, chain_id);
    <b>if</b> (chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64) || chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64)) {
        // <b>for</b> BTC chains min is in satoshi and amount is token amount (satoshi), compared directly
        min_usd
    } <b>else</b> {
        <a href="../bridge/treasury.md#bridge_treasury_get_token_amount_by_usd">treasury::get_token_amount_by_usd</a>(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, token_id, min_usd)
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_min_limit_cross_in"></a>

## Function `get_min_limit_cross_in`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_in">get_min_limit_cross_in</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_in">get_min_limit_cross_in</a>(parent_id: &UID,chain_id: u64): u64{
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(parent_id);
    <b>if</b> (!self.min_limit_in.contains(chain_id)) {
        <b>return</b> 0
    };
    *self.min_limit_in.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(chain_id)
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_min_limit_token_amount_cross_in"></a>

## Function `get_min_limit_token_amount_cross_in`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_token_amount_cross_in">get_min_limit_token_amount_cross_in</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, chain_id: u64, token_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_token_amount_cross_in">get_min_limit_token_amount_cross_in</a>(
    parent_id: &UID,
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">treasury::BridgeTreasury</a>,
    chain_id: u64,
    token_id: u64): u64 {
    <b>let</b> min_usd = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_in">get_min_limit_cross_in</a>(parent_id, chain_id);
    <b>if</b> (chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64) || chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64)) {
        // <b>for</b> BTC chains min is in satoshi and amount is token amount (satoshi), compared directly
        min_usd
    } <b>else</b> {
        <a href="../bridge/treasury.md#bridge_treasury_get_token_amount_by_usd">treasury::get_token_amount_by_usd</a>(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, token_id, min_usd)
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_min_fee_cross_out"></a>

## Function `get_min_fee_cross_out`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_fee_cross_out">get_min_fee_cross_out</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_fee_cross_out">get_min_fee_cross_out</a>(parent_id: &UID,chain_id: u64, token_id: u64): u64{
    <b>if</b> (!dynamic_field::exists_(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>)) {
        <b>return</b> 0
    };
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(parent_id);
    <b>if</b> (!self.fee_limit.contains(chain_id)) {
        <b>return</b> 0
    };
    <b>let</b> inner = self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> 0
    };
    inner.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(token_id).cross_out_fee_min
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_min_fee_cross_in"></a>

## Function `get_min_fee_cross_in`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_fee_cross_in">get_min_fee_cross_in</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_fee_cross_in">get_min_fee_cross_in</a>(parent_id: &UID,chain_id: u64, token_id: u64): u64{
    <b>if</b> (!dynamic_field::exists_(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>)) {
        <b>return</b> 0
    };
    <b>let</b> self=<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(parent_id);
    <b>if</b> (!self.fee_limit.contains(chain_id)) {
        <b>return</b> 0
    };
    <b>let</b> inner = self.fee_limit.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(chain_id);
    <b>if</b> (!inner.contains(token_id)) {
        <b>return</b> 0
    };
    inner.<a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_borrow">borrow</a>(token_id).cross_in_fee_min
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_check_cross_in_amount_ok"></a>

## Function `check_cross_in_amount_ok`

Returns true if cross-in amount meets the chain's single-tx cross-in minimum. If registry is not installed or not configured, no minimum is enforced and returns true.
For BTC chains min is in satoshi and amount is token amount (satoshi), compared directly; for other chains min is 8-decimal USD, amount is converted to USD before comparison.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_check_cross_in_amount_ok">check_cross_in_amount_ok</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, chain_id: u64, token_id: u64, amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_check_cross_in_amount_ok">check_cross_in_amount_ok</a>(
    parent_id: &UID,
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">treasury::BridgeTreasury</a>,
    chain_id: u64,
    token_id: u64,
    amount: u64,
): bool {
    <b>if</b> (!dynamic_field::exists_(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>)) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> min = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_in">get_min_limit_cross_in</a>(parent_id, chain_id);
    <b>if</b> (chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64) || chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64)) {
        amount &gt;= min
    } <b>else</b> {
        <b>let</b> amount_usd = <a href="../bridge/treasury.md#bridge_treasury_get_amount_in_usd_by_token_id">treasury::get_amount_in_usd_by_token_id</a>(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, token_id, amount);
        amount_usd &gt;= min
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_check_cross_out_amount_ok"></a>

## Function `check_cross_out_amount_ok`

Returns true if cross-out amount meets the chain's single-tx cross-out minimum. If registry is not installed or not configured, no minimum is enforced and returns true.
For BTC chains min is in satoshi and amount is token amount (satoshi), compared directly; for other chains min is 8-decimal USD, amount is converted to USD before comparison.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_check_cross_out_amount_ok">check_cross_out_amount_ok</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, chain_id: u64, token_id: u64, amount: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_check_cross_out_amount_ok">check_cross_out_amount_ok</a>(
    parent_id: &UID,
    <a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">treasury::BridgeTreasury</a>,
    chain_id: u64,
    token_id: u64,
    amount: u64,
): bool {
    <b>if</b> (!dynamic_field::exists_(parent_id, <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_KEY">KEY</a>)) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> min = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_limit_cross_out">get_min_limit_cross_out</a>(parent_id, chain_id);
    <b>if</b> (chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64) || chain_id == (<a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64)) {
        amount &gt;= min
    } <b>else</b> {
        <b>let</b> amount_usd = <a href="../bridge/treasury.md#bridge_treasury_get_amount_in_usd_by_token_id">treasury::get_amount_in_usd_by_token_id</a>(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, token_id, amount);
        amount_usd &gt;= min
    }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_effective_cross_out_fee"></a>

## Function `get_effective_cross_out_fee`

Computes cross-out fee and returns the greater of that and the configured cross-out fee minimum.
Configured fee minimum is in token amount (smallest unit for the token, e.g. wei for ETH); comparison is done in token.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_effective_cross_out_fee">get_effective_cross_out_fee</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_effective_cross_out_fee">get_effective_cross_out_fee</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64,
    amount: u64,
): u64 {
    <b>let</b> fee_token = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_out_fee_amount">bridge_fee::calculate_cross_out_fee_amount</a>(parent_id, chain_id, token_id, amount);
    <b>let</b> min_token = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_fee_cross_out">get_min_fee_cross_out</a>(parent_id, chain_id, token_id);
    <b>if</b> (fee_token &gt;= min_token) { fee_token } <b>else</b> { min_token }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_effective_cross_in_fee"></a>

## Function `get_effective_cross_in_fee`

Computes cross-in fee and returns the greater of that and the configured cross-in fee minimum.
Configured fee minimum is in token amount (smallest unit for the token, e.g. wei for ETH); comparison is done in token.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_effective_cross_in_fee">get_effective_cross_in_fee</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u64, token_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_effective_cross_in_fee">get_effective_cross_in_fee</a>(
    parent_id: &UID,
    chain_id: u64,
    token_id: u64,
    amount: u64,
): u64 {
    <b>let</b> fee_token = <a href="../bridge/bridge_fee.md#bridge_bridge_fee_calculate_cross_in_fee_amount">bridge_fee::calculate_cross_in_fee_amount</a>(parent_id, chain_id, token_id, amount);
    <b>let</b> min_token = <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_min_fee_cross_in">get_min_fee_cross_in</a>(parent_id, chain_id, token_id);
    <b>if</b> (fee_token &gt;= min_token) { fee_token } <b>else</b> { min_token }
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_get_usd_value"></a>

## Function `get_usd_value`

Converts token amount (in smallest unit for the given token_id) to USD value with 8 decimals (same as limiter USD precision).
Requires treasury that holds price info for the token.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_usd_value">get_usd_value</a>(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, token_id: u64, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_get_usd_value">get_usd_value</a>(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">treasury::BridgeTreasury</a>, token_id: u64, amount: u64): u64 {
    <a href="../bridge/treasury.md#bridge_treasury_get_amount_in_usd_by_token_id">treasury::get_amount_in_usd_by_token_id</a>(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, token_id, amount)
}
</code></pre>



</details>

<a name="bridge_bridge_min_config_initial_min_fee_limits"></a>

## Function `initial_min_fee_limits`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_initial_min_fee_limits">initial_min_fee_limits</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_initial_min_fee_limits">initial_min_fee_limits</a>(parent_id: &<b>mut</b> UID, ctx: &<b>mut</b> TxContext) {
    // ========================
    // Mainnet configuration
    // ========================
    {
        // BTC: min single cross-in/out = 0.0001 BTC (10_000 satoshi)
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">chain_ids::btc_mainnet</a>() <b>as</b> u64;
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 10_000);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 10_000);
    };
    {
        // ETH: min_in = 0 USD, min_out = 1 USD (8 dp: 100_000_000)
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Base: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Optimism: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // BSC: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Polygon: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Arbitrum: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // AVAX-C: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Tron: min_in = 0 USD, min_out = 10 USD (8 dp: 1_000_000_000)
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">chain_ids::tron_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 1_000_000_000);
        // Tron USDT min cross-out fee: 5 USDT (6 dp: 5_000_000)
        <b>let</b> usdt = <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>();
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(parent_id, chain, usdt, 0, ctx);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(parent_id, chain, usdt, 5_000_000, ctx);
    };
    {
        // Solana: min_in = 0 USD, min_out = 10 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">chain_ids::solana_mainnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 1_000_000_000);
        // Solana stablecoins min cross-out fee: USDT/USDC = 0.5 USD (6 dp: 500_000)
        <b>let</b> usdt = <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>();
        <b>let</b> usdc = <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>();
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(parent_id, chain, usdt, 500_000, ctx);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(parent_id, chain, usdt, 0, ctx);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(parent_id, chain, usdc, 500_000, ctx);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(parent_id, chain, usdc, 0, ctx);
    };
    // ========================
    // Testnet configuration
    // ========================
    {
        // BTC Testnet: 0.0001 BTC (satoshi)
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">chain_ids::btc_testnet</a>() <b>as</b> u64;
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 10_000);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 10_000);
    };
    {
        // ETH Sepolia: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Base Testnet: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Optimism Testnet: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // BSC Testnet: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Polygon Testnet: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Arbitrum Testnet: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>() <b>as</b> u64;
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // AVAX-C Testnet: min_in = 0 USD, min_out = 1 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 100_000_000);
    };
    {
        // Tron Testnet: min_in = 0 USD, min_out = 10 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">chain_ids::tron_testnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 1_000_000_000);
        // Tron Testnet USDT min cross-out fee: 5 USDT
        <b>let</b> usdt = <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>();
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(parent_id, chain, usdt, 5_000_000, ctx);
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(parent_id, chain, usdt, 0, ctx);
    };
    {
        // Solana Testnet: min_in = 0 USD, min_out = 10 USD
        <b>let</b> chain = <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">chain_ids::solana_testnet</a>() <b>as</b> u64;
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_in">set_min_limit_cross_in</a>(parent_id, chain, 0);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_limit_cross_out">set_min_limit_cross_out</a>(parent_id, chain, 1_000_000_000);
        // Solana Testnet stablecoins min cross-out fee: USDT/USDC = 0.5 USD
        <b>let</b> usdt = <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdt_token_id">tokenlist::get_usdt_token_id</a>();
        <b>let</b> usdc = <a href="../bridge/tokenlist.md#bridge_tokenlist_get_usdc_token_id">tokenlist::get_usdc_token_id</a>();
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(parent_id, chain, usdt, 500_000, ctx);
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(parent_id, chain, usdt, 0, ctx);
        <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_out">set_min_fee_cross_out</a>(parent_id, chain, usdc, 500_000, ctx);
        // <a href="../bridge/bridge_min_config.md#bridge_bridge_min_config_set_min_fee_cross_in">set_min_fee_cross_in</a>(parent_id, chain, usdc, 0, ctx);
    };
}
</code></pre>



</details>

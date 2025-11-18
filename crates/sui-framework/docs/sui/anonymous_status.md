---
title: Module `sui::anonymous`
---

This module provides functionality for anonymous.


-  [Struct `Anonymous`](#sui_anonymous_Anonymous)
-  [Struct `AnonymousInner`](#sui_anonymous_AnonymousInner)
-  [Constants](#@Constants_0)
-  [Function `create`](#sui_anonymous_create)
-  [Function `load_inner_mut`](#sui_anonymous_load_inner_mut)
-  [Function `load_inner`](#sui_anonymous_load_inner)
-  [Function `update_annoymous_state`](#sui_anonymous_update_annoymous_state)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/versioned.md#sui_versioned">sui::versioned</a>;
</code></pre>



<a name="sui_anonymous_Anonymous"></a>

## Struct `Anonymous`

Singleton shared object which stores the global Anonymous state.
The actual state is stored in a versioned inner field.


<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">Anonymous</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui/object.md#sui_object_UID">sui::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>inner: <a href="../sui/versioned.md#sui_versioned_Versioned">sui::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="sui_anonymous_AnonymousInner"></a>

## Struct `AnonymousInner`



<pre><code><b>public</b> <b>struct</b> <a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">AnonymousInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>value: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="sui_anonymous_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_status.md#sui_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>: u64 = 1;
</code></pre>



<a name="sui_anonymous_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_status.md#sui_anonymous_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="sui_anonymous_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../sui/anonymous_status.md#sui_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 1;
</code></pre>



<a name="sui_anonymous_create"></a>

## Function `create`

Create and share the Anonymous object. This function is called exactly once, when
the Anonymous object is first created.
Can only be called by genesis or change_epoch transactions.


<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_create">create</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_create">create</a>(ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../sui/anonymous_status.md#sui_anonymous_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> version = <a href="../sui/anonymous_status.md#sui_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>;
    <b>let</b> inner = <a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">AnonymousInner</a> {
        version,
        epoch: ctx.epoch(),
        value: 0,
    };
    <b>let</b> self = <a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">Anonymous</a> {
        id: <a href="../sui/object.md#sui_object_anonymous_state_id">object::anonymous_state_id</a>(),
        //id: <a href="../sui/object.md#sui_object_new">object::new</a>(ctx),
        inner: <a href="../sui/versioned.md#sui_versioned_create">versioned::create</a>(version, inner, ctx),
    };
    <a href="../sui/transfer.md#sui_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="sui_anonymous_load_inner_mut"></a>

## Function `load_inner_mut`



<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_load_inner_mut">load_inner_mut</a>(self: &<b>mut</b> <a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">sui::anonymous::Anonymous</a>): &<b>mut</b> <a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">sui::anonymous::AnonymousInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_load_inner_mut">load_inner_mut</a>(self: &<b>mut</b> <a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">Anonymous</a>): &<b>mut</b> <a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">AnonymousInner</a> {
    <b>let</b> version = <a href="../sui/versioned.md#sui_versioned_version">versioned::version</a>(&self.inner);
    // Replace this with a lazy update function when we add a new version of the inner <a href="../sui/object.md#sui_object">object</a>.
    <b>assert</b>!(version == <a href="../sui/anonymous_status.md#sui_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../sui/anonymous_status.md#sui_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<b>mut</b> <a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">AnonymousInner</a> = <a href="../sui/versioned.md#sui_versioned_load_value_mut">versioned::load_value_mut</a>(&<b>mut</b> self.inner);
    <b>assert</b>!(inner.version == version, <a href="../sui/anonymous_status.md#sui_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="sui_anonymous_load_inner"></a>

## Function `load_inner`



<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_load_inner">load_inner</a>(self: &<a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">sui::anonymous::Anonymous</a>): &<a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">sui::anonymous::AnonymousInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_load_inner">load_inner</a>(self: &<a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">Anonymous</a>): &<a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">AnonymousInner</a> {
    <b>let</b> version = <a href="../sui/versioned.md#sui_versioned_version">versioned::version</a>(&self.inner);
    // Replace this with a lazy update function when we add a new version of the inner <a href="../sui/object.md#sui_object">object</a>.
    <b>assert</b>!(version == <a href="../sui/anonymous_status.md#sui_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../sui/anonymous_status.md#sui_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<a href="../sui/anonymous_status.md#sui_anonymous_AnonymousInner">AnonymousInner</a> = <a href="../sui/versioned.md#sui_versioned_load_value">versioned::load_value</a>(&self.inner);
    <b>assert</b>!(inner.version == version, <a href="../sui/anonymous_status.md#sui_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="sui_anonymous_update_annoymous_state"></a>

## Function `update_annoymous_state`



<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_update_annoymous_state">update_annoymous_state</a>(self: &<b>mut</b> <a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">sui::anonymous::Anonymous</a>, new_value: u64, ctx: &<a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui/anonymous_status.md#sui_anonymous_update_annoymous_state">update_annoymous_state</a>(
    self: &<b>mut</b> <a href="../sui/anonymous_status.md#sui_anonymous_Anonymous">Anonymous</a>,
    new_value: u64,
    ctx: &TxContext,
) {
    // Validator will make a special system call with sender set <b>as</b> 0x0.
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../sui/anonymous_status.md#sui_anonymous_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> inner = self.<a href="../sui/anonymous_status.md#sui_anonymous_load_inner_mut">load_inner_mut</a>();
    inner.epoch = ctx.epoch();
    inner.value = new_value;
}
</code></pre>



</details>

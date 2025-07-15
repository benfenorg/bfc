---
title: Module `0x2::anonymous`
---

This module provides functionality for anonymous.


-  [Resource `Anonymous`](#0x2_anonymous_Anonymous)
-  [Struct `AnonymousInner`](#0x2_anonymous_AnonymousInner)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x2_anonymous_create)
-  [Function `load_inner_mut`](#0x2_anonymous_load_inner_mut)
-  [Function `load_inner`](#0x2_anonymous_load_inner)
-  [Function `update_annoymous_state`](#0x2_anonymous_update_annoymous_state)


<pre><code><b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/versioned.md#0x2_versioned">0x2::versioned</a>;
</code></pre>



<a name="0x2_anonymous_Anonymous"></a>

## Resource `Anonymous`

Singleton shared object which stores the global Anonymous state.
The actual state is stored in a versioned inner field.


<pre><code><b>struct</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">Anonymous</a> <b>has</b> key
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
<code>inner: <a href="../sui-framework/versioned.md#0x2_versioned_Versioned">versioned::Versioned</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_anonymous_AnonymousInner"></a>

## Struct `AnonymousInner`



<pre><code><b>struct</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">AnonymousInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>version: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>epoch: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_anonymous_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_ENotSystemAddress">ENotSystemAddress</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0x2_anonymous_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x2_anonymous_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0x2_anonymous_create"></a>

## Function `create`

Create and share the Anonymous object. This function is called exactly once, when
the Anonymous object is first created.
Can only be called by genesis or change_epoch transactions.


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_create">create</a>(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_create">create</a>(ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../sui-framework/anonymous_status.md#0x2_anonymous_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> version = <a href="../sui-framework/anonymous_status.md#0x2_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>;

    <b>let</b> inner = <a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">AnonymousInner</a> {
        version,
        epoch: ctx.epoch(),
        value: 0,
    };

    <b>let</b> self = <a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">Anonymous</a> {
        id: <a href="../sui-framework/object.md#0x2_object_anonymous_state_id">object::anonymous_state_id</a>(),
        //id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        inner: <a href="../sui-framework/versioned.md#0x2_versioned_create">versioned::create</a>(version, inner, ctx),
    };
    <a href="../sui-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0x2_anonymous_load_inner_mut"></a>

## Function `load_inner_mut`



<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_load_inner_mut">load_inner_mut</a>(self: &<b>mut</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">anonymous::Anonymous</a>): &<b>mut</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">anonymous::AnonymousInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_load_inner_mut">load_inner_mut</a>(self: &<b>mut</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">Anonymous</a>): &<b>mut</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">AnonymousInner</a> {
    <b>let</b> version = <a href="../sui-framework/versioned.md#0x2_versioned_version">versioned::version</a>(&self.inner);

    // Replace this <b>with</b> a lazy <b>update</b> function when we add a new version of the inner <a href="../sui-framework/object.md#0x2_object">object</a>.
    <b>assert</b>!(version == <a href="../sui-framework/anonymous_status.md#0x2_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../sui-framework/anonymous_status.md#0x2_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<b>mut</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">AnonymousInner</a> = <a href="../sui-framework/versioned.md#0x2_versioned_load_value_mut">versioned::load_value_mut</a>(&<b>mut</b> self.inner);
    <b>assert</b>!(inner.version == version, <a href="../sui-framework/anonymous_status.md#0x2_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="0x2_anonymous_load_inner"></a>

## Function `load_inner`



<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_load_inner">load_inner</a>(self: &<a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">anonymous::Anonymous</a>): &<a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">anonymous::AnonymousInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_load_inner">load_inner</a>(self: &<a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">Anonymous</a>): &<a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">AnonymousInner</a> {
    <b>let</b> version = <a href="../sui-framework/versioned.md#0x2_versioned_version">versioned::version</a>(&self.inner);

    // Replace this <b>with</b> a lazy <b>update</b> function when we add a new version of the inner <a href="../sui-framework/object.md#0x2_object">object</a>.
    <b>assert</b>!(version == <a href="../sui-framework/anonymous_status.md#0x2_anonymous_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../sui-framework/anonymous_status.md#0x2_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<a href="../sui-framework/anonymous_status.md#0x2_anonymous_AnonymousInner">AnonymousInner</a> = <a href="../sui-framework/versioned.md#0x2_versioned_load_value">versioned::load_value</a>(&self.inner);
    <b>assert</b>!(inner.version == version, <a href="../sui-framework/anonymous_status.md#0x2_anonymous_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="0x2_anonymous_update_annoymous_state"></a>

## Function `update_annoymous_state`



<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_update_annoymous_state">update_annoymous_state</a>(self: &<b>mut</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">anonymous::Anonymous</a>, new_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_update_annoymous_state">update_annoymous_state</a>(
    self: &<b>mut</b> <a href="../sui-framework/anonymous_status.md#0x2_anonymous_Anonymous">Anonymous</a>,
    new_value: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>,
    ctx: &TxContext,
) {
    // Validator will make a special system call <b>with</b> sender set <b>as</b> 0x0.
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../sui-framework/anonymous_status.md#0x2_anonymous_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> inner = self.<a href="../sui-framework/anonymous_status.md#0x2_anonymous_load_inner_mut">load_inner_mut</a>();


    inner.epoch = ctx.epoch();
    inner.value = new_value;
}
</code></pre>



</details>

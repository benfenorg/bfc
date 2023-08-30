
<a name="0xc8_obc_system"></a>

# Module `0xc8::obc_system`



-  [Resource `ObcSystemState`](#0xc8_obc_system_ObcSystemState)
-  [Struct `ObcSystemStateInner`](#0xc8_obc_system_ObcSystemStateInner)
-  [Constants](#@Constants_0)
-  [Function `create`](#0xc8_obc_system_create)
-  [Function `request_exchange_stable_obc`](#0xc8_obc_system_request_exchange_stable_obc)
-  [Function `obc_round`](#0xc8_obc_system_obc_round)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="obc.md#0xc8_obc">0xc8::obc</a>;
</code></pre>



<a name="0xc8_obc_system_ObcSystemState"></a>

## Resource `ObcSystemState`



<pre><code><b>struct</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a> <b>has</b> key
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
<code>version: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_obc_system_ObcSystemStateInner"></a>

## Struct `ObcSystemStateInner`



<pre><code><b>struct</b> <a href="obc_system.md#0xc8_obc_system_ObcSystemStateInner">ObcSystemStateInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>round: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1"></a>



<pre><code><b>const</b> <a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>: u64 = 1;
</code></pre>



<a name="0xc8_obc_system_create"></a>

## Function `create`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(
    id: UID,
    ctx: &<b>mut</b> TxContext,
){
    //<b>let</b> exchange_gas_coin_pool =  <a href="exchange_inner.md#0xc8_exchange_inner_new_exchange_pool">exchange_inner::new_exchange_pool</a>(ctx, 0);
    <b>let</b> system_state = <a href="obc_system.md#0xc8_obc_system_ObcSystemStateInner">ObcSystemStateInner</a>{
         round:0,
    };

    <b>let</b> self = <a href="obc_system.md#0xc8_obc_system_ObcSystemState">ObcSystemState</a> {
        id,
        version:<a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>
    };

    <a href="../../../.././build/Sui/docs/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id,<a href="obc_system.md#0xc8_obc_system_OBC_SYSTEM_STATE_VERSION_V1">OBC_SYSTEM_STATE_VERSION_V1</a>, system_state);

    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0xc8_obc_system_request_exchange_stable_obc"></a>

## Function `request_exchange_stable_obc`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable_obc">request_exchange_stable_obc</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_request_exchange_stable_obc">request_exchange_stable_obc</a>(
    ctx: &<b>mut</b> TxContext
){

}
</code></pre>



</details>

<a name="0xc8_obc_system_obc_round"></a>

## Function `obc_round`



<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="obc_system.md#0xc8_obc_system_obc_round">obc_round</a>(ctx: &<b>mut</b> TxContext):u64 {
    length()
}
</code></pre>



</details>

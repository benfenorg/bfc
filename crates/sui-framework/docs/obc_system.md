
<a name="0xc8_obc_system"></a>

# Module `0xc8::obc_system`



-  [Struct `ObcSystemStateInner`](#0xc8_obc_system_ObcSystemStateInner)
-  [Function `create`](#0xc8_obc_system_create)
-  [Function `request_exchange_stable_obc`](#0xc8_obc_system_request_exchange_stable_obc)
-  [Function `obc_round`](#0xc8_obc_system_obc_round)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="obc.md#0xc8_obc">0xc8::obc</a>;
</code></pre>



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
 The current epoch ID, starting from 0.
</dd>
</dl>


</details>

<a name="0xc8_obc_system_create"></a>

## Function `create`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system.md#0xc8_obc_system_create">create</a>(
    ctx: &<b>mut</b> TxContext,
){
    //<b>let</b> exchange_gas_coin_pool =  <a href="exchange_inner.md#0xc8_exchange_inner_new_exchange_pool">exchange_inner::new_exchange_pool</a>(ctx, 0);
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

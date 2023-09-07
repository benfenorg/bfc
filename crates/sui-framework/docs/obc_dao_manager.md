
<a name="0xc8_obc_dao_manager"></a>

# Module `0xc8::obc_dao_manager`



-  [Resource `OBCDaoManageKey`](#0xc8_obc_dao_manager_OBCDaoManageKey)
-  [Function `new`](#0xc8_obc_dao_manager_new)
-  [Function `getKeyAddress`](#0xc8_obc_dao_manager_getKeyAddress)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0xc8_obc_dao_manager_OBCDaoManageKey"></a>

## Resource `OBCDaoManageKey`



<pre><code><b>struct</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">OBCDaoManageKey</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_obc_dao_manager_new"></a>

## Function `new`

Create a new key.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_new">new</a>(sender: <b>address</b>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_new">new</a>(sender: <b>address</b>, ctx: &<b>mut</b> TxContext)  {
    <b>let</b> key = <a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">OBCDaoManageKey</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),

    };
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(key, sender);
}
</code></pre>



</details>

<a name="0xc8_obc_dao_manager_getKeyAddress"></a>

## Function `getKeyAddress`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_getKeyAddress">getKeyAddress</a>(key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_getKeyAddress">getKeyAddress</a>(key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">OBCDaoManageKey</a>) : <b>address</b> {
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_uid_to_address">object::uid_to_address</a>(&key.id)
}
</code></pre>



</details>

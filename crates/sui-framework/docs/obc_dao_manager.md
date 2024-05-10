
<a name="0xc8_obc_dao_manager"></a>

# Module `0xc8::obc_dao_manager`



-  [Resource `OBCDaoManageKey`](#0xc8_obc_dao_manager_OBCDaoManageKey)
-  [Resource `ManagerKeyObc`](#0xc8_obc_dao_manager_ManagerKeyObc)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_obc_dao_manager_new)
-  [Function `create_stake_key`](#0xc8_obc_dao_manager_create_stake_key)
-  [Function `unstake_key`](#0xc8_obc_dao_manager_unstake_key)
-  [Function `getKeyAddress`](#0xc8_obc_dao_manager_getKeyAddress)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
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
<dt>
<code>key_type: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_obc_dao_manager_ManagerKeyObc"></a>

## Resource `ManagerKeyObc`



<pre><code><b>struct</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ManagerKeyObc">ManagerKeyObc</a> <b>has</b> store, key
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
<code>principal: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_obc_dao_manager_ERROR_KEY_NOT_MATCH"></a>



<pre><code><b>const</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ERROR_KEY_NOT_MATCH">ERROR_KEY_NOT_MATCH</a>: u64 = 1401;
</code></pre>



<a name="0xc8_obc_dao_manager_ERROR_KEY_TYPE"></a>



<pre><code><b>const</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ERROR_KEY_TYPE">ERROR_KEY_TYPE</a>: u64 = 1400;
</code></pre>



<a name="0xc8_obc_dao_manager_FREE_KEY"></a>



<pre><code><b>const</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_FREE_KEY">FREE_KEY</a>: u64 = 0;
</code></pre>



<a name="0xc8_obc_dao_manager_STAKE_KEY"></a>



<pre><code><b>const</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_STAKE_KEY">STAKE_KEY</a>: u64 = 1;
</code></pre>



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
        key_type: <a href="obc_dao_manager.md#0xc8_obc_dao_manager_FREE_KEY">FREE_KEY</a>,
        amount: 0,
    };
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(key, sender);
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>false</b>;
<b>aborts_if</b> ctx.ids_created + 1 &gt; MAX_U64;
</code></pre>



</details>

<a name="0xc8_obc_dao_manager_create_stake_key"></a>

## Function `create_stake_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_create_stake_key">create_stake_key</a>(sender: <b>address</b>, payment: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_create_stake_key">create_stake_key</a>(sender: <b>address</b>,
                                    payment: Balance&lt;BFC&gt;,
                                    ctx: &<b>mut</b> TxContext)  {
    <b>let</b> key = <a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">OBCDaoManageKey</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        key_type: <a href="obc_dao_manager.md#0xc8_obc_dao_manager_STAKE_KEY">STAKE_KEY</a>,
        amount: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&payment)
    };

    <b>let</b> managerObc = <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ManagerKeyObc">ManagerKeyObc</a> {
        id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(ctx),
        principal: payment,
    };


    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(key, sender);
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(managerObc, sender);

}
</code></pre>



</details>

<a name="0xc8_obc_dao_manager_unstake_key"></a>

## Function `unstake_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_unstake_key">unstake_key</a>(key: <a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, token: <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ManagerKeyObc">obc_dao_manager::ManagerKeyObc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> (<b>friend</b>) <b>fun</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_unstake_key">unstake_key</a>(key:<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">OBCDaoManageKey</a>, token: <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ManagerKeyObc">ManagerKeyObc</a>, ctx: &<b>mut</b> TxContext){

    <b>assert</b>!(key.key_type == <a href="obc_dao_manager.md#0xc8_obc_dao_manager_STAKE_KEY">STAKE_KEY</a>, <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ERROR_KEY_TYPE">ERROR_KEY_TYPE</a>);
    <b>assert</b>!(key.amount == <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&token.principal), <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ERROR_KEY_NOT_MATCH">ERROR_KEY_NOT_MATCH</a>);

    //convert proposal payment <b>to</b> voting_obc
    <b>let</b> sender = <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

    <b>let</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">OBCDaoManageKey</a>{id:uid,
                        key_type: _key_type,
                        amount: _amount,}= key;
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_delete">object::delete</a>(uid);

    <b>let</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ManagerKeyObc">ManagerKeyObc</a>{id:uid,
        principal:obc}= token;

    <a href="../../../.././build/Sui/docs/object.md#0x2_object_delete">object::delete</a>(uid);

    <b>let</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a> = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(obc, ctx);
    <a href="../../../.././build/Sui/docs/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, sender);

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

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify;
</code></pre>

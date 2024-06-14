
<a name="0xc8_event"></a>

# Module `0xc8::event`



-  [Struct `InitTreasuryEvent`](#0xc8_event_InitTreasuryEvent)
-  [Struct `InitTreasuryPoolEvent`](#0xc8_event_InitTreasuryPoolEvent)
-  [Struct `CreateVaultEvent`](#0xc8_event_CreateVaultEvent)
-  [Struct `PauseEvent`](#0xc8_event_PauseEvent)
-  [Struct `SwapEvent`](#0xc8_event_SwapEvent)
-  [Struct `DepositEvent`](#0xc8_event_DepositEvent)
-  [Struct `UpdateStateEvent`](#0xc8_event_UpdateStateEvent)
-  [Struct `RebalanceEvent`](#0xc8_event_RebalanceEvent)
-  [Function `init_treasury`](#0xc8_event_init_treasury)
-  [Function `init_treasury_pool`](#0xc8_event_init_treasury_pool)
-  [Function `create_vault`](#0xc8_event_create_vault)
-  [Function `swap`](#0xc8_event_swap)
-  [Function `deposit`](#0xc8_event_deposit)
-  [Function `update_state`](#0xc8_event_update_state)
-  [Function `set_pause`](#0xc8_event_set_pause)
-  [Function `rebalance`](#0xc8_event_rebalance)
-  [Module Specification](#@Module_Specification_0)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
</code></pre>



<a name="0xc8_event_InitTreasuryEvent"></a>

## Struct `InitTreasuryEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_InitTreasuryEvent">InitTreasuryEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vaults_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_InitTreasuryPoolEvent"></a>

## Struct `InitTreasuryPoolEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_InitTreasuryPoolEvent">InitTreasuryPoolEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>treasury_pool_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_CreateVaultEvent"></a>

## Struct `CreateVaultEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_CreateVaultEvent">CreateVaultEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>vault_key: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_a: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_b: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tick_spacing: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>spacing_times: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_PauseEvent"></a>

## Struct `PauseEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_PauseEvent">PauseEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="vault.md#0xc8_vault">vault</a>: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>is_pause: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_SwapEvent"></a>

## Struct `SwapEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_SwapEvent">SwapEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>atob: bool</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="vault.md#0xc8_vault">vault</a>: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_in: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_out: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount_in: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>amount_out: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vault_a_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vault_b_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>before_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>after_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>steps: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_DepositEvent"></a>

## Struct `DepositEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_DepositEvent">DepositEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_UpdateStateEvent"></a>

## Struct `UpdateStateEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_UpdateStateEvent">UpdateStateEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>current_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>last_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>state: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>state_counter: u32</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_RebalanceEvent"></a>

## Struct `RebalanceEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_RebalanceEvent">RebalanceEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_type: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_init_treasury"></a>

## Function `init_treasury`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_init_treasury">init_treasury</a>(vaults_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_init_treasury">init_treasury</a>(vaults_id: ID) {
    emit(<a href="event.md#0xc8_event_InitTreasuryEvent">InitTreasuryEvent</a> { vaults_id })
}
</code></pre>



</details>

<a name="0xc8_event_init_treasury_pool"></a>

## Function `init_treasury_pool`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_init_treasury_pool">init_treasury_pool</a>(treasury_pool_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_init_treasury_pool">init_treasury_pool</a>(treasury_pool_id: ID) {
    emit(<a href="event.md#0xc8_event_InitTreasuryPoolEvent">InitTreasuryPoolEvent</a> { treasury_pool_id })
}
</code></pre>



</details>

<a name="0xc8_event_create_vault"></a>

## Function `create_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_create_vault">create_vault</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, vault_key: <a href="_String">ascii::String</a>, coin_type_a: <a href="_String">ascii::String</a>, coin_type_b: <a href="_String">ascii::String</a>, tick_spacing: u32, spacing_times: u32, index: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_create_vault">create_vault</a>(
    vault_id: ID,
    vault_key: String,
    coin_type_a: String,
    coin_type_b: String,
    tick_spacing: u32,
    spacing_times: u32,
    index: u64,
) {
    emit(<a href="event.md#0xc8_event_CreateVaultEvent">CreateVaultEvent</a> {
        vault_id,
        vault_key,
        coin_type_a,
        coin_type_b,
        tick_spacing,
        spacing_times,
        index,
    })
}
</code></pre>



</details>

<a name="0xc8_event_swap"></a>

## Function `swap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_swap">swap</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, atob: bool, coin_type_in: <a href="_String">ascii::String</a>, coin_type_out: <a href="_String">ascii::String</a>, amount_in: u64, amount_out: u64, vault_a_amount: u64, vault_b_amount: u64, before_sqrt_price: u128, after_sqrt_price: u128, steps: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_swap">swap</a>(
    vault_id: ID,
    atob: bool, // <b>true</b> a-&gt;b <b>false</b> b-&gt;a
    coin_type_in: String,
    coin_type_out: String,
    amount_in: u64,
    amount_out: u64,
    vault_a_amount: u64, // current <a href="vault.md#0xc8_vault">vault</a> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>(A)
    vault_b_amount: u64, // current <a href="vault.md#0xc8_vault">vault</a> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>(B)
    before_sqrt_price: u128,
    after_sqrt_price: u128,
    steps: u64
) {
    emit(
        <a href="event.md#0xc8_event_SwapEvent">SwapEvent</a> {
            <a href="vault.md#0xc8_vault">vault</a>: vault_id,
            atob,
            coin_type_in,
            coin_type_out,
            amount_in,
            amount_out,
            vault_a_amount,
            vault_b_amount,
            before_sqrt_price,
            after_sqrt_price,
            steps
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_deposit"></a>

## Function `deposit`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_deposit">deposit</a>(amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_deposit">deposit</a>(amount: u64) {
    emit(
        <a href="event.md#0xc8_event_DepositEvent">DepositEvent</a> {
            amount
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_update_state"></a>

## Function `update_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_update_state">update_state</a>(coin_type: <a href="_String">ascii::String</a>, current_sqrt_price: u128, last_sqrt_price: u128, state: u8, state_counter: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_update_state">update_state</a>(
    coin_type: String,
    current_sqrt_price: u128,
    last_sqrt_price: u128,
    state: u8,
    state_counter: u32,
) {
    emit(
        <a href="event.md#0xc8_event_UpdateStateEvent">UpdateStateEvent</a> {
            coin_type,
            current_sqrt_price,
            last_sqrt_price,
            state,
            state_counter,
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_set_pause"></a>

## Function `set_pause`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_set_pause">set_pause</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, is_pause: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_set_pause">set_pause</a>(vault_id: ID, is_pause: bool) {
    emit(
        <a href="event.md#0xc8_event_PauseEvent">PauseEvent</a> {
            <a href="vault.md#0xc8_vault">vault</a>: vault_id,
            is_pause
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_rebalance">rebalance</a>(coin_type: <a href="_String">ascii::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_rebalance">rebalance</a>(coin_type: String) {
    emit(
        <a href="event.md#0xc8_event_RebalanceEvent">RebalanceEvent</a> {
            coin_type,
        }
    )
}
</code></pre>



</details>

<a name="@Module_Specification_0"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

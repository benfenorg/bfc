
<a name="0xc8_event"></a>

# Module `0xc8::event`



-  [Struct `InitTreasuryEvent`](#0xc8_event_InitTreasuryEvent)
-  [Struct `CreateVaultEvent`](#0xc8_event_CreateVaultEvent)
-  [Struct `OpenPositionEvent`](#0xc8_event_OpenPositionEvent)
-  [Struct `ClosePositionEvent`](#0xc8_event_ClosePositionEvent)
-  [Struct `LiquidityEvent`](#0xc8_event_LiquidityEvent)
-  [Struct `SwapEvent`](#0xc8_event_SwapEvent)
-  [Struct `DepositEvent`](#0xc8_event_DepositEvent)
-  [Struct `UpdateStateEvent`](#0xc8_event_UpdateStateEvent)
-  [Function `init_treasury`](#0xc8_event_init_treasury)
-  [Function `create_vault`](#0xc8_event_create_vault)
-  [Function `open_position`](#0xc8_event_open_position)
-  [Function `close_position`](#0xc8_event_close_position)
-  [Function `add_liquidity`](#0xc8_event_add_liquidity)
-  [Function `remove_liquidity`](#0xc8_event_remove_liquidity)
-  [Function `swap`](#0xc8_event_swap)
-  [Function `deposit`](#0xc8_event_deposit)
-  [Function `update_state`](#0xc8_event_update_state)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="i32.md#0xc8_i32">0xc8::i32</a>;
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

<a name="0xc8_event_OpenPositionEvent"></a>

## Struct `OpenPositionEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_OpenPositionEvent">OpenPositionEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="position.md#0xc8_position">position</a>: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_ClosePositionEvent"></a>

## Struct `ClosePositionEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_ClosePositionEvent">ClosePositionEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="position.md#0xc8_position">position</a>: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_LiquidityEvent"></a>

## Struct `LiquidityEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_LiquidityEvent">LiquidityEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="position.md#0xc8_position">position</a>: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>after_liquidity: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>amount_a: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>amount_b: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>action: <a href="_String">ascii::String</a></code>
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

<a name="0xc8_event_open_position"></a>

## Function `open_position`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_open_position">open_position</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, position_id: u64, tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_open_position">open_position</a>(
    vault_id: ID,
    position_id: u64,
    tick_lower: I32,
    tick_upper: I32
) {
    emit(
        <a href="event.md#0xc8_event_OpenPositionEvent">OpenPositionEvent</a> {
            <a href="vault.md#0xc8_vault">vault</a>: vault_id,
            <a href="position.md#0xc8_position">position</a>: position_id,
            tick_lower,
            tick_upper
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_close_position"></a>

## Function `close_position`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_close_position">close_position</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, position_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_close_position">close_position</a>(
    vault_id: ID,
    position_id: u64
) {
    emit(
        <a href="event.md#0xc8_event_ClosePositionEvent">ClosePositionEvent</a> {
            <a href="vault.md#0xc8_vault">vault</a>: vault_id,
            <a href="position.md#0xc8_position">position</a>: position_id
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_add_liquidity"></a>

## Function `add_liquidity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_add_liquidity">add_liquidity</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, position_id: u64, tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, liquidity: u128, after_liquidity: u128, amount_a: u64, amount_b: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_add_liquidity">add_liquidity</a>(
    vault_id: ID,
    position_id: u64,
    tick_lower: I32,
    tick_upper: I32,
    liquidity: u128,
    after_liquidity: u128,
    amount_a: u64,
    amount_b: u64
) {
    emit(
        <a href="event.md#0xc8_event_LiquidityEvent">LiquidityEvent</a> {
            <a href="vault.md#0xc8_vault">vault</a>: vault_id,
            <a href="position.md#0xc8_position">position</a>: position_id,
            tick_lower,
            tick_upper,
            liquidity,
            after_liquidity,
            amount_a,
            amount_b,
            action: <a href="_string">ascii::string</a>(b"add")
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_remove_liquidity"></a>

## Function `remove_liquidity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_remove_liquidity">remove_liquidity</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, position_id: u64, tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, liquidity: u128, after_liquidity: u128, amount_a: u64, amount_b: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_remove_liquidity">remove_liquidity</a>(
    vault_id: ID,
    position_id: u64,
    tick_lower: I32,
    tick_upper: I32,
    liquidity: u128,
    after_liquidity: u128,
    amount_a: u64,
    amount_b: u64
) {
    emit(
        <a href="event.md#0xc8_event_LiquidityEvent">LiquidityEvent</a> {
            <a href="vault.md#0xc8_vault">vault</a>: vault_id,
            <a href="position.md#0xc8_position">position</a>: position_id,
            tick_lower,
            tick_upper,
            liquidity,
            after_liquidity,
            amount_a,
            amount_b,
            action: <a href="_string">ascii::string</a>(b"remove")
        }
    )
}
</code></pre>



</details>

<a name="0xc8_event_swap"></a>

## Function `swap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_swap">swap</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, atob: bool, amount_in: u64, amount_out: u64, vault_a_amount: u64, vault_b_amount: u64, before_sqrt_price: u128, after_sqrt_price: u128, steps: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_swap">swap</a>(
    vault_id: ID,
    atob: bool, // <b>true</b> a-&gt;b <b>false</b> b-&gt;a
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



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_update_state">update_state</a>(current_sqrt_price: u128, last_sqrt_price: u128, state: u8, state_counter: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_update_state">update_state</a>(
    current_sqrt_price: u128,
    last_sqrt_price: u128,
    state: u8,
    state_counter: u32,
) {
    emit(
        <a href="event.md#0xc8_event_UpdateStateEvent">UpdateStateEvent</a> {
            current_sqrt_price,
            last_sqrt_price,
            state,
            state_counter,
        }
    )
}
</code></pre>



</details>


<a name="0xc8_vault"></a>

# Module `0xc8::vault`



-  [Resource `Vault`](#0xc8_vault_Vault)
-  [Struct `VaultInfo`](#0xc8_vault_VaultInfo)
-  [Struct `AddLiquidityReceipt`](#0xc8_vault_AddLiquidityReceipt)
-  [Struct `SwapStepResult`](#0xc8_vault_SwapStepResult)
-  [Struct `CalculatedSwapResult`](#0xc8_vault_CalculatedSwapResult)
-  [Struct `FlashSwapReceipt`](#0xc8_vault_FlashSwapReceipt)
-  [Constants](#@Constants_0)
-  [Function `create_vault`](#0xc8_vault_create_vault)
-  [Function `set_pause`](#0xc8_vault_set_pause)
-  [Function `init_positions`](#0xc8_vault_init_positions)
-  [Function `open_position`](#0xc8_vault_open_position)
-  [Function `close_position`](#0xc8_vault_close_position)
-  [Function `add_liquidity_internal`](#0xc8_vault_add_liquidity_internal)
-  [Function `add_liquidity`](#0xc8_vault_add_liquidity)
-  [Function `remove_liquidity`](#0xc8_vault_remove_liquidity)
-  [Function `add_liquidity_fix_coin`](#0xc8_vault_add_liquidity_fix_coin)
-  [Function `repay_add_liquidity`](#0xc8_vault_repay_add_liquidity)
-  [Function `calculated_swap_result_amount_out`](#0xc8_vault_calculated_swap_result_amount_out)
-  [Function `calculated_swap_result_is_exceed`](#0xc8_vault_calculated_swap_result_is_exceed)
-  [Function `calculated_swap_result_amount_in`](#0xc8_vault_calculated_swap_result_amount_in)
-  [Function `calculated_swap_result_after_sqrt_price`](#0xc8_vault_calculated_swap_result_after_sqrt_price)
-  [Function `calculate_swap_result_step_results`](#0xc8_vault_calculate_swap_result_step_results)
-  [Function `default_calculated_swap_result`](#0xc8_vault_default_calculated_swap_result)
-  [Function `check_remainer_amount_sub`](#0xc8_vault_check_remainer_amount_sub)
-  [Function `update_swap_result`](#0xc8_vault_update_swap_result)
-  [Function `calculate_swap_result`](#0xc8_vault_calculate_swap_result)
-  [Function `swap`](#0xc8_vault_swap)
-  [Function `repay_flash_swap`](#0xc8_vault_repay_flash_swap)
-  [Function `flash_swap_internal`](#0xc8_vault_flash_swap_internal)
-  [Function `swap_in_vault`](#0xc8_vault_swap_in_vault)
-  [Function `get_position_amounts`](#0xc8_vault_get_position_amounts)
-  [Function `get_position_liquidity`](#0xc8_vault_get_position_liquidity)
-  [Function `get_position_tick_range_and_price`](#0xc8_vault_get_position_tick_range_and_price)
-  [Function `fetch_ticks`](#0xc8_vault_fetch_ticks)
-  [Function `fetch_positions`](#0xc8_vault_fetch_positions)
-  [Function `vault_info`](#0xc8_vault_vault_info)
-  [Function `vault_id`](#0xc8_vault_vault_id)
-  [Function `vault_current_sqrt_price`](#0xc8_vault_vault_current_sqrt_price)
-  [Function `vault_current_tick_index`](#0xc8_vault_vault_current_tick_index)
-  [Function `last_bfc_rebalance_amount`](#0xc8_vault_last_bfc_rebalance_amount)
-  [Function `balances`](#0xc8_vault_balances)
-  [Function `get_liquidity`](#0xc8_vault_get_liquidity)
-  [Function `get_vault_state`](#0xc8_vault_get_vault_state)
-  [Function `get_last_rebalance_vault_state`](#0xc8_vault_get_last_rebalance_vault_state)
-  [Function `bfc_required`](#0xc8_vault_bfc_required)
-  [Function `min_liquidity_rate`](#0xc8_vault_min_liquidity_rate)
-  [Function `max_liquidity_rate`](#0xc8_vault_max_liquidity_rate)
-  [Function `update_state`](#0xc8_vault_update_state)
-  [Function `rebuild_positions_after_clean_liquidities`](#0xc8_vault_rebuild_positions_after_clean_liquidities)
-  [Function `get_liquidity_from_base_point`](#0xc8_vault_get_liquidity_from_base_point)
-  [Function `positions_liquidity_size_balance`](#0xc8_vault_positions_liquidity_size_balance)
-  [Function `rebalance_internal`](#0xc8_vault_rebalance_internal)
-  [Function `rebalance`](#0xc8_vault_rebalance)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="">0x1::type_name</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/curve.md#0x2_curve">0x2::curve</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="clmm_math.md#0xc8_clmm_math">0xc8::clmm_math</a>;
<b>use</b> <a href="event.md#0xc8_event">0xc8::event</a>;
<b>use</b> <a href="i32.md#0xc8_i32">0xc8::i32</a>;
<b>use</b> <a href="math_u128.md#0xc8_math_u128">0xc8::math_u128</a>;
<b>use</b> <a href="math_u64.md#0xc8_math_u64">0xc8::math_u64</a>;
<b>use</b> <a href="option_u64.md#0xc8_option_u64">0xc8::option_u64</a>;
<b>use</b> <a href="position.md#0xc8_position">0xc8::position</a>;
<b>use</b> <a href="tick.md#0xc8_tick">0xc8::tick</a>;
<b>use</b> <a href="tick_math.md#0xc8_tick_math">0xc8::tick_math</a>;
</code></pre>



<a name="0xc8_vault_Vault"></a>

## Resource `Vault`



<pre><code><b>struct</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt; <b>has</b> store, key
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
<code>position_number: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>state: u8</code>
</dt>
<dd>
 0 -- init, equal, 1 -- down, 2 -- up
</dd>
<dt>
<code>last_rebalance_state: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>state_counter: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>max_counter_times: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>last_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>tick_spacing: u32</code>
</dt>
<dd>
 The tick spacing
</dd>
<dt>
<code>spacing_times: u32</code>
</dt>
<dd>
 The tick spacing times
</dd>
<dt>
<code>liquidity: u128</code>
</dt>
<dd>
 The liquidity of current tick index
</dd>
<dt>
<code>current_sqrt_price: u128</code>
</dt>
<dd>
 The current sqrt price
</dd>
<dt>
<code>current_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>
 The current tick index
</dd>
<dt>
<code>tick_manager: <a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a></code>
</dt>
<dd>
 The tick manager
</dd>
<dt>
<code>position_manager: <a href="position.md#0xc8_position_PositionManager">position::PositionManager</a></code>
</dt>
<dd>
 The position manager
</dd>
<dt>
<code>is_pause: bool</code>
</dt>
<dd>
 is the vault pause
</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>
 The vault index
</dd>
<dt>
<code>base_point: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_market_cap: u64</code>
</dt>
<dd>
 stable coin market cap
</dd>
<dt>
<code>last_bfc_rebalance_amount: u64</code>
</dt>
<dd>
 last rebalance bfc amount
</dd>
</dl>


</details>

<a name="0xc8_vault_VaultInfo"></a>

## Struct `VaultInfo`



<pre><code><b>struct</b> <a href="vault.md#0xc8_vault_VaultInfo">VaultInfo</a> <b>has</b> <b>copy</b>, drop
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
<code>position_number: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>state: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>last_rebalance_state: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>state_counter: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>max_counter_times: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>last_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a_balance: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_balance: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a_type: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_type: <a href="_String">ascii::String</a></code>
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
<code>liquidity: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>current_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>current_tick_index: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>is_pause: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>base_point: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_market_cap: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>last_bfc_rebalance_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_vault_AddLiquidityReceipt"></a>

## Struct `AddLiquidityReceipt`

Flash loan resource for add_liquidity


<pre><code><b>struct</b> <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a>&lt;StableCoinType&gt;
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
<code>amount_a: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>amount_b: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_vault_SwapStepResult"></a>

## Struct `SwapStepResult`

The step swap result


<pre><code><b>struct</b> <a href="vault.md#0xc8_vault_SwapStepResult">SwapStepResult</a> <b>has</b> <b>copy</b>, drop, store
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
<code>target_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>current_liquidity: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>current_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
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
<code>remainer_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_vault_CalculatedSwapResult"></a>

## Struct `CalculatedSwapResult`

The calculated swap result


<pre><code><b>struct</b> <a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>vault_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>after_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>is_exceed: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>step_results: <a href="">vector</a>&lt;<a href="vault.md#0xc8_vault_SwapStepResult">vault::SwapStepResult</a>&gt;</code>
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

<a name="0xc8_vault_FlashSwapReceipt"></a>

## Struct `FlashSwapReceipt`

Flash loan resource for swap.
There is no way in Move to pass calldata and make dynamic calls, but a resource can be used for this purpose.
To make the execution into a single transaction, the flash loan function must return a resource
that cannot be copied, cannot be saved, cannot be dropped, or cloned.


<pre><code><b>struct</b> <a href="vault.md#0xc8_vault_FlashSwapReceipt">FlashSwapReceipt</a>&lt;StableCoinType&gt;
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
<code>a2b: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>pay_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_vault_ERR_AMOUNT_INSUFFICIENT"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_AMOUNT_INSUFFICIENT">ERR_AMOUNT_INSUFFICIENT</a>: u64 = 200;
</code></pre>



<a name="0xc8_vault_ERR_AMOUNT_IS_ZERO"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_AMOUNT_IS_ZERO">ERR_AMOUNT_IS_ZERO</a>: u64 = 206;
</code></pre>



<a name="0xc8_vault_ERR_AMOUNT_MISMATCH"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_AMOUNT_MISMATCH">ERR_AMOUNT_MISMATCH</a>: u64 = 210;
</code></pre>



<a name="0xc8_vault_ERR_INVALID_SHAPE_KINDS"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_INVALID_SHAPE_KINDS">ERR_INVALID_SHAPE_KINDS</a>: u64 = 212;
</code></pre>



<a name="0xc8_vault_ERR_LIQUIDITY_DELTA_IS_ZERO"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_LIQUIDITY_DELTA_IS_ZERO">ERR_LIQUIDITY_DELTA_IS_ZERO</a>: u64 = 205;
</code></pre>



<a name="0xc8_vault_ERR_MAX_AMOUNT"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_MAX_AMOUNT">ERR_MAX_AMOUNT</a>: u64 = 201;
</code></pre>



<a name="0xc8_vault_ERR_MAX_LIQUIDITY"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_MAX_LIQUIDITY">ERR_MAX_LIQUIDITY</a>: u64 = 202;
</code></pre>



<a name="0xc8_vault_ERR_PAY_AMOUNT_INVALID"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_PAY_AMOUNT_INVALID">ERR_PAY_AMOUNT_INVALID</a>: u64 = 204;
</code></pre>



<a name="0xc8_vault_ERR_POOL_INVALID"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_POOL_INVALID">ERR_POOL_INVALID</a>: u64 = 203;
</code></pre>



<a name="0xc8_vault_ERR_POOL_IS_PAUSE"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_POOL_IS_PAUSE">ERR_POOL_IS_PAUSE</a>: u64 = 207;
</code></pre>



<a name="0xc8_vault_ERR_POSITIONS_IS_NOT_EMPTY"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_POSITIONS_IS_NOT_EMPTY">ERR_POSITIONS_IS_NOT_EMPTY</a>: u64 = 211;
</code></pre>



<a name="0xc8_vault_ERR_POSITION_LENGTH_MISMATCH"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_POSITION_LENGTH_MISMATCH">ERR_POSITION_LENGTH_MISMATCH</a>: u64 = 213;
</code></pre>



<a name="0xc8_vault_ERR_SQRT_PRICE_LIMIT_INVALID"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_SQRT_PRICE_LIMIT_INVALID">ERR_SQRT_PRICE_LIMIT_INVALID</a>: u64 = 208;
</code></pre>



<a name="0xc8_vault_ERR_TICK_INDEX_OPTION_IS_NONE"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_ERR_TICK_INDEX_OPTION_IS_NONE">ERR_TICK_INDEX_OPTION_IS_NONE</a>: u64 = 209;
</code></pre>



<a name="0xc8_vault_Q64"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_Q64">Q64</a>: u128 = 18446744073709551616;
</code></pre>



<a name="0xc8_vault_SHAPE_DECREMENT_SIZE"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_SHAPE_DECREMENT_SIZE">SHAPE_DECREMENT_SIZE</a>: u8 = 1;
</code></pre>



<a name="0xc8_vault_SHAPE_EQUAL_SIZE"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_SHAPE_EQUAL_SIZE">SHAPE_EQUAL_SIZE</a>: u8 = 0;
</code></pre>



<a name="0xc8_vault_SHAPE_INCREMENT_SIZE"></a>



<pre><code><b>const</b> <a href="vault.md#0xc8_vault_SHAPE_INCREMENT_SIZE">SHAPE_INCREMENT_SIZE</a>: u8 = 2;
</code></pre>



<a name="0xc8_vault_create_vault"></a>

## Function `create_vault`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_create_vault">create_vault</a>&lt;StableCoinType&gt;(_index: u64, _tick_spacing: u32, _spacing_times: u32, _position_number: u32, _initialize_price: u128, _base_point: u64, _max_counter_times: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_create_vault">create_vault</a>&lt;StableCoinType&gt;(
    _index: u64,
    _tick_spacing: u32,
    _spacing_times: u32,
    _position_number: u32,
    _initialize_price: u128,
    _base_point: u64,
    _max_counter_times: u32,
    _ts: u64,
    _ctx: &<b>mut</b> TxContext,
): <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt; {
    <b>let</b> current_tick_index = <a href="tick_math.md#0xc8_tick_math_get_tick_at_sqrt_price">tick_math::get_tick_at_sqrt_price</a>(_initialize_price);
    <b>let</b> valid_index = <a href="tick_math.md#0xc8_tick_math_get_next_valid_tick_index">tick_math::get_next_valid_tick_index</a>(current_tick_index, _tick_spacing);
    <b>let</b> uid = <a href="../../../.././build/Sui/docs/object.md#0x2_object_new">object::new</a>(_ctx);
    <b>let</b> pid = <a href="../../../.././build/Sui/docs/object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&uid);
    <b>let</b> current_sqrt_price = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(valid_index);
    <a href="vault.md#0xc8_vault_Vault">Vault</a> {
        id: uid,
        position_number: _position_number,
        state: 0,
        last_rebalance_state: 0,
        state_counter: _max_counter_times, // init
        last_sqrt_price: current_sqrt_price,
        coin_a: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;StableCoinType&gt;(),
        coin_b: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;(),
        tick_spacing: _tick_spacing,
        spacing_times: _spacing_times,
        liquidity: 0,
        current_sqrt_price,
        current_tick_index: valid_index,
        tick_manager: <a href="tick.md#0xc8_tick_create_tick_manager">tick::create_tick_manager</a>(_tick_spacing, _ts, _ctx),
        position_manager: <a href="position.md#0xc8_position_create_position_manager">position::create_position_manager</a>(pid, _tick_spacing, _ctx),
        is_pause: <b>false</b>,
        index: _index,
        base_point: _base_point,
        max_counter_times: _max_counter_times,
        coin_market_cap: 0,
        last_bfc_rebalance_amount: 0,
    }
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>pragma</b> opaque;
</code></pre>



</details>

<a name="0xc8_vault_set_pause"></a>

## Function `set_pause`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_set_pause">set_pause</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _pause: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_set_pause">set_pause</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _pause: bool,
) {
    _vault.is_pause = _pause;
    event::set_pause(<a href="vault.md#0xc8_vault_vault_id">vault_id</a>(_vault), _pause);
}
</code></pre>



</details>

<a name="0xc8_vault_init_positions"></a>

## Function `init_positions`

open <code>position_number</code> positions


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_init_positions">init_positions</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _spacing_times: u32, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="">vector</a>&lt;<a href="">vector</a>&lt;<a href="i32.md#0xc8_i32_I32">i32::I32</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_init_positions">init_positions</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _spacing_times: u32,
    _ctx: &<b>mut</b> TxContext
): <a href="">vector</a>&lt;<a href="">vector</a>&lt;I32&gt;&gt; {
    <b>assert</b>!(<a href="position.md#0xc8_position_get_total_positions">position::get_total_positions</a>(&_vault.position_manager) == 0, <a href="vault.md#0xc8_vault_ERR_POSITIONS_IS_NOT_EMPTY">ERR_POSITIONS_IS_NOT_EMPTY</a>);
    <b>let</b> ticks = <a href="tick.md#0xc8_tick_get_ticks">tick::get_ticks</a>(
        &_vault.tick_manager,
        _vault.current_tick_index,
        _spacing_times,
        _vault.position_number,
    );
    <b>let</b> index = 0;
    <b>while</b> (index &lt; <a href="_length">vector::length</a>(&ticks)) {
        <b>let</b> current = <a href="_borrow">vector::borrow</a>(&ticks, index);
        <a href="vault.md#0xc8_vault_open_position">open_position</a>(
            _vault,
            *<a href="_borrow">vector::borrow</a>(current, 0),
            *<a href="_borrow">vector::borrow</a>(current, 1),
            _ctx,
        );
        index = index + 1;
    };
    ticks
}
</code></pre>



</details>

<a name="0xc8_vault_open_position"></a>

## Function `open_position`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_open_position">open_position</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_open_position">open_position</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _tick_lower: I32,
    _tick_upper: I32,
    _ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(!_vault.is_pause, <a href="vault.md#0xc8_vault_ERR_POOL_IS_PAUSE">ERR_POOL_IS_PAUSE</a>);
    <a href="position.md#0xc8_position_open_position">position::open_position</a>&lt;StableCoinType&gt;(
        &<b>mut</b> _vault.position_manager,
        _vault.index,
        _tick_lower,
        _tick_upper,
        _ctx,
    );
}
</code></pre>



</details>

<a name="0xc8_vault_close_position"></a>

## Function `close_position`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_close_position">close_position</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_close_position">close_position</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64
)
{
    <b>assert</b>!(!_vault.is_pause, <a href="vault.md#0xc8_vault_ERR_POOL_IS_PAUSE">ERR_POOL_IS_PAUSE</a>);
    <a href="position.md#0xc8_position_close_position">position::close_position</a>(
        &<b>mut</b> _vault.position_manager,
        _index
    );
}
</code></pre>



</details>

<a name="0xc8_vault_add_liquidity_internal"></a>

## Function `add_liquidity_internal`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_add_liquidity_internal">add_liquidity_internal</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64, _use_amount: bool, _liquidity_delta: u128, _amount: u64, _fix_amount_a: bool): <a href="vault.md#0xc8_vault_AddLiquidityReceipt">vault::AddLiquidityReceipt</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_add_liquidity_internal">add_liquidity_internal</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64,
    _use_amount: bool,
    _liquidity_delta: u128,
    _amount: u64,
    _fix_amount_a: bool
): <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a>&lt;StableCoinType&gt; {
    <b>assert</b>!(!_vault.is_pause, <a href="vault.md#0xc8_vault_ERR_POOL_IS_PAUSE">ERR_POOL_IS_PAUSE</a>);
    <b>let</b> expect_vault_id = <a href="vault.md#0xc8_vault_vault_id">vault_id</a>(_vault);
    <b>let</b> mut_position = <a href="position.md#0xc8_position_borrow_mut_position">position::borrow_mut_position</a>(
        &<b>mut</b> _vault.position_manager,
        _index
    );
    <b>let</b> (tick_lower, tick_upper) = <a href="position.md#0xc8_position_get_tick_range">position::get_tick_range</a>(mut_position);
    <b>let</b> _vault_id = <a href="position.md#0xc8_position_get_vault_id">position::get_vault_id</a>(mut_position);
    <b>assert</b>!(_vault_id == expect_vault_id, <a href="vault.md#0xc8_vault_ERR_POOL_INVALID">ERR_POOL_INVALID</a>);
    <b>let</b> liquidity_delta: u128;
    <b>let</b> amount_a: u64;
    <b>let</b> amount_b: u64;
    <b>if</b> (_use_amount) {
        (liquidity_delta, amount_a, amount_b) = <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_by_amount">clmm_math::get_liquidity_by_amount</a>(
            tick_lower,
            tick_upper,
            _vault.current_tick_index,
            _vault.current_sqrt_price,
            _amount,
            _fix_amount_a,
        );
    } <b>else</b> {
        liquidity_delta = _liquidity_delta;
        (amount_a, amount_b) = <a href="clmm_math.md#0xc8_clmm_math_get_amount_by_liquidity">clmm_math::get_amount_by_liquidity</a>(
            tick_lower,
            tick_upper,
            _vault.current_tick_index,
            _vault.current_sqrt_price,
            _liquidity_delta,
            <b>true</b>,
        );
    };
    <b>let</b> liquidity = <a href="position.md#0xc8_position_increase_liquidity">position::increase_liquidity</a>(mut_position, liquidity_delta);
    <a href="tick.md#0xc8_tick_increase_liquidity">tick::increase_liquidity</a>(
        &<b>mut</b> _vault.tick_manager,
        _vault.current_tick_index,
        tick_lower,
        tick_upper,
        liquidity_delta,
    );
    <b>let</b> is_in = <b>false</b>;
    <b>if</b> (<a href="i32.md#0xc8_i32_gte">i32::gte</a>(_vault.current_tick_index, tick_lower)) {
        is_in = <a href="i32.md#0xc8_i32_lt">i32::lt</a>(_vault.current_tick_index, tick_upper);
    };

    <b>if</b> (is_in) {
        <b>assert</b>!(<a href="math_u128.md#0xc8_math_u128_add_check">math_u128::add_check</a>(_vault.liquidity, liquidity), <a href="vault.md#0xc8_vault_ERR_MAX_LIQUIDITY">ERR_MAX_LIQUIDITY</a>);
        _vault.liquidity = _vault.liquidity + liquidity;
    };
    <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a>&lt;StableCoinType&gt; {
        vault_id: _vault_id,
        amount_a,
        amount_b,
    }
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>pragma</b> opaque;
</code></pre>



</details>

<a name="0xc8_vault_add_liquidity"></a>

## Function `add_liquidity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_add_liquidity">add_liquidity</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64, _delta_liquidity: u128): <a href="vault.md#0xc8_vault_AddLiquidityReceipt">vault::AddLiquidityReceipt</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_add_liquidity">add_liquidity</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64,
    _delta_liquidity: u128
): <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a>&lt;StableCoinType&gt; {
    <b>assert</b>!(_delta_liquidity &gt; 0, <a href="vault.md#0xc8_vault_ERR_LIQUIDITY_DELTA_IS_ZERO">ERR_LIQUIDITY_DELTA_IS_ZERO</a>);
    <a href="vault.md#0xc8_vault_add_liquidity_internal">add_liquidity_internal</a>(
        _vault,
        _index,
        <b>false</b>,
        _delta_liquidity,
        0u64,
        <b>false</b>
    )
}
</code></pre>



</details>

<a name="0xc8_vault_remove_liquidity"></a>

## Function `remove_liquidity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_remove_liquidity">remove_liquidity</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64, _delta_liquidity: u128): (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_remove_liquidity">remove_liquidity</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64,
    _delta_liquidity: u128
): (Balance&lt;StableCoinType&gt;, Balance&lt;BFC&gt;) {
    <b>assert</b>!(!_vault.is_pause, <a href="vault.md#0xc8_vault_ERR_POOL_IS_PAUSE">ERR_POOL_IS_PAUSE</a>);
    <b>let</b> expect_vault_id = <a href="vault.md#0xc8_vault_vault_id">vault_id</a>(_vault);
    <b>let</b> mut_position = <a href="position.md#0xc8_position_borrow_mut_position">position::borrow_mut_position</a>(
        &<b>mut</b> _vault.position_manager,
        _index
    );
    <b>let</b> (tick_lower, tick_upper) = <a href="position.md#0xc8_position_get_tick_range">position::get_tick_range</a>(mut_position);
    <b>let</b> _vault_id = <a href="position.md#0xc8_position_get_vault_id">position::get_vault_id</a>(mut_position);
    <b>assert</b>!(_vault_id == expect_vault_id, <a href="vault.md#0xc8_vault_ERR_POOL_INVALID">ERR_POOL_INVALID</a>);
    <b>let</b> _ = <a href="position.md#0xc8_position_decrease_liquidity">position::decrease_liquidity</a>(mut_position, _delta_liquidity);
    <a href="tick.md#0xc8_tick_decrease_liquidity">tick::decrease_liquidity</a>(
        &<b>mut</b> _vault.tick_manager,
        _vault.current_tick_index,
        tick_lower,
        tick_upper,
        _delta_liquidity,
    );
    <b>if</b> (<a href="i32.md#0xc8_i32_gte">i32::gte</a>(_vault.current_tick_index, tick_lower) && <a href="i32.md#0xc8_i32_lt">i32::lt</a>(_vault.current_tick_index, tick_upper)) {
        _vault.liquidity = _vault.liquidity - _delta_liquidity;
    };

    <b>let</b> (amount_a, amount_b) = <a href="clmm_math.md#0xc8_clmm_math_get_amount_by_liquidity">clmm_math::get_amount_by_liquidity</a>(
        tick_lower,
        tick_upper,
        _vault.current_tick_index,
        _vault.current_sqrt_price,
        _delta_liquidity,
        <b>false</b>,
    );
    <b>let</b> balance_a = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> _vault.coin_a, amount_a);
    <b>let</b> balance_b = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> _vault.coin_b, amount_b);
    (balance_a, balance_b)
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>pragma</b> opaque;
</code></pre>



</details>

<a name="0xc8_vault_add_liquidity_fix_coin"></a>

## Function `add_liquidity_fix_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_add_liquidity_fix_coin">add_liquidity_fix_coin</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64, _amount: u64, _fix_amount_a: bool): <a href="vault.md#0xc8_vault_AddLiquidityReceipt">vault::AddLiquidityReceipt</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_add_liquidity_fix_coin">add_liquidity_fix_coin</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64,
    _amount: u64,
    _fix_amount_a: bool
): <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a>&lt;StableCoinType&gt; {
    <b>assert</b>!(_amount &gt; 0, <a href="vault.md#0xc8_vault_ERR_AMOUNT_IS_ZERO">ERR_AMOUNT_IS_ZERO</a>);
    <a href="vault.md#0xc8_vault_add_liquidity_internal">add_liquidity_internal</a>(
        _vault,
        _index,
        <b>true</b>,
        0u128,
        _amount,
        _fix_amount_a
    )
}
</code></pre>



</details>

<a name="0xc8_vault_repay_add_liquidity"></a>

## Function `repay_add_liquidity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_repay_add_liquidity">repay_add_liquidity</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _balance_a: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, _balance_b: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _receipt: <a href="vault.md#0xc8_vault_AddLiquidityReceipt">vault::AddLiquidityReceipt</a>&lt;StableCoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_repay_add_liquidity">repay_add_liquidity</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _balance_a: Balance&lt;StableCoinType&gt;,
    _balance_b: Balance&lt;BFC&gt;,
    _receipt: <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a>&lt;StableCoinType&gt;
)
{
    <b>let</b> <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a> { vault_id, amount_a, amount_b } = _receipt;
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_balance_a) == amount_a, <a href="vault.md#0xc8_vault_ERR_AMOUNT_MISMATCH">ERR_AMOUNT_MISMATCH</a>);
    <b>assert</b>!(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_balance_b) == amount_b, <a href="vault.md#0xc8_vault_ERR_AMOUNT_MISMATCH">ERR_AMOUNT_MISMATCH</a>);
    <b>assert</b>!(vault_id == <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(_vault), <a href="vault.md#0xc8_vault_ERR_POOL_INVALID">ERR_POOL_INVALID</a>);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> _vault.coin_a, _balance_a);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> _vault.coin_b, _balance_b);
}
</code></pre>



</details>

<a name="0xc8_vault_calculated_swap_result_amount_out"></a>

## Function `calculated_swap_result_amount_out`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_amount_out">calculated_swap_result_amount_out</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_amount_out">calculated_swap_result_amount_out</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a>): u64 {
    _calculatedSwapResult.amount_out
}
</code></pre>



</details>

<a name="0xc8_vault_calculated_swap_result_is_exceed"></a>

## Function `calculated_swap_result_is_exceed`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_is_exceed">calculated_swap_result_is_exceed</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_is_exceed">calculated_swap_result_is_exceed</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a>): bool {
    _calculatedSwapResult.is_exceed
}
</code></pre>



</details>

<a name="0xc8_vault_calculated_swap_result_amount_in"></a>

## Function `calculated_swap_result_amount_in`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_amount_in">calculated_swap_result_amount_in</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_amount_in">calculated_swap_result_amount_in</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a>): u64 {
    _calculatedSwapResult.amount_in
}
</code></pre>



</details>

<a name="0xc8_vault_calculated_swap_result_after_sqrt_price"></a>

## Function `calculated_swap_result_after_sqrt_price`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_after_sqrt_price">calculated_swap_result_after_sqrt_price</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculated_swap_result_after_sqrt_price">calculated_swap_result_after_sqrt_price</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a>): u128 {
    _calculatedSwapResult.after_sqrt_price
}
</code></pre>



</details>

<a name="0xc8_vault_calculate_swap_result_step_results"></a>

## Function `calculate_swap_result_step_results`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculate_swap_result_step_results">calculate_swap_result_step_results</a>(_calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>): &<a href="">vector</a>&lt;<a href="vault.md#0xc8_vault_SwapStepResult">vault::SwapStepResult</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculate_swap_result_step_results">calculate_swap_result_step_results</a>(
    _calculatedSwapResult: &<a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a>
): &<a href="">vector</a>&lt;<a href="vault.md#0xc8_vault_SwapStepResult">SwapStepResult</a>&gt; {
    &_calculatedSwapResult.step_results
}
</code></pre>



</details>

<a name="0xc8_vault_default_calculated_swap_result"></a>

## Function `default_calculated_swap_result`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_default_calculated_swap_result">default_calculated_swap_result</a>(): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_default_calculated_swap_result">default_calculated_swap_result</a>(): <a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a> {
    <a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a> {
        amount_in: 0,
        amount_out: 0,
        steps: 0,
        step_results: <a href="_empty">vector::empty</a>(),
        is_exceed: <b>false</b>,
        after_sqrt_price: 0,
        vault_sqrt_price: 0
    }
}
</code></pre>



</details>

<a name="0xc8_vault_check_remainer_amount_sub"></a>

## Function `check_remainer_amount_sub`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_check_remainer_amount_sub">check_remainer_amount_sub</a>(amount: u64, amount_in: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_check_remainer_amount_sub">check_remainer_amount_sub</a>(amount: u64, amount_in: u64): u64 {
    <b>assert</b>!(amount &gt;= amount_in, <a href="vault.md#0xc8_vault_ERR_AMOUNT_INSUFFICIENT">ERR_AMOUNT_INSUFFICIENT</a>);
    amount - amount_in
}
</code></pre>



</details>

<a name="0xc8_vault_update_swap_result"></a>

## Function `update_swap_result`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_update_swap_result">update_swap_result</a>(_swap_result: &<b>mut</b> <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>, _in: u64, _out: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_update_swap_result">update_swap_result</a>(_swap_result: &<b>mut</b> <a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a>, _in: u64, _out: u64) {
    <b>assert</b>!(<a href="math_u64.md#0xc8_math_u64_add_check">math_u64::add_check</a>(_swap_result.amount_in, _in), <a href="vault.md#0xc8_vault_ERR_MAX_AMOUNT">ERR_MAX_AMOUNT</a>);
    <b>assert</b>!(<a href="math_u64.md#0xc8_math_u64_add_check">math_u64::add_check</a>(_swap_result.amount_out, _out), <a href="vault.md#0xc8_vault_ERR_MAX_AMOUNT">ERR_MAX_AMOUNT</a>);
    _swap_result.amount_in = _swap_result.amount_in + _in;
    _swap_result.amount_out = _swap_result.amount_out + _out;
}
</code></pre>



</details>

<a name="0xc8_vault_calculate_swap_result"></a>

## Function `calculate_swap_result`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculate_swap_result">calculate_swap_result</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _a2b: bool, _by_amount_in: bool, _amount: u64): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_calculate_swap_result">calculate_swap_result</a>&lt;StableCoinType&gt;(
    _vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _a2b: bool,
    _by_amount_in: bool,
    _amount: u64,
): <a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a> {
    <b>let</b> swap_result = <a href="vault.md#0xc8_vault_default_calculated_swap_result">default_calculated_swap_result</a>();
    swap_result.vault_sqrt_price = _vault.current_sqrt_price;
    swap_result.after_sqrt_price = _vault.current_sqrt_price;
    <b>let</b> liquidity = _vault.liquidity;
    <b>let</b> current_sqrt_price = _vault.current_sqrt_price;
    <b>let</b> remainer_amount = _amount;
    <b>let</b> tick_index = _vault.current_tick_index;
    <b>let</b> start_score = <a href="tick.md#0xc8_tick_first_score_for_swap">tick::first_score_for_swap</a>(
        &_vault.tick_manager,
        tick_index,
        _a2b,
    );
    <b>while</b> (remainer_amount &gt; 0) {
        <b>if</b> (<a href="option_u64.md#0xc8_option_u64_is_none">option_u64::is_none</a>(&start_score)) {
            swap_result.is_exceed = <b>true</b>;
            <b>break</b>
        };
        // get next <a href="tick.md#0xc8_tick">tick</a>
        <b>let</b> (next_tick, next_score) = <a href="tick.md#0xc8_tick_borrow_tick_for_swap">tick::borrow_tick_for_swap</a>(
            &_vault.tick_manager,
            <a href="option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(&start_score),
            _a2b,
        );
        start_score = next_score;

        <b>let</b> target_sqrt_price = <a href="tick.md#0xc8_tick_sqrt_price">tick::sqrt_price</a>(next_tick);
        <b>let</b> (amount_in, amount_out, next_sqrt_price) = <a href="clmm_math.md#0xc8_clmm_math_compute_swap_step">clmm_math::compute_swap_step</a>(
            current_sqrt_price,
            target_sqrt_price,
            liquidity,
            remainer_amount,
            _a2b,
            _by_amount_in,
        );

        <b>if</b> (amount_in != 0) {
            <b>if</b> (_by_amount_in) {
                remainer_amount = <a href="vault.md#0xc8_vault_check_remainer_amount_sub">check_remainer_amount_sub</a>(remainer_amount, amount_in);
            } <b>else</b> {
                remainer_amount = <a href="vault.md#0xc8_vault_check_remainer_amount_sub">check_remainer_amount_sub</a>(remainer_amount, amount_out);
            };
            <a href="vault.md#0xc8_vault_update_swap_result">update_swap_result</a>(&<b>mut</b> swap_result, amount_in, amount_out);
        };
        <a href="_push_back">vector::push_back</a>(&<b>mut</b> swap_result.step_results, <a href="vault.md#0xc8_vault_SwapStepResult">SwapStepResult</a> {
            current_sqrt_price,
            target_sqrt_price,
            current_liquidity: liquidity,
            amount_in,
            amount_out,
            remainer_amount,
            current_tick_index: tick_index,
        });
        <b>if</b> (target_sqrt_price == next_sqrt_price) {
            current_sqrt_price = target_sqrt_price;
            liquidity = <a href="tick.md#0xc8_tick_cross_by_tick">tick::cross_by_tick</a>(next_tick, _a2b, liquidity);
            tick_index = <a href="tick.md#0xc8_tick_tick_index">tick::tick_index</a>(next_tick);
        } <b>else</b> {
            current_sqrt_price = next_sqrt_price
        };
        swap_result.steps = swap_result.steps + 1;
        swap_result.after_sqrt_price = current_sqrt_price;
    };
    swap_result
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>pragma</b> opaque;
</code></pre>



</details>

<a name="0xc8_vault_swap"></a>

## Function `swap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_swap">swap</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _coin_a: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, _coin_b: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _a2b: bool, _by_amount_in: bool, _amount: u64, _amount_limit: u64, _sqrt_price_limit: u128, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_swap">swap</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _coin_a: Coin&lt;StableCoinType&gt;,
    _coin_b: Coin&lt;BFC&gt;,
    _a2b: bool,
    _by_amount_in: bool,
    _amount: u64,
    _amount_limit: u64,
    _sqrt_price_limit: u128,
    _ctx: &<b>mut</b> TxContext
): (Balance&lt;StableCoinType&gt;, Balance&lt;BFC&gt;) {
    <b>assert</b>!(!_vault.is_pause, <a href="vault.md#0xc8_vault_ERR_POOL_IS_PAUSE">ERR_POOL_IS_PAUSE</a>);
    <b>let</b> (
        receive_a,
        receive_b,
        flash_receipt
    ) = <a href="vault.md#0xc8_vault_flash_swap_internal">flash_swap_internal</a>&lt;StableCoinType&gt;(
        _vault,
        _a2b,
        _by_amount_in,
        _amount,
        _sqrt_price_limit
    );

    <b>let</b> pay_amount = flash_receipt.pay_amount;
    <b>let</b> pay_coin_a;
    <b>let</b> pay_coin_b;

    <b>if</b> (_a2b) {
        pay_coin_a = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_split">coin::split</a>(&<b>mut</b> _coin_a, pay_amount, _ctx));
        pay_coin_b = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;();
    } <b>else</b> {
        pay_coin_a = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;StableCoinType&gt;();
        pay_coin_b = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_split">coin::split</a>(&<b>mut</b> _coin_b, pay_amount, _ctx));
    };

    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_join">coin::join</a>(&<b>mut</b> _coin_a, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(receive_a, _ctx));
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_join">coin::join</a>(&<b>mut</b> _coin_b, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(receive_b, _ctx));

    <a href="vault.md#0xc8_vault_repay_flash_swap">repay_flash_swap</a>&lt;StableCoinType&gt;(
        _vault,
        pay_coin_a,
        pay_coin_b,
        flash_receipt
    );
    (<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(_coin_a), <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_into_balance">coin::into_balance</a>(_coin_b))
}
</code></pre>



</details>

<a name="0xc8_vault_repay_flash_swap"></a>

## Function `repay_flash_swap`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_repay_flash_swap">repay_flash_swap</a>&lt;StableCoinType&gt;(<a href="vault.md#0xc8_vault">vault</a>: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, balance_a: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, balance_b: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, receipt: <a href="vault.md#0xc8_vault_FlashSwapReceipt">vault::FlashSwapReceipt</a>&lt;StableCoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_repay_flash_swap">repay_flash_swap</a>&lt;StableCoinType&gt;(
    <a href="vault.md#0xc8_vault">vault</a>: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    balance_a: Balance&lt;StableCoinType&gt;,
    balance_b: Balance&lt;BFC&gt;,
    receipt: <a href="vault.md#0xc8_vault_FlashSwapReceipt">FlashSwapReceipt</a>&lt;StableCoinType&gt;
) {
    <b>let</b> <a href="vault.md#0xc8_vault_FlashSwapReceipt">FlashSwapReceipt</a>&lt;StableCoinType&gt; {
        vault_id: _vault_id,
        a2b,
        pay_amount
    } = receipt;

    <b>assert</b>!(_vault_id == <a href="vault.md#0xc8_vault_vault_id">vault_id</a>(<a href="vault.md#0xc8_vault">vault</a>), <a href="vault.md#0xc8_vault_ERR_POOL_INVALID">ERR_POOL_INVALID</a>);
    <b>assert</b>!(pay_amount &gt; 0, <a href="vault.md#0xc8_vault_ERR_PAY_AMOUNT_INVALID">ERR_PAY_AMOUNT_INVALID</a>);

    <b>if</b> (a2b) {
        <b>assert</b>!(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&balance_a) == pay_amount, <a href="vault.md#0xc8_vault_ERR_PAY_AMOUNT_INVALID">ERR_PAY_AMOUNT_INVALID</a>);
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="vault.md#0xc8_vault">vault</a>.coin_a, balance_a);
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(balance_b);
    } <b>else</b> {
        <b>assert</b>!(<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&balance_b) == pay_amount, <a href="vault.md#0xc8_vault_ERR_PAY_AMOUNT_INVALID">ERR_PAY_AMOUNT_INVALID</a>);
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> <a href="vault.md#0xc8_vault">vault</a>.coin_b, balance_b);
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(balance_a);
    };
}
</code></pre>



</details>

<a name="0xc8_vault_flash_swap_internal"></a>

## Function `flash_swap_internal`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_flash_swap_internal">flash_swap_internal</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _a2b: bool, _by_amount_in: bool, _amount: u64, _sqrt_price_limit: u128): (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="vault.md#0xc8_vault_FlashSwapReceipt">vault::FlashSwapReceipt</a>&lt;StableCoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_flash_swap_internal">flash_swap_internal</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _a2b: bool,
    _by_amount_in: bool,
    _amount: u64,
    _sqrt_price_limit: u128
): (Balance&lt;StableCoinType&gt;, Balance&lt;BFC&gt;, <a href="vault.md#0xc8_vault_FlashSwapReceipt">FlashSwapReceipt</a>&lt;StableCoinType&gt;)
{
    <b>let</b> min_price = <a href="tick_math.md#0xc8_tick_math_min_sqrt_price">tick_math::min_sqrt_price</a>();
    <b>let</b> max_price = <a href="tick_math.md#0xc8_tick_math_max_sqrt_price">tick_math::max_sqrt_price</a>();
    <b>if</b> (_a2b) {
        <b>assert</b>!(
            min_price &lt;= _sqrt_price_limit && _sqrt_price_limit &lt; _vault.current_sqrt_price,
            <a href="vault.md#0xc8_vault_ERR_SQRT_PRICE_LIMIT_INVALID">ERR_SQRT_PRICE_LIMIT_INVALID</a>
        );
    } <b>else</b> {
        <b>assert</b>!(
            _vault.current_sqrt_price &lt; _sqrt_price_limit && _sqrt_price_limit &lt;= max_price,
            <a href="vault.md#0xc8_vault_ERR_SQRT_PRICE_LIMIT_INVALID">ERR_SQRT_PRICE_LIMIT_INVALID</a>
        );
    };
    <b>let</b> before_sqrt_price = _vault.current_sqrt_price;
    <b>let</b> swap_res = <a href="vault.md#0xc8_vault_swap_in_vault">swap_in_vault</a>(_vault, _a2b, _by_amount_in, _sqrt_price_limit, _amount);
    <b>let</b> balance_a_ret;
    <b>let</b> balance_b_ret;
    <b>let</b> coin_type_in: String;
    <b>let</b> coin_type_out: String;

    <b>if</b> (_a2b) {
        <b>if</b> (_vault.coin_market_cap &gt;= swap_res.amount_in) {
            _vault.coin_market_cap = _vault.coin_market_cap - swap_res.amount_in;
        } <b>else</b> {
            _vault.coin_market_cap = 0;
        };

        balance_b_ret = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>&lt;BFC&gt;(&<b>mut</b> _vault.coin_b, swap_res.amount_out);
        balance_a_ret = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;StableCoinType&gt;();
        coin_type_in = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;StableCoinType&gt;());
        coin_type_out = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BFC&gt;());
    } <b>else</b> {
        _vault.coin_market_cap = _vault.coin_market_cap + swap_res.amount_out;
        balance_a_ret = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>&lt;StableCoinType&gt;(&<b>mut</b> _vault.coin_a, swap_res.amount_out);
        balance_b_ret = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;();
        coin_type_in = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BFC&gt;());
        coin_type_out = <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;StableCoinType&gt;());
    };
    event::swap(
        <a href="vault.md#0xc8_vault_vault_id">vault_id</a>(_vault),
        _a2b,
        coin_type_in,
        coin_type_out,
        swap_res.amount_in,
        swap_res.amount_out,
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&balance_a_ret),
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&balance_b_ret),
        before_sqrt_price,
        _vault.current_sqrt_price,
        swap_res.steps
    );
    (balance_a_ret, balance_b_ret, <a href="vault.md#0xc8_vault_FlashSwapReceipt">FlashSwapReceipt</a>&lt;StableCoinType&gt; {
        vault_id: <a href="vault.md#0xc8_vault_vault_id">vault_id</a>(_vault),
        a2b: _a2b,
        pay_amount: swap_res.amount_in
    })
}
</code></pre>



</details>

<a name="0xc8_vault_swap_in_vault"></a>

## Function `swap_in_vault`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_swap_in_vault">swap_in_vault</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _a2b: bool, _by_amount_in: bool, _sqrt_price_limit: u128, _amount: u64): <a href="vault.md#0xc8_vault_CalculatedSwapResult">vault::CalculatedSwapResult</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_swap_in_vault">swap_in_vault</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _a2b: bool,
    _by_amount_in: bool,
    _sqrt_price_limit: u128,
    _amount: u64,
): <a href="vault.md#0xc8_vault_CalculatedSwapResult">CalculatedSwapResult</a>
{
    <b>let</b> swap_result = <a href="vault.md#0xc8_vault_default_calculated_swap_result">default_calculated_swap_result</a>();
    <b>let</b> next_score = <a href="tick.md#0xc8_tick_first_score_for_swap">tick::first_score_for_swap</a>(&_vault.tick_manager, _vault.current_tick_index, _a2b);
    <b>let</b> remaining_amount = _amount;
    swap_result.vault_sqrt_price = _vault.current_sqrt_price;
    <b>while</b> (remaining_amount &gt; 0 && _vault.current_sqrt_price != _sqrt_price_limit) {
        <b>assert</b>!(!<a href="option_u64.md#0xc8_option_u64_is_none">option_u64::is_none</a>(&next_score), <a href="vault.md#0xc8_vault_ERR_TICK_INDEX_OPTION_IS_NONE">ERR_TICK_INDEX_OPTION_IS_NONE</a>);
        <b>let</b> (<a href="tick.md#0xc8_tick">tick</a>, tick_score) = <a href="tick.md#0xc8_tick_borrow_tick_for_swap">tick::borrow_tick_for_swap</a>(
            &_vault.tick_manager,
            <a href="option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(&next_score),
            _a2b
        );
        next_score = tick_score;
        <b>let</b> next_tick_index = <a href="tick.md#0xc8_tick_tick_index">tick::tick_index</a>(<a href="tick.md#0xc8_tick">tick</a>);
        <b>let</b> next_tick_sqrt_price = <a href="tick.md#0xc8_tick_sqrt_price">tick::sqrt_price</a>(<a href="tick.md#0xc8_tick">tick</a>);
        <b>let</b> target_sqrt_price = <b>if</b> (_a2b) {
            <a href="math_u128.md#0xc8_math_u128_max">math_u128::max</a>(_sqrt_price_limit, next_tick_sqrt_price)
        } <b>else</b> {
            <a href="math_u128.md#0xc8_math_u128_min">math_u128::min</a>(_sqrt_price_limit, next_tick_sqrt_price)
        };
        <b>let</b> (amount_in, amount_out, next_sqrt_price) = <a href="clmm_math.md#0xc8_clmm_math_compute_swap_step">clmm_math::compute_swap_step</a>(
            _vault.current_sqrt_price,
            target_sqrt_price,
            _vault.liquidity,
            remaining_amount,
            _a2b,
            _by_amount_in
        );
        <b>if</b> (amount_in != 0) {
            <b>if</b> (_by_amount_in) {
                remaining_amount = <a href="vault.md#0xc8_vault_check_remainer_amount_sub">check_remainer_amount_sub</a>(remaining_amount, amount_in);
            } <b>else</b> {
                remaining_amount = <a href="vault.md#0xc8_vault_check_remainer_amount_sub">check_remainer_amount_sub</a>(remaining_amount, amount_out);
            };
            <a href="vault.md#0xc8_vault_update_swap_result">update_swap_result</a>(&<b>mut</b> swap_result, amount_in, amount_out);
        };
        <b>if</b> (next_sqrt_price == next_tick_sqrt_price) {
            _vault.current_sqrt_price = target_sqrt_price;
            <b>let</b> next_tick = <b>if</b> (_a2b) {
                <a href="i32.md#0xc8_i32_sub">i32::sub</a>(next_tick_index, <a href="i32.md#0xc8_i32_from_u32">i32::from_u32</a>(1))
            } <b>else</b> {
                next_tick_index
            };
            _vault.current_tick_index = next_tick;
            _vault.liquidity = <a href="tick.md#0xc8_tick_cross_by_swap">tick::cross_by_swap</a>(
                &<b>mut</b> _vault.tick_manager,
                next_tick_index,
                _a2b,
                _vault.liquidity
            );
        } <b>else</b> {
            <b>if</b> (_vault.current_sqrt_price != next_tick_sqrt_price) {
                _vault.current_sqrt_price = next_sqrt_price;
                _vault.current_tick_index = <a href="tick_math.md#0xc8_tick_math_get_tick_at_sqrt_price">tick_math::get_tick_at_sqrt_price</a>(next_sqrt_price);
            }
        };
    };
    swap_result
}
</code></pre>



</details>

<a name="0xc8_vault_get_position_amounts"></a>

## Function `get_position_amounts`

Read Functions
Calculate the position's amount_a/amount_b
Params
- <code><a href="vault.md#0xc8_vault">vault</a></code> The clmm vault object.
- <code>_index</code> The index of position.
Returns
- <code>amount_a</code> The amount of <code>StableCoinType</code>
- <code>amount_b</code> The amount of <code>BFC</code>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_position_amounts">get_position_amounts</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64, _round_up: bool): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_position_amounts">get_position_amounts</a>&lt;StableCoinType&gt;(
    _vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64,
    _round_up: bool
): (u64, u64) {
    <b>let</b> <a href="position.md#0xc8_position">position</a> = <a href="position.md#0xc8_position_borrow_position">position::borrow_position</a>(
        &_vault.position_manager,
        _index
    );
    <b>let</b> (tick_lower, tick_upper) = <a href="position.md#0xc8_position_get_tick_range">position::get_tick_range</a>(<a href="position.md#0xc8_position">position</a>);
    <a href="clmm_math.md#0xc8_clmm_math_get_amount_by_liquidity">clmm_math::get_amount_by_liquidity</a>(
        tick_lower,
        tick_upper,
        _vault.current_tick_index,
        _vault.current_sqrt_price,
        <a href="position.md#0xc8_position_get_liquidity">position::get_liquidity</a>(<a href="position.md#0xc8_position">position</a>),
        _round_up
    )
}
</code></pre>



</details>

<a name="0xc8_vault_get_position_liquidity"></a>

## Function `get_position_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_position_liquidity">get_position_liquidity</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_position_liquidity">get_position_liquidity</a>&lt;StableCoinType&gt;(
    _vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64
): u128
{
    <b>let</b> <a href="position.md#0xc8_position">position</a> = <a href="position.md#0xc8_position_borrow_position">position::borrow_position</a>(
        &_vault.position_manager,
        _index
    );
    <a href="position.md#0xc8_position_get_liquidity">position::get_liquidity</a>(<a href="position.md#0xc8_position">position</a>)
}
</code></pre>



</details>

<a name="0xc8_vault_get_position_tick_range_and_price"></a>

## Function `get_position_tick_range_and_price`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_position_tick_range_and_price">get_position_tick_range_and_price</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _index: u64): (<a href="i32.md#0xc8_i32_I32">i32::I32</a>, <a href="i32.md#0xc8_i32_I32">i32::I32</a>, u128, u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_position_tick_range_and_price">get_position_tick_range_and_price</a>&lt;StableCoinType&gt;(
    _vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _index: u64
): (I32, I32, u128, u128)
{
    <b>let</b> <a href="position.md#0xc8_position">position</a> = <a href="position.md#0xc8_position_borrow_position">position::borrow_position</a>(
        &_vault.position_manager,
        _index
    );
    <b>let</b> (tick_lower_index, tick_upper_index) = <a href="position.md#0xc8_position_get_tick_range">position::get_tick_range</a>(<a href="position.md#0xc8_position">position</a>);
    <b>let</b> price_lower = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(tick_lower_index);
    <b>let</b> price_upper = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(tick_upper_index);
    (tick_lower_index, tick_upper_index, price_lower, price_upper)
}
</code></pre>



</details>

<a name="0xc8_vault_fetch_ticks"></a>

## Function `fetch_ticks`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_fetch_ticks">fetch_ticks</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): <a href="">vector</a>&lt;<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_fetch_ticks">fetch_ticks</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): <a href="">vector</a>&lt;Tick&gt; {
    <a href="tick.md#0xc8_tick_fetch_ticks">tick::fetch_ticks</a>(&_vault.tick_manager)
}
</code></pre>



</details>

<a name="0xc8_vault_fetch_positions"></a>

## Function `fetch_positions`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_fetch_positions">fetch_positions</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): <a href="">vector</a>&lt;<a href="position.md#0xc8_position_Position">position::Position</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_fetch_positions">fetch_positions</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): <a href="">vector</a>&lt;Position&gt; {
    <a href="position.md#0xc8_position_fetch_positions">position::fetch_positions</a>(&_vault.position_manager, 1, (_vault.position_number <b>as</b> u64))
}
</code></pre>



</details>

<a name="0xc8_vault_vault_info"></a>

## Function `vault_info`

vault info


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_info">vault_info</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): <a href="vault.md#0xc8_vault_VaultInfo">vault::VaultInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_info">vault_info</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): <a href="vault.md#0xc8_vault_VaultInfo">VaultInfo</a> {
    <a href="vault.md#0xc8_vault_VaultInfo">VaultInfo</a> {
        vault_id: <a href="vault.md#0xc8_vault_vault_id">vault_id</a>(_vault),
        position_number: _vault.position_number,
        state: _vault.state,
        last_rebalance_state: _vault.last_rebalance_state,
        state_counter: _vault.state_counter,
        max_counter_times: _vault.max_counter_times,
        last_sqrt_price: _vault.last_sqrt_price,
        coin_a_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_vault.coin_a),
        coin_b_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_vault.coin_b),
        coin_a_type: <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;StableCoinType&gt;()),
        coin_b_type: <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;BFC&gt;()),
        tick_spacing: _vault.tick_spacing,
        spacing_times: _vault.spacing_times,
        liquidity: _vault.liquidity,
        current_sqrt_price: _vault.current_sqrt_price,
        current_tick_index: <a href="i32.md#0xc8_i32_abs_u32">i32::abs_u32</a>(_vault.current_tick_index),
        is_pause: _vault.is_pause,
        index: _vault.index,
        base_point: _vault.base_point,
        coin_market_cap: _vault.coin_market_cap,
        last_bfc_rebalance_amount: _vault.last_bfc_rebalance_amount,
    }
}
</code></pre>



</details>

<a name="0xc8_vault_vault_id"></a>

## Function `vault_id`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_id">vault_id</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_id">vault_id</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): ID {
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_id">object::id</a>(_vault)
}
</code></pre>



</details>

<a name="0xc8_vault_vault_current_sqrt_price"></a>

## Function `vault_current_sqrt_price`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_current_sqrt_price">vault_current_sqrt_price</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_current_sqrt_price">vault_current_sqrt_price</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): u128 {
    _vault.current_sqrt_price
}
</code></pre>



</details>

<a name="0xc8_vault_vault_current_tick_index"></a>

## Function `vault_current_tick_index`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_current_tick_index">vault_current_tick_index</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): <a href="i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_vault_current_tick_index">vault_current_tick_index</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): I32 {
    _vault.current_tick_index
}
</code></pre>



</details>

<a name="0xc8_vault_last_bfc_rebalance_amount"></a>

## Function `last_bfc_rebalance_amount`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_last_bfc_rebalance_amount">last_bfc_rebalance_amount</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_last_bfc_rebalance_amount">last_bfc_rebalance_amount</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): u64 {
    _vault.last_bfc_rebalance_amount
}
</code></pre>



</details>

<a name="0xc8_vault_balances"></a>

## Function `balances`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_balances">balances</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_balances">balances</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): (u64, u64) {
    (
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>&lt;StableCoinType&gt;(&_vault.coin_a),
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>&lt;BFC&gt;(&_vault.coin_b)
    )
}
</code></pre>



</details>

<a name="0xc8_vault_get_liquidity"></a>

## Function `get_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_liquidity">get_liquidity</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_liquidity">get_liquidity</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): u128 {
    _vault.liquidity
}
</code></pre>



</details>

<a name="0xc8_vault_get_vault_state"></a>

## Function `get_vault_state`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_vault_state">get_vault_state</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_vault_state">get_vault_state</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): u8
{
    _vault.state
}
</code></pre>



</details>

<a name="0xc8_vault_get_last_rebalance_vault_state"></a>

## Function `get_last_rebalance_vault_state`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_get_last_rebalance_vault_state">get_last_rebalance_vault_state</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b>  <a href="vault.md#0xc8_vault_get_last_rebalance_vault_state">get_last_rebalance_vault_state</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;): u8 {
    _vault.last_rebalance_state
}
</code></pre>



</details>

<a name="0xc8_vault_bfc_required"></a>

## Function `bfc_required`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_bfc_required">bfc_required</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _treasury_total_bfc_supply: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_bfc_required">bfc_required</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;, _treasury_total_bfc_supply: u64): u64 {
    <b>let</b> curve_dx_q64 = curve_dx((_vault.coin_market_cap <b>as</b> u128), (_treasury_total_bfc_supply <b>as</b> u128));
    <b>let</b> base_point_amount = (((_vault.base_point <b>as</b> u128) * (<a href="vault.md#0xc8_vault_Q64">Q64</a> + curve_dx_q64) / <a href="vault.md#0xc8_vault_Q64">Q64</a>) <b>as</b> u64);
    <b>let</b> total_required_amount = (_vault.position_number <b>as</b> u64) * base_point_amount;
    <b>if</b> (total_required_amount &gt; <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_vault.coin_b)) {
        total_required_amount - <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_vault.coin_b)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="0xc8_vault_min_liquidity_rate"></a>

## Function `min_liquidity_rate`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_min_liquidity_rate">min_liquidity_rate</a>(): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_min_liquidity_rate">min_liquidity_rate</a>(): u128 {
    6
}
</code></pre>



</details>

<a name="0xc8_vault_max_liquidity_rate"></a>

## Function `max_liquidity_rate`



<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_max_liquidity_rate">max_liquidity_rate</a>(): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0xc8_vault_max_liquidity_rate">max_liquidity_rate</a>(): u128 {
    14
}
</code></pre>



</details>

<a name="0xc8_vault_update_state"></a>

## Function `update_state`

Rebalance
State checker


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_update_state">update_state</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_update_state">update_state</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;) {
    <b>let</b> price = _vault.current_sqrt_price;
    <b>let</b> last_price = _vault.last_sqrt_price;
    <b>if</b> (price &lt; last_price) {
        // down
        <b>if</b> (_vault.state == <a href="vault.md#0xc8_vault_SHAPE_INCREMENT_SIZE">SHAPE_INCREMENT_SIZE</a>) {
            _vault.state_counter = _vault.state_counter + 1;
        } <b>else</b> {
            <b>if</b> (_vault.state == <a href="vault.md#0xc8_vault_SHAPE_EQUAL_SIZE">SHAPE_EQUAL_SIZE</a>) {
                // reset counter = 1  & set state = down
                _vault.state_counter = 1;
                _vault.state = <a href="vault.md#0xc8_vault_SHAPE_INCREMENT_SIZE">SHAPE_INCREMENT_SIZE</a>;
            } <b>else</b> {
                _vault.state = <a href="vault.md#0xc8_vault_SHAPE_EQUAL_SIZE">SHAPE_EQUAL_SIZE</a>;
                _vault.state_counter = <b>if</b> (_vault.last_rebalance_state == <a href="vault.md#0xc8_vault_SHAPE_DECREMENT_SIZE">SHAPE_DECREMENT_SIZE</a>) {
                    _vault.max_counter_times
                } <b>else</b> {
                    0
                };
            }
        }
    } <b>else</b> <b>if</b> (price &gt; last_price) {
        // up
        <b>if</b> (_vault.state == <a href="vault.md#0xc8_vault_SHAPE_DECREMENT_SIZE">SHAPE_DECREMENT_SIZE</a>) {
            _vault.state_counter = _vault.state_counter + 1;
        } <b>else</b> {
            <b>if</b> (_vault.state == <a href="vault.md#0xc8_vault_SHAPE_EQUAL_SIZE">SHAPE_EQUAL_SIZE</a>) {
                // reset counter = 1  & set state = up
                _vault.state_counter = 1;
                _vault.state = <a href="vault.md#0xc8_vault_SHAPE_DECREMENT_SIZE">SHAPE_DECREMENT_SIZE</a>;
            } <b>else</b> {
                _vault.state = <a href="vault.md#0xc8_vault_SHAPE_EQUAL_SIZE">SHAPE_EQUAL_SIZE</a>;
                _vault.state_counter = <b>if</b> (_vault.last_rebalance_state == <a href="vault.md#0xc8_vault_SHAPE_INCREMENT_SIZE">SHAPE_INCREMENT_SIZE</a>) {
                    _vault.max_counter_times
                } <b>else</b> {
                    0
                };
            }
        }
    };

    _vault.last_sqrt_price = price;
    event::update_state(
        <a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;StableCoinType&gt;()),
        price,
        last_price,
        _vault.state,
        _vault.state_counter,
    );
}
</code></pre>



</details>

<a name="0xc8_vault_rebuild_positions_after_clean_liquidities"></a>

## Function `rebuild_positions_after_clean_liquidities`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_rebuild_positions_after_clean_liquidities">rebuild_positions_after_clean_liquidities</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="">vector</a>&lt;<a href="">vector</a>&lt;<a href="i32.md#0xc8_i32_I32">i32::I32</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_rebuild_positions_after_clean_liquidities">rebuild_positions_after_clean_liquidities</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _ctx: &<b>mut</b> TxContext
): (Balance&lt;StableCoinType&gt;, Balance&lt;BFC&gt;, <a href="">vector</a>&lt;<a href="">vector</a>&lt;I32&gt;&gt;)
{
    <b>let</b> position_index = 1u64;
    <b>let</b> balance0 = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;StableCoinType&gt;();
    <b>let</b> balance1 = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_zero">balance::zero</a>&lt;BFC&gt;();
    <b>while</b> (position_index &lt;= (_vault.position_number <b>as</b> u64)) {
        <b>let</b> <a href="position.md#0xc8_position">position</a> = <a href="position.md#0xc8_position_borrow_mut_position">position::borrow_mut_position</a>(&<b>mut</b> _vault.position_manager, position_index);
        <b>let</b> liquidity_delta = <a href="position.md#0xc8_position_get_liquidity">position::get_liquidity</a>(<a href="position.md#0xc8_position">position</a>);
        <b>if</b> (liquidity_delta != 0) {
            <b>let</b> (_balance0, _balance1) = <a href="vault.md#0xc8_vault_remove_liquidity">remove_liquidity</a>(_vault, position_index, liquidity_delta);
            <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> balance0, _balance0);
            <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> balance1, _balance1);
        };
        <a href="position.md#0xc8_position_close_position">position::close_position</a>(&<b>mut</b> _vault.position_manager, position_index);
        position_index = position_index + 1;
    };
    <a href="tick.md#0xc8_tick_rebuild_ticks">tick::rebuild_ticks</a>(&<b>mut</b> _vault.tick_manager, _ctx);
    <b>let</b> spacing_times = _vault.spacing_times;
    <b>let</b> ticks = <a href="vault.md#0xc8_vault_init_positions">init_positions</a>(_vault, spacing_times, _ctx);
    (balance0, balance1, ticks)
}
</code></pre>



</details>

<a name="0xc8_vault_get_liquidity_from_base_point"></a>

## Function `get_liquidity_from_base_point`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_get_liquidity_from_base_point">get_liquidity_from_base_point</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _ticks: &<a href="">vector</a>&lt;<a href="">vector</a>&lt;<a href="i32.md#0xc8_i32_I32">i32::I32</a>&gt;&gt;, _amount: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_get_liquidity_from_base_point">get_liquidity_from_base_point</a>&lt;StableCoinType&gt;(
    _vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _ticks: &<a href="">vector</a>&lt;<a href="">vector</a>&lt;I32&gt;&gt;,
    _amount: u64,
): u128
{
    <b>let</b> middle_tick = <a href="_borrow">vector::borrow</a>(_ticks, (_vault.position_number / 2 <b>as</b> u64));
    <b>let</b> (tick_lower_index, tick_upper_index) = (<a href="_borrow">vector::borrow</a>(middle_tick, 0), <a href="_borrow">vector::borrow</a>(middle_tick, 1));
    <b>let</b> (liquidity, _, _) = <a href="clmm_math.md#0xc8_clmm_math_get_liquidity_by_amount">clmm_math::get_liquidity_by_amount</a>(
        *tick_lower_index,
        *tick_upper_index,
        _vault.current_tick_index,
        _vault.current_sqrt_price,
        _amount,
        <b>false</b>
    );
    liquidity
}
</code></pre>



</details>

<a name="0xc8_vault_positions_liquidity_size_balance"></a>

## Function `positions_liquidity_size_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_positions_liquidity_size_balance">positions_liquidity_size_balance</a>&lt;StableCoinType&gt;(_vault: &<a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _ticks: &<a href="">vector</a>&lt;<a href="">vector</a>&lt;<a href="i32.md#0xc8_i32_I32">i32::I32</a>&gt;&gt;, _shape: u8, _treasury_total_bfc_supply: u64): <a href="">vector</a>&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_positions_liquidity_size_balance">positions_liquidity_size_balance</a>&lt;StableCoinType&gt;(
    _vault: &<a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _ticks: &<a href="">vector</a>&lt;<a href="">vector</a>&lt;I32&gt;&gt;,
    _shape: u8,
    _treasury_total_bfc_supply: u64,
): <a href="">vector</a>&lt;u128&gt; {
    // base point <a href="position.md#0xc8_position">position</a> liquidity
    <b>let</b> curve_dx_q64 = curve_dx((_vault.coin_market_cap <b>as</b> u128), (_treasury_total_bfc_supply <b>as</b> u128));
    <b>let</b> base_point_amount = (((_vault.base_point <b>as</b> u128) * (<a href="vault.md#0xc8_vault_Q64">Q64</a> + curve_dx_q64) / <a href="vault.md#0xc8_vault_Q64">Q64</a>) <b>as</b> u64);
    <b>let</b> liquidity = <a href="vault.md#0xc8_vault_get_liquidity_from_base_point">get_liquidity_from_base_point</a>(_vault, _ticks, base_point_amount);
    <b>let</b> liquidities = <a href="_empty">vector::empty</a>&lt;u128&gt;();
    <b>let</b> index: u128 ;
    <b>let</b> length: u128;
    <b>if</b> (_shape == <a href="vault.md#0xc8_vault_SHAPE_EQUAL_SIZE">SHAPE_EQUAL_SIZE</a>) {
        index = 0;
        length = (_vault.position_number <b>as</b> u128);
        <b>while</b> (index &lt; length) {
            <a href="_push_back">vector::push_back</a>(&<b>mut</b> liquidities, liquidity);
            index = index + 1;
        };
    } <b>else</b> <b>if</b> (_shape == <a href="vault.md#0xc8_vault_SHAPE_INCREMENT_SIZE">SHAPE_INCREMENT_SIZE</a>) {
        index = <a href="vault.md#0xc8_vault_min_liquidity_rate">min_liquidity_rate</a>();
        length = <a href="vault.md#0xc8_vault_max_liquidity_rate">max_liquidity_rate</a>();
        <b>let</b> base_liquidity_rate = index + (_vault.position_number <b>as</b> u128) / 2;
        <b>while</b> (index &lt;= length) {
            <a href="_push_back">vector::push_back</a>(&<b>mut</b> liquidities, liquidity * index / base_liquidity_rate);
            index = index + 1;
        };
    } <b>else</b> {
        <b>assert</b>!(_shape == <a href="vault.md#0xc8_vault_SHAPE_DECREMENT_SIZE">SHAPE_DECREMENT_SIZE</a>, <a href="vault.md#0xc8_vault_ERR_INVALID_SHAPE_KINDS">ERR_INVALID_SHAPE_KINDS</a>);
        index = <a href="vault.md#0xc8_vault_max_liquidity_rate">max_liquidity_rate</a>();
        length = <a href="vault.md#0xc8_vault_min_liquidity_rate">min_liquidity_rate</a>();
        <b>let</b> base_liquidity_rate = index - (_vault.position_number <b>as</b> u128) / 2;
        <b>while</b> (index &gt;= length) {
            <a href="_push_back">vector::push_back</a>(&<b>mut</b> liquidities, liquidity * index / base_liquidity_rate);
            index = index - 1;
        };
    };
    liquidities
}
</code></pre>



</details>

<a name="0xc8_vault_rebalance_internal"></a>

## Function `rebalance_internal`



<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_rebalance_internal">rebalance_internal</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _bfc_balance: &<b>mut</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _supply: &<b>mut</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _balance0: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;, _balance1: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _liquidities: <a href="">vector</a>&lt;u128&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0xc8_vault_rebalance_internal">rebalance_internal</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _bfc_balance: &<b>mut</b> Balance&lt;BFC&gt;,
    _supply: &<b>mut</b> Supply&lt;StableCoinType&gt;,
    _balance0: Balance&lt;StableCoinType&gt;,
    _balance1: Balance&lt;BFC&gt;,
    _liquidities: <a href="">vector</a>&lt;u128&gt;
)
{
    // <b>return</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(_bfc_balance, _balance1);
    <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_decrease_supply">balance::decrease_supply</a>(_supply, _balance0);

    <b>let</b> index = 0u64;
    <b>let</b> length = <a href="_length">vector::length</a>(&_liquidities);
    <b>let</b> position_length = <a href="position.md#0xc8_position_get_total_positions">position::get_total_positions</a>(&_vault.position_manager);
    <b>assert</b>!(length == position_length, <a href="vault.md#0xc8_vault_ERR_POSITION_LENGTH_MISMATCH">ERR_POSITION_LENGTH_MISMATCH</a>);
    <b>while</b> (index &lt; length) {
        <b>let</b> receipt = <a href="vault.md#0xc8_vault_add_liquidity">add_liquidity</a>(
            _vault,
            index + 1, // <a href="position.md#0xc8_position">position</a> index
            *<a href="_borrow">vector::borrow</a>(&_liquidities, index)
        );
        <b>let</b> <a href="vault.md#0xc8_vault_AddLiquidityReceipt">AddLiquidityReceipt</a> { vault_id: _, amount_a, amount_b } = receipt;
        <b>if</b> (amount_a &gt; 0) {
            <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> _vault.coin_a, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_increase_supply">balance::increase_supply</a>(_supply, amount_a));
        };
        <b>if</b> (amount_b &gt; 0) {
            <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_join">balance::join</a>(&<b>mut</b> _vault.coin_b, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(_bfc_balance, amount_b));
        };
        index = index + 1;
    };
    event::rebalance(<a href="_into_string">type_name::into_string</a>(<a href="_get">type_name::get</a>&lt;StableCoinType&gt;()));
}
</code></pre>



</details>

<a name="0xc8_vault_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_rebalance">rebalance</a>&lt;StableCoinType&gt;(_vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">vault::Vault</a>&lt;StableCoinType&gt;, _bfc_balance: &<b>mut</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, _supply: &<b>mut</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;StableCoinType&gt;, _treasury_total_bfc_supply: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0xc8_vault_rebalance">rebalance</a>&lt;StableCoinType&gt;(
    _vault: &<b>mut</b> <a href="vault.md#0xc8_vault_Vault">Vault</a>&lt;StableCoinType&gt;,
    _bfc_balance: &<b>mut</b> Balance&lt;BFC&gt;,
    _supply: &<b>mut</b> Supply&lt;StableCoinType&gt;,
    _treasury_total_bfc_supply: u64,
    _ctx: &<b>mut</b> TxContext
): u64 {
    <b>if</b> (_vault.state_counter &gt;= _vault.max_counter_times) {
        // reset state counter
        _vault.state_counter = 0;
    } <b>else</b> {
        <b>let</b> should_break = <b>false</b>;
        <b>let</b> edge_position = <a href="position.md#0xc8_position_borrow_position">position::borrow_position</a>(&_vault.position_manager, 1);
        <b>let</b> (_, tu) = <a href="position.md#0xc8_position_get_tick_range">position::get_tick_range</a>(edge_position);
        <b>if</b> (<a href="i32.md#0xc8_i32_lte">i32::lte</a>(_vault.current_tick_index, tu)) {
            should_break = <b>true</b>;
        };
        <b>if</b> (!should_break) {
            edge_position = <a href="position.md#0xc8_position_borrow_position">position::borrow_position</a>(&_vault.position_manager, (_vault.position_number <b>as</b> u64));
            <b>let</b> (tl, _) = <a href="position.md#0xc8_position_get_tick_range">position::get_tick_range</a>(edge_position);
            <b>if</b> (<a href="i32.md#0xc8_i32_gte">i32::gte</a>(_vault.current_tick_index, tl)) {
                should_break = <b>true</b>;
            };
        };
        <b>if</b> (!should_break) {
            <b>return</b> _vault.last_bfc_rebalance_amount
        } <b>else</b> {
            _vault.state = <a href="vault.md#0xc8_vault_SHAPE_EQUAL_SIZE">SHAPE_EQUAL_SIZE</a>;
            // reset state counter
            _vault.state_counter = 0;
        };
    };
    <b>let</b> (
        balance0,
        balance1,
        ticks
    ) = <a href="vault.md#0xc8_vault_rebuild_positions_after_clean_liquidities">rebuild_positions_after_clean_liquidities</a>(_vault, _ctx);
    <b>let</b> shape = _vault.state;
    <b>let</b> liquidities = <a href="vault.md#0xc8_vault_positions_liquidity_size_balance">positions_liquidity_size_balance</a>(
        _vault,
        &ticks,
        shape,
        _treasury_total_bfc_supply
    );
    <a href="vault.md#0xc8_vault_rebalance_internal">rebalance_internal</a>(
        _vault,
        _bfc_balance,
        _supply,
        balance0,
        balance1,
        liquidities
    );
    _vault.last_rebalance_state = _vault.state;
    _vault.last_bfc_rebalance_amount = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&_vault.coin_b);
    _vault.last_bfc_rebalance_amount
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>


<a name="0xc8_position"></a>

# Module `0xc8::position`



-  [Struct `PositionManager`](#0xc8_position_PositionManager)
-  [Struct `Position`](#0xc8_position_Position)
-  [Constants](#@Constants_0)
-  [Function `create_position_manager`](#0xc8_position_create_position_manager)
-  [Function `get_vault_id`](#0xc8_position_get_vault_id)
-  [Function `get_position_id`](#0xc8_position_get_position_id)
-  [Function `is_empty`](#0xc8_position_is_empty)
-  [Function `is_position_exist`](#0xc8_position_is_position_exist)
-  [Function `get_liquidity`](#0xc8_position_get_liquidity)
-  [Function `fetch_positions`](#0xc8_position_fetch_positions)
-  [Function `borrow_mut_position`](#0xc8_position_borrow_mut_position)
-  [Function `borrow_position`](#0xc8_position_borrow_position)
-  [Function `check_position_tick_range`](#0xc8_position_check_position_tick_range)
-  [Function `open_position`](#0xc8_position_open_position)
-  [Function `close_position`](#0xc8_position_close_position)
-  [Function `increase_liquidity`](#0xc8_position_increase_liquidity)
-  [Function `decrease_liquidity`](#0xc8_position_decrease_liquidity)
-  [Function `destory`](#0xc8_position_destory)
-  [Function `generate_position_key`](#0xc8_position_generate_position_key)
-  [Function `new_position_name`](#0xc8_position_new_position_name)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="">0x1::string</a>;
<b>use</b> <a href="">0x1::type_name</a>;
<b>use</b> <a href="">0x1::vector</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/hash.md#0x2_hash">0x2::hash</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="i32.md#0xc8_i32">0xc8::i32</a>;
<b>use</b> <a href="linked_table.md#0xc8_linked_table">0xc8::linked_table</a>;
<b>use</b> <a href="math_u128.md#0xc8_math_u128">0xc8::math_u128</a>;
<b>use</b> <a href="tick_math.md#0xc8_tick_math">0xc8::tick_math</a>;
<b>use</b> <a href="utils.md#0xc8_utils">0xc8::utils</a>;
</code></pre>



<a name="0xc8_position_PositionManager"></a>

## Struct `PositionManager`



<pre><code><b>struct</b> <a href="position.md#0xc8_position_PositionManager">PositionManager</a> <b>has</b> store
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
<code>tick_spacing: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>position_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>positions: <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;<a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, <a href="position.md#0xc8_position_Position">position::Position</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_position_Position"></a>

## Struct `Position`



<pre><code><b>struct</b> <a href="position.md#0xc8_position_Position">Position</a> <b>has</b> <b>copy</b>, drop, store
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
<code>position_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_a: <a href="_TypeName">type_name::TypeName</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_b: <a href="_TypeName">type_name::TypeName</a></code>
</dt>
<dd>

</dd>
<dt>
<code>name: <a href="_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tick_lower_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tick_upper_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_position_ERR_INVALID_LIMIT"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_INVALID_LIMIT">ERR_INVALID_LIMIT</a>: u64 = 305;
</code></pre>



<a name="0xc8_position_ERR_INVALID_VECTOR_LENGTH"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_INVALID_VECTOR_LENGTH">ERR_INVALID_VECTOR_LENGTH</a>: u64 = 306;
</code></pre>



<a name="0xc8_position_ERR_POSITION_INFO_EMPTY"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_POSITION_INFO_EMPTY">ERR_POSITION_INFO_EMPTY</a>: u64 = 308;
</code></pre>



<a name="0xc8_position_ERR_POSITION_INFO_NOT_EMPTY"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_POSITION_INFO_NOT_EMPTY">ERR_POSITION_INFO_NOT_EMPTY</a>: u64 = 307;
</code></pre>



<a name="0xc8_position_ERR_POSITION_INSUFFICIENT_LIQUIDITY"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_POSITION_INSUFFICIENT_LIQUIDITY">ERR_POSITION_INSUFFICIENT_LIQUIDITY</a>: u64 = 309;
</code></pre>



<a name="0xc8_position_ERR_TICK_INVALID_RANGE"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_TICK_INVALID_RANGE">ERR_TICK_INVALID_RANGE</a>: u64 = 303;
</code></pre>



<a name="0xc8_position_ERR_TICK_INVALID_VALUE"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_TICK_INVALID_VALUE">ERR_TICK_INVALID_VALUE</a>: u64 = 304;
</code></pre>



<a name="0xc8_position_ERR_TICK_LOWER_TOO_SMALL"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_TICK_LOWER_TOO_SMALL">ERR_TICK_LOWER_TOO_SMALL</a>: u64 = 301;
</code></pre>



<a name="0xc8_position_ERR_TICK_SPACING_INVALID_RANGE"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_TICK_SPACING_INVALID_RANGE">ERR_TICK_SPACING_INVALID_RANGE</a>: u64 = 300;
</code></pre>



<a name="0xc8_position_ERR_TICK_UPPER_TOO_LARGE"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_TICK_UPPER_TOO_LARGE">ERR_TICK_UPPER_TOO_LARGE</a>: u64 = 302;
</code></pre>



<a name="0xc8_position_ERR_U128_ADD_CHECK_FAILED"></a>



<pre><code><b>const</b> <a href="position.md#0xc8_position_ERR_U128_ADD_CHECK_FAILED">ERR_U128_ADD_CHECK_FAILED</a>: u64 = 310;
</code></pre>



<a name="0xc8_position_create_position_manager"></a>

## Function `create_position_manager`

create PositionManager


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_create_position_manager">create_position_manager</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, _tick_spacing: u32, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="position.md#0xc8_position_PositionManager">position::PositionManager</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_create_position_manager">create_position_manager</a>(
    vault_id: ID,
    _tick_spacing: u32,
    _ctx: &<b>mut</b> TxContext,
): <a href="position.md#0xc8_position_PositionManager">PositionManager</a> {
    <a href="position.md#0xc8_position_PositionManager">PositionManager</a> {
        vault_id,
        tick_spacing: _tick_spacing,
        position_index: 0,
        positions: <a href="linked_table.md#0xc8_linked_table_new">linked_table::new</a>&lt;ID, <a href="position.md#0xc8_position_Position">Position</a>&gt;(_ctx),
    }
}
</code></pre>



</details>

<a name="0xc8_position_get_vault_id"></a>

## Function `get_vault_id`

position info


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_get_vault_id">get_vault_id</a>(_position: &<a href="position.md#0xc8_position_Position">position::Position</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_get_vault_id">get_vault_id</a>(_position: &<a href="position.md#0xc8_position_Position">Position</a>): ID {
    _position.vault_id
}
</code></pre>



</details>

<a name="0xc8_position_get_position_id"></a>

## Function `get_position_id`



<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_get_position_id">get_position_id</a>(_position: &<a href="position.md#0xc8_position_Position">position::Position</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_get_position_id">get_position_id</a>(_position: &<a href="position.md#0xc8_position_Position">Position</a>): ID {
    _position.position_id
}
</code></pre>



</details>

<a name="0xc8_position_is_empty"></a>

## Function `is_empty`



<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_is_empty">is_empty</a>(_position: &<a href="position.md#0xc8_position_Position">position::Position</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_is_empty">is_empty</a>(_position: &<a href="position.md#0xc8_position_Position">Position</a>): bool {
    _position.liquidity == 0
}
</code></pre>



</details>

<a name="0xc8_position_is_position_exist"></a>

## Function `is_position_exist`



<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_is_position_exist">is_position_exist</a>(_manager: &<a href="position.md#0xc8_position_PositionManager">position::PositionManager</a>, _position_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_is_position_exist">is_position_exist</a>(_manager: &<a href="position.md#0xc8_position_PositionManager">PositionManager</a>, _position_id: ID): bool {
    <a href="linked_table.md#0xc8_linked_table_contains">linked_table::contains</a>(&_manager.positions, _position_id)
}
</code></pre>



</details>

<a name="0xc8_position_get_liquidity"></a>

## Function `get_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_get_liquidity">get_liquidity</a>(_position: &<a href="position.md#0xc8_position_Position">position::Position</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_get_liquidity">get_liquidity</a>(_position: &<a href="position.md#0xc8_position_Position">Position</a>): u128 {
    _position.liquidity
}
</code></pre>



</details>

<a name="0xc8_position_fetch_positions"></a>

## Function `fetch_positions`



<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_fetch_positions">fetch_positions</a>(_manager: &<a href="position.md#0xc8_position_PositionManager">position::PositionManager</a>, _start: <a href="">vector</a>&lt;<a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>&gt;, _limit: u64): <a href="">vector</a>&lt;<a href="position.md#0xc8_position_Position">position::Position</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_fetch_positions">fetch_positions</a>(
    _manager: &<a href="position.md#0xc8_position_PositionManager">PositionManager</a>,
    _start: <a href="">vector</a>&lt;ID&gt;,
    _limit: u64
): <a href="">vector</a>&lt;<a href="position.md#0xc8_position_Position">Position</a>&gt; {
    <b>assert</b>!(_limit &gt; 0, <a href="position.md#0xc8_position_ERR_INVALID_LIMIT">ERR_INVALID_LIMIT</a>);
    <b>let</b> len = <a href="_length">vector::length</a>(&_start);
    <b>assert</b>!(len &gt; 0, <a href="position.md#0xc8_position_ERR_INVALID_VECTOR_LENGTH">ERR_INVALID_VECTOR_LENGTH</a>);
    <b>let</b> ret = <a href="_empty">vector::empty</a>&lt;<a href="position.md#0xc8_position_Position">Position</a>&gt;();
    <b>let</b> idx = 0;
    <b>while</b> (idx &lt; len) {
        <b>let</b> positions = <a href="linked_table.md#0xc8_linked_table_fetch">linked_table::fetch</a>(
            &_manager.positions,
            *<a href="_borrow">vector::borrow</a>&lt;ID&gt;(&_start, idx),
            _limit
        );
        <b>if</b> (<a href="_length">vector::length</a>(&positions) &gt; 0) {
            <a href="_append">vector::append</a>(&<b>mut</b> ret, positions);
        };
        idx = idx + 1;
    };
    ret
}
</code></pre>



</details>

<a name="0xc8_position_borrow_mut_position"></a>

## Function `borrow_mut_position`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_borrow_mut_position">borrow_mut_position</a>(_manager: &<b>mut</b> <a href="position.md#0xc8_position_PositionManager">position::PositionManager</a>, _tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>): &<b>mut</b> <a href="position.md#0xc8_position_Position">position::Position</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_borrow_mut_position">borrow_mut_position</a>(
    _manager: &<b>mut</b> <a href="position.md#0xc8_position_PositionManager">PositionManager</a>,
    _tick_lower: I32,
    _tick_upper: I32
): &<b>mut</b> <a href="position.md#0xc8_position_Position">Position</a> {
    <b>let</b> position_id = <a href="position.md#0xc8_position_generate_position_key">generate_position_key</a>(_manager.vault_id, _tick_lower, _tick_upper);
    <a href="linked_table.md#0xc8_linked_table_borrow_mut">linked_table::borrow_mut</a>(&<b>mut</b> _manager.positions, position_id)
}
</code></pre>



</details>

<a name="0xc8_position_borrow_position"></a>

## Function `borrow_position`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_borrow_position">borrow_position</a>(_manager: &<a href="position.md#0xc8_position_PositionManager">position::PositionManager</a>, _tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>): &<a href="position.md#0xc8_position_Position">position::Position</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_borrow_position">borrow_position</a>(
    _manager: & <a href="position.md#0xc8_position_PositionManager">PositionManager</a>,
    _tick_lower: I32,
    _tick_upper: I32
): &<a href="position.md#0xc8_position_Position">Position</a> {
    <b>let</b> position_id = <a href="position.md#0xc8_position_generate_position_key">generate_position_key</a>(_manager.vault_id, _tick_lower, _tick_upper);
    <a href="linked_table.md#0xc8_linked_table_borrow">linked_table::borrow</a>(&_manager.positions, position_id)
}
</code></pre>



</details>

<a name="0xc8_position_check_position_tick_range"></a>

## Function `check_position_tick_range`

check tick


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_check_position_tick_range">check_position_tick_range</a>(_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_spacing: u32)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="position.md#0xc8_position_check_position_tick_range">check_position_tick_range</a>(_lower: I32, _upper: I32, _tick_spacing: u32) {
    <b>let</b> tick_spacing = <a href="i32.md#0xc8_i32_from_u32">i32::from_u32</a>(_tick_spacing);
    <b>assert</b>!(<a href="i32.md#0xc8_i32_gt">i32::gt</a>(tick_spacing, <a href="tick_math.md#0xc8_tick_math_max_tick">tick_math::max_tick</a>()), <a href="position.md#0xc8_position_ERR_TICK_SPACING_INVALID_RANGE">ERR_TICK_SPACING_INVALID_RANGE</a>);
    <b>assert</b>!(<a href="i32.md#0xc8_i32_lt">i32::lt</a>(tick_spacing, <a href="tick_math.md#0xc8_tick_math_min_tick">tick_math::min_tick</a>()), <a href="position.md#0xc8_position_ERR_TICK_SPACING_INVALID_RANGE">ERR_TICK_SPACING_INVALID_RANGE</a>);
    <b>assert</b>!(<a href="i32.md#0xc8_i32_lt">i32::lt</a>(_lower, _upper), <a href="position.md#0xc8_position_ERR_TICK_INVALID_RANGE">ERR_TICK_INVALID_RANGE</a>);
    <b>assert</b>!(<a href="i32.md#0xc8_i32_gte">i32::gte</a>(_lower, <a href="tick_math.md#0xc8_tick_math_min_tick">tick_math::min_tick</a>()), <a href="position.md#0xc8_position_ERR_TICK_LOWER_TOO_SMALL">ERR_TICK_LOWER_TOO_SMALL</a>);
    <b>assert</b>!(<a href="i32.md#0xc8_i32_lte">i32::lte</a>(_upper, <a href="tick_math.md#0xc8_tick_math_max_tick">tick_math::max_tick</a>()), <a href="position.md#0xc8_position_ERR_TICK_UPPER_TOO_LARGE">ERR_TICK_UPPER_TOO_LARGE</a>);
    <b>assert</b>!(<a href="i32.md#0xc8_i32_eq">i32::eq</a>(<a href="i32.md#0xc8_i32_zero">i32::zero</a>(), <a href="i32.md#0xc8_i32_mod">i32::mod</a>(_lower, tick_spacing)), <a href="position.md#0xc8_position_ERR_TICK_INVALID_VALUE">ERR_TICK_INVALID_VALUE</a>);
    <b>assert</b>!(<a href="i32.md#0xc8_i32_eq">i32::eq</a>(<a href="i32.md#0xc8_i32_zero">i32::zero</a>(), <a href="i32.md#0xc8_i32_mod">i32::mod</a>(_upper, tick_spacing)), <a href="position.md#0xc8_position_ERR_TICK_INVALID_VALUE">ERR_TICK_INVALID_VALUE</a>);
}
</code></pre>



</details>

<a name="0xc8_position_open_position"></a>

## Function `open_position`

open / close position


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_open_position">open_position</a>&lt;CoinTypeA, CoinTypeB&gt;(_position_manager: &<b>mut</b> <a href="position.md#0xc8_position_PositionManager">position::PositionManager</a>, _vault_index: u64, _tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_open_position">open_position</a>&lt;CoinTypeA, CoinTypeB&gt;(
    _position_manager: &<b>mut</b> <a href="position.md#0xc8_position_PositionManager">PositionManager</a>,
    _vault_index: u64,
    _tick_lower: I32,
    _tick_upper: I32,
    _ctx: &<b>mut</b> TxContext
): ID
{
    <b>let</b> tick_spacing = _position_manager.tick_spacing;
    <a href="position.md#0xc8_position_check_position_tick_range">check_position_tick_range</a>(_tick_lower, _tick_upper, tick_spacing);
    _position_manager.position_index = _position_manager.position_index + 1;
    <b>let</b> <a href="position.md#0xc8_position">position</a> = <a href="position.md#0xc8_position_Position">Position</a> {
        position_id: <a href="position.md#0xc8_position_generate_position_key">generate_position_key</a>(_position_manager.vault_id, _tick_lower, _tick_lower),
        vault_id: _position_manager.vault_id,
        index: _position_manager.position_index,
        coin_type_a: <a href="_get">type_name::get</a>&lt;CoinTypeA&gt;(),
        coin_type_b: <a href="_get">type_name::get</a>&lt;CoinTypeB&gt;(),
        name: <a href="position.md#0xc8_position_new_position_name">new_position_name</a>(_position_manager.position_index, _vault_index),
        tick_lower_index: _tick_lower,
        tick_upper_index: _tick_upper,
        liquidity: 0
    };
    <a href="linked_table.md#0xc8_linked_table_push_back">linked_table::push_back</a>(&<b>mut</b> _position_manager.positions, <a href="position.md#0xc8_position">position</a>.position_id, <a href="position.md#0xc8_position">position</a>);
    <a href="position.md#0xc8_position">position</a>.position_id
}
</code></pre>



</details>

<a name="0xc8_position_close_position"></a>

## Function `close_position`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_close_position">close_position</a>(_manager: &<b>mut</b> <a href="position.md#0xc8_position_PositionManager">position::PositionManager</a>, _tick_lower: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper: <a href="i32.md#0xc8_i32_I32">i32::I32</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_close_position">close_position</a>(
    _manager: &<b>mut</b> <a href="position.md#0xc8_position_PositionManager">PositionManager</a>,
    _tick_lower: I32,
    _tick_upper: I32
): ID
{
    <b>let</b> position_id = <a href="position.md#0xc8_position_generate_position_key">generate_position_key</a>(_manager.vault_id, _tick_lower, _tick_upper);
    <b>let</b> <a href="position.md#0xc8_position">position</a> = <a href="linked_table.md#0xc8_linked_table_remove">linked_table::remove</a>(&<b>mut</b> _manager.positions, position_id);
    <b>assert</b>!(!<a href="position.md#0xc8_position_is_empty">is_empty</a>(&<a href="position.md#0xc8_position">position</a>), <a href="position.md#0xc8_position_ERR_POSITION_INFO_NOT_EMPTY">ERR_POSITION_INFO_NOT_EMPTY</a>);
    <a href="position.md#0xc8_position_destory">destory</a>(<a href="position.md#0xc8_position">position</a>);
    position_id
}
</code></pre>



</details>

<a name="0xc8_position_increase_liquidity"></a>

## Function `increase_liquidity`

add/remove liquidity


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_increase_liquidity">increase_liquidity</a>(<a href="position.md#0xc8_position">position</a>: &<b>mut</b> <a href="position.md#0xc8_position_Position">position::Position</a>, _liquidity_delta: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_increase_liquidity">increase_liquidity</a>(<a href="position.md#0xc8_position">position</a>: &<b>mut</b> <a href="position.md#0xc8_position_Position">Position</a>, _liquidity_delta: u128): u128 {
    <b>assert</b>!(!<a href="position.md#0xc8_position_is_empty">is_empty</a>(<a href="position.md#0xc8_position">position</a>), <a href="position.md#0xc8_position_ERR_POSITION_INFO_EMPTY">ERR_POSITION_INFO_EMPTY</a>);
    <b>assert</b>!(<a href="math_u128.md#0xc8_math_u128_add_check">math_u128::add_check</a>(_liquidity_delta, <a href="position.md#0xc8_position">position</a>.liquidity), <a href="position.md#0xc8_position_ERR_U128_ADD_CHECK_FAILED">ERR_U128_ADD_CHECK_FAILED</a>);
    <a href="position.md#0xc8_position">position</a>.liquidity = <a href="position.md#0xc8_position">position</a>.liquidity + _liquidity_delta;
    <a href="position.md#0xc8_position">position</a>.liquidity
}
</code></pre>



</details>

<a name="0xc8_position_decrease_liquidity"></a>

## Function `decrease_liquidity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_decrease_liquidity">decrease_liquidity</a>(<a href="position.md#0xc8_position">position</a>: &<b>mut</b> <a href="position.md#0xc8_position_Position">position::Position</a>, _liquidity_delta: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="position.md#0xc8_position_decrease_liquidity">decrease_liquidity</a>(<a href="position.md#0xc8_position">position</a>: &<b>mut</b> <a href="position.md#0xc8_position_Position">Position</a>, _liquidity_delta: u128): u128 {
    <b>assert</b>!(!<a href="position.md#0xc8_position_is_empty">is_empty</a>(<a href="position.md#0xc8_position">position</a>), <a href="position.md#0xc8_position_ERR_POSITION_INFO_EMPTY">ERR_POSITION_INFO_EMPTY</a>);
    <b>if</b> (_liquidity_delta == 0) {
        <b>return</b> <a href="position.md#0xc8_position">position</a>.liquidity
    };
    <b>assert</b>!(<a href="position.md#0xc8_position">position</a>.liquidity &lt; _liquidity_delta, <a href="position.md#0xc8_position_ERR_POSITION_INSUFFICIENT_LIQUIDITY">ERR_POSITION_INSUFFICIENT_LIQUIDITY</a>);
    <a href="position.md#0xc8_position">position</a>.liquidity = <a href="position.md#0xc8_position">position</a>.liquidity - _liquidity_delta;
    <a href="position.md#0xc8_position">position</a>.liquidity
}
</code></pre>



</details>

<a name="0xc8_position_destory"></a>

## Function `destory`

private fun


<pre><code><b>fun</b> <a href="position.md#0xc8_position_destory">destory</a>(_position: <a href="position.md#0xc8_position_Position">position::Position</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="position.md#0xc8_position_destory">destory</a>(_position: <a href="position.md#0xc8_position_Position">Position</a>) {}
</code></pre>



</details>

<a name="0xc8_position_generate_position_key"></a>

## Function `generate_position_key`



<pre><code><b>fun</b> <a href="position.md#0xc8_position_generate_position_key">generate_position_key</a>(_vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, _tick_lower_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>): <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="position.md#0xc8_position_generate_position_key">generate_position_key</a>(
    _vault_id: ID,
    _tick_lower_index: I32,
    _tick_upper_index: I32
): ID
{
    <b>let</b> bytes = <a href="_empty">vector::empty</a>&lt;u8&gt;();
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, <a href="../../../.././build/Sui/docs/object.md#0x2_object_id_to_bytes">object::id_to_bytes</a>(&_vault_id));
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, b"-");
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, b"[");
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, <a href="i32.md#0xc8_i32_get_bytes">i32::get_bytes</a>(_tick_lower_index));
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, b",");
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, <a href="i32.md#0xc8_i32_get_bytes">i32::get_bytes</a>(_tick_upper_index));
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, b"]");
    <a href="../../../.././build/Sui/docs/object.md#0x2_object_id_from_bytes">object::id_from_bytes</a>(sui::hash::blake2b256(&bytes))
}
</code></pre>



</details>

<a name="0xc8_position_new_position_name"></a>

## Function `new_position_name`



<pre><code><b>fun</b> <a href="position.md#0xc8_position_new_position_name">new_position_name</a>(_position_index: u64, _vault_index: u64): <a href="_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="position.md#0xc8_position_new_position_name">new_position_name</a>(_position_index: u64, _vault_index: u64): String {
    <b>let</b> lp_name = <a href="_utf8">string::utf8</a>(b"");
    <a href="_append_utf8">string::append_utf8</a>(&<b>mut</b> lp_name, b"OpenBlock LP | Pool");
    <a href="_append_utf8">string::append_utf8</a>(&<b>mut</b> lp_name, b"-");
    <a href="_append_utf8">string::append_utf8</a>(&<b>mut</b> lp_name, into_bytes(to_string((_vault_index <b>as</b> u128))));
    <a href="_append_utf8">string::append_utf8</a>(&<b>mut</b> lp_name, b"-");
    <a href="_append_utf8">string::append_utf8</a>(&<b>mut</b> lp_name, into_bytes(to_string((_position_index <b>as</b> u128))));
    lp_name
}
</code></pre>



</details>


<a name="0xc8_tick"></a>

# Module `0xc8::tick`



-  [Struct `TickManager`](#0xc8_tick_TickManager)
-  [Struct `Tick`](#0xc8_tick_Tick)
-  [Constants](#@Constants_0)
-  [Function `create_tick_manager`](#0xc8_tick_create_tick_manager)
-  [Function `sqrt_price`](#0xc8_tick_sqrt_price)
-  [Function `liquidity_net`](#0xc8_tick_liquidity_net)
-  [Function `tick_index`](#0xc8_tick_tick_index)
-  [Function `tick_spacing`](#0xc8_tick_tick_spacing)
-  [Function `fetch_ticks`](#0xc8_tick_fetch_ticks)
-  [Function `default`](#0xc8_tick_default)
-  [Function `tick_score`](#0xc8_tick_tick_score)
-  [Function `update_by_liquidity`](#0xc8_tick_update_by_liquidity)
-  [Function `increase_liquidity`](#0xc8_tick_increase_liquidity)
-  [Function `decrease_liquidity`](#0xc8_tick_decrease_liquidity)
-  [Function `try_borrow_tick`](#0xc8_tick_try_borrow_tick)
-  [Function `borrow_tick_for_swap`](#0xc8_tick_borrow_tick_for_swap)
-  [Function `cross_by_swap`](#0xc8_tick_cross_by_swap)
-  [Function `first_score_for_swap`](#0xc8_tick_first_score_for_swap)
-  [Function `get_ticks`](#0xc8_tick_get_ticks)


<pre><code><b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="i128.md#0xc8_i128">0xc8::i128</a>;
<b>use</b> <a href="i32.md#0xc8_i32">0xc8::i32</a>;
<b>use</b> <a href="math_u128.md#0xc8_math_u128">0xc8::math_u128</a>;
<b>use</b> <a href="option_u64.md#0xc8_option_u64">0xc8::option_u64</a>;
<b>use</b> <a href="skip_list.md#0xc8_skip_list">0xc8::skip_list</a>;
<b>use</b> <a href="tick_math.md#0xc8_tick_math">0xc8::tick_math</a>;
</code></pre>



<a name="0xc8_tick_TickManager"></a>

## Struct `TickManager`



<pre><code><b>struct</b> <a href="tick.md#0xc8_tick_TickManager">TickManager</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>tick_spacing: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>ticks: <a href="skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_tick_Tick"></a>

## Struct `Tick`



<pre><code><b>struct</b> <a href="tick.md#0xc8_tick_Tick">Tick</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>index: <a href="i32.md#0xc8_i32_I32">i32::I32</a></code>
</dt>
<dd>

</dd>
<dt>
<code>sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_net: <a href="i128.md#0xc8_i128_I128">i128::I128</a></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_gross: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_tick_ERR_TICK_EXCEED_TWICE_MAXIMUM"></a>



<pre><code><b>const</b> <a href="tick.md#0xc8_tick_ERR_TICK_EXCEED_TWICE_MAXIMUM">ERR_TICK_EXCEED_TWICE_MAXIMUM</a>: u64 = 400;
</code></pre>



<a name="0xc8_tick_ERR_TICK_EXCEED_U128_MAXIMUM"></a>



<pre><code><b>const</b> <a href="tick.md#0xc8_tick_ERR_TICK_EXCEED_U128_MAXIMUM">ERR_TICK_EXCEED_U128_MAXIMUM</a>: u64 = 401;
</code></pre>



<a name="0xc8_tick_ERR_TICK_LIQUIDITY_INSUFFICIENT"></a>



<pre><code><b>const</b> <a href="tick.md#0xc8_tick_ERR_TICK_LIQUIDITY_INSUFFICIENT">ERR_TICK_LIQUIDITY_INSUFFICIENT</a>: u64 = 402;
</code></pre>



<a name="0xc8_tick_ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY"></a>



<pre><code><b>const</b> <a href="tick.md#0xc8_tick_ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY">ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY</a>: u64 = 403;
</code></pre>



<a name="0xc8_tick_create_tick_manager"></a>

## Function `create_tick_manager`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_create_tick_manager">create_tick_manager</a>(_tick_spacing: u32, _ts: u64, _ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_create_tick_manager">create_tick_manager</a>(
    _tick_spacing: u32,
    _ts: u64,
    _ctx: &<b>mut</b> TxContext,
): <a href="tick.md#0xc8_tick_TickManager">TickManager</a> {
    <a href="tick.md#0xc8_tick_TickManager">TickManager</a> {
        tick_spacing: _tick_spacing,
        ticks: <a href="skip_list.md#0xc8_skip_list_new">skip_list::new</a>(16, 2, _ts, _ctx),
    }
}
</code></pre>



</details>

<a name="0xc8_tick_sqrt_price"></a>

## Function `sqrt_price`

tick info


<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_sqrt_price">sqrt_price</a>(_tick: &<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_sqrt_price">sqrt_price</a>(_tick: &<a href="tick.md#0xc8_tick_Tick">Tick</a>): u128 {
    _tick.sqrt_price
}
</code></pre>



</details>

<a name="0xc8_tick_liquidity_net"></a>

## Function `liquidity_net`



<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_liquidity_net">liquidity_net</a>(_tick: &<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>): <a href="i128.md#0xc8_i128_I128">i128::I128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_liquidity_net">liquidity_net</a>(_tick: &<a href="tick.md#0xc8_tick_Tick">Tick</a>): I128 {
    _tick.liquidity_net
}
</code></pre>



</details>

<a name="0xc8_tick_tick_index"></a>

## Function `tick_index`



<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_tick_index">tick_index</a>(_tick: &<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>): <a href="i32.md#0xc8_i32_I32">i32::I32</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_tick_index">tick_index</a>(_tick: &<a href="tick.md#0xc8_tick_Tick">Tick</a>): I32 {
    _tick.index
}
</code></pre>



</details>

<a name="0xc8_tick_tick_spacing"></a>

## Function `tick_spacing`



<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_tick_spacing">tick_spacing</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_tick_spacing">tick_spacing</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">TickManager</a>): u32 {
    _tick_manager.tick_spacing
}
</code></pre>



</details>

<a name="0xc8_tick_fetch_ticks"></a>

## Function `fetch_ticks`



<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_fetch_ticks">fetch_ticks</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="tick.md#0xc8_tick_fetch_ticks">fetch_ticks</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">TickManager</a>) {
    <b>abort</b> 0
}
</code></pre>



</details>

<a name="0xc8_tick_default"></a>

## Function `default`

private fun


<pre><code><b>fun</b> <a href="tick.md#0xc8_tick_default">default</a>(_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>): <a href="tick.md#0xc8_tick_Tick">tick::Tick</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="tick.md#0xc8_tick_default">default</a>(_tick_index: I32): <a href="tick.md#0xc8_tick_Tick">Tick</a> {
    <b>let</b> sqrt_price = <a href="tick_math.md#0xc8_tick_math_get_sqrt_price_at_tick">tick_math::get_sqrt_price_at_tick</a>(_tick_index);
    <a href="tick.md#0xc8_tick_Tick">Tick</a> {
        sqrt_price,
        index: _tick_index,
        liquidity_net: <a href="i128.md#0xc8_i128_from">i128::from</a>(sqrt_price),
        liquidity_gross: 0
    }
}
</code></pre>



</details>

<a name="0xc8_tick_tick_score"></a>

## Function `tick_score`



<pre><code><b>fun</b> <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_index: I32): u64 {
    <b>let</b> score = <a href="i32.md#0xc8_i32_as_u32">i32::as_u32</a>(<a href="i32.md#0xc8_i32_add">i32::add</a>(_tick_index, <a href="tick_math.md#0xc8_tick_math_max_tick">tick_math::max_tick</a>()));
    <b>let</b> bound = <a href="i32.md#0xc8_i32_as_u32">i32::as_u32</a>(<a href="tick_math.md#0xc8_tick_math_max_tick">tick_math::max_tick</a>()) * 2;
    <b>assert</b>!(score &lt;= bound, <a href="tick.md#0xc8_tick_ERR_TICK_EXCEED_TWICE_MAXIMUM">ERR_TICK_EXCEED_TWICE_MAXIMUM</a>);
    (score <b>as</b> u64)
}
</code></pre>



</details>

<a name="0xc8_tick_update_by_liquidity"></a>

## Function `update_by_liquidity`



<pre><code><b>fun</b> <a href="tick.md#0xc8_tick_update_by_liquidity">update_by_liquidity</a>(_tick: &<b>mut</b> <a href="tick.md#0xc8_tick_Tick">tick::Tick</a>, _current_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _liquidity_delta: u128, _is_add_liquidity: bool, _is_cross_net: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="tick.md#0xc8_tick_update_by_liquidity">update_by_liquidity</a>(
    _tick: &<b>mut</b> <a href="tick.md#0xc8_tick_Tick">Tick</a>,
    _current_tick_index: I32,
    _liquidity_delta: u128,
    _is_add_liquidity: bool,
    _is_cross_net: bool
)
{
    <b>if</b> (_is_add_liquidity == <b>true</b>) {
        <b>assert</b>!(<a href="math_u128.md#0xc8_math_u128_add_check">math_u128::add_check</a>(_tick.liquidity_gross, _liquidity_delta), <a href="tick.md#0xc8_tick_ERR_TICK_EXCEED_U128_MAXIMUM">ERR_TICK_EXCEED_U128_MAXIMUM</a>);
        _tick.liquidity_gross = _tick.liquidity_gross + _liquidity_delta;
    } <b>else</b> {
        <b>assert</b>!(_tick.liquidity_gross &gt;= _liquidity_delta, <a href="tick.md#0xc8_tick_ERR_TICK_LIQUIDITY_INSUFFICIENT">ERR_TICK_LIQUIDITY_INSUFFICIENT</a>);
        _tick.liquidity_gross = _tick.liquidity_gross - _liquidity_delta;
    };
    <b>let</b> is_overflowing: bool;
    <b>let</b> liquidity_net: I128;
    <b>if</b> (_is_add_liquidity) {
        <b>if</b> (_is_cross_net) {
            (liquidity_net, is_overflowing) = <a href="i128.md#0xc8_i128_overflowing_sub">i128::overflowing_sub</a>(
                _tick.liquidity_net,
                <a href="i128.md#0xc8_i128_from">i128::from</a>(_liquidity_delta)
            );
        } <b>else</b> {
            (liquidity_net, is_overflowing) = <a href="i128.md#0xc8_i128_overflowing_add">i128::overflowing_add</a>(
                _tick.liquidity_net,
                <a href="i128.md#0xc8_i128_from">i128::from</a>(_liquidity_delta)
            );
        };
    } <b>else</b> {
        <b>if</b> (_is_cross_net) {
            (liquidity_net, is_overflowing) = <a href="i128.md#0xc8_i128_overflowing_add">i128::overflowing_add</a>(
                _tick.liquidity_net,
                <a href="i128.md#0xc8_i128_from">i128::from</a>(_liquidity_delta)
            );
        } <b>else</b> {
            (liquidity_net, is_overflowing) = <a href="i128.md#0xc8_i128_overflowing_sub">i128::overflowing_sub</a>(
                _tick.liquidity_net,
                <a href="i128.md#0xc8_i128_from">i128::from</a>(_liquidity_delta)
            );
        };
    };
    <b>assert</b>!(!is_overflowing, <a href="tick.md#0xc8_tick_ERR_TICK_LIQUIDITY_INSUFFICIENT">ERR_TICK_LIQUIDITY_INSUFFICIENT</a>);
    _tick.liquidity_net = liquidity_net;
}
</code></pre>



</details>

<a name="0xc8_tick_increase_liquidity"></a>

## Function `increase_liquidity`

add/remove liquidity


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_increase_liquidity">increase_liquidity</a>(_tick_manager: &<b>mut</b> <a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>, _current_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_lower_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _liquidity_delta: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_increase_liquidity">increase_liquidity</a>(
    _tick_manager: &<b>mut</b> <a href="tick.md#0xc8_tick_TickManager">TickManager</a>,
    _current_tick_index: I32,
    _tick_lower_index: I32,
    _tick_upper_index: I32,
    _liquidity_delta: u128
)
{
    <b>if</b> (_liquidity_delta == 0) {
        <b>return</b>
    };
    <b>let</b> tick_lower_score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_lower_index);
    <b>let</b> tick_upper_score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_upper_index);

    <b>if</b> (!<a href="skip_list.md#0xc8_skip_list_contains">skip_list::contains</a>(&_tick_manager.ticks, tick_lower_score)) {
        <a href="skip_list.md#0xc8_skip_list_insert">skip_list::insert</a>(&<b>mut</b> _tick_manager.ticks, tick_lower_score, <a href="tick.md#0xc8_tick_default">default</a>(_tick_lower_index));
    };
    <b>if</b> (!<a href="skip_list.md#0xc8_skip_list_contains">skip_list::contains</a>(&_tick_manager.ticks, tick_upper_score)) {
        <a href="skip_list.md#0xc8_skip_list_insert">skip_list::insert</a>(&<b>mut</b> _tick_manager.ticks, tick_upper_score, <a href="tick.md#0xc8_tick_default">default</a>(_tick_upper_index));
    };

    <b>let</b> lower_tick = <a href="skip_list.md#0xc8_skip_list_borrow_mut">skip_list::borrow_mut</a>(&<b>mut</b> _tick_manager.ticks, tick_lower_score);
    <a href="tick.md#0xc8_tick_update_by_liquidity">update_by_liquidity</a>(
        lower_tick,
        _current_tick_index,
        _liquidity_delta,
        <b>true</b>,
        <b>false</b>
    );
    <b>let</b> upper_tick = <a href="skip_list.md#0xc8_skip_list_borrow_mut">skip_list::borrow_mut</a>(&<b>mut</b> _tick_manager.ticks, tick_upper_score);
    <a href="tick.md#0xc8_tick_update_by_liquidity">update_by_liquidity</a>(
        upper_tick,
        _current_tick_index,
        _liquidity_delta,
        <b>true</b>,
        <b>true</b>
    );
}
</code></pre>



</details>

<a name="0xc8_tick_decrease_liquidity"></a>

## Function `decrease_liquidity`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_decrease_liquidity">decrease_liquidity</a>(_tick_manager: &<b>mut</b> <a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>, _current_tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_lower_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _tick_upper_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _liquidity_delta: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_decrease_liquidity">decrease_liquidity</a>(
    _tick_manager: &<b>mut</b> <a href="tick.md#0xc8_tick_TickManager">TickManager</a>,
    _current_tick_index: I32,
    _tick_lower_index: I32,
    _tick_upper_index: I32,
    _liquidity_delta: u128
)
{
    <b>if</b> (_liquidity_delta == 0) {
        <b>return</b>
    };
    <b>let</b> tick_lower_score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_lower_index);
    <b>let</b> tick_upper_score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_upper_index);
    <b>assert</b>!(
        !<a href="skip_list.md#0xc8_skip_list_contains">skip_list::contains</a>(&_tick_manager.ticks, tick_lower_score) &&
            !<a href="skip_list.md#0xc8_skip_list_contains">skip_list::contains</a>(&_tick_manager.ticks, tick_upper_score),
        <a href="tick.md#0xc8_tick_ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY">ERR_TICK_RANGE_NOT_HAVE_LIQUIDITY</a>
    );
    <b>let</b> lower_tick = <a href="skip_list.md#0xc8_skip_list_borrow_mut">skip_list::borrow_mut</a>(&<b>mut</b> _tick_manager.ticks, tick_lower_score);
    <a href="tick.md#0xc8_tick_update_by_liquidity">update_by_liquidity</a>(
        lower_tick,
        _current_tick_index,
        _liquidity_delta,
        <b>false</b>,
        <b>false</b>
    );
    <b>let</b> is_liquidity_changed = lower_tick.liquidity_gross != _liquidity_delta;
    <b>if</b> (is_liquidity_changed && lower_tick.liquidity_gross == 0 && _current_tick_index != _tick_upper_index) {
        <a href="skip_list.md#0xc8_skip_list_remove">skip_list::remove</a>(&<b>mut</b> _tick_manager.ticks, tick_lower_score);
    };
    <b>let</b> upper_tick = <a href="skip_list.md#0xc8_skip_list_borrow_mut">skip_list::borrow_mut</a>(&<b>mut</b> _tick_manager.ticks, tick_upper_score);
    <a href="tick.md#0xc8_tick_update_by_liquidity">update_by_liquidity</a>(
        upper_tick,
        _current_tick_index,
        _liquidity_delta,
        <b>false</b>,
        <b>true</b>
    );
    is_liquidity_changed = upper_tick.liquidity_gross != _liquidity_delta;
    <b>if</b> (is_liquidity_changed && upper_tick.liquidity_gross == 0 && _current_tick_index != _tick_lower_index) {
        <a href="skip_list.md#0xc8_skip_list_remove">skip_list::remove</a>(&<b>mut</b> _tick_manager.ticks, tick_upper_score);
    };
}
</code></pre>



</details>

<a name="0xc8_tick_try_borrow_tick"></a>

## Function `try_borrow_tick`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_try_borrow_tick">try_borrow_tick</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>, _tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>): <a href="_Option">option::Option</a>&lt;<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_try_borrow_tick">try_borrow_tick</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">TickManager</a>, _tick_index: I32): Option&lt;<a href="tick.md#0xc8_tick_Tick">Tick</a>&gt; {
    <b>let</b> tick_score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_index);
    <b>if</b> (!<a href="skip_list.md#0xc8_skip_list_contains">skip_list::contains</a>(&_tick_manager.ticks, tick_score)) {
        <b>return</b> <a href="_none">option::none</a>&lt;<a href="tick.md#0xc8_tick_Tick">Tick</a>&gt;()
    };
    <b>let</b> tick_borrow = <a href="skip_list.md#0xc8_skip_list_borrow">skip_list::borrow</a>(&_tick_manager.ticks, tick_score);
    <b>return</b> <a href="_some">option::some</a>(*tick_borrow)
}
</code></pre>



</details>

<a name="0xc8_tick_borrow_tick_for_swap"></a>

## Function `borrow_tick_for_swap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_borrow_tick_for_swap">borrow_tick_for_swap</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>, _score: u64, _is_x2y: bool): (&<a href="tick.md#0xc8_tick_Tick">tick::Tick</a>, <a href="option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_borrow_tick_for_swap">borrow_tick_for_swap</a>(
    _tick_manager: &<a href="tick.md#0xc8_tick_TickManager">TickManager</a>,
    _score: u64,
    _is_x2y: bool
): (&<a href="tick.md#0xc8_tick_Tick">Tick</a>, OptionU64) {
    <b>let</b> node = <a href="skip_list.md#0xc8_skip_list_borrow_node">skip_list::borrow_node</a>(&_tick_manager.ticks, _score);
    <b>let</b> score = <b>if</b> (_is_x2y) {
        <a href="skip_list.md#0xc8_skip_list_prev_score">skip_list::prev_score</a>(node)
    } <b>else</b> {
        <a href="skip_list.md#0xc8_skip_list_next_score">skip_list::next_score</a>(node)
    };
    (<a href="skip_list.md#0xc8_skip_list_borrow_value">skip_list::borrow_value</a>(node), score)
}
</code></pre>



</details>

<a name="0xc8_tick_cross_by_swap"></a>

## Function `cross_by_swap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_cross_by_swap">cross_by_swap</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>, _tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _is_x2y: bool, _liquidity: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_cross_by_swap">cross_by_swap</a>(
    _tick_manager: &<a href="tick.md#0xc8_tick_TickManager">TickManager</a>,
    _tick_index: I32,
    _is_x2y: bool,
    _liquidity: u128
): u128
{
    <b>let</b> <a href="tick.md#0xc8_tick">tick</a> = <a href="skip_list.md#0xc8_skip_list_borrow">skip_list::borrow</a>(&_tick_manager.ticks, <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_index));
    <b>let</b> liquidity_net = <b>if</b> (_is_x2y) {
        <a href="i128.md#0xc8_i128_neg">i128::neg</a>(<a href="tick.md#0xc8_tick">tick</a>.liquidity_net)
    } <b>else</b> {
        <a href="tick.md#0xc8_tick">tick</a>.liquidity_net
    };
    <b>let</b> liquidity_ret: u128;
    <b>if</b> (<a href="i128.md#0xc8_i128_is_neg">i128::is_neg</a>(liquidity_net)) {
        <b>assert</b>!(<a href="i128.md#0xc8_i128_abs_u128">i128::abs_u128</a>(liquidity_net) &lt;= _liquidity, <a href="tick.md#0xc8_tick_ERR_TICK_LIQUIDITY_INSUFFICIENT">ERR_TICK_LIQUIDITY_INSUFFICIENT</a>);
        liquidity_ret = _liquidity - <a href="i128.md#0xc8_i128_abs_u128">i128::abs_u128</a>(liquidity_net);
    } <b>else</b> {
        <b>assert</b>!(<a href="math_u128.md#0xc8_math_u128_add_check">math_u128::add_check</a>(<a href="i128.md#0xc8_i128_abs_u128">i128::abs_u128</a>(liquidity_net), _liquidity), <a href="tick.md#0xc8_tick_ERR_TICK_EXCEED_U128_MAXIMUM">ERR_TICK_EXCEED_U128_MAXIMUM</a>);
        liquidity_ret = <a href="i128.md#0xc8_i128_abs_u128">i128::abs_u128</a>(liquidity_net) + _liquidity;
    };
    liquidity_ret
}
</code></pre>



</details>

<a name="0xc8_tick_first_score_for_swap"></a>

## Function `first_score_for_swap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_first_score_for_swap">first_score_for_swap</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>, _tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _is_x2y: bool): <a href="option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_first_score_for_swap">first_score_for_swap</a>(
    _tick_manager: &<a href="tick.md#0xc8_tick_TickManager">TickManager</a>,
    _tick_index: I32,
    _is_x2y: bool,
): OptionU64 {
    <b>let</b> score;
    <b>if</b> (_is_x2y) {
        score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_index);
        <a href="skip_list.md#0xc8_skip_list_find_prev">skip_list::find_prev</a>(&_tick_manager.ticks, score, <b>true</b>)
    } <b>else</b> {
        <b>if</b> (<a href="i32.md#0xc8_i32_eq">i32::eq</a>(
            _tick_index,
            <a href="i32.md#0xc8_i32_neg_from">i32::neg_from</a>(<a href="tick_math.md#0xc8_tick_math_tick_bound">tick_math::tick_bound</a>() + 1),
        )) {
            score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(<a href="tick_math.md#0xc8_tick_math_min_tick">tick_math::min_tick</a>());
            <a href="skip_list.md#0xc8_skip_list_find_next">skip_list::find_next</a>(&_tick_manager.ticks, score, <b>true</b>)
        } <b>else</b> {
            score = <a href="tick.md#0xc8_tick_tick_score">tick_score</a>(_tick_index);
            <a href="skip_list.md#0xc8_skip_list_find_next">skip_list::find_next</a>(&_tick_manager.ticks, score, <b>false</b>)
        }
    }
}
</code></pre>



</details>

<a name="0xc8_tick_get_ticks"></a>

## Function `get_ticks`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_get_ticks">get_ticks</a>(_tick_manager: &<a href="tick.md#0xc8_tick_TickManager">tick::TickManager</a>, _tick_index: <a href="i32.md#0xc8_i32_I32">i32::I32</a>, _spacing_times: u32, _total_count: u32): <a href="">vector</a>&lt;<a href="">vector</a>&lt;<a href="i32.md#0xc8_i32_I32">i32::I32</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="tick.md#0xc8_tick_get_ticks">get_ticks</a>(
    _tick_manager: &<a href="tick.md#0xc8_tick_TickManager">TickManager</a>,
    _tick_index: I32,
    _spacing_times: u32,
    _total_count: u32,
): <a href="">vector</a>&lt;<a href="">vector</a>&lt;I32&gt;&gt; {
    <b>let</b> half_ticks = _total_count / 2 * (_spacing_times * _tick_manager.tick_spacing);
    <b>let</b> start_gap = <a href="i32.md#0xc8_i32_from_u32">i32::from_u32</a>(
        half_ticks + ((_spacing_times / 2) * _tick_manager.tick_spacing)
    );
    <b>let</b> tick_gap = <a href="i32.md#0xc8_i32_from_u32">i32::from_u32</a>(_spacing_times * _tick_manager.tick_spacing);
    <b>let</b> count = _total_count;
    <b>let</b> start = <a href="i32.md#0xc8_i32_sub">i32::sub</a>(_tick_index, start_gap);
    <b>let</b> ticks = <a href="_empty">vector::empty</a>&lt;<a href="">vector</a>&lt;I32&gt;&gt;();
    <b>while</b> (count &gt; 0) {
        <b>let</b> new_start = <a href="i32.md#0xc8_i32_add">i32::add</a>(start, tick_gap);
        <a href="_push_back">vector::push_back</a>(&<b>mut</b> ticks, <a href="">vector</a>&lt;I32&gt;[start, new_start]);
        start = new_start;
        count = count - 1
    };
    ticks
}
</code></pre>



</details>

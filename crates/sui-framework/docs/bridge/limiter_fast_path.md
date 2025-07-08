---
title: Module `bridge::limiter_fast_path`
---



-  [Struct `UserLimitRecord`](#bridge_limiter_fast_path_UserLimitRecord)
-  [Struct `UserLimiter`](#bridge_limiter_fast_path_UserLimiter)
-  [Struct `LimitConfigKey`](#bridge_limiter_fast_path_LimitConfigKey)
-  [Struct `UserLimiterKey`](#bridge_limiter_fast_path_UserLimiterKey)
-  [Struct `UserLimitUsedEvent`](#bridge_limiter_fast_path_UserLimitUsedEvent)
-  [Struct `UserLimitInfo`](#bridge_limiter_fast_path_UserLimitInfo)
-  [Constants](#@Constants_0)
-  [Function `borrow`](#bridge_limiter_fast_path_borrow)
-  [Function `borrow_mut`](#bridge_limiter_fast_path_borrow_mut)
-  [Function `registry`](#bridge_limiter_fast_path_registry)
-  [Function `initial_limiter_fast_path`](#bridge_limiter_fast_path_initial_limiter_fast_path)
-  [Function `add_limiter`](#bridge_limiter_fast_path_add_limiter)
-  [Function `new`](#bridge_limiter_fast_path_new)
-  [Function `check_and_record_user_limit`](#bridge_limiter_fast_path_check_and_record_user_limit)
-  [Function `get_user_limit_info`](#bridge_limiter_fast_path_get_user_limit_info)
-  [Function `get_user_remaining_limit`](#bridge_limiter_fast_path_get_user_remaining_limit)
-  [Function `set_default_limit`](#bridge_limiter_fast_path_set_default_limit)
-  [Function `get_default_limit`](#bridge_limiter_fast_path_get_default_limit)
-  [Function `set_default_time_window`](#bridge_limiter_fast_path_set_default_time_window)
-  [Function `get_default_time_window`](#bridge_limiter_fast_path_get_default_time_window)
-  [Function `set_enabled`](#bridge_limiter_fast_path_set_enabled)
-  [Function `get_enabled`](#bridge_limiter_fast_path_get_enabled)
-  [Function `get_user_count`](#bridge_limiter_fast_path_get_user_count)
-  [Function `current_hour_since_epoch`](#bridge_limiter_fast_path_current_hour_since_epoch)
-  [Function `adjust_user_limit_records`](#bridge_limiter_fast_path_adjust_user_limit_records)


<pre><code><b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/clock.md#sui_clock">sui::clock</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
</code></pre>



<a name="bridge_limiter_fast_path_UserLimitRecord"></a>

## Struct `UserLimitRecord`

用户限额记录，存储每个用户的限额信息


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitRecord">UserLimitRecord</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
 用户地址
</dd>
<dt>
<code>hour_head: u64</code>
</dt>
<dd>
 当前滑动窗口的头（最新小时）
</dd>
<dt>
<code>hour_tail: u64</code>
</dt>
<dd>
 当前滑动窗口的尾（最早小时）
</dd>
<dt>
<code>per_hour_amounts: vector&lt;u64&gt;</code>
</dt>
<dd>
 每小时的使用量（长度最多为 USER_TIME_WINDOW_HOURS）
</dd>
<dt>
<code>total_amount: u64</code>
</dt>
<dd>
 当前窗口内总使用量
</dd>
</dl>


</details>

<a name="bridge_limiter_fast_path_UserLimiter"></a>

## Struct `UserLimiter`

用户限额管理器


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">UserLimiter</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>user_records: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiterKey">bridge::limiter_fast_path::UserLimiterKey</a>, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitRecord">bridge::limiter_fast_path::UserLimitRecord</a>&gt;</code>
</dt>
<dd>
 用户限额记录表
</dd>
<dt>
<code>limit_configs: <a href="../sui/table.md#sui_table_Table">sui::table::Table</a>&lt;<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_LimitConfigKey">bridge::limiter_fast_path::LimitConfigKey</a>, u64&gt;</code>
</dt>
<dd>
 限额配置
</dd>
<dt>
<code>default_limit: u64</code>
</dt>
<dd>
 全局默认限额
</dd>
<dt>
<code>default_time_window: u64</code>
</dt>
<dd>
 全局默认时间窗口
</dd>
<dt>
<code>enabled: bool</code>
</dt>
<dd>
 是否启用用户限额
</dd>
</dl>


</details>

<a name="bridge_limiter_fast_path_LimitConfigKey"></a>

## Struct `LimitConfigKey`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_LimitConfigKey">LimitConfigKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>chain_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>token_id: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_limiter_fast_path_UserLimiterKey"></a>

## Struct `UserLimiterKey`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiterKey">UserLimiterKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>chain_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>token_id: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_limiter_fast_path_UserLimitUsedEvent"></a>

## Struct `UserLimitUsedEvent`

用户限额使用事件


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitUsedEvent">UserLimitUsedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>used_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>remaining_limit: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>window_start_hour: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_limiter_fast_path_UserLimitInfo"></a>

## Struct `UserLimitInfo`

用户限额信息


<pre><code><b>public</b> <b>struct</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitInfo">UserLimitInfo</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender_address: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>limit_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>used_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>remaining_limit: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>time_window_hours: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>window_start_hour: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>window_end_hour: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_limiter_fast_path_DEFAULT_TIME_WINDOW_HOURS"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_DEFAULT_TIME_WINDOW_HOURS">DEFAULT_TIME_WINDOW_HOURS</a>: u64 = 24;
</code></pre>



<a name="bridge_limiter_fast_path_EInvalidLimitAmount"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_EInvalidLimitAmount">EInvalidLimitAmount</a>: u64 = 3;
</code></pre>



<a name="bridge_limiter_fast_path_EInvalidTimeWindow"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_EInvalidTimeWindow">EInvalidTimeWindow</a>: u64 = 4;
</code></pre>



<a name="bridge_limiter_fast_path_ELimiterFastPathRegistryAlreadyExists"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_ELimiterFastPathRegistryAlreadyExists">ELimiterFastPathRegistryAlreadyExists</a>: u64 = 0;
</code></pre>



<a name="bridge_limiter_fast_path_KEY"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_KEY">KEY</a>: vector&lt;u8&gt; = vector[108, 105, 109, 105, 116, 101, 114, 95, 102, 97, 115, 116, 95, 112, 97, 116, 104];
</code></pre>



<a name="bridge_limiter_fast_path_TOKEN_ID_BUSD"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a>: u64 = 5;
</code></pre>



<a name="bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>: u64 = 100000000000;
</code></pre>



<a name="bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>: u64 = 1000000000000;
</code></pre>



<a name="bridge_limiter_fast_path_USER_LIMIT_5K_IN_BUSD"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_5K_IN_BUSD">USER_LIMIT_5K_IN_BUSD</a>: u64 = 5000000000000;
</code></pre>



<a name="bridge_limiter_fast_path_USER_TIME_WINDOW_HOURS"></a>



<pre><code><b>const</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_TIME_WINDOW_HOURS">USER_TIME_WINDOW_HOURS</a>: u64 = 24;
</code></pre>



<a name="bridge_limiter_fast_path_borrow"></a>

## Function `borrow`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow">borrow</a>(parent_id: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">bridge::limiter_fast_path::UserLimiter</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow">borrow</a>(parent_id: &UID): &<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">UserLimiter</a>{
    dynamic_field::borrow&lt;vector&lt;u8&gt;,<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">UserLimiter</a>&gt;(parent_id, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): &<b>mut</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">bridge::limiter_fast_path::UserLimiter</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(parent_id: &<b>mut</b> UID): &<b>mut</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">UserLimiter</a>{
    dynamic_field::borrow_mut&lt;vector&lt;u8&gt;,<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">UserLimiter</a>&gt;(parent_id, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_KEY">KEY</a>)
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_registry"></a>

## Function `registry`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_registry">registry</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_registry">registry</a>(parent_id: &<b>mut</b> UID,ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(
        !dynamic_field::exists_(parent_id, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_KEY">KEY</a>),
        <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_ELimiterFastPathRegistryAlreadyExists">ELimiterFastPathRegistryAlreadyExists</a>
    );
    dynamic_field::add(
        parent_id,
        <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_KEY">KEY</a>,
        <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_new">new</a>(ctx),
    );
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_initial_limiter_fast_path">initial_limiter_fast_path</a>(parent_id);
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_initial_limiter_fast_path"></a>

## Function `initial_limiter_fast_path`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_initial_limiter_fast_path">initial_limiter_fast_path</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_initial_limiter_fast_path">initial_limiter_fast_path</a>(parent_id: &<b>mut</b> UID) {
    //eth
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">chain_ids::eth_mainnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_5K_IN_BUSD">USER_LIMIT_5K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">chain_ids::eth_sepolia</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_5K_IN_BUSD">USER_LIMIT_5K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">chain_ids::eth_custom</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>);
    //base
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">chain_ids::base_mainnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">chain_ids::base_testnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">chain_ids::base_custom</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>);
    //arb
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">chain_ids::arb_mainnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">chain_ids::arb_testnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">chain_ids::arb_custom</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>);
    //op
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">chain_ids::op_mainnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">chain_ids::op_testnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">chain_ids::op_custom</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>);
    //pol
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">chain_ids::pol_mainnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">chain_ids::pol_testnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">chain_ids::pol_custom</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>);
    //avax
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">chain_ids::avax_mainnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">chain_ids::avax_testnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">chain_ids::avax_custom</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>);
    //bsc
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">chain_ids::bsc_mainnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">chain_ids::bsc_testnet</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>);
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id, <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">chain_ids::bsc_custom</a>(), <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_TOKEN_ID_BUSD">TOKEN_ID_BUSD</a> <b>as</b> u64, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_100_IN_BUSD">USER_LIMIT_100_IN_BUSD</a>);
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_add_limiter"></a>

## Function `add_limiter`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, chain_id: u8, token_id: u64, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_add_limiter">add_limiter</a>(
    parent_id: &<b>mut</b> UID,
    chain_id: u8,
    token_id: u64,
    amount:u64,
) {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(parent_id);
    <b>let</b> limit_config_key = <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_LimitConfigKey">LimitConfigKey</a> { chain_id, token_id };
    <b>if</b> (!self.limit_configs.contains(limit_config_key)) {
        self.limit_configs.add(limit_config_key, amount);
    };
    *self.limit_configs.<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(limit_config_key) = amount;
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_new"></a>

## Function `new`

创建新的用户限额管理器


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_new">new</a>(ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">bridge::limiter_fast_path::UserLimiter</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_new">new</a>(ctx: &<b>mut</b> TxContext): <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">UserLimiter</a> {
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiter">UserLimiter</a> {
        user_records: table::new(ctx),
        limit_configs: table::new(ctx),
        default_limit: <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_LIMIT_1K_IN_BUSD">USER_LIMIT_1K_IN_BUSD</a>,
        default_time_window: <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_DEFAULT_TIME_WINDOW_HOURS">DEFAULT_TIME_WINDOW_HOURS</a>,
        enabled: <b>true</b>,
    }
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_check_and_record_user_limit"></a>

## Function `check_and_record_user_limit`

检查并记录用户限额使用


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_check_and_record_user_limit">check_and_record_user_limit</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, sender_address: vector&lt;u8&gt;, chain_id: u8, token_id: u64, amount: u64, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_check_and_record_user_limit">check_and_record_user_limit</a>(
    parent_id: &<b>mut</b> UID,
    sender_address: vector&lt;u8&gt;,
    chain_id: u8,
    token_id: u64,
    amount: u64,
    clock: &Clock,
): bool {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!self.enabled) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> current_hour = <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_current_hour_since_epoch">current_hour_since_epoch</a>(clock);
    // 如果用户没有限额记录，初始化
    <b>if</b> (!table::contains(&self.user_records, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiterKey">UserLimiterKey</a> { sender_address, chain_id, token_id })) {
        <b>let</b> record = <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitRecord">UserLimitRecord</a> {
            sender_address,
            hour_head: current_hour,
            hour_tail: current_hour,
            per_hour_amounts: vector[0],
            total_amount: 0,
        };
        table::add(&<b>mut</b> self.user_records, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiterKey">UserLimiterKey</a> { sender_address, chain_id, token_id }, record);
    };
    <b>let</b> record = table::borrow_mut(&<b>mut</b> self.user_records, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiterKey">UserLimiterKey</a> { sender_address, chain_id, token_id });
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_adjust_user_limit_records">adjust_user_limit_records</a>(record, current_hour);
    <b>let</b> limit_config_key = <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_LimitConfigKey">LimitConfigKey</a> { chain_id, token_id };
    <b>let</b> limit_amount = <b>if</b> (!table::contains(&self.limit_configs, limit_config_key)) {
        self.default_limit
    }<b>else</b>{
        <b>let</b> limit_config = table::borrow(&self.limit_configs, limit_config_key);
        *limit_config
    };
    // 检查限额是否足够
    <b>if</b> (record.total_amount + amount &gt; limit_amount) {
        <b>return</b> <b>false</b>
    };
    // record the amount of this hour
    <b>let</b> new_amount = record.per_hour_amounts.pop_back() + amount;
    record.per_hour_amounts.push_back(new_amount);
    record.total_amount = record.total_amount + amount;
    emit(<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitUsedEvent">UserLimitUsedEvent</a> {
        sender_address,
        used_amount: amount,
        remaining_limit: limit_amount - record.total_amount,
        window_start_hour: record.hour_tail,
    });
    <b>true</b>
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_get_user_limit_info"></a>

## Function `get_user_limit_info`

获取用户当前限额信息


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_user_limit_info">get_user_limit_info</a>(parent_id: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, sender_address: vector&lt;u8&gt;, chain_id: u8, token_id: u64, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitInfo">bridge::limiter_fast_path::UserLimitInfo</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_user_limit_info">get_user_limit_info</a>(
    parent_id: &<b>mut</b> UID,
    sender_address: vector&lt;u8&gt;,
    chain_id: u8,
    token_id: u64,
    clock: &Clock,
): Option&lt;<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitInfo">UserLimitInfo</a>&gt; {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(parent_id);
    <b>if</b> (!table::contains(&self.user_records, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiterKey">UserLimiterKey</a> { sender_address, chain_id, token_id })) {
        <b>return</b> option::none()
    };
    <b>let</b> record = table::borrow_mut(&<b>mut</b> self.user_records, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimiterKey">UserLimiterKey</a> { sender_address, chain_id, token_id });
    <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_adjust_user_limit_records">adjust_user_limit_records</a>(record, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_current_hour_since_epoch">current_hour_since_epoch</a>(clock));
    <b>let</b> limit_config_key = <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_LimitConfigKey">LimitConfigKey</a> { chain_id, token_id };
    <b>let</b> limit = <b>if</b> (!table::contains(&self.limit_configs, limit_config_key)) {
        self.default_limit
    }<b>else</b>{
        <b>let</b> limit_config = table::borrow(&self.limit_configs, limit_config_key);
        *limit_config
    };
    option::some(<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitInfo">UserLimitInfo</a> {
        sender_address,
        limit_amount: limit,
        used_amount: record.total_amount,
        remaining_limit: limit - record.total_amount,
        time_window_hours: <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_USER_TIME_WINDOW_HOURS">USER_TIME_WINDOW_HOURS</a>,
        window_start_hour: record.hour_tail,
        window_end_hour: record.hour_head,
    })
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_get_user_remaining_limit"></a>

## Function `get_user_remaining_limit`

获取用户剩余限额


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_user_remaining_limit">get_user_remaining_limit</a>(self: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, sender_address: vector&lt;u8&gt;, chain_id: u8, token_id: u64, clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_user_remaining_limit">get_user_remaining_limit</a>(
    self: &<b>mut</b> UID,
    sender_address: vector&lt;u8&gt;,
    chain_id: u8,
    token_id: u64,
    clock: &Clock,
): u64 {
    <b>let</b> limit_info_opt = <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_user_limit_info">get_user_limit_info</a>(self, sender_address, chain_id, token_id, clock);
    <b>if</b> (option::is_none(&limit_info_opt)) {
        <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(self);
        <b>if</b>(!table::contains(&self.limit_configs, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_LimitConfigKey">LimitConfigKey</a> { chain_id, token_id })){
            <b>return</b> self.default_limit
        };
        <b>let</b> limit_config = table::borrow(&self.limit_configs, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_LimitConfigKey">LimitConfigKey</a> { chain_id, token_id });
        *limit_config
    } <b>else</b> {
        <b>let</b> limit_info = option::destroy_some(limit_info_opt);
        limit_info.remaining_limit
    }
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_set_default_limit"></a>

## Function `set_default_limit`

设置全局默认限额


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_set_default_limit">set_default_limit</a>(self: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, new_limit: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_set_default_limit">set_default_limit</a>(self: &<b>mut</b> UID, new_limit: u64) {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(self);
    <b>assert</b>!(new_limit &gt; 0, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_EInvalidLimitAmount">EInvalidLimitAmount</a>);
    self.default_limit = new_limit;
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_get_default_limit"></a>

## Function `get_default_limit`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_default_limit">get_default_limit</a>(self: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_default_limit">get_default_limit</a>(self: &UID): u64 {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow">borrow</a>(self);
    self.default_limit
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_set_default_time_window"></a>

## Function `set_default_time_window`

设置全局默认时间窗口


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_set_default_time_window">set_default_time_window</a>(self: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, new_time_window: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_set_default_time_window">set_default_time_window</a>(self: &<b>mut</b> UID, new_time_window: u64) {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(self);
    <b>assert</b>!(new_time_window &gt; 0, <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_EInvalidTimeWindow">EInvalidTimeWindow</a>);
    self.default_time_window = new_time_window;
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_get_default_time_window"></a>

## Function `get_default_time_window`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_default_time_window">get_default_time_window</a>(self: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_default_time_window">get_default_time_window</a>(self: &UID): u64 {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow">borrow</a>(self);
    self.default_time_window
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_set_enabled"></a>

## Function `set_enabled`

启用或禁用用户限额


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_set_enabled">set_enabled</a>(self: &<b>mut</b> <a href="../sui/object.md#sui_object_UID">sui::object::UID</a>, enabled: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_set_enabled">set_enabled</a>(self: &<b>mut</b> UID, enabled: bool) {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow_mut">borrow_mut</a>(self);
    self.enabled = enabled;
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_get_enabled"></a>

## Function `get_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_enabled">get_enabled</a>(self: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_enabled">get_enabled</a>(self: &UID): bool {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow">borrow</a>(self);
    self.enabled
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_get_user_count"></a>

## Function `get_user_count`

获取用户限额记录数量


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_user_count">get_user_count</a>(self: &<a href="../sui/object.md#sui_object_UID">sui::object::UID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_get_user_count">get_user_count</a>(self: &UID): u64 {
    <b>let</b> self=<a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_borrow">borrow</a>(self);
    table::length(&self.user_records)
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_current_hour_since_epoch"></a>

## Function `current_hour_since_epoch`

获取当前小时（从 Unix epoch 开始）


<pre><code><b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_current_hour_since_epoch">current_hour_since_epoch</a>(clock: &<a href="../sui/clock.md#sui_clock_Clock">sui::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_current_hour_since_epoch">current_hour_since_epoch</a>(clock: &Clock): u64 {
    clock::timestamp_ms(clock) / 3600000
}
</code></pre>



</details>

<a name="bridge_limiter_fast_path_adjust_user_limit_records"></a>

## Function `adjust_user_limit_records`

滑动窗口，移除过期小时


<pre><code><b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_adjust_user_limit_records">adjust_user_limit_records</a>(record: &<b>mut</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitRecord">bridge::limiter_fast_path::UserLimitRecord</a>, current_hour: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_adjust_user_limit_records">adjust_user_limit_records</a>(record: &<b>mut</b> <a href="../bridge/limiter_fast_path.md#bridge_limiter_fast_path_UserLimitRecord">UserLimitRecord</a>, current_hour: u64) {
    <b>if</b>(record.hour_head==current_hour) {
        <b>return</b> // nothing to backfill
    };
    <b>let</b> target_tail = current_hour - 23;
    // <b>if</b> `hour_head` is even older than 24 hours ago, it means all items in
    // `per_hour_amounts` are to be evicted.
    <b>if</b> (record.hour_head &lt; target_tail) {
        record.per_hour_amounts = vector[];
        record.total_amount = 0;
        record.hour_tail = target_tail;
        record.hour_head = target_tail;
        record.per_hour_amounts.push_back(0);
    }<b>else</b>{
        // `hour_head` is within 24 hour range.
        // some items in `per_hour_amounts` are still valid, we remove stale hours.
        <b>while</b>(record.hour_tail &lt; target_tail) {
            record.total_amount = record.total_amount - record.per_hour_amounts.remove(0);
            record.hour_tail = record.hour_tail + 1;
        }
    };
    // Backfill from hour_head to current hour
    <b>while</b>(record.hour_head &lt; current_hour) {
        record.per_hour_amounts.push_back(0);
        record.hour_head = record.hour_head + 1;
    }
}
</code></pre>



</details>

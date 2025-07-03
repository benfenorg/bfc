---
title: Module `bridge::message_types`
---



-  [Constants](#@Constants_0)
-  [Function `token`](#bridge_message_types_token)
-  [Function `committee_blocklist`](#bridge_message_types_committee_blocklist)
-  [Function `emergency_op`](#bridge_message_types_emergency_op)
-  [Function `update_bridge_limit`](#bridge_message_types_update_bridge_limit)
-  [Function `update_asset_price`](#bridge_message_types_update_asset_price)
-  [Function `add_tokens_on_sui`](#bridge_message_types_add_tokens_on_sui)
-  [Function `update_bridge_limit_fast_path`](#bridge_message_types_update_bridge_limit_fast_path)
-  [Function `add_external_coin_admin`](#bridge_message_types_add_external_coin_admin)
-  [Function `remove_external_coin_admin`](#bridge_message_types_remove_external_coin_admin)
-  [Function `refund_admin_operate`](#bridge_message_types_refund_admin_operate)
-  [Function `add_external_coin_witness`](#bridge_message_types_add_external_coin_witness)
-  [Function `remove_external_coin_witness`](#bridge_message_types_remove_external_coin_witness)
-  [Function `add_external_coin_target`](#bridge_message_types_add_external_coin_target)
-  [Function `remove_external_coin_target`](#bridge_message_types_remove_external_coin_target)


<pre><code></code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="bridge_message_types_ADD_EXTERNAL_COIN_ADMIN"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_ADD_EXTERNAL_COIN_ADMIN">ADD_EXTERNAL_COIN_ADMIN</a>: u8 = 11;
</code></pre>



<a name="bridge_message_types_ADD_EXTERNAL_COIN_TARGET"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_ADD_EXTERNAL_COIN_TARGET">ADD_EXTERNAL_COIN_TARGET</a>: u8 = 15;
</code></pre>



<a name="bridge_message_types_ADD_EXTERNAL_COIN_WITNESS"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_ADD_EXTERNAL_COIN_WITNESS">ADD_EXTERNAL_COIN_WITNESS</a>: u8 = 13;
</code></pre>



<a name="bridge_message_types_ADD_TOKENS_ON_SUI"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_ADD_TOKENS_ON_SUI">ADD_TOKENS_ON_SUI</a>: u8 = 6;
</code></pre>



<a name="bridge_message_types_COMMITTEE_BLOCKLIST"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_COMMITTEE_BLOCKLIST">COMMITTEE_BLOCKLIST</a>: u8 = 1;
</code></pre>



<a name="bridge_message_types_EMERGENCY_OP"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_EMERGENCY_OP">EMERGENCY_OP</a>: u8 = 2;
</code></pre>



<a name="bridge_message_types_REFUND_ADMIN_OPERATE"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_REFUND_ADMIN_OPERATE">REFUND_ADMIN_OPERATE</a>: u8 = 8;
</code></pre>



<a name="bridge_message_types_REMOVE_EXTERNAL_COIN_ADMIN"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_REMOVE_EXTERNAL_COIN_ADMIN">REMOVE_EXTERNAL_COIN_ADMIN</a>: u8 = 12;
</code></pre>



<a name="bridge_message_types_REMOVE_EXTERNAL_COIN_TARGET"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_REMOVE_EXTERNAL_COIN_TARGET">REMOVE_EXTERNAL_COIN_TARGET</a>: u8 = 16;
</code></pre>



<a name="bridge_message_types_REMOVE_EXTERNAL_COIN_WITNESS"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_REMOVE_EXTERNAL_COIN_WITNESS">REMOVE_EXTERNAL_COIN_WITNESS</a>: u8 = 14;
</code></pre>



<a name="bridge_message_types_TOKEN"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_TOKEN">TOKEN</a>: u8 = 0;
</code></pre>



<a name="bridge_message_types_UPDATE_ASSET_PRICE"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_UPDATE_ASSET_PRICE">UPDATE_ASSET_PRICE</a>: u8 = 4;
</code></pre>



<a name="bridge_message_types_UPDATE_BRIDGE_LIMIT"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_UPDATE_BRIDGE_LIMIT">UPDATE_BRIDGE_LIMIT</a>: u8 = 3;
</code></pre>



<a name="bridge_message_types_UPDATE_BRIDGE_LIMIT_FAST_PATH"></a>



<pre><code><b>const</b> <a href="../bridge/message_types.md#bridge_message_types_UPDATE_BRIDGE_LIMIT_FAST_PATH">UPDATE_BRIDGE_LIMIT_FAST_PATH</a>: u8 = 9;
</code></pre>



<a name="bridge_message_types_token"></a>

## Function `token`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_token">token</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_token">token</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_TOKEN">TOKEN</a> }
</code></pre>



</details>

<a name="bridge_message_types_committee_blocklist"></a>

## Function `committee_blocklist`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_committee_blocklist">committee_blocklist</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_committee_blocklist">committee_blocklist</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_COMMITTEE_BLOCKLIST">COMMITTEE_BLOCKLIST</a> }
</code></pre>



</details>

<a name="bridge_message_types_emergency_op"></a>

## Function `emergency_op`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_emergency_op">emergency_op</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_emergency_op">emergency_op</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_EMERGENCY_OP">EMERGENCY_OP</a> }
</code></pre>



</details>

<a name="bridge_message_types_update_bridge_limit"></a>

## Function `update_bridge_limit`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_update_bridge_limit">update_bridge_limit</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_update_bridge_limit">update_bridge_limit</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_UPDATE_BRIDGE_LIMIT">UPDATE_BRIDGE_LIMIT</a> }
</code></pre>



</details>

<a name="bridge_message_types_update_asset_price"></a>

## Function `update_asset_price`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_update_asset_price">update_asset_price</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_update_asset_price">update_asset_price</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_UPDATE_ASSET_PRICE">UPDATE_ASSET_PRICE</a> }
</code></pre>



</details>

<a name="bridge_message_types_add_tokens_on_sui"></a>

## Function `add_tokens_on_sui`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_tokens_on_sui">add_tokens_on_sui</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_tokens_on_sui">add_tokens_on_sui</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_ADD_TOKENS_ON_SUI">ADD_TOKENS_ON_SUI</a> }
</code></pre>



</details>

<a name="bridge_message_types_update_bridge_limit_fast_path"></a>

## Function `update_bridge_limit_fast_path`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_update_bridge_limit_fast_path">update_bridge_limit_fast_path</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_update_bridge_limit_fast_path">update_bridge_limit_fast_path</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_UPDATE_BRIDGE_LIMIT_FAST_PATH">UPDATE_BRIDGE_LIMIT_FAST_PATH</a> }
</code></pre>



</details>

<a name="bridge_message_types_add_external_coin_admin"></a>

## Function `add_external_coin_admin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_admin">add_external_coin_admin</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_admin">add_external_coin_admin</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_ADD_EXTERNAL_COIN_ADMIN">ADD_EXTERNAL_COIN_ADMIN</a> }
</code></pre>



</details>

<a name="bridge_message_types_remove_external_coin_admin"></a>

## Function `remove_external_coin_admin`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_admin">remove_external_coin_admin</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_admin">remove_external_coin_admin</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_REMOVE_EXTERNAL_COIN_ADMIN">REMOVE_EXTERNAL_COIN_ADMIN</a> }
</code></pre>



</details>

<a name="bridge_message_types_refund_admin_operate"></a>

## Function `refund_admin_operate`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_refund_admin_operate">refund_admin_operate</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_refund_admin_operate">refund_admin_operate</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_REFUND_ADMIN_OPERATE">REFUND_ADMIN_OPERATE</a> }
</code></pre>



</details>

<a name="bridge_message_types_add_external_coin_witness"></a>

## Function `add_external_coin_witness`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_witness">add_external_coin_witness</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_witness">add_external_coin_witness</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_ADD_EXTERNAL_COIN_WITNESS">ADD_EXTERNAL_COIN_WITNESS</a> }
</code></pre>



</details>

<a name="bridge_message_types_remove_external_coin_witness"></a>

## Function `remove_external_coin_witness`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_witness">remove_external_coin_witness</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_witness">remove_external_coin_witness</a>(): u8 { <a href="../bridge/message_types.md#bridge_message_types_REMOVE_EXTERNAL_COIN_WITNESS">REMOVE_EXTERNAL_COIN_WITNESS</a> }
</code></pre>



</details>

<a name="bridge_message_types_add_external_coin_target"></a>

## Function `add_external_coin_target`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_target">add_external_coin_target</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_add_external_coin_target">add_external_coin_target</a>(): u8{
    <a href="../bridge/message_types.md#bridge_message_types_ADD_EXTERNAL_COIN_TARGET">ADD_EXTERNAL_COIN_TARGET</a>
}
</code></pre>



</details>

<a name="bridge_message_types_remove_external_coin_target"></a>

## Function `remove_external_coin_target`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_target">remove_external_coin_target</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/message_types.md#bridge_message_types_remove_external_coin_target">remove_external_coin_target</a>(): u8{
    <a href="../bridge/message_types.md#bridge_message_types_REMOVE_EXTERNAL_COIN_TARGET">REMOVE_EXTERNAL_COIN_TARGET</a>
}
</code></pre>



</details>

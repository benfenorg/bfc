---
title: Module `0xb::chain_ids`
---



-  [Struct `BridgeRoute`](#0xb_chain_ids_BridgeRoute)
-  [Constants](#@Constants_0)
-  [Function `sui_mainnet`](#0xb_chain_ids_sui_mainnet)
-  [Function `sui_testnet`](#0xb_chain_ids_sui_testnet)
-  [Function `sui_custom`](#0xb_chain_ids_sui_custom)
-  [Function `eth_mainnet`](#0xb_chain_ids_eth_mainnet)
-  [Function `eth_sepolia`](#0xb_chain_ids_eth_sepolia)
-  [Function `eth_custom`](#0xb_chain_ids_eth_custom)
-  [Function `btc_mainnet`](#0xb_chain_ids_btc_mainnet)
-  [Function `btc_testnet`](#0xb_chain_ids_btc_testnet)
-  [Function `bsc_mainnet`](#0xb_chain_ids_bsc_mainnet)
-  [Function `bsc_testnet`](#0xb_chain_ids_bsc_testnet)
-  [Function `bsc_custom`](#0xb_chain_ids_bsc_custom)
-  [Function `base_mainnet`](#0xb_chain_ids_base_mainnet)
-  [Function `base_testnet`](#0xb_chain_ids_base_testnet)
-  [Function `base_custom`](#0xb_chain_ids_base_custom)
-  [Function `op_mainnet`](#0xb_chain_ids_op_mainnet)
-  [Function `op_testnet`](#0xb_chain_ids_op_testnet)
-  [Function `op_custom`](#0xb_chain_ids_op_custom)
-  [Function `route_source`](#0xb_chain_ids_route_source)
-  [Function `route_destination`](#0xb_chain_ids_route_destination)
-  [Function `assert_valid_chain_id`](#0xb_chain_ids_assert_valid_chain_id)
-  [Function `valid_routes`](#0xb_chain_ids_valid_routes)
-  [Function `is_valid_route`](#0xb_chain_ids_is_valid_route)
-  [Function `get_route`](#0xb_chain_ids_get_route)


<pre><code><b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a name="0xb_chain_ids_BridgeRoute"></a>

## Struct `BridgeRoute`



<pre><code><b>struct</b> <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>source: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>destination: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xb_chain_ids_BaseCustom"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BaseCustom">BaseCustom</a>: u8 = 44;
</code></pre>



<a name="0xb_chain_ids_BaseMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BaseMainnet">BaseMainnet</a>: u8 = 42;
</code></pre>



<a name="0xb_chain_ids_BaseTestnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BaseTestnet">BaseTestnet</a>: u8 = 43;
</code></pre>



<a name="0xb_chain_ids_BscCustom"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BscCustom">BscCustom</a>: u8 = 32;
</code></pre>



<a name="0xb_chain_ids_BscMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BscMainnet">BscMainnet</a>: u8 = 30;
</code></pre>



<a name="0xb_chain_ids_BscTestnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BscTestnet">BscTestnet</a>: u8 = 31;
</code></pre>



<a name="0xb_chain_ids_BtcMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BtcMainnet">BtcMainnet</a>: u8 = 20;
</code></pre>



<a name="0xb_chain_ids_BtcTestnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_BtcTestnet">BtcTestnet</a>: u8 = 21;
</code></pre>



<a name="0xb_chain_ids_EInvalidBridgeRoute"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xb_chain_ids_EthCustom"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_EthCustom">EthCustom</a>: u8 = 12;
</code></pre>



<a name="0xb_chain_ids_EthMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a>: u8 = 10;
</code></pre>



<a name="0xb_chain_ids_EthSepolia"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a>: u8 = 11;
</code></pre>



<a name="0xb_chain_ids_OPCustom"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_OPCustom">OPCustom</a>: u8 = 35;
</code></pre>



<a name="0xb_chain_ids_OPMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_OPMainnet">OPMainnet</a>: u8 = 33;
</code></pre>



<a name="0xb_chain_ids_OPTestnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_OPTestnet">OPTestnet</a>: u8 = 34;
</code></pre>



<a name="0xb_chain_ids_SuiCustom"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>: u8 = 2;
</code></pre>



<a name="0xb_chain_ids_SuiMainnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a>: u8 = 0;
</code></pre>



<a name="0xb_chain_ids_SuiTestnet"></a>



<pre><code><b>const</b> <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>: u8 = 1;
</code></pre>



<a name="0xb_chain_ids_sui_mainnet"></a>

## Function `sui_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_sui_mainnet">sui_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_sui_mainnet">sui_mainnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_sui_testnet"></a>

## Function `sui_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_sui_testnet">sui_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_sui_testnet">sui_testnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_sui_custom"></a>

## Function `sui_custom`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_sui_custom">sui_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_sui_custom">sui_custom</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_eth_mainnet"></a>

## Function `eth_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_mainnet">eth_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_mainnet">eth_mainnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_eth_sepolia"></a>

## Function `eth_sepolia`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_sepolia">eth_sepolia</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_sepolia">eth_sepolia</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_eth_custom"></a>

## Function `eth_custom`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_custom">eth_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_eth_custom">eth_custom</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_EthCustom">EthCustom</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_btc_mainnet"></a>

## Function `btc_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_btc_mainnet">btc_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_btc_mainnet">btc_mainnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_BtcMainnet">BtcMainnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_btc_testnet"></a>

## Function `btc_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_btc_testnet">btc_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_btc_testnet">btc_testnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_BtcTestnet">BtcTestnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_bsc_mainnet"></a>

## Function `bsc_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">bsc_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_bsc_mainnet">bsc_mainnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_BscMainnet">BscMainnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_bsc_testnet"></a>

## Function `bsc_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_bsc_testnet">bsc_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_bsc_testnet">bsc_testnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_BscTestnet">BscTestnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_bsc_custom"></a>

## Function `bsc_custom`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_bsc_custom">bsc_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_bsc_custom">bsc_custom</a>(): u8  { <a href="chain_ids.md#0xb_chain_ids_BscCustom">BscCustom</a>  }
</code></pre>



</details>

<a name="0xb_chain_ids_base_mainnet"></a>

## Function `base_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_base_mainnet">base_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_base_mainnet">base_mainnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_BaseMainnet">BaseMainnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_base_testnet"></a>

## Function `base_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_base_testnet">base_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_base_testnet">base_testnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_BaseTestnet">BaseTestnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_base_custom"></a>

## Function `base_custom`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_base_custom">base_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_base_custom">base_custom</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_BaseCustom">BaseCustom</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_op_mainnet"></a>

## Function `op_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_op_mainnet">op_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_op_mainnet">op_mainnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_OPMainnet">OPMainnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_op_testnet"></a>

## Function `op_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_op_testnet">op_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_op_testnet">op_testnet</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_OPTestnet">OPTestnet</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_op_custom"></a>

## Function `op_custom`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_op_custom">op_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_op_custom">op_custom</a>(): u8 { <a href="chain_ids.md#0xb_chain_ids_OPCustom">OPCustom</a> }
</code></pre>



</details>

<a name="0xb_chain_ids_route_source"></a>

## Function `route_source`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_route_source">route_source</a>(route: &<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">chain_ids::BridgeRoute</a>): &u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_route_source">route_source</a>(route: &<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a>): &u8 {
    &route.source
}
</code></pre>



</details>

<a name="0xb_chain_ids_route_destination"></a>

## Function `route_destination`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_route_destination">route_destination</a>(route: &<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">chain_ids::BridgeRoute</a>): &u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_route_destination">route_destination</a>(route: &<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a>): &u8 {
    &route.destination
}
</code></pre>



</details>

<a name="0xb_chain_ids_assert_valid_chain_id"></a>

## Function `assert_valid_chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">assert_valid_chain_id</a>(id: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_assert_valid_chain_id">assert_valid_chain_id</a>(id: u8) {
    <b>assert</b>!(
        id == <a href="chain_ids.md#0xb_chain_ids_BtcMainnet">BtcMainnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_BtcTestnet">BtcTestnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_EthCustom">EthCustom</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_BaseMainnet">BaseMainnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_BaseTestnet">BaseTestnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_BaseCustom">BaseCustom</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_OPMainnet">OPMainnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_OPTestnet">OPTestnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_OPCustom">OPCustom</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_BscMainnet">BscMainnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_BscTestnet">BscTestnet</a> ||
        id == <a href="chain_ids.md#0xb_chain_ids_BscCustom">BscCustom</a>,

        <a href="chain_ids.md#0xb_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>
    )
}
</code></pre>



</details>

<a name="0xb_chain_ids_valid_routes"></a>

## Function `valid_routes`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_valid_routes">valid_routes</a>(): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">chain_ids::BridgeRoute</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_valid_routes">valid_routes</a>(): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a>&gt; {
    <a href="../move-stdlib/vector.md#0x1_vector">vector</a>[
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BtcMainnet">BtcMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BtcMainnet">BtcMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BtcTestnet">BtcTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BtcTestnet">BtcTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BtcTestnet">BtcTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BtcTestnet">BtcTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthMainnet">EthMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthCustom">EthCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthCustom">EthCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthCustom">EthCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_EthCustom">EthCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BscMainnet">BscMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_OPMainnet">OPMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BaseMainnet">BaseMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BscMainnet">BscMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_OPMainnet">OPMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BaseMainnet">BaseMainnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiMainnet">SuiMainnet</a> },


        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BscTestnet">BscTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BscCustom">BscCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BscTestnet">BscTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BscCustom">BscCustom</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_OPTestnet">OPTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_OPCustom">OPCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_OPTestnet">OPTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_OPCustom">OPCustom</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BaseTestnet">BaseTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BaseCustom">BaseCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BaseTestnet">BaseTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_BaseCustom">BaseCustom</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BscTestnet">BscTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BscTestnet">BscTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BscCustom">BscCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BscCustom">BscCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_OPTestnet">OPTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_OPTestnet">OPTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_OPCustom">OPCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_OPCustom">OPCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },

        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BaseTestnet">BaseTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BaseTestnet">BaseTestnet</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BaseCustom">BaseCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="chain_ids.md#0xb_chain_ids_BaseCustom">BaseCustom</a>, destination: <a href="chain_ids.md#0xb_chain_ids_SuiCustom">SuiCustom</a> }
    ]
}
</code></pre>



</details>

<a name="0xb_chain_ids_is_valid_route"></a>

## Function `is_valid_route`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool {
    <b>let</b> route = <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source, destination };
    <a href="chain_ids.md#0xb_chain_ids_valid_routes">valid_routes</a>().contains(&route)
}
</code></pre>



</details>

<a name="0xb_chain_ids_get_route"></a>

## Function `get_route`



<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_get_route">get_route</a>(source: u8, destination: u8): <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">chain_ids::BridgeRoute</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="chain_ids.md#0xb_chain_ids_get_route">get_route</a>(source: u8, destination: u8): <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> {
    <b>let</b> route = <a href="chain_ids.md#0xb_chain_ids_BridgeRoute">BridgeRoute</a> { source, destination };
    <b>assert</b>!(<a href="chain_ids.md#0xb_chain_ids_valid_routes">valid_routes</a>().contains(&route), <a href="chain_ids.md#0xb_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    route
}
</code></pre>



</details>

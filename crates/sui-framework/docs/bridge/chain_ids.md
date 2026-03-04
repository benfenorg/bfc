---
title: Module `bridge::chain_ids`
---



-  [Struct `BridgeRoute`](#bridge_chain_ids_BridgeRoute)
-  [Constants](#@Constants_0)
-  [Function `sui_mainnet`](#bridge_chain_ids_sui_mainnet)
-  [Function `sui_testnet`](#bridge_chain_ids_sui_testnet)
-  [Function `sui_custom`](#bridge_chain_ids_sui_custom)
-  [Function `eth_mainnet`](#bridge_chain_ids_eth_mainnet)
-  [Function `eth_sepolia`](#bridge_chain_ids_eth_sepolia)
-  [Function `eth_custom`](#bridge_chain_ids_eth_custom)
-  [Function `btc_mainnet`](#bridge_chain_ids_btc_mainnet)
-  [Function `btc_testnet`](#bridge_chain_ids_btc_testnet)
-  [Function `bsc_mainnet`](#bridge_chain_ids_bsc_mainnet)
-  [Function `bsc_testnet`](#bridge_chain_ids_bsc_testnet)
-  [Function `bsc_custom`](#bridge_chain_ids_bsc_custom)
-  [Function `base_mainnet`](#bridge_chain_ids_base_mainnet)
-  [Function `base_testnet`](#bridge_chain_ids_base_testnet)
-  [Function `base_custom`](#bridge_chain_ids_base_custom)
-  [Function `op_mainnet`](#bridge_chain_ids_op_mainnet)
-  [Function `op_testnet`](#bridge_chain_ids_op_testnet)
-  [Function `op_custom`](#bridge_chain_ids_op_custom)
-  [Function `arb_mainnet`](#bridge_chain_ids_arb_mainnet)
-  [Function `arb_testnet`](#bridge_chain_ids_arb_testnet)
-  [Function `arb_custom`](#bridge_chain_ids_arb_custom)
-  [Function `pol_mainnet`](#bridge_chain_ids_pol_mainnet)
-  [Function `pol_testnet`](#bridge_chain_ids_pol_testnet)
-  [Function `pol_custom`](#bridge_chain_ids_pol_custom)
-  [Function `avax_mainnet`](#bridge_chain_ids_avax_mainnet)
-  [Function `avax_testnet`](#bridge_chain_ids_avax_testnet)
-  [Function `avax_custom`](#bridge_chain_ids_avax_custom)
-  [Function `tron_mainnet`](#bridge_chain_ids_tron_mainnet)
-  [Function `tron_testnet`](#bridge_chain_ids_tron_testnet)
-  [Function `solana_mainnet`](#bridge_chain_ids_solana_mainnet)
-  [Function `solana_testnet`](#bridge_chain_ids_solana_testnet)
-  [Function `ltc_mainnet`](#bridge_chain_ids_ltc_mainnet)
-  [Function `ltc_testnet`](#bridge_chain_ids_ltc_testnet)
-  [Function `doge_mainnet`](#bridge_chain_ids_doge_mainnet)
-  [Function `doge_testnet`](#bridge_chain_ids_doge_testnet)
-  [Function `sui_official_mainnet`](#bridge_chain_ids_sui_official_mainnet)
-  [Function `sui_official_testnet`](#bridge_chain_ids_sui_official_testnet)
-  [Function `aptos_mainnet`](#bridge_chain_ids_aptos_mainnet)
-  [Function `aptos_testnet`](#bridge_chain_ids_aptos_testnet)
-  [Function `route_source`](#bridge_chain_ids_route_source)
-  [Function `route_destination`](#bridge_chain_ids_route_destination)
-  [Function `assert_valid_chain_id`](#bridge_chain_ids_assert_valid_chain_id)
-  [Function `valid_routes`](#bridge_chain_ids_valid_routes)
-  [Function `is_evm_l2`](#bridge_chain_ids_is_evm_l2)
-  [Function `is_eth`](#bridge_chain_ids_is_eth)
-  [Function `is_solana`](#bridge_chain_ids_is_solana)
-  [Function `is_valid_route`](#bridge_chain_ids_is_valid_route)
-  [Function `get_route`](#bridge_chain_ids_get_route)


<pre><code><b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="bridge_chain_ids_BridgeRoute"></a>

## Struct `BridgeRoute`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> <b>has</b> <b>copy</b>, drop, store
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


<a name="bridge_chain_ids_AptosMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosMainnet">AptosMainnet</a>: u8 = 58;
</code></pre>



<a name="bridge_chain_ids_AptosTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosTestnet">AptosTestnet</a>: u8 = 59;
</code></pre>



<a name="bridge_chain_ids_ArbCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a>: u8 = 38;
</code></pre>



<a name="bridge_chain_ids_ArbMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbMainnet">ArbMainnet</a>: u8 = 36;
</code></pre>



<a name="bridge_chain_ids_ArbTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a>: u8 = 37;
</code></pre>



<a name="bridge_chain_ids_AvaxCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a>: u8 = 47;
</code></pre>



<a name="bridge_chain_ids_AvaxMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxMainnet">AvaxMainnet</a>: u8 = 45;
</code></pre>



<a name="bridge_chain_ids_AvaxTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a>: u8 = 46;
</code></pre>



<a name="bridge_chain_ids_BaseCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a>: u8 = 44;
</code></pre>



<a name="bridge_chain_ids_BaseMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseMainnet">BaseMainnet</a>: u8 = 42;
</code></pre>



<a name="bridge_chain_ids_BaseTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a>: u8 = 43;
</code></pre>



<a name="bridge_chain_ids_BscCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a>: u8 = 32;
</code></pre>



<a name="bridge_chain_ids_BscMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BscMainnet">BscMainnet</a>: u8 = 30;
</code></pre>



<a name="bridge_chain_ids_BscTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a>: u8 = 31;
</code></pre>



<a name="bridge_chain_ids_BtcMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcMainnet">BtcMainnet</a>: u8 = 20;
</code></pre>



<a name="bridge_chain_ids_BtcTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcTestnet">BtcTestnet</a>: u8 = 21;
</code></pre>



<a name="bridge_chain_ids_DogeMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeMainnet">DogeMainnet</a>: u8 = 54;
</code></pre>



<a name="bridge_chain_ids_DogeTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeTestnet">DogeTestnet</a>: u8 = 55;
</code></pre>



<a name="bridge_chain_ids_EInvalidBridgeRoute"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>: u64 = 0;
</code></pre>



<a name="bridge_chain_ids_EthCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>: u8 = 12;
</code></pre>



<a name="bridge_chain_ids_EthMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a>: u8 = 10;
</code></pre>



<a name="bridge_chain_ids_EthSepolia"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a>: u8 = 11;
</code></pre>



<a name="bridge_chain_ids_LTCMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCMainnet">LTCMainnet</a>: u8 = 52;
</code></pre>



<a name="bridge_chain_ids_LTCTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCTestnet">LTCTestnet</a>: u8 = 53;
</code></pre>



<a name="bridge_chain_ids_OPCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a>: u8 = 35;
</code></pre>



<a name="bridge_chain_ids_OPMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_OPMainnet">OPMainnet</a>: u8 = 33;
</code></pre>



<a name="bridge_chain_ids_OPTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a>: u8 = 34;
</code></pre>



<a name="bridge_chain_ids_PolCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a>: u8 = 41;
</code></pre>



<a name="bridge_chain_ids_PolMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_PolMainnet">PolMainnet</a>: u8 = 39;
</code></pre>



<a name="bridge_chain_ids_PolTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a>: u8 = 40;
</code></pre>



<a name="bridge_chain_ids_SolanaMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaMainnet">SolanaMainnet</a>: u8 = 50;
</code></pre>



<a name="bridge_chain_ids_SolanaTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a>: u8 = 51;
</code></pre>



<a name="bridge_chain_ids_SuiCustom"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>: u8 = 2;
</code></pre>



<a name="bridge_chain_ids_SuiMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>: u8 = 0;
</code></pre>



<a name="bridge_chain_ids_SuiOfficialMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialMainnet">SuiOfficialMainnet</a>: u8 = 56;
</code></pre>



<a name="bridge_chain_ids_SuiOfficialTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialTestnet">SuiOfficialTestnet</a>: u8 = 57;
</code></pre>



<a name="bridge_chain_ids_SuiTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>: u8 = 1;
</code></pre>



<a name="bridge_chain_ids_TronMainnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_TronMainnet">TronMainnet</a>: u8 = 48;
</code></pre>



<a name="bridge_chain_ids_TronTestnet"></a>



<pre><code><b>const</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_TronTestnet">TronTestnet</a>: u8 = 49;
</code></pre>



<a name="bridge_chain_ids_sui_mainnet"></a>

## Function `sui_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">sui_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_mainnet">sui_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_sui_testnet"></a>

## Function `sui_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">sui_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_testnet">sui_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_sui_custom"></a>

## Function `sui_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">sui_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_custom">sui_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_eth_mainnet"></a>

## Function `eth_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">eth_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_mainnet">eth_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_eth_sepolia"></a>

## Function `eth_sepolia`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">eth_sepolia</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_sepolia">eth_sepolia</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_eth_custom"></a>

## Function `eth_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">eth_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_eth_custom">eth_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_btc_mainnet"></a>

## Function `btc_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">btc_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_mainnet">btc_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcMainnet">BtcMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_btc_testnet"></a>

## Function `btc_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">btc_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_btc_testnet">btc_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcTestnet">BtcTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_bsc_mainnet"></a>

## Function `bsc_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">bsc_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_mainnet">bsc_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_BscMainnet">BscMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_bsc_testnet"></a>

## Function `bsc_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">bsc_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_testnet">bsc_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_bsc_custom"></a>

## Function `bsc_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">bsc_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_bsc_custom">bsc_custom</a>(): u8  { <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a>  }
</code></pre>



</details>

<a name="bridge_chain_ids_base_mainnet"></a>

## Function `base_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">base_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_base_mainnet">base_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseMainnet">BaseMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_base_testnet"></a>

## Function `base_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">base_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_base_testnet">base_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_base_custom"></a>

## Function `base_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">base_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_base_custom">base_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_op_mainnet"></a>

## Function `op_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">op_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_op_mainnet">op_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_OPMainnet">OPMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_op_testnet"></a>

## Function `op_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">op_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_op_testnet">op_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_op_custom"></a>

## Function `op_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">op_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_op_custom">op_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_arb_mainnet"></a>

## Function `arb_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">arb_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_mainnet">arb_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbMainnet">ArbMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_arb_testnet"></a>

## Function `arb_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">arb_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_testnet">arb_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_arb_custom"></a>

## Function `arb_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">arb_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_arb_custom">arb_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_pol_mainnet"></a>

## Function `pol_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">pol_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_mainnet">pol_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_PolMainnet">PolMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_pol_testnet"></a>

## Function `pol_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">pol_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_testnet">pol_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_pol_custom"></a>

## Function `pol_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">pol_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_pol_custom">pol_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_avax_mainnet"></a>

## Function `avax_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">avax_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_mainnet">avax_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxMainnet">AvaxMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_avax_testnet"></a>

## Function `avax_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">avax_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_testnet">avax_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_avax_custom"></a>

## Function `avax_custom`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">avax_custom</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_avax_custom">avax_custom</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_tron_mainnet"></a>

## Function `tron_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">tron_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_mainnet">tron_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_TronMainnet">TronMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_tron_testnet"></a>

## Function `tron_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">tron_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_tron_testnet">tron_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_TronTestnet">TronTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_solana_mainnet"></a>

## Function `solana_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">solana_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_mainnet">solana_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaMainnet">SolanaMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_solana_testnet"></a>

## Function `solana_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">solana_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_solana_testnet">solana_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_ltc_mainnet"></a>

## Function `ltc_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_mainnet">ltc_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_mainnet">ltc_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCMainnet">LTCMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_ltc_testnet"></a>

## Function `ltc_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_testnet">ltc_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_ltc_testnet">ltc_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCTestnet">LTCTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_doge_mainnet"></a>

## Function `doge_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_mainnet">doge_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_mainnet">doge_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeMainnet">DogeMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_doge_testnet"></a>

## Function `doge_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_testnet">doge_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_doge_testnet">doge_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeTestnet">DogeTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_sui_official_mainnet"></a>

## Function `sui_official_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_mainnet">sui_official_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_mainnet">sui_official_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialMainnet">SuiOfficialMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_sui_official_testnet"></a>

## Function `sui_official_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">sui_official_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_sui_official_testnet">sui_official_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialTestnet">SuiOfficialTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_aptos_mainnet"></a>

## Function `aptos_mainnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_mainnet">aptos_mainnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_mainnet">aptos_mainnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosMainnet">AptosMainnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_aptos_testnet"></a>

## Function `aptos_testnet`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">aptos_testnet</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_aptos_testnet">aptos_testnet</a>(): u8 { <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosTestnet">AptosTestnet</a> }
</code></pre>



</details>

<a name="bridge_chain_ids_route_source"></a>

## Function `route_source`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_source">route_source</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): &u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_source">route_source</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a>): &u8 {
    &route.source
}
</code></pre>



</details>

<a name="bridge_chain_ids_route_destination"></a>

## Function `route_destination`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_destination">route_destination</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>): &u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_route_destination">route_destination</a>(route: &<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a>): &u8 {
    &route.destination
}
</code></pre>



</details>

<a name="bridge_chain_ids_assert_valid_chain_id"></a>

## Function `assert_valid_chain_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_assert_valid_chain_id">assert_valid_chain_id</a>(id: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_assert_valid_chain_id">assert_valid_chain_id</a>(id: u8) {
    <b>assert</b>!(
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcMainnet">BtcMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcTestnet">BtcTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseMainnet">BaseMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_OPMainnet">OPMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BscMainnet">BscMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbMainnet">ArbMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_PolMainnet">PolMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxMainnet">AvaxMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_TronMainnet">TronMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_TronTestnet">TronTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaMainnet">SolanaMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCMainnet">LTCMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCTestnet">LTCTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeMainnet">DogeMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeTestnet">DogeTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialMainnet">SuiOfficialMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialTestnet">SuiOfficialTestnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosMainnet">AptosMainnet</a> ||
        id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosTestnet">AptosTestnet</a>,
        <a href="../bridge/chain_ids.md#bridge_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>
    )
}
</code></pre>



</details>

<a name="bridge_chain_ids_valid_routes"></a>

## Function `valid_routes`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>(): vector&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>(): vector&lt;<a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a>&gt; {
    vector[
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcMainnet">BtcMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcMainnet">BtcMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcTestnet">BtcTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcTestnet">BtcTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcTestnet">BtcTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BtcTestnet">BtcTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        // tron
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_TronMainnet">TronMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_TronMainnet">TronMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_TronTestnet">TronTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_TronTestnet">TronTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_TronTestnet">TronTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_TronTestnet">TronTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        // solana
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaMainnet">SolanaMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaMainnet">SolanaMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        // ltc
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCMainnet">LTCMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCMainnet">LTCMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCTestnet">LTCTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCTestnet">LTCTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCTestnet">LTCTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_LTCTestnet">LTCTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        // doge
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeMainnet">DogeMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeMainnet">DogeMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeTestnet">DogeTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeTestnet">DogeTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeTestnet">DogeTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_DogeTestnet">DogeTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        // sui official
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialMainnet">SuiOfficialMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialMainnet">SuiOfficialMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialTestnet">SuiOfficialTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialTestnet">SuiOfficialTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialTestnet">SuiOfficialTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiOfficialTestnet">SuiOfficialTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        // aptos
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosMainnet">AptosMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosMainnet">AptosMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosTestnet">AptosTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosTestnet">AptosTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosTestnet">AptosTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AptosTestnet">AptosTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscMainnet">BscMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPMainnet">OPMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseMainnet">BaseMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscMainnet">BscMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPMainnet">OPMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseMainnet">BaseMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbMainnet">ArbMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolMainnet">PolMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxMainnet">AvaxMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbMainnet">ArbMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolMainnet">PolMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxMainnet">AvaxMainnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiMainnet">SuiMainnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiTestnet">SuiTestnet</a> },
        <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source: <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a>, destination: <a href="../bridge/chain_ids.md#bridge_chain_ids_SuiCustom">SuiCustom</a> },
    ]
}
</code></pre>



</details>

<a name="bridge_chain_ids_is_evm_l2"></a>

## Function `is_evm_l2`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_evm_l2">is_evm_l2</a>(chain_id: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_evm_l2">is_evm_l2</a>(chain_id: u8): bool {
    // Commonly recognized EVM L2s in this file:
    // OP - Optimism
    // Arbitrum
    // Base
    // Polygon (POL)
    // bsc
    // avax
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_OPMainnet">OPMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_OPTestnet">OPTestnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_OPCustom">OPCustom</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbMainnet">ArbMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbTestnet">ArbTestnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_ArbCustom">ArbCustom</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseMainnet">BaseMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseTestnet">BaseTestnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BaseCustom">BaseCustom</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_PolMainnet">PolMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_PolTestnet">PolTestnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_PolCustom">PolCustom</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BscMainnet">BscMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BscTestnet">BscTestnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_BscCustom">BscCustom</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxMainnet">AvaxMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxTestnet">AvaxTestnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_AvaxCustom">AvaxCustom</a>
}
</code></pre>



</details>

<a name="bridge_chain_ids_is_eth"></a>

## Function `is_eth`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_eth">is_eth</a>(chain_id: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_eth">is_eth</a>(chain_id: u8): bool {
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthMainnet">EthMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthSepolia">EthSepolia</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_EthCustom">EthCustom</a>
}
</code></pre>



</details>

<a name="bridge_chain_ids_is_solana"></a>

## Function `is_solana`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_solana">is_solana</a>(chain_id: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_solana">is_solana</a>(chain_id: u8): bool {
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaMainnet">SolanaMainnet</a> ||
    chain_id == <a href="../bridge/chain_ids.md#bridge_chain_ids_SolanaTestnet">SolanaTestnet</a>
}
</code></pre>



</details>

<a name="bridge_chain_ids_is_valid_route"></a>

## Function `is_valid_route`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_is_valid_route">is_valid_route</a>(source: u8, destination: u8): bool {
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source, destination };
    <a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>().contains(&route)
}
</code></pre>



</details>

<a name="bridge_chain_ids_get_route"></a>

## Function `get_route`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">get_route</a>(source: u8, destination: u8): <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">bridge::chain_ids::BridgeRoute</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/chain_ids.md#bridge_chain_ids_get_route">get_route</a>(source: u8, destination: u8): <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> {
    <b>let</b> route = <a href="../bridge/chain_ids.md#bridge_chain_ids_BridgeRoute">BridgeRoute</a> { source, destination };
    <b>assert</b>!(<a href="../bridge/chain_ids.md#bridge_chain_ids_valid_routes">valid_routes</a>().contains(&route), <a href="../bridge/chain_ids.md#bridge_chain_ids_EInvalidBridgeRoute">EInvalidBridgeRoute</a>);
    route
}
</code></pre>



</details>


<a name="0xc8_bfc_system_state_inner"></a>

# Module `0xc8::bfc_system_state_inner`



-  [Struct `BfcSystemStateInner`](#0xc8_bfc_system_state_inner_BfcSystemStateInner)
-  [Struct `TreasuryParameters`](#0xc8_bfc_system_state_inner_TreasuryParameters)
-  [Struct `BfcSystemParameters`](#0xc8_bfc_system_state_inner_BfcSystemParameters)
-  [Constants](#@Constants_0)
-  [Function `create_inner_state`](#0xc8_bfc_system_state_inner_create_inner_state)
-  [Function `create_stake_manager_key`](#0xc8_bfc_system_state_inner_create_stake_manager_key)
-  [Function `unstake_manager_key`](#0xc8_bfc_system_state_inner_unstake_manager_key)
-  [Function `update_round`](#0xc8_bfc_system_state_inner_update_round)
-  [Function `request_exchange_stable`](#0xc8_bfc_system_state_inner_request_exchange_stable)
-  [Function `request_exchange_all`](#0xc8_bfc_system_state_inner_request_exchange_all)
-  [Function `request_withdraw_stable`](#0xc8_bfc_system_state_inner_request_withdraw_stable)
-  [Function `requst_get_exchange_rate`](#0xc8_bfc_system_state_inner_requst_get_exchange_rate)
-  [Function `request_add_gas_coin`](#0xc8_bfc_system_state_inner_request_add_gas_coin)
-  [Function `request_update_gas_coin`](#0xc8_bfc_system_state_inner_request_update_gas_coin)
-  [Function `request_remove_gas_coin`](#0xc8_bfc_system_state_inner_request_remove_gas_coin)
-  [Function `init_exchange_pool`](#0xc8_bfc_system_state_inner_init_exchange_pool)
-  [Function `create_treasury`](#0xc8_bfc_system_state_inner_create_treasury)
-  [Function `swap_bfc_to_stablecoin`](#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin)
-  [Function `swap_bfc_to_stablecoin_balance`](#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance)
-  [Function `swap_stablecoin_to_bfc`](#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc)
-  [Function `swap_stablecoin_to_bfc_balance`](#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance)
-  [Function `get_stablecoin_by_bfc`](#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc)
-  [Function `get_bfc_by_stablecoin`](#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin)
-  [Function `next_epoch_bfc_required`](#0xc8_bfc_system_state_inner_next_epoch_bfc_required)
-  [Function `treasury_balance`](#0xc8_bfc_system_state_inner_treasury_balance)
-  [Function `deposit_to_treasury`](#0xc8_bfc_system_state_inner_deposit_to_treasury)
-  [Function `rebalance`](#0xc8_bfc_system_state_inner_rebalance)
-  [Function `vault_info`](#0xc8_bfc_system_state_inner_vault_info)
-  [Function `bfc_system_stat_parameter`](#0xc8_bfc_system_state_inner_bfc_system_stat_parameter)
-  [Function `create_bfcdao_action`](#0xc8_bfc_system_state_inner_create_bfcdao_action)
-  [Function `propose`](#0xc8_bfc_system_state_inner_propose)
-  [Function `set_voting_delay`](#0xc8_bfc_system_state_inner_set_voting_delay)
-  [Function `set_voting_period`](#0xc8_bfc_system_state_inner_set_voting_period)
-  [Function `set_voting_quorum_rate`](#0xc8_bfc_system_state_inner_set_voting_quorum_rate)
-  [Function `set_min_action_delay`](#0xc8_bfc_system_state_inner_set_min_action_delay)
-  [Function `destroy_terminated_proposal`](#0xc8_bfc_system_state_inner_destroy_terminated_proposal)
-  [Function `judge_proposal_state`](#0xc8_bfc_system_state_inner_judge_proposal_state)
-  [Function `modify_proposal`](#0xc8_bfc_system_state_inner_modify_proposal)
-  [Function `cast_vote`](#0xc8_bfc_system_state_inner_cast_vote)
-  [Function `change_vote`](#0xc8_bfc_system_state_inner_change_vote)
-  [Function `queue_proposal_action`](#0xc8_bfc_system_state_inner_queue_proposal_action)
-  [Function `revoke_vote`](#0xc8_bfc_system_state_inner_revoke_vote)
-  [Function `withdraw_voting`](#0xc8_bfc_system_state_inner_withdraw_voting)
-  [Function `create_voting_bfc`](#0xc8_bfc_system_state_inner_create_voting_bfc)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="bfc_dao.md#0xc8_bfc_dao">0xc8::bfc_dao</a>;
<b>use</b> <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager">0xc8::bfc_dao_manager</a>;
<b>use</b> <a href="busd.md#0xc8_busd">0xc8::busd</a>;
<b>use</b> <a href="exchange_inner.md#0xc8_exchange_inner">0xc8::exchange_inner</a>;
<b>use</b> <a href="gas_coin_map.md#0xc8_gas_coin_map">0xc8::gas_coin_map</a>;
<b>use</b> <a href="treasury.md#0xc8_treasury">0xc8::treasury</a>;
<b>use</b> <a href="treasury_pool.md#0xc8_treasury_pool">0xc8::treasury_pool</a>;
<b>use</b> <a href="vault.md#0xc8_vault">0xc8::vault</a>;
<b>use</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool">0xc8::voting_pool</a>;
</code></pre>



<a name="0xc8_bfc_system_state_inner_BfcSystemStateInner"></a>

## Struct `BfcSystemStateInner`



<pre><code><b>struct</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>round: u64</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>: <a href="gas_coin_map.md#0xc8_gas_coin_map_GasCoinMap">gas_coin_map::GasCoinMap</a></code>
</dt>
<dd>
 Contains gas coin information
</dd>
<dt>
<code>exchange_pool: <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;<a href="busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;</code>
</dt>
<dd>
 Exchange gas coin pool
</dd>
<dt>
<code>dao: <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="treasury.md#0xc8_treasury">treasury</a>: <a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>: <a href="treasury_pool.md#0xc8_treasury_pool_TreasuryPool">treasury_pool::TreasuryPool</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_system_state_inner_TreasuryParameters"></a>

## Struct `TreasuryParameters`



<pre><code><b>struct</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">TreasuryParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>position_number: u32</code>
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
<code>time_interval: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>max_counter_times: u32</code>
</dt>
<dd>

</dd>
<dt>
<code>base_point: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>initialize_price: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_system_state_inner_BfcSystemParameters"></a>

## Struct `BfcSystemParameters`



<pre><code><b>struct</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>chain_start_timestamp_ms: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>treasury_parameters: <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">bfc_system_state_inner::TreasuryParameters</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_bfc_system_state_inner_BFC_SYSTEM_STATE_START_ROUND"></a>



<pre><code><b>const</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BFC_SYSTEM_STATE_START_ROUND">BFC_SYSTEM_STATE_START_ROUND</a>: u64 = 0;
</code></pre>



<a name="0xc8_bfc_system_state_inner_BFC_SYSTEM_TREASURY_KEY"></a>



<pre><code><b>const</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BFC_SYSTEM_TREASURY_KEY">BFC_SYSTEM_TREASURY_KEY</a>: u64 = 1;
</code></pre>



<a name="0xc8_bfc_system_state_inner_DEFAULT_ADMIN_ADDRESSES"></a>



<pre><code><b>const</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>: <a href="">vector</a>&lt;<b>address</b>&gt; = [];
</code></pre>



<a name="0xc8_bfc_system_state_inner_create_inner_state"></a>

## Function `create_inner_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_inner_state">create_inner_state</a>(usd_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;, bfc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, parameters: <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_inner_state">create_inner_state</a>(
    usd_supply: Supply&lt;BUSD&gt;,
    bfc_balance: Balance&lt;BFC&gt;,
    parameters: <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a> {
    // init gas <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a> mappings
    <b>let</b> init_gas_coins_map = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>&lt;<b>address</b>, GasCoinEntity&gt;();
    <b>let</b> <a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a> = <a href="gas_coin_map.md#0xc8_gas_coin_map_new">gas_coin_map::new</a>(init_gas_coins_map, ctx);
    <b>let</b> exchange_pool = <a href="exchange_inner.md#0xc8_exchange_inner_new_exchange_pool">exchange_inner::new_exchange_pool</a>&lt;BUSD&gt;(ctx, 0);
    <b>let</b> dao = <a href="bfc_dao.md#0xc8_bfc_dao_create_dao">bfc_dao::create_dao</a>(<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>, ctx);
    <b>let</b> (t, remain_balance) = <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_treasury">create_treasury</a>(usd_supply, bfc_balance, parameters, ctx);
    <b>let</b> tp = <a href="treasury_pool.md#0xc8_treasury_pool_create_treasury_pool">treasury_pool::create_treasury_pool</a>(remain_balance, ctx);

    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a> {
        round: <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BFC_SYSTEM_STATE_START_ROUND">BFC_SYSTEM_STATE_START_ROUND</a>,
        <a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>,
        exchange_pool,
        dao,
        <a href="treasury.md#0xc8_treasury">treasury</a>: t,
        <a href="treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>: tp,
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_stake_manager_key"></a>

## Function `create_stake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_stake_manager_key">create_stake_manager_key</a>(payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> (<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_stake_manager_key">create_stake_manager_key</a>( payment: Coin&lt;BFC&gt;,
                                              ctx: &<b>mut</b> TxContext) {
    <a href="bfc_dao.md#0xc8_bfc_dao_create_stake_manager_key">bfc_dao::create_stake_manager_key</a>(payment, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_unstake_manager_key"></a>

## Function `unstake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_unstake_manager_key">unstake_manager_key</a>(key: <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, token: <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_ManagerKeyBfc">bfc_dao_manager::ManagerKeyBfc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_unstake_manager_key">unstake_manager_key</a>(key: BFCDaoManageKey,
                                       token: ManagerKeyBfc,
                                       ctx: &<b>mut</b> TxContext) {
    <a href="bfc_dao.md#0xc8_bfc_dao_unstake_manager_key">bfc_dao::unstake_manager_key</a>(key, token, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_update_round"></a>

## Function `update_round`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round">update_round</a>(inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, round: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_update_round">update_round</a>(
    inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    round: u64,
) {
    inner.round = round;
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_request_exchange_stable"></a>

## Function `request_exchange_stable`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_exchange_stable">request_exchange_stable</a>(inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_exchange_stable">request_exchange_stable</a>(
    inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: Coin&lt;BUSD&gt;,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    //get exchange rate
    <b>let</b> rate = <a href="gas_coin_map.md#0xc8_gas_coin_map_requst_get_exchange_rate">gas_coin_map::requst_get_exchange_rate</a>&lt;BUSD&gt;(&inner.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, &<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>);
    <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_stable">exchange_inner::request_exchange_stable</a>&lt;BUSD&gt;(rate, &<b>mut</b> inner.exchange_pool, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_request_exchange_all"></a>

## Function `request_exchange_all`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_exchange_all">request_exchange_all</a>(inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_exchange_all">request_exchange_all</a>(
    inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    ctx: &<b>mut</b> TxContext
) {
    //get <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a> amount of inner exchange pool
    <b>let</b> bfc_amount = <a href="exchange_inner.md#0xc8_exchange_inner_get_bfc_amount">exchange_inner::get_bfc_amount</a>(&inner.exchange_pool);
    <b>if</b> (bfc_amount &gt; 0) {
        //set pool is disactivate
        <b>let</b> epoch = <a href="exchange_inner.md#0xc8_exchange_inner_dis_activate">exchange_inner::dis_activate</a>(&<b>mut</b> inner.exchange_pool);
        //get <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>
        <b>let</b> stable_balance = <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_all_stable">exchange_inner::request_withdraw_all_stable</a>(&<b>mut</b> inner.exchange_pool);
        //exchange from <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> swap
        <b>let</b> bfc_balance = <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance">swap_stablecoin_to_bfc_balance</a>(
            inner,
            <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(stable_balance, ctx),
            ctx,
        );
        //add <a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc">bfc</a> <b>to</b> inner exchange pool
        <a href="exchange_inner.md#0xc8_exchange_inner_request_deposit_bfc_balance">exchange_inner::request_deposit_bfc_balance</a>(&<b>mut</b> inner.exchange_pool, bfc_balance);
        // active pool
        <a href="exchange_inner.md#0xc8_exchange_inner_activate">exchange_inner::activate</a>(&<b>mut</b> inner.exchange_pool, epoch);
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_request_withdraw_stable"></a>

## Function `request_withdraw_stable`

Request withdraw stable coin.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_withdraw_stable">request_withdraw_stable</a>(inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_withdraw_stable">request_withdraw_stable</a>(
    inner: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
): Balance&lt;BUSD&gt; {
    <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_all_stable">exchange_inner::request_withdraw_all_stable</a>(&<b>mut</b> inner.exchange_pool)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_requst_get_exchange_rate"></a>

## Function `requst_get_exchange_rate`

Getter of the gas coin exchange pool rate.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, _stable: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(
    self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    _stable: &Coin&lt;CoinType&gt;
): u64 {
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;CoinType&gt;(
        self,
        <a href="gas_coin_map.md#0xc8_gas_coin_map_get_default_rate">gas_coin_map::get_default_rate</a>(),
    )
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_request_add_gas_coin"></a>

## Function `request_add_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;, rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
    rate: u64,
) {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_add_gas_coin">gas_coin_map::request_add_gas_coin</a>&lt;CoinType&gt;(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_request_update_gas_coin"></a>

## Function `request_update_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
) {
    <b>let</b> rate = <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;CoinType&gt;(
        self,
        <a href="gas_coin_map.md#0xc8_gas_coin_map_get_default_rate">gas_coin_map::get_default_rate</a>(),
    );
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_update_gas_coin">gas_coin_map::request_update_gas_coin</a>(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_request_remove_gas_coin"></a>

## Function `request_remove_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
) {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_remove_gas_coin">gas_coin_map::request_remove_gas_coin</a>&lt;CoinType&gt;(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_init_exchange_pool"></a>

## Function `init_exchange_pool`

Init exchange pool by add bfc coin.


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_exchange_pool">init_exchange_pool</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_init_exchange_pool">init_exchange_pool</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;BFC&gt;,
) {
    <a href="exchange_inner.md#0xc8_exchange_inner_add_bfc_to_pool">exchange_inner::add_bfc_to_pool</a>(&<b>mut</b> self.exchange_pool, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_treasury"></a>

## Function `create_treasury`

X treasury  init treasury


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_treasury">create_treasury</a>(supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="busd.md#0xc8_busd_BUSD">busd::BUSD</a>&gt;, bfc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, parameters: <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_treasury">create_treasury</a>(
    supply: Supply&lt;BUSD&gt;,
    bfc_balance: Balance&lt;BFC&gt;,
    parameters: <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext
): (Treasury, Balance&lt;BFC&gt;) {
    <b>let</b> treasury_parameters = parameters.treasury_parameters;
    <b>let</b> t = <a href="treasury.md#0xc8_treasury_create_treasury">treasury::create_treasury</a>(treasury_parameters.time_interval, ctx);

    <a href="treasury.md#0xc8_treasury_init_vault_with_positions">treasury::init_vault_with_positions</a>&lt;BUSD&gt;(
        &<b>mut</b> t,
        supply,
        treasury_parameters.initialize_price,
        treasury_parameters.base_point,
        treasury_parameters.position_number,
        treasury_parameters.tick_spacing,
        treasury_parameters.spacing_times,
        treasury_parameters.max_counter_times,
        parameters.chain_start_timestamp_ms,
        ctx,
    );
    <b>if</b> (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>&lt;BFC&gt;(&bfc_balance) &gt; 0) {
        <b>let</b> deposit_balance = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> bfc_balance, <a href="treasury.md#0xc8_treasury_next_epoch_bfc_required">treasury::next_epoch_bfc_required</a>(&t));
        <a href="treasury.md#0xc8_treasury_deposit">treasury::deposit</a>(&<b>mut</b> t, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(deposit_balance, ctx));
        <a href="treasury.md#0xc8_treasury_rebalance_first_init">treasury::rebalance_first_init</a>(&<b>mut</b> t, ctx);
    };
    (t, bfc_balance)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin"></a>

## Function `swap_bfc_to_stablecoin`

swap bfc to stablecoin


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, coin_bfc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin">swap_bfc_to_stablecoin</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    coin_bfc: Coin&lt;BFC&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="treasury.md#0xc8_treasury_mint">treasury::mint</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_bfc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance"></a>

## Function `swap_bfc_to_stablecoin_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance">swap_bfc_to_stablecoin_balance</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, coin_bfc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_bfc_to_stablecoin_balance">swap_bfc_to_stablecoin_balance</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    coin_bfc: Coin&lt;BFC&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_mint_internal">treasury::mint_internal</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_bfc, amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc"></a>

## Function `swap_stablecoin_to_bfc`

swap stablecoin to bfc


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc">swap_stablecoin_to_bfc</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="treasury.md#0xc8_treasury_redeem">treasury::redeem</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_sc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance"></a>

## Function `swap_stablecoin_to_bfc_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance">swap_stablecoin_to_bfc_balance</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_swap_stablecoin_to_bfc_balance">swap_stablecoin_to_bfc_balance</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;BFC&gt; {
    <b>let</b> amount = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&coin_sc);
    <a href="treasury.md#0xc8_treasury_redeem_internal">treasury::redeem_internal</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_sc, amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_stablecoin_by_bfc"></a>

## Function `get_stablecoin_by_bfc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_stablecoin_by_bfc">get_stablecoin_by_bfc</a>&lt;StableCoinType&gt;(
    self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    amount: u64
): u64
{
    <a href="treasury.md#0xc8_treasury_calculate_swap_result">treasury::calculate_swap_result</a>&lt;StableCoinType&gt;(&self.<a href="treasury.md#0xc8_treasury">treasury</a>, <b>false</b>, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_get_bfc_by_stablecoin"></a>

## Function `get_bfc_by_stablecoin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_get_bfc_by_stablecoin">get_bfc_by_stablecoin</a>&lt;StableCoinType&gt;(
    self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    amount: u64
): u64
{
    <a href="treasury.md#0xc8_treasury_calculate_swap_result">treasury::calculate_swap_result</a>&lt;StableCoinType&gt;(&self.<a href="treasury.md#0xc8_treasury">treasury</a>, <b>true</b>, amount)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_next_epoch_bfc_required"></a>

## Function `next_epoch_bfc_required`

X-treasury


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_next_epoch_bfc_required">next_epoch_bfc_required</a>(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_next_epoch_bfc_required">next_epoch_bfc_required</a>(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): u64 {
    <a href="treasury.md#0xc8_treasury_next_epoch_bfc_required">treasury::next_epoch_bfc_required</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_treasury_balance"></a>

## Function `treasury_balance`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): u64 {
    <a href="treasury.md#0xc8_treasury_get_balance">treasury::get_balance</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_deposit_to_treasury"></a>

## Function `deposit_to_treasury`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, coin_bfc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>, coin_bfc: Coin&lt;BFC&gt;) {
    <a href="treasury.md#0xc8_treasury_deposit">treasury::deposit</a>(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_bfc);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance">rebalance</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_rebalance">rebalance</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> amount = <a href="treasury.md#0xc8_treasury_next_epoch_bfc_required">treasury::next_epoch_bfc_required</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>);
    <b>let</b> withdraw_balance =
        <a href="treasury_pool.md#0xc8_treasury_pool_withdraw_to_treasury">treasury_pool::withdraw_to_treasury</a>(&<b>mut</b> self.<a href="treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>, amount, ctx);
    <b>if</b> (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>(&withdraw_balance) &gt; 0) {
        <a href="treasury.md#0xc8_treasury_deposit">treasury::deposit</a>(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(withdraw_balance, ctx));
    } <b>else</b> {
        <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_destroy_zero">balance::destroy_zero</a>(withdraw_balance);
    };
    <a href="treasury.md#0xc8_treasury_rebalance">treasury::rebalance</a>(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_vault_info"></a>

## Function `vault_info`

X-vault


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info">vault_info</a>&lt;StableCoinType&gt;(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>): <a href="vault.md#0xc8_vault_VaultInfo">vault::VaultInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_vault_info">vault_info</a>&lt;StableCoinType&gt;(self: &<a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>): VaultInfo {
    <a href="treasury.md#0xc8_treasury_vault_info">treasury::vault_info</a>&lt;StableCoinType&gt;(&self.<a href="treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_bfc_system_stat_parameter"></a>

## Function `bfc_system_stat_parameter`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_system_stat_parameter">bfc_system_stat_parameter</a>(position_number: u32, tick_spacing: u32, spacing_times: u32, initialize_price: u128, time_interval: u32, base_point: u64, max_counter_times: u32, chain_start_timestamp_ms: u64): <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">bfc_system_state_inner::BfcSystemParameters</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_bfc_system_stat_parameter">bfc_system_stat_parameter</a>(
    position_number: u32,
    tick_spacing: u32,
    spacing_times: u32,
    initialize_price: u128,
    time_interval: u32,
    base_point: u64,
    max_counter_times: u32,
    chain_start_timestamp_ms: u64,
): <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a> {
    <b>let</b> treasury_parameters = <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_TreasuryParameters">TreasuryParameters</a> {
        position_number,
        tick_spacing,
        spacing_times,
        initialize_price,
        time_interval,
        max_counter_times,
        base_point,
    };
    <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemParameters">BfcSystemParameters</a> {
        treasury_parameters,
        chain_start_timestamp_ms,
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_bfcdao_action"></a>

## Function `create_bfcdao_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfcdao_action">create_bfcdao_action</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, actionName: <a href="">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_bfcdao_action">create_bfcdao_action</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    payment: Coin&lt;BFC&gt;,

    actionName: <a href="">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext) {
    <a href="bfc_dao.md#0xc8_bfc_dao_create_bfcdao_action">bfc_dao::create_bfcdao_action</a>(&<b>mut</b> self.dao, payment, actionName, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_propose"></a>

## Function `propose`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_propose">propose</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, version_id: u64, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, action_id: u64, action_delay: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_propose">propose</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    version_id: u64,
    payment: Coin&lt;BFC&gt;,
    action_id: u64,
    action_delay: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    bfc_dao:: propose(&<b>mut</b> self.dao, version_id, payment, action_id, action_delay, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_voting_delay"></a>

## Function `set_voting_delay`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>, manager_key: &BFCDaoManageKey, value: u64) {
    <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_delay">bfc_dao::set_voting_delay</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_voting_period"></a>

## Function `set_voting_period`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_period">set_voting_period</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_period">set_voting_period</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    manager_key: &BFCDaoManageKey,
    value: u64,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_period">bfc_dao::set_voting_period</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_voting_quorum_rate"></a>

## Function `set_voting_quorum_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_quorum_rate">set_voting_quorum_rate</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_voting_quorum_rate">set_voting_quorum_rate</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    manager_key: &BFCDaoManageKey,
    value: u8,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_quorum_rate">bfc_dao::set_voting_quorum_rate</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_set_min_action_delay"></a>

## Function `set_min_action_delay`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_min_action_delay">set_min_action_delay</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_set_min_action_delay">set_min_action_delay</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    manager_key: &BFCDaoManageKey,
    value: u64,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_set_min_action_delay">bfc_dao::set_min_action_delay</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    self: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_destroy_terminated_proposal">bfc_dao::destroy_terminated_proposal</a>(&<b>mut</b> self.dao, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, current_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>, current_time: u64) {
    <b>let</b> proposal_record = <a href="bfc_dao.md#0xc8_bfc_dao_getProposalRecord">bfc_dao::getProposalRecord</a>(&<b>mut</b> wrapper.dao);
    <b>let</b> size: u64 = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&proposal_record);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; size) {
        <b>let</b> (_, proposalInfo) = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get_entry_by_idx">vec_map::get_entry_by_idx</a>(&proposal_record, size - 1);
        <b>let</b> cur_status = <a href="bfc_dao.md#0xc8_bfc_dao_judge_proposal_state">bfc_dao::judge_proposal_state</a>(proposalInfo, current_time);
        <a href="bfc_dao.md#0xc8_bfc_dao_set_current_status_into_dao">bfc_dao::set_current_status_into_dao</a>(&<b>mut</b> wrapper.dao, proposalInfo, cur_status);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_modify_proposal"></a>

## Function `modify_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_modify_proposal">modify_proposal</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, proposal_obj: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_modify_proposal">modify_proposal</a>(
    system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    proposal_obj: &<b>mut</b> Proposal,
    index: u8,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_modify_proposal_obj">bfc_dao::modify_proposal_obj</a>(&<b>mut</b> system_state.dao, proposal_obj, index, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_cast_vote"></a>

## Function `cast_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_cast_vote">cast_vote</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, agreeInt: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_cast_vote">cast_vote</a>(
    system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: VotingBfc,
    agreeInt: u8,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_cast_vote">bfc_dao::cast_vote</a>(&<b>mut</b> system_state.dao, proposal, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, agreeInt, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_change_vote"></a>

## Function `change_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_change_vote">change_vote</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, my_vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, agree: bool, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_change_vote">change_vote</a>(
    system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    my_vote: &<b>mut</b> Vote,
    proposal: &<b>mut</b> Proposal,
    agree: bool,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_change_vote">bfc_dao::change_vote</a>(&<b>mut</b> system_state.dao, my_vote, proposal, agree, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_queue_proposal_action"></a>

## Function `queue_proposal_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_queue_proposal_action">queue_proposal_action</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, manager_key: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_queue_proposal_action">queue_proposal_action</a>(
    system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    manager_key: &BFCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_queue_proposal_action">bfc_dao::queue_proposal_action</a>(&<b>mut</b> system_state.dao, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_revoke_vote"></a>

## Function `revoke_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_revoke_vote">revoke_vote</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, my_vote: <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, voting_power: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_revoke_vote">revoke_vote</a>(
    system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
    proposal: &<b>mut</b> Proposal,
    my_vote: Vote,
    voting_power: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="bfc_dao.md#0xc8_bfc_dao_revoke_vote">bfc_dao::revoke_vote</a>(&<b>mut</b> system_state.dao, proposal, my_vote, voting_power, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_withdraw_voting"></a>

## Function `withdraw_voting`



<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_voting">withdraw_voting</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_withdraw_voting">withdraw_voting</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
                           voting_bfc: VotingBfc,
                           ctx: &<b>mut</b> TxContext) {
    <a href="bfc_dao.md#0xc8_bfc_dao_withdraw_voting">bfc_dao::withdraw_voting</a>(&<b>mut</b> system_state.dao, voting_bfc, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_system_state_inner_create_voting_bfc"></a>

## Function `create_voting_bfc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_voting_bfc">create_voting_bfc</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">bfc_system_state_inner::BfcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_create_voting_bfc">create_voting_bfc</a>(system_state: &<b>mut</b> <a href="bfc_system_state_inner.md#0xc8_bfc_system_state_inner_BfcSystemStateInner">BfcSystemStateInner</a>,
                                     <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;BFC&gt;,
                                     ctx: &<b>mut</b> TxContext) {
    <a href="bfc_dao.md#0xc8_bfc_dao_create_voting_bfc">bfc_dao::create_voting_bfc</a>(&<b>mut</b> system_state.dao, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, ctx);
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

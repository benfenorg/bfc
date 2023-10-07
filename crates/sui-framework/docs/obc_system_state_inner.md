
<a name="0xc8_obc_system_state_inner"></a>

# Module `0xc8::obc_system_state_inner`



-  [Struct `ObcSystemStateInner`](#0xc8_obc_system_state_inner_ObcSystemStateInner)
-  [Struct `TreasuryParameters`](#0xc8_obc_system_state_inner_TreasuryParameters)
-  [Struct `ObcSystemParameters`](#0xc8_obc_system_state_inner_ObcSystemParameters)
-  [Constants](#@Constants_0)
-  [Function `create_inner_state`](#0xc8_obc_system_state_inner_create_inner_state)
-  [Function `create_stake_manager_key`](#0xc8_obc_system_state_inner_create_stake_manager_key)
-  [Function `unstake_manager_key`](#0xc8_obc_system_state_inner_unstake_manager_key)
-  [Function `update_round`](#0xc8_obc_system_state_inner_update_round)
-  [Function `request_exchange_stable`](#0xc8_obc_system_state_inner_request_exchange_stable)
-  [Function `request_exchange_all`](#0xc8_obc_system_state_inner_request_exchange_all)
-  [Function `request_withdraw_stable`](#0xc8_obc_system_state_inner_request_withdraw_stable)
-  [Function `requst_get_exchange_rate`](#0xc8_obc_system_state_inner_requst_get_exchange_rate)
-  [Function `request_add_gas_coin`](#0xc8_obc_system_state_inner_request_add_gas_coin)
-  [Function `request_update_gas_coin`](#0xc8_obc_system_state_inner_request_update_gas_coin)
-  [Function `request_remove_gas_coin`](#0xc8_obc_system_state_inner_request_remove_gas_coin)
-  [Function `init_exchange_pool`](#0xc8_obc_system_state_inner_init_exchange_pool)
-  [Function `create_treasury`](#0xc8_obc_system_state_inner_create_treasury)
-  [Function `swap_obc_to_stablecoin`](#0xc8_obc_system_state_inner_swap_obc_to_stablecoin)
-  [Function `swap_obc_to_stablecoin_balance`](#0xc8_obc_system_state_inner_swap_obc_to_stablecoin_balance)
-  [Function `swap_stablecoin_to_obc`](#0xc8_obc_system_state_inner_swap_stablecoin_to_obc)
-  [Function `swap_stablecoin_to_obc_balance`](#0xc8_obc_system_state_inner_swap_stablecoin_to_obc_balance)
-  [Function `get_stablecoin_by_obc`](#0xc8_obc_system_state_inner_get_stablecoin_by_obc)
-  [Function `get_obc_by_stablecoin`](#0xc8_obc_system_state_inner_get_obc_by_stablecoin)
-  [Function `next_epoch_obc_required`](#0xc8_obc_system_state_inner_next_epoch_obc_required)
-  [Function `treasury_balance`](#0xc8_obc_system_state_inner_treasury_balance)
-  [Function `deposit_to_treasury`](#0xc8_obc_system_state_inner_deposit_to_treasury)
-  [Function `rebalance`](#0xc8_obc_system_state_inner_rebalance)
-  [Function `obc_system_stat_parameter`](#0xc8_obc_system_state_inner_obc_system_stat_parameter)
-  [Function `create_obcdao_action`](#0xc8_obc_system_state_inner_create_obcdao_action)
-  [Function `propose`](#0xc8_obc_system_state_inner_propose)
-  [Function `set_voting_delay`](#0xc8_obc_system_state_inner_set_voting_delay)
-  [Function `set_voting_period`](#0xc8_obc_system_state_inner_set_voting_period)
-  [Function `set_voting_quorum_rate`](#0xc8_obc_system_state_inner_set_voting_quorum_rate)
-  [Function `set_min_action_delay`](#0xc8_obc_system_state_inner_set_min_action_delay)
-  [Function `destroy_terminated_proposal`](#0xc8_obc_system_state_inner_destroy_terminated_proposal)
-  [Function `judge_proposal_state`](#0xc8_obc_system_state_inner_judge_proposal_state)
-  [Function `modify_proposal`](#0xc8_obc_system_state_inner_modify_proposal)
-  [Function `cast_vote`](#0xc8_obc_system_state_inner_cast_vote)
-  [Function `change_vote`](#0xc8_obc_system_state_inner_change_vote)
-  [Function `queue_proposal_action`](#0xc8_obc_system_state_inner_queue_proposal_action)
-  [Function `revoke_vote`](#0xc8_obc_system_state_inner_revoke_vote)
-  [Function `withdraw_voting`](#0xc8_obc_system_state_inner_withdraw_voting)
-  [Function `create_voting_obc`](#0xc8_obc_system_state_inner_create_voting_obc)
-  [Module Specification](#@Module_Specification_1)


<pre><code><b>use</b> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">0x2::obc</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="exchange_inner.md#0xc8_exchange_inner">0xc8::exchange_inner</a>;
<b>use</b> <a href="gas_coin_map.md#0xc8_gas_coin_map">0xc8::gas_coin_map</a>;
<b>use</b> <a href="obc_dao.md#0xc8_obc_dao">0xc8::obc_dao</a>;
<b>use</b> <a href="obc_dao_manager.md#0xc8_obc_dao_manager">0xc8::obc_dao_manager</a>;
<b>use</b> <a href="treasury.md#0xc8_treasury">0xc8::treasury</a>;
<b>use</b> <a href="treasury_pool.md#0xc8_treasury_pool">0xc8::treasury_pool</a>;
<b>use</b> <a href="usd.md#0xc8_usd">0xc8::usd</a>;
<b>use</b> <a href="obc_dao_voting_pool.md#0xc8_voting_pool">0xc8::voting_pool</a>;
</code></pre>



<a name="0xc8_obc_system_state_inner_ObcSystemStateInner"></a>

## Struct `ObcSystemStateInner`



<pre><code><b>struct</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a> <b>has</b> store
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
<code>exchange_pool: <a href="exchange_inner.md#0xc8_exchange_inner_ExchangePool">exchange_inner::ExchangePool</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;</code>
</dt>
<dd>
 Exchange gas coin pool
</dd>
<dt>
<code>dao: <a href="obc_dao.md#0xc8_obc_dao_Dao">obc_dao::Dao</a></code>
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

<a name="0xc8_obc_system_state_inner_TreasuryParameters"></a>

## Struct `TreasuryParameters`



<pre><code><b>struct</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_TreasuryParameters">TreasuryParameters</a> <b>has</b> <b>copy</b>, drop
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

<a name="0xc8_obc_system_state_inner_ObcSystemParameters"></a>

## Struct `ObcSystemParameters`



<pre><code><b>struct</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a> <b>has</b> <b>copy</b>, drop
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
<code>treasury_parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_TreasuryParameters">obc_system_state_inner::TreasuryParameters</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_obc_system_state_inner_DEFAULT_ADMIN_ADDRESSES"></a>



<pre><code><b>const</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>: <a href="">vector</a>&lt;<b>address</b>&gt; = [15835301299490436797531864052756717558919000202302735629799491391021929516032, 65291099566713687366712577645016528323844253956509590706950403481947121946472, 24534971471998884076320588073588140533011227823836498849038009586284449996719, 51146047687078908496806713158095522211891725112102928692628967303014320690576, 99840256252410854934884720094907096870421318439837223403968912893243261564088, 34823872716959626420800761234109339257697856766564337894110377572280284587387];
</code></pre>



<a name="0xc8_obc_system_state_inner_OBC_SYSTEM_STATE_START_ROUND"></a>



<pre><code><b>const</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_OBC_SYSTEM_STATE_START_ROUND">OBC_SYSTEM_STATE_START_ROUND</a>: u64 = 0;
</code></pre>



<a name="0xc8_obc_system_state_inner_OBC_SYSTEM_TREASURY_KEY"></a>



<pre><code><b>const</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_OBC_SYSTEM_TREASURY_KEY">OBC_SYSTEM_TREASURY_KEY</a>: u64 = 1;
</code></pre>



<a name="0xc8_obc_system_state_inner_create_inner_state"></a>

## Function `create_inner_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">create_inner_state</a>(usd_supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, obc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_inner_state">create_inner_state</a>(
    usd_supply: Supply&lt;USD&gt;,
    obc_balance: Balance&lt;OBC&gt;,
    parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext,
): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a> {
    // init gas <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a> mappings
    <b>let</b> init_gas_coins_map = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>&lt;<b>address</b>, GasCoinEntity&gt;();
    <b>let</b> <a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a> = <a href="gas_coin_map.md#0xc8_gas_coin_map_new">gas_coin_map::new</a>(init_gas_coins_map, ctx);
    <b>let</b> exchange_pool = <a href="exchange_inner.md#0xc8_exchange_inner_new_exchange_pool">exchange_inner::new_exchange_pool</a>&lt;USD&gt;(ctx, 0);
    <b>let</b> dao = <a href="obc_dao.md#0xc8_obc_dao_create_dao">obc_dao::create_dao</a>(<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_DEFAULT_ADMIN_ADDRESSES">DEFAULT_ADMIN_ADDRESSES</a>, ctx);
    <b>let</b> (t, remain_balance) = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_treasury">create_treasury</a>(usd_supply, obc_balance, parameters, ctx);
    <b>let</b> tp = <a href="treasury_pool.md#0xc8_treasury_pool_create_treasury_pool">treasury_pool::create_treasury_pool</a>(remain_balance, ctx);

    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a> {
        round: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_OBC_SYSTEM_STATE_START_ROUND">OBC_SYSTEM_STATE_START_ROUND</a>,
        <a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>,
        exchange_pool,
        dao,
        <a href="treasury.md#0xc8_treasury">treasury</a>: t,
        <a href="treasury_pool.md#0xc8_treasury_pool">treasury_pool</a>: tp,
    }
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_create_stake_manager_key"></a>

## Function `create_stake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_stake_manager_key">create_stake_manager_key</a>(payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> (<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_stake_manager_key">create_stake_manager_key</a>( payment: Coin&lt;OBC&gt;,
                                              ctx: &<b>mut</b> TxContext) {
    <a href="obc_dao.md#0xc8_obc_dao_create_stake_manager_key">obc_dao::create_stake_manager_key</a>(payment, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_unstake_manager_key"></a>

## Function `unstake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_unstake_manager_key">unstake_manager_key</a>(key: <a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, token: <a href="obc_dao_manager.md#0xc8_obc_dao_manager_ManagerKeyObc">obc_dao_manager::ManagerKeyObc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_unstake_manager_key">unstake_manager_key</a>(key: OBCDaoManageKey,
                                       token: ManagerKeyObc,
                                       ctx: &<b>mut</b> TxContext) {
    <a href="obc_dao.md#0xc8_obc_dao_unstake_manager_key">obc_dao::unstake_manager_key</a>(key, token, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_update_round"></a>

## Function `update_round`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_update_round">update_round</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, round: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_update_round">update_round</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    round: u64,
) {
    inner.round = round;
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_exchange_stable"></a>

## Function `request_exchange_stable`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_stable">request_exchange_stable</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_stable">request_exchange_stable</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>: Coin&lt;USD&gt;,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;OBC&gt; {
    //get exchange rate
    <b>let</b> rate = <a href="gas_coin_map.md#0xc8_gas_coin_map_requst_get_exchange_rate">gas_coin_map::requst_get_exchange_rate</a>&lt;USD&gt;(&inner.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, &<a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>);
    <a href="exchange_inner.md#0xc8_exchange_inner_request_exchange_stable">exchange_inner::request_exchange_stable</a>&lt;USD&gt;(rate, &<b>mut</b> inner.exchange_pool, <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a>, ctx)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_exchange_all"></a>

## Function `request_exchange_all`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_all">request_exchange_all</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_exchange_all">request_exchange_all</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    ctx: &<b>mut</b> TxContext
) {
    //get <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">obc</a> amount of inner exchange pool
    <b>let</b> obc_amount = <a href="exchange_inner.md#0xc8_exchange_inner_get_obc_amount">exchange_inner::get_obc_amount</a>(&inner.exchange_pool);
    <b>if</b> (obc_amount &gt; 0) {
        //set pool is disactivate
        <b>let</b> epoch = <a href="exchange_inner.md#0xc8_exchange_inner_dis_activate">exchange_inner::dis_activate</a>(&<b>mut</b> inner.exchange_pool);
        //get <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>
        <b>let</b> stable_balance = <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_all_stable">exchange_inner::request_withdraw_all_stable</a>(&<b>mut</b> inner.exchange_pool);
        //exchange from <a href="../../../.././build/Sui/docs/stable.md#0x2_stable">stable</a> swap
        <b>let</b> obc_balance = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_stablecoin_to_obc_balance">swap_stablecoin_to_obc_balance</a>(
            inner,
            <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(stable_balance, ctx),
            ctx,
        );
        //add <a href="../../../.././build/Sui/docs/obc.md#0x2_obc">obc</a> <b>to</b> inner exchange pool
        <a href="exchange_inner.md#0xc8_exchange_inner_request_deposit_obc_balance">exchange_inner::request_deposit_obc_balance</a>(&<b>mut</b> inner.exchange_pool, obc_balance);
        // active pool
        <a href="exchange_inner.md#0xc8_exchange_inner_activate">exchange_inner::activate</a>(&<b>mut</b> inner.exchange_pool, epoch);
    }
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_withdraw_stable"></a>

## Function `request_withdraw_stable`

Request withdraw stable coin.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_withdraw_stable">request_withdraw_stable</a>(inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_withdraw_stable">request_withdraw_stable</a>(
    inner: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
): Balance&lt;USD&gt; {
    <a href="exchange_inner.md#0xc8_exchange_inner_request_withdraw_all_stable">exchange_inner::request_withdraw_all_stable</a>(&<b>mut</b> inner.exchange_pool)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_requst_get_exchange_rate"></a>

## Function `requst_get_exchange_rate`

Getter of the gas coin exchange pool rate.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, _stable: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_requst_get_exchange_rate">requst_get_exchange_rate</a>&lt;CoinType&gt;(
    self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    _stable: &Coin&lt;CoinType&gt;
): u64 {
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_stablecoin_by_obc">get_stablecoin_by_obc</a>&lt;CoinType&gt;(
        self,
        <a href="gas_coin_map.md#0xc8_gas_coin_map_get_default_rate">gas_coin_map::get_default_rate</a>(),
    )
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_add_gas_coin"></a>

## Function `request_add_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;, rate: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_add_gas_coin">request_add_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
    rate: u64,
) {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_add_gas_coin">gas_coin_map::request_add_gas_coin</a>&lt;CoinType&gt;(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_update_gas_coin"></a>

## Function `request_update_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_update_gas_coin">request_update_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
) {
    <b>let</b> rate = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_stablecoin_by_obc">get_stablecoin_by_obc</a>&lt;CoinType&gt;(
        self,
        <a href="gas_coin_map.md#0xc8_gas_coin_map_get_default_rate">gas_coin_map::get_default_rate</a>(),
    );
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_update_gas_coin">gas_coin_map::request_update_gas_coin</a>(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin, rate)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_request_remove_gas_coin"></a>

## Function `request_remove_gas_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, gas_coin: &<a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;CoinType&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_request_remove_gas_coin">request_remove_gas_coin</a>&lt;CoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    gas_coin: &Coin&lt;CoinType&gt;,
) {
    <a href="gas_coin_map.md#0xc8_gas_coin_map_request_remove_gas_coin">gas_coin_map::request_remove_gas_coin</a>&lt;CoinType&gt;(&<b>mut</b> self.<a href="gas_coin_map.md#0xc8_gas_coin_map">gas_coin_map</a>, gas_coin)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_init_exchange_pool"></a>

## Function `init_exchange_pool`

Init exchange pool by add obc coin.


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_init_exchange_pool">init_exchange_pool</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_init_exchange_pool">init_exchange_pool</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;OBC&gt;,
) {
    <a href="exchange_inner.md#0xc8_exchange_inner_add_obc_to_pool">exchange_inner::add_obc_to_pool</a>(&<b>mut</b> self.exchange_pool, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_create_treasury"></a>

## Function `create_treasury`

X treasury  init treasury


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_treasury">create_treasury</a>(supply: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Supply">balance::Supply</a>&lt;<a href="usd.md#0xc8_usd_USD">usd::USD</a>&gt;, obc_balance: <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="treasury.md#0xc8_treasury_Treasury">treasury::Treasury</a>, <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_treasury">create_treasury</a>(
    supply: Supply&lt;USD&gt;,
    obc_balance: Balance&lt;OBC&gt;,
    parameters: <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a>,
    ctx: &<b>mut</b> TxContext
): (Treasury, Balance&lt;OBC&gt;) {
    <b>let</b> treasury_parameters = parameters.treasury_parameters;
    <b>let</b> t = <a href="treasury.md#0xc8_treasury_create_treasury">treasury::create_treasury</a>(treasury_parameters.time_interval, ctx);

    <a href="treasury.md#0xc8_treasury_init_vault_with_positions">treasury::init_vault_with_positions</a>&lt;USD&gt;(
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
    <b>if</b> (<a href="../../../.././build/Sui/docs/balance.md#0x2_balance_value">balance::value</a>&lt;OBC&gt;(&obc_balance) &gt; 0) {
        <b>let</b> deposit_balance = <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_split">balance::split</a>(&<b>mut</b> obc_balance, <a href="treasury.md#0xc8_treasury_next_epoch_obc_required">treasury::next_epoch_obc_required</a>(&t));
        <a href="treasury.md#0xc8_treasury_deposit">treasury::deposit</a>(&<b>mut</b> t, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_from_balance">coin::from_balance</a>(deposit_balance, ctx));
    };
    (t, obc_balance)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_swap_obc_to_stablecoin"></a>

## Function `swap_obc_to_stablecoin`

swap obc to stablecoin


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_obc_to_stablecoin">swap_obc_to_stablecoin</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_obc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_obc_to_stablecoin">swap_obc_to_stablecoin</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    coin_obc: Coin&lt;OBC&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="treasury.md#0xc8_treasury_mint">treasury::mint</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_obc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_swap_obc_to_stablecoin_balance"></a>

## Function `swap_obc_to_stablecoin_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_obc_to_stablecoin_balance">swap_obc_to_stablecoin_balance</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_obc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;StableCoinType&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_obc_to_stablecoin_balance">swap_obc_to_stablecoin_balance</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    coin_obc: Coin&lt;OBC&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;StableCoinType&gt; {
    <a href="treasury.md#0xc8_treasury_mint_internal">treasury::mint_internal</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_obc, amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_swap_stablecoin_to_obc"></a>

## Function `swap_stablecoin_to_obc`

swap stablecoin to obc


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_stablecoin_to_obc">swap_stablecoin_to_obc</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, amount: u64, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_stablecoin_to_obc">swap_stablecoin_to_obc</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="treasury.md#0xc8_treasury_redeem">treasury::redeem</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_sc, amount, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_swap_stablecoin_to_obc_balance"></a>

## Function `swap_stablecoin_to_obc_balance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_stablecoin_to_obc_balance">swap_stablecoin_to_obc_balance</a>&lt;StableCoinType&gt;(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_sc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;StableCoinType&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../../.././build/Sui/docs/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_swap_stablecoin_to_obc_balance">swap_stablecoin_to_obc_balance</a>&lt;StableCoinType&gt;(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    coin_sc: Coin&lt;StableCoinType&gt;,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;OBC&gt; {
    <b>let</b> amount = <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_value">coin::value</a>(&coin_sc);
    <a href="treasury.md#0xc8_treasury_redeem_internal">treasury::redeem_internal</a>&lt;StableCoinType&gt;(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_sc, amount, ctx)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_get_stablecoin_by_obc"></a>

## Function `get_stablecoin_by_obc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_stablecoin_by_obc">get_stablecoin_by_obc</a>&lt;StableCoinType&gt;(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_stablecoin_by_obc">get_stablecoin_by_obc</a>&lt;StableCoinType&gt;(
    self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    amount: u64
): u64
{
    <a href="treasury.md#0xc8_treasury_calculate_swap_result">treasury::calculate_swap_result</a>&lt;StableCoinType&gt;(&self.<a href="treasury.md#0xc8_treasury">treasury</a>, <b>false</b>, amount)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_get_obc_by_stablecoin"></a>

## Function `get_obc_by_stablecoin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_obc_by_stablecoin">get_obc_by_stablecoin</a>&lt;StableCoinType&gt;(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_get_obc_by_stablecoin">get_obc_by_stablecoin</a>&lt;StableCoinType&gt;(
    self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    amount: u64
): u64
{
    <a href="treasury.md#0xc8_treasury_calculate_swap_result">treasury::calculate_swap_result</a>&lt;StableCoinType&gt;(&self.<a href="treasury.md#0xc8_treasury">treasury</a>, <b>true</b>, amount)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_next_epoch_obc_required"></a>

## Function `next_epoch_obc_required`

X-treasury


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_next_epoch_obc_required">next_epoch_obc_required</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_next_epoch_obc_required">next_epoch_obc_required</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>): u64 {
    <a href="treasury.md#0xc8_treasury_next_epoch_obc_required">treasury::next_epoch_obc_required</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_treasury_balance"></a>

## Function `treasury_balance`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_treasury_balance">treasury_balance</a>(self: &<a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>): u64 {
    <a href="treasury.md#0xc8_treasury_get_balance">treasury::get_balance</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>)
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_deposit_to_treasury"></a>

## Function `deposit_to_treasury`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, coin_obc: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_deposit_to_treasury">deposit_to_treasury</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>, coin_obc: Coin&lt;OBC&gt;) {
    <a href="treasury.md#0xc8_treasury_deposit">treasury::deposit</a>(&<b>mut</b> self.<a href="treasury.md#0xc8_treasury">treasury</a>, coin_obc);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_rebalance"></a>

## Function `rebalance`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_rebalance">rebalance</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_rebalance">rebalance</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> amount = <a href="treasury.md#0xc8_treasury_next_epoch_obc_required">treasury::next_epoch_obc_required</a>(&self.<a href="treasury.md#0xc8_treasury">treasury</a>);
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

<a name="0xc8_obc_system_state_inner_obc_system_stat_parameter"></a>

## Function `obc_system_stat_parameter`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_obc_system_stat_parameter">obc_system_stat_parameter</a>(position_number: u32, tick_spacing: u32, spacing_times: u32, initialize_price: u128, time_interval: u32, base_point: u64, max_counter_times: u32, chain_start_timestamp_ms: u64): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">obc_system_state_inner::ObcSystemParameters</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_obc_system_stat_parameter">obc_system_stat_parameter</a>(
    position_number: u32,
    tick_spacing: u32,
    spacing_times: u32,
    initialize_price: u128,
    time_interval: u32,
    base_point: u64,
    max_counter_times: u32,
    chain_start_timestamp_ms: u64,
): <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a> {
    <b>let</b> treasury_parameters = <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_TreasuryParameters">TreasuryParameters</a> {
        position_number,
        tick_spacing,
        spacing_times,
        initialize_price,
        time_interval,
        max_counter_times,
        base_point,
    };
    <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemParameters">ObcSystemParameters</a> {
        treasury_parameters,
        chain_start_timestamp_ms,
    }
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_create_obcdao_action"></a>

## Function `create_obcdao_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_obcdao_action">create_obcdao_action</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, actionName: <a href="">vector</a>&lt;u8&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_obcdao_action">create_obcdao_action</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    payment: Coin&lt;OBC&gt;,

    actionName: <a href="">vector</a>&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext) {
    <a href="obc_dao.md#0xc8_obc_dao_create_obcdao_action">obc_dao::create_obcdao_action</a>(&<b>mut</b> self.dao, payment, actionName, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_propose"></a>

## Function `propose`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_propose">propose</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, version_id: u64, payment: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, action_id: u64, action_delay: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_propose">propose</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    version_id: u64,
    payment: Coin&lt;OBC&gt;,
    action_id: u64,
    action_delay: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    obc_dao:: propose(&<b>mut</b> self.dao, version_id, payment, action_id, action_delay, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_set_voting_delay"></a>

## Function `set_voting_delay`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_delay">set_voting_delay</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>, manager_key: &OBCDaoManageKey, value: u64) {
    <a href="obc_dao.md#0xc8_obc_dao_set_voting_delay">obc_dao::set_voting_delay</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_set_voting_period"></a>

## Function `set_voting_period`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_period">set_voting_period</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_period">set_voting_period</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    manager_key: &OBCDaoManageKey,
    value: u64,
) {
    <a href="obc_dao.md#0xc8_obc_dao_set_voting_period">obc_dao::set_voting_period</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_set_voting_quorum_rate"></a>

## Function `set_voting_quorum_rate`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_quorum_rate">set_voting_quorum_rate</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_voting_quorum_rate">set_voting_quorum_rate</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    manager_key: &OBCDaoManageKey,
    value: u8,
) {
    <a href="obc_dao.md#0xc8_obc_dao_set_voting_quorum_rate">obc_dao::set_voting_quorum_rate</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_set_min_action_delay"></a>

## Function `set_min_action_delay`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_min_action_delay">set_min_action_delay</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_set_min_action_delay">set_min_action_delay</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    manager_key: &OBCDaoManageKey,
    value: u64,
) {
    <a href="obc_dao.md#0xc8_obc_dao_set_min_action_delay">obc_dao::set_min_action_delay</a>(&<b>mut</b> self.dao, manager_key, value);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    self: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    manager_key: &OBCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock
) {
    <a href="obc_dao.md#0xc8_obc_dao_destroy_terminated_proposal">obc_dao::destroy_terminated_proposal</a>(&<b>mut</b> self.dao, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, current_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_judge_proposal_state">judge_proposal_state</a>(wrapper: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>, current_time: u64) {
    <b>let</b> proposal_record = <a href="obc_dao.md#0xc8_obc_dao_getProposalRecord">obc_dao::getProposalRecord</a>(&<b>mut</b> wrapper.dao);
    <b>let</b> size: u64 = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&proposal_record);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; size) {
        <b>let</b> (_, proposalInfo) = <a href="../../../.././build/Sui/docs/vec_map.md#0x2_vec_map_get_entry_by_idx">vec_map::get_entry_by_idx</a>(&proposal_record, size - 1);
        <b>let</b> cur_status = <a href="obc_dao.md#0xc8_obc_dao_judge_proposal_state">obc_dao::judge_proposal_state</a>(proposalInfo, current_time);
        <a href="obc_dao.md#0xc8_obc_dao_set_current_status_into_dao">obc_dao::set_current_status_into_dao</a>(&<b>mut</b> wrapper.dao, proposalInfo, cur_status);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_modify_proposal"></a>

## Function `modify_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_modify_proposal">modify_proposal</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, proposal_obj: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, index: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_modify_proposal">modify_proposal</a>(
    system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    proposal_obj: &<b>mut</b> Proposal,
    index: u8,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &Clock
) {
    <a href="obc_dao.md#0xc8_obc_dao_modify_proposal_obj">obc_dao::modify_proposal_obj</a>(&<b>mut</b> system_state.dao, proposal_obj, index, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_cast_vote"></a>

## Function `cast_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_cast_vote">cast_vote</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, agreeInt: u8, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_cast_vote">cast_vote</a>(
    system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: VotingObc,
    agreeInt: u8,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_dao.md#0xc8_obc_dao_cast_vote">obc_dao::cast_vote</a>(&<b>mut</b> system_state.dao, proposal, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, agreeInt, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_change_vote"></a>

## Function `change_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_change_vote">change_vote</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, my_vote: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Vote">obc_dao::Vote</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, agree: bool, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_change_vote">change_vote</a>(
    system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    my_vote: &<b>mut</b> Vote,
    proposal: &<b>mut</b> Proposal,
    agree: bool,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_dao.md#0xc8_obc_dao_change_vote">obc_dao::change_vote</a>(&<b>mut</b> system_state.dao, my_vote, proposal, agree, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_queue_proposal_action"></a>

## Function `queue_proposal_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_queue_proposal_action">queue_proposal_action</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, manager_key: &<a href="obc_dao_manager.md#0xc8_obc_dao_manager_OBCDaoManageKey">obc_dao_manager::OBCDaoManageKey</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_queue_proposal_action">queue_proposal_action</a>(
    system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    manager_key: &OBCDaoManageKey,
    proposal: &<b>mut</b> Proposal,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
) {
    <a href="obc_dao.md#0xc8_obc_dao_queue_proposal_action">obc_dao::queue_proposal_action</a>(&<b>mut</b> system_state.dao, manager_key, proposal, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_revoke_vote"></a>

## Function `revoke_vote`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_revoke_vote">revoke_vote</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, proposal: &<b>mut</b> <a href="obc_dao.md#0xc8_obc_dao_Proposal">obc_dao::Proposal</a>, my_vote: <a href="obc_dao.md#0xc8_obc_dao_Vote">obc_dao::Vote</a>, voting_power: u64, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: &<a href="../../../.././build/Sui/docs/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_revoke_vote">revoke_vote</a>(
    system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
    proposal: &<b>mut</b> Proposal,
    my_vote: Vote,
    voting_power: u64,
    <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="obc_dao.md#0xc8_obc_dao_revoke_vote">obc_dao::revoke_vote</a>(&<b>mut</b> system_state.dao, proposal, my_vote, voting_power, <a href="../../../.././build/Sui/docs/clock.md#0x2_clock">clock</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_withdraw_voting"></a>

## Function `withdraw_voting`



<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_withdraw_voting">withdraw_voting</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, voting_obc: <a href="obc_dao_voting_pool.md#0xc8_voting_pool_VotingObc">voting_pool::VotingObc</a>, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_withdraw_voting">withdraw_voting</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
                           voting_obc: VotingObc,
                           ctx: &<b>mut</b> TxContext) {
    <a href="obc_dao.md#0xc8_obc_dao_withdraw_voting">obc_dao::withdraw_voting</a>(&<b>mut</b> system_state.dao, voting_obc, ctx);
}
</code></pre>



</details>

<a name="0xc8_obc_system_state_inner_create_voting_obc"></a>

## Function `create_voting_obc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_voting_obc">create_voting_obc</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">obc_system_state_inner::ObcSystemStateInner</a>, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: <a href="../../../.././build/Sui/docs/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../../../.././build/Sui/docs/obc.md#0x2_obc_OBC">obc::OBC</a>&gt;, ctx: &<b>mut</b> <a href="../../../.././build/Sui/docs/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_create_voting_obc">create_voting_obc</a>(system_state: &<b>mut</b> <a href="obc_system_state_inner.md#0xc8_obc_system_state_inner_ObcSystemStateInner">ObcSystemStateInner</a>,
                                     <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>: Coin&lt;OBC&gt;,
                                     ctx: &<b>mut</b> TxContext) {
    <a href="obc_dao.md#0xc8_obc_dao_create_voting_obc">obc_dao::create_voting_obc</a>(&<b>mut</b> system_state.dao, <a href="../../../.././build/Sui/docs/coin.md#0x2_coin">coin</a>, ctx);
}
</code></pre>



</details>

<a name="@Module_Specification_1"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

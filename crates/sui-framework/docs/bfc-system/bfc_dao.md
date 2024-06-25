---
title: Module `0xc8::bfc_dao`
---



-  [Struct `DaoEvent`](#0xc8_bfc_dao_DaoEvent)
-  [Struct `DaoManagerEvent`](#0xc8_bfc_dao_DaoManagerEvent)
-  [Struct `ProposalCreatedEvent`](#0xc8_bfc_dao_ProposalCreatedEvent)
-  [Struct `VoteChangedEvent`](#0xc8_bfc_dao_VoteChangedEvent)
-  [Struct `ActionCreateEvent`](#0xc8_bfc_dao_ActionCreateEvent)
-  [Struct `ProposalStateEvent`](#0xc8_bfc_dao_ProposalStateEvent)
-  [Struct `BooleanEvent`](#0xc8_bfc_dao_BooleanEvent)
-  [Struct `DaoGlobalInfo`](#0xc8_bfc_dao_DaoGlobalInfo)
-  [Struct `DaoConfig`](#0xc8_bfc_dao_DaoConfig)
-  [Resource `Dao`](#0xc8_bfc_dao_Dao)
-  [Struct `ProposalStatus`](#0xc8_bfc_dao_ProposalStatus)
-  [Struct `BFCDaoAction`](#0xc8_bfc_dao_BFCDaoAction)
-  [Struct `ProposalInfo`](#0xc8_bfc_dao_ProposalInfo)
-  [Resource `Proposal`](#0xc8_bfc_dao_Proposal)
-  [Resource `Vote`](#0xc8_bfc_dao_Vote)
-  [Struct `VoteInfoEvent`](#0xc8_bfc_dao_VoteInfoEvent)
-  [Struct `ProposalInfoEvent`](#0xc8_bfc_dao_ProposalInfoEvent)
-  [Constants](#@Constants_0)
-  [Function `getProposalRecord`](#0xc8_bfc_dao_getProposalRecord)
-  [Function `get_bfcdao_actionid`](#0xc8_bfc_dao_get_bfcdao_actionid)
-  [Function `create_bfcdao_action`](#0xc8_bfc_dao_create_bfcdao_action)
-  [Function `remove_action`](#0xc8_bfc_dao_remove_action)
-  [Function `remove_proposal`](#0xc8_bfc_dao_remove_proposal)
-  [Function `create_dao`](#0xc8_bfc_dao_create_dao)
-  [Function `getDaoActionByActionId`](#0xc8_bfc_dao_getDaoActionByActionId)
-  [Function `new_dao_config`](#0xc8_bfc_dao_new_dao_config)
-  [Function `propose`](#0xc8_bfc_dao_propose)
-  [Function `synchronize_proposal_into_dao`](#0xc8_bfc_dao_synchronize_proposal_into_dao)
-  [Function `cast_vote`](#0xc8_bfc_dao_cast_vote)
-  [Function `change_vote`](#0xc8_bfc_dao_change_vote)
-  [Function `do_flip_vote`](#0xc8_bfc_dao_do_flip_vote)
-  [Function `revoke_vote`](#0xc8_bfc_dao_revoke_vote)
-  [Function `do_revoke_vote`](#0xc8_bfc_dao_do_revoke_vote)
-  [Function `unvote_votes`](#0xc8_bfc_dao_unvote_votes)
-  [Function `vote_of`](#0xc8_bfc_dao_vote_of)
-  [Function `has_vote`](#0xc8_bfc_dao_has_vote)
-  [Function `queue_proposal_action`](#0xc8_bfc_dao_queue_proposal_action)
-  [Function `extract_proposal_action`](#0xc8_bfc_dao_extract_proposal_action)
-  [Function `proposal_exists`](#0xc8_bfc_dao_proposal_exists)
-  [Function `proposal_state`](#0xc8_bfc_dao_proposal_state)
-  [Function `judge_proposal_state`](#0xc8_bfc_dao_judge_proposal_state)
-  [Function `proposal_info`](#0xc8_bfc_dao_proposal_info)
-  [Function `generate_next_proposal_id`](#0xc8_bfc_dao_generate_next_proposal_id)
-  [Function `generate_next_action_id`](#0xc8_bfc_dao_generate_next_action_id)
-  [Function `quorum_votes`](#0xc8_bfc_dao_quorum_votes)
-  [Function `voting_delay`](#0xc8_bfc_dao_voting_delay)
-  [Function `voting_period`](#0xc8_bfc_dao_voting_period)
-  [Function `voting_quorum_rate`](#0xc8_bfc_dao_voting_quorum_rate)
-  [Function `min_action_delay`](#0xc8_bfc_dao_min_action_delay)
-  [Function `get_config`](#0xc8_bfc_dao_get_config)
-  [Function `modify_dao_config`](#0xc8_bfc_dao_modify_dao_config)
-  [Function `set_voting_delay`](#0xc8_bfc_dao_set_voting_delay)
-  [Function `set_voting_period`](#0xc8_bfc_dao_set_voting_period)
-  [Function `set_voting_quorum_rate`](#0xc8_bfc_dao_set_voting_quorum_rate)
-  [Function `set_min_action_delay`](#0xc8_bfc_dao_set_min_action_delay)
-  [Function `set_admins`](#0xc8_bfc_dao_set_admins)
-  [Function `create_stake_manager_key`](#0xc8_bfc_dao_create_stake_manager_key)
-  [Function `unstake_manager_key`](#0xc8_bfc_dao_unstake_manager_key)
-  [Function `modify_proposal_obj`](#0xc8_bfc_dao_modify_proposal_obj)
-  [Function `create_voting_bfc`](#0xc8_bfc_dao_create_voting_bfc)
-  [Function `withdraw_voting`](#0xc8_bfc_dao_withdraw_voting)
-  [Function `destroy_terminated_proposal`](#0xc8_bfc_dao_destroy_terminated_proposal)
-  [Function `set_current_status_into_dao`](#0xc8_bfc_dao_set_current_status_into_dao)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../sui-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="../sui-framework/bfc.md#0x2_bfc">0x2::bfc</a>;
<b>use</b> <a href="../sui-framework/clock.md#0x2_clock">0x2::clock</a>;
<b>use</b> <a href="../sui-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="../sui-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../sui-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager">0xc8::bfc_dao_manager</a>;
<b>use</b> <a href="bfc_dao_voting_pool.md#0xc8_voting_pool">0xc8::voting_pool</a>;
</code></pre>



<a name="0xc8_bfc_dao_DaoEvent"></a>

## Struct `DaoEvent`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_DaoEvent">DaoEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>name: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_DaoManagerEvent"></a>

## Struct `DaoManagerEvent`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_DaoManagerEvent">DaoManagerEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>msg: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>key: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_ProposalCreatedEvent"></a>

## Struct `ProposalCreatedEvent`

emitted when proposal created.


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_ProposalCreatedEvent">ProposalCreatedEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>
 the proposal id.
</dd>
<dt>
<code>proposer: <b>address</b></code>
</dt>
<dd>
 proposer is the user who create the proposal.
</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_VoteChangedEvent"></a>

## Struct `VoteChangedEvent`

emitted when user vote/revoke_vote.


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_VoteChangedEvent">VoteChangedEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>
 the proposal id.
</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>
 the voter.
</dd>
<dt>
<code>proposer: <b>address</b></code>
</dt>
<dd>
 creator of the proposal.
</dd>
<dt>
<code>agree: bool</code>
</dt>
<dd>
 agree with the proposal or not
</dd>
<dt>
<code>vote: u64</code>
</dt>
<dd>
 latest vote count of the voter.
</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_ActionCreateEvent"></a>

## Struct `ActionCreateEvent`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_ActionCreateEvent">ActionCreateEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>actionId: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>name: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 Name for the action
</dd>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_ProposalStateEvent"></a>

## Struct `ProposalStateEvent`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_ProposalStateEvent">ProposalStateEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposalId: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>state: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_BooleanEvent"></a>

## Struct `BooleanEvent`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_BooleanEvent">BooleanEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>value: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_DaoGlobalInfo"></a>

## Struct `DaoGlobalInfo`

global DAO info of the specified token type <code>Token</code>.


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_DaoGlobalInfo">DaoGlobalInfo</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>next_proposal_id: u64</code>
</dt>
<dd>
 next proposal id.
</dd>
<dt>
<code>next_action_id: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_DaoConfig"></a>

## Struct `DaoConfig`

Configuration of the <code>Token</code>'s DAO.


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_DaoConfig">DaoConfig</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>voting_delay: u64</code>
</dt>
<dd>
 after proposal created, how long use should wait before he can vote (in milliseconds)
</dd>
<dt>
<code>voting_period: u64</code>
</dt>
<dd>
 how long the voting window is (in milliseconds).
</dd>
<dt>
<code>voting_quorum_rate: u8</code>
</dt>
<dd>
 the quorum rate to agree on the proposal.
 if 50% votes needed, then the voting_quorum_rate should be 50.
 it should between (0, 100].
</dd>
<dt>
<code>min_action_delay: u64</code>
</dt>
<dd>
 how long the proposal should wait before it can be executed (in milliseconds).
</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_Dao"></a>

## Resource `Dao`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>config: <a href="bfc_dao.md#0xc8_bfc_dao_DaoConfig">bfc_dao::DaoConfig</a></code>
</dt>
<dd>

</dd>
<dt>
<code>info: <a href="bfc_dao.md#0xc8_bfc_dao_DaoGlobalInfo">bfc_dao::DaoGlobalInfo</a></code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_record: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">bfc_dao::ProposalInfo</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>action_record: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">bfc_dao::BFCDaoAction</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>votes_record: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="bfc_dao_voting_pool.md#0xc8_voting_pool">voting_pool</a>: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingPool">voting_pool::VotingPool</a></code>
</dt>
<dd>

</dd>
<dt>
<code>current_proposal_status: <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, <a href="bfc_dao.md#0xc8_bfc_dao_ProposalStatus">bfc_dao::ProposalStatus</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_ProposalStatus"></a>

## Struct `ProposalStatus`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_ProposalStatus">ProposalStatus</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>version_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_BFCDaoAction"></a>

## Struct `BFCDaoAction`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>action_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>name: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 Name for the action
</dd>
<dt>
<code>status: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_ProposalInfo"></a>

## Struct `ProposalInfo`



<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_uid: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>pid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proposer: <b>address</b></code>
</dt>
<dd>
 creator of the proposal
</dd>
<dt>
<code>start_time: u64</code>
</dt>
<dd>
 when voting begins.
</dd>
<dt>
<code>end_time: u64</code>
</dt>
<dd>
 when voting ends.
</dd>
<dt>
<code>for_votes: u64</code>
</dt>
<dd>
 count of voters who agree with the proposal
</dd>
<dt>
<code>against_votes: u64</code>
</dt>
<dd>
 count of voters who're against the proposal
</dd>
<dt>
<code>eta: u64</code>
</dt>
<dd>
 executable after this time.
</dd>
<dt>
<code>action_delay: u64</code>
</dt>
<dd>
 after how long, the agreed proposal can be executed.
</dd>
<dt>
<code>quorum_votes: u64</code>
</dt>
<dd>
 how many votes to reach to make the proposal pass.
</dd>
<dt>
<code>action: <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">bfc_dao::BFCDaoAction</a></code>
</dt>
<dd>
 proposal action.
</dd>
<dt>
<code>version_id: u64</code>
</dt>
<dd>
 version id.
</dd>
<dt>
<code>description: <a href="../move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 description
</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_Proposal"></a>

## Resource `Proposal`

Proposal data struct.


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 id of the proposal
</dd>
<dt>
<code>proposal: <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">bfc_dao::ProposalInfo</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_Vote"></a>

## Resource `Vote`

User vote info.


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>vid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proposer: <b>address</b></code>
</dt>
<dd>
 vote for the proposal under the <code>proposer</code>.
</dd>
<dt>
<code>vote: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a></code>
</dt>
<dd>
 how many tokens to vote.
</dd>
<dt>
<code>agree: bool</code>
</dt>
<dd>
 vote for or vote against.
</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_VoteInfoEvent"></a>

## Struct `VoteInfoEvent`

Get voter's vote info on proposal with <code>proposal_id</code> of <code>proposer_address</code>.


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_VoteInfoEvent">VoteInfoEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>proposer: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>agree: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>vote: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_bfc_dao_ProposalInfoEvent"></a>

## Struct `ProposalInfoEvent`

get proposal's information.
return: (id, start_time, end_time, for_votes, against_votes).


<pre><code><b>struct</b> <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfoEvent">ProposalInfoEvent</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>start_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>end_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>for_votes: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>against_votes: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_bfc_dao_ACTIVE"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE">ACTIVE</a>: u8 = 2;
</code></pre>



<a name="0xc8_bfc_dao_ACTIVE_MAX_NUM_THRESGOLD"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE_MAX_NUM_THRESGOLD">ACTIVE_MAX_NUM_THRESGOLD</a>: u64 = 200;
</code></pre>



<a name="0xc8_bfc_dao_ACTIVE_MIN_NUM_THRESGOLD"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE_MIN_NUM_THRESGOLD">ACTIVE_MIN_NUM_THRESGOLD</a>: u64 = 20;
</code></pre>



<a name="0xc8_bfc_dao_AGREED"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_AGREED">AGREED</a>: u8 = 4;
</code></pre>



<a name="0xc8_bfc_dao_DEFAULT_BFC_SUPPLY"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_BFC_SUPPLY">DEFAULT_BFC_SUPPLY</a>: u64 = 100000000000000000;
</code></pre>



<a name="0xc8_bfc_dao_DEFAULT_MIN_ACTION_DELAY"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_MIN_ACTION_DELAY">DEFAULT_MIN_ACTION_DELAY</a>: u64 = 604800000;
</code></pre>



<a name="0xc8_bfc_dao_DEFAULT_START_PROPOSAL_VERSION_ID"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_START_PROPOSAL_VERSION_ID">DEFAULT_START_PROPOSAL_VERSION_ID</a>: u64 = 19;
</code></pre>



<a name="0xc8_bfc_dao_DEFAULT_VOTE_DELAY"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_VOTE_DELAY">DEFAULT_VOTE_DELAY</a>: u64 = 259200000;
</code></pre>



<a name="0xc8_bfc_dao_DEFAULT_VOTE_PERIOD"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_VOTE_PERIOD">DEFAULT_VOTE_PERIOD</a>: u64 = 604800000;
</code></pre>



<a name="0xc8_bfc_dao_DEFAULT_VOTE_QUORUM_RATE"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_VOTE_QUORUM_RATE">DEFAULT_VOTE_QUORUM_RATE</a>: u8 = 10;
</code></pre>



<a name="0xc8_bfc_dao_DEFEATED"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_DEFEATED">DEFEATED</a>: u8 = 3;
</code></pre>



<a name="0xc8_bfc_dao_ERR_ACTION_DELAY_TOO_SMALL"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_DELAY_TOO_SMALL">ERR_ACTION_DELAY_TOO_SMALL</a>: u64 = 1402;
</code></pre>



<a name="0xc8_bfc_dao_ERR_ACTION_ID_ALREADY_INDAO"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_ID_ALREADY_INDAO">ERR_ACTION_ID_ALREADY_INDAO</a>: u64 = 1414;
</code></pre>



<a name="0xc8_bfc_dao_ERR_ACTION_ID_NOT_EXIST"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_ID_NOT_EXIST">ERR_ACTION_ID_NOT_EXIST</a>: u64 = 1422;
</code></pre>



<a name="0xc8_bfc_dao_ERR_ACTION_MUST_EXIST"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_MUST_EXIST">ERR_ACTION_MUST_EXIST</a>: u64 = 1409;
</code></pre>



<a name="0xc8_bfc_dao_ERR_ACTION_NAME_TOO_LONG"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_NAME_TOO_LONG">ERR_ACTION_NAME_TOO_LONG</a>: u64 = 1416;
</code></pre>



<a name="0xc8_bfc_dao_ERR_ACTION_NUM_TOO_LITTLE"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_NUM_TOO_LITTLE">ERR_ACTION_NUM_TOO_LITTLE</a>: u64 = 1420;
</code></pre>



<a name="0xc8_bfc_dao_ERR_ACTION_NUM_TOO_MUCH"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_NUM_TOO_MUCH">ERR_ACTION_NUM_TOO_MUCH</a>: u64 = 1418;
</code></pre>



<a name="0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>: u64 = 1407;
</code></pre>



<a name="0xc8_bfc_dao_ERR_DESCRIPTION_TOO_LONG"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_DESCRIPTION_TOO_LONG">ERR_DESCRIPTION_TOO_LONG</a>: u64 = 1417;
</code></pre>



<a name="0xc8_bfc_dao_ERR_EINSUFFICIENT_FUNDS"></a>

Error codes


<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_EINSUFFICIENT_FUNDS">ERR_EINSUFFICIENT_FUNDS</a>: u64 = 1001;
</code></pre>



<a name="0xc8_bfc_dao_ERR_INVALID_STRING"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_INVALID_STRING">ERR_INVALID_STRING</a>: u64 = 1413;
</code></pre>



<a name="0xc8_bfc_dao_ERR_NOT_AUTHORIZED"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_NOT_AUTHORIZED">ERR_NOT_AUTHORIZED</a>: u64 = 1401;
</code></pre>



<a name="0xc8_bfc_dao_ERR_PROPOSAL_ID_MISMATCH"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_ID_MISMATCH">ERR_PROPOSAL_ID_MISMATCH</a>: u64 = 1404;
</code></pre>



<a name="0xc8_bfc_dao_ERR_PROPOSAL_NOT_EXIST"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_NOT_EXIST">ERR_PROPOSAL_NOT_EXIST</a>: u64 = 1415;
</code></pre>



<a name="0xc8_bfc_dao_ERR_PROPOSAL_NUM_TOO_LITTLE"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_NUM_TOO_LITTLE">ERR_PROPOSAL_NUM_TOO_LITTLE</a>: u64 = 1421;
</code></pre>



<a name="0xc8_bfc_dao_ERR_PROPOSAL_NUM_TOO_MANY"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_NUM_TOO_MANY">ERR_PROPOSAL_NUM_TOO_MANY</a>: u64 = 1419;
</code></pre>



<a name="0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>: u64 = 1403;
</code></pre>



<a name="0xc8_bfc_dao_ERR_PROPOSER_MISMATCH"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSER_MISMATCH">ERR_PROPOSER_MISMATCH</a>: u64 = 1405;
</code></pre>



<a name="0xc8_bfc_dao_ERR_QUORUM_RATE_INVALID"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_QUORUM_RATE_INVALID">ERR_QUORUM_RATE_INVALID</a>: u64 = 1406;
</code></pre>



<a name="0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>: u64 = 1411;
</code></pre>



<a name="0xc8_bfc_dao_ERR_VOTED_OTHERS_ALREADY"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_OTHERS_ALREADY">ERR_VOTED_OTHERS_ALREADY</a>: u64 = 1410;
</code></pre>



<a name="0xc8_bfc_dao_ERR_VOTE_STATE_MISMATCH"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTE_STATE_MISMATCH">ERR_VOTE_STATE_MISMATCH</a>: u64 = 1408;
</code></pre>



<a name="0xc8_bfc_dao_ERR_WRONG_VOTING_POOL"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ERR_WRONG_VOTING_POOL">ERR_WRONG_VOTING_POOL</a>: u64 = 1412;
</code></pre>



<a name="0xc8_bfc_dao_EXECUTABLE"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_EXECUTABLE">EXECUTABLE</a>: u8 = 6;
</code></pre>



<a name="0xc8_bfc_dao_EXTRACTED"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_EXTRACTED">EXTRACTED</a>: u8 = 7;
</code></pre>



<a name="0xc8_bfc_dao_MAX_ACTION_NAME_LENGTH"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MAX_ACTION_NAME_LENGTH">MAX_ACTION_NAME_LENGTH</a>: u64 = 100;
</code></pre>



<a name="0xc8_bfc_dao_MAX_ADMIN_COUNT"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MAX_ADMIN_COUNT">MAX_ADMIN_COUNT</a>: u64 = 1000;
</code></pre>



<a name="0xc8_bfc_dao_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>: u64 = 200;
</code></pre>



<a name="0xc8_bfc_dao_MAX_TIME_PERIOD"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>: u64 = 3153600000000;
</code></pre>



<a name="0xc8_bfc_dao_MAX_VOTE_AMOUNT"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MAX_VOTE_AMOUNT">MAX_VOTE_AMOUNT</a>: u64 = 1000000000000000000;
</code></pre>



<a name="0xc8_bfc_dao_MIN_NEW_ACTION_COST"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MIN_NEW_ACTION_COST">MIN_NEW_ACTION_COST</a>: u64 = 10000000000;
</code></pre>



<a name="0xc8_bfc_dao_MIN_NEW_PROPOSE_COST"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MIN_NEW_PROPOSE_COST">MIN_NEW_PROPOSE_COST</a>: u64 = 200000000000;
</code></pre>



<a name="0xc8_bfc_dao_MIN_STAKE_MANAGER_KEY_COST"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MIN_STAKE_MANAGER_KEY_COST">MIN_STAKE_MANAGER_KEY_COST</a>: u64 = 100000000000000;
</code></pre>



<a name="0xc8_bfc_dao_MIN_VOTING_THRESHOLD"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>: u64 = 1000000000;
</code></pre>



<a name="0xc8_bfc_dao_PENDING"></a>

Proposal state


<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_PENDING">PENDING</a>: u8 = 1;
</code></pre>



<a name="0xc8_bfc_dao_QUEUED"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_QUEUED">QUEUED</a>: u8 = 5;
</code></pre>



<a name="0xc8_bfc_dao_ZERO_ADDRESS"></a>



<pre><code><b>const</b> <a href="bfc_dao.md#0xc8_bfc_dao_ZERO_ADDRESS">ZERO_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0xc8_bfc_dao_getProposalRecord"></a>

## Function `getProposalRecord`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_getProposalRecord">getProposalRecord</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): <a href="../sui-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">bfc_dao::ProposalInfo</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_getProposalRecord">getProposalRecord</a>(dao : &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>) :VecMap&lt;u64, <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a>&gt;{
    dao.proposal_record
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_get_bfcdao_actionid"></a>

## Function `get_bfcdao_actionid`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_get_bfcdao_actionid">get_bfcdao_actionid</a>(bfcDaoAction: <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">bfc_dao::BFCDaoAction</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_get_bfcdao_actionid">get_bfcdao_actionid</a>(bfcDaoAction: <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a>): u64 {
    bfcDaoAction.action_id
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_create_bfcdao_action"></a>

## Function `create_bfcdao_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_bfcdao_action">create_bfcdao_action</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, payment: &<b>mut</b> <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, actionName: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">bfc_dao::BFCDaoAction</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_bfcdao_action">create_bfcdao_action</a>(
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    actionName:<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext
): <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a> {

    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    // ensure the user pays enough
    <b>assert</b>!(<a href="../sui-framework/coin.md#0x2_coin_value">coin::value</a>(payment) &gt;= <a href="bfc_dao.md#0xc8_bfc_dao_MIN_NEW_ACTION_COST">MIN_NEW_ACTION_COST</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_EINSUFFICIENT_FUNDS">ERR_EINSUFFICIENT_FUNDS</a>);
    <b>assert</b>!(<a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&actionName) &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_ACTION_NAME_LENGTH">MAX_ACTION_NAME_LENGTH</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_NAME_TOO_LONG">ERR_ACTION_NAME_TOO_LONG</a>);
    <b>let</b> size=<a href="../sui-framework/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&dao.action_record);
    <b>assert</b>!(size &lt; <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE_MAX_NUM_THRESGOLD">ACTIVE_MAX_NUM_THRESGOLD</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_NUM_TOO_MUCH">ERR_ACTION_NUM_TOO_MUCH</a>);

    // burn 10 BFC <b>to</b> prevent DDOS attacks
    <b>let</b> burn_bfc=<a href="../sui-framework/coin.md#0x2_coin_split">coin::split</a>(payment, <a href="bfc_dao.md#0xc8_bfc_dao_MIN_NEW_ACTION_COST">MIN_NEW_ACTION_COST</a>, ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(burn_bfc, <a href="bfc_dao.md#0xc8_bfc_dao_ZERO_ADDRESS">ZERO_ADDRESS</a>);

    <b>let</b> <b>mut</b> nameString = <a href="../move-stdlib/string.md#0x1_string_try_utf8">string::try_utf8</a>(actionName);
    <b>assert</b>!(nameString != <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), <a href="bfc_dao.md#0xc8_bfc_dao_ERR_INVALID_STRING">ERR_INVALID_STRING</a>);

    <b>let</b> name_ref = <a href="../move-stdlib/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> nameString);
    <b>let</b> action_id = <a href="bfc_dao.md#0xc8_bfc_dao_generate_next_action_id">generate_next_action_id</a>(dao);

    <b>let</b> action = <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a>{
        action_id: action_id,
        name: name_ref,
        status: <b>false</b>,
    };

    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_ActionCreateEvent">ActionCreateEvent</a>{
            actionId: action_id,
            name: name_ref,
            creator: sender,
        }
    );

    <b>assert</b>!(<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&dao.action_record, &action_id) == <b>false</b>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_ID_ALREADY_INDAO">ERR_ACTION_ID_ALREADY_INDAO</a>);
    <a href="../sui-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> dao.action_record, action_id, <b>copy</b> action);
    action
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_remove_action"></a>

## Function `remove_action`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_remove_action">remove_action</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, actionId: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_remove_action">remove_action</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,_: &BFCDaoManageKey, actionId: u64){
    <b>let</b> size=<a href="../sui-framework/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&dao.action_record);
    <b>assert</b>!(size &gt; <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE_MIN_NUM_THRESGOLD">ACTIVE_MIN_NUM_THRESGOLD</a>,<a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_NUM_TOO_LITTLE">ERR_ACTION_NUM_TOO_LITTLE</a>);
    <b>assert</b>!(<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>&lt;u64,<a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a>&gt;(&dao.action_record,&actionId),<a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_ID_NOT_EXIST">ERR_ACTION_ID_NOT_EXIST</a>);
    <a href="../sui-framework/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>&lt;u64,<a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a>&gt;(&<b>mut</b> dao.action_record,&actionId);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_remove_proposal"></a>

## Function `remove_proposal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_remove_proposal">remove_proposal</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposalId: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> (package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_remove_proposal">remove_proposal</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,_: &BFCDaoManageKey, proposalId: u64){
    <b>let</b> size=<a href="../sui-framework/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&dao.proposal_record);
    <b>assert</b>!(size &gt; <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE_MIN_NUM_THRESGOLD">ACTIVE_MIN_NUM_THRESGOLD</a>,<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_NUM_TOO_LITTLE">ERR_PROPOSAL_NUM_TOO_LITTLE</a>);
    <b>assert</b>!(<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>&lt;u64,<a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a>&gt;(&dao.proposal_record,&proposalId),<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_ID_MISMATCH">ERR_PROPOSAL_ID_MISMATCH</a>);
    <a href="../sui-framework/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>&lt;u64,<a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a>&gt;(&<b>mut</b> dao.proposal_record,&proposalId);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_create_dao"></a>

## Function `create_dao`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_dao">create_dao</a>(admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_dao">create_dao</a>(
                                admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
                              ctx: &<b>mut</b> TxContext ) : <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a> {


    <b>assert</b>!( <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&admins) &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_ADMIN_COUNT">MAX_ADMIN_COUNT</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a> );
    <b>assert</b>!( <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&admins) &gt; 0, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a> );


    <b>let</b> daoConfig = <a href="bfc_dao.md#0xc8_bfc_dao_new_dao_config">new_dao_config</a>(<a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_VOTE_DELAY">DEFAULT_VOTE_DELAY</a>,
        <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_VOTE_PERIOD">DEFAULT_VOTE_PERIOD</a>,
        <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_VOTE_QUORUM_RATE">DEFAULT_VOTE_QUORUM_RATE</a>,
        <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_MIN_ACTION_DELAY">DEFAULT_MIN_ACTION_DELAY</a>);


    <b>let</b> daoInfo = <a href="bfc_dao.md#0xc8_bfc_dao_DaoGlobalInfo">DaoGlobalInfo</a>{
        next_proposal_id: <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_START_PROPOSAL_VERSION_ID">DEFAULT_START_PROPOSAL_VERSION_ID</a>,
        next_action_id: 1,
    };

    // <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
    //     <a href="bfc_dao.md#0xc8_bfc_dao_ProposalCreatedEvent">ProposalCreatedEvent</a>{
    //         proposal_id: <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_START_PROPOSAL_VERSION_ID">DEFAULT_START_PROPOSAL_VERSION_ID</a>,
    //         proposer: DEFAULT_TOKEN_ADDRESS,
    //     }
    // );

    // <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
    //     <a href="bfc_dao.md#0xc8_bfc_dao_VoteChangedEvent">VoteChangedEvent</a>{
    //         proposal_id: <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_START_PROPOSAL_VERSION_ID">DEFAULT_START_PROPOSAL_VERSION_ID</a>,
    //         voter: DEFAULT_TOKEN_ADDRESS,
    //         proposer: DEFAULT_TOKEN_ADDRESS,
    //         agree: <b>false</b>,
    //         vote: 0,
    // });

    <b>let</b> votingPool = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_new">voting_pool::new</a>(ctx);
    <b>let</b> rootAdmin = <a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&admins, 0);
    <b>let</b> dao_obj = <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>{
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        admin: *rootAdmin,  //using the first of the admins <b>as</b> the admin of the dao
        config: daoConfig,
        info: daoInfo,
        proposal_record: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        action_record: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        votes_record: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        <a href="bfc_dao_voting_pool.md#0xc8_voting_pool">voting_pool</a>: votingPool,
        current_proposal_status: <a href="../sui-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
    };


    <a href="bfc_dao.md#0xc8_bfc_dao_set_admins">set_admins</a>(admins,  ctx);

    dao_obj
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_getDaoActionByActionId"></a>

## Function `getDaoActionByActionId`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_getDaoActionByActionId">getDaoActionByActionId</a>(dao: &<a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, actionId: u64): <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">bfc_dao::BFCDaoAction</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_getDaoActionByActionId">getDaoActionByActionId</a>(dao: &<a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>, actionId: u64) : <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a> {
    <b>let</b> data = <a href="../sui-framework/vec_map.md#0x2_vec_map_get">vec_map::get</a>(&dao.action_record, &actionId);
    *data
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_new_dao_config"></a>

## Function `new_dao_config`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_new_dao_config">new_dao_config</a>(voting_delay: u64, voting_period: u64, voting_quorum_rate: u8, min_action_delay: u64): <a href="bfc_dao.md#0xc8_bfc_dao_DaoConfig">bfc_dao::DaoConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_new_dao_config">new_dao_config</a>(
    voting_delay: u64,
    voting_period: u64,
    voting_quorum_rate: u8,
    min_action_delay: u64,
): <a href="bfc_dao.md#0xc8_bfc_dao_DaoConfig">DaoConfig</a> {
    <b>assert</b>!(voting_delay &gt; 0 && <a href="bfc_dao.md#0xc8_bfc_dao_voting_delay">voting_delay</a> &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>);
    <b>assert</b>!(voting_period&gt; 0 && <a href="bfc_dao.md#0xc8_bfc_dao_voting_period">voting_period</a> &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>);
    <b>assert</b>!(min_action_delay &gt; 0 && <a href="bfc_dao.md#0xc8_bfc_dao_min_action_delay">min_action_delay</a> &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>);
    <b>assert</b>!(voting_quorum_rate &gt;= 1 && <a href="bfc_dao.md#0xc8_bfc_dao_voting_quorum_rate">voting_quorum_rate</a> &lt;= 100, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>);

    <a href="bfc_dao.md#0xc8_bfc_dao_DaoConfig">DaoConfig</a> { voting_delay, voting_period, voting_quorum_rate, min_action_delay }
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_propose"></a>

## Function `propose`

propose a proposal.
<code>action</code>: the actual action to execute.
<code>action_delay</code>: the delay to execute after the proposal is agreed


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_propose">propose</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, version_id: u64, payment: &<b>mut</b> <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, action_id: u64, action_delay: u64, description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_propose">propose</a> (
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    version_id: u64,
    payment: &<b>mut</b> Coin&lt;BFC&gt;,
    action_id: u64,
    <b>mut</b> action_delay: u64,
    description: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock,
    ctx: &<b>mut</b> TxContext,
) {

    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    // ensure the user pays enough
    <b>assert</b>!(<a href="../sui-framework/coin.md#0x2_coin_value">coin::value</a>(payment) &gt;= <a href="bfc_dao.md#0xc8_bfc_dao_MIN_NEW_PROPOSE_COST">MIN_NEW_PROPOSE_COST</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_EINSUFFICIENT_FUNDS">ERR_EINSUFFICIENT_FUNDS</a>);
    <b>assert</b>!( <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&description) &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_ACTION_NAME_TOO_LONG">ERR_ACTION_NAME_TOO_LONG</a>);

    <b>let</b> size=<a href="../sui-framework/vec_map.md#0x2_vec_map_size">vec_map::size</a>(&dao.proposal_record);
    <b>assert</b>!(size &lt; <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE_MAX_NUM_THRESGOLD">ACTIVE_MAX_NUM_THRESGOLD</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_NUM_TOO_MANY">ERR_PROPOSAL_NUM_TOO_MANY</a>);

    // burn 200 BFC <b>to</b> prevent DDOS attacks
    <b>let</b> burn_bfc=<a href="../sui-framework/coin.md#0x2_coin_split">coin::split</a>(payment, <a href="bfc_dao.md#0xc8_bfc_dao_MIN_NEW_PROPOSE_COST">MIN_NEW_PROPOSE_COST</a>, ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(burn_bfc, <a href="bfc_dao.md#0xc8_bfc_dao_ZERO_ADDRESS">ZERO_ADDRESS</a>);


    <b>let</b> action = <a href="bfc_dao.md#0xc8_bfc_dao_getDaoActionByActionId">getDaoActionByActionId</a>(dao, action_id);

    <b>if</b> (action_delay &lt;= 0 || action_delay &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_min_action_delay">min_action_delay</a>(dao)) {
        action_delay = <a href="bfc_dao.md#0xc8_bfc_dao_min_action_delay">min_action_delay</a>(dao);
    };

    <b>let</b> proposal_id = <a href="bfc_dao.md#0xc8_bfc_dao_generate_next_proposal_id">generate_next_proposal_id</a>(dao);
    <b>let</b> start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  + <a href="bfc_dao.md#0xc8_bfc_dao_voting_delay">voting_delay</a>(dao);
    <b>let</b> quorum_votes = <a href="bfc_dao.md#0xc8_bfc_dao_quorum_votes">quorum_votes</a>(dao);
    <b>let</b> object_id = <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx);

    <b>let</b> <b>mut</b> descriptionString = <a href="../move-stdlib/string.md#0x1_string_try_utf8">string::try_utf8</a>(description);
    <b>assert</b>!(descriptionString != <a href="../move-stdlib/option.md#0x1_option_none">option::none</a>(), <a href="bfc_dao.md#0xc8_bfc_dao_ERR_INVALID_STRING">ERR_INVALID_STRING</a>);

    <b>let</b> description_ref = <a href="../move-stdlib/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> descriptionString);

    <b>let</b> proposalInfo = <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a> {
        proposal_uid: <a href="../sui-framework/object.md#0x2_object_uid_to_address">object::uid_to_address</a>(&object_id),
        pid: proposal_id,
        proposer: sender,
        start_time,
        end_time: start_time + <a href="bfc_dao.md#0xc8_bfc_dao_voting_period">voting_period</a>(dao),
        for_votes: 0,
        against_votes: 0,
        eta: 0,
        action_delay,
        quorum_votes,
        action,
        version_id,
        description: description_ref,
    };

    <b>let</b> proposal = <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>{
        id: object_id,
        proposal: <b>copy</b> proposalInfo,
    };
    <a href="../sui-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> dao.proposal_record, proposal_id, proposalInfo);


    <a href="../sui-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(proposal);

    // emit <a href="../sui-framework/event.md#0x2_event">event</a>
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_ProposalCreatedEvent">ProposalCreatedEvent</a>{
            proposal_id,
            proposer: sender,
        }
    );
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_synchronize_proposal_into_dao"></a>

## Function `synchronize_proposal_into_dao`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_synchronize_proposal_into_dao">synchronize_proposal_into_dao</a>(proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_synchronize_proposal_into_dao">synchronize_proposal_into_dao</a>(proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>, dao:  &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>) {
    <b>if</b> (<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>( &dao.proposal_record,&proposal.proposal.pid)) {
        <b>let</b> <b>old</b> = <a href="../sui-framework/vec_map.md#0x2_vec_map_get_mut">vec_map::get_mut</a>(&<b>mut</b> dao.proposal_record,& proposal.proposal.pid);
        *<b>old</b> = proposal.proposal;
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_cast_vote"></a>

## Function `cast_vote`

votes for a proposal.
User can only vote once, then the vote is locked,
which can only be un vote by user after the proposal is expired, or cancelled, or executed.
So think twice before casting vote.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_cast_vote">cast_vote</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/coin.md#0x2_coin">coin</a>: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, agreeInt: u8, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_cast_vote">cast_vote</a>(
    dao:  &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    <a href="../sui-framework/coin.md#0x2_coin">coin</a>: VotingBfc,
    agreeInt: u8,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
)  {
    <b>let</b> agree = agreeInt == 1;

    {
        <b>let</b> state = <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal,<a href="../sui-framework/clock.md#0x2_clock">clock</a>);
        // only when proposal is active, <b>use</b> can cast vote.
        <b>assert</b>!(state == <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE">ACTIVE</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>));
    };

    <b>let</b> vote_amount = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_voting_bfc_amount">voting_pool::voting_bfc_amount</a>(&<a href="../sui-framework/coin.md#0x2_coin">coin</a>);
    {
        <b>assert</b>!(vote_amount &gt;= <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>);
        <b>assert</b>!(vote_amount &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_VOTE_AMOUNT">MAX_VOTE_AMOUNT</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>);
    };

    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

    <b>let</b> total_voted = {

        <b>let</b> voteCoin = <a href="../sui-framework/coin.md#0x2_coin">coin</a>;

        <b>let</b> my_vote = <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a> {
            id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
            vid: proposal.proposal.pid,
            proposer: proposal.proposal.proposer,
            vote: voteCoin,
            agree,
        };

        <b>if</b> (agree) {
            proposal.proposal.for_votes = proposal.proposal.for_votes + vote_amount;
        } <b>else</b> {
            proposal.proposal.against_votes = proposal.proposal.against_votes + vote_amount;
        };
        <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(my_vote, sender);

        vote_amount
    };

    <a href="bfc_dao.md#0xc8_bfc_dao_synchronize_proposal_into_dao">synchronize_proposal_into_dao</a>(proposal, dao);
    // emit <a href="../sui-framework/event.md#0x2_event">event</a>
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_VoteChangedEvent">VoteChangedEvent</a>{
            proposal_id: proposal.proposal.pid,
            voter: sender,
            proposer: proposal.proposal.proposer,
            agree,
            vote: total_voted,
        });
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_change_vote"></a>

## Function `change_vote`

Let user change their vote during the voting time.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_change_vote">change_vote</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, my_vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, agree: bool, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_change_vote">change_vote</a>(
    dao:  &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    my_vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a>,
    proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    agree: bool,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
)  {
    {
        <b>let</b> state = <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
        // only when proposal is active, user can change vote.
        <b>assert</b>!(state == <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE">ACTIVE</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>));
    };


    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    //<b>let</b> total_voted = voting_bfc_amount(&my_vote.vote);
    {
        <b>assert</b>!(my_vote.proposer == proposal.proposal.proposer, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSER_MISMATCH">ERR_PROPOSER_MISMATCH</a>));
        <b>assert</b>!(my_vote.vid == proposal.proposal.pid, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_OTHERS_ALREADY">ERR_VOTED_OTHERS_ALREADY</a>));

    };

    // flip the vote
    <b>if</b> (my_vote.agree != agree) {
        <b>let</b> total_voted = <a href="bfc_dao.md#0xc8_bfc_dao_do_flip_vote">do_flip_vote</a>(my_vote, proposal);

        <a href="bfc_dao.md#0xc8_bfc_dao_synchronize_proposal_into_dao">synchronize_proposal_into_dao</a>(proposal, dao);
        // emit <a href="../sui-framework/event.md#0x2_event">event</a>
        <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
            <a href="bfc_dao.md#0xc8_bfc_dao_VoteChangedEvent">VoteChangedEvent</a>{
                proposal_id: proposal.proposal.pid,
                voter: sender,
                proposer: proposal.proposal.proposer,
                agree,
                vote: total_voted, });

    };
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_do_flip_vote"></a>

## Function `do_flip_vote`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_do_flip_vote">do_flip_vote</a>(my_vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_do_flip_vote">do_flip_vote</a>(my_vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a>,
                 proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>): u64 {
    my_vote.agree = !my_vote.agree;
    <b>let</b> total_voted = voting_bfc_amount(&my_vote.vote);
    <b>if</b> (my_vote.agree) {
        <b>assert</b>!(proposal.proposal.against_votes &gt;= total_voted, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>));
        <b>assert</b>!(proposal.proposal.for_votes + total_voted &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_VOTE_AMOUNT">MAX_VOTE_AMOUNT</a> , (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>));

        proposal.proposal.for_votes = proposal.proposal.for_votes + total_voted;
        proposal.proposal.against_votes = proposal.proposal.against_votes - total_voted;
    } <b>else</b> {
        <b>assert</b>!(proposal.proposal.for_votes &gt;= total_voted, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>));
        <b>assert</b>!(proposal.proposal.against_votes + total_voted &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_VOTE_AMOUNT">MAX_VOTE_AMOUNT</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>));

        proposal.proposal.for_votes = proposal.proposal.for_votes - total_voted;
        proposal.proposal.against_votes = proposal.proposal.against_votes + total_voted;
    };
    total_voted
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_revoke_vote"></a>

## Function `revoke_vote`

Revoke some voting powers from vote on <code>proposal_id</code> of <code>proposer_address</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_revoke_vote">revoke_vote</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, my_vote: <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, voting_power: u64, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_revoke_vote">revoke_vote</a>(
    dao:  &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    <b>mut</b> my_vote: <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a>,
    voting_power: u64,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &<b>mut</b> TxContext,
){
    {
        <b>let</b> state = <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
        // only when proposal is active, user can revoke vote.
        <b>assert</b>!(state == <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE">ACTIVE</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>));
    };
    // get proposal

    // get vote
    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    {
        <b>assert</b>!(my_vote.proposer == proposal.proposal.proposer, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSER_MISMATCH">ERR_PROPOSER_MISMATCH</a>));
        <b>assert</b>!(my_vote.vid == proposal.proposal.pid, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_OTHERS_ALREADY">ERR_VOTED_OTHERS_ALREADY</a>));
        <b>assert</b>!(voting_bfc_amount(&my_vote.vote) &gt;= voting_power, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>));
        <b>assert</b>!(voting_power &gt;= <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a> && voting_power &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_VOTE_AMOUNT">MAX_VOTE_AMOUNT</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>));
        <b>assert</b>!(voting_bfc_amount(&my_vote.vote) - voting_power &gt;= <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_ERR_AMOUNT">ERR_VOTED_ERR_AMOUNT</a>));
    };
    // revoke vote on proposal
    <a href="bfc_dao.md#0xc8_bfc_dao_do_revoke_vote">do_revoke_vote</a>(proposal, &<b>mut</b> my_vote, voting_power,ctx);

    <a href="bfc_dao.md#0xc8_bfc_dao_synchronize_proposal_into_dao">synchronize_proposal_into_dao</a>(proposal, dao);

    // emit vote changed <a href="../sui-framework/event.md#0x2_event">event</a>
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_VoteChangedEvent">VoteChangedEvent</a>{
            proposal_id: proposal.proposal.pid,
            voter: sender,
            proposer: proposal.proposal.proposer,
            agree: my_vote.agree,
            vote: voting_bfc_amount(&my_vote.vote),
        }
    );

    <b>if</b> (voting_bfc_amount(&my_vote.vote) == 0u64) {
        <b>let</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a> {
            proposer: _,
            id: uid,
            vid: _,
            vote,
            agree: _} = my_vote;

        <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(uid);
        <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(vote, sender);
    } <b>else</b> {
        <b>let</b> some_vote = my_vote;
        <a href="../sui-framework/transfer.md#0x2_transfer_transfer">transfer::transfer</a>(some_vote, sender);
    };

    //todo <a href="../sui-framework/transfer.md#0x2_transfer">transfer</a> back
    //reverted_vote
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_do_revoke_vote"></a>

## Function `do_revoke_vote`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_do_revoke_vote">do_revoke_vote</a>(proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, to_revoke: u64, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_do_revoke_vote">do_revoke_vote</a>(
    proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    vote: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a>,
    to_revoke: u64,
    ctx: &<b>mut</b> TxContext,
){
    // <b>spec</b> {
    //     <b>assume</b> vote.vote.principal.value &gt;= to_revoke;
    // };

    //todo: unlock vote <a href="../sui-framework/coin.md#0x2_coin">coin</a> or <b>return</b>...
    //// Token::withdraw(&<b>mut</b> vote.vote, to_revoke);
    <b>let</b> reverted_vote = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_split">voting_pool::split</a>(&<b>mut</b> vote.vote, to_revoke, ctx);

    <b>if</b> (vote.agree) {
        proposal.proposal.for_votes = proposal.proposal.for_votes - to_revoke;
    } <b>else</b> {
        proposal.proposal.against_votes = proposal.proposal.against_votes - to_revoke;
    };
    // <b>spec</b> {
    //     <b>assert</b> reverted_vote.principal.value == to_revoke;
    // };

    //reverted_vote
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(reverted_vote, <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_unvote_votes"></a>

## Function `unvote_votes`

Retrieve back my voted token voted for a proposal.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_unvote_votes">unvote_votes</a>(proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, vote: <a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_unvote_votes">unvote_votes</a>(
    proposal: & <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    vote: <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
    ctx: &TxContext,
) {
    // only check state when proposal exists.
    // because proposal can be destroyed after it ends in <a href="bfc_dao.md#0xc8_bfc_dao_DEFEATED">DEFEATED</a> or <a href="bfc_dao.md#0xc8_bfc_dao_EXTRACTED">EXTRACTED</a> state.
    {
        <b>let</b> state = <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal,<a href="../sui-framework/clock.md#0x2_clock">clock</a>);
        // Only after vote period end, user can unvote his votes.
        <b>assert</b>!(state &gt; <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE">ACTIVE</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>));
    };

    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);


    // delete vote.
    <b>let</b> <a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a> { proposer, id,vid, vote, agree: _ } = vote;


    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);



    // these checks are still required.
    <b>assert</b>!(proposer == proposal.proposal.proposer, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSER_MISMATCH">ERR_PROPOSER_MISMATCH</a>));
    <b>assert</b>!(vid == proposal.proposal.pid, (
        <a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_OTHERS_ALREADY">ERR_VOTED_OTHERS_ALREADY</a>));

    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(vote, sender);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_vote_of"></a>

## Function `vote_of`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_vote_of">vote_of</a>(vote: &<a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_vote_of">vote_of</a>(
    vote: &<a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a>,
    proposal: & <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    ctx: &<b>mut</b> TxContext,
){
    <b>assert</b>!(vote.proposer == proposal.proposal.proposer, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSER_MISMATCH">ERR_PROPOSER_MISMATCH</a>));
    <b>assert</b>!(vote.vid == proposal.proposal.pid, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_VOTED_OTHERS_ALREADY">ERR_VOTED_OTHERS_ALREADY</a>));
    //(vote.agree, staking_pool::vote_sui_amount(&vote.vote))
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_VoteInfoEvent">VoteInfoEvent</a>{
            proposal_id: proposal.proposal.pid,
            voter: <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            proposer: proposal.proposal.proposer,
            agree: vote.agree,
            vote: voting_bfc_amount(&vote.vote),
        }
    );
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_has_vote"></a>

## Function `has_vote`

Check whether voter has voted on proposal with <code>proposal_id</code> of <code>proposer_address</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_has_vote">has_vote</a>(vote: &<a href="bfc_dao.md#0xc8_bfc_dao_Vote">bfc_dao::Vote</a>, proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_has_vote">has_vote</a>(
    vote: &<a href="bfc_dao.md#0xc8_bfc_dao_Vote">Vote</a>,
    proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
): bool  {
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_BooleanEvent">BooleanEvent</a>{value:
        vote.proposer == proposal.proposal.proposer && vote.vid == proposal.proposal.pid});

    vote.proposer == proposal.proposal.proposer && vote.vid == proposal.proposal.pid
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_queue_proposal_action"></a>

## Function `queue_proposal_action`

queue agreed proposal to execute.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_queue_proposal_action">queue_proposal_action</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_queue_proposal_action">queue_proposal_action</a>(
    dao:  &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    _: &BFCDaoManageKey,
    proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
)  {

    //<b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

        // Only agreed proposal can be submitted.
        <b>assert</b>!(
            <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>) == <a href="bfc_dao.md#0xc8_bfc_dao_AGREED">AGREED</a>,
            (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>)
        );
    <b>assert</b>!(proposal.proposal.action_delay &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>);

    proposal.proposal.eta =  <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  + proposal.proposal.action_delay;

    <a href="bfc_dao.md#0xc8_bfc_dao_synchronize_proposal_into_dao">synchronize_proposal_into_dao</a>(proposal, dao);
    //send_bfc_dao_event(manager_key, b"proposal_queued");
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_extract_proposal_action"></a>

## Function `extract_proposal_action`

extract proposal action to execute.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_extract_proposal_action">extract_proposal_action</a>(proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>): <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">bfc_dao::BFCDaoAction</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_extract_proposal_action">extract_proposal_action</a>(
    proposal: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
): <a href="bfc_dao.md#0xc8_bfc_dao_BFCDaoAction">BFCDaoAction</a>  {
    // Only executable proposal's action can be extracted.
    <b>assert</b>!(
        <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal, <a href="../sui-framework/clock.md#0x2_clock">clock</a>) == <a href="bfc_dao.md#0xc8_bfc_dao_EXECUTABLE">EXECUTABLE</a>,
        (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>),
    );
    <b>let</b> action = proposal.proposal.action;
    action
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_proposal_exists"></a>

## Function `proposal_exists`

check whether a proposal exists in <code>proposer_address</code> with id <code>proposal_id</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_proposal_exists">proposal_exists</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_proposal_exists">proposal_exists</a> (
    dao : &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
): bool {
    <b>let</b> result = <a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&dao.proposal_record, &proposal.proposal.pid);
    result
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_proposal_state"></a>

## Function `proposal_state`

Get the proposal state.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(
    proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
): u8  {
    //<b>assert</b>!(proposal.proposal.pid == proposal.proposal.pid, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_ID_MISMATCH">ERR_PROPOSAL_ID_MISMATCH</a>));
    <b>let</b> current_time =  <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) ;
    <b>let</b> status = <a href="bfc_dao.md#0xc8_bfc_dao_judge_proposal_state">judge_proposal_state</a>(& proposal.proposal, current_time);

    // emit <a href="../sui-framework/event.md#0x2_event">event</a>
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_ProposalStateEvent">ProposalStateEvent</a> {
            proposalId: proposal.proposal.pid,
            state: status,
        });
    status
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_judge_proposal_state"></a>

## Function `judge_proposal_state`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_judge_proposal_state">judge_proposal_state</a>(proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">bfc_dao::ProposalInfo</a>, current_time: u64): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_judge_proposal_state">judge_proposal_state</a>(
    proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a>,
    current_time: u64,
): u8 {
    <b>if</b> (current_time &lt; proposal.start_time) {
        // Pending
        <a href="bfc_dao.md#0xc8_bfc_dao_PENDING">PENDING</a>
    } <b>else</b> <b>if</b> (current_time &lt;= proposal.end_time) {
        // Active
        <a href="bfc_dao.md#0xc8_bfc_dao_ACTIVE">ACTIVE</a>
    } <b>else</b> <b>if</b> (proposal.for_votes &lt;= proposal.against_votes ||
        proposal.for_votes &lt; proposal.quorum_votes) {
        // Defeated
        <a href="bfc_dao.md#0xc8_bfc_dao_DEFEATED">DEFEATED</a>
    } <b>else</b> <b>if</b> (proposal.eta == 0) {
        // Agreed.
        <a href="bfc_dao.md#0xc8_bfc_dao_AGREED">AGREED</a>
    } <b>else</b> <b>if</b> (current_time &lt; proposal.eta) {
        // Queued, waiting <b>to</b> execute
        <a href="bfc_dao.md#0xc8_bfc_dao_QUEUED">QUEUED</a>
    } <b>else</b> <b>if</b> (proposal.action.status == <b>false</b> ) {
        <a href="bfc_dao.md#0xc8_bfc_dao_EXECUTABLE">EXECUTABLE</a>
    } <b>else</b> {
        <a href="bfc_dao.md#0xc8_bfc_dao_EXTRACTED">EXTRACTED</a>
    }
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_proposal_info"></a>

## Function `proposal_info`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_proposal_info">proposal_info</a>(proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_proposal_info">proposal_info</a>(
    proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
) : (u64, u64) {
    <a href="../sui-framework/event.md#0x2_event_emit">event::emit</a>(
        <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfoEvent">ProposalInfoEvent</a>{
            proposal_id: proposal.proposal.pid,
            start_time: proposal.proposal.start_time,
            end_time: proposal.proposal.end_time,
            for_votes: proposal.proposal.for_votes,
            against_votes: proposal.proposal.against_votes,
        }
    );

    (proposal.proposal.for_votes, proposal.proposal.against_votes)
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_generate_next_proposal_id"></a>

## Function `generate_next_proposal_id`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_generate_next_proposal_id">generate_next_proposal_id</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_generate_next_proposal_id">generate_next_proposal_id</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): u64 {
    <b>let</b> info = &<b>mut</b> dao.info;
    <b>let</b> proposal_id = info.next_proposal_id;
    info.next_proposal_id = proposal_id + 1;
    proposal_id

}
</code></pre>



</details>

<a name="0xc8_bfc_dao_generate_next_action_id"></a>

## Function `generate_next_action_id`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_generate_next_action_id">generate_next_action_id</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_generate_next_action_id">generate_next_action_id</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): u64 {
    <b>let</b> info = &<b>mut</b> dao.info;
    <b>let</b> action_id = info.next_action_id;
    info.next_action_id = action_id + 1;
    action_id

}
</code></pre>



</details>

<a name="0xc8_bfc_dao_quorum_votes"></a>

## Function `quorum_votes`

Quorum votes to make proposal pass.
temply using 4000* 000_0000 as the pass rate.


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_quorum_votes">quorum_votes</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_quorum_votes">quorum_votes</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): u64 {
    <b>let</b> total_supply_sui: u64 = <a href="bfc_dao.md#0xc8_bfc_dao_DEFAULT_BFC_SUPPLY">DEFAULT_BFC_SUPPLY</a>;
    <b>let</b> supply = total_supply_sui;

    <b>let</b> rate = <a href="bfc_dao.md#0xc8_bfc_dao_voting_quorum_rate">voting_quorum_rate</a>(dao);
    <b>let</b> rate = (rate <b>as</b> u64);
    supply * rate / 100
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_voting_delay"></a>

## Function `voting_delay`

get default voting delay of the DAO.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_voting_delay">voting_delay</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_voting_delay">voting_delay</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): u64 {
    <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao).voting_delay
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_voting_period"></a>

## Function `voting_period`

get the default voting period of the DAO.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_voting_period">voting_period</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_voting_period">voting_period</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): u64 {
    <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao).voting_period
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_voting_quorum_rate"></a>

## Function `voting_quorum_rate`

Get the quorum rate in percent.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_voting_quorum_rate">voting_quorum_rate</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_voting_quorum_rate">voting_quorum_rate</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): u8 {
    <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao).voting_quorum_rate
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_min_action_delay"></a>

## Function `min_action_delay`

Get the min_action_delay of the DAO.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_min_action_delay">min_action_delay</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_min_action_delay">min_action_delay</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): u64 {
    <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao).min_action_delay
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_get_config"></a>

## Function `get_config`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>): &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_DaoConfig">bfc_dao::DaoConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>): &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_DaoConfig">DaoConfig</a> {
    &<b>mut</b> dao.config
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_modify_dao_config"></a>

## Function `modify_dao_config`

update function, modify dao config.
if any param is 0, it means no change to that param.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_modify_dao_config">modify_dao_config</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, voting_delay: u64, voting_period: u64, voting_quorum_rate: u8, min_action_delay: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_modify_dao_config">modify_dao_config</a>(
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    _: &BFCDaoManageKey,
    voting_delay: u64,
    voting_period: u64,
    voting_quorum_rate: u8,
    min_action_delay: u64,
) {

    <b>assert</b>!(<a href="bfc_dao.md#0xc8_bfc_dao_voting_delay">voting_delay</a> &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a> && voting_delay &gt; 0, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));
    <b>assert</b>!(<a href="bfc_dao.md#0xc8_bfc_dao_voting_period">voting_period</a> &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a> && voting_period &gt; 0, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));
    <b>assert</b>!(<a href="bfc_dao.md#0xc8_bfc_dao_min_action_delay">min_action_delay</a> &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a> && min_action_delay &gt; 0, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));
    <b>assert</b>!(voting_quorum_rate&gt;0 && <a href="bfc_dao.md#0xc8_bfc_dao_voting_quorum_rate">voting_quorum_rate</a> &lt;= 100, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_QUORUM_RATE_INVALID">ERR_QUORUM_RATE_INVALID</a>));



    <b>let</b> config = <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao);
    <b>if</b> (voting_period &gt; 0) {
        config.voting_period = voting_period;
    };
    <b>if</b> (voting_delay &gt; 0) {
        config.voting_delay = voting_delay;
    };
    <b>if</b> (voting_quorum_rate &gt; 0 && <a href="bfc_dao.md#0xc8_bfc_dao_voting_quorum_rate">voting_quorum_rate</a> &lt;= 100) {
        config.voting_quorum_rate = voting_quorum_rate;
    };
    <b>if</b> (min_action_delay &gt; 0) {
        config.min_action_delay = min_action_delay;
    };

    //send_bfc_dao_event(manager_key, b"modify_dao_config");
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_set_voting_delay"></a>

## Function `set_voting_delay`

set voting delay


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_delay">set_voting_delay</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_delay">set_voting_delay</a>(
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    _: &BFCDaoManageKey,
    value: u64,
) {

    <b>assert</b>!(value &gt; 0, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));
    <b>assert</b>!(value &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));

    <b>let</b> config = <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao);
    config.voting_delay = value;

}
</code></pre>



</details>

<a name="0xc8_bfc_dao_set_voting_period"></a>

## Function `set_voting_period`

set voting period


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_period">set_voting_period</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_period">set_voting_period</a>(
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    _: &BFCDaoManageKey,
    value: u64,
) {

    <b>assert</b>!(value &gt; 0, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));
    <b>assert</b>!(value &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));

    <b>let</b> config = <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao);
    config.voting_period = value;

    //send_bfc_dao_event(manager_key, b"set_voting_period");
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_set_voting_quorum_rate"></a>

## Function `set_voting_quorum_rate`

set voting quorum rate: .


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_quorum_rate">set_voting_quorum_rate</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_voting_quorum_rate">set_voting_quorum_rate</a>(
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    _: &BFCDaoManageKey,
    value: u8,
) {
    <b>assert</b>!(value &lt;= 100 && value &gt; 0, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_QUORUM_RATE_INVALID">ERR_QUORUM_RATE_INVALID</a>));
    <b>let</b> config = <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao);
    config.voting_quorum_rate = value;

    //send_bfc_dao_event(manager_key, b"set_voting_quorum_rate");
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_set_min_action_delay"></a>

## Function `set_min_action_delay`

set min action delay


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_min_action_delay">set_min_action_delay</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_min_action_delay">set_min_action_delay</a>(
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    _: &BFCDaoManageKey,
    value: u64,
) {
    <b>assert</b>!(value &gt; 0, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));
    <b>assert</b>!(value &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_TIME_PERIOD">MAX_TIME_PERIOD</a>, (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>));

    <b>let</b> config = <a href="bfc_dao.md#0xc8_bfc_dao_get_config">get_config</a>(dao);
    config.min_action_delay = value;

    //send_bfc_dao_event(manager_key, b"set_min_action_delay");
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_set_admins"></a>

## Function `set_admins`



<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_admins">set_admins</a>(new_admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_admins">set_admins</a>(
    new_admins: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    //<b>let</b> index = 0;
    <b>let</b> count = <a href="../move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&new_admins);
    <b>assert</b>!(count &gt; 0 && count &lt;= <a href="bfc_dao.md#0xc8_bfc_dao_MAX_ADMIN_COUNT">MAX_ADMIN_COUNT</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_CONFIG_PARAM_INVALID">ERR_CONFIG_PARAM_INVALID</a>);

    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        <b>let</b> admin = <a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&new_admins, i);
        <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_new">bfc_dao_manager::new</a>(*admin, ctx);
        i = i+1;
    };

}
</code></pre>



</details>

<a name="0xc8_bfc_dao_create_stake_manager_key"></a>

## Function `create_stake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_stake_manager_key">create_stake_manager_key</a>(payment: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_stake_manager_key">create_stake_manager_key</a>( payment: Coin&lt;BFC&gt;,
                              ctx: &<b>mut</b> TxContext){

    //convert proposal payment <b>to</b> voting_bfc
    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> <a href="../sui-framework/balance.md#0x2_balance">balance</a> = <a href="../sui-framework/coin.md#0x2_coin_into_balance">coin::into_balance</a>(payment);
    <b>let</b> value = <a href="../sui-framework/balance.md#0x2_balance_value">balance::value</a>(&<a href="../sui-framework/balance.md#0x2_balance">balance</a>);
    // ensure the user pays enough
    <b>assert</b>!(value &gt;= <a href="bfc_dao.md#0xc8_bfc_dao_MIN_STAKE_MANAGER_KEY_COST">MIN_STAKE_MANAGER_KEY_COST</a>, <a href="bfc_dao.md#0xc8_bfc_dao_ERR_EINSUFFICIENT_FUNDS">ERR_EINSUFFICIENT_FUNDS</a>);
    <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_create_stake_key">bfc_dao_manager::create_stake_key</a>(sender,<a href="../sui-framework/balance.md#0x2_balance">balance</a>, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_unstake_manager_key"></a>

## Function `unstake_manager_key`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_unstake_manager_key">unstake_manager_key</a>(key: <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, token: <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_ManagerKeyBfc">bfc_dao_manager::ManagerKeyBfc</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_unstake_manager_key">unstake_manager_key</a>(key: BFCDaoManageKey,
                        token: ManagerKeyBfc,
                        ctx: &<b>mut</b> TxContext){
    <a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_unstake_key">bfc_dao_manager::unstake_key</a>(key,token, ctx);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_modify_proposal_obj"></a>

## Function `modify_proposal_obj`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_modify_proposal_obj">modify_proposal_obj</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, proposal_obj: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, index: u8, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_modify_proposal_obj">modify_proposal_obj</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>, proposal_obj: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>, index : u8, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &Clock) {
    //<b>let</b> proposal = proposal_obj.proposal;
    <b>if</b> (index == 1) {
        // Pending
        proposal_obj.proposal.start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  + 1000000000;
    }<b>else</b> <b>if</b> (index == 2) {
        // active
        proposal_obj.proposal.start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 1000000000;
        proposal_obj.proposal.end_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) + 1000000000;
    } <b>else</b> <b>if</b> (index == 3){
        //afer voting  Defeated...
        proposal_obj.proposal.start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 2000000000;
        proposal_obj.proposal.end_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) - 1000000000;
        proposal_obj.proposal.for_votes = 1 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.against_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
    } <b>else</b> <b>if</b> (index == 4) {
        //afer voting <a href="bfc_dao.md#0xc8_bfc_dao_AGREED">AGREED</a>
        proposal_obj.proposal.start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 2000000000;
        proposal_obj.proposal.end_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) - 1000000000;
        proposal_obj.proposal.for_votes = 3 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.against_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.quorum_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.eta = 0;
    } <b>else</b> <b>if</b> (index == 5) {
        // Queued, waiting <b>to</b> execute
        proposal_obj.proposal.start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 2000000000;
        proposal_obj.proposal.end_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) - 1000000000;
        proposal_obj.proposal.for_votes = 3 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.against_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.quorum_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.eta = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  + 100000000;
    } <b>else</b> <b>if</b> (index == 6) {
        proposal_obj.proposal.start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 2000000000;
        proposal_obj.proposal.end_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) - 1000000000;
        proposal_obj.proposal.for_votes = 3 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.against_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.quorum_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.eta = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 100000000;
        proposal_obj.proposal.action.status = <b>false</b>;
    } <b>else</b> <b>if</b> (index == 7) {
        proposal_obj.proposal.start_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 2000000000;
        proposal_obj.proposal.end_time = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>) - 1000000000;
        proposal_obj.proposal.for_votes = 3 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.against_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.quorum_votes = 2 * <a href="bfc_dao.md#0xc8_bfc_dao_MIN_VOTING_THRESHOLD">MIN_VOTING_THRESHOLD</a>;
        proposal_obj.proposal.eta = <a href="../sui-framework/clock.md#0x2_clock_timestamp_ms">clock::timestamp_ms</a>(<a href="../sui-framework/clock.md#0x2_clock">clock</a>)  - 100000000;
        proposal_obj.proposal.action.status = <b>true</b>;
    };
    <a href="bfc_dao.md#0xc8_bfc_dao_synchronize_proposal_into_dao">synchronize_proposal_into_dao</a>(proposal_obj, dao);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_create_voting_bfc"></a>

## Function `create_voting_bfc`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_voting_bfc">create_voting_bfc</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, <a href="../sui-framework/coin.md#0x2_coin">coin</a>: <a href="../sui-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="../sui-framework/bfc.md#0x2_bfc_BFC">bfc::BFC</a>&gt;, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_create_voting_bfc">create_voting_bfc</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
                                   <a href="../sui-framework/coin.md#0x2_coin">coin</a>: Coin&lt;BFC&gt;,
                                    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
                                   ctx: &<b>mut</b> TxContext) {
    // sender <b>address</b>
    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> <a href="../sui-framework/balance.md#0x2_balance">balance</a> = <a href="../sui-framework/coin.md#0x2_coin_into_balance">coin::into_balance</a>(<a href="../sui-framework/coin.md#0x2_coin">coin</a>);
    <b>let</b> voting_bfc = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_request_add_voting">voting_pool::request_add_voting</a>(& dao.<a href="bfc_dao_voting_pool.md#0xc8_voting_pool">voting_pool</a>, <a href="../sui-framework/balance.md#0x2_balance">balance</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>,  ctx);

    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(voting_bfc, sender);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_withdraw_voting"></a>

## Function `withdraw_voting`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_withdraw_voting">withdraw_voting</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, voting_bfc: <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_VotingBfc">voting_pool::VotingBfc</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_withdraw_voting">withdraw_voting</a>(  dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
                                   voting_bfc: VotingBfc,
                                    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
                                   ctx: &<b>mut</b> TxContext ,) {
    // sender <b>address</b>
    <b>let</b> sender = <a href="../sui-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>assert</b>!(pool_id(&voting_bfc) == <a href="../sui-framework/object.md#0x2_object_id">object::id</a>(&dao.<a href="bfc_dao_voting_pool.md#0xc8_voting_pool">voting_pool</a>), <a href="bfc_dao.md#0xc8_bfc_dao_ERR_WRONG_VOTING_POOL">ERR_WRONG_VOTING_POOL</a>);
    <b>let</b> voting_bfc = <a href="bfc_dao_voting_pool.md#0xc8_voting_pool_request_withdraw_voting">voting_pool::request_withdraw_voting</a>(&dao.<a href="bfc_dao_voting_pool.md#0xc8_voting_pool">voting_pool</a>, voting_bfc, <a href="../sui-framework/clock.md#0x2_clock">clock</a>);
    <b>let</b> <a href="../sui-framework/coin.md#0x2_coin">coin</a> = <a href="../sui-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(voting_bfc, ctx);
    <a href="../sui-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../sui-framework/coin.md#0x2_coin">coin</a>, sender);
}
</code></pre>



</details>

<a name="0xc8_bfc_dao_destroy_terminated_proposal"></a>

## Function `destroy_terminated_proposal`

remove terminated proposal from proposer


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_destroy_terminated_proposal">destroy_terminated_proposal</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, _: &<a href="bfc_dao_manager.md#0xc8_bfc_dao_manager_BFCDaoManageKey">bfc_dao_manager::BFCDaoManageKey</a>, proposal: &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">bfc_dao::Proposal</a>, <a href="../sui-framework/clock.md#0x2_clock">clock</a>: &<a href="../sui-framework/clock.md#0x2_clock_Clock">clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_destroy_terminated_proposal">destroy_terminated_proposal</a>(
    dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>,
    _: &BFCDaoManageKey,
    proposal:  &<a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a>,
    <a href="../sui-framework/clock.md#0x2_clock">clock</a>: & Clock,
)  {


    <b>let</b> proposal_state = <a href="bfc_dao.md#0xc8_bfc_dao_proposal_state">proposal_state</a>(proposal,<a href="../sui-framework/clock.md#0x2_clock">clock</a>);
    <b>assert</b>!(
        proposal_state == <a href="bfc_dao.md#0xc8_bfc_dao_DEFEATED">DEFEATED</a> || proposal_state == <a href="bfc_dao.md#0xc8_bfc_dao_EXTRACTED">EXTRACTED</a>,
        (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_STATE_INVALID">ERR_PROPOSAL_STATE_INVALID</a>),
    );



    <b>assert</b>!(<a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&dao.proposal_record, &proposal.proposal.pid), (<a href="bfc_dao.md#0xc8_bfc_dao_ERR_PROPOSAL_NOT_EXIST">ERR_PROPOSAL_NOT_EXIST</a>));
    <a href="../sui-framework/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>(&<b>mut</b> dao.proposal_record, &proposal.proposal.pid);
    <b>if</b> (proposal_state == <a href="bfc_dao.md#0xc8_bfc_dao_DEFEATED">DEFEATED</a>) {
        <b>let</b> _ =  proposal.proposal.action;
    };

    // <b>let</b> <a href="bfc_dao.md#0xc8_bfc_dao_Proposal">Proposal</a> {
    //     id: uid,
    //     proposal: <a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a>{
    //         pid: _,
    //         proposer: _,
    //         start_time: _,
    //         end_time: _,
    //         for_votes: _,
    //         against_votes: _,
    //         eta: _,
    //         action_delay: _,
    //         quorum_votes: _,
    //         action: _c
    //         } ,
    //     } = proposal;
    //
    //  <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(uid);
    //send_bfc_dao_event(manager_key, b"ProposalDestroyed");

}
</code></pre>



</details>

<a name="0xc8_bfc_dao_set_current_status_into_dao"></a>

## Function `set_current_status_into_dao`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_current_status_into_dao">set_current_status_into_dao</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">bfc_dao::Dao</a>, proposalInfo: &<a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">bfc_dao::ProposalInfo</a>, curProposalStatus: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="bfc_dao.md#0xc8_bfc_dao_set_current_status_into_dao">set_current_status_into_dao</a>(dao: &<b>mut</b> <a href="bfc_dao.md#0xc8_bfc_dao_Dao">Dao</a>, proposalInfo : &<a href="bfc_dao.md#0xc8_bfc_dao_ProposalInfo">ProposalInfo</a>, curProposalStatus: u8) {
    <b>let</b> flag = <a href="../sui-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&dao.current_proposal_status, &proposalInfo.pid);
    <b>if</b> (flag) {
        <a href="../sui-framework/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>(&<b>mut</b> dao.current_proposal_status, &proposalInfo.pid);
    };

    <b>let</b> proposal_status = <a href="bfc_dao.md#0xc8_bfc_dao_ProposalStatus">ProposalStatus</a> {
        version_id : proposalInfo.version_id,
        status: curProposalStatus,
    };
    <a href="../sui-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> (dao.current_proposal_status), proposalInfo.pid, proposal_status);
}
</code></pre>



</details>

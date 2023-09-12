
<a name="0x2_dao"></a>

# Module `0x2::dao`

APIs for accessing time from move calls, via the <code>Clock</code>: a unique
shared object that is created at 0x6 during genesis.


-  [Struct `DaoConfig`](#0x2_dao_DaoConfig)
-  [Struct `ProposalCreatedEvent`](#0x2_dao_ProposalCreatedEvent)
-  [Struct `DaoGlobalInfo`](#0x2_dao_DaoGlobalInfo)
-  [Struct `VoteChangedEvent`](#0x2_dao_VoteChangedEvent)
-  [Struct `ProposalInfo`](#0x2_dao_ProposalInfo)
-  [Struct `OBCDaoAction`](#0x2_dao_OBCDaoAction)
-  [Resource `Dao`](#0x2_dao_Dao)
-  [Constants](#@Constants_0)
-  [Function `admin`](#0x2_dao_admin)
-  [Function `create`](#0x2_dao_create)
-  [Function `set_admins`](#0x2_dao_set_admins)
-  [Function `new_dao_config`](#0x2_dao_new_dao_config)


<pre><code><b>use</b> <a href="">0x1::string</a>;
<b>use</b> <a href="">0x1::vector</a>;
<b>use</b> <a href="obc_dao_manager.md#0x2_obc_dao_manager">0x2::obc_dao_manager</a>;
<b>use</b> <a href="object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="vec_map.md#0x2_vec_map">0x2::vec_map</a>;
</code></pre>



<a name="0x2_dao_DaoConfig"></a>

## Struct `DaoConfig`



<pre><code><b>struct</b> <a href="dao.md#0x2_dao_DaoConfig">DaoConfig</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="0x2_dao_ProposalCreatedEvent"></a>

## Struct `ProposalCreatedEvent`



<pre><code><b>struct</b> <a href="dao.md#0x2_dao_ProposalCreatedEvent">ProposalCreatedEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="0x2_dao_DaoGlobalInfo"></a>

## Struct `DaoGlobalInfo`



<pre><code><b>struct</b> <a href="dao.md#0x2_dao_DaoGlobalInfo">DaoGlobalInfo</a> <b>has</b> <b>copy</b>, store
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
<dt>
<code>proposal_create_event: <a href="dao.md#0x2_dao_ProposalCreatedEvent">dao::ProposalCreatedEvent</a></code>
</dt>
<dd>
 proposal creating event.
</dd>
<dt>
<code>vote_changed_event: <a href="dao.md#0x2_dao_VoteChangedEvent">dao::VoteChangedEvent</a></code>
</dt>
<dd>
 voting event.
</dd>
</dl>


</details>

<a name="0x2_dao_VoteChangedEvent"></a>

## Struct `VoteChangedEvent`



<pre><code><b>struct</b> <a href="dao.md#0x2_dao_VoteChangedEvent">VoteChangedEvent</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="0x2_dao_ProposalInfo"></a>

## Struct `ProposalInfo`



<pre><code><b>struct</b> <a href="dao.md#0x2_dao_ProposalInfo">ProposalInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>action: <a href="dao.md#0x2_dao_OBCDaoAction">dao::OBCDaoAction</a></code>
</dt>
<dd>
 proposal action.
</dd>
</dl>


</details>

<a name="0x2_dao_OBCDaoAction"></a>

## Struct `OBCDaoAction`



<pre><code><b>struct</b> <a href="dao.md#0x2_dao_OBCDaoAction">OBCDaoAction</a> <b>has</b> <b>copy</b>, drop, store
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
<code>name: <a href="_String">string::String</a></code>
</dt>
<dd>
 Name for the action
</dd>
</dl>


</details>

<a name="0x2_dao_Dao"></a>

## Resource `Dao`

Singleton shared object that exposes time to Move calls.  This
object is found at address 0x6, and can only be read (accessed
via an immutable reference) by entry functions.

Entry Functions that attempt to accept <code>Clock</code> by mutable
reference or value will fail to verify, and honest validators
will not sign or execute transactions that use <code>Clock</code> as an
input parameter, unless it is passed by immutable reference.


<pre><code><b>struct</b> <a href="dao.md#0x2_dao_Dao">Dao</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>config: <a href="dao.md#0x2_dao_DaoConfig">dao::DaoConfig</a></code>
</dt>
<dd>

</dd>
<dt>
<code>info: <a href="dao.md#0x2_dao_DaoGlobalInfo">dao::DaoGlobalInfo</a></code>
</dt>
<dd>

</dd>
<dt>
<code>proposalRecord: <a href="vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, <a href="dao.md#0x2_dao_ProposalInfo">dao::ProposalInfo</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>actionRecord: <a href="vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, <a href="dao.md#0x2_dao_OBCDaoAction">dao::OBCDaoAction</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>votesRecord: <a href="vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;u64, u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_dao_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="dao.md#0x2_dao_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="0x2_dao_DEFAULT_MIN_ACTION_DELAY"></a>



<pre><code><b>const</b> <a href="dao.md#0x2_dao_DEFAULT_MIN_ACTION_DELAY">DEFAULT_MIN_ACTION_DELAY</a>: u64 = 25200000;
</code></pre>



<a name="0x2_dao_DEFAULT_TOKEN_ADDRESS"></a>



<pre><code><b>const</b> <a href="dao.md#0x2_dao_DEFAULT_TOKEN_ADDRESS">DEFAULT_TOKEN_ADDRESS</a>: <b>address</b> = 0;
</code></pre>



<a name="0x2_dao_DEFAULT_VOTE_DELAY"></a>



<pre><code><b>const</b> <a href="dao.md#0x2_dao_DEFAULT_VOTE_DELAY">DEFAULT_VOTE_DELAY</a>: u64 = 3600000;
</code></pre>



<a name="0x2_dao_DEFAULT_VOTE_PERIOD"></a>



<pre><code><b>const</b> <a href="dao.md#0x2_dao_DEFAULT_VOTE_PERIOD">DEFAULT_VOTE_PERIOD</a>: u64 = 25200000;
</code></pre>



<a name="0x2_dao_DEFAULT_VOTE_QUORUM_RATE"></a>



<pre><code><b>const</b> <a href="dao.md#0x2_dao_DEFAULT_VOTE_QUORUM_RATE">DEFAULT_VOTE_QUORUM_RATE</a>: u8 = 50;
</code></pre>



<a name="0x2_dao_admin"></a>

## Function `admin`



<pre><code><b>public</b> entry <b>fun</b> <a href="dao.md#0x2_dao_admin">admin</a>(<a href="dao.md#0x2_dao">dao</a>: &<b>mut</b> <a href="dao.md#0x2_dao_Dao">dao::Dao</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="dao.md#0x2_dao_admin">admin</a>(<a href="dao.md#0x2_dao">dao</a>: &<b>mut</b> <a href="dao.md#0x2_dao_Dao">Dao</a>) {
    <a href="dao.md#0x2_dao">dao</a>.config.min_action_delay = 25200001;
}
</code></pre>



</details>

<a name="0x2_dao_create"></a>

## Function `create`

Create and share the singleton Clock -- this function is
called exactly once, during genesis.


<pre><code><b>public</b> entry <b>fun</b> <a href="dao.md#0x2_dao_create">create</a>(ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="dao.md#0x2_dao_create">create</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = <a href="tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);

    <b>let</b> daoConfig = <a href="dao.md#0x2_dao_new_dao_config">new_dao_config</a>(<a href="dao.md#0x2_dao_DEFAULT_VOTE_DELAY">DEFAULT_VOTE_DELAY</a>,
        <a href="dao.md#0x2_dao_DEFAULT_VOTE_PERIOD">DEFAULT_VOTE_PERIOD</a>,
        <a href="dao.md#0x2_dao_DEFAULT_VOTE_QUORUM_RATE">DEFAULT_VOTE_QUORUM_RATE</a>,
        <a href="dao.md#0x2_dao_DEFAULT_MIN_ACTION_DELAY">DEFAULT_MIN_ACTION_DELAY</a>);


    <b>let</b> daoInfo = <a href="dao.md#0x2_dao_DaoGlobalInfo">DaoGlobalInfo</a>{
        next_proposal_id: 0,
        next_action_id: 0,
        proposal_create_event: <a href="dao.md#0x2_dao_ProposalCreatedEvent">ProposalCreatedEvent</a>{
            proposal_id: 0,
            proposer: <a href="dao.md#0x2_dao_DEFAULT_TOKEN_ADDRESS">DEFAULT_TOKEN_ADDRESS</a>,
        },
        vote_changed_event: <a href="dao.md#0x2_dao_VoteChangedEvent">VoteChangedEvent</a>{
            proposal_id: 0,
            voter: <a href="dao.md#0x2_dao_DEFAULT_TOKEN_ADDRESS">DEFAULT_TOKEN_ADDRESS</a>,
            proposer: <a href="dao.md#0x2_dao_DEFAULT_TOKEN_ADDRESS">DEFAULT_TOKEN_ADDRESS</a>,
            agree: <b>false</b>,
            vote: 0,
        }
    };
    <b>let</b> dao_obj = <a href="dao.md#0x2_dao_Dao">Dao</a>{
        id: <a href="object.md#0x2_object_dao">object::dao</a>(),
        admin: sender,
        config: daoConfig,
        info: daoInfo,
        proposalRecord: <a href="vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        actionRecord: <a href="vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        votesRecord: <a href="vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),

    };


    <a href="transfer.md#0x2_transfer_share_object">transfer::share_object</a>(dao_obj);

   // <a href="dao.md#0x2_dao_set_admins">set_admins</a>(admins, ctx);
}
</code></pre>



</details>

<a name="0x2_dao_set_admins"></a>

## Function `set_admins`



<pre><code><b>public</b> <b>fun</b> <a href="dao.md#0x2_dao_set_admins">set_admins</a>(new_admins: <a href="">vector</a>&lt;<b>address</b>&gt;, ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dao.md#0x2_dao_set_admins">set_admins</a>(
    new_admins: <a href="">vector</a>&lt;<b>address</b>&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>while</b> (!<a href="_is_empty">vector::is_empty</a>(&new_admins)) {
        <b>let</b> admin = <a href="_pop_back">vector::pop_back</a>(&<b>mut</b> new_admins);
        <a href="obc_dao_manager.md#0x2_obc_dao_manager_new">obc_dao_manager::new</a>(admin, ctx);
    }

}
</code></pre>



</details>

<a name="0x2_dao_new_dao_config"></a>

## Function `new_dao_config`



<pre><code><b>public</b> <b>fun</b> <a href="dao.md#0x2_dao_new_dao_config">new_dao_config</a>(voting_delay: u64, voting_period: u64, voting_quorum_rate: u8, min_action_delay: u64): <a href="dao.md#0x2_dao_DaoConfig">dao::DaoConfig</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="dao.md#0x2_dao_new_dao_config">new_dao_config</a>(
    voting_delay: u64,
    voting_period: u64,
    voting_quorum_rate: u8,
    min_action_delay: u64,
): <a href="dao.md#0x2_dao_DaoConfig">DaoConfig</a> {
    <a href="dao.md#0x2_dao_DaoConfig">DaoConfig</a> { voting_delay, voting_period, voting_quorum_rate, min_action_delay }
}
</code></pre>



</details>

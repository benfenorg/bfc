---
title: Module `sui_system::validator_wrapper`
---



-  [Struct `ValidatorWrapper`](#sui_system_validator_wrapper_ValidatorWrapper)
-  [Constants](#@Constants_0)
-  [Function `create_v1`](#sui_system_validator_wrapper_create_v1)
-  [Function `load_validator_maybe_upgrade`](#sui_system_validator_wrapper_load_validator_maybe_upgrade)
-  [Function `destroy`](#sui_system_validator_wrapper_destroy)
-  [Function `upgrade_to_latest`](#sui_system_validator_wrapper_upgrade_to_latest)
-  [Function `version`](#sui_system_validator_wrapper_version)


<pre><code><b>use</b> <a href="../bfc_system/bars.md#bfc_system_bars">bfc_system::bars</a>;
<b>use</b> <a href="../bfc_system/baud.md#bfc_system_baud">bfc_system::baud</a>;
<b>use</b> <a href="../bfc_system/bbrl.md#bfc_system_bbrl">bfc_system::bbrl</a>;
<b>use</b> <a href="../bfc_system/bcad.md#bfc_system_bcad">bfc_system::bcad</a>;
<b>use</b> <a href="../bfc_system/beur.md#bfc_system_beur">bfc_system::beur</a>;
<b>use</b> <a href="../bfc_system/bgbp.md#bfc_system_bgbp">bfc_system::bgbp</a>;
<b>use</b> <a href="../bfc_system/bidr.md#bfc_system_bidr">bfc_system::bidr</a>;
<b>use</b> <a href="../bfc_system/binr.md#bfc_system_binr">bfc_system::binr</a>;
<b>use</b> <a href="../bfc_system/bjpy.md#bfc_system_bjpy">bfc_system::bjpy</a>;
<b>use</b> <a href="../bfc_system/bkrw.md#bfc_system_bkrw">bfc_system::bkrw</a>;
<b>use</b> <a href="../bfc_system/bmxn.md#bfc_system_bmxn">bfc_system::bmxn</a>;
<b>use</b> <a href="../bfc_system/brub.md#bfc_system_brub">bfc_system::brub</a>;
<b>use</b> <a href="../bfc_system/bsar.md#bfc_system_bsar">bfc_system::bsar</a>;
<b>use</b> <a href="../bfc_system/btry.md#bfc_system_btry">bfc_system::btry</a>;
<b>use</b> <a href="../bfc_system/busd.md#bfc_system_busd">bfc_system::busd</a>;
<b>use</b> <a href="../bfc_system/bzar.md#bfc_system_bzar">bfc_system::bzar</a>;
<b>use</b> <a href="../bfc_system/mgg.md#bfc_system_mgg">bfc_system::mgg</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
<b>use</b> <a href="../sui/address.md#sui_address">sui::address</a>;
<b>use</b> <a href="../sui/bag.md#sui_bag">sui::bag</a>;
<b>use</b> <a href="../sui/balance.md#sui_balance">sui::balance</a>;
<b>use</b> <a href="../sui/bfc.md#sui_bfc">sui::bfc</a>;
<b>use</b> <a href="../sui/coin.md#sui_coin">sui::coin</a>;
<b>use</b> <a href="../sui/config.md#sui_config">sui::config</a>;
<b>use</b> <a href="../sui/deny_list.md#sui_deny_list">sui::deny_list</a>;
<b>use</b> <a href="../sui/dynamic_field.md#sui_dynamic_field">sui::dynamic_field</a>;
<b>use</b> <a href="../sui/dynamic_object_field.md#sui_dynamic_object_field">sui::dynamic_object_field</a>;
<b>use</b> <a href="../sui/event.md#sui_event">sui::event</a>;
<b>use</b> <a href="../sui/hex.md#sui_hex">sui::hex</a>;
<b>use</b> <a href="../sui/object.md#sui_object">sui::object</a>;
<b>use</b> <a href="../sui/party.md#sui_party">sui::party</a>;
<b>use</b> <a href="../sui/table.md#sui_table">sui::table</a>;
<b>use</b> <a href="../sui/transfer.md#sui_transfer">sui::transfer</a>;
<b>use</b> <a href="../sui/tx_context.md#sui_tx_context">sui::tx_context</a>;
<b>use</b> <a href="../sui/types.md#sui_types">sui::types</a>;
<b>use</b> <a href="../sui/url.md#sui_url">sui::url</a>;
<b>use</b> <a href="../sui/vec_map.md#sui_vec_map">sui::vec_map</a>;
<b>use</b> <a href="../sui/vec_set.md#sui_vec_set">sui::vec_set</a>;
<b>use</b> <a href="../sui/versioned.md#sui_versioned">sui::versioned</a>;
<b>use</b> <a href="../sui_system/stable_pool.md#sui_system_stable_pool">sui_system::stable_pool</a>;
<b>use</b> <a href="../sui_system/staking_pool.md#sui_system_staking_pool">sui_system::staking_pool</a>;
<b>use</b> <a href="../sui_system/validator.md#sui_system_validator">sui_system::validator</a>;
<b>use</b> <a href="../sui_system/validator_cap.md#sui_system_validator_cap">sui_system::validator_cap</a>;
</code></pre>



<a name="sui_system_validator_wrapper_ValidatorWrapper"></a>

## Struct `ValidatorWrapper`



<pre><code><b>public</b> <b>struct</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: <a href="../sui/versioned.md#sui_versioned_Versioned">sui::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="sui_system_validator_wrapper_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>: u64 = 0;
</code></pre>



<a name="sui_system_validator_wrapper_create_v1"></a>

## Function `create_v1`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_create_v1">create_v1</a>(<a href="../sui_system/validator.md#sui_system_validator">validator</a>: <a href="../sui_system/validator.md#sui_system_validator_Validator">sui_system::validator::Validator</a>, ctx: &<b>mut</b> <a href="../sui/tx_context.md#sui_tx_context_TxContext">sui::tx_context::TxContext</a>): <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">sui_system::validator_wrapper::ValidatorWrapper</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_create_v1">create_v1</a>(<a href="../sui_system/validator.md#sui_system_validator">validator</a>: Validator, ctx: &<b>mut</b> TxContext): <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
    <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
        inner: versioned::create(1, <a href="../sui_system/validator.md#sui_system_validator">validator</a>, ctx),
    }
}
</code></pre>



</details>

<a name="sui_system_validator_wrapper_load_validator_maybe_upgrade"></a>

## Function `load_validator_maybe_upgrade`

This function should always return the latest supported version.
If the inner version is old, we upgrade it lazily in-place.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">sui_system::validator_wrapper::ValidatorWrapper</a>): &<b>mut</b> <a href="../sui_system/validator.md#sui_system_validator_Validator">sui_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): &<b>mut</b> Validator {
    self.<a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>();
    self.inner.load_value_mut()
}
</code></pre>



</details>

<a name="sui_system_validator_wrapper_destroy"></a>

## Function `destroy`

Destroy the wrapper and retrieve the inner validator object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_destroy">destroy</a>(self: <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">sui_system::validator_wrapper::ValidatorWrapper</a>): <a href="../sui_system/validator.md#sui_system_validator_Validator">sui_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_destroy">destroy</a>(<b>mut</b> self: <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): Validator {
    <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(&<b>mut</b> self);
    <b>let</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> { inner } = self;
    versioned::destroy(inner)
}
</code></pre>



</details>

<a name="sui_system_validator_wrapper_upgrade_to_latest"></a>

## Function `upgrade_to_latest`



<pre><code><b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<b>mut</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">sui_system::validator_wrapper::ValidatorWrapper</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<b>mut</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>) {
    <b>let</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_version">version</a> = <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_version">version</a>(self);
    // TODO: When new versions are added, we need to explicitly upgrade here.
    <b>assert</b>!(<a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_version">version</a> == 1, <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>);
}
</code></pre>



</details>

<a name="sui_system_validator_wrapper_version"></a>

## Function `version`



<pre><code><b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_version">version</a>(self: &<a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">sui_system::validator_wrapper::ValidatorWrapper</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_version">version</a>(self: &<a href="../sui_system/validator_wrapper.md#sui_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): u64 {
    versioned::version(&self.inner)
}
</code></pre>



</details>

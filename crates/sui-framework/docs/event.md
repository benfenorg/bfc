
<a name="0xc8_event"></a>

# Module `0xc8::event`



-  [Struct `SwapEvent`](#0xc8_event_SwapEvent)
-  [Function `swap`](#0xc8_event_swap)
-  [Module Specification](#@Module_Specification_0)


<pre><code><b>use</b> <a href="">0x1::ascii</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../../../.././build/Sui/docs/object.md#0x2_object">0x2::object</a>;
</code></pre>



<a name="0xc8_event_SwapEvent"></a>

## Struct `SwapEvent`



<pre><code><b>struct</b> <a href="event.md#0xc8_event_SwapEvent">SwapEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>atob: bool</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="vault.md#0xc8_vault">vault</a>: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_in: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_type_out: <a href="_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount_in: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>amount_out: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vault_a_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vault_b_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>before_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>after_sqrt_price: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>steps: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_event_swap"></a>

## Function `swap`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_swap">swap</a>(vault_id: <a href="../../../.././build/Sui/docs/object.md#0x2_object_ID">object::ID</a>, atob: bool, coin_type_in: <a href="_String">ascii::String</a>, coin_type_out: <a href="_String">ascii::String</a>, amount_in: u64, amount_out: u64, vault_a_amount: u64, vault_b_amount: u64, before_sqrt_price: u128, after_sqrt_price: u128, steps: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="event.md#0xc8_event_swap">swap</a>(
    vault_id: ID,
    atob: bool, // <b>true</b> a-&gt;b <b>false</b> b-&gt;a
    coin_type_in: String,
    coin_type_out: String,
    amount_in: u64,
    amount_out: u64,
    vault_a_amount: u64, // current <a href="vault.md#0xc8_vault">vault</a> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>(A)
    vault_b_amount: u64, // current <a href="vault.md#0xc8_vault">vault</a> <a href="../../../.././build/Sui/docs/balance.md#0x2_balance">balance</a>(B)
    before_sqrt_price: u128,
    after_sqrt_price: u128,
    steps: u64
) {
    emit(
        <a href="event.md#0xc8_event_SwapEvent">SwapEvent</a> {
            <a href="vault.md#0xc8_vault">vault</a>: vault_id,
            atob,
            coin_type_in,
            coin_type_out,
            amount_in,
            amount_out,
            vault_a_amount,
            vault_b_amount,
            before_sqrt_price,
            after_sqrt_price,
            steps
        }
    )
}
</code></pre>



</details>

<a name="@Module_Specification_0"></a>

## Module Specification



<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>

---
title: Module `0xc8::linked_table`
---



-  [Resource `LinkedTable`](#0xc8_linked_table_LinkedTable)
-  [Struct `Node`](#0xc8_linked_table_Node)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_linked_table_new)
-  [Function `is_empty`](#0xc8_linked_table_is_empty)
-  [Function `length`](#0xc8_linked_table_length)
-  [Function `contains`](#0xc8_linked_table_contains)
-  [Function `head`](#0xc8_linked_table_head)
-  [Function `tail`](#0xc8_linked_table_tail)
-  [Function `next`](#0xc8_linked_table_next)
-  [Function `prev`](#0xc8_linked_table_prev)
-  [Function `borrow`](#0xc8_linked_table_borrow)
-  [Function `borrow_mut`](#0xc8_linked_table_borrow_mut)
-  [Function `borrow_node`](#0xc8_linked_table_borrow_node)
-  [Function `borrow_mut_node`](#0xc8_linked_table_borrow_mut_node)
-  [Function `borrow_value`](#0xc8_linked_table_borrow_value)
-  [Function `borrow_mut_value`](#0xc8_linked_table_borrow_mut_value)
-  [Function `push_back`](#0xc8_linked_table_push_back)
-  [Function `push_front`](#0xc8_linked_table_push_front)
-  [Function `insert_before`](#0xc8_linked_table_insert_before)
-  [Function `insert_after`](#0xc8_linked_table_insert_after)
-  [Function `remove`](#0xc8_linked_table_remove)
-  [Function `destroy_empty`](#0xc8_linked_table_destroy_empty)
-  [Function `drop`](#0xc8_linked_table_drop)
-  [Function `fetch`](#0xc8_linked_table_fetch)


<pre><code><b>use</b> <a href="../move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
</code></pre>



<a name="0xc8_linked_table_LinkedTable"></a>

## Resource `LinkedTable`



<pre><code><b>struct</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K: <b>copy</b>, drop, store, V: store&gt; <b>has</b> store, key
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
<code>head: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>tail: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>size: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0xc8_linked_table_Node"></a>

## Struct `Node`



<pre><code><b>struct</b> <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K: <b>copy</b>, drop, store, V: store&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>prev: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>next: <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>value: V</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_linked_table_ELimitInvalid"></a>



<pre><code><b>const</b> <a href="linked_table.md#0xc8_linked_table_ELimitInvalid">ELimitInvalid</a>: u64 = 1;
</code></pre>



<a name="0xc8_linked_table_EListNotEmpty"></a>



<pre><code><b>const</b> <a href="linked_table.md#0xc8_linked_table_EListNotEmpty">EListNotEmpty</a>: u64 = 0;
</code></pre>



<a name="0xc8_linked_table_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_new">new</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_new">new</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(ctx: &<b>mut</b> TxContext): <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt; {
    <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt; {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        head: none&lt;K&gt;(),
        tail: none&lt;K&gt;(),
        size: 0
    }
}
</code></pre>



</details>

<a name="0xc8_linked_table_is_empty"></a>

## Function `is_empty`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_is_empty">is_empty</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_is_empty">is_empty</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;): bool {
    <a href="../sui-framework/table.md#0x2_table">table</a>.size == 0
}
</code></pre>



</details>

<a name="0xc8_linked_table_length"></a>

## Function `length`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_length">length</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_length">length</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;): u64 {
    <a href="../sui-framework/table.md#0x2_table">table</a>.size
}
</code></pre>



</details>

<a name="0xc8_linked_table_contains"></a>

## Function `contains`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_contains">contains</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_contains">contains</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;, key: K): bool {
    field::exists_with_type&lt;K, <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;&gt;(&<a href="../sui-framework/table.md#0x2_table">table</a>.id, key)
}
</code></pre>



</details>

<a name="0xc8_linked_table_head"></a>

## Function `head`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_head">head</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_head">head</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt; {
    <a href="../sui-framework/table.md#0x2_table">table</a>.head
}
</code></pre>



</details>

<a name="0xc8_linked_table_tail"></a>

## Function `tail`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_tail">tail</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_tail">tail</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt; {
    <a href="../sui-framework/table.md#0x2_table">table</a>.tail
}
</code></pre>



</details>

<a name="0xc8_linked_table_next"></a>

## Function `next`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_next">next</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(node: &<a href="linked_table.md#0xc8_linked_table_Node">linked_table::Node</a>&lt;K, V&gt;): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_next">next</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(node: &<a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;): Option&lt;K&gt; {
    node.next
}
</code></pre>



</details>

<a name="0xc8_linked_table_prev"></a>

## Function `prev`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_prev">prev</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(node: &<a href="linked_table.md#0xc8_linked_table_Node">linked_table::Node</a>&lt;K, V&gt;): <a href="../move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;K&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_prev">prev</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(node: &<a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;): Option&lt;K&gt; {
    node.prev
}
</code></pre>



</details>

<a name="0xc8_linked_table_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow">borrow</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow">borrow</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;, key: K): &V {
    &field::borrow&lt;K, <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;&gt;(&<a href="../sui-framework/table.md#0x2_table">table</a>.id, key).value
}
</code></pre>



</details>

<a name="0xc8_linked_table_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_mut">borrow_mut</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;, key: K): &<b>mut</b> V {
    &<b>mut</b> field::borrow_mut&lt;K, <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;&gt;(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, key).value
}
</code></pre>



</details>

<a name="0xc8_linked_table_borrow_node"></a>

## Function `borrow_node`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_node">borrow_node</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K): &<a href="linked_table.md#0xc8_linked_table_Node">linked_table::Node</a>&lt;K, V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_node">borrow_node</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;, key: K): &<a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt; {
    field::borrow&lt;K, <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;&gt;(&<a href="../sui-framework/table.md#0x2_table">table</a>.id, key)
}
</code></pre>



</details>

<a name="0xc8_linked_table_borrow_mut_node"></a>

## Function `borrow_mut_node`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K): &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_Node">linked_table::Node</a>&lt;K, V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(
    <a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;,
    key: K
): &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt; {
    field::borrow_mut&lt;K, <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;&gt;(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, key)
}
</code></pre>



</details>

<a name="0xc8_linked_table_borrow_value"></a>

## Function `borrow_value`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_value">borrow_value</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(node: &<a href="linked_table.md#0xc8_linked_table_Node">linked_table::Node</a>&lt;K, V&gt;): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_value">borrow_value</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(node: &<a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;): &V {
    &node.value
}
</code></pre>



</details>

<a name="0xc8_linked_table_borrow_mut_value"></a>

## Function `borrow_mut_value`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_mut_value">borrow_mut_value</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(node: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_Node">linked_table::Node</a>&lt;K, V&gt;): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_borrow_mut_value">borrow_mut_value</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(node: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;): &<b>mut</b> V {
    &<b>mut</b> node.value
}
</code></pre>



</details>

<a name="0xc8_linked_table_push_back"></a>

## Function `push_back`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_push_back">push_back</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_push_back">push_back</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;, key: K, value: V) {
    <b>let</b> node = <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt; {
        prev: <a href="../sui-framework/table.md#0x2_table">table</a>.tail,
        next: none(),
        value
    };
    swap_or_fill(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.tail, key);
    <b>if</b> (is_none(&<a href="../sui-framework/table.md#0x2_table">table</a>.head)) {
        swap_or_fill(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.head, key);
    };
    <b>if</b> (is_some(&node.prev)) {
        <b>let</b> prev_node = <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>(<a href="../sui-framework/table.md#0x2_table">table</a>, *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&node.prev));
        swap_or_fill(&<b>mut</b> prev_node.next, key);
    };
    field::add(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, key, node);
    <a href="../sui-framework/table.md#0x2_table">table</a>.size = <a href="../sui-framework/table.md#0x2_table">table</a>.size + 1;
}
</code></pre>



</details>

<a name="0xc8_linked_table_push_front"></a>

## Function `push_front`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_push_front">push_front</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_push_front">push_front</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;, key: K, value: V) {
    <b>let</b> node = <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt; {
        prev: none(),
        next: <a href="../sui-framework/table.md#0x2_table">table</a>.head,
        value
    };
    swap_or_fill(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.head, key);
    <b>if</b> (is_none(&<a href="../sui-framework/table.md#0x2_table">table</a>.tail)) {
        swap_or_fill(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.tail, key);
    };
    <b>if</b> (is_some(&node.next)) {
        <b>let</b> next_node = <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>(<a href="../sui-framework/table.md#0x2_table">table</a>, *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&node.next));
        swap_or_fill(&<b>mut</b> next_node.prev, key);
    };
    field::add(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, key, node);
    <a href="../sui-framework/table.md#0x2_table">table</a>.size = <a href="../sui-framework/table.md#0x2_table">table</a>.size + 1;
}
</code></pre>



</details>

<a name="0xc8_linked_table_insert_before"></a>

## Function `insert_before`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_insert_before">insert_before</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, current_key: K, key: K, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_insert_before">insert_before</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(
    <a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;,
    current_key: K,
    key: K,
    value: V
) {
    <b>let</b> current_node = <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>(<a href="../sui-framework/table.md#0x2_table">table</a>, current_key);
    <b>let</b> node = <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt; {
        prev: current_node.prev,
        next: some(current_key),
        value
    };
    swap_or_fill(&<b>mut</b> current_node.prev, key);
    <b>if</b> (is_some(&node.prev)) {
        <b>let</b> prev_node = <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>(<a href="../sui-framework/table.md#0x2_table">table</a>, *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&node.prev));
        swap_or_fill(&<b>mut</b> prev_node.next, key);
    } <b>else</b> {
        swap_or_fill(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.head, key);
    };
    field::add(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, key, node);
    <a href="../sui-framework/table.md#0x2_table">table</a>.size = <a href="../sui-framework/table.md#0x2_table">table</a>.size + 1;
}
</code></pre>



</details>

<a name="0xc8_linked_table_insert_after"></a>

## Function `insert_after`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_insert_after">insert_after</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, current_key: K, key: K, value: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_insert_after">insert_after</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(
    <a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;,
    current_key: K,
    key: K,
    value: V
) {
    <b>let</b> current_node = <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>(<a href="../sui-framework/table.md#0x2_table">table</a>, current_key);
    <b>let</b> node = <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt; {
        prev: some(current_key),
        next: current_node.next,
        value
    };
    swap_or_fill(&<b>mut</b> current_node.next, key);

    <b>if</b> (is_some(&node.next)) {
        <b>let</b> next_node = <a href="linked_table.md#0xc8_linked_table_borrow_mut_node">borrow_mut_node</a>(<a href="../sui-framework/table.md#0x2_table">table</a>, *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&node.next));
        swap_or_fill(&<b>mut</b> next_node.prev, key);
    } <b>else</b> {
        swap_or_fill(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.tail, key);
    };
    field::add(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, key, node);
    <a href="../sui-framework/table.md#0x2_table">table</a>.size = <a href="../sui-framework/table.md#0x2_table">table</a>.size + 1;
}
</code></pre>



</details>

<a name="0xc8_linked_table_remove"></a>

## Function `remove`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_remove">remove</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, key: K): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_remove">remove</a>&lt;K: store + drop + <b>copy</b>, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<b>mut</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;, key: K): V {
    <b>let</b> <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt; { prev, next, value } = field::remove(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, key);
    <a href="../sui-framework/table.md#0x2_table">table</a>.size = <a href="../sui-framework/table.md#0x2_table">table</a>.size - 1;
    <b>if</b> (<a href="../move-stdlib/option.md#0x1_option_is_some">option::is_some</a>(&prev)) {
        field::borrow_mut&lt;K, <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;&gt;(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&prev)).next = next
    };
    <b>if</b> (<a href="../move-stdlib/option.md#0x1_option_is_some">option::is_some</a>(&next)) {
        field::borrow_mut&lt;K, <a href="linked_table.md#0xc8_linked_table_Node">Node</a>&lt;K, V&gt;&gt;(&<b>mut</b> <a href="../sui-framework/table.md#0x2_table">table</a>.id, *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&next)).prev = prev
    };
    <b>if</b> (<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&<a href="../sui-framework/table.md#0x2_table">table</a>.head) == &key) <a href="../sui-framework/table.md#0x2_table">table</a>.head = next;
    <b>if</b> (<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&<a href="../sui-framework/table.md#0x2_table">table</a>.tail) == &key) <a href="../sui-framework/table.md#0x2_table">table</a>.tail = prev;
    value
}
</code></pre>



</details>

<a name="0xc8_linked_table_destroy_empty"></a>

## Function `destroy_empty`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_destroy_empty">destroy_empty</a>&lt;K: <b>copy</b>, drop, store, V: drop, store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_destroy_empty">destroy_empty</a>&lt;K: store + <b>copy</b> + drop, V: store + drop&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;) {
    <b>let</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a> { id, size, head: _, tail: _ } = <a href="../sui-framework/table.md#0x2_table">table</a>;
    <b>assert</b>!(size == 0, <a href="linked_table.md#0xc8_linked_table_EListNotEmpty">EListNotEmpty</a>);
    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id)
}
</code></pre>



</details>

<a name="0xc8_linked_table_drop"></a>

## Function `drop`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_drop">drop</a>&lt;K: <b>copy</b>, drop, store, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: <a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_drop">drop</a>&lt;K: store + <b>copy</b> + drop, V: store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;) {
    <b>let</b> <a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a> { id, size: _, head: _, tail: _ } = <a href="../sui-framework/table.md#0x2_table">table</a>;
    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id)
}
</code></pre>



</details>

<a name="0xc8_linked_table_fetch"></a>

## Function `fetch`



<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_fetch">fetch</a>&lt;K: <b>copy</b>, drop, store, V: <b>copy</b>, store&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">linked_table::LinkedTable</a>&lt;K, V&gt;, start_key: K, limit: u64): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="linked_table.md#0xc8_linked_table_fetch">fetch</a>&lt;K: store + <b>copy</b> + drop, V: <b>copy</b> + store&gt;(
    <a href="../sui-framework/table.md#0x2_table">table</a>: &<a href="linked_table.md#0xc8_linked_table_LinkedTable">LinkedTable</a>&lt;K, V&gt;,
    start_key: K,
    limit: u64
): <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;V&gt; {
    <b>assert</b>!(limit &gt; 0, <a href="linked_table.md#0xc8_linked_table_ELimitInvalid">ELimitInvalid</a>);
    <b>let</b> <b>mut</b> values = <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;V&gt;();
    <b>let</b> <b>mut</b> start = <a href="linked_table.md#0xc8_linked_table_borrow_node">borrow_node</a>&lt;K, V&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>, start_key);
    <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> values, *<a href="linked_table.md#0xc8_linked_table_borrow_value">borrow_value</a>(start));
    <b>let</b> <b>mut</b> idx = 1;
    <b>while</b> (idx &lt; limit) {
        <b>let</b> next_key = <a href="linked_table.md#0xc8_linked_table_next">next</a>(start);
        <b>if</b> (<a href="../move-stdlib/option.md#0x1_option_is_some">option::is_some</a>(&next_key)) {
            start = <a href="linked_table.md#0xc8_linked_table_borrow_node">borrow_node</a>&lt;K, V&gt;(<a href="../sui-framework/table.md#0x2_table">table</a>, *<a href="../move-stdlib/option.md#0x1_option_borrow">option::borrow</a>&lt;K&gt;(&next_key));
            <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> values, *<a href="linked_table.md#0xc8_linked_table_borrow_value">borrow_value</a>(start));
        };
        idx = idx + 1;
    };
    values
}
</code></pre>



</details>

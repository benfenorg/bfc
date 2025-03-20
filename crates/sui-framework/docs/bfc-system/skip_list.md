---
title: Module `0xc8::skip_list`
---



-  [Resource `SkipList`](#0xc8_skip_list_SkipList)
-  [Struct `Node`](#0xc8_skip_list_Node)
-  [Constants](#@Constants_0)
-  [Function `new`](#0xc8_skip_list_new)
-  [Function `length`](#0xc8_skip_list_length)
-  [Function `is_empty`](#0xc8_skip_list_is_empty)
-  [Function `head`](#0xc8_skip_list_head)
-  [Function `tail`](#0xc8_skip_list_tail)
-  [Function `destroy_empty`](#0xc8_skip_list_destroy_empty)
-  [Function `contains`](#0xc8_skip_list_contains)
-  [Function `borrow`](#0xc8_skip_list_borrow)
-  [Function `borrow_mut`](#0xc8_skip_list_borrow_mut)
-  [Function `borrow_node`](#0xc8_skip_list_borrow_node)
-  [Function `borrow_mut_node`](#0xc8_skip_list_borrow_mut_node)
-  [Function `metadata`](#0xc8_skip_list_metadata)
-  [Function `next_score`](#0xc8_skip_list_next_score)
-  [Function `prev_score`](#0xc8_skip_list_prev_score)
-  [Function `borrow_value`](#0xc8_skip_list_borrow_value)
-  [Function `borrow_mut_value`](#0xc8_skip_list_borrow_mut_value)
-  [Function `insert`](#0xc8_skip_list_insert)
-  [Function `remove`](#0xc8_skip_list_remove)
-  [Function `find_next`](#0xc8_skip_list_find_next)
-  [Function `find_prev`](#0xc8_skip_list_find_prev)
-  [Function `find`](#0xc8_skip_list_find)
-  [Function `rand_level`](#0xc8_skip_list_rand_level)
-  [Function `create_node`](#0xc8_skip_list_create_node)
-  [Function `drop_node`](#0xc8_skip_list_drop_node)


<pre><code><b>use</b> <a href="../move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../sui-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="../sui-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../sui-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../bfc-system/option_u64.md#0xc8_option_u64">0xc8::option_u64</a>;
<b>use</b> <a href="../bfc-system/random.md#0xc8_random">0xc8::random</a>;
</code></pre>



<a name="0xc8_skip_list_SkipList"></a>

## Resource `SkipList`

The skip list.


<pre><code><b>struct</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V: store&gt; <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../sui-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>
 The id of this skip list.
</dd>
<dt>
<code>head: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>&gt;</code>
</dt>
<dd>
 The skip list header of each level. i.e. the score of node.
</dd>
<dt>
<code>tail: <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a></code>
</dt>
<dd>
 The level0's tail of skip list. i.e. the score of node.
</dd>
<dt>
<code>level: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The current level of this skip list.
</dd>
<dt>
<code>max_level: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The max level of this skip list.
</dd>
<dt>
<code>list_p: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 Basic probability of random of node indexer's level i.e. (list_p = 2, level2 = 1/2, level3 = 1/4).
</dd>
<dt>
<code>size: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The size of skip list
</dd>
<dt>
<code><a href="../sui-framework/random.md#0x2_random">random</a>: <a href="../sui-framework/random.md#0x2_random_Random">random::Random</a></code>
</dt>
<dd>
 The random for generate ndoe's level
</dd>
</dl>


</details>

<a name="0xc8_skip_list_Node"></a>

## Struct `Node`

The node of skip list.


<pre><code><b>struct</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V: store&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a></code>
</dt>
<dd>
 The score of node.
</dd>
<dt>
<code>nexts: <a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>&gt;</code>
</dt>
<dd>
 The next node score of node's each level.
</dd>
<dt>
<code>prev: <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a></code>
</dt>
<dd>
 The prev node score of node.
</dd>
<dt>
<code>value: V</code>
</dt>
<dd>
 The data being stored
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0xc8_skip_list_ENodeAlreadyExist"></a>



<pre><code><b>const</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_ENodeAlreadyExist">ENodeAlreadyExist</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 0;
</code></pre>



<a name="0xc8_skip_list_ENodeDoesNotExist"></a>



<pre><code><b>const</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_ENodeDoesNotExist">ENodeDoesNotExist</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 1;
</code></pre>



<a name="0xc8_skip_list_ESkipListIsEmpty"></a>



<pre><code><b>const</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_ESkipListIsEmpty">ESkipListIsEmpty</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 4;
</code></pre>



<a name="0xc8_skip_list_ESkipListNotEmpty"></a>



<pre><code><b>const</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_ESkipListNotEmpty">ESkipListNotEmpty</a>: <a href="../move-stdlib/u64.md#0x1_u64">u64</a> = 3;
</code></pre>



<a name="0xc8_skip_list_new"></a>

## Function `new`

Create a new empty skip list.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_new">new</a>&lt;V: store&gt;(max_level: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, list_p: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, seed: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> <a href="../sui-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_new">new</a>&lt;V: store&gt;(max_level: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, list_p: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, seed: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, ctx: &<b>mut</b> TxContext): <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt; {
    <b>let</b> list = <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt; {
        id: <a href="../sui-framework/object.md#0x2_object_new">object::new</a>(ctx),
        head: <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>(),
        tail: none(),
        level: 0,
        max_level,
        list_p,
        <a href="../sui-framework/random.md#0x2_random">random</a>: random::new(seed),
        size: 0
    };
    list
}
</code></pre>



</details>

<a name="0xc8_skip_list_length"></a>

## Function `length`

Return the length of the skip list.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_length">length</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_length">length</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    list.size
}
</code></pre>



</details>

<a name="0xc8_skip_list_is_empty"></a>

## Function `is_empty`

Returns true if the skip list is empty (if <code>length</code> returns <code>0</code>)


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_is_empty">is_empty</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_is_empty">is_empty</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;): bool {
    list.size == 0
}
</code></pre>



</details>

<a name="0xc8_skip_list_head"></a>

## Function `head`

Return the head of the skip list.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_head">head</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_head">head</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;): OptionU64 {
    <b>if</b> (<a href="../bfc-system/skip_list.md#0xc8_skip_list_is_empty">is_empty</a>(list)) {
        <b>return</b> none()
    };
    *<a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&list.head, 0)
}
</code></pre>



</details>

<a name="0xc8_skip_list_tail"></a>

## Function `tail`

Return the tail of the skip list.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_tail">tail</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_tail">tail</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;): OptionU64 {
    list.tail
}
</code></pre>



</details>

<a name="0xc8_skip_list_destroy_empty"></a>

## Function `destroy_empty`

Destroys an empty skip list
Aborts with <code>ETableNotEmpty</code> if the list still contains values


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_destroy_empty">destroy_empty</a>&lt;V: drop, store&gt;(list: <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_destroy_empty">destroy_empty</a>&lt;V: store + drop&gt;(list: <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;) {
    <b>let</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt; {
        id,
        head: _,
        tail: _,
        level: _,
        max_level: _,
        list_p: _,
        <a href="../sui-framework/random.md#0x2_random">random</a>: _,
        size,
    } = list;
    <b>assert</b>!(size == 0, <a href="../bfc-system/skip_list.md#0xc8_skip_list_ESkipListNotEmpty">ESkipListNotEmpty</a>);
    <a href="../sui-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0xc8_skip_list_contains"></a>

## Function `contains`

Returns true if there is a value associated with the score <code>score</code> in skip list


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_contains">contains</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_contains">contains</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): bool {
    field::exists_with_type&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;&gt;(&list.id, score)
}
</code></pre>



</details>

<a name="0xc8_skip_list_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the <code>score</code> element of the skip list <code>list</code>.
Aborts if element not exist.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow">borrow</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow">borrow</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &V {
    &field::borrow&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;&gt;(&list.id, score).value
}
</code></pre>



</details>

<a name="0xc8_skip_list_borrow_mut"></a>

## Function `borrow_mut`

Return a mutable reference to the <code>score</code> element in the skip list <code>list</code>.
Aborts if element is not exist.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut">borrow_mut</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut">borrow_mut</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &<b>mut</b> V {
    &<b>mut</b> field::borrow_mut&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;&gt;(&<b>mut</b> list.id, score).value
}
</code></pre>



</details>

<a name="0xc8_skip_list_borrow_node"></a>

## Function `borrow_node`

Acquire an immutable reference to the <code>score</code> node of the skip list <code>list</code>.
Aborts if node not exist.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_node">borrow_node</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_node">borrow_node</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt; {
    field::borrow&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;&gt;(&list.id, score)
}
</code></pre>



</details>

<a name="0xc8_skip_list_borrow_mut_node"></a>

## Function `borrow_mut_node`

Return a mutable reference to the <code>score</code> node in the skip list <code>list</code>.
Aborts if node is not exist.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut_node">borrow_mut_node</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut_node">borrow_mut_node</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt; {
    field::borrow_mut&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;&gt;(&<b>mut</b> list.id, score)
}
</code></pre>



</details>

<a name="0xc8_skip_list_metadata"></a>

## Function `metadata`

Return the metadata info of skip list.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_metadata">metadata</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;): (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>&gt;, <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_metadata">metadata</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;): (<a href="../move-stdlib/vector.md#0x1_vector">vector</a>&lt;OptionU64&gt;, OptionU64, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../move-stdlib/u64.md#0x1_u64">u64</a>) {
    (
        list.head,
        list.tail,
        list.level,
        list.max_level,
        list.list_p,
        list.size
    )
}
</code></pre>



</details>

<a name="0xc8_skip_list_next_score"></a>

## Function `next_score`

Return the next score of the node.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_next_score">next_score</a>&lt;V: store&gt;(node: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_next_score">next_score</a>&lt;V: store&gt;(node: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;): OptionU64 {
    *<a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&node.nexts, 0)
}
</code></pre>



</details>

<a name="0xc8_skip_list_prev_score"></a>

## Function `prev_score`

Return the prev score of the node.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_prev_score">prev_score</a>&lt;V: store&gt;(node: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_prev_score">prev_score</a>&lt;V: store&gt;(node: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;): OptionU64 {
    node.prev
}
</code></pre>



</details>

<a name="0xc8_skip_list_borrow_value"></a>

## Function `borrow_value`

Return the immutable reference to the ndoe's value.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_value">borrow_value</a>&lt;V: store&gt;(node: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;): &V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_value">borrow_value</a>&lt;V: store&gt;(node: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;): &V {
    &node.value
}
</code></pre>



</details>

<a name="0xc8_skip_list_borrow_mut_value"></a>

## Function `borrow_mut_value`

Return the mutable reference to the ndoe's value.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut_value">borrow_mut_value</a>&lt;V: store&gt;(node: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;): &<b>mut</b> V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut_value">borrow_mut_value</a>&lt;V: store&gt;(node: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;): &<b>mut</b> V {
    &<b>mut</b> node.value
}
</code></pre>



</details>

<a name="0xc8_skip_list_insert"></a>

## Function `insert`

Insert a score-value into skip list, abort if the score alread exist.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_insert">insert</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, v: V)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_insert">insert</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, v: V) {
    <b>assert</b>!(!<a href="../bfc-system/skip_list.md#0xc8_skip_list_contains">contains</a>(list, score), <a href="../bfc-system/skip_list.md#0xc8_skip_list_ENodeAlreadyExist">ENodeAlreadyExist</a>);
    <b>let</b> (level, <b>mut</b> new_node) = <a href="../bfc-system/skip_list.md#0xc8_skip_list_create_node">create_node</a>(list, score, v);
    <b>let</b> (<b>mut</b> l, <b>mut</b> nexts, <b>mut</b> prev) = (list.level, &<b>mut</b> list.head, none());
    <b>let</b> <b>mut</b> opt_l0_next_score = none();
    <b>while</b> (l &gt; 0) {
        <b>let</b> <b>mut</b> opt_next_score = <a href="../move-stdlib/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(nexts, l - 1);
        <b>while</b> (is_some_and_lte(opt_next_score, score)) {
            <b>let</b> node =
                field::borrow_mut&lt;<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;&gt;(&<b>mut</b> list.id, <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(opt_next_score));
            prev = some(node.score);
            nexts = &<b>mut</b> node.nexts;
            opt_next_score = <a href="../move-stdlib/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(nexts, l - 1);
        };
        <b>if</b> (level &gt;= l) {
            <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> new_node.nexts, *opt_next_score);
            <b>if</b> (l == 1) {
                new_node.prev = prev;
                <b>if</b> (is_some(opt_next_score)) {
                    opt_l0_next_score = *opt_next_score;
                } <b>else</b> {
                    list.tail = some(score);
                }
            };
            swap_or_fill(opt_next_score, score);
        };
        l = l - 1;
    };
    <b>if</b> (is_some(&opt_l0_next_score)) {
        <b>let</b> next_node = <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut_node">borrow_mut_node</a>(list, <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(&opt_l0_next_score));
        next_node.prev = some(score);
    };

    <a href="../move-stdlib/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> new_node.nexts);
    field::add(&<b>mut</b> list.id, score, new_node);
    list.size = list.size + 1;
}
</code></pre>



</details>

<a name="0xc8_skip_list_remove"></a>

## Function `remove`

Remove the score-value from skip list, abort if the score not exist in list.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_remove">remove</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_remove">remove</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): V {
    <b>assert</b>!(<a href="../bfc-system/skip_list.md#0xc8_skip_list_contains">contains</a>(list, score), <a href="../bfc-system/skip_list.md#0xc8_skip_list_ENodeDoesNotExist">ENodeDoesNotExist</a>);
    <b>let</b> (<b>mut</b> l, <b>mut</b> nexts) = (list.level, &<b>mut</b> list.head);
    <b>let</b> node: <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt; = field::remove(&<b>mut</b> list.id, score);
    <b>while</b> (l &gt; 0) {
        <b>let</b> <b>mut</b> opt_next_score = <a href="../move-stdlib/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(nexts, l - 1);
        <b>while</b> (is_some_and_lte(opt_next_score, score)) {
            <b>let</b> next_score = <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(opt_next_score);
            <b>if</b> (next_score == score) {
                *opt_next_score = *<a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&node.nexts, l - 1);
            } <b>else</b> {
                <b>let</b> node = <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut_node">borrow_mut_node</a>(list, next_score);
                nexts = &<b>mut</b> node.nexts;
                opt_next_score = <a href="../move-stdlib/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(nexts, l - 1);
            }
        };
        l = l - 1;
    };

    <b>if</b> (<a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(&list.tail) == score) {
        list.tail = node.prev;
    };

    <b>let</b> opt_l0_next_score = <a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&node.nexts, 0);
    <b>if</b> (is_some(opt_l0_next_score)) {
        <b>let</b> next_node = <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_mut_node">borrow_mut_node</a>(list, <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(opt_l0_next_score));
        next_node.prev = node.prev;
    };
    list.size = list.size - 1;

    <a href="../bfc-system/skip_list.md#0xc8_skip_list_drop_node">drop_node</a>(node)
}
</code></pre>



</details>

<a name="0xc8_skip_list_find_next"></a>

## Function `find_next`

Return the next score.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_find_next">find_next</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <b>include</b>: bool): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_find_next">find_next</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <b>include</b>: bool): OptionU64 {
    <b>let</b> opt_finded_score = <a href="../bfc-system/skip_list.md#0xc8_skip_list_find">find</a>(list, score);
    <b>if</b> (is_none(&opt_finded_score)) {
        <b>return</b> opt_finded_score
    };
    <b>let</b> finded_score = <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(&opt_finded_score);
    <b>if</b> ((<b>include</b> && finded_score == score) || (finded_score &gt; score)) {
        <b>return</b> opt_finded_score
    };
    <b>let</b> node = <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_node">borrow_node</a>(list, finded_score);
    *<a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&node.nexts, 0)
}
</code></pre>



</details>

<a name="0xc8_skip_list_find_prev"></a>

## Function `find_prev`

Return the prev socre.


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_find_prev">find_prev</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <b>include</b>: bool): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_find_prev">find_prev</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <b>include</b>: bool): OptionU64 {
    <b>let</b> opt_finded_score = <a href="../bfc-system/skip_list.md#0xc8_skip_list_find">find</a>(list, score);
    <b>if</b> (is_none(&opt_finded_score)) {
        <b>return</b> opt_finded_score
    };
    <b>let</b> finded_score = <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(&opt_finded_score);
    <b>if</b> ((<b>include</b> && finded_score == score) || (finded_score &lt; score)) {
        <b>return</b> opt_finded_score
    };
    <b>let</b> node = <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_node">borrow_node</a>(list, finded_score);
    node.prev
}
</code></pre>



</details>

<a name="0xc8_skip_list_find"></a>

## Function `find`

Find the nearest score. 1. score, 2. prev, 3. next


<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_find">find</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../bfc-system/option_u64.md#0xc8_option_u64_OptionU64">option_u64::OptionU64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_find">find</a>&lt;V: store&gt;(list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): OptionU64 {
    <b>if</b> (list.size == 0) {
        <b>return</b> none()
    };
    <b>let</b> (<b>mut</b> l, <b>mut</b> nexts, <b>mut</b> current_score) = (list.level, &list.head, none());
    <b>while</b> (l &gt; 0) {
        <b>let</b> <b>mut</b> opt_next_score = *<a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(nexts, l - 1);
        <b>while</b> (is_some_and_lte(&opt_next_score, score)) {
            <b>let</b> next_score = <a href="../bfc-system/option_u64.md#0xc8_option_u64_borrow">option_u64::borrow</a>(&opt_next_score);
            <b>if</b> (next_score == score) {
                <b>return</b> some(next_score)
            } <b>else</b> {
                <b>let</b> node = <a href="../bfc-system/skip_list.md#0xc8_skip_list_borrow_node">borrow_node</a>(list, next_score);
                current_score = opt_next_score;
                nexts = &node.nexts;
                opt_next_score = *<a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(nexts, l - 1);
            };
        };
        <b>if</b> (l == 1 && is_some(&current_score)) {
            <b>return</b> current_score
        };
        l = l - 1;
    };
    <b>return</b> *<a href="../move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&list.head, 0)
}
</code></pre>



</details>

<a name="0xc8_skip_list_rand_level"></a>

## Function `rand_level`



<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_rand_level">rand_level</a>&lt;V: store&gt;(seed: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_rand_level">rand_level</a>&lt;V: store&gt;(seed: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, list: &<a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;): <a href="../move-stdlib/u64.md#0x1_u64">u64</a> {
    <b>let</b> <b>mut</b> level = 1;
    <b>let</b> <b>mut</b> mod = list.list_p;
    <b>while</b> ((seed % mod) == 0) {
        mod = mod * list.list_p;
        level = level + 1;
        <b>if</b> (level &gt; list.level) {
            <b>if</b> (level &gt;= list.max_level) {
                level = list.max_level;
                <b>break</b>
            } <b>else</b> {
                level = list.level + 1;
                <b>break</b>
            }
        }
    };
    level
}
</code></pre>



</details>

<a name="0xc8_skip_list_create_node"></a>

## Function `create_node`

Create a new skip list node


<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_create_node">create_node</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">skip_list::SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, value: V): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_create_node">create_node</a>&lt;V: store&gt;(list: &<b>mut</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_SkipList">SkipList</a>&lt;V&gt;, score: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, value: V): (<a href="../move-stdlib/u64.md#0x1_u64">u64</a>, <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;) {
    <b>let</b> rand = random::rand(&<b>mut</b> list.<a href="../sui-framework/random.md#0x2_random">random</a>);
    <b>let</b> level = <a href="../bfc-system/skip_list.md#0xc8_skip_list_rand_level">rand_level</a>(rand, list);

    // Create a new level for skip list.
    <b>if</b> (level &gt; list.level) {
        list.level = level;
        <a href="../move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> list.head, none());
    };

    (
        level,
        <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt; {
            score,
            nexts: <a href="../move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>(),
            prev: none(),
            value
        }
    )
}
</code></pre>



</details>

<a name="0xc8_skip_list_drop_node"></a>

## Function `drop_node`



<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_drop_node">drop_node</a>&lt;V: store&gt;(node: <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">skip_list::Node</a>&lt;V&gt;): V
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_drop_node">drop_node</a>&lt;V: store&gt;(node: <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a>&lt;V&gt;): V {
    <b>let</b> <a href="../bfc-system/skip_list.md#0xc8_skip_list_Node">Node</a> {
        score: _,
        nexts: _,
        prev: _,
        value,
    } = node;
    value
}
</code></pre>



</details>

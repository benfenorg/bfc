---
title: Module `0xc8::auth_utils`
---



-  [Function `has_mint_busd`](#0xc8_auth_utils_has_mint_busd)
-  [Function `has_mint_other_stablecoin`](#0xc8_auth_utils_has_mint_other_stablecoin)
-  [Function `substring`](#0xc8_auth_utils_substring)


<pre><code><b>use</b> <a href="../move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../move-stdlib/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a name="0xc8_auth_utils_has_mint_busd"></a>

## Function `has_mint_busd`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_has_mint_busd">has_mint_busd</a>(s: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_has_mint_busd">has_mint_busd</a>(s: &String): bool {
    <b>let</b> sub = <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_substring">substring</a>(*s, 0, 9);
    std::string::bytes(&sub) == b"MINT-BUSD"
}
</code></pre>



</details>

<a name="0xc8_auth_utils_has_mint_other_stablecoin"></a>

## Function `has_mint_other_stablecoin`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_has_mint_other_stablecoin">has_mint_other_stablecoin</a>(s: &<a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_has_mint_other_stablecoin">has_mint_other_stablecoin</a>(s: &String): bool {
    <b>let</b> sub = <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_substring">substring</a>(*s, 0, 21);
    std::string::bytes(&sub) == b"MINT-OTHER-STABLECOIN"
}
</code></pre>



</details>

<a name="0xc8_auth_utils_substring"></a>

## Function `substring`



<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_substring">substring</a>(s: <a href="../move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>, start: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, end: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): <a href="../move-stdlib/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bfc-system/auth_utils.md#0xc8_auth_utils_substring">substring</a>(s: String, start: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>, end: <a href="../move-stdlib/u64.md#0x1_u64">u64</a>): std::string::String {
    <b>let</b> str_std = std::string::from_ascii(s);
    <b>if</b> (start &gt;= end || end &gt; std::string::length(&str_std)) {
        <b>return</b> std::string::from_ascii(std::ascii::string(b""))
    };
    std::string::sub_string(&str_std, start, end)
}
</code></pre>



</details>

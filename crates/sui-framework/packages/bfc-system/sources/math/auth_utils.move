module bfc_system::auth_utils {

    use std::ascii::String;

    
    public fun has_mint_usdt_usdc(s: &String): bool {
        let sub = substring(*s, 0, 14);
        std::string::bytes(&sub) == b"MINT-USDT-USDC"
    }

    public fun has_mint_other_stablecoin(s: &String): bool {
        let sub = substring(*s, 0, 21);
        std::string::bytes(&sub) == b"MINT-OTHER-STABLECOIN"
    }

    #[test]
    fun test_has_mint_usdt_usdc() {
        let s = std::ascii::string(b"MINT-USDT-USDC-TEST");
        assert!(has_mint_usdt_usdc(&s), 0);

        let s2 = std::ascii::string(b"MINT-OTHER-TEST");
        assert!(!has_mint_usdt_usdc(&s2), 1);

        let s3 = std::ascii::string(b"");
        assert!(!has_mint_usdt_usdc(&s3), 2);
    }

    #[test]
    fun test_has_mint_other_stablecoin() {
        let s = std::ascii::string(b"MINT-OTHER-STABLECOIN-TEST");
        assert!(has_mint_other_stablecoin(&s), 0);

        let s2 = std::ascii::string(b"MINT-USDT-TEST");
        assert!(!has_mint_other_stablecoin(&s2), 1);

        let s3 = std::ascii::string(b"");
        assert!(!has_mint_other_stablecoin(&s3), 2);
    }
    
    /// 提取 ASCII 字符串的子串
    /// 参数：
    /// - s: 输入的 ASCII 字符串
    /// - start: 子串的起始位置（包含）
    /// - end: 子串的结束位置（不包含）
    /// 返回：
    /// - 子串，如果起始位置或结束位置无效，则返回错误
    public fun substring(s: String, start: u64, end: u64): std::string::String {
        let str_std = std::string::from_ascii(s);
        if (start >= end || end > std::string::length(&str_std)) {
            return std::string::from_ascii(std::ascii::string(b""))
        };
        std::string::sub_string(&str_std, start, end)
    }

    #[test]
    fun test_substring() {
        let s = std::ascii::string(b"MINT-USDT-USDC-POLY");
        let sub = substring(s, 0, 19);
        std::debug::print(&sub);
        assert!(std::string::bytes(&sub) == &b"MINT-USDT-USDC-POLY", 0);

        let sub2 = substring(s, 0, 14); 
        assert!(std::string::bytes(&sub2) == &b"MINT-USDT-USDC", 1);

        let sub3 = substring(s, 15, 19);
        assert!(std::string::bytes(&sub3) == &b"POLY", 2);

        let sub4 = substring(s, 0, 0);
        assert!(std::string::bytes(&sub4) == &b"", 3);

        let sub5 = substring(s, 0, 20);
        assert!(std::string::bytes(&sub5) == &b"", 4);
    }
}
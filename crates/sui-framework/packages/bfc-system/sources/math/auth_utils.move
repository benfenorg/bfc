#[allow(deprecated_usage)]
module bfc_system::auth_utils {

    use std::ascii::String;

    
    public fun has_mint_busd(s: &String): bool {
        let sub = substring(*s, 0, 9);
        std::string::bytes(&sub) == b"MINT-BUSD"
    }

    public fun has_mint_other_stablecoin(s: &String): bool {
        let sub = substring(*s, 0, 21);
        std::string::bytes(&sub) == b"MINT-OTHER-STABLECOIN"
    }

    #[test]
    fun test_has_mint_busd() {
        let s = std::ascii::string(b"MINT-BUSD-TEST");
        assert!(has_mint_busd(&s), 0);

        let s2 = std::ascii::string(b"MINT-OTHER-TEST");
        assert!(!has_mint_busd(&s2), 1);

        let s3 = std::ascii::string(b"");
        assert!(!has_mint_busd(&s3), 2);
    }

    #[test]
    fun test_has_mint_other_stablecoin() {
        let s = std::ascii::string(b"MINT-OTHER-STABLECOIN-TEST");
        assert!(has_mint_other_stablecoin(&s), 0);

        let s2 = std::ascii::string(b"MINT-BUSD-TEST");
        assert!(!has_mint_other_stablecoin(&s2), 1);

        let s3 = std::ascii::string(b"");
        assert!(!has_mint_other_stablecoin(&s3), 2);
    }
    
    
    public fun substring(s: String, start: u64, end: u64): std::string::String {
        let str_std = std::string::from_ascii(s);
        if (start >= end || end > std::string::length(&str_std)) {
            return std::string::from_ascii(std::ascii::string(b""))
        };
        std::string::sub_string(&str_std, start, end)
    }

    #[test]
    fun test_substring() {
        let s = std::ascii::string(b"MINT-BUSD-POLY");
        let sub = substring(s, 0, 14);
        std::debug::print(&sub);
        assert!(std::string::bytes(&sub) == &b"MINT-BUSD-POLY", 0);

        let sub2 = substring(s, 0, 9);
        assert!(std::string::bytes(&sub2) == &b"MINT-BUSD", 1);

        let sub3 = substring(s, 10, 14);
        assert!(std::string::bytes(&sub3) == &b"POLY", 2);

        let sub4 = substring(s, 0, 0);
        assert!(std::string::bytes(&sub4) == &b"", 3);

        let sub5 = substring(s, 0, 20);
        assert!(std::string::bytes(&sub5) == &b"", 4);
    }
}
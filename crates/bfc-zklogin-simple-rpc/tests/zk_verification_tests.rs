// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod tests {
    use fastcrypto::encoding::{Base64, Encoding};

    use serial_test::serial;
    use bfc_zklogin_simple_rpc::zk_verification::verify_zk_login_sig;

    #[tokio::test]
    #[serial]
    async fn test_base64_encoding() {
        let encode: String = Base64::encode("BFC0f2c739c38c9a580145e335fa93b243ad6ac5a086a72db9da2d69daa1423ce943962:DhCxzCVEhcj7z9oeHKl7pLDZT0nd1uTu1C46A3MzFcA-6X-PnJHgmFXAFg5M4ZFV");
        println!("encode={}", encode);
    }

    #[tokio::test]
    async fn test_zk_login_sig_personal_message_verify() {
        let author = "0x57740ee44a54bd8cdfa247456c61d1765db39283c6f927c73aa0dcc620a9d40b";
        let cur_epoch= Some(64u64);
        let intent_scope = 3u8;
        let bytes = "aGVsbG86MHhjMGYwZDBlMmEyY2E4YjhkMGU0MDU1ZWM0ODIxMGVjNzdkMDU1ZGIzNTM0MDJjZGEwMWQ3MDg1YmE2MWQzZDVj";
        let sig = "BQNMNjI4OTcxMjM2MjIzMjQwNDQ2NDAyNzU4NzgxNjgyNjg0MTYyMzk0Nzg1MzU5MDE1Mzk2Njc1ODU0OTk5NDQxMjM3ODYyMzM4NzYwM0w2OTU4ODMwOTY4NTkwMTAyOTEzOTUwNjIyNDAxMTcxMzY5MjE1MTI3Njc0NDE1NDgyNjA0OTE0MjI2NjI2MjA2MzU1NjMxOTY0NTQxATEDAk0xMTAyODQ5Nzk2NDg4MTE1NzkyNTY1OTk4NjQ0NjYwMDMxODUzMzUyMjQ1NTk1OTI3Mzc3Njc2MjA0MTQ3NDA0Mjg2ODI3NTYyNDU1M00xOTg2MjMxMjE1MjI3Nzk0Mzc1ODc2Mjg1NzA0OTczNzE0MTM2MDY4NTU0NDk3NzE4ODg1Njg5OTE4NjcwOTg3ODQ4NDYwNDM1NjA2MgJMNTg3NTkwNjM1NTcwMzY1NzQ4NjU0OTExNzAzMDIyNTc3MDQ5MTEyNTU0MDI0NTU0Njk1Mzg3ODM2NjQ0MTM1NDQyMDU4NzY1MjYwMUw2NTg1MjEwNjY4NzI1ODQyMjI1MjcyNDk1MjY0MjAxMDEyMDA1ODcxODAyNTg4NDA0NjI1MzY1OTYxNTg5NzkzMjk0MTE5NjczMjk4AgExATADTTExMjU2MTI5NDUyOTc5MzA4NjQ2MjcyMTI2NzQ2NTIyNzg1NTA3NDUzMDkxNTk2Nzk0Mzk0MzMwMDcyMjE0NzgwNDAyMDcyMzk5NDU1TDc0NzI0ODI4MzE4NzU1Mjk2NDAxMDc3MTc1MTgxNzgxNTE2OTM0MTQwNDY3NjYxNjg2Mzk4Mzg5NjU2OTYwODE4NDM5MDI3NjcyNDMBMSh3aWFYTnpJam9pYUhSMGNITTZMeTl2WVhWMGFDNXpkV2t1YVc4aUxDAj5leUpyYVdRaU9pSnpkV2t0YTJWNUxXbGtJaXdpZEhsd0lqb2lTbGRVSWl3aVlXeG5Jam9pVWxNeU5UWWlmUUs4NDc1MjUwMDIwMTEyODA1OTE3NDgxMDUyNjczOTA3ODc2MDgxNzM5NjE4ODczNDI5OTIxODY2NjcyOTE2Mjg1MTEyODIwMTk4MTlAAAAAAAAAAGEADQcJ1OI6iOgaUN6+FbvqtzldzrTvS9lOujvyTyLjzoV9HcUQw6efh0hRT6Y3T4N3Wdm+eFILoBKAFjBmnkRvDrnG7hYw7z5xEUSmSNsGu7IoT3J0z77lP/zuUDzBpJIA";
        let result = verify_zk_login_sig(sig.to_string(), bytes.to_string(), intent_scope, cur_epoch, None, author.to_string(), "test".to_string()).await;
        println!("result={:?}", &result);
        assert_eq!(result.is_ok(), true);
        let sui_result = result.unwrap();
        assert_eq!(sui_result.is_ok(), true);
        println!("sui_result={:?}", sui_result.unwrap())
    }

    #[tokio::test]
    async fn test_zk_login_sig_personal_message_verify_fail() {
        let author = "BFC0f2c739c38c9a580145e335fa93b243ad6ac5a086a72db9da2d69daa1423ce943962";
        let cur_epoch= Some(300u64);
        let intent_scope = 3u8;
        let bytes = "QkZDMGYyYzczOWMzOGM5YTU4MDE0NWUzMzVmYTkzYjI0M2FkNmFjNWEwODZhNzJkYjlkYTJkNjlkYWExNDIzY2U5NDM5NjI6RGhDeHpDVkVoY2o3ejlvZUhLbDdwTERaVDBuZDF1VHUxQzQ2QTNNekZjQS02WC1QbkpIZ21GWEFGZzVNNFpGVg==";
        let sig = "EWZ2wzshZUICy/IeTuQCQeqgKnlexlvGBbB/+iBlBlYHpLT6r0kZC2mTyrDpdtIpgXaTwojxmcWSfykKNGBUGglFHTdqnsXHJiVoVz";
        let result = verify_zk_login_sig(sig.to_string(), bytes.to_string(), intent_scope, cur_epoch, None, author.to_string(), "test".to_string()).await;
        println!("result={:?}", &result);
        assert_eq!(result.is_err(), true);
    }

}

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

mod tests {
    use fastcrypto::encoding::{Base64, Encoding};

    use serial_test::serial;
    use sui_simple_rpc::zk_verification::verify_zk_login_sig;

    #[tokio::test]
    #[serial]
    async fn test_base64_encoding() {
        let encode: String = Base64::encode("BFC0f2c739c38c9a580145e335fa93b243ad6ac5a086a72db9da2d69daa1423ce943962:DhCxzCVEhcj7z9oeHKl7pLDZT0nd1uTu1C46A3MzFcA-6X-PnJHgmFXAFg5M4ZFV");
        println!("encode={}", encode);
    }

    #[tokio::test]
    // #[serial]
    async fn test_zk_login_sig_personal_message_verify() {
        let author = "BFC0f2c739c38c9a580145e335fa93b243ad6ac5a086a72db9da2d69daa1423ce943962";
        let cur_epoch= 300u64;
        let intent_scope = 3u8;
        let bytes = "QkZDMGYyYzczOWMzOGM5YTU4MDE0NWUzMzVmYTkzYjI0M2FkNmFjNWEwODZhNzJkYjlkYTJkNjlkYWExNDIzY2U5NDM5NjI6RGhDeHpDVkVoY2o3ejlvZUhLbDdwTERaVDBuZDF1VHUxQzQ2QTNNekZjQS02WC1QbkpIZ21GWEFGZzVNNFpGVg==";
        let sig = "BQNNMTI3MzMyODExMDcxMTcxNzg2MDU0ODA5NjAwNTk2NzgzMDA2MzE4ODI3MTkyNTAzMDkwODQzOTA5ODczMzc4OTUyNTA4NTAyNjA2MDZNMTQxMjEyNzg2NjQ2Mzk2MjM1NDU0MjE5NjA3NTY1MDgwNjQ1NTAzMzgyMDk5NDI5NjI3NjczMjI2MjQwMzg5NjE2NTQwMjk5MjgwMTMBMQMCTDUyNjU4MjAwMDEzNjQ3MTIyNjcyOTQ0MjQyMDU2MzIzNTMzOTMzOTU2MDEwNzM5MzI4MjY4MzkxMDQwNzUwMjE5Mzg4MjUxMDk0NzRNMTgwMDgzMTg1MjQ5MjU4MzAxMTc4NjU1ODcxMTQ0MDY3NzQzNzQyOTg0ODAzMzc0NDg1NDA2NTc3NDc2NTc4MjI2ODIwMjY4ODM3NzYCTTExMzc3MTA0NDkzMjU2MTkxODk3OTMzOTk3ODc2NzMzOTM4Mzk0NDYwNTc4MzgwODI4MjQzNjE2NTg4MTkzODc4Njc3NzExNzA0NDUwTTIxMjU3OTA3OTI4Njk4MzM3NjEzMTAwNzU2OTc4MjM3Nzg2ODE1Njc4ODA2NjM2NjI0MjE5MTk5Mjk3NDQ0MzgxMjQ5MzU1NTIwOTU2AgExATADTTEwNDcxNzQwMjUwMjE2OTM3MTcxMTI4MDc4MzEwODIyNTY4MjA0NjU3MjE0MTA3NjkzNzE5NjgyMTIwNzE5NTUwMTI0MTk4NjQ3NDM0TDQ4NjY4ODU5NjYzMTk3NDQ3Mjc1Njc4NjYxNzAwODkyMTY0ODE3NzUyMDY4NzE4NjIzNTk0MjI5MDU5MTIzNjg0Mjk5NTI2ODk3MzQBMTF5SnBjM01pT2lKb2RIUndjem92TDJGalkyOTFiblJ6TG1kdmIyZHNaUzVqYjIwaUxDAWZleUpoYkdjaU9pSlNVekkxTmlJc0ltdHBaQ0k2SWpnM1ltSmxNRGd4TldJd05qUmxObVEwTkRsallXTTVPVGxtTUdVMU1HVTNNbUV6WlRRek56UWlMQ0owZVhBaU9pSktWMVFpZlFNMjAxNDY5NDcxODM1NTMxMTM4MjY5ODIwNjQxNDAzODI5MzY4NzM4MDMyNTgyODI1OTM3ODU1ODkxMDU2NDY2MTQ0MjEwNTM5MjQxNDcsAQAAAAAAAGEAG2XQ5rYhbo5NvG8tMDcuhIDKr/EWZ2wzshZUICy/IeTuQCQeqgKnlexlvGBbB/+iBlBlYHpLT6r0kZC2mTyrDpdtIpgXaTwojxmcWSfykKNGBUGglFHTdqnsXHJiVoVz";
        let result = verify_zk_login_sig(sig.to_string(), bytes.to_string(), intent_scope, cur_epoch, author.to_string()).await;
        assert_eq!(result.is_ok(), true);
        let sui_result = result.unwrap();
        assert_eq!(sui_result.is_ok(), true);
        println!("sui_result={:?}", sui_result.unwrap())
    }

    #[tokio::test]
    async fn test_zk_login_sig_personal_message_verify_fail() {
        let author = "BFC0f2c739c38c9a580145e335fa93b243ad6ac5a086a72db9da2d69daa1423ce943962";
        let cur_epoch= 300u64;
        let intent_scope = 3u8;
        let bytes = "QkZDMGYyYzczOWMzOGM5YTU4MDE0NWUzMzVmYTkzYjI0M2FkNmFjNWEwODZhNzJkYjlkYTJkNjlkYWExNDIzY2U5NDM5NjI6RGhDeHpDVkVoY2o3ejlvZUhLbDdwTERaVDBuZDF1VHUxQzQ2QTNNekZjQS02WC1QbkpIZ21GWEFGZzVNNFpGVg==";
        let sig = "EWZ2wzshZUICy/IeTuQCQeqgKnlexlvGBbB/+iBlBlYHpLT6r0kZC2mTyrDpdtIpgXaTwojxmcWSfykKNGBUGglFHTdqnsXHJiVoVz";
        let result = verify_zk_login_sig(sig.to_string(), bytes.to_string(), intent_scope, cur_epoch, author.to_string()).await;
        println!("result={:?}", &result);
        assert_eq!(result.is_err(), true);
    }


}

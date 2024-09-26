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
        let sig = "BQNMNjU3NDQxNDQwOTE1MTgxNDcyOTAxMjExNzA3NDc0NzczMzY1NDUyMDE1MjY3MjkwNzk3NjY4MjA4NTQyMjIwOTgzMjY3ODQyODE1NEw5NTM2OTk5NDQzOTc5OTUwNTkwMjkyODUwMjY0MDM1OTY1MDAxODAyNjkyOTk0ODkxNzQyNDg0NzU5NDA1MDMyNzU4NzcwMzM2MDcxATEDAk0xNzYzNjE1NTk3MTg0MDM2MzUxNTE4ODMxMjAwNjExMjI3NjU3OTg4NTU4NTc3OTkzNTk2NjYzOTMxMDMxNzk0MDkzNDA2NDgwNDQwN00xMTE5MzU2NDE4ODQ4NzM5MTY2Mjc4OTAwMDQ3NjA2NzM5NDYxNDk5NjIzMjEyNTE4MDA4MzUzNTY3NTU1MDExNDQxNzM5MTg5MjA0NwJMNjc4NDU4NjIxNDk2NTkyNTE5MjU5MjEyMDkwNzEwMTE1MzkyMDAzMDI2OTYyOTEzMzYwMTA4NTcyNjE4NDQ5MzU3MjEzODcyMTk2NU0xNDkyOTAyMjE3Mzg4MTkyMDQ3MTEzNDg1OTc5NTYyNTY1ODI2NTE2MjI2MDg4MjM2NDMxMjY5NjkzMzcyMjI1MDExMDcxNjM2MzA2NQIBMQEwA0wzMzE1ODk5NjA1MDQ0MzAxOTc1NDEzMTQ1MjY3MDU4NTI2MjY0MDkzMjg3OTc2NDYwMTAxMTQ4OTY0MTIxNDA4MTQ3NDMxOTQ2MjIxTTE2OTA0OTgxNDQyMDgwMDI4OTk5NzkzMDYyMDAwMDc5NjEwMzA3MjkyNTEwMzI0MDcyMzg5Njc4MzUwMzQ0Mjk5MzI3MjQ0MTU1MDE1ATEod2lhWE56SWpvaWFIUjBjSE02THk5dllYVjBhQzV6ZFdrdWFXOGlMQwI+ZXlKcmFXUWlPaUp6ZFdrdGEyVjVMV2xrSWl3aWRIbHdJam9pU2xkVUlpd2lZV3huSWpvaVVsTXlOVFlpZlFLODQ3NTI1MDAyMDExMjgwNTkxNzQ4MTA1MjY3MzkwNzg3NjA4MTczOTYxODg3MzQyOTkyMTg2NjY3MjkxNjI4NTExMjgyMDE5ODE5QAAAAAAAAABhAA0HCdTiOojoGlDevhW76rc5Xc6070vZTro78k8i486FfR3FEMOnn4dIUU+mN0+Dd1nZvnhSC6ASgBYwZp5Ebw65xu4WMO8+cRFEpkjbBruyKE9ydM++5T/87lA8waSSAA==";
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

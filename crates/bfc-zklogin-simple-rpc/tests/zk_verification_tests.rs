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
    // #[serial]
    async fn test_zk_login_sig_personal_message_verify() {
        let author = "0xc0f0d0e2a2ca8b8d0e4055ec48210ec77d055db353402cda01d7085ba61d3d5c";
        let cur_epoch= Some(122u64);
        let intent_scope = 3u8;
        let bytes = "QkZDMGYyYzczOWMzOGM5YTU4MDE0NWUzMzVmYTkzYjI0M2FkNmFjNWEwODZhNzJkYjlkYTJkNjlkYWExNDIzY2U5NDM5NjI6RGhDeHpDVkVoY2o3ejlvZUhLbDdwTERaVDBuZDF1VHUxQzQ2QTNNekZjQS02WC1QbkpIZ21GWEFGZzVNNFpGVg==";
        let sig = "BQNNMTA2MjcwNzgyMjE1NzkzNDg0MjI0MTczNzc5MzgxNzAwMDE3NjcyNzQyOTA3NTAyMzY4NjA5ODA1ODIxMjExODgzODA5MjAyNDE3MDlNMTk3ODA4NzQ1NDkzNTMwNzgzNDQ5NTYwODEyMDc2NTcwODE4MTM4NjY2OTI4MDU2MDE4MjA3MDc5ODcxODA1MDM2ODgyMDI2MzgxODEBMQMCTDI5ODMzOTYzNDg5Njk2NDEwNDA4NDMxOTI5NTE0Mzc3MDQ3OTM0OTc5NTc3MDMxNzUyOTY1NDU1MTM3NDU4NjA4MTIzNzUzNTM2MjVMMzk5NDkxMTYyMzI4MTM2MzE5MjQ2NjQ0NTM5MTg5NzYzNTgyNTI2MDQyMTI0MDAwNDk1MTcyNDkzNTUxOTc1NDY5MDEzNDI5NzIyOQJLNjMwNDA3NjQ4Mjk0MzUzMDkxNjI1MTU1MzIzOTIzNzYxOTQ5OTA5NDMyMzA5Njk5Njc2MjQ4Njk1MTEzODcyOTI4NTE3MzUwODE3SzY0MDk4NjE2NzUzNzU4ODQ1OTQ1NjQyMDg2OTUxNjI3MjgyODk4MTI4OTg1Mzc2MTY1NTEyNjA2OTc2NzI2MDYzMDc2NjI4NjIxMgIBMQEwA0wyMDU3NTgxODg2OTM4ODE1NjI0NjkwNDQxNDY5NTYzNDIxNDM2NzY2NDY3NTk5MDE1NTQ1ODQ1NDQwOTY3Mjc0MjcwMzgyNzU2MDIySzY3Mjc4MjY3ODg3NTI0MjQzNTE2NTA1NzQ2NjQ2MzY4MjU2NjM4MjUxNTQwNjU2NzY4MDQzNzI0NzM3NTQ0NzE2ODU4ODI3MTcxMQExKHdpYVhOeklqb2lhSFIwY0hNNkx5OXZZWFYwYUM1emRXa3VhVzhpTEMCPmV5SnJhV1FpT2lKemRXa3RhMlY1TFdsa0lpd2lkSGx3SWpvaVNsZFVJaXdpWVd4bklqb2lVbE15TlRZaWZRTTIwNDM1MzY2NjAwMDM2Mzc1NzQ1OTI1OTYzNDU2ODYxMzA3OTI1MjA5NDcwMjE5MzM0MDE4NTY0MTU4MTQ4NTQ0MDM2MTk2Mjg0NjQyegAAAAAAAABhANvXECC64Dj48lJAmNGFpCoVUJYwYIvlCmdQ1OdPtJv7T9H7KY2VLKr+HQT2eOit6jOt1S9CccdLKZpWg7I9wga5xu4WMO8+cRFEpkjbBruyKE9ydM++5T/87lA8waSSAA==";
        let result = verify_zk_login_sig(sig.to_string(), bytes.to_string(), intent_scope, cur_epoch, None, author.to_string()).await;
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
        let result = verify_zk_login_sig(sig.to_string(), bytes.to_string(), intent_scope, cur_epoch, None, author.to_string()).await;
        println!("result={:?}", &result);
        assert_eq!(result.is_err(), true);
    }

    #[tokio::test]
    // #[serial]
    async fn test_zk_login_sig_personal_message_verify_with_url() {
        let author = "0xc0f0d0e2a2ca8b8d0e4055ec48210ec77d055db353402cda01d7085ba61d3d5c";
        let cur_rpc_url = Some("https://devrpc.openblock.vip".to_string());
        let intent_scope = 3u8;
        let bytes = "QkZDMGYyYzczOWMzOGM5YTU4MDE0NWUzMzVmYTkzYjI0M2FkNmFjNWEwODZhNzJkYjlkYTJkNjlkYWExNDIzY2U5NDM5NjI6RGhDeHpDVkVoY2o3ejlvZUhLbDdwTERaVDBuZDF1VHUxQzQ2QTNNekZjQS02WC1QbkpIZ21GWEFGZzVNNFpGVg==";
        let sig = "BQNNMTQ1MzE5NTM0NzI4NzIzMTc3Nzc4NTYxODQyMzc2ODk1MjQzODc0NjIyMDc2Nzg3Mzk2MTA3OTcxMDMzMzg4MzYzMzQ5NDAyODkyMzhNMTkxMTQ2NDQxOTczMjE2NTExNzY3NTk3MDA4MDM1MzMyMzg5ODA4ODgzMzAxMzIzMzQ4ODkzOTU0Mzg1NTU1Mjc1Mzc0NjAyNTc5NzcBMQMCTTEwNTc4MjQ3MzU3MTY0OTkyMDExNTkwMDE1MjgzMDczMzM2OTI5OTM2MTk4NjIzOTM4MzYxNjExMDMzMDkyMTk4ODQ1MjExMDU3NDUyTTE4ODUzMjQzMjE0ODc1MzAyMzUyNDQxMzExNTczMjg1MjMyODUzNTIwNjk1OTkyNzA1NjM4NTcyNTIwMTI3ODgyMTE0ODI1MTgxNTk4Ak0xNjk0NTU0NDYyODU3OTE0MzQwNzA4MDIxODMyOTIwMDY5NTg2MjQxODczMTI0NzkxNzc2ODYyMTQ4OTE2NDE5MjE4MzM2ODE1MjMwNUw2MTgyODQ4NDQ0NzMxOTA5NTE0OTYwMDgyNDU1NTkzNDcwNTYyMjI1ODg0ODM3MjMxMzA2NDQ1NDUwNDYwMzIzNjY2ODUwNzg5NDI5AgExATADTDU4MjI5MzIwNjA2MDMwNTc1NTE2MzUzODMyNzcxMzI5NzQ3ODgxMzgzMDEzOTU2NDY1NTMzMDM2NDgyNzEyNjgxNzU2NTgwNzE2NzZMODM2ODM2NjY3NzI2OTc3Nzg2NTUzMzA0OTA0MDE1NTQzNTM0MDY3NDY0NjA3NjIzMjk3NDg4MDA0MjU5NDM0NDQ3OTY4NzI0NzQwMAExKHdpYVhOeklqb2lhSFIwY0hNNkx5OXZZWFYwYUM1emRXa3VhVzhpTEMCPmV5SnJhV1FpT2lKemRXa3RhMlY1TFdsa0lpd2lkSGx3SWpvaVNsZFVJaXdpWVd4bklqb2lVbE15TlRZaWZRTTIwNDM1MzY2NjAwMDM2Mzc1NzQ1OTI1OTYzNDU2ODYxMzA3OTI1MjA5NDcwMjE5MzM0MDE4NTY0MTU4MTQ4NTQ0MDM2MTk2Mjg0NjQyMHUAAAAAAABhANvXECC64Dj48lJAmNGFpCoVUJYwYIvlCmdQ1OdPtJv7T9H7KY2VLKr+HQT2eOit6jOt1S9CccdLKZpWg7I9wga5xu4WMO8+cRFEpkjbBruyKE9ydM++5T/87lA8waSSAA==";
        let result = verify_zk_login_sig(sig.to_string(), bytes.to_string(), intent_scope, None, cur_rpc_url, author.to_string()).await;
        println!("result={:?}", &result);
        assert_eq!(result.is_ok(), true);
        let sui_result = result.unwrap();
        assert_eq!(sui_result.is_ok(), true);
        println!("sui_result={:?}", sui_result.unwrap())
    }


}

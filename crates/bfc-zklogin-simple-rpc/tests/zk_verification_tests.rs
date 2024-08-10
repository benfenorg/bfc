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
        let author = "0xc0f0d0e2a2ca8b8d0e4055ec48210ec77d055db353402cda01d7085ba61d3d5c";
        let cur_epoch= Some(5u64);
        let intent_scope = 3u8;
        let bytes = "QkZDMGYyYzczOWMzOGM5YTU4MDE0NWUzMzVmYTkzYjI0M2FkNmFjNWEwODZhNzJkYjlkYTJkNjlkYWExNDIzY2U5NDM5NjI6RGhDeHpDVkVoY2o3ejlvZUhLbDdwTERaVDBuZDF1VHUxQzQ2QTNNekZjQS02WC1QbkpIZ21GWEFGZzVNNFpGVg==";
        let sig = "BQNMOTY3NjQ5ODY5NjU1MTE2NTE1MjMzMjYyNDA3MjMxNjExNDgwMTQyNDg4NjE3MzYxNjc5MTIxOTc3MTM3Nzk3NzIyMTcwNzYwODkzN00yMTYyOTIzMTc3NjE5MjA5NDk0NTYyNzU0ODA2ODUyOTMxNDcxNzUyNzU4NzAyNzQ1ODUxOTI3MDY5NDkwODI5OTQxNDA5NzI3MTM3OQExAwJNMTc4ODk4MjQwNTk4NzExMzQ1Nzg2NzMzMTk1NzQyMjcxMDg4Njk5ODI0NjQ5NTk1ODU2OTQ4OTg0NDA0NzI3NDA0MzA0NTQ0MzMwOTlMNjI1OTMxNTQ4NTg5MDU2NTUwMTMwOTI0ODIwNTkxMzY0NzY4NTk4MTYyMzMyNjIyNzMwNTgyODQ5Mjg0NTYzMzA1OTg5MDQyOTYwNwJNMTQyOTYxMzc3MzE1OTE3NTY5NTk3MTE5MTUzNDU0MjAzMDU5MjgyMzU4MTg3NDA5NTIwMjMxOTMyODA1MzA0NzI5NDUxNTY5NjA1MTdMMTg1MTEwNDQyNTY4NDg0Mjk2OTc1MTI3MzExNDI5NzQ5NDA0MDExMDUyMzg3MTIyNzUzMTUxNjI4MzAwNDg3OTY4NDU2NjY0NzQ4NAIBMQEwA00yMDI0MTM3ODc4MTYzODgzMDE0OTUwNzE0OTUzNzI3MzczMjM4MTkyNjA1MzI1ODgyMTA4OTk4NDcxMjcyNDUyMjE3NDIzODQ4MzIwMkw4MzYyMjU2NzM5MDE1NTc1OTExNzAxOTcyNDkwMDY1ODU4NzIyODExNTA0NTgzMTM3MDkyMjI0MTg2NjI3MDk0ODk1OTAxNDYzMDM0ATEod2lhWE56SWpvaWFIUjBjSE02THk5dllYVjBhQzV6ZFdrdWFXOGlMQwI+ZXlKcmFXUWlPaUp6ZFdrdGEyVjVMV2xrSWl3aWRIbHdJam9pU2xkVUlpd2lZV3huSWpvaVVsTXlOVFlpZlFNMjA0MzUzNjY2MDAwMzYzNzU3NDU5MjU5NjM0NTY4NjEzMDc5MjUyMDk0NzAyMTkzMzQwMTg1NjQxNTgxNDg1NDQwMzYxOTYyODQ2NDIFAAAAAAAAAGEA29cQILrgOPjyUkCY0YWkKhVQljBgi+UKZ1DU50+0m/tP0fspjZUsqv4dBPZ46K3qM63VL0Jxx0spmlaDsj3CBrnG7hYw7z5xEUSmSNsGu7IoT3J0z77lP/zuUDzBpJIA";
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

}

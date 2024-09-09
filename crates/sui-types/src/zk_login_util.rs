// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto_zkp::bn254::zk_login::ZkLoginInputs;

// Used in tests or anywhere that fetching up to date JWKs is not possible. This is an example response from https://id.twitch.tv/oauth2/keys
pub const DEFAULT_JWK_BYTES: &[u8] = r#"{"keys":[{"alg":"RS256","e":"AQAB","kid":"1","kty":"RSA","n":"6lq9MQ-q6hcxr7kOUp-tHlHtdcDsVLwVIw13iXUCvuDOeCi0VSuxCCUY6UmMjy53dX00ih2E4Y4UvlrmmurK0eG26b-HMNNAvCGsVXHU3RcRhVoHDaOwHwU72j7bpHn9XbP3Q3jebX6KIfNbei2MiR0Wyb8RZHE-aZhRYO8_-k9G2GycTpvc-2GBsP8VHLUKKfAs2B6sW3q3ymU6M0L-cFXkZ9fHkn9ejs-sqZPhMJxtBPBxoUIUQFTgv4VXTSv914f_YkNw-EjuwbgwXMvpyr06EyfImxHoxsZkFYB-qBYHtaMxTnFsZBr6fn8Ha2JqT1hoP7Z5r5wxDu3GQhKkHw","use":"sig"}]}"#.as_bytes();

/// Returns a valid ZkLoginInputs based on a fixed key, for testing only.
pub fn get_zklogin_inputs() -> ZkLoginInputs {
    thread_local! {
    static ZKLOGIN_INPUTS: ZkLoginInputs = ZkLoginInputs::from_json("{\"proofPoints\":{\"a\":[\"15200387022306772469780103920392862677802601879527915327677966523999524908895\",\"9812603768067793765979542039470632035744209822501998565655040006528244200197\",\"1\"],\"b\":[[\"885264670031809429821333565134817852263508437350062670143619463083426734559\",\"19244085992274056650330977335069586072998063003258522730688987208806475696739\"],[\"5053860066357759678930040579801334617766702650563332529037762035887306399743\",\"14081097970314919363679924210195018012067264685250670120690368511466983295818\"],[\"1\",\"0\"]],\"c\":[\"20792301904262722099604906864474181909479899041962528515892640376537438911084\",\"4899002069245696667230976861237705633766592990777686122323065957764959295596\",\"1\"]},\"issBase64Details\":{\"value\":\"wiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiw\",\"indexMod4\":2},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ\"}", "11123741740672126602579782119889048712914817144266543370111732841664868829558").unwrap(); }
    ZKLOGIN_INPUTS.with(|a| a.clone())
}

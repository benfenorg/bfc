// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{MultiSigPublicKey, ThresholdUnit, WeightUnit};
use crate::{
    base_types::SuiAddress,
    crypto::{
        get_key_pair, get_key_pair_from_rng, Ed25519SuiSignature, PublicKey, Signature, SuiKeyPair,
        SuiSignatureInner, ZkLoginPublicIdentifier,
    },
    multisig::{as_indices, MultiSig, MAX_SIGNER_IN_MULTISIG},
    multisig_legacy::bitmap_to_u16,
    signature::{AuthenticatorTrait, GenericSignature, VerifyParams},
    utils::{
        keys, load_test_vectors, make_transaction_data, make_zklogin_tx, DEFAULT_ADDRESS_SEED,
        SHORT_ADDRESS_SEED,
    },
    zk_login_authenticator::ZkLoginAuthenticator,
    zk_login_util::DEFAULT_JWK_BYTES,
};
use fastcrypto::{
    ed25519::Ed25519KeyPair,
    encoding::{Base64, Encoding},
    traits::ToFromBytes,
};
use fastcrypto_zkp::bn254::zk_login::{parse_jwks, JwkId, OIDCProvider, ZkLoginInputs, JWK};
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use fastcrypto_zkp::zk_login_utils::Bn254FrElement;
use im::hashmap::HashMap as ImHashMap;
use once_cell::sync::OnceCell;
use rand::{rngs::StdRng, SeedableRng};
use roaring::RoaringBitmap;
use shared_crypto::intent::{Intent, IntentMessage, PersonalMessage};
use std::str::FromStr;
use fastcrypto::traits::KeyPair;
use num_bigint::BigUint;

#[test]
fn test_combine_sigs() {
    let kp1: SuiKeyPair = SuiKeyPair::Ed25519(get_key_pair().1);
    let kp2: SuiKeyPair = SuiKeyPair::Secp256k1(get_key_pair().1);
    let kp3: SuiKeyPair = SuiKeyPair::Secp256r1(get_key_pair().1);

    let pk1 = kp1.public();
    let pk2 = kp2.public();

    let multisig_pk = MultiSigPublicKey::new(vec![pk1, pk2], vec![1, 1], 2).unwrap();

    let msg = IntentMessage::new(
        Intent::sui_transaction(),
        PersonalMessage {
            message: "Hello".as_bytes().to_vec(),
        },
    );
    let sig1: GenericSignature = Signature::new_secure(&msg, &kp1).into();
    let sig2 = Signature::new_secure(&msg, &kp2).into();
    let sig3 = Signature::new_secure(&msg, &kp3).into();

    // MultiSigPublicKey contains only 2 public key but 3 signatures are passed, fails to combine.
    assert!(MultiSig::combine(vec![sig1.clone(), sig2, sig3], multisig_pk.clone()).is_err());

    // Cannot create malformed MultiSig.
    assert!(MultiSig::combine(vec![], multisig_pk.clone()).is_err());
    assert!(MultiSig::combine(vec![sig1.clone(), sig1], multisig_pk).is_err());
}
#[test]
fn test_serde_roundtrip() {
    let msg = IntentMessage::new(
        Intent::sui_transaction(),
        PersonalMessage {
            message: "Hello".as_bytes().to_vec(),
        },
    );

    for kp in keys() {
        let pk = kp.public();
        let multisig_pk = MultiSigPublicKey::new(vec![pk], vec![1], 1).unwrap();
        let sig = Signature::new_secure(&msg, &kp).into();
        let multisig = MultiSig::combine(vec![sig], multisig_pk).unwrap();
        let plain_bytes = bcs::to_bytes(&multisig).unwrap();

        let generic_sig = GenericSignature::MultiSig(multisig);
        let generic_sig_bytes = generic_sig.as_bytes();
        let generic_sig_roundtrip = GenericSignature::from_bytes(generic_sig_bytes).unwrap();
        assert_eq!(generic_sig, generic_sig_roundtrip);

        // A MultiSig flag 0x03 is appended before the bcs serialized bytes.
        assert_eq!(plain_bytes.len() + 1, generic_sig_bytes.len());
        assert_eq!(generic_sig_bytes.first().unwrap(), &0x03);
    }

    // Malformed multisig cannot be deserialized
    let multisig_pk = MultiSigPublicKey {
        pk_map: vec![(keys()[0].public(), 1)],
        threshold: 1,
    };
    let multisig = MultiSig {
        sigs: vec![], // No sigs
        bitmap: 0,
        multisig_pk,
        bytes: OnceCell::new(),
    };

    let generic_sig = GenericSignature::MultiSig(multisig);
    let generic_sig_bytes = generic_sig.as_bytes();
    assert!(GenericSignature::from_bytes(generic_sig_bytes).is_err());

    // Malformed multisig_pk cannot be deserialized
    let multisig_pk_1 = MultiSigPublicKey {
        pk_map: vec![],
        threshold: 0,
    };

    let multisig_1 = MultiSig {
        sigs: vec![],
        bitmap: 0,
        multisig_pk: multisig_pk_1,
        bytes: OnceCell::new(),
    };

    let generic_sig_1 = GenericSignature::MultiSig(multisig_1);
    let generic_sig_bytes = generic_sig_1.as_bytes();
    assert!(GenericSignature::from_bytes(generic_sig_bytes).is_err());

    // Single sig serialization unchanged.
    let sig = Ed25519SuiSignature::default();
    let single_sig = GenericSignature::Signature(sig.clone().into());
    let single_sig_bytes = single_sig.as_bytes();
    let single_sig_roundtrip = GenericSignature::from_bytes(single_sig_bytes).unwrap();
    assert_eq!(single_sig, single_sig_roundtrip);
    assert_eq!(single_sig_bytes.len(), Ed25519SuiSignature::LENGTH);
    assert_eq!(
        single_sig_bytes.first().unwrap(),
        &Ed25519SuiSignature::SCHEME.flag()
    );
    assert_eq!(sig.as_bytes().len(), single_sig_bytes.len());
}

#[test]
fn test_multisig_pk_new() {
    let keys = keys();
    let pk1 = keys[0].public();
    let pk2 = keys[1].public();
    let pk3 = keys[2].public();

    // Fails on weight 0.
    assert!(MultiSigPublicKey::new(
        vec![pk1.clone(), pk2.clone(), pk3.clone()],
        vec![0, 1, 1],
        2
    )
    .is_err());

    // Fails on threshold 0.
    assert!(MultiSigPublicKey::new(
        vec![pk1.clone(), pk2.clone(), pk3.clone()],
        vec![1, 1, 1],
        0
    )
    .is_err());

    // Fails on incorrect array length.
    assert!(
        MultiSigPublicKey::new(vec![pk1.clone(), pk2.clone(), pk3.clone()], vec![1], 2).is_err()
    );

    // Fails on empty array length.
    assert!(MultiSigPublicKey::new(vec![pk1.clone(), pk2, pk3], vec![], 2).is_err());

    // Fails on dup pks.
    assert!(
        MultiSigPublicKey::new(vec![pk1.clone(), pk1.clone(), pk1], vec![1, 2, 3], 4,).is_err()
    );
}

#[test]
fn test_multisig_address() {
    // Pin an hardcoded multisig address generation here. If this fails, the address
    // generation logic may have changed. If this is intended, update the hardcoded value below.
    let keys = keys();
    let pk1 = keys[0].public();
    let pk2 = keys[1].public();
    let pk3 = keys[2].public();

    let threshold: ThresholdUnit = 2;
    let w1: WeightUnit = 1;
    let w2: WeightUnit = 2;
    let w3: WeightUnit = 3;

    let multisig_pk =
        MultiSigPublicKey::new(vec![pk1, pk2, pk3], vec![w1, w2, w3], threshold).unwrap();
    let address: SuiAddress = (&multisig_pk).into();
    assert_eq!(
        SuiAddress::from_str("0xe35c69eb504de34afdbd9f307fb3ca152646c92d549fea00065d26fc422109ea")
            .unwrap(),
        address
    );
}

#[test]
fn test_max_sig() {
    let msg = IntentMessage::new(
        Intent::sui_transaction(),
        PersonalMessage {
            message: "Hello".as_bytes().to_vec(),
        },
    );
    let mut seed = StdRng::from_seed([0; 32]);
    let mut keys = Vec::new();
    let mut pks = Vec::new();

    for _ in 0..11 {
        let k = SuiKeyPair::Ed25519(get_key_pair_from_rng(&mut seed).1);
        pks.push(k.public());
        keys.push(k);
    }

    // multisig_pk with larger that max number of pks fails.
    assert!(MultiSigPublicKey::new(
        pks.clone(),
        vec![WeightUnit::MAX; MAX_SIGNER_IN_MULTISIG + 1],
        ThresholdUnit::MAX
    )
    .is_err());

    // multisig_pk with unreachable threshold fails.
    assert!(MultiSigPublicKey::new(pks.clone()[..5].to_vec(), vec![3; 5], 16).is_err());

    // multisig_pk with max weights for each pk and max reachable threshold is ok.
    let res = MultiSigPublicKey::new(
        pks.clone()[..10].to_vec(),
        vec![WeightUnit::MAX; MAX_SIGNER_IN_MULTISIG],
        (WeightUnit::MAX as ThresholdUnit) * (MAX_SIGNER_IN_MULTISIG as ThresholdUnit),
    );
    assert!(res.is_ok());

    // multisig_pk with unreachable threshold fails.
    let res = MultiSigPublicKey::new(
        pks.clone()[..10].to_vec(),
        vec![WeightUnit::MAX; MAX_SIGNER_IN_MULTISIG],
        (WeightUnit::MAX as ThresholdUnit) * (MAX_SIGNER_IN_MULTISIG as ThresholdUnit) + 1,
    );
    assert!(res.is_err());

    // multisig_pk with max weights for each pk with threshold is 1x max weight validates ok.
    let low_threshold_pk = MultiSigPublicKey::new(
        pks.clone()[..10].to_vec(),
        vec![WeightUnit::MAX; 10],
        WeightUnit::MAX.into(),
    )
    .unwrap();
    let sig = Signature::new_secure(&msg, &keys[0]).into();
    assert!(MultiSig::combine(vec![sig; 1], low_threshold_pk)
        .unwrap()
        .init_and_validate()
        .is_ok());
}

#[test]
fn test_to_from_indices() {
    assert!(as_indices(0b11111111110).is_err());
    assert_eq!(as_indices(0b0000010110).unwrap(), vec![1, 2, 4]);
    assert_eq!(
        as_indices(0b1111111111).unwrap(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    let mut bitmap = RoaringBitmap::new();
    bitmap.insert(1);
    bitmap.insert(2);
    bitmap.insert(4);
    assert_eq!(bitmap_to_u16(bitmap.clone()).unwrap(), 0b0000010110);
    bitmap.insert(11);
    assert!(bitmap_to_u16(bitmap).is_err());
}

#[test]
fn multisig_get_pk() {
    let keys = keys();
    let pk1 = keys[0].public();
    let pk2 = keys[1].public();

    let multisig_pk = MultiSigPublicKey::new(vec![pk1, pk2], vec![1, 1], 2).unwrap();
    let msg = IntentMessage::new(
        Intent::sui_transaction(),
        PersonalMessage {
            message: "Hello".as_bytes().to_vec(),
        },
    );
    let sig1: GenericSignature = Signature::new_secure(&msg, &keys[0]).into();
    let sig2: GenericSignature = Signature::new_secure(&msg, &keys[1]).into();

    let multi_sig =
        MultiSig::combine(vec![sig1.clone(), sig2.clone()], multisig_pk.clone()).unwrap();

    assert!(multi_sig.get_pk().clone() == multisig_pk);
    assert!(
        *multi_sig.get_sigs() == vec![sig1.to_compressed().unwrap(), sig2.to_compressed().unwrap()]
    );
}

#[test]
fn multisig_get_indices() {
    let keys = keys();
    let pk1 = keys[0].public();
    let pk2 = keys[1].public();
    let pk3 = keys[2].public();

    let multisig_pk = MultiSigPublicKey::new(vec![pk1, pk2, pk3], vec![1, 1, 1], 2).unwrap();
    let msg = IntentMessage::new(
        Intent::sui_transaction(),
        PersonalMessage {
            message: "Hello".as_bytes().to_vec(),
        },
    );
    let sig1: GenericSignature = Signature::new_secure(&msg, &keys[0]).into();
    let sig2: GenericSignature = Signature::new_secure(&msg, &keys[1]).into();
    let sig3: GenericSignature = Signature::new_secure(&msg, &keys[2]).into();

    let multi_sig1 =
        MultiSig::combine(vec![sig2.clone(), sig3.clone()], multisig_pk.clone()).unwrap();

    let multi_sig2 = MultiSig::combine(
        vec![sig1.clone(), sig2.clone(), sig3.clone()],
        multisig_pk.clone(),
    )
    .unwrap();

    let invalid_multisig = MultiSig::combine(vec![sig3, sig2, sig1], multisig_pk).unwrap();

    // Indexes of public keys in multisig public key instance according to the combined sigs.
    assert!(multi_sig1.get_indices().unwrap() == vec![1, 2]);
    assert!(multi_sig2.get_indices().unwrap() == vec![0, 1, 2]);
    assert!(invalid_multisig.get_indices().unwrap() == vec![0, 1, 2]);
}

#[test]
fn multisig_zklogin_scenarios() {
    // consistency test with sui/sdk/typescript/test/unit/cryptography/multisig.test.ts
    let mut seed = StdRng::from_seed([1; 32]);
    // let kp: Ed25519KeyPair = get_key_pair_from_rng(&mut seed).1;
    let skp: SuiKeyPair = SuiKeyPair::Ed25519(Ed25519KeyPair::generate(&mut seed));
    // let skp: SuiKeyPair = SuiKeyPair::Ed25519(kp);
    let pk1 = skp.public();

    let mut eph_pk_bytes = vec![pk1.flag()];
    eph_pk_bytes.extend(pk1.as_ref());
    let kp_bigint = BigUint::from_bytes_be(&eph_pk_bytes);
    println!("Ephemeral keypair: {:?}", skp.encode());
    println!("Ephemeral pubkey (BigInt): {:?}", kp_bigint);

    let (_, _, inputs) = &load_test_vectors("./src/unit_tests/zklogin_test_vectors.json")[0];
    // pk consistent with the one in make_zklogin_tx
    let pk2 = PublicKey::ZkLogin(
        ZkLoginPublicIdentifier::new(
            &OIDCProvider::Twitch.get_config().iss,
            inputs.get_address_seed(),
        )
        .unwrap(),
    );

    // set up 1-out-of-2 multisig with one zklogin public identifier and one traditional public key.
    let multisig_pk = MultiSigPublicKey::new(vec![pk2, pk1], vec![1, 1], 1).unwrap();
    let multisig_addr = SuiAddress::from(&multisig_pk);
    assert_eq!(
        multisig_addr,
        SuiAddress::from_str("0xcdfb9850cbbe54e2093c1e104a50896e1069216e58ec0000d83251b9096eba21")
            .unwrap()
    );

    let (_, envelop, zklogin_sig) = make_zklogin_tx(multisig_addr, false);
    let binding = envelop.into_data();
    let tx = binding.transaction_data();
    assert_eq!(Base64::encode(bcs::to_bytes(tx).unwrap()), "AAABACACAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgEBAQABAADN+5hQy75U4gk8HhBKUIluEGkhbljsAADYMlG5CW66IQGbB4FfBEl+LgXSLKw6oGFBCyCGjMYZFUxCocYb6ZAnFwEAAAAAAAAAIHqXpR03f3carjyhraTVCgGVN1SwvsqRpfTLPY64YFRFzfuYUMu+VOIJPB4QSlCJbhBpIW5Y7AAA2DJRuQluuiEBAAAAAAAAAEANAwAAAAAAAA==".to_string());

    let intent_msg = &IntentMessage::new(Intent::sui_transaction(), tx.clone());
    assert_eq!(Base64::encode(zklogin_sig.as_ref()), "BQNMMjIzODg4NDk0OTc5MjA4NjM4MDE5OTQ3ODkxNDQ0MzEwNTYwODg4MzMxNjkyMDI0ODQ5MzY4Nzg2NDc3OTU1NzI5NDQzNTAxNTA1ME0yMDg4MjE0NjgxMDYzODEzMDMyMjAzOTY0MjcxNzE2NDM1OTU3OTcyNzczMzY2NDYxMDYxMDc3Mzc0MTAwNDE1Nzk2MjYzMDk5MDE4NwExAwJNMTMyNTk4NDEyMTM0OTc0NTU0MDE5ODQxNzcwNDU1NTYzMjU4NDQ1NzIwODE4ODg5NTAwNjQ4MzU0MDExOTU5NTgyNzU4NTI0ODkxNTVNMTIxODgxMDcyNTc4ODI3MTA3ODE5OTE0MTg0NzExNTc5MDc3ODU2NTc0MTc0NTUyODg2NDQ2NDk1MTk3NTM5MjEyNTc2MDEwMDk5NDMCTTExMTAxNjc2MjM5NTU4Nzc1MzkxMTU0MDcwOTQyNzExMDk0MDg4NDQ2MzM1ODA5NjkyNjQ0MTAyODYwMjkxMTU3MDEyMzUxNTg5ODI0TDgwNjY3NjAxNTIzMDA5NDk3NzY0NjYyMzE3MTIxODkzMTEwNTgyOTAwMjcwODc4MjkzMDEyNzg3ODgyMzkyMTE1MjEzOTUzNDMxMzQCATEBMANMNjI1Njc1NjkwMzAyMTUxNjQ3MjUxNTc3NTkyNjA4NDkwNTE5OTk3OTc5MzAxNDU0MTQyMTk2MDIxMjMyNTcxNDM1OTI1NDk4ODU0NU0xNDQ0MzQwMzYyMjI5MDc1MzEzNzQ5Mjk0OTg5MjUyNTY0NzQwMTI0NTQ3ODk2MzEwMTIyNjUwMDM4NzI1Nzg2NDM3MDAxMDU2MDQ3NgExMXdpYVhOeklqb2lhSFIwY0hNNkx5OXBaQzUwZDJsMFkyZ3VkSFl2YjJGMWRHZ3lJaXcCMmV5SmhiR2NpT2lKU1V6STFOaUlzSW5SNWNDSTZJa3BYVkNJc0ltdHBaQ0k2SWpFaWZRTTE1MjgzMDkyMjI5OTY4NDU5NTU1ODg1MTU0MTAxODQ1NzE1MzQyMTM1OTk4MDE0NDQwNzU5MjQ0Mjk5OTgzNTEzNDQ3NTM4NzUyNTM4CgAAAAAAAABhAGyjF3dDoIA+OtCT/tyaAB+O4iRnpWwQewsDxvGV2hRpWAH4QDHrIvPYBRHWSplje7nNGXyJDCl9hnV3sYx/gQG5xu4WMO8+cRFEpkjbBruyKE9ydM++5T/87lA8waSSAA==".to_string());

    let single_sig = GenericSignature::Signature(Signature::new_secure(intent_msg, &skp));
    let multisig = GenericSignature::MultiSig(
        MultiSig::combine(vec![single_sig, zklogin_sig], multisig_pk.clone()).unwrap(),
    );
    assert_eq!(Base64::encode(multisig.as_ref()), "AwIAC1EoYyi9D1XJFxLwIilYJ2ktMp6TmXjdg5WPvwKSio9RYAabgN71ga4LLpMTK9HdByWKVWq7JpfwdEVXCzn5BAOaBwUDTDIyMzg4ODQ5NDk3OTIwODYzODAxOTk0Nzg5MTQ0NDMxMDU2MDg4ODMzMTY5MjAyNDg0OTM2ODc4NjQ3Nzk1NTcyOTQ0MzUwMTUwNTBNMjA4ODIxNDY4MTA2MzgxMzAzMjIwMzk2NDI3MTcxNjQzNTk1Nzk3Mjc3MzM2NjQ2MTA2MTA3NzM3NDEwMDQxNTc5NjI2MzA5OTAxODcBMQMCTTEzMjU5ODQxMjEzNDk3NDU1NDAxOTg0MTc3MDQ1NTU2MzI1ODQ0NTcyMDgxODg4OTUwMDY0ODM1NDAxMTk1OTU4Mjc1ODUyNDg5MTU1TTEyMTg4MTA3MjU3ODgyNzEwNzgxOTkxNDE4NDcxMTU3OTA3Nzg1NjU3NDE3NDU1Mjg4NjQ0NjQ5NTE5NzUzOTIxMjU3NjAxMDA5OTQzAk0xMTEwMTY3NjIzOTU1ODc3NTM5MTE1NDA3MDk0MjcxMTA5NDA4ODQ0NjMzNTgwOTY5MjY0NDEwMjg2MDI5MTE1NzAxMjM1MTU4OTgyNEw4MDY2NzYwMTUyMzAwOTQ5Nzc2NDY2MjMxNzEyMTg5MzExMDU4MjkwMDI3MDg3ODI5MzAxMjc4Nzg4MjM5MjExNTIxMzk1MzQzMTM0AgExATADTDYyNTY3NTY5MDMwMjE1MTY0NzI1MTU3NzU5MjYwODQ5MDUxOTk5Nzk3OTMwMTQ1NDE0MjE5NjAyMTIzMjU3MTQzNTkyNTQ5ODg1NDVNMTQ0NDM0MDM2MjIyOTA3NTMxMzc0OTI5NDk4OTI1MjU2NDc0MDEyNDU0Nzg5NjMxMDEyMjY1MDAzODcyNTc4NjQzNzAwMTA1NjA0NzYBMTF3aWFYTnpJam9pYUhSMGNITTZMeTlwWkM1MGQybDBZMmd1ZEhZdmIyRjFkR2d5SWl3AjJleUpoYkdjaU9pSlNVekkxTmlJc0luUjVjQ0k2SWtwWFZDSXNJbXRwWkNJNklqRWlmUU0xNTI4MzA5MjIyOTk2ODQ1OTU1NTg4NTE1NDEwMTg0NTcxNTM0MjEzNTk5ODAxNDQ0MDc1OTI0NDI5OTk4MzUxMzQ0NzUzODc1MjUzOAoAAAAAAAAAYQBsoxd3Q6CAPjrQk/7cmgAfjuIkZ6VsEHsLA8bxldoUaVgB+EAx6yLz2AUR1kqZY3u5zRl8iQwpfYZ1d7GMf4EBucbuFjDvPnERRKZI2wa7sihPcnTPvuU//O5QPMGkkgADAAIDPBtodHRwczovL2lkLnR3aXRjaC50di9vYXV0aDIhyex1AlxW6vOIayyBvgGzyPMGmEAn50k8km4kUan8GgEAjFUzNe7oC5v6DFRKRf5jR0oJ3/nEsLM9srZi+TTqRsQBAQA=".to_string());
}

#[test]
fn zklogin_in_multisig_works_with_both_addresses() {
    let mut seed = StdRng::from_seed([0; 32]);
    let kp: Ed25519KeyPair = get_key_pair_from_rng(&mut seed).1;
    let skp: SuiKeyPair = SuiKeyPair::Ed25519(kp);
    let pk = skp.public();

    // printing important key information
    let mut eph_pk_bytes = vec![pk.flag()];
    eph_pk_bytes.extend(pk.as_ref());
    let kp_bigint = BigUint::from_bytes_be(&eph_pk_bytes);
    println!("Ephemeral keypair: {:?}", skp.encode());
    println!("Ephemeral pubkey (BigInt): {:?}", kp_bigint);

    // create a new multisig address based on pk1 and pk2 where pk1 is a zklogin public identifier, with a crafted unpadded bytes.
    let mut bytes = Vec::new();
    let binding = OIDCProvider::Twitch.get_config();
    let iss_bytes = binding.iss.as_bytes();
    bytes.extend([iss_bytes.len() as u8]);
    bytes.extend(iss_bytes);
    // length here is 31 bytes and left unpadded.
    let address_seed = Bn254FrElement::from_str(SHORT_ADDRESS_SEED).unwrap();
    bytes.extend(address_seed.unpadded());

    let pk1 = PublicKey::ZkLogin(ZkLoginPublicIdentifier(bytes));
    let pk2 = skp.public();
    let multisig_pk = MultiSigPublicKey::new(vec![pk1, pk2.clone()], vec![1; 2], 1).unwrap();
    let multisig_address = SuiAddress::from(&multisig_pk);

    let (kp, _pk, input) = &load_test_vectors("./src/unit_tests/zklogin_test_vectors.json")[0];
    let intent_msg = &IntentMessage::new(
        Intent::sui_transaction(),
        make_transaction_data(multisig_address),
    );
    let user_signature = Signature::new_secure(intent_msg, kp);

    let modified_inputs =
        ZkLoginInputs::from_json(&serde_json::to_string(input).unwrap(), SHORT_ADDRESS_SEED)
            .unwrap();
    let zklogin_sig = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        modified_inputs.clone(),
        10,
        user_signature,
    ));
    let multisig =
        MultiSig::insecure_new(vec![zklogin_sig.to_compressed().unwrap()], 1, multisig_pk);

    let parsed: ImHashMap<JwkId, JWK> = parse_jwks(DEFAULT_JWK_BYTES, &OIDCProvider::Twitch)
        .unwrap()
        .into_iter()
        .collect();

    let aux_verify_data = VerifyParams::new(parsed, vec![], ZkLoginEnv::Test, true, true, Some(30));
    let res = multisig.verify_claims(intent_msg, multisig_address, &aux_verify_data);
    // since the zklogin inputs is crafted, it is expected that the proof verify failed, but all checks before passes.
    assert!(
        matches!(res, Err(crate::error::SuiError::InvalidSignature { error }) if error.contains("General cryptographic error: Groth16 proof verify failed"))
    );

    // initialize zklogin pk (pk1_padd) with padded address seed
    let pk1_padded = PublicKey::ZkLogin(
        ZkLoginPublicIdentifier::new(
            &OIDCProvider::Twitch.get_config().iss,
            &Bn254FrElement::from_str(SHORT_ADDRESS_SEED).unwrap(),
        )
        .unwrap(),
    );
    let multisig_pk_padded = MultiSigPublicKey::new(vec![pk1_padded, pk2], vec![1; 2], 1).unwrap();
    let multisig_address_padded = SuiAddress::from(&multisig_pk_padded);
    let modified_inputs_padded =
        ZkLoginInputs::from_json(&serde_json::to_string(input).unwrap(), SHORT_ADDRESS_SEED)
            .unwrap();
    let intent_msg_padded = &IntentMessage::new(
        Intent::sui_transaction(),
        make_transaction_data(multisig_address_padded),
    );
    let user_signature_padded = Signature::new_secure(intent_msg_padded, kp);
    let zklogin_sig_padded = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        modified_inputs_padded.clone(),
        10,
        user_signature_padded,
    ));
    let multisig_padded = MultiSig::insecure_new(
        vec![zklogin_sig_padded.to_compressed().unwrap()],
        1,
        multisig_pk_padded,
    );

    let res =
        multisig_padded.verify_claims(intent_msg_padded, multisig_address_padded, &aux_verify_data);
    assert!(
        matches!(res, Err(crate::error::SuiError::InvalidSignature { error }) if error.contains("General cryptographic error: Groth16 proof verify failed"))
    );
}

#[test]
fn test_derive_multisig_address() {
    // consistency test with typescript: /sdk/typescript/test/unit/cryptography/multisig.test.ts
    let pk1 = PublicKey::ZkLogin(
        ZkLoginPublicIdentifier::new(
            &OIDCProvider::Twitch.get_config().iss,
            &Bn254FrElement::from_str(DEFAULT_ADDRESS_SEED).unwrap(),
        )
        .unwrap(),
    );
    // address seed here is padded with leading 0 to 32 bytes.
    let pk2 = PublicKey::ZkLogin(
        ZkLoginPublicIdentifier::new(
            &OIDCProvider::Twitch.get_config().iss,
            &Bn254FrElement::from_str(SHORT_ADDRESS_SEED).unwrap(),
        )
        .unwrap(),
    );
    assert_eq!(pk1.as_ref().len(), pk2.as_ref().len());

    let multisig_pk = MultiSigPublicKey::new(vec![pk1, pk2], vec![1, 1], 1).unwrap();
    let multisig_addr = SuiAddress::from(&multisig_pk);
    assert_eq!(
        multisig_addr,
        SuiAddress::from_str("0x77a9fbf3c695d78dd83449a81a9e70aa79a77dbfd6fb72037bf09201c12052cd")
            .unwrap()
    );
}

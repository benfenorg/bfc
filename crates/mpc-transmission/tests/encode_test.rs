
#[cfg(test)]
mod encode_tests {
    use rand::random;
    use mpc_transmission::{recover_value, split_to_two_value};

    #[test]
    fn test_encode_decode_share_data_roundtrip() {
        let start_value = 200000002u64;

        for extend_value in 0..10*10000 {
           let test_value = start_value + extend_value;
            let mask_secret = 0x00033344555u64;
            let user_id = random();

            let (encoded1, encode2) = split_to_two_value(test_value, user_id, mask_secret);
            let decoded_value = recover_value(encoded1.clone(), encode2.clone(), mask_secret).unwrap();

            assert_eq!(test_value, decoded_value);
            //println!("test value: {}, decoded value: {}", test_value, decoded_value);
            println!("encode1: {}, encode2: {}", encoded1, encode2);

        }
    }
}
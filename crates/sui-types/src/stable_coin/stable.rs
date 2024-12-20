#[sui_macros::with_checked_arithmetic]
pub mod checked {
    use crate::BFC_SYSTEM_ADDRESS;
    use move_core_types::ident_str;
    use move_core_types::language_storage::{StructTag, TypeTag};
    use std::collections::HashMap;
    use std::convert::TryFrom;
    use std::str::FromStr;
    use std::sync::{LazyLock, RwLock};

    struct AllowStableGasCoin {
        rate_map: HashMap<String, u64>,
    }

    //let rate_map: HashMap<String, u64> = bfc_system_state.get_rate_map().contents
    // .iter()
    // .map(|entity| ((*entity.key).to_string(), entity.value))
    // .collect();
    impl AllowStableGasCoin {
        fn new() -> Self {
            AllowStableGasCoin {
                rate_map: HashMap::new(),
            }
        }

        fn set_rate_map(&mut self, value: HashMap<String, u64>) {
            self.rate_map = value;
        }

        fn get_rate_map(&self) -> &HashMap<String, u64> {
            &self.rate_map
        }
    }

    static INSTANCE: LazyLock<RwLock<AllowStableGasCoin>> =
        LazyLock::new(|| RwLock::new(AllowStableGasCoin::new()));

    pub fn update_allow_stable_gas_coins(value: HashMap<String, u64>) {
        let w = INSTANCE.write();
        if w.is_err() {
            return;
        }

        w.unwrap().set_rate_map(value);
    }

    pub fn get_allow_stable_gas_coins_rate_map() -> HashMap<String, u64> {
        let r = INSTANCE.read();
        if r.is_err() {
            return HashMap::new();
        }

        r.unwrap().get_rate_map().clone()
    }

    fn convert_and_format_hex_address(input: &str) -> String {
        if input.starts_with("0x") {
            return input.to_string();
        }

       format!("0x{}", input)
    }

    fn is_new_gas_type(other: &TypeTag) -> bool {
        let rate_map = get_allow_stable_gas_coins_rate_map();
        rate_map.iter().any(|(key, _)| {
            let tag = TypeTag::from_str(&(convert_and_format_hex_address(key)));
            if tag.is_err() {
                return false;
            }

            return &(tag.unwrap()) == other;
        })
    }

    pub enum STABLE {
        BUSD,
        BARS,
        BAUD,
        BBRL,
        BCAD,
        BEUR,
        BGBP,
        BIDR,
        BINR,
        BJPY,
        BKRW,
        BMXN,
        BRUB,
        BSAR,
        BTRY,
        BZAR,
        MGG,
    }

    impl STABLE {
        pub fn type_(&self) -> StructTag {
            let (module_name, struct_name) = match self {
                STABLE::BARS => (ident_str!("bars"), ident_str!("BARS")),
                STABLE::BAUD => (ident_str!("baud"), ident_str!("BAUD")),
                STABLE::BUSD => (ident_str!("busd"), ident_str!("BUSD")),
                STABLE::BBRL => (ident_str!("bbrl"), ident_str!("BBRL")),
                STABLE::BCAD => (ident_str!("bcad"), ident_str!("BCAD")),
                STABLE::BEUR => (ident_str!("beur"), ident_str!("BEUR")),
                STABLE::BGBP => (ident_str!("bgbp"), ident_str!("BGBP")),
                STABLE::BIDR => (ident_str!("bidr"), ident_str!("BIDR")),
                STABLE::BINR => (ident_str!("binr"), ident_str!("BINR")),
                STABLE::BJPY => (ident_str!("bjpy"), ident_str!("BJPY")),
                STABLE::BKRW => (ident_str!("bkrw"), ident_str!("BKRW")),
                STABLE::BMXN => (ident_str!("bmxn"), ident_str!("BMXN")),
                STABLE::BRUB => (ident_str!("brub"), ident_str!("BRUB")),
                STABLE::BSAR => (ident_str!("bsar"), ident_str!("BSAR")),
                STABLE::BTRY => (ident_str!("btry"), ident_str!("BTRY")),
                STABLE::BZAR => (ident_str!("bzar"), ident_str!("BZAR")),
                STABLE::MGG => (ident_str!("mgg"), ident_str!("MGG")),
            };

            StructTag {
                address: BFC_SYSTEM_ADDRESS,
                name: struct_name.to_owned(),
                module: module_name.to_owned(),
                type_params: Vec::new(),
            }
        }

        pub fn all_stable_coins_type() -> Vec<TypeTag> {
            let mut types: Vec<TypeTag> = vec![
                TypeTag::from_str("0xc8::busd::BUSD").unwrap(),
                TypeTag::from_str("0xc8::bjpy::BJPY").unwrap(),
                TypeTag::from_str("0xc8::beur::BEUR").unwrap(),
                TypeTag::from_str("0xc8::bars::BARS").unwrap(),
                TypeTag::from_str("0xc8::baud::BAUD").unwrap(),
                TypeTag::from_str("0xc8::bbrl::BBRL").unwrap(),
                TypeTag::from_str("0xc8::bcad::BCAD").unwrap(),
                TypeTag::from_str("0xc8::bgbp::BGBP").unwrap(),
                TypeTag::from_str("0xc8::bidr::BIDR").unwrap(),
                TypeTag::from_str("0xc8::binr::BINR").unwrap(),
                TypeTag::from_str("0xc8::bkrw::BKRW").unwrap(),
                TypeTag::from_str("0xc8::bmxn::BMXN").unwrap(),
                TypeTag::from_str("0xc8::brub::BRUB").unwrap(),
                TypeTag::from_str("0xc8::bsar::BSAR").unwrap(),
                TypeTag::from_str("0xc8::btry::BTRY").unwrap(),
                TypeTag::from_str("0xc8::bzar::BZAR").unwrap(),
                TypeTag::from_str("0xc8::mgg::MGG").unwrap(),
            ];

            let mut keys: Vec<String> = get_allow_stable_gas_coins_rate_map().keys().cloned().collect();
            keys.sort();

            for key in keys {
                let tag_result = TypeTag::from_str(&(convert_and_format_hex_address(&key)));
                if tag_result.is_ok() {
                    let tag = tag_result.unwrap();
                    if types.contains(&tag) {
                        continue;
                    }

                    types.push(tag);
                }
            }

            types
        }

        pub fn get_index(&self) -> u8 {
            match self {
                STABLE::BUSD => 0,
                STABLE::BARS => 1,
                STABLE::BAUD => 2,
                STABLE::BBRL => 3,
                STABLE::BCAD => 4,
                STABLE::BEUR => 5,
                STABLE::BGBP => 6,
                STABLE::BIDR => 7,
                STABLE::BINR => 8,
                STABLE::BJPY => 9,
                STABLE::BKRW => 10,
                STABLE::BMXN => 11,
                STABLE::BRUB => 12,
                STABLE::BSAR => 13,
                STABLE::BTRY => 14,
                STABLE::BZAR => 15,
                STABLE::MGG => 16,
            }
        }

        pub fn type_tag(&self) -> TypeTag {
            TypeTag::Struct(Box::new(self.type_()))
        }

        pub fn is_gas_type(other: &TypeTag) -> bool {
            [   STABLE::BARS,
                STABLE::BAUD,
                STABLE::BZAR,
                STABLE::BUSD,
                STABLE::BBRL,
                STABLE::BCAD,
                STABLE::BEUR,
                STABLE::BGBP,
                STABLE::BIDR,
                STABLE::BINR,
                STABLE::BJPY,
                STABLE::BKRW,
                STABLE::BMXN,
                STABLE::BRUB,
                STABLE::BSAR,
                STABLE::BTRY,
                STABLE::MGG,
            ]
            .iter()
            .map(|stable_type| stable_type.type_tag())
            .any(|stable_tag| &stable_tag == other)
                || is_new_gas_type(other)
        }

        pub fn is_gas_struct(other: &StructTag) -> bool {
            [   STABLE::BARS,
                STABLE::BAUD,
                STABLE::BZAR,
                STABLE::BUSD,
                STABLE::BBRL,
                STABLE::BCAD,
                STABLE::BEUR,
                STABLE::BGBP,
                STABLE::BIDR,
                STABLE::BINR,
                STABLE::BJPY,
                STABLE::BKRW,
                STABLE::BMXN,
                STABLE::BRUB,
                STABLE::BSAR,
                STABLE::BTRY,
                STABLE::MGG,
            ]
                .iter()
                .map(|stable_type| stable_type.type_())
                .any(|struct_tag| &struct_tag == other)
        }
    }

    impl TryFrom<u8> for STABLE {
        type Error = anyhow::Error;

        fn try_from(index: u8) -> Result<Self, Self::Error> {
            match index {
                0 => Ok(STABLE::BUSD),
                1 => Ok(STABLE::BARS),
                2 => Ok(STABLE::BAUD),
                3 => Ok(STABLE::BBRL),
                4 => Ok(STABLE::BCAD),
                5 => Ok(STABLE::BEUR),
                6 => Ok(STABLE::BGBP),
                7 => Ok(STABLE::BIDR),
                8 => Ok(STABLE::BINR),
                9 => Ok(STABLE::BJPY),
                10 => Ok(STABLE::BKRW),
                11 => Ok(STABLE::BMXN),
                12 => Ok(STABLE::BRUB),
                13 => Ok(STABLE::BSAR),
                14 => Ok(STABLE::BTRY),
                15 => Ok(STABLE::BZAR),
                16 => Ok(STABLE::MGG),
                _ => Err(anyhow::anyhow!("Invalid index for stable coin: {}", index)),
            }
        }
    }

    impl TryFrom<StructTag> for STABLE {
        type Error = anyhow::Error;
        fn try_from(s: StructTag) -> Result<Self, Self::Error> {

            match (s.module.as_str(), s.name.as_str()) {
                ("bars", "BARS") => Ok(STABLE::BARS),
                ("busd", "BUSD") => Ok(STABLE::BUSD),
                ("baud", "BAUD") => Ok(STABLE::BAUD),
                ("bbrl", "BBRL") => Ok(STABLE::BBRL),
                ("bcad", "BCAD") => Ok(STABLE::BCAD),
                ("beur", "BEUR") => Ok(STABLE::BEUR),
                ("bgbp", "BGBP") => Ok(STABLE::BGBP),
                ("bidr", "BIDR") => Ok(STABLE::BIDR),
                ("binr", "BINR") => Ok(STABLE::BINR),
                ("bjpy", "BJPY") => Ok(STABLE::BJPY),
                ("bkrw", "BKRW") => Ok(STABLE::BKRW),
                ("bmxn", "BMXN") => Ok(STABLE::BMXN),
                ("brub", "BRUB") => Ok(STABLE::BRUB),
                ("bsar", "BSAR") => Ok(STABLE::BSAR),
                ("btry", "BTRY") => Ok(STABLE::BTRY),
                ("bzar", "BZAR") => Ok(STABLE::BZAR),
                ("mgg", "MGG") => Ok(STABLE::MGG),
                _ => Err(anyhow::anyhow!("unreachable tag: {:?}", s)),
            }
        }
    }

    impl TryFrom<TypeTag> for STABLE {
        type Error = anyhow::Error;
        fn try_from(s: TypeTag) -> Result<Self, Self::Error> {
            match s {
                TypeTag::Struct(s1) => STABLE::try_from(*s1),
                _ => Err(anyhow::anyhow!("unreachable tag: {:?}", s)),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::thread;

        #[test]
        fn test_singleton_initial_value() {
            assert_eq!(get_allow_stable_gas_coins_rate_map().len(), 0);

            let mut m = HashMap::new();
            m.insert("BUSD".to_string(), 100);
            update_allow_stable_gas_coins(m);

            assert_eq!(get_allow_stable_gas_coins_rate_map().len(), 1);
        }



        #[test]
        fn test_singleton_thread_safety() {
            let handles: Vec<_> = (0..100)
                .map(|i| {
                    thread::spawn(move || {
                        let mut m = HashMap::new();
                        m.insert("BUSD".to_string(), 100);
                        update_allow_stable_gas_coins(m);
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }

            assert_eq!(get_allow_stable_gas_coins_rate_map().len(), 1);
        }

        #[test]
        fn test_concurrent_read_write() {
            let write_handles: Vec<_> = (0..50)
                .map(|i| {
                    thread::spawn(move || {
                        let mut m = HashMap::new();
                        m.insert("BUSD".to_string(), 100);
                        update_allow_stable_gas_coins(m);
                    })
                })
                .collect();

            let read_handles: Vec<_> = (0..50)
                .map(|_| {
                    thread::spawn(|| {
                        assert_eq!(get_allow_stable_gas_coins_rate_map().len(), 1);
                    })
                })
                .collect();

            for handle in write_handles {
                handle.join().unwrap();
            }

            for handle in read_handles {
                handle.join().unwrap();
            }

            assert_eq!(get_allow_stable_gas_coins_rate_map().len(), 1);
        }

        #[test]
        fn test_new_gas_type() {
            {
                assert!(TypeTag::from_str("0xc8::bcad::BCAD").is_ok());
                assert!(TypeTag::from_str("00000c8::bcad::BCAD").is_err());
                assert!(TypeTag::from_str("0x000000c8::bcad::BCAD").is_ok());
            }

            let mut m = HashMap::new();
            m.insert(
                "00000000000000000000000000000000000000000000000000000000000000c8::bars::BARS"
                    .to_string(),
                100,
            );
            m.insert("0000000000000000000c8::bcad::BCAD".to_string(), 100);
            m.insert("0xc8::baud::BAUD".to_string(), 100);
            update_allow_stable_gas_coins(m);

            let ok = is_new_gas_type(&TypeTag::from_str("0xc8::bars::BARS").unwrap());
            assert!(ok);
            let ok = is_new_gas_type(&TypeTag::from_str("0xc8::baud::BAUD").unwrap());
            assert!(ok);
            let ok = is_new_gas_type(&TypeTag::from_str("0xc8::bcad::BCAD").unwrap());
            assert!(ok);

            let ok = is_new_gas_type(&TypeTag::from_str("0x00c8::bcad::BCAD").unwrap());
            assert!(ok);
        }

        #[test]
        fn test_convert_and_format_hex() {
            let hex_str = "0x123456789abcdef";
            let result = convert_and_format_hex_address(hex_str);
            assert_eq!(result, "0x123456789abcdef");

            let hex_str = "000000c8::bars::BARS";
            let result = convert_and_format_hex_address(hex_str);
            assert_eq!(result, "0x000000c8::bars::BARS");

            let hex_str = "0xc8::bars::BARS";
            let result = convert_and_format_hex_address(hex_str);
            assert_eq!(result, "0xc8::bars::BARS");
        }

        #[test]
        fn test_all_stable_coins_type() {
            let tags = STABLE::all_stable_coins_type();
            assert_eq!(tags.len(), 17);

            let mut m = HashMap::new();
            m.insert(
                "00000000000000000000000000000000000000000000000000000000000000c8::bars::BARS"
                    .to_string(),
                100,
            );
            m.insert("0000000000000000000c8::bcad::BCAD".to_string(), 100);
            m.insert("0xc8::baud::BAUD".to_string(), 100);
            update_allow_stable_gas_coins(m);
            let tags = STABLE::all_stable_coins_type();
            assert_eq!(tags.len(), 17);

            let mut m = HashMap::new();
            m.insert("0000000000000000000c8::abc::abc".to_string(), 100);
            m.insert("0xc8::fff::fff".to_string(), 100);
            update_allow_stable_gas_coins(m);
            let tags = STABLE::all_stable_coins_type();
            println!("{:?}", tags.clone());
            assert_eq!(tags.len(), 19);
        }
    }
}

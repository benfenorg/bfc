#[sui_macros::with_checked_arithmetic]
pub mod checked {
    use move_core_types::ident_str;
    use move_core_types::language_storage::{StructTag, TypeTag};
    use crate::BFC_SYSTEM_ADDRESS;
    use std::str::FromStr;
    use std::convert::TryFrom;

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
            vec![
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
            ]
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
}
#[sui_macros::with_checked_arithmetic]
pub mod checked {
    use move_core_types::ident_str;
    use move_core_types::language_storage::{StructTag, TypeTag};
    use crate::BFC_SYSTEM_ADDRESS;

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
            };

            StructTag {
                address: BFC_SYSTEM_ADDRESS,
                name: struct_name.to_owned(),
                module: module_name.to_owned(),
                type_params: Vec::new(),
            }
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
            }
        }

        pub fn from_index(index: u8) -> Self {
            match index {
                0 => STABLE::BUSD,
                1 => STABLE::BARS,
                2 => STABLE::BAUD,
                3 => STABLE::BBRL,
                4 => STABLE::BCAD,
                5 => STABLE::BEUR,
                6 => STABLE::BGBP,
                7 => STABLE::BIDR,
                8 => STABLE::BINR,
                9 => STABLE::BJPY,
                10 => STABLE::BKRW,
                11 => STABLE::BMXN,
                12 => STABLE::BRUB,
                13 => STABLE::BSAR,
                14 => STABLE::BTRY,
                15 => STABLE::BZAR,
                _ => panic!("Invalid index for stable coin: {}", index),
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
            ]
                .iter()
                .map(|stable_type| stable_type.type_())
                .any(|struct_tag| &struct_tag == other)
        }
    }


    impl From<StructTag> for STABLE {
        fn from(s: StructTag) -> Self {
            match (s.module.as_str(), s.name.as_str()) {
                ("bars", "BARS") => STABLE::BARS,
                ("busd", "BUSD") => STABLE::BUSD,
                ("baud", "BAUD") => STABLE::BAUD,
                ("bbrl", "BBRL") => STABLE::BBRL,
                ("bcad", "BCAD") => STABLE::BCAD,
                ("beur", "BEUR") => STABLE::BEUR,
                ("bgbp", "BGBP") => STABLE::BGBP,
                ("bidr", "BIDR") => STABLE::BIDR,
                ("binr", "BINR") => STABLE::BINR,
                ("bjpy", "BJPY") => STABLE::BJPY,
                ("bkrw", "BKRW") => STABLE::BKRW,
                ("bmxn", "BMXN") => STABLE::BMXN,
                ("brub", "BRUB") => STABLE::BRUB,
                ("bsar", "BSAR") => STABLE::BSAR,
                ("btry", "BTRY") => STABLE::BTRY,
                ("bzar", "BZAR") => STABLE::BZAR,
                // default
                _ => STABLE::BUSD
            }
        }
    }
    impl From<TypeTag> for STABLE {
        fn from(s: TypeTag) -> Self {
            match s {
                TypeTag::Struct(s) => STABLE::from(*s),
                _ => panic!("unreachable tag: {:?}", s),
            }
        }
    }
}
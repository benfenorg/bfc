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
                // 其他情况
                _ => panic!("unknown StructTag: {:?}, {:?}", s.module, s.name),
            }
        }
    }
    impl From<TypeTag> for STABLE {
        fn from(s: TypeTag) -> Self {
            match s {
                TypeTag::Struct(s) => STABLE::from(*s),
                _ => panic!("unknown TypeTag: {:?}", s),
            }
        }
    }
}
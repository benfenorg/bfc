#[sui_macros::with_checked_arithmetic]
pub mod checked {
    use move_core_types::ident_str;
    use move_core_types::identifier::IdentStr;
    use move_core_types::language_storage::{StructTag, TypeTag};
    use crate::BFC_SYSTEM_ADDRESS;

    pub const MODULE_NAME: &IdentStr = ident_str!("busd");
    pub const STRUCT_NAME: &IdentStr = ident_str!("BUSD");

    pub struct BUSD {}
    impl BUSD {
        pub fn type_() -> StructTag {
            StructTag {
                address: BFC_SYSTEM_ADDRESS,
                name: STRUCT_NAME.to_owned(),
                module: MODULE_NAME.to_owned(),
                type_params: Vec::new(),
            }
        }

        pub fn type_tag() -> TypeTag {
            TypeTag::Struct(Box::new(Self::type_()))
        }

        pub fn is_gas(other: &StructTag) -> bool {
            &Self::type_() == other
        }

        pub fn is_gas_type(other: &TypeTag) -> bool {
            match other {
                TypeTag::Struct(s) => Self::is_gas(s),
                _ => false,
            }
        }
    }
}
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use crate::base_types::SequenceNumber;
use crate::error::SuiResult;
use crate::storage::ObjectStore;
use crate::{BFC_ANONYMOUS_STATE_OBJECT_ID, SUI_FRAMEWORK_ADDRESS};
use crate::object::Owner;

pub const ANONYMOUS_MODULE_NAME: &IdentStr = ident_str!("anonymous");
pub const ANONYMOUS_STATE_STRUCT_NAME: &IdentStr = ident_str!("Anonymous");
pub const ANONYMOUS_STATE_UPDATE_FUNCTION_NAME: &IdentStr = ident_str!("update_anonymous_state");
pub const ANONYMOUS_STATE_CREATE_FUNCTION_NAME: &IdentStr = ident_str!("create");


//the default address for ABFC valut
pub const ANONYMOUS_COIND_DEFAULT_ADDRESS: &str = "BFCfc171f86c07b0311a347d7e71b261c684848becbececec78802f1bf8a599f729d85a";
pub const RESOLVED_BFC_ANONYMOUS_STATE: (&AccountAddress, &IdentStr, &IdentStr) = (
    &SUI_FRAMEWORK_ADDRESS,
    crate::anonymous_status::ANONYMOUS_MODULE_NAME,
    crate::anonymous_status::ANONYMOUS_STATE_STRUCT_NAME,
);



pub fn get_anonymous_state_obj_initial_shared_version(
    object_store: &dyn ObjectStore,
) -> SuiResult<Option<SequenceNumber>> {
    Ok(object_store
        .get_object(&BFC_ANONYMOUS_STATE_OBJECT_ID)
        .map(|obj| match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => initial_shared_version,
            _ => unreachable!("Anonymous state object must be shared"),
        }))
}
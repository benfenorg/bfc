use diesel::sql_types::{BigInt, Text};
use diesel::QueryableByName;

use sui_json_rpc_types::NetworkOverview;

#[derive(QueryableByName, Debug, Clone, Default)]
pub struct DBNetworkOverview {
    #[diesel(sql_type = Text)]
    pub volume_24h: String,
    #[diesel(sql_type = BigInt)]
    pub total_addresses_24h: i64,
    #[diesel(sql_type = Text)]
    pub current_gas: String,
}

impl From<DBNetworkOverview> for NetworkOverview {
    fn from(db: DBNetworkOverview) -> Self {
        Self {
            volume_24h: db.volume_24h,
            total_addresses_24h: db.total_addresses_24h as u64,
            current_gas: db.current_gas,
        }
    }
}

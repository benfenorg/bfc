use diesel::{Insertable, Queryable};
use crate::schema::stake_pending_item;

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = stake_pending_item)]
pub struct StakePendingItem {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub owner: String,
    pub miner_id: String,
    pub ticket_id: String,
    pub debt: i64,
}
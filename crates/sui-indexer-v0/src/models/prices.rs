use crate::schema::price_history;
use diesel::prelude::*;

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = price_history)]
pub struct PriceHistory {
    #[diesel(deserialize_as = i64)]
    pub ts: i64,
    pub coin: String,
    pub price: i64,
}

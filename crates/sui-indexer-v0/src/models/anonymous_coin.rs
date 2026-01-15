use diesel::{Insertable, Queryable};
use crate::schema::anonymous_coin::{self};

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = anonymous_coin)]
pub struct AnonymousCoin {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub owner: String,
    pub object_id: String,
    pub bcs_str: String,
}
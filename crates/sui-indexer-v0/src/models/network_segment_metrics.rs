use diesel::prelude::*;

use crate::schema::network_segment_metrics::{self};

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = network_segment_metrics)]
pub struct NetworkSegmentMetrics {
    pub segment_started_at: i64,
    pub total_transact_obc: i64,
    pub avg_gas_cost: i64,
    pub gas_checkpoint: i64,
}

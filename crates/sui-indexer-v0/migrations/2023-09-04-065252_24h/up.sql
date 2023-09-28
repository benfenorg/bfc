-- Your SQL goes here
CREATE INDEX addresses_last_appearance_time ON addresses (last_appearance_time);
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS transact_obc BIGINT NOT NULL DEFAULT 0;
ALTER TABLE checkpoints ADD COLUMN IF NOT EXISTS total_transact_obc BIGINT NOT NULL DEFAULT 0,
      ADD COLUMN IF NOT EXISTS system_tick BOOLEAN NOT NULL DEFAULT false;

CREATE TABLE network_segment_metrics
(
    segment_started_at      BIGINT PRIMARY KEY, -- use the time of a segment started at as a slot: CURRENT_TIMESTAMP - CURRENT_TIMESTAMP % SEGMENT_INTERVAL
    total_transact_obc      BIGINT NOT NULL,    -- How much OBCs were transacted in this minute
    avg_gas_cost            BIGINT NOT NULL,    -- The current avg gas cost of the last checkpoint that contains transactions which not only system ticks.
    gas_checkpoint          BIGINT NOT NULL     -- Which checkpoint we were using to calculate the avg_gas_cost.
);


CREATE OR REPLACE VIEW network_overviews AS
SELECT (SELECT SUM(total_transact_obc)::varchar FROM network_segment_metrics WHERE segment_started_at > extract(epoch from current_timestamp)::BIGINT - 86400)
               AS volume_24h,
       (SELECT COUNT(1) FROM addresses WHERE last_appearance_time > extract(epoch from current_timestamp)::BIGINT - 86400)
               AS total_addresses_24h,
       (SELECT avg_gas_cost FROM network_segment_metrics ORDER BY segment_started_at DESC LIMIT 1)
               AS current_gas;

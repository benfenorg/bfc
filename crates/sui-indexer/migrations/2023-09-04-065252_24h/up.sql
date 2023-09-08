-- Your SQL goes here
CREATE INDEX addresses_last_appearance_time ON addresses (last_appearance_time);
ALTER TABLE transactions ADD COLUMN IF NOT EXISTS transact_obc BIGINT NOT NULL DEFAULT 0;
ALTER TABLE checkpoints ADD COLUMN IF NOT EXISTS total_transact_obc BIGINT NOT NULL DEFAULT 0,
      ADD COLUMN IF NOT EXISTS system_tick BOOLEAN NOT NULL DEFAULT false;

CREATE OR REPLACE VIEW network_overviews AS
SELECT (SELECT SUM(total_transact_obc)::varchar FROM checkpoints WHERE TO_TIMESTAMP(timestamp_ms / 1000) > CURRENT_TIMESTAMP - INTERVAL '24 HOURS')
               AS volume_24h,
       (SELECT COUNT(1) FROM addresses WHERE TO_TIMESTAMP(last_appearance_time) > CURRENT_TIMESTAMP - INTERVAL '24 HOURS')
               AS total_addresses_24h,
       (SELECT AVG(total_gas_cost)::varchar FROM transactions
        WHERE checkpoint_sequence_number=(SELECT MAX(sequence_number) FROM checkpoints WHERE system_tick=false ) AND total_gas_cost > 0)
               AS current_gas;

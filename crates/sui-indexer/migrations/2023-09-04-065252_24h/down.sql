-- This file should undo anything in `up.sql`

DROP VIEW IF EXISTS network_overviews;
DROP INDEX IF EXISTS addresses_last_appearance_time;
ALTER TABLE checkpoints DROP COLUMN IF EXISTS total_transact_obc,
      DROP COLUMN IF EXISTS system_tick;
ALTER TABLE transactions DROP COLUMN IF EXISTS transact_obc;

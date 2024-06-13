-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS price_history;
DROP MATERIALIZED view IF EXISTS mining_nfts_view;
DROP TABLE IF EXISTS mining_nfts;
DROP MATERIALIZED view IF EXISTS mining_nft_staking_view;
DROP TABLE IF EXISTS mining_nft_staking;
DROP TABLE IF EXISTS mining_nft_profits;
DROP TABLE IF EXISTS mining_nft_rewards;
DROP TABLE IF EXISTS mining_nft_liquidities;

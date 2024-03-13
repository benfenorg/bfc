-- Your SQL goes here
CREATE TABLE price_history
(
  ts      BIGINT        NOT NULL,
  coin    VARCHAR(200)  NOT NULL,
  price   BIGINT        NOT NULL,
  CONSTRAINT price_history_pk PRIMARY KEY (ts, coin)
);


CREATE TABLE mining_nfts
(
  id                        BIGSERIAL     PRIMARY KEY,
  owner                     address       NOT NULL,
  miner_id                  address       NOT NULL,
  miner_url                 VARCHAR       NOT NULL,
  miner_name                VARCHAR       NOT NULL,
  token_id                  VARCHAR       NOT NULL,
  power                     BIGINT        NOT NULL,
  mint_at                   BIGINT        NOT NULL,
  earliest_held_at          BIGINT        NOT NULL,
  -- Total mint duration in the past staking.
  mint_duration             BIGINT        NOT NULL,
  mining_ticket_id          VARCHAR       NULL,
  -- When is the mining started at of current staking.
  mining_started_at         BIGINT        NOT NULL,
  total_mint_bfc            BIGINT        NOT NULL,
  yesterday_mint_bfc        BIGINT        NOT NULL,
  yesterday_dt_ms           BIGINT        NOT NULL,
  miner_redeem              BOOLEAN       NOT NULL,
  transfered_at             BIGINT        NOT NULL
);
CREATE UNIQUE INDEX mining_nfts_owner_miner_id_uniq ON mining_nfts (owner, miner_id);
CREATE INDEX mining_nfts_yesterday_dt_ms ON mining_nfts (yesterday_dt_ms);
CREATE INDEX mining_nfts_transfered_at ON mining_nfts (transfered_at);


CREATE TABLE mining_nft_staking
(
  ticket_id                 address       PRIMARY KEY,
  owner                     address       NOT NULL,
  miner_id                  address       NOT NULL,
  staked_at                 BIGINT        NOT NULL,
  unstaked_at               BIGINT        NULL,
  total_mint_bfc            BIGINT        NOT NULL
);

CREATE INDEX mining_nft_staking_miner_id_index ON mining_nft_staking(miner_id);
CREATE INDEX mining_nft_staking_owner_index ON mining_nft_staking(owner);

CREATE TABLE mining_nft_history_profits
(
  owner                     address       NOT NULL,
  miner_id                  address       NOT NULL,
  dt_timestamp_ms           BIGINT        NOT NULL,
  mint_bfc                  BIGINT        NOT NULL,
  mint_usd                  BIGINT        NOT NULL,
  pending_reward            BIGINT        NOT NULL,
  claimed_reward            BIGINT        NOT NULL,
  CONSTRAINT mining_nft_hisotry_profits_pk PRIMARY KEY (owner, dt_timestamp_ms, miner_id)
);

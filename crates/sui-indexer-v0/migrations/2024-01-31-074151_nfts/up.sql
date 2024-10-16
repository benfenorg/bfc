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
  cost_bfc                  BIGINT        NOT NULL,
  mint_duration             BIGINT        NOT NULL,
  mining_ticket_id          VARCHAR       NULL,
  -- When is the mining started at of current staking.
  mining_started_at         BIGINT        NOT NULL,
  total_mint_bfc            BIGINT        NOT NULL,
  yesterday_mint_bfc        BIGINT        NOT NULL,
  yesterday_dt_ms           BIGINT        NOT NULL,
  miner_redeem              BOOLEAN       NOT NULL,
  transfered_at             BIGINT        NOT NULL,
  sequence_number           BIGINT        NOT NULL
);
CREATE INDEX mining_nfts_owner_miner_id_uniq ON mining_nfts (owner, miner_id);
CREATE INDEX mining_nfts_yesterday_dt_ms ON mining_nfts (yesterday_dt_ms);
CREATE INDEX mining_nfts_transfered_at ON mining_nfts (transfered_at);

CREATE MATERIALIZED VIEW mining_nfts_view as
select * from mining_nfts where id in (SELECT max(id) FROM mining_nfts group by owner, miner_id);

CREATE TABLE mining_nft_staking
(
  id                        BIGSERIAL     PRIMARY KEY,
  ticket_id                 address       NOT NULL,
  owner                     address       NOT NULL,
  miner_id                  address       NOT NULL,
  staked_at                 BIGINT        NOT NULL,
  unstaked_at               BIGINT        NULL,
  total_mint_bfc            BIGINT        NOT NULL,
  sequence_number           BIGINT        NOT NULL
);

CREATE INDEX mining_nft_staking_miner_id_index ON mining_nft_staking(miner_id);
CREATE INDEX mining_nft_staking_owner_index ON mining_nft_staking(owner);
CREATE INDEX mining_nft_staking_ticket_id_index ON mining_nft_staking(ticket_id);

CREATE MATERIALIZED VIEW mining_nft_staking_view as
select * from mining_nft_staking where id in (SELECT max(id) FROM mining_nft_staking group by ticket_id);

CREATE TABLE mining_nft_history_profits
(
  owner                     address       NOT NULL,
  miner_id                  address       NOT NULL,
  dt_timestamp_ms           BIGINT        NOT NULL,
  mint_bfc                  BIGINT        NOT NULL,
  mint_usd                  BIGINT        NOT NULL,
  cost_bfc                  BIGINT        NOT NULL,
  pending_reward            BIGINT        NOT NULL,
  claimed_reward            BIGINT        NOT NULL,
  CONSTRAINT mining_nft_hisotry_profits_pk PRIMARY KEY (owner, dt_timestamp_ms, miner_id)
);

CREATE TABLE mining_nft_liquidities
(
    transaction_digest        base58digest  PRIMARY KEY,
    base_coin                 VARCHAR       NOT NULL,
    quote_coin                VARCHAR       NOT NULL,
    base_price_gte            BIGINT        NOT NULL,
    base_price_lte            BIGINT        NOT NULL,
    base_amount               BIGINT        NOT NULL,
    quote_amount              BIGINT        NOT NULL,
    timestamp_ms              BIGINT        NOT NULL
);
CREATE INDEX mining_nft_liquidities_base_coin_index ON mining_nft_liquidities(base_coin);
CREATE INDEX mining_nft_liquidities_quote_coin_index ON mining_nft_liquidities(quote_coin);
CREATE INDEX mining_nft_liquidities_timestamp_ms_index ON mining_nft_liquidities(timestamp_ms);
CREATE TABLE mint_long_coin
 (
   id                          BIGSERIAL     PRIMARY KEY,
   transaction_digest          base58digest  NOT NULL,
   amount                      BIGINT        NOT NULL,
   timestamp_ms                BIGINT        NOT NULL,
   UNIQUE (transaction_digest)
  );
-- Your SQL goes here
CREATE TABLE epoch_stake_coins
(
  epoch             BIGINT    NOT NULL,
  coin_type         VARCHAR   NOT NULL,
  coin_balance      BIGINT    NOT NULL,
  bfc_value         BIGINT    NOT NULL,
  stable_rate       BIGINT,
  CONSTRAINT epoch_stake_coins_pk PRIMARY KEY (epoch, coin_type)
);


CREATE TABLE epoch_stakes
(
  epoch                     BIGINT          PRIMARY KEY,
  -- Total stake in BFC, including stablecoins.
  total_stake               BIGINT          NOT NULL,
  -- Total reward in BFC(reward are always pay in BFC).
  total_reward              BIGINT          NOT NULL,
  -- The accumulated_* of the previous epoch plus the total_* of the current epoch.
  accumulated_reward        BIGINT          NOT NULL,
  -- The average exchange rate * 10_000.
  avg_exchange_rate         BIGINT          NOT NULL,
  apy                       BIGINT          NOT NULL,
  last_epoch_reward         BIGINT          NOT NULL,
  last_epoch_stake          BIGINT          NOT NULL
);


CREATE TABLE address_stakes
(
  -- The object_id of StakedBfc or StakedStable<Coin>.
  staked_object_id          address         PRIMARY KEY,
  staker_address            address         NOT NULL,
  pool_id                   address         NOT NULL,
  validator_address         address         NOT NULL,
  stake_coin                VARCHAR         NOT NULL,

  principal_epoch           BIGINT          NOT NULL,
  principal_amount          BIGINT          NOT NULL,
  principal_timestamp_ms    BIGINT          NOT NULL,

  estimated_reward          BIGINT          NOT NULL,
  estimated_at_epoch        BIGINT          NOT NULL,

  stake_activation_epoch    BIGINT          NOT NULL,

  unstaking_epoch           BIGINT,
  unstaking_amount          BIGINT,
  unstaking_digest          base58digest,
  unstaking_timestamp_ms    BIGINT,
  unstaking_reward_amount   BIGINT,

  timestamp_ms              BIGINT          NOT NULL
);

CREATE INDEX address_stakes_address_index ON address_stakes(staker_address);
CREATE INDEX address_stakes_unstaking_epoch_index ON address_stakes(unstaking_epoch);
CREATE INDEX address_stakes_principal_timestamp_ms ON address_stakes(principal_timestamp_ms);

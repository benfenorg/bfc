CREATE TABLE stake_reward_detail
(
  -- The object_id of StakedBfc or StakedStable<Coin>.
  id                        BIGSERIAL          PRIMARY KEY,
  staked_object_id          address         NOT NULL,
  staker_address            address         NOT NULL,
  pool_id                   address         NOT NULL,
  validator_address         address         NOT NULL,
  stake_coin                VARCHAR         NOT NULL,

  principal_epoch           BIGINT          NOT NULL,
  principal_amount          BIGINT          NOT NULL,
  principal_amount_bfc      BIGINT          NOT NULL,
  principal_timestamp_ms    BIGINT          NOT NULL,

  estimated_reward          BIGINT          NOT NULL,
  estimated_at_epoch        BIGINT          NOT NULL,

  stake_activation_epoch    BIGINT          NOT NULL,
  timestamp_ms              BIGINT          NOT NULL,
  constraint stake_reward_detail_unique unique (staked_object_id, estimated_at_epoch)
);

create index stake_reward_detail_index on stake_reward_detail (staker_address asc, estimated_at_epoch desc);

CREATE TABLE stake_reward_summary
(
  id                        BIGSERIAL          PRIMARY KEY,
  staker_address            address         NOT NULL,

  stake_amount           BIGINT          NOT NULL,
  stake_reward          BIGINT          NOT NULL,
  estimated_at_epoch    BIGINT          NOT NULL,
  principal_timestamp_ms    BIGINT          NOT NULL,
  timestamp_ms              BIGINT          NOT NULL,
  constraint stake_reward_summary_unique unique (staker_address, estimated_at_epoch)
);

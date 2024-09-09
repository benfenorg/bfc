CREATE TABLE stake_pending_item
 (
   id                         BIGSERIAL       PRIMARY KEY,
   owner                      address         NOT NULL,
   miner_id                   address         NOT NULL,
   ticket_id                  address         NOT NULL,
   debt                       BIGINT          NOT NULL
  );
CREATE INDEX stake_pending_item_ticket_id ON stake_pending_item (ticket_id);
CREATE INDEX stake_pending_item_owner ON stake_pending_item (owner);
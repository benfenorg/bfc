CREATE TABLE anonymous_coin
(
    id                         BIGSERIAL       PRIMARY KEY,
    owner                      address         NOT NULL,
    object_id                  address         NOT NULL,
    bcs_str                     VARCHAR         NOT NULL
);
CREATE INDEX anonymous_coin_owner_object_id ON anonymous_coin (owner, object_id, bcs_str);
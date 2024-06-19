-- Your SQL goes here

CREATE TABLE dao_proposals (
  object_id       address   PRIMARY KEY,
  action_id       BIGINT    NOT NULL,
  action_name     TEXT      NOT NULL,
  action_status   BOOLEAN   NOT NULL,
  pid             BIGINT    NOT NULL,
  proposer        address   NOT NULL,
  start_time      BIGINT    NOT NULL,
  end_time        BIGINT    NOT NULL,
  for_votes       BIGINT    NOT NULL,
  against_votes   BIGINT    NOT NULL,
  eta             BIGINT    NOT NULL,
  action_delay    BIGINT    NOT NULL,
  quorum_votes    BIGINT    NOT NULL,
  "state"         SMALLINT  NOT NULL,
  description     TEXT      NOT NULL
);

CREATE UNIQUE INDEX dao_proposals_pid_action_id_uniq ON dao_proposals(pid, action_id);
CREATE INDEX dao_proposals_proposer ON dao_proposals(proposer);

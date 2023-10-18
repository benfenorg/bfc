use diesel::prelude::*;

use sui_types::proposal;

use crate::errors::IndexerError;

use super::objects::Object;
use crate::schema::dao_proposals::{self};

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = dao_proposals)]
pub struct Proposal {
    pub object_id: String,
    pub action_id: i64,
    pub action_name: String,
    pub pid: i64,
    pub proposer: String,
    pub start_time: i64,
    pub end_time: i64,
    pub for_votes: i64,
    pub against_votes: i64,
    pub eta: i64,
    pub action_delay: i64,
    pub quorum_votes: i64,
    pub state: i16,
}

const PROPOSAL_OBJECT_TYPE: &'static str =
    "0x00000000000000000000000000000000000000000000000000000000000000c8::bfc_dao::Proposal";

impl Proposal {
    pub fn is_proposal(value: &Object) -> bool {
        value.object_type == PROPOSAL_OBJECT_TYPE
    }
}

impl TryFrom<Object> for Proposal {
    type Error = IndexerError;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        if !Self::is_proposal(&value) {
            return Err(IndexerError::NotSupportedError(format!(
                "object is not a proposal"
            )));
        }
        value.bcs.iter().for_each(|x| {
            tracing::info!("object bcs key: {}", x.0);
        });
        let byts = value.bcs.iter().filter(|x| x.0 == "object").last();
        if let Some(got) = byts {
            let p: proposal::Proposal = bcs::from_bytes(&got.1)?;
            let val = p.proposal;
            Ok(Proposal {
                object_id: value.object_id,
                action_id: val.action.action_id as i64,
                action_name: val.action.name,
                pid: val.pid as i64,
                proposer: val.proposer.bytes.to_string(),
                start_time: val.start_time as i64,
                end_time: val.end_time as i64,
                for_votes: val.for_votes as i64,
                against_votes: val.against_votes as i64,
                eta: val.eta as i64,
                action_delay: val.action_delay as i64,
                quorum_votes: val.quorum_votes as i64,
                state: 0,
            })
        } else {
            Err(IndexerError::InvalidArgumentError(format!(
                "no proposal in bcs"
            )))
        }
    }
}

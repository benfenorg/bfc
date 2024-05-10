use diesel::prelude::*;
use sui_types::dao;
use sui_types::{parse_sui_struct_tag, BFC_SYSTEM_ADDRESS};

use crate::errors::IndexerError;
use crate::models::events::Event;
use crate::schema::dao_votes::{self};

#[derive(Queryable, Insertable, Debug, Clone, Default)]
#[diesel(table_name = dao_votes)]
pub struct Vote {
    #[diesel(deserialize_as = i64)]
    pub id: Option<i64>,
    pub transaction_digest: String,
    pub sender: String,
    pub agree: bool,
    pub pid: i64,
    pub vote: i64,
    pub voter: String,
}

fn is_vote_changed_event(event_type: &str) -> Result<bool, IndexerError> {
    let event = parse_sui_struct_tag(event_type)?;
    // 0x00000000000000000000000000000000000000000000000000000000000000c8::bfc_dao::VoteChangedEvent
    Ok(event.address == BFC_SYSTEM_ADDRESS
        && event.module.as_str() == "bfc_dao"
        && event.name.as_str() == "VoteChangedEvent")
}

impl TryFrom<&Event> for Option<Vote> {
    type Error = IndexerError;

    fn try_from(value: &Event) -> Result<Self, Self::Error> {
        if !is_vote_changed_event(&value.event_type)? {
            return Ok(None);
        }

        let event: dao::VoteChangedEvent = bcs::from_bytes(&value.event_bcs)?;
        Ok(Some(Vote {
            id: None,
            pid: event.proposal_id as i64,
            transaction_digest: value.transaction_digest.clone(),
            sender: value.sender.clone(),
            agree: event.agree,
            vote: event.vote as i64,
            voter: event.voter.bytes.to_string(),
        }))
    }
}

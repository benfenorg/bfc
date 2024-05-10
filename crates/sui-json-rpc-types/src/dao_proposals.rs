use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sui_types::base_types::SuiAddress;

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum DaoProposalFilter {
    /// Filter the proposals that proposed by proposer.
    Proposer(SuiAddress),

    /// Filter the proposals that voted by voter.
    Voter(SuiAddress),
}

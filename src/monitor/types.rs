use sp_finality_grandpa::AuthorityId;
use serde::{Serialize, Deserialize};
use std::collections::BTreeSet;


#[derive(Debug, Serialize, Deserialize)] #[serde(rename_all = "camelCase")] 
pub struct Prevotes {
    pub current_weight: u32,
    pub missing: BTreeSet<AuthorityId>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Precommits {
    pub current_weight: u32,
    pub missing: BTreeSet<AuthorityId>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundState {
    pub round: u32,
    pub total_weight: u32,
    pub threshold_weight: u32,
    pub prevotes: Prevotes,
    pub precommits: Precommits,
}

/// The state of the current best round, as well as the background rounds in a
/// form suitable for serialization.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportedRoundStates {
    pub set_id: u32,
    pub best: RoundState,
    pub background: Vec<RoundState>,
}

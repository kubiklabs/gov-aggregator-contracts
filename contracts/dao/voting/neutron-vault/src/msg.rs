use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};
use cwd_interface::voting::InfoResponse;
use cwd_interface::voting::{
    BondingStatusResponse, TotalPowerAtHeightResponse, VotingPowerAtHeightResponse,
};
use cwd_macros::{info_query, voting_query, voting_vault, voting_vault_query};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct InstantiateMsg {
    /// Name contains the vault name which is used to ease the vault's recognition
    pub name: String,
    // Description contains information that characterizes the vault
    pub description: String,
    // Owner can update all configs including changing the owner. This will generally be a DAO
    pub owner: String,
    // Token denom e.g. untrn, or some ibc denom
    pub denom: String,
    // Chain-id of remote chain to query
    pub remote_chain_id: String,
    // ICQ helper contract address
    pub icq_helper: Addr,
}

#[voting_vault]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateVoter {
        remote_address: Addr,
    },
    UpdateConfig {
        name: String,
        description: String,
        owner: String,
    },
}

#[voting_query]
#[voting_vault_query]
#[info_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Config)]
    Config {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum IcqHelerQueryMsg {
    StakingValidators { query_id: u64 },
    // GovernmentProposals { query_id: u64 },
    GetDelegations { address: Addr },
    // GetRecipientTxs { recipient: String },
    GetDelegateTxs { delegator: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DelegatorDelegationsResponse {
    pub delegations: Vec<cosmwasm_std::Delegation>,
    pub last_submitted_local_height: u64,
}

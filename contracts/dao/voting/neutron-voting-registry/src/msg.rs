use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cwd_interface::voting::{
    InfoResponse, TotalPowerAtHeightResponse, VotingPowerAtHeightResponse,
};
use cwd_macros::{info_query, voting_query};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cwd_macros::voting_vault_query;
use cwd_interface::voting::BondingStatusResponse;
use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct InstantiateMsg {
    // Owner can update all configs including changing the owner. This will generally be a DAO.
    pub owner: String,
    // A list of addresses of relative voting vault contracts.
    pub voting_vaults: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddVotingVault { new_voting_vault_contract: String },
    RemoveVotingVault { old_voting_vault_contract: String },
    UpdateConfig { owner: String },
}

#[voting_query]
#[voting_vault_query]
#[info_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum VaultQueryMsg {
    #[returns(crate::state::Config)]
    Config {},
}
#[voting_query]
#[info_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Addr)]
    Dao {},
    #[returns(crate::state::Config)]
    Config {},
    #[returns(Vec<VotingVault>)]
    VotingVaults {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
pub struct VotingVault {
    pub address: String,
    pub name: String,
    pub description: String,
}

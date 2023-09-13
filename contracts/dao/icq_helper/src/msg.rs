use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterVotingPowerQuery {
        connection_id: String,
        update_period: u64,
        addr: String,
        valaddrs: Vec<String>,
    },
    RegisterNewDelegationQuery {
        connection_id: String,
        update_period: u64,
        // user_address: String,
        min_height: Option<u64>
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetDelegations { query_id: u64 },
    GetNewDelegations { address: String },
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NewDelegateResponse {
    pub delegations: Uint128,
    pub chain_id: String
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

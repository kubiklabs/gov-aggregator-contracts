use crate::state::{Transfer, Delegation};
use cosmwasm_std::Addr;
use neutron_sdk::bindings::types::KVKey;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    RegisterBalanceQuery {
        connection_id: String,
        update_period: u64,
        addr: String,
        denom: String,
    },
    RegisterDelegatorDelegationsQuery {
        delegator: String,
        validators: Vec<String>,
        connection_id: String,
        update_period: u64,
    },
    RegisterStakingValidatorsQuery {
        connection_id: String,
        update_period: u64,
        validators: Vec<String>,
    },
    RegisterGovernmentProposalsQuery {
        connection_id: String,
        proposals_ids: Vec<u64>,
        update_period: u64,
    },
    RegisterTransfersQuery {
        connection_id: String,
        update_period: u64,
        recipient: String,
        min_height: Option<u64>,
    },
    RegisterDelegationsQuery {
        connection_id: String,
        update_period: u64,
        delegator: String,
        min_height: Option<u64>,
    },
    UpdateInterchainQuery {
        query_id: u64,
        new_keys: Option<Vec<KVKey>>,
        new_update_period: Option<u64>,
        new_recipient: Option<String>,
    },
    RemoveInterchainQuery {
        query_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance { address: Addr },
    BankTotalSupply { query_id: u64 },
    DistributionFeePool { query_id: u64 },
    StakingValidators { query_id: u64 },
    GovernmentProposals { query_id: u64 },
    GetDelegations { address: Addr },
    GetRegisteredQuery { query_id: u64 },
    GetRecipientTxs { recipient: String },
    GetDelegateTxs { delegator: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetRecipientTxsResponse {
    pub transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetDelegateTxsResponse {
    pub delegations: Vec<Delegation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct MigrateMsg {}

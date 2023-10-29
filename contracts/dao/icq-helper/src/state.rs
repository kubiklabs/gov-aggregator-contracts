use cosmwasm_std::Uint128;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Recipient = str;

/// contains all transfers mapped by a recipient address observed by the contract.
pub const RECIPIENT_TXS: Map<&Recipient, Vec<Transfer>> = Map::new("recipient_txs");
/// contains number of transfers to addresses observed by the contract.
pub const TRANSFERS: Item<u64> = Item::new("transfers");
// Map of (proposal block,user address) -> staked
// It will store staked amount after after minimum block(we will use proposal block)
// pub const DELEGATE_AMOUNT: Map<(u64,String),Uint128> = Map::new("user_delegate_amount");
pub const DELEGATE_AMOUNT: Map<String,Uint128> = Map::new("user_delegate_amount");
pub const HEIGHT_FIELD: &str = "tx.height";
pub const DELEGATION_FIELD: &str = "message.action";
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Transfer {
    pub recipient: String,
    pub sender: String,
    pub denom: String,
    pub amount: String,
}
/// Protobuf type url of standard Cosmos SDK staking message message
pub const COSMOS_SDK_STAKING_MSG_URL: &str = "/cosmos.staking.v1beta1.delegations";

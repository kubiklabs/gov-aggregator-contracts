use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map, Deque};

pub type Recipient = str;

pub const BALANCE_QUERY_REPLY_ID: u64 = 0;
pub const DELEGATION_USER_QUERY_REPLY_ID: u64 = 1;
pub const TRANSFERS_TX_QUERY_REPLY_ID: u64 = 2;
pub const DELEGATIONS_TX_QUERY_REPLY_ID: u64 = 3;
// pub const VOTE_MODULE_UPDATE_REPLY_ID: u64 = 2;

/// maps remote address with interchain query_id
pub const BALANCE_QUERY_ID: Map<Addr, u64> = Map::new("balance_ids");
pub const DELEGATION_USER_QUERY_ID: Map<Addr, u64> = Map::new("delegation_user_ids");
pub const TRANSFERS_TX_QUERY_ID: Map<Addr, u64> = Map::new("transfers_tx_ids");
pub const DELEGATIONS_TX_QUERY_ID: Map<Addr, u64> = Map::new("delegations_tx_ids");

/// contains all transfers mapped by a recipient address observed by the contract.
pub const RECIPIENT_TXS: Map<&Recipient, Vec<Transfer>> = Map::new("recipient_txs");
/// contains number of transfers to addresses observed by the contract.
pub const TRANSFERS: Item<u64> = Item::new("transfers");

/// contains all delegations mapped by a recipient address observed by the contract.
pub const DELEGATE_TXS: Map<&Recipient, Vec<Delegation>> = Map::new("delegate_txs");
/// contains number of delegations to addresses observed by the contract.
pub const DELEGATIONS: Item<u64> = Item::new("delegations");

/// Reply ID queues to map query id with metdata keys such as address
/// TODO: Not too sure here if reordering will happen
/// if multiple queries created in a single atomic txn
/// but works fine if only one query created per txn
pub const BALANCE_QUERY_QUEUE: Deque<Addr> = Deque::new("balance_query_queue");
pub const DELEGATION_USER_QUERY_QUEUE: Deque<Addr> = Deque::new("delegation_user_query_queue");
pub const TRANSFERS_TX_QUERY_QUEUE: Deque<Addr> = Deque::new("transfers_tx_query_queue");
pub const DELEGATIONS_TX_QUERY_QUEUE: Deque<Addr> = Deque::new("delegations_tx_query_queue");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Transfer {
    pub recipient: String,
    pub sender: String,
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Delegation {
    pub delegator: String,
    pub validator: String,
    pub denom: String,
    pub amount: String,
}

/// Struct fetched from query register reply
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ReplyId {
    pub id: u64,
}
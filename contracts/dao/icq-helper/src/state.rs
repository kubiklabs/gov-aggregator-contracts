use cosmwasm_std::{Storage, StdResult, to_vec, from_binary, Binary, Order};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type Recipient = str;

pub const BALANCE_QUERY_REPLY_ID: u64 = 0;
// pub const VOTE_MODULE_INSTANTIATE_REPLY_ID: u64 = 1;
// pub const VOTE_MODULE_UPDATE_REPLY_ID: u64 = 2;
// pub const ICA_HELPER_INSTANTIATE_REPLY_ID: u64 = 3;
// pub const ICQ_HELPER_INSTANTIATE_REPLY_ID: u64 = 4;

// pub const SUDO_PAYLOAD_REPLY_ID: u64 = 1;
// pub const REPLY_ID_STORAGE: Item<Vec<u8>> = Item::new("reply_queue_id");
// pub const SUDO_PAYLOAD: Map<(String, u64), Vec<u8>> = Map::new("sudo_payload");
// pub const ERRORS_QUEUE: Map<u32, String> = Map::new("errors_queue");
// pub const ACKNOWLEDGEMENT_RESULTS: Map<(String, u64), AcknowledgementResult> =
//     Map::new("acknowledgement_results");

/// contains all transfers mapped by a recipient address observed by the contract.
pub const RECIPIENT_TXS: Map<&Recipient, Vec<Transfer>> = Map::new("recipient_txs");
/// contains number of transfers to addresses observed by the contract.
pub const TRANSFERS: Item<u64> = Item::new("transfers");

// /// SudoPayload is a type that stores information about a transaction that we try to execute
// /// on the host chain. This is a type introduced for our convenience.
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// pub struct SudoPayload {
//     pub message: String,
//     // pub port_id: String,
// }

// /// Serves for storing acknowledgement calls for interchain transactions
// #[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum AcknowledgementResult {
//     /// Success - Got success acknowledgement in sudo with array of message item types in it
//     Success(Vec<String>),
//     /// Error - Got error acknowledgement in sudo with payload message in it and error details
//     Error((String, String)),
//     /// Timeout - Got timeout acknowledgement in sudo with payload message in it
//     Timeout(String),
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Transfer {
    pub recipient: String,
    pub sender: String,
    pub denom: String,
    pub amount: String,
}

// pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<()> {
//     REPLY_ID_STORAGE.save(store, &to_vec(&payload)?)
// }

// pub fn read_sudo_payload(
//     store: &mut dyn Storage,
//     channel_id: String,
//     seq_id: u64,
// ) -> StdResult<SudoPayload> {
//     let data = SUDO_PAYLOAD.load(store, (channel_id, seq_id))?;
//     from_binary(&Binary(data))
// }

// pub fn add_error_to_queue(store: &mut dyn Storage, error_msg: String) -> Option<()> {
//     let result = ERRORS_QUEUE
//         .keys(store, None, None, Order::Descending)
//         .next()
//         .and_then(|data| data.ok())
//         .map(|c| c + 1)
//         .or(Some(0));

//     result.and_then(|idx| ERRORS_QUEUE.save(store, idx, &error_msg).ok())
// }

/// a struct into which to decode the thing
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct ReplyId {
    pub id: u64,
    // add the other fields if you need them
}
use cosmwasm_std::{StdResult, Storage, Addr};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{ Item, Map};

pub const CHAIN: Map<String,String> = Map::new("state");
pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
}

pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    CONFIG.save(storage, config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Chain {
//     pub global_index: Decimal,
//     pub total_balance: Uint128,
//     // decrease when user claim their reward
//     pub prev_reward_balance: Uint128,
// }

// pub fn store_state(storage: &mut dyn Storage, state: &State) -> StdResult<()> {
//     STATE.save(storage, state)
// }

// pub fn read_state(storage: &dyn Storage) -> StdResult<State> {
//     STATE.load(storage)
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct Holder {
//     pub balance: Uint128,
//     pub index: Decimal,
//     pub pending_rewards: Decimal,
//     pub is_whitelisted: bool,
// }

// // This is similar to HashMap<holder's address, Hodler>
// pub fn store_holder(
//     storage: &mut dyn Storage,
//     holder_address: &CanonicalAddr,
//     holder: &Holder,
// ) -> StdResult<()> {
//     HOLDERS.save(storage, holder_address.as_slice(), holder)
// }

// pub fn read_holder(storage: &dyn Storage, holder_address: &CanonicalAddr) -> StdResult<Holder> {
//     let res = HOLDERS.may_load(storage, holder_address.as_slice())?;
//     match res {
//         Some(holder) => Ok(holder),
//         None => Ok(Holder {
//             balance: Uint128::zero(),
//             index: Decimal::zero(),
//             pending_rewards: Decimal::zero(),
//             is_whitelisted: false,
//         }),
//     }
// }

// pub fn query_holder(deps: Deps, address: String) -> StdResult<HolderResponse> {
//     let holder: Holder = read_holder(deps.storage, &deps.api.addr_canonicalize(&address)?)?;
//     Ok(HolderResponse {
//         address,
//         balance: holder.balance,
//         index: holder.index,
//         pending_rewards: holder.pending_rewards,
//         is_whitelisted: holder.is_whitelisted,
//     })
// }

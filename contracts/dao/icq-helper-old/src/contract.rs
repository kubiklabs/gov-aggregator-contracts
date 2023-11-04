use std::error::Error;
use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgDelegate;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxRaw, TxBody};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Uint128, StdError,
};
use cw2::set_contract_version;
use neutron_sdk::bindings::types::Height;
use neutron_sdk::interchain_queries::get_registered_query;
use neutron_sdk::interchain_queries::types::{
    TransactionFilterItem, TransactionFilterOp, TransactionFilterValue, QueryPayload,
};

use prost::Message;

use crate::msg::{
    ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, NewDelegateResponse
};
use crate::state::{
    DELEGATE_AMOUNT, DELEGATION_FIELD, HEIGHT_FIELD, COSMOS_SDK_STAKING_MSG_URL,
};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::{NeutronQuery, QueryRegisteredQueryResponse};
use neutron_sdk::interchain_queries::v045::queries::{query_delegations, query_balance};
use neutron_sdk::interchain_queries::v045::{new_register_delegator_delegations_query_msg, new_register_balance_query_msg};
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_sdk::{NeutronResult, NeutronError};

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResult<Response> {
    // deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut<NeutronQuery>,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::RegisterStakedAmountTotal {
            connection_id,
            update_period,
        } => register_staked_amount_total(connection_id, update_period),
        ExecuteMsg::RegisterBalanceAmountUser {
            address,
            connection_id,
            update_period,
        } => register_balance_amount_user(connection_id, address, update_period),
        ExecuteMsg::RegisterNewDelegationQuery {
            connection_id,
            update_period,
            // user_address,
            min_height,
        } => register_stake_query_msg(connection_id, update_period,min_height),
        ExecuteMsg::RemoveInterchainQuery { query_id } => remove_interchain_query(query_id),
    }
}

pub fn register_balance_amount_user(
    connection_id: String,
    address: String,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_balance_query_msg(
        connection_id,
        address,
        "uatom".to_owned(),  // TODO: move to params
        update_period,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn register_staked_amount_total(
    _connection_id: String,
    _update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    // let msg = new_register_delegator_delegations_query_msg(
    //     connection_id,
    //     addr,
    //     valaddrs,
    //     update_period,
    // )?;
    
    Ok(Response::new())
}

// till here you have register query
pub fn register_voting_power_query(
    connection_id: String,
    addr: String,
    valaddrs: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_delegator_delegations_query_msg(
        connection_id,
        addr,
        valaddrs,
        update_period,
    )?;
    
    Ok(Response::new().add_message(msg))
}

pub fn register_stake_query_msg(
    connection_id: String,
    update_period: u64,
    // user_address: String,
    min_height: Option<u64>
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_stake_query_msg(
        connection_id,
        update_period,
        min_height,
    )?;

    Ok(Response::new().add_message(msg))
}

pub fn remove_interchain_query(
    query_id: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let remove_msg = NeutronMsg::remove_interchain_query(query_id);
    Ok(Response::new().add_message(remove_msg))
}

// Not required as such, we won't query specifically
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps<NeutronQuery>,
    env: Env,
    msg: QueryMsg,
) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::GetBalance { query_id } => {
            Ok(to_binary(&query_balance(deps, env, query_id)?)?)
        }
        QueryMsg::GetDelegations { query_id } => {
            Ok(to_binary(&query_delegations(deps, env, query_id)?)?)
        }
        QueryMsg::GetNewDelegations { address } => {
            Ok(to_binary(&query_new_delegations(deps, env, address)?)?)
        }
    }
}

pub fn query_new_delegations(
    deps: Deps<NeutronQuery>,
    _env: Env,
    address: String
) -> NeutronResult<NewDelegateResponse> {
    let stored_delegate = DELEGATE_AMOUNT.load(deps.storage,address)?;

    Ok(NewDelegateResponse {
        delegations: stored_delegate,
        chain_id: "cosmos-4".to_string()
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> StdResult<Response> {
    // deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}

#[entry_point]
pub fn sudo(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    msg: SudoMsg,
) -> NeutronResult<Response> {
    match msg {
        // For handling tx query result
        SudoMsg::TxQueryResult {
            query_id,
            height,
            data,
        } => sudo_tx_query_result(deps, env, query_id, height, data),

        // For handling kv query result
        SudoMsg::KVQueryResult { query_id } => sudo_kv_query_result(deps, env, query_id),
        _ => Ok(Response::default()),
    }
}

// /// sudo_check_tx_query_result is an example callback for transaction query results that stores the
// /// deposits received as a result on the registered query in the contract's state.
pub fn sudo_tx_query_result(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    query_id: u64,
    _height: Height,
    data: Binary,
) -> NeutronResult<Response> {
    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(data.as_slice())?;
    let body: TxBody = TxBody::decode(tx.body_bytes.as_slice())?;

    // Get the registered query by ID and retrieve the raw query string
    let registered_query: QueryRegisteredQueryResponse =
        get_registered_query(deps.as_ref(), query_id)?;
    let transactions_filter = registered_query.registered_query.transactions_filter;

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.
    //
    // TODO: come up with solution to determine transactions filter type
    match registered_query.registered_query.query_type {
        _ => {
            // For transfer queries, query data looks like `[{"field:"message.action", "op":"eq", "value":"delegate"}]`
            // it i
            // let query_data: Vec<TransactionFilterItem> =
            //     serde_json_wasm::from_str(transactions_filter.as_str())?;

            // let recipient = query_data
            //     .iter()
            //     .find(|x| x.field == DELEGATION_KEY.to_string() && x.op == TransactionFilterOp::Eq)
            //     .map(|x| match &x.value {
            //         TransactionFilterValue::String(v) => v.as_str(),
            //         _ => "",
            //     })
            //     .unwrap_or("");
            // Just parse the data in this struct format MsgDelegate and store the address
            // let delegate = recipient_delegate_from_tx_body(body)?;
            const MAX_ALLOWED_MESSAGES: usize = 5; // should be hell lot, think about it
            for msg in body.messages.iter().take(MAX_ALLOWED_MESSAGES) {
                // Skip all messages in this transaction that are not delegate messages.
                if msg.type_url != *COSMOS_SDK_STAKING_MSG_URL.to_string() {
                    continue;
                }
                // Parse a Send message and check that it has the required recipient.
                // let transfer_msg: MsgSend = MsgSend::decode(msg.value.as_slice())?;
                let delegate_msg: MsgDelegate = MsgDelegate::decode(msg.value.as_slice())?;
                // if delegate_msg.delegator_address == recipient {
                //     // check for validator if you want
                //     for coin in delegate_msg.amount {
                //         delegate_amount += Uint128::new(coin.amount.parse::<u128>().unwrap());
                //     }
                // }

                let mut stored_delegates = DELEGATE_AMOUNT.load(deps.storage, delegate_msg.delegator_address.to_string())?;
                let mut delegate_amount: Uint128 = Uint128::new(0);
                for coin in delegate_msg.amount {
                    delegate_amount += Uint128::new(coin.amount.parse::<u128>().unwrap());
                }
                stored_delegates += delegate_amount;
                
                DELEGATE_AMOUNT.save(deps.storage, delegate_msg.delegator_address, &stored_delegates)?;
            }
            Ok(Response::new())
        }
    }
}

/// parses tx body and retrieves transactions to the given recipient.
fn recipient_delegate_from_tx_body(
    tx_body: TxBody,
    recipient: &str,
) -> NeutronResult<Uint128> {
    let mut delegate_amount: Uint128 = Uint128::new(0);
    // Only handle up to MAX_ALLOWED_MESSAGES messages, everything else
    // will be ignored to prevent 'out of gas' conditions.
    // Note: in real contracts you will have to somehow save ignored
    // data in order to handle it later.
    const MAX_ALLOWED_MESSAGES: usize = 5;
    for msg in tx_body.messages.iter().take(MAX_ALLOWED_MESSAGES) {
        // Skip all messages in this transaction that are not Send messages.
        if msg.type_url != *COSMOS_SDK_STAKING_MSG_URL.to_string() {
            continue;
        }

        // Parse a Send message and check that it has the required recipient.
        // let transfer_msg: MsgSend = MsgSend::decode(msg.value.as_slice())?;
        let delegate_msg: MsgDelegate = MsgDelegate::decode(msg.value.as_slice())?;
        if delegate_msg.delegator_address == recipient {
            // check for validator if you want
            for coin in delegate_msg.amount {
                delegate_amount += Uint128::new(coin.amount.parse::<u128>().unwrap());
            }
        }
    }
    Ok(delegate_amount)
}

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    query_id: u64,
) -> NeutronResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    let delegations = query_delegations(
        deps.as_ref(),
        env,
        query_id,
    )?;

    let mut total_staked:Uint128 = Uint128::new(0);

    for delegation in delegations.delegations {
        total_staked += delegation.amount.amount
    }
    // create a msg to send to main contract and rest will be handled from there
    /////////////////////////////////
    /////////////////////////////////
    //////SAVE IT IN THE CONTRACT STORAGE FROM WHERE DAO WILL QUERY
    /////////////////////////////////
    /////////////////////////////////
    /////////////////////////////////

    // TODO: provide an actual example. Currently to many things are going to change
    // after @pro0n00gler's PRs to implement this.

    Ok(Response::default())
}

pub fn new_register_stake_query_msg(
    connection_id: String,
    update_period: u64,
    // user_address: String,
    min_height: Option<u64>,
) -> NeutronResult<NeutronMsg> {
    let mut query_data = vec![TransactionFilterItem {
        field: DELEGATION_FIELD.to_string(),
        op: TransactionFilterOp::Eq,
        value: TransactionFilterValue::String("delegate".to_string()),
    }];
    if let Some(min_height) = min_height {
        query_data.push(TransactionFilterItem {
            field: HEIGHT_FIELD.to_string(),
            op: TransactionFilterOp::Gte,
            value: TransactionFilterValue::Int(min_height),
        })
    }

    NeutronMsg::register_interchain_query(
        QueryPayload::TX(query_data),
        connection_id,
        update_period,
    )
}
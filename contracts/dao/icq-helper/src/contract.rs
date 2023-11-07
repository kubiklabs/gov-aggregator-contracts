use cosmos_sdk_proto::cosmos::bank::v1beta1::MsgSend;
use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgDelegate;
use cosmos_sdk_proto::cosmos::tx::v1beta1::{TxBody, TxRaw};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, SubMsg, Reply, Addr,
};
use cw2::set_contract_version;
use cw_utils::ParseReplyError;
use neutron_sdk::interchain_queries::v045::register_queries::new_register_delegate_query_msg;
use crate::error::ContractError;
use prost::Message as ProstMessage;
// use neutron_sdk::interchain_txs::helpers::{
//     decode_acknowledgement_response, decode_message_response
// };
// use neutron_sdk::interchain_txs::helpers::get_port_id;

use crate::msg::{ExecuteMsg, GetRecipientTxsResponse, InstantiateMsg, MigrateMsg, QueryMsg, GetDelegateTxsResponse};
use crate::state::{
    ReplyId, BALANCE_QUERY_ID, DELEGATIONS_TX_QUERY_QUEUE,
    Transfer, RECIPIENT_TXS, TRANSFERS,BALANCE_QUERY_REPLY_ID, BALANCE_QUERY_QUEUE,
    DELEGATION_USER_QUERY_REPLY_ID, DELEGATION_USER_QUERY_QUEUE, DELEGATION_USER_QUERY_ID,
    TRANSFERS_TX_QUERY_REPLY_ID, TRANSFERS_TX_QUERY_QUEUE, DELEGATIONS_TX_QUERY_REPLY_ID, Delegation, DELEGATE_TXS,
};
use neutron_sdk::bindings::msg::NeutronMsg;
use neutron_sdk::bindings::query::NeutronQuery;
use neutron_sdk::bindings::types::{Height, KVKey};
use neutron_sdk::interchain_queries::get_registered_query;
use neutron_sdk::interchain_queries::v045::queries::{
    query_balance, query_bank_total, query_delegations, query_distribution_fee_pool,
    query_government_proposals, query_staking_validators,
};
use neutron_sdk::interchain_queries::v045::{
    new_register_balance_query_msg, new_register_bank_total_supply_query_msg,
    new_register_delegator_delegations_query_msg, new_register_distribution_fee_pool_query_msg,
    new_register_gov_proposal_query_msg, new_register_staking_validators_query_msg,
    new_register_transfers_query_msg,
};
use neutron_sdk::sudo::msg::SudoMsg;
use neutron_sdk::{NeutronError, NeutronResult};

use neutron_sdk::interchain_queries::types::{
    TransactionFilterItem, TransactionFilterOp, TransactionFilterValue,
};
use neutron_sdk::interchain_queries::v045::types::{COSMOS_SDK_TRANSFER_MSG_URL, RECIPIENT_FIELD, DELEGATOR_FIELD};
use serde_json_wasm;

/// defines the incoming transfers limit to make a case of failed callback possible.
const MAX_ALLOWED_TRANSFER: u64 = 20000;
const MAX_ALLOWED_MESSAGES: usize = 20;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: move to neutron-sdk fork
pub const COSMOS_SDK_DELEGATE_MSG_URL: &str = "/cosmos.staking.v1beta1.MsgDelegate";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResult<Response> {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResult<Response<NeutronMsg>> {
    match msg {
        ExecuteMsg::RegisterBalanceQuery {
            connection_id,
            addr,
            denom,
            update_period,
        } => register_balance_query(
            deps,
            env,
            connection_id,
            addr,
            denom,
            update_period,
        ),
        ExecuteMsg::RegisterGovernmentProposalsQuery {
            connection_id,
            proposals_ids,
            update_period,
        } => register_gov_proposal_query(connection_id, proposals_ids, update_period),
        ExecuteMsg::RegisterStakingValidatorsQuery {
            connection_id,
            validators,
            update_period,
        } => register_staking_validators_query(connection_id, validators, update_period),
        ExecuteMsg::RegisterDelegatorDelegationsQuery {
            connection_id,
            delegator,
            validators,
            update_period,
        } => register_delegations_query(
            deps,
            env,
            connection_id,
            delegator,
            validators,
            update_period,
        ),
        ExecuteMsg::RegisterTransfersQuery {
            connection_id,
            recipient,
            update_period,
            min_height,
        } => register_transfers_query(
            deps,
            env,
            connection_id,
            recipient,
            update_period,
            min_height,
        ),
        ExecuteMsg::RegisterDelegationsQuery {
            connection_id,
            delegator,
            update_period,
            min_height,
        } => register_delegations_tx_query(
            deps,
            env,
            connection_id,
            delegator,
            update_period,
            min_height,
        ),
        ExecuteMsg::UpdateInterchainQuery {
            query_id,
            new_keys,
            new_update_period,
            new_recipient,
        } => update_interchain_query(query_id, new_keys, new_update_period, new_recipient),
        ExecuteMsg::RemoveInterchainQuery { query_id } => remove_interchain_query(query_id),
    }
}

pub fn register_balance_query(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    connection_id: String,
    addr: String,
    denom: String,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_balance_query_msg(
        connection_id,
        addr.clone(),
        denom,
        update_period,
    )?;

    let sub_msg = SubMsg::reply_on_success(
        msg,
        BALANCE_QUERY_REPLY_ID,
    );
    let address = Addr::unchecked(addr);

    BALANCE_QUERY_QUEUE.push_back(deps.storage, &address)?;

    Ok(Response::new()
        .add_submessage(sub_msg))
}

pub fn register_bank_total_supply_query(
    connection_id: String,
    denoms: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_bank_total_supply_query_msg(connection_id, denoms, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_distribution_fee_pool_query(
    connection_id: String,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_distribution_fee_pool_query_msg(connection_id, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_gov_proposal_query(
    connection_id: String,
    proposals_ids: Vec<u64>,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_gov_proposal_query_msg(connection_id, proposals_ids, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_staking_validators_query(
    connection_id: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_staking_validators_query_msg(connection_id, validators, update_period)?;

    Ok(Response::new().add_message(msg))
}

pub fn register_delegations_query(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    connection_id: String,
    delegator: String,
    validators: Vec<String>,
    update_period: u64,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_delegator_delegations_query_msg(
        connection_id,
        delegator.clone(),
        validators,
        update_period,
    )?;

    let sub_msg = SubMsg::reply_on_success(
        msg,
        DELEGATION_USER_QUERY_REPLY_ID,
    );
    let address = Addr::unchecked(delegator);

    DELEGATION_USER_QUERY_QUEUE.push_back(deps.storage, &address)?;

    Ok(Response::new()
        .add_submessage(sub_msg))
}

pub fn register_transfers_query(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    connection_id: String,
    recipient: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_transfers_query_msg(
        connection_id,
        recipient.clone(),
        update_period,
        min_height,
    )?;

    let sub_msg = SubMsg::reply_on_success(
        msg,
        TRANSFERS_TX_QUERY_REPLY_ID,
    );
    let address = Addr::unchecked(recipient);

    TRANSFERS_TX_QUERY_QUEUE.push_back(deps.storage, &address)?;

    Ok(Response::new()
        .add_submessage(sub_msg))
}

pub fn register_delegations_tx_query(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    connection_id: String,
    delegator: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    let msg = new_register_delegate_query_msg(
        connection_id,
        delegator.clone(),
        update_period,
        min_height,
    )?;

    let sub_msg = SubMsg::reply_on_success(
        msg,
        DELEGATIONS_TX_QUERY_REPLY_ID,
    );
    let address = Addr::unchecked(delegator);

    DELEGATIONS_TX_QUERY_QUEUE.push_back(deps.storage, &address)?;

    Ok(Response::new()
        .add_submessage(sub_msg))
}

pub fn update_interchain_query(
    query_id: u64,
    new_keys: Option<Vec<KVKey>>,
    new_update_period: Option<u64>,
    new_recipient: Option<String>,
) -> NeutronResult<Response<NeutronMsg>> {
    let new_filter = new_recipient.map(|recipient| {
        vec![TransactionFilterItem {
            field: RECIPIENT_FIELD.to_string(),
            op: TransactionFilterOp::Eq,
            value: TransactionFilterValue::String(recipient),
        }]
    });

    let update_msg =
        NeutronMsg::update_interchain_query(query_id, new_keys, new_update_period, new_filter)?;
    Ok(Response::new().add_message(update_msg))
}

pub fn remove_interchain_query(query_id: u64) -> NeutronResult<Response<NeutronMsg>> {
    let remove_msg = NeutronMsg::remove_interchain_query(query_id);
    Ok(Response::new().add_message(remove_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        //TODO: check if query.result.height is too old (for all interchain queries)
        QueryMsg::Balance {
            address,
        } => query_address_balance(deps, env, address),
        QueryMsg::BankTotalSupply { query_id } => {
            Ok(to_binary(&query_bank_total(deps, env, query_id)?)?)
        }
        QueryMsg::DistributionFeePool { query_id } => Ok(to_binary(&query_distribution_fee_pool(
            deps, env, query_id,
        )?)?),
        QueryMsg::StakingValidators { query_id } => {
            Ok(to_binary(&query_staking_validators(deps, env, query_id)?)?)
        }
        QueryMsg::GovernmentProposals { query_id } => Ok(to_binary(&query_government_proposals(
            deps, env, query_id,
        )?)?),
        QueryMsg::GetDelegations {
            address,
        } => query_address_delegations(deps, env, address),
        QueryMsg::GetRegisteredQuery { query_id } => {
            Ok(to_binary(&get_registered_query(deps, query_id)?)?)
        }
        QueryMsg::GetRecipientTxs { recipient } => query_recipient_txs(deps, recipient),
        QueryMsg::GetDelegateTxs { delegator } => query_delegate_txs(deps, delegator),
    }
}

fn query_address_balance(
    deps: Deps<NeutronQuery>,
    env: Env,
    address: Addr,
) -> NeutronResult<Binary> {
    // get query_id from address for balance query
    let registered_query_id = BALANCE_QUERY_ID.load(deps.storage, address)?;

    let query_balance = query_balance(
        deps,
        env, 
        registered_query_id,
    )?;
    Ok(to_binary(&query_balance)?)
}

fn query_address_delegations(
    deps: Deps<NeutronQuery>,
    env: Env,
    address: Addr,
) -> NeutronResult<Binary> {
    // get query_id from address for balance query
    let registered_query_id = DELEGATION_USER_QUERY_ID.load(deps.storage, address)?;

    let query_delegation_user = query_delegations(
        deps,
        env, 
        registered_query_id,
    )?;
    Ok(to_binary(&query_delegation_user)?)
}

fn query_recipient_txs(
    deps: Deps<NeutronQuery>,
    recipient: String,
) -> NeutronResult<Binary> {
    let txs = RECIPIENT_TXS
        .load(deps.storage, &recipient)
        .unwrap_or_default();
    Ok(to_binary(&GetRecipientTxsResponse { transfers: txs })?)
}

fn query_delegate_txs(
    deps: Deps<NeutronQuery>,
    delegator: String,
) -> NeutronResult<Binary> {
    // TODO: replace with delegate txns
    let txs = DELEGATE_TXS
        .load(deps.storage, &delegator)
        .unwrap_or_default();
    Ok(to_binary(&GetDelegateTxsResponse { delegations: txs })?)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
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

/// sudo_check_tx_query_result is an example callback for transaction query results that stores the
/// deposits received as a result on the registered query in the contract's state.
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

    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_tx_query_result received; body: {:?}",
            body,
        )
        .as_str(),
    );

    // Get the registered query by ID and retrieve the raw query string
    let registered_query = get_registered_query(
        deps.as_ref(),
        query_id,
    )?;
    let transactions_filter = registered_query.registered_query.transactions_filter;

    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_tx_query_result received; transactions_filter: {:?}",
            transactions_filter,
        )
        .as_str(),
    );

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.
    //
    // TODO: come up with solution to determine transactions filter type
    match registered_query.registered_query.query_type {
        _ => {
            // For transfer queries, query data looks like
            // `[{"field:"transfer.recipient", "op":"eq", "value":"some_address"}]`
            let query_data: Vec<TransactionFilterItem> =
                serde_json_wasm::from_str(transactions_filter.as_str())?;

            deps.api.debug(
                format!(
                    "WASMDEBUG: sudo_tx_query_result received; query_data: {:?}",
                    query_data,
                )
                .as_str(),
            );

            let recipient = query_data
                .iter()
                .find(
                    |x| x.field == DELEGATOR_FIELD  // TODO: separate from transfer
                    && x.op == TransactionFilterOp::Eq
                )
                .map(|x| match &x.value {
                    TransactionFilterValue::String(v) => v.as_str(),
                    _ => "",
                })
                .unwrap_or("");

            let delegations = delegator_deposits_from_tx_body(body, recipient)?;
            // If we didn't find a Delegate message with the correct recipient, return an error, and
            // this query result will be rejected by Neutron: no data will be saved to state.
            if delegations.is_empty() {
                return Err(NeutronError::Std(StdError::generic_err(
                    "failed to find a matching Delegate transaction message",
                )));
            }

            let mut stored_delegations: u64 = TRANSFERS.load(deps.storage).unwrap_or_default();
            stored_delegations += delegations.len() as u64;
            TRANSFERS.save(deps.storage, &stored_delegations)?;

            check_delegations_size(&delegations)?;
            let mut stored_deposits: Vec<Delegation> = DELEGATE_TXS
                .load(deps.storage, recipient)
                .unwrap_or_default();
            stored_deposits.extend(delegations);
            DELEGATE_TXS.save(deps.storage, recipient, &stored_deposits)?;
            Ok(Response::new())
        }
    }
}

/// parses tx body and retrieves transactions to the given recipient.
fn recipient_deposits_from_tx_body(
    tx_body: TxBody,
    recipient: &str,
) -> NeutronResult<Vec<Transfer>> {
    let mut deposits: Vec<Transfer> = vec![];
    // Only handle up to MAX_ALLOWED_MESSAGES messages, everything else
    // will be ignored to prevent 'out of gas' conditions.
    // Note: in real contracts you will have to somehow save ignored
    // data in order to handle it later.
    for msg in tx_body.messages.iter().take(MAX_ALLOWED_MESSAGES) {
        // Skip all messages in this transaction that are not Send messages.
        if msg.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
            continue;
        }

        // Parse a Send message and check that it has the required recipient.
        let transfer_msg: MsgSend = MsgSend::decode(msg.value.as_slice())?;
        if transfer_msg.to_address == recipient {
            for coin in transfer_msg.amount {
                deposits.push(Transfer {
                    sender: transfer_msg.from_address.clone(),
                    amount: coin.amount.clone(),
                    denom: coin.denom,
                    recipient: recipient.to_string(),
                });
            }
        }
    }
    Ok(deposits)
}

/// parses tx body and retrieves transactions to the given delegator.
fn delegator_deposits_from_tx_body(
    tx_body: TxBody,
    delegator: &str,
) -> NeutronResult<Vec<Delegation>> {
    let mut delegations: Vec<Delegation> = vec![];
    // Only handle up to MAX_ALLOWED_MESSAGES messages, everything else
    // will be ignored to prevent 'out of gas' conditions.
    // Note: in real contracts you will have to somehow save ignored
    // data in order to handle it later.
    for msg in tx_body.messages.iter().take(MAX_ALLOWED_MESSAGES) {
        // Skip all messages in this transaction that are not Delegate messages.
        if msg.type_url != *COSMOS_SDK_DELEGATE_MSG_URL.to_string() {
            continue;
        }

        // Parse a Delegate message and check that it has the required delegator.
        let delegate_msg: MsgDelegate = MsgDelegate::decode(msg.value.as_slice())?;
        if delegate_msg.delegator_address == delegator {
            while let Some(ref coin) = delegate_msg.amount {
                delegations.push(Delegation {
                    delegator: delegate_msg.delegator_address.clone(),
                    amount: coin.amount.clone(),
                    denom: coin.denom.clone(),
                    validator: delegate_msg.validator_address.clone(),
                });
            }
        }
    }
    Ok(delegations)
}

// checks whether there are deposits that are greater then MAX_ALLOWED_TRANSFER.
fn check_deposits_size(deposits: &Vec<Transfer>) -> StdResult<()> {
    for deposit in deposits {
        match deposit.amount.parse::<u64>() {
            Ok(amount) => {
                if amount > MAX_ALLOWED_TRANSFER {
                    return Err(StdError::generic_err(format!(
                        "maximum allowed transfer is {}",
                        MAX_ALLOWED_TRANSFER
                    )));
                };
            }
            Err(error) => {
                return Err(StdError::generic_err(format!(
                    "failed to cast transfer amount to u64: {}",
                    error
                )));
            }
        };
    }
    Ok(())
}

// checks whether there are delegations that are greater then MAX_ALLOWED_TRANSFER.
fn check_delegations_size(delegations: &Vec<Delegation>) -> StdResult<()> {
    for delegation in delegations {
        match delegation.amount.parse::<u64>() {
            Ok(amount) => {
                if amount > MAX_ALLOWED_TRANSFER {
                    return Err(StdError::generic_err(format!(
                        "maximum allowed delegation is {}",
                        MAX_ALLOWED_TRANSFER
                    )));
                };
            }
            Err(error) => {
                return Err(StdError::generic_err(format!(
                    "failed to cast delegation amount to u64: {}",
                    error
                )));
            }
        };
    }
    Ok(())
}

/// sudo_kv_query_result is the contract's callback for KV query results. Note that only the query
/// id is provided, so you need to read the query result from the state.
pub fn sudo_kv_query_result(
    deps: DepsMut<NeutronQuery>,
    _env: Env,
    query_id: u64,
) -> NeutronResult<Response> {
    deps.api.debug(
        format!(
            "WASMDEBUG: sudo_kv_query_result received; query_id: {:?}",
            query_id,
        )
        .as_str(),
    );

    // TODO: provide an actual example. Currently to many things are going to change
    // after @pro0n00gler's PRs to implement this.

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response<NeutronMsg>, ContractError> {
    match msg.id {
        BALANCE_QUERY_REPLY_ID => {
            let data = msg
                .result
                .into_result()
                .map_err(ParseReplyError::SubMsgFailure)?
                .data
                .ok_or_else(|| ParseReplyError::ParseFailure("Missing reply data".to_owned()))?;

            let reply_id= serde_json_wasm::from_slice::<ReplyId>(data.as_slice()).unwrap();
            let id_value: u64 = reply_id.id;

            // read the front of balance query address queue
            if let Some(corresponding_address) = BALANCE_QUERY_QUEUE.pop_front(deps.storage)? {
                BALANCE_QUERY_ID.save(deps.storage, corresponding_address, &id_value)?;
            }

            Ok(Response::default()
                .add_attribute("balance_query_reply_id", id_value.to_string()))
        }
        DELEGATION_USER_QUERY_REPLY_ID => {
            let data = msg
                .result
                .into_result()
                .map_err(ParseReplyError::SubMsgFailure)?
                .data
                .ok_or_else(|| ParseReplyError::ParseFailure("Missing reply data".to_owned()))?;

            let reply_id= serde_json_wasm::from_slice::<ReplyId>(data.as_slice()).unwrap();
            let id_value: u64 = reply_id.id;

            // read the front of delegation user query address queue
            if let Some(corresponding_address) = DELEGATION_USER_QUERY_QUEUE.pop_front(deps.storage)? {
                DELEGATION_USER_QUERY_ID.save(deps.storage, corresponding_address, &id_value)?;
            }

            Ok(Response::default()
                .add_attribute("delegation_user_query_reply_id", id_value.to_string()))
        }
        // _ => Err(ContractError::UnknownReplyID {}),
        _ => Ok(Response::default()),   // TODO: Replace with err
    }
}

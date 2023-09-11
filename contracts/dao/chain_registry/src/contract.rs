#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cw2::set_contract_version;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut,
    Env, MessageInfo, Response, StdResult, StdError,
};

use crate::error::{ContractError, self};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, ConnectionResponse};

use crate::state::{
    Config, store_config,
    read_config, CHAIN, 
};


// version info for migration info
const CONTRACT_NAME: &str = "chain_registry";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config {
        admin: info.sender,
    };

    store_config(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        //giving option to user to send rewards to other addresses
        ExecuteMsg::UpdateChainInfo { remote_chain,connection_id } => {
            try_update_chain_info(deps, env, info, remote_chain,connection_id)
        },
    }
}

pub fn try_update_chain_info(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    remote_chain: String,
    connection_id: String,
) -> Result<Response, ContractError> {
    let mut config = read_config(deps.storage)?;
    if info.sender != config.admin {
        return Err(error::ContractError::Std(StdError::generic_err("unauthorized")));
    }
    CHAIN.save(deps.storage, remote_chain, &connection_id)?;
    let res = Response::default();
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::ConnectionId {remote_chain} => to_binary(&query_connection_id(deps,remote_chain)?),
    }
}


fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = read_config(deps.storage)?;
    
    return Ok(ConfigResponse { admin: config.admin.to_string() })
    
}

fn query_connection_id(deps: Deps,remote_chain: String) -> StdResult<ConnectionResponse> {
    let connection_id = CHAIN.load(deps.storage, remote_chain)?;
    Ok(ConnectionResponse { 
        connection_id
    })
}
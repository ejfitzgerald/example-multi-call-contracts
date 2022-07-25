#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, SubMsgResponse, WasmMsg,
};

use crate::error::ContractError;
use crate::extract_value_from_events;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use oracle::msg::ExecuteMsg as OracleExecuteMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Consume { oracle_address } => try_consume(oracle_address),
    }
}

pub fn try_consume(oracle_address: Addr) -> Result<Response, ContractError> {
    let reply_id = 14;
    let sub = SubMsg::reply_always(
        WasmMsg::Execute {
            contract_addr: oracle_address.to_string(),
            msg: to_binary(&OracleExecuteMsg::RequestOracleValue {})?,
            funds: vec![],
        },
        reply_id,
    );

    Ok(Response::new().add_submessage(sub))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Err(StdError::generic_err("not implemented"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let result = msg
        .result
        .into_result()
        .map_err(|_| ContractError::CustomError {
            val: "the message was not successful, oh dear".to_string(),
        })?;

    fn extract_oracle_value(result: SubMsgResponse) -> Result<String, ContractError> {
        extract_value_from_events(&result.events, "wasm", "oracle-value").ok_or_else(|| {
            StdError::generic_err("unable to find the oracle value in the response").into()
        })
    }

    let oracle_value = extract_oracle_value(result)?;

    let resp = Response::new()
        .add_attribute("reply-id", msg.id.to_string())
        .add_attribute("oracle-value", oracle_value);

    Ok(resp)
}

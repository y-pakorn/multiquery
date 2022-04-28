use cosmwasm_std::{
    entry_point, to_binary, to_vec, Binary, ContractResult, Deps, DepsMut, Empty, Env, MessageInfo,
    QuerierResult, Response, StdError, StdResult, SystemResult,
};

use crate::msg::{MultiQueryResponseItem, QueryMsg};

#[entry_point]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Err(StdError::generic_err("[multiquery]: execute is not implemented"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let res = msg
        .iter()
        .map(|request| {
            process_query_result(deps.querier.raw_query(&to_vec(request)?))
        })
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&res)
}

fn process_query_result(result: QuerierResult) -> StdResult<MultiQueryResponseItem> {
    let item = match result {
        SystemResult::Ok(ContractResult::Ok(data)) => MultiQueryResponseItem {
            success: true,
            data,
        },
        SystemResult::Ok(ContractResult::Err(err)) => MultiQueryResponseItem {
            success: false,
            data: err.as_bytes().into(),
        },
        SystemResult::Err(err) => MultiQueryResponseItem {
            success: false,
            data: err.to_string().as_bytes().into(),
        },
    };

    Ok(item)
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}
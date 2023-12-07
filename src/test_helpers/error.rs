use crate::{Contract, ContractWrapper};
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError};

fn instantiate_err(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Empty,
) -> Result<Response, StdError> {
    let _ = (deps, env, info, msg);
    Err(StdError::generic_err("Init failed"))
}

fn instantiate_ok(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Empty,
) -> Result<Response, StdError> {
    let _ = (deps, env, info, msg);
    Ok(Response::default())
}

fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> Result<Response, StdError> {
    let _ = (deps, env, info, msg);
    Err(StdError::generic_err("Execute failed"))
}

fn query(_deps: Deps, _env: Env, _msg: Empty) -> Result<Binary, StdError> {
    Err(StdError::generic_err("Query failed"))
}

pub fn contract(instantiable: bool) -> Box<dyn Contract> {
    Box::new(if instantiable {
        ContractWrapper::new_with_empty(execute, instantiate_ok, query)
    } else {
        ContractWrapper::new_with_empty(execute, instantiate_err, query)
    })
}

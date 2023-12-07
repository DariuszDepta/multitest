use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError};
use multitest::{Contract, ContractWrapper};

fn instantiate(
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
    Ok(Response::default())
}

fn query(deps: Deps, env: Env, msg: Empty) -> Result<Binary, StdError> {
    let _ = (deps, env, msg);
    Ok(Binary::default())
}

pub fn contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
}

use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use multitest::{Contract, ContractWrapper};

fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    let _ = (deps, env, info, msg);
    Ok(Response::default())
}

fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    let _ = (deps, env, info, msg);
    Ok(Response::default())
}

fn query(deps: Deps, env: Env, msg: Empty) -> StdResult<Binary> {
    let _ = (deps, env);
    to_json_binary(&msg)
}

pub fn contract() -> Box<dyn Contract> {
    Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
}

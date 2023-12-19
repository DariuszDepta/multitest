use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    let _ = (deps, env, info, msg);
    Ok(Response::default())
}

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    let _ = (deps, env, info, msg);
    Ok(Response::default())
}

pub fn query(deps: Deps, env: Env, msg: Empty) -> StdResult<Binary> {
    let _ = (deps, env);
    to_json_binary(&msg)
}

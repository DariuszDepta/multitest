use cosmwasm_std::{
    Binary, CosmosMsg, Deps, DepsMut, Empty, Env, IbcMsg, MessageInfo, Response, StdError,
    StdResult,
};

pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    let _ = (deps, env, info, msg);
    Ok(Response::new())
}

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    let _ = (deps, env, info, msg);
    let msg: CosmosMsg = IbcMsg::CloseChannel {
        channel_id: "channel".to_string(),
    }
    .into();
    Ok(Response::new().add_message(msg))
}

pub fn query(deps: Deps, env: Env, msg: Empty) -> StdResult<Binary> {
    let _ = (deps, env, msg);
    Err(StdError::generic_err("Query failed"))
}

use cosmwasm_std::{
    Binary, CosmosMsg, Deps, DepsMut, Empty, Env, IbcMsg, MessageInfo, Response, StdError,
    StdResult,
};
use multitest::{Contract, ContractWrapper};

fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    let msg: CosmosMsg = IbcMsg::CloseChannel {
        channel_id: "channel".to_string(),
    }
    .into();
    Ok(Response::new().add_message(msg))
}

fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    Err(StdError::generic_err("Query failed"))
}

pub fn contract() -> Box<dyn Contract> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdError,
};
use cw_storage_plus::Item;
use multitest::{Contract, ContractWrapper};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PayoutInit {
    pub payout: Coin,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SudoMsg {
    pub set_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayoutQuery {
    Count {},
    Payout {},
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountResponse {
    pub count: u32,
}

/// Number of payouts.
const COUNT: Item<u32> = Item::new("count");

/// Payout amount.
const PAYOUT: Item<PayoutInit> = Item::new("payout");

fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: PayoutInit,
) -> Result<Response, StdError> {
    PAYOUT.save(deps.storage, &msg)?;
    COUNT.save(deps.storage, &0)?;
    Ok(Response::default())
}

fn execute(deps: DepsMut, _env: Env, info: MessageInfo, _msg: Empty) -> Result<Response, StdError> {
    // always try to payout what was set originally
    let payout = PAYOUT.load(deps.storage)?;
    let msg = BankMsg::Send {
        to_address: info.sender.into(),
        amount: vec![payout.payout],
    };
    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "payout"))
}

fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, StdError> {
    COUNT.save(deps.storage, &msg.set_count)?;
    Ok(Response::default())
}

fn query(deps: Deps, _env: Env, msg: PayoutQuery) -> Result<Binary, StdError> {
    match msg {
        PayoutQuery::Count {} => {
            let count = COUNT.load(deps.storage)?;
            let res = CountResponse { count };
            to_json_binary(&res)
        }
        PayoutQuery::Payout {} => {
            let payout = PAYOUT.load(deps.storage)?;
            to_json_binary(&payout)
        }
    }
}

pub fn contract() -> Box<dyn Contract> {
    Box::new(ContractWrapper::new_with_empty(execute, instantiate, query).with_sudo_empty(sudo))
}
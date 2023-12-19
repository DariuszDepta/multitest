use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult,
};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutInit {
    pub payout: Coin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PayoutQuery {
    Count {},
    Payout {},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutSudo {
    pub set_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutCountResponse {
    pub count: u32,
}

/// Number of payouts.
const COUNT: Item<u32> = Item::new("count");

/// Payout amount.
const PAYOUT: Item<PayoutInit> = Item::new("payout");

pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: PayoutInit,
) -> StdResult<Response> {
    let _ = (env, info);
    PAYOUT.save(deps.storage, &msg)?;
    COUNT.save(deps.storage, &0)?;
    Ok(Response::default())
}

pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    let _ = (env, msg);
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

pub fn query(deps: Deps, env: Env, msg: PayoutQuery) -> StdResult<Binary> {
    let _ = env;
    match msg {
        PayoutQuery::Count {} => {
            let count = COUNT.load(deps.storage)?;
            let res = PayoutCountResponse { count };
            to_json_binary(&res)
        }
        PayoutQuery::Payout {} => {
            let payout = PAYOUT.load(deps.storage)?;
            to_json_binary(&payout)
        }
    }
}

pub fn sudo(deps: DepsMut, env: Env, msg: PayoutSudo) -> StdResult<Response> {
    let _ = env;
    COUNT.save(deps.storage, &msg.set_count)?;
    Ok(Response::default())
}

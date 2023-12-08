use crate::utils::bin;
use crate::{
    bail, AnyResult, AppResponse, Bank, BankSudo, Distribution, Gov, Ibc, MockCustomMsg,
    MockCustomQuery, Module, Staking, StakingSudo, Stargate, Wasm, WasmSudo,
};
use cosmwasm_std::{
    from_json, to_json_vec, Addr, Api, Binary, BlockInfo, ContractResult, CosmosMsg, Empty,
    Querier, QuerierResult, QueryRequest, StdError, Storage, SystemError, SystemResult, WasmQuery,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Router {
    pub wasm: Box<dyn Wasm>,
    pub bank: Box<dyn Bank>,
    pub custom: Box<dyn Module>,
    pub staking: Box<dyn Staking>,
    pub distribution: Box<dyn Distribution>,
    pub ibc: Box<dyn Ibc>,
    pub gov: Box<dyn Gov>,
    pub stargate: Box<dyn Stargate>,
}

impl Router {
    pub fn querier<'a>(
        &'a self,
        api: &'a dyn Api,
        storage: &'a dyn Storage,
        block_info: &'a BlockInfo,
    ) -> RouterQuerier<'a> {
        RouterQuerier {
            router: self,
            api,
            storage,
            block_info,
        }
    }
}

/// We use it to allow calling into modules from another module in sudo mode.
/// Things like gov proposals belong here.
#[derive(Serialize, Deserialize)]
pub enum SudoMsg {
    Bank(BankSudo),
    Custom(Empty),
    Staking(StakingSudo),
    Wasm(WasmSudo),
}

impl From<WasmSudo> for SudoMsg {
    fn from(wasm: WasmSudo) -> Self {
        SudoMsg::Wasm(wasm)
    }
}

impl From<BankSudo> for SudoMsg {
    fn from(bank: BankSudo) -> Self {
        SudoMsg::Bank(bank)
    }
}

impl From<StakingSudo> for SudoMsg {
    fn from(staking: StakingSudo) -> Self {
        SudoMsg::Staking(staking)
    }
}

pub trait CosmosRouter {
    /// Routes messages.
    fn execute(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        sender: Addr,
        msg: &[u8],
    ) -> AnyResult<AppResponse>;

    /// Routes queries.
    fn query(
        &self,
        api: &dyn Api,
        storage: &dyn Storage,
        block: &BlockInfo,
        request: &[u8],
    ) -> AnyResult<Binary>;

    /// Routes privileged actions.
    fn sudo(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        msg: &[u8],
    ) -> AnyResult<AppResponse>;
}

impl CosmosRouter for Router {
    fn execute(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        sender: Addr,
        msg: &[u8],
    ) -> AnyResult<AppResponse> {
        match from_json(msg) {
            Ok(CosmosMsg::Wasm(msg)) => self.wasm.execute(api, storage, self, block, sender, msg),
            Ok(CosmosMsg::Bank(msg)) => {
                self.bank
                    .execute(api, storage, self, block, sender, bin!(msg))
            }
            Ok(CosmosMsg::Custom(msg)) => {
                self.custom
                    .execute(api, storage, self, block, sender, bin!(msg))
            }
            Ok(CosmosMsg::Staking(msg)) => {
                self.staking
                    .execute(api, storage, self, block, sender, bin!(msg))
            }
            Ok(CosmosMsg::Distribution(msg)) => {
                self.distribution
                    .execute(api, storage, self, block, sender, bin!(msg))
            }
            Ok(CosmosMsg::Ibc(msg)) => {
                self.ibc
                    .execute(api, storage, self, block, sender, bin!(msg))
            }
            Ok(CosmosMsg::Gov(msg)) => {
                self.gov
                    .execute(api, storage, self, block, sender, bin!(msg))
            }
            Ok(CosmosMsg::Stargate { type_url, value }) => self
                .stargate
                .execute(api, storage, self, block, sender, type_url, value),
            _ => unimplemented!(),
        }
    }

    /// this is used by `RouterQuerier` to actual implement the `Querier` interface.
    /// you most likely want to use `router.querier(storage, block).wrap()` to get a
    /// QuerierWrapper to interact with
    fn query(
        &self,
        api: &dyn Api,
        storage: &dyn Storage,
        block: &BlockInfo,
        request: &[u8],
    ) -> AnyResult<Binary> {
        let querier = self.querier(api, storage, block);
        match from_json(request) {
            Ok(QueryRequest::Wasm(req)) => self.wasm.query(api, storage, &querier, block, req),
            Ok(QueryRequest::Bank(req)) => {
                self.bank.query(api, storage, &querier, block, bin!(req))
            }
            Ok(QueryRequest::Custom(req)) => {
                self.custom.query(api, storage, &querier, block, bin!(req))
            }
            Ok(QueryRequest::Staking(req)) => {
                self.staking.query(api, storage, &querier, block, bin!(req))
            }
            Ok(QueryRequest::Ibc(req)) => self.ibc.query(api, storage, &querier, block, bin!(req)),
            Ok(QueryRequest::Stargate { path, data }) => self
                .stargate
                .query(api, storage, &querier, block, path, data),
            _ => unimplemented!(),
        }
    }

    fn sudo(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        msg: &[u8],
    ) -> AnyResult<AppResponse> {
        match from_json(msg) {
            Ok(SudoMsg::Wasm(msg)) => {
                self.wasm
                    .sudo(api, msg.contract_addr, storage, self, block, msg.msg)
            }
            Ok(SudoMsg::Bank(msg)) => self.bank.sudo(api, storage, self, block, bin!(msg)),
            Ok(SudoMsg::Staking(msg)) => self.staking.sudo(api, storage, self, block, bin!(msg)),
            Ok(SudoMsg::Custom(_)) => unimplemented!(),
        }
    }
}

pub struct MockRouter;

impl Default for MockRouter {
    fn default() -> Self {
        Self
    }
}

impl MockRouter {
    pub fn new() -> Self {
        Self
    }
}

impl CosmosRouter for MockRouter {
    fn execute(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _block: &BlockInfo,
        _sender: Addr,
        _msg_x: &[u8],
    ) -> AnyResult<AppResponse> {
        panic!("Cannot execute MockRouters");
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _block: &BlockInfo,
        _request: &[u8],
    ) -> AnyResult<Binary> {
        panic!("Cannot query MockRouters");
    }

    fn sudo(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _block: &BlockInfo,
        _msg: &[u8],
    ) -> AnyResult<AppResponse> {
        panic!("Cannot sudo MockRouters");
    }
}

pub struct RouterQuerier<'a> {
    router: &'a dyn CosmosRouter,
    api: &'a dyn Api,
    storage: &'a dyn Storage,
    block_info: &'a BlockInfo,
}

impl<'a> RouterQuerier<'a> {
    pub fn new(
        router: &'a dyn CosmosRouter,
        api: &'a dyn Api,
        storage: &'a dyn Storage,
        block_info: &'a BlockInfo,
    ) -> Self {
        Self {
            router,
            api,
            storage,
            block_info,
        }
    }
}

impl<'a> Querier for RouterQuerier<'a> {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        match from_json(bin_request) {
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
            _ => {}
        };
        let contract_result: ContractResult<Binary> = self
            .router
            .query(self.api, self.storage, self.block_info, bin_request)
            .into();
        SystemResult::Ok(contract_result)
    }
}

use crate::{
    bail, AnyResult, AppResponse, Bank, BankSudo, Distribution, Gov, Ibc, Module, Staking,
    StakingSudo, Stargate, Wasm, WasmSudo,
};
use core::marker::PhantomData;
use cosmwasm_std::{
    from_json, Addr, Api, Binary, BlockInfo, ContractResult, CosmosMsg, CustomMsg, CustomQuery,
    Empty, Querier, QuerierResult, QueryRequest, Storage, SystemError, SystemResult,
};
use serde::de::DeserializeOwned;

#[derive(Clone)]
pub struct Router<Bank, Custom, Wasm, Staking, Distr, Ibc, Gov, Stargate> {
    // this can remain crate-only as all special functions are wired up to app currently
    // we need to figure out another format for wasm, as some like sudo need to be called after init
    pub(crate) wasm: Wasm,
    // these must be pub so we can initialize them (super user) on build
    pub bank: Bank,
    pub custom: Custom,
    pub staking: Staking,
    pub distribution: Distr,
    pub ibc: Ibc,
    pub gov: Gov,
    pub stargate: Stargate,
}

impl<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
    Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
where
    CustomT::ExecT: CustomMsg + DeserializeOwned + 'static,
    CustomT::QueryT: CustomQuery + DeserializeOwned + 'static,
    CustomT: Module,
    WasmT: Wasm<CustomT::ExecT, CustomT::QueryT>,
    BankT: Bank,
    StakingT: Staking,
    DistrT: Distribution,
    IbcT: Ibc,
    GovT: Gov,
    StargateT: Stargate,
{
    pub fn querier<'a>(
        &'a self,
        api: &'a dyn Api,
        storage: &'a dyn Storage,
        block_info: &'a BlockInfo,
    ) -> RouterQuerier<'a, CustomT::ExecT, CustomT::QueryT> {
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
    type ExecC;
    type QueryC: CustomQuery;

    fn execute(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        sender: Addr,
        msg: CosmosMsg<Self::ExecC>,
    ) -> AnyResult<AppResponse>;

    fn query(
        &self,
        api: &dyn Api,
        storage: &dyn Storage,
        block: &BlockInfo,
        request: QueryRequest<Self::QueryC>,
    ) -> AnyResult<Binary>;

    fn sudo(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        msg: SudoMsg,
    ) -> AnyResult<AppResponse>;
}

impl<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT> CosmosRouter
    for Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
where
    CustomT::ExecT: CustomMsg + DeserializeOwned + 'static,
    CustomT::QueryT: CustomQuery + DeserializeOwned + 'static,
    CustomT: Module,
    WasmT: Wasm<CustomT::ExecT, CustomT::QueryT>,
    BankT: Bank,
    StakingT: Staking,
    DistrT: Distribution,
    IbcT: Ibc,
    GovT: Gov,
    StargateT: Stargate,
{
    type ExecC = CustomT::ExecT;
    type QueryC = CustomT::QueryT;

    fn execute(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        sender: Addr,
        msg: CosmosMsg<Self::ExecC>,
    ) -> AnyResult<AppResponse> {
        match msg {
            CosmosMsg::Wasm(msg) => self.wasm.execute(api, storage, self, block, sender, msg),
            CosmosMsg::Bank(msg) => self.bank.execute(api, storage, self, block, sender, msg),
            CosmosMsg::Custom(msg) => self.custom.execute(api, storage, self, block, sender, msg),
            CosmosMsg::Staking(msg) => self.staking.execute(api, storage, self, block, sender, msg),
            CosmosMsg::Distribution(msg) => self
                .distribution
                .execute(api, storage, self, block, sender, msg),
            CosmosMsg::Ibc(msg) => self.ibc.execute(api, storage, self, block, sender, msg),
            CosmosMsg::Gov(msg) => self.gov.execute(api, storage, self, block, sender, msg),
            CosmosMsg::Stargate { type_url, value } => self
                .stargate
                .execute(api, storage, self, block, sender, type_url, value),
            _ => bail!("Cannot execute {:?}", msg),
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
        request: QueryRequest<Self::QueryC>,
    ) -> AnyResult<Binary> {
        let querier = self.querier(api, storage, block);
        match request {
            QueryRequest::Wasm(req) => self.wasm.query(api, storage, &querier, block, req),
            QueryRequest::Bank(req) => self.bank.query(api, storage, &querier, block, req),
            QueryRequest::Custom(req) => self.custom.query(api, storage, &querier, block, req),
            QueryRequest::Staking(req) => self.staking.query(api, storage, &querier, block, req),
            QueryRequest::Ibc(req) => self.ibc.query(api, storage, &querier, block, req),
            QueryRequest::Stargate { path, data } => self
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
        msg: SudoMsg,
    ) -> AnyResult<AppResponse> {
        match msg {
            SudoMsg::Wasm(msg) => {
                self.wasm
                    .sudo(api, msg.contract_addr, storage, self, block, msg.msg)
            }
            SudoMsg::Bank(msg) => self.bank.sudo(api, storage, self, block, msg),
            SudoMsg::Staking(msg) => self.staking.sudo(api, storage, self, block, msg),
            SudoMsg::Custom(_) => unimplemented!(),
        }
    }
}

pub struct MockRouter<ExecC, QueryC>(PhantomData<(ExecC, QueryC)>);

impl Default for MockRouter<Empty, Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl<ExecC, QueryC> MockRouter<ExecC, QueryC> {
    pub fn new() -> Self
    where
        QueryC: CustomQuery,
    {
        MockRouter(PhantomData)
    }
}

impl<ExecC, QueryC> CosmosRouter for MockRouter<ExecC, QueryC>
where
    QueryC: CustomQuery,
{
    type ExecC = ExecC;
    type QueryC = QueryC;

    fn execute(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _block: &BlockInfo,
        _sender: Addr,
        _msg: CosmosMsg<Self::ExecC>,
    ) -> AnyResult<AppResponse> {
        panic!("Cannot execute MockRouters");
    }

    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _block: &BlockInfo,
        _request: QueryRequest<Self::QueryC>,
    ) -> AnyResult<Binary> {
        panic!("Cannot query MockRouters");
    }

    fn sudo(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _block: &BlockInfo,
        _msg: SudoMsg,
    ) -> AnyResult<AppResponse> {
        panic!("Cannot sudo MockRouters");
    }
}

pub struct RouterQuerier<'a, ExecC, QueryC> {
    router: &'a dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
    api: &'a dyn Api,
    storage: &'a dyn Storage,
    block_info: &'a BlockInfo,
}

impl<'a, ExecC, QueryC> RouterQuerier<'a, ExecC, QueryC> {
    pub fn new(
        router: &'a dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
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

impl<'a, ExecC, QueryC> Querier for RouterQuerier<'a, ExecC, QueryC>
where
    ExecC: CustomMsg + DeserializeOwned + 'static,
    QueryC: CustomQuery + DeserializeOwned + 'static,
{
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<QueryC> = match from_json(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        let contract_result: ContractResult<Binary> = self
            .router
            .query(self.api, self.storage, self.block_info, request)
            .into();
        SystemResult::Ok(contract_result)
    }
}

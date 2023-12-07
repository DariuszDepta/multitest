use crate::router::SudoMsg;
use crate::transactions::transactional;
use crate::wasm::{ContractData, Wasm, WasmKeeper};
use crate::Contract;
use crate::Gov;
use crate::Ibc;
use crate::{AnyResult, CosmosRouter, Router};
use crate::{AppBuilder, GovFailingModule, IbcFailingModule};
use crate::{AppResponse, Executor};
use crate::{Bank, BankKeeper};
use crate::{Distribution, DistributionKeeper, StakeKeeper, Staking};
use crate::{FailingModule, Module};
use crate::{Stargate, StargateFailing};
use cosmwasm_std::testing::{MockApi, MockStorage};
use cosmwasm_std::{
    to_json_binary, Addr, Api, BlockInfo, CosmosMsg, CustomQuery, Empty, Querier, QuerierResult,
    QuerierWrapper, Record, Storage,
};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub fn next_block(block: &mut BlockInfo) {
    block.time = block.time.plus_seconds(5);
    block.height += 1;
}

/// Type alias for default build `App` to make its storing simpler in typical scenario
pub type BasicApp<ExecC = Empty, QueryC = Empty> = App<
    BankKeeper,
    MockApi,
    MockStorage,
    FailingModule<ExecC, QueryC, Empty>,
    WasmKeeper<ExecC, QueryC>,
    StakeKeeper,
    DistributionKeeper,
    IbcFailingModule,
    GovFailingModule,
    StargateFailing,
>;

/// Router is a persisted state. You can query this.
/// Execution generally happens on the RouterCache, which then can be atomically committed or rolled back.
/// We offer .execute() as a wrapper around cache, execute, commit/rollback process.
#[derive(Clone)]
pub struct App<
    Bank = BankKeeper,
    Api = MockApi,
    Storage = MockStorage,
    Custom = FailingModule<Empty, Empty, Empty>,
    Wasm = WasmKeeper<Empty, Empty>,
    Staking = StakeKeeper,
    Distr = DistributionKeeper,
    Ibc = IbcFailingModule,
    Gov = GovFailingModule,
    Stargate = StargateFailing,
> {
    pub(crate) router: Router<Bank, Custom, Wasm, Staking, Distr, Ibc, Gov, Stargate>,
    pub(crate) api: Api,
    pub(crate) storage: Storage,
    pub(crate) block: BlockInfo,
}

/// No-op application initialization function.
pub fn no_init<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>(
    router: &mut Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>,
    api: &dyn Api,
    storage: &mut dyn Storage,
) {
    let _ = (router, api, storage);
}

impl Default for BasicApp {
    fn default() -> Self {
        Self::new(no_init)
    }
}

impl BasicApp {
    /// Creates new default `App` implementation working with Empty custom messages.
    pub fn new<F>(init_fn: F) -> Self
    where
        F: FnOnce(
            &mut Router<
                BankKeeper,
                FailingModule<Empty, Empty, Empty>,
                WasmKeeper<Empty, Empty>,
                StakeKeeper,
                DistributionKeeper,
                IbcFailingModule,
                GovFailingModule,
                StargateFailing,
            >,
            &dyn Api,
            &mut dyn Storage,
        ),
    {
        AppBuilder::new().build(init_fn)
    }
}

/// Creates new default `App` implementation working with customized exec and query messages.
/// Outside of `App` implementation to make type elision better.
pub fn custom_app<ExecC, QueryC, F>(init_fn: F) -> BasicApp<ExecC, QueryC>
where
    ExecC: Debug + Clone + PartialEq + JsonSchema + DeserializeOwned + 'static,
    QueryC: Debug + CustomQuery + DeserializeOwned + 'static,
    F: FnOnce(
        &mut Router<
            BankKeeper,
            FailingModule<ExecC, QueryC, Empty>,
            WasmKeeper<ExecC, QueryC>,
            StakeKeeper,
            DistributionKeeper,
            IbcFailingModule,
            GovFailingModule,
            StargateFailing,
        >,
        &dyn Api,
        &mut dyn Storage,
    ),
{
    AppBuilder::new_custom().build(init_fn)
}

impl<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT> Querier
    for App<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
where
    CustomT::ExecT: Clone + Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
    CustomT::QueryT: CustomQuery + DeserializeOwned + 'static,
    WasmT: Wasm<CustomT::ExecT, CustomT::QueryT>,
    BankT: Bank,
    ApiT: Api,
    StorageT: Storage,
    CustomT: Module,
    StakingT: Staking,
    DistrT: Distribution,
    IbcT: Ibc,
    GovT: Gov,
    StargateT: Stargate,
{
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        self.router
            .querier(&self.api, &self.storage, &self.block)
            .raw_query(bin_request)
    }
}

impl<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
    Executor<CustomT::ExecT>
    for App<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
where
    CustomT::ExecT: Clone + Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
    CustomT::QueryT: CustomQuery + DeserializeOwned + 'static,
    WasmT: Wasm<CustomT::ExecT, CustomT::QueryT>,
    BankT: Bank,
    ApiT: Api,
    StorageT: Storage,
    CustomT: Module,
    StakingT: Staking,
    DistrT: Distribution,
    IbcT: Ibc,
    GovT: Gov,
    StargateT: Stargate,
{
    fn execute(&mut self, sender: Addr, msg: CosmosMsg<CustomT::ExecT>) -> AnyResult<AppResponse> {
        let mut all = self.execute_multi(sender, vec![msg])?;
        let res = all.pop().unwrap();
        Ok(res)
    }
}

impl<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
    App<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
where
    WasmT: Wasm<CustomT::ExecT, CustomT::QueryT>,
    BankT: Bank,
    ApiT: Api,
    StorageT: Storage,
    CustomT: Module,
    StakingT: Staking,
    DistrT: Distribution,
    IbcT: Ibc,
    GovT: Gov,
    StargateT: Stargate,
{
    /// Returns a shared reference to application's router.
    pub fn router(
        &self,
    ) -> &Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT> {
        &self.router
    }

    /// Returns a shared reference to application's api.
    pub fn api(&self) -> &ApiT {
        &self.api
    }

    /// Returns a shared reference to application's storage.
    pub fn storage(&self) -> &StorageT {
        &self.storage
    }

    /// Returns a mutable reference to application's storage.
    pub fn storage_mut(&mut self) -> &mut StorageT {
        &mut self.storage
    }

    pub fn init_modules<F, T>(&mut self, init_fn: F) -> T
    where
        F: FnOnce(
            &mut Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>,
            &dyn Api,
            &mut dyn Storage,
        ) -> T,
    {
        init_fn(&mut self.router, &self.api, &mut self.storage)
    }

    pub fn read_module<F, T>(&self, query_fn: F) -> T
    where
        F: FnOnce(
            &Router<BankT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>,
            &dyn Api,
            &dyn Storage,
        ) -> T,
    {
        query_fn(&self.router, &self.api, &self.storage)
    }
}

// Helper functions to call some custom WasmKeeper logic.
// They show how we can easily add such calls to other custom keepers (CustomT, StakingT, etc)
impl<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
    App<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
where
    BankT: Bank,
    ApiT: Api,
    StorageT: Storage,
    CustomT: Module,
    WasmT: Wasm<CustomT::ExecT, CustomT::QueryT>,
    StakingT: Staking,
    DistrT: Distribution,
    IbcT: Ibc,
    GovT: Gov,
    StargateT: Stargate,
    CustomT::ExecT: Clone + Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
    CustomT::QueryT: CustomQuery + DeserializeOwned + 'static,
{
    /// Registers contract code (like uploading wasm bytecode on a chain),
    /// so it can later be used to instantiate a contract.
    pub fn store_code(
        &mut self,
        creator: Addr,
        code: Box<dyn Contract<CustomT::ExecT, CustomT::QueryT>>,
    ) -> u64 {
        self.init_modules(|router, _, _| router.wasm.store_code(creator, code))
    }

    /// Duplicates the contract code identified by `code_id` and returns
    /// the identifier of the newly created copy of the contract code.
    ///
    /// # Examples
    ///
    /// ```
    /// use cosmwasm_std::Addr;
    /// use multitest::App;
    ///
    /// // contract implementation
    /// mod echo {
    ///   // contract entry points not shown here
    /// #  use std::todo;
    /// #  use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, SubMsg, WasmMsg};
    /// #  use serde::{Deserialize, Serialize};
    /// #  use multitest::{Contract, ContractWrapper};
    /// #
    /// #  fn instantiate(_: DepsMut, _: Env, _: MessageInfo, _: Empty) -> Result<Response, StdError> {
    /// #    todo!()
    /// #  }
    /// #
    /// #  fn execute(_: DepsMut, _: Env, _info: MessageInfo, msg: WasmMsg) -> Result<Response, StdError> {
    /// #    todo!()
    /// #  }
    /// #
    /// #  fn query(_deps: Deps, _env: Env, _msg: Empty) -> Result<Binary, StdError> {
    /// #    todo!()
    /// #  }
    /// #
    ///   pub fn contract() -> Box<dyn Contract<Empty>> {
    ///     // should return the contract
    /// #   Box::new(ContractWrapper::new(execute, instantiate, query))
    ///   }
    /// }
    ///
    /// let mut app = App::default();
    ///
    /// // prepare creator address
    ///
    /// let creator = app.api().addr_make("creator");
    ///
    /// // store a new contract, save the code id
    /// let code_id = app.store_code(creator, echo::contract());
    ///
    /// // duplicate the existing contract, duplicated contract has different code id
    /// assert_ne!(code_id, app.duplicate_code(code_id).unwrap());
    ///
    /// // zero is an invalid identifier for contract code, returns an error
    /// assert_eq!("code id: invalid", app.duplicate_code(0).unwrap_err().to_string());
    ///
    /// // there is no contract code with identifier 100 stored yet, returns an error
    /// assert_eq!("code id 100: no such code", app.duplicate_code(100).unwrap_err().to_string());
    /// ```
    pub fn duplicate_code(&mut self, code_id: u64) -> AnyResult<u64> {
        self.init_modules(|router, _, _| router.wasm.duplicate_code(code_id))
    }

    /// Returns `ContractData` for the contract with specified address.
    pub fn contract_data(&self, address: &Addr) -> AnyResult<ContractData> {
        self.read_module(|router, _, storage| router.wasm.contract_data(storage, address))
    }

    /// Returns a raw state dump of all key-values held by a contract with specified address.
    pub fn dump_wasm_raw(&self, address: &Addr) -> Vec<Record> {
        self.read_module(|router, _, storage| router.wasm.dump_wasm_raw(storage, address))
    }
}

impl<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
    App<BankT, ApiT, StorageT, CustomT, WasmT, StakingT, DistrT, IbcT, GovT, StargateT>
where
    CustomT::ExecT: Debug + PartialEq + Clone + JsonSchema + DeserializeOwned + 'static,
    CustomT::QueryT: CustomQuery + DeserializeOwned + 'static,
    WasmT: Wasm<CustomT::ExecT, CustomT::QueryT>,
    BankT: Bank,
    ApiT: Api,
    StorageT: Storage,
    CustomT: Module,
    StakingT: Staking,
    DistrT: Distribution,
    IbcT: Ibc,
    GovT: Gov,
    StargateT: Stargate,
{
    pub fn set_block(&mut self, block: BlockInfo) {
        self.router
            .staking
            .process_queue(&self.api, &mut self.storage, &self.router, &self.block)
            .unwrap();
        self.block = block;
    }

    // this let's use use "next block" steps that add eg. one height and 5 seconds
    pub fn update_block<F: Fn(&mut BlockInfo)>(&mut self, action: F) {
        self.router
            .staking
            .process_queue(&self.api, &mut self.storage, &self.router, &self.block)
            .unwrap();
        action(&mut self.block);
    }

    /// Returns a copy of the current block_info
    pub fn block_info(&self) -> BlockInfo {
        self.block.clone()
    }

    /// Simple helper so we get access to all the QuerierWrapper helpers,
    /// eg. wrap().query_wasm_smart, query_all_balances, ...
    pub fn wrap(&self) -> QuerierWrapper<CustomT::QueryT> {
        QuerierWrapper::new(self)
    }

    /// Runs multiple CosmosMsg in one atomic operation.
    /// This will create a cache before the execution, so no state changes are persisted if any of them
    /// return an error. But all writes are persisted on success.
    pub fn execute_multi(
        &mut self,
        sender: Addr,
        msgs: Vec<CosmosMsg<CustomT::ExecT>>,
    ) -> AnyResult<Vec<AppResponse>> {
        // we need to do some caching of storage here, once in the entry point:
        // meaning, wrap current state, all writes go to a cache, only when execute
        // returns a success do we flush it (otherwise drop it)

        let Self {
            block,
            router,
            api,
            storage,
        } = self;

        transactional(&mut *storage, |write_cache, _| {
            msgs.into_iter()
                .map(|msg| router.execute(&*api, write_cache, block, sender.clone(), msg))
                .collect()
        })
    }

    /// Call a smart contract in "sudo" mode.
    /// This will create a cache before the execution, so no state changes are persisted if this
    /// returns an error, but all are persisted on success.
    pub fn wasm_sudo<T: Serialize, U: Into<Addr>>(
        &mut self,
        contract_addr: U,
        msg: &T,
    ) -> AnyResult<AppResponse> {
        let msg = to_json_binary(msg)?;

        let Self {
            block,
            router,
            api,
            storage,
        } = self;

        transactional(&mut *storage, |write_cache, _| {
            router
                .wasm
                .sudo(&*api, contract_addr.into(), write_cache, router, block, msg)
        })
    }

    /// Runs arbitrary SudoMsg.
    /// This will create a cache before the execution, so no state changes are persisted if this
    /// returns an error, but all are persisted on success.
    pub fn sudo(&mut self, msg: SudoMsg) -> AnyResult<AppResponse> {
        // we need to do some caching of storage here, once in the entry point:
        // meaning, wrap current state, all writes go to a cache, only when execute
        // returns a success do we flush it (otherwise drop it)
        let Self {
            block,
            router,
            api,
            storage,
        } = self;

        transactional(&mut *storage, |write_cache, _| {
            router.sudo(&*api, write_cache, block, msg)
        })
    }
}

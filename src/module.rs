use crate::{bail, AnyResult, AppResponse, CosmosRouter};
use cosmwasm_std::{Addr, Api, Binary, BlockInfo, Querier, Storage};
use std::fmt::Debug;

/// Module interface.
pub trait Module {
    /// Processes any message, which can be called by any external actor or smart contract.
    fn execute(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter,
        block: &BlockInfo,
        sender: Addr,
        msg: &[u8],
    ) -> AnyResult<AppResponse>;

    /// Processes any query request, which can be called by any external actor or smart contract.
    fn query(
        &self,
        api: &dyn Api,
        storage: &dyn Storage,
        querier: &dyn Querier,
        block: &BlockInfo,
        request: &[u8],
    ) -> AnyResult<Binary>;

    /// Runs privileged actions, like minting tokens, or governance proposals.
    /// This allows modules to have full access to these privileged actions,
    /// that cannot be triggered by smart contracts.
    ///
    /// There is no sender, as this must be previously authorized before calling.
    fn sudo(
        &self,
        api: &dyn Api,
        storage: &mut dyn Storage,
        router: &dyn CosmosRouter,
        block: &BlockInfo,
        msg: &[u8],
    ) -> AnyResult<AppResponse>;
}

#[derive(Default)]
pub struct FailingModule;

impl FailingModule {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Module for FailingModule {
    /// Processes any message, always returns an error.
    fn execute(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter,
        _block: &BlockInfo,
        sender: Addr,
        msg: &[u8],
    ) -> AnyResult<AppResponse> {
        bail!("Unexpected execute message {:?} from {:?}", msg, sender)
    }

    /// Processes any query request, always returns an error.
    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: &[u8],
    ) -> AnyResult<Binary> {
        bail!("Unexpected query request {:?}", request)
    }

    /// Processes any privileged action, always returns an error.
    fn sudo(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter,
        _block: &BlockInfo,
        msg: &[u8],
    ) -> AnyResult<AppResponse> {
        bail!("Unexpected sudo message {:?}", msg)
    }
}

#[derive(Default)]
pub struct AcceptingModule;

impl AcceptingModule {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Module for AcceptingModule {
    /// processes any message, always returns a default response.
    fn execute(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter,
        _block: &BlockInfo,
        _sender: Addr,
        _msg: &[u8],
    ) -> AnyResult<AppResponse> {
        Ok(AppResponse::default())
    }

    /// Processes any query request, always returns an empty binary.
    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _request: &[u8],
    ) -> AnyResult<Binary> {
        Ok(Binary::default())
    }

    /// Processes any privileged action, always returns a default response.
    fn sudo(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter,
        _block: &BlockInfo,
        _msg: &[u8],
    ) -> AnyResult<AppResponse> {
        Ok(AppResponse::default())
    }
}

//! # Contract definitions used in tests

use multitest::{Contract, ContractWrapper};

pub mod ibc;
pub mod nop;
pub mod payout;

pub fn contract_ibc() -> Box<dyn Contract> {
    Box::new(ContractWrapper::new(
        ibc::execute,
        ibc::instantiate,
        ibc::query,
    ))
}

pub fn contract_nop() -> Box<dyn Contract> {
    Box::new(ContractWrapper::new_with_empty(
        nop::execute,
        nop::instantiate,
        nop::query,
    ))
}

pub fn contract_payout() -> Box<dyn Contract> {
    Box::new(
        ContractWrapper::new_with_empty(payout::execute, payout::instantiate, payout::query)
            .with_sudo_empty(payout::sudo),
    )
}

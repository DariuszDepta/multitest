use crate::test_contract_helpers::nop;
use cosmwasm_std::{Addr, Empty};
use multitest::{App, Executor};

/// Utility function for initialization of the chain used in all tests.
fn chain() -> App {
    App::default()
}

/// Utility function for creating an address of the creator in all tests.
fn creator(chain: &App) -> Addr {
    chain.api().addr_make("creator")
}

/// Utility function for creating an address of the owner in all tests.
fn owner(chain: &App) -> Addr {
    chain.api().addr_make("owner")
}

#[test]
fn storing_contract_should_work() {
    // initialize the chain
    let mut chain = chain();

    // prepare an address of the contract's code creator
    let creator = creator(&chain);

    // store the contract on a chain - no code of the contract is called
    let code_id = chain.store_code(creator, nop::contract());

    // because there are no previously stored contracts then code id should be 1
    assert_eq!(1, code_id);
}

#[test]
fn instantiating_contract_should_work() {
    // initialize the chain
    let mut chain = chain();

    // prepare an address of the contract's code creator
    let creator = creator(&chain);

    // store the contract on a chain - no code of the contract is called
    let code_id = chain.store_code(creator, nop::contract());

    // instantiate the contract
    let sender = owner(&chain);
    let contract_addr = chain
        .instantiate_contract(code_id, sender, &Empty {}, &[], "NOP", None)
        .unwrap();

    // because there are no previously stored contracts then code id should be 1
    assert_eq!("contract0", contract_addr.as_str());
}

#[test]
fn executing_contract_should_work() {
    // initialize the chain
    let mut chain = chain();

    // prepare an address of the contract's code creator
    let creator = creator(&chain);

    // store the contract on a chain - no code of the contract is called
    let code_id = chain.store_code(creator, nop::contract());

    // instantiate the contract
    let sender = owner(&chain);
    let contract_addr = chain
        .instantiate_contract(code_id, sender.clone(), &Empty {}, &[], "NOP", None)
        .unwrap();

    // execute the contract
    let response = chain
        .execute_contract(sender, contract_addr, &Empty {}, &[])
        .unwrap();

    // there is no resulting data expected after executing this contract
    assert_eq!(None, response.data);
}

#[test]
fn querying_contract_should_work() {
    // initialize the chain
    let mut chain = chain();

    // prepare an address of the contract's code creator
    let creator = creator(&chain);

    // store the contract on a chain - no code of the contract is called
    let code_id = chain.store_code(creator, nop::contract());

    // instantiate the contract
    let sender = owner(&chain);
    let contract_addr = chain
        .instantiate_contract(code_id, sender, &Empty {}, &[], "NOP", None)
        .unwrap();

    // query the contract
    let result: Empty = chain
        .wrap()
        .query_wasm_smart(contract_addr, &Empty {})
        .unwrap();

    // empty message is the expected result
    assert_eq!(Empty {}, result);
}

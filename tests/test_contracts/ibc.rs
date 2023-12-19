use crate::test_contract_helpers::contract_ibc;
use cosmwasm_std::{Addr, Empty, StdResult};
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
    let code_id = chain.store_code(creator, contract_ibc());

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
    let code_id = chain.store_code(creator, contract_ibc());

    // instantiate the contract
    let sender = owner(&chain);
    let contract_addr = chain
        .instantiate_contract(code_id, sender, &Empty {}, &[], "IBC", None)
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
    let code_id = chain.store_code(creator, contract_ibc());

    // instantiate the contract
    let sender = owner(&chain);
    let contract_addr = chain
        .instantiate_contract(code_id, sender.clone(), &Empty {}, &[], "IBC", None)
        .unwrap();

    // executing the contract, expected result is an error,
    // because default stargate handler always fails
    chain
        .execute_contract(sender, contract_addr, &Empty {}, &[])
        .unwrap_err();
}

#[test]
fn querying_contract_should_work() {
    // initialize the chain
    let mut chain = chain();

    // prepare an address of the contract's code creator
    let creator = creator(&chain);

    // store the contract on a chain - no code of the contract is called
    let code_id = chain.store_code(creator, contract_ibc());

    // instantiate the contract
    let sender = owner(&chain);
    let contract_addr = chain
        .instantiate_contract(code_id, sender, &Empty {}, &[], "IBC", None)
        .unwrap();

    // query the contract
    let result: StdResult<Empty> = chain.wrap().query_wasm_smart(contract_addr, &Empty {});

    // expected result is an error
    assert!(result.is_err());
    assert_eq!(
        r#"Err(GenericErr { msg: "Querier contract error: Generic error: Query failed" })"#,
        format!("{:?}", result)
    );
}

use crate::test_contract_helpers::payout;
use crate::test_contract_helpers::payout::PayoutInit;
use cosmwasm_std::{coin, coins, Addr, Coin, Empty, Event};
use multitest::{App, Executor};

fn get_balance(chain: &App, addr: &Addr) -> Vec<Coin> {
    chain.wrap().query_all_balances(addr).unwrap()
}

#[test]
fn payout_operations_should_work() {
    // create the owner address
    let owner = App::default().api().addr_make("owner");

    // prepare initial funds of the owner
    let owner_initial_funds = vec![coin(20, "btc"), coin(100, "eth")];

    // initialize the chain
    let mut chain = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &owner, owner_initial_funds)
            .unwrap();
    });

    // prepare an address of the contract's code creator
    let creator = chain.api().addr_make("creator");

    // store the contract on a chain
    let code_id = chain.store_code(creator, payout::contract());

    // prepare the initial amount to payed
    let payout_init = PayoutInit {
        payout: Coin::new(5, "eth"),
    };

    // instantiate the contract
    let instantiation_cost = coins(23, "eth");
    let contract_addr = chain
        .instantiate_contract(
            code_id,
            owner.clone(),
            &payout_init,
            &instantiation_cost,
            "PAYOUT",
            None,
        )
        .unwrap();

    // check the balances of the owner after contract instantiation
    assert_eq!(
        vec![coin(20, "btc"), coin(77, "eth")],
        get_balance(&chain, &owner)
    );

    // now the contract has funds taken from owner during instantiation
    assert_eq!(coins(23, "eth"), get_balance(&chain, &contract_addr));

    // create an empty account
    let sender = chain.api().addr_make("sender");

    // balance should be empty
    assert!(get_balance(&chain, &sender).is_empty());

    // do one payout and see money coming in
    let res = chain
        .execute_contract(sender.clone(), contract_addr.clone(), &Empty {}, &[])
        .unwrap();
    assert_eq!(3, res.events.len());

    // the call to payout does emit this as well as custom attributes
    let payout_exec = &res.events[0];
    assert_eq!(payout_exec.ty.as_str(), "execute");
    assert_eq!(
        payout_exec.attributes,
        [("_contract_address", &contract_addr)]
    );

    // next is a custom wasm event
    let custom_attrs = res.custom_attrs(1);
    assert_eq!(custom_attrs, [("action", "payout")]);

    // then the transfer event
    let expected_transfer = Event::new("transfer")
        .add_attribute("recipient", sender.as_str())
        .add_attribute("sender", &contract_addr)
        .add_attribute("amount", "5eth");
    assert_eq!(&expected_transfer, &res.events[2]);

    // random got cash
    let funds = get_balance(&chain, &sender);
    assert_eq!(funds, coins(5, "eth"));
    // contract lost it
    let funds = get_balance(&chain, &contract_addr);
    assert_eq!(funds, coins(18, "eth"));
}

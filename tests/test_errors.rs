use cosmwasm_std::{WasmMsg, WasmQuery};
use multitest::Error;

#[test]
fn creating_errors_should_work() {
    assert_eq!(
        "Empty attribute key. Value: alpha",
        Error::empty_attribute_key("alpha").to_string()
    );
    assert_eq!(
        "Empty attribute value. Key: beta",
        Error::empty_attribute_value("beta").to_string()
    );
    assert_eq!(
        "Attribute key starts with reserved prefix: bowl",
        Error::reserved_attribute_key("bowl").to_string()
    );
    assert_eq!(
        "Event type too short: too_short",
        Error::event_type_too_short("too_short").to_string()
    );
    assert_eq!(
        r#"Unsupported wasm query: Smart { contract_addr: "", msg: Binary() }"#,
        Error::unsupported_wasm_query(WasmQuery::Smart {
            contract_addr: "".to_string(),
            msg: Default::default()
        })
        .to_string()
    );
    #[cfg(feature = "cosmwasm_1_2")]
    assert_eq!(
        "Unsupported wasm query: CodeInfo { code_id: 12 }",
        Error::unsupported_wasm_query(WasmQuery::CodeInfo { code_id: 12 }).to_string()
    );
    assert_eq!(
        r#"Unsupported wasm message: Migrate { contract_addr: "", new_code_id: 0, msg:  }"#,
        Error::unsupported_wasm_msg(WasmMsg::Migrate {
            contract_addr: "".to_string(),
            new_code_id: 0,
            msg: Default::default()
        })
        .to_string()
    );
    #[cfg(feature = "cosmwasm_1_2")]
    assert_eq!(
        r#"Unsupported wasm message: Instantiate2 { admin: None, code_id: 0, label: "", msg: , funds: [], salt: Binary() }"#,
        Error::unsupported_wasm_msg(WasmMsg::Instantiate2 {
            admin: None,
            code_id: 0,
            label: "".to_string(),
            msg: Default::default(),
            funds: vec![],
            salt: Default::default(),
        })
        .to_string()
    );
    assert_eq!("code id: invalid", Error::invalid_code_id().to_string());
    assert_eq!(
        "code id 54: no such code",
        Error::unregistered_code_id(54).to_string()
    );
    assert_eq!(
        "Contract with this address already exists: juno12938",
        Error::duplicated_contract_address("juno12938").to_string()
    );
}

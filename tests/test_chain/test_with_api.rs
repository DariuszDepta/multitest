use cosmwasm_std::{CanonicalAddr, HexBinary};
use multitest::addons::{MockApiBech32, MockApiBech32m};
use multitest::Chain;

const HEX: &str = "bc6bfd848ebd7819c9a82bf124d65e7f739d08e002601e23bb906aacd40a3d81";

#[test]
fn building_chain_with_custom_bech32_api_should_work() {
    // prepare test data
    let human = "juno1h34lmpywh4upnjdg90cjf4j70aee6z8qqfspugamjp42e4q28kqsksmtyp";

    // create a chain with custom API that implements
    // Bech32 address encoding with 'juno' prefix
    let chain = Chain::default().with_api(MockApiBech32::new("juno"));

    // check address validation function
    assert_eq!(human, chain.api().addr_validate(human).unwrap().as_str(),);

    // check if address can be canonicalized
    assert_eq!(
        CanonicalAddr::from(HexBinary::from_hex(HEX).unwrap()),
        chain.api().addr_canonicalize(human).unwrap(),
    );

    // check if address can be humanized
    assert_eq!(
        human,
        chain
            .api()
            .addr_humanize(&chain.api().addr_canonicalize(human).unwrap())
            .unwrap()
            .as_str()
    );

    // check extension function for creating addresses
    assert_eq!(human, chain.api().addr_make("creator").as_str());
}

#[test]
fn building_chain_with_custom_bech32m_api_should_work() {
    // prepare test data
    let human = "osmo1h34lmpywh4upnjdg90cjf4j70aee6z8qqfspugamjp42e4q28kqs455vpp";

    // create a chain with custom API that implements
    // Bech32m address encoding with 'osmo' prefix
    let chain = Chain::default().with_api(MockApiBech32m::new("osmo"));

    // check address validation function
    assert_eq!(human, chain.api().addr_validate(human).unwrap().as_str(),);

    // check if address can be canonicalized
    assert_eq!(
        CanonicalAddr::from(HexBinary::from_hex(HEX).unwrap()),
        chain.api().addr_canonicalize(human).unwrap(),
    );

    // check if address can be humanized
    assert_eq!(
        human,
        chain
            .api()
            .addr_humanize(&chain.api().addr_canonicalize(human).unwrap())
            .unwrap()
            .as_str()
    );

    // check extension function for creating addresses
    assert_eq!(human, chain.api().addr_make("creator").as_str());
}

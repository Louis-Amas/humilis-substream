pub mod contracts {
    use std::collections::HashMap;

    use alloy_json_abi::JsonAbi;


    const CONTRACT_NAMES: [(&'static str, &'static str); 1] = [
        ("ERC20", include_str!("../../abi/erc20.abi.json")),
    ];

    use lazy_static::lazy_static;


    lazy_static! {
        // pub static ref ERC20_ABI: (&'static str, JsonAbi) = generate_abi_from_string("ERC20", include_str!("../../abi/erc20.abi.json"));
        pub static ref EVENTS_SIGNATURE_TO_CONTRACTS: HashMap<String, Vec<(String,JsonAbi)>> = {
            let mut m = HashMap::new();

            for contract in CONTRACT_NAMES.iter() {
                let abi: JsonAbi = serde_json::from_str(contract.1).unwrap();

                for event in abi.events() {
                    let signature = event.signature();

                    // Creating a new vector if the signature is not found
                    let abi_and_keys = m.entry(signature.clone()).or_insert(Vec::new());

                    // Add the current contract name and ABI to the vector
                    abi_and_keys.push((contract.0.to_string(), abi.clone()));
                }
            }
            m
        };
    }


    #[test]
    fn test() {
        assert_eq!(EVENTS_SIGNATURE_TO_CONTRACTS.len(), 4);

        for contracts in EVENTS_SIGNATURE_TO_CONTRACTS.values() {
            assert_eq!(contracts.get(0).unwrap().0, "ERC20");
        }
    }
}

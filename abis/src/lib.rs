pub mod contracts {
    use std::{collections::HashMap, pin::Pin};

    use alloy_json_abi::JsonAbi;

    use alloy_primitives::B256;


    const CONTRACT_NAMES: [(&'static str, &'static str); 1] = [
        ("ERC20", include_str!("../abi/erc20.abi.json")),
    ];

    use lazy_static::lazy_static;

    pub struct EventItem {
        pub name: String,
        pub signature: String,
        pub abi: Pin<Box<JsonAbi>>,
    }

    lazy_static! {
        pub static ref EVENTS_SELECTOR_TO_ABI: HashMap<B256, Vec<EventItem>> = {
            let mut m = HashMap::new();

            for contract in CONTRACT_NAMES.iter() {
                let abi: Pin<Box<JsonAbi>> = Pin::new(Box::new(serde_json::from_str(contract.1).unwrap()));

                for event in abi.events() {
                    // Creating a new vector if the signature is not found
                    let abi_and_keys = m.entry(event.selector()).or_insert(Vec::new());

                    // Add the current contract name and ABI to the vector
                    abi_and_keys.push(
                        EventItem {
                            name: contract.0.to_string(),
                            signature: event.signature(),
                            abi: abi.clone(),
                        }
                    );
                }
            }
            m
        };
    }


    #[test]
    fn test() {
        assert_eq!(EVENTS_SELECTOR_TO_ABI.len(), 4);

        for selector in EVENTS_SELECTOR_TO_ABI.keys() {
            let names_and_abis = EVENTS_SELECTOR_TO_ABI.get(selector).unwrap();
            let value = names_and_abis.get(0).unwrap();
            assert_eq!(value.name, "ERC20");
            println!("{}", selector);
            println!("{}", value.signature);
        }

    }
}

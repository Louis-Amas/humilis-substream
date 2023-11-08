pub mod contracts {

    pub mod tokens {
        include!("erc20.rs");
    }

    use ethers::{prelude::Lazy, types::H256};
    
    use std::collections::HashMap;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref NAME_TO_CONTRACTS: HashMap<&'static str, &'static Lazy<ethers::core::abi::Abi>> = {
            let mut m = HashMap::new();
            m.insert("ERC20", &tokens::ERC20_ABI);
            m
        };
    }

    lazy_static! {
        pub static ref EVENTS_SIGNATURE_TO_CONTRACTS: HashMap<H256, Vec<(&'static str, &'static Lazy<ethers::core::abi::Abi>)>> = {
            let mut m = HashMap::new();

            for key in NAME_TO_CONTRACTS.keys() {
                let contract = NAME_TO_CONTRACTS.get(key).unwrap();

                for event in contract.events().into_iter() {
                    let signature = event.signature();
                    let vector_options = m.get_mut(&signature);
                    if vector_options.is_none() {
                        let mut contracts_and_keys = Vec::new();
                        contracts_and_keys.push((*key, *contract));
                        m.insert(signature, contracts_and_keys);
                    } else {
                        let contracts_and_keys = vector_options.unwrap();
                        contracts_and_keys.push((*key, *contract));
                    }
                }
            }
            m
        };
    }

}

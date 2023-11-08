use ethers::prelude::Abigen;

const DESTINATION: &str = "./src/generated";

const ERC20_ABI: &str = include_str!("../../abi/erc20.abi.json");

const ABIS: [(&str, &str); 1] = [
    ("ERC20", ERC20_ABI), 
];

fn generate_abi_file_from_abi(
    name: &str,
    abi: &str,
) -> Result<(), String> {

    Abigen::new(&name, abi)
        .unwrap()
        .generate()
        .unwrap()
        .write_module_in_dir(DESTINATION)
        .unwrap();

    return Ok(());
}

fn main() {
    for abi_and_name in ABIS.into_iter() {
        let result = generate_abi_file_from_abi(abi_and_name.0, abi_and_name.1);
        if result.is_err() {
            panic!("{}", result.unwrap_err());
        }

        println!("Success building abis");
    }
}

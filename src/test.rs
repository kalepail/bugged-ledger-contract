#![cfg(test)]

use std::println;

use soroban_sdk::Env;

use crate::{Contract, ContractClient};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let res = client.test();

    println!("{:?}", res);
}

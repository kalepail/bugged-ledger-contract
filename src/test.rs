#![cfg(test)]

use std::println;

use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::{Contract, ContractClient};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let client = ContractClient::new(&env, &contract_id);

    let account_1 = Address::random(&env);

    let res = client.test(&account_1);

    println!("res: {:?}", res);
}

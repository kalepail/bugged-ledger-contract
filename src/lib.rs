#![no_std]

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use soroban_sdk::{contracterror, contractimpl, xdr::ToXdr, Address, Bytes, BytesN, Env};

pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MissingPew = 1,
    WrongString = 2,
    WrongHash = 3,
}

#[contractimpl]
impl Contract {
    pub fn test(env: Env, source: Address) -> (BytesN<32>, Bytes) {
        source.require_auth();

        let mut nonce: u32 = 0;
        let mut array_1 = [0u8; 32];
        let array_2 = [255u8; 32];
        let state = get_entropy(&env, &mut nonce);

        let mut rng_store = SmallRng::seed_from_u64(state);

        rng_store.fill(&mut array_1[..]);

        let mut bytes = source.clone().to_xdr(&env);

        bytes.extend_from_array(&array_1);
        bytes.extend_from_array(&array_2);

        let hash = env.crypto().sha256(&bytes);

        env.storage().set(&source, &hash);
        env.storage().set(&hash, &array_1); // <-- comment this line out and it all "magically" works
        env.events().publish((hash.clone(),), array_2.clone());

        (hash, bytes)
    }
}

fn get_entropy(env: &Env, nonce: &mut u32) -> u64 {
    let mut contract_entropy: u64 = nonce.clone() as u64;

    env.current_contract_id()
        .to_array()
        .chunks(8)
        .for_each(|chunk| {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(chunk);
            contract_entropy = contract_entropy.wrapping_add(u64::from_be_bytes(bytes));
        });

    let ledger_sequence: u64 = env.ledger().sequence() as u64;
    let ledger_timestamp = env.ledger().timestamp();
    // OR comment out the above and use the below
    // let ledger_sequence: u64 = 0u32 as u64;
    // let ledger_timestamp: u64 = 0;
    let entropy = contract_entropy
        .wrapping_add(ledger_sequence)
        .wrapping_add(ledger_timestamp);

    *nonce += 1;
    entropy
}

mod test;

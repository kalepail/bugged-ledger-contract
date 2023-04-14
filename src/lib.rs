#![no_std]

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use soroban_sdk::{contractimpl, Bytes, BytesN, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn test(env: Env) -> BytesN<32> {
        let mut array = [0u8; 32];
        let state = get_entropy(&env);

        let mut rng_store = SmallRng::seed_from_u64(state);

        rng_store.fill(&mut array[..]);

        let hash = env.crypto().sha256(&Bytes::from_slice(&env, &array));

        env.storage().set(&hash, &array); // <-- comment this line out and it all "magically" works

        hash
    }
}

fn get_entropy(env: &Env) -> u64 {
    let ledger_sequence = env.ledger().sequence() as u64;
    let ledger_timestamp = env.ledger().timestamp();
    // OR comment out the above and use the below and it will also "magically" work even whe the storage line above is used /shrug
    // let ledger_sequence = 0u32 as u64;
    // let ledger_timestamp = 0u64;
    let entropy = 0u64
        .wrapping_add(ledger_sequence)
        .wrapping_add(ledger_timestamp);

    entropy
}

mod test;

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
    let ledger_sequence = u64::from(env.ledger().sequence());
    let ledger_timestamp = env.ledger().timestamp();
    // OR comment out the above and use the below and it will also "magically" work even whe the storage line above is used /shrug
    // let ledger_sequence = u64::from(u32::MAX);
    // let ledger_timestamp = u64::MAX;
    let entropy = u64::MIN
        .wrapping_add(ledger_sequence)
        .wrapping_add(ledger_timestamp);

    entropy
}

mod test;

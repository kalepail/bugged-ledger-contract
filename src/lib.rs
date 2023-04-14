#![no_std]

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use soroban_sdk::{contractimpl, Bytes, BytesN, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn test(env: Env) -> BytesN<32> {
        let state = get_entropy(&env);
        let mut rng = SmallRng::seed_from_u64(state);
        let mut array = [0u8; 32];

        rng.fill(&mut array[..]);

        let bytes = Bytes::from_slice(&env, &array);
        let hash = env.crypto().sha256(&bytes);

        env.storage().set(&hash, &array); // <-- comment this line out and it all "magically" works

        hash
    }
}

fn get_entropy(env: &Env) -> u64 {
    let ledger_sequence = u64::from(env.ledger().sequence());
    let ledger_timestamp = env.ledger().timestamp();
    // OR comment out the two lines above and use the two lines below and it will also "magically" work even when the storage line from above is used /shrug
    // let ledger_sequence = u64::from(u32::MAX);
    // let ledger_timestamp = u64::MAX;
    let entropy = u64::MIN
        .wrapping_add(ledger_sequence)
        .wrapping_add(ledger_timestamp);

    entropy
}

mod test;

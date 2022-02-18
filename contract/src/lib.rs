/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::AccountId;
use near_sdk::{env, near_bindgen, setup_alloc};

setup_alloc!();

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RonaldoVsMessi {
    votes_for_ronaldo: Vector<AccountId>,
    votes_for_messi: Vector<AccountId>,
}

impl Default for RonaldoVsMessi {
    fn default() -> Self {
        RonaldoVsMessi {
            votes_for_messi: Vector::new(b"s".to_vec()),
            votes_for_ronaldo: Vector::new(b"s".to_vec()),
        }
    }
}

#[near_bindgen]
impl RonaldoVsMessi {
    pub fn vote_ronaldo(&mut self) {
        let account_id = env::signer_account_id();

        self.votes_for_ronaldo.push(&account_id);
    }

    pub fn votes_messi(&mut self) {
        let account_id = env::signer_account_id();

        self.votes_for_messi.push(&account_id);
    }

    pub fn get_total_votes(self) -> (u64, u64) {
        (self.votes_for_ronaldo.len(), self.votes_for_messi.len())
    }

    pub fn check_if_user_voted(self) -> (bool, bool) {
        let account_id = env::signer_account_id();

        (
            self.votes_for_ronaldo.iter().any(|i| i == account_id),
            self.votes_for_messi.iter().any(|j| j == account_id),
        )
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use near_sdk::MockedBlockchain;
//     use near_sdk::{testing_env, VMContext};

//     // mock the context for testing, notice "signer_account_id" that was accessed above from env::
//     fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
//         VMContext {
//             current_account_id: "alice_near".to_string(),
//             signer_account_id: "bob_near".to_string(),
//             signer_account_pk: vec![0, 1, 2],
//             predecessor_account_id: "carol_near".to_string(),
//             input,
//             block_index: 0,
//             block_timestamp: 0,
//             account_balance: 0,
//             account_locked_balance: 0,
//             storage_usage: 0,
//             attached_deposit: 0,
//             prepaid_gas: 10u64.pow(18),
//             random_seed: vec![0, 1, 2],
//             is_view,
//             output_data_receivers: vec![],
//             epoch_height: 19,
//         }
//     }

//     #[test]
//     fn set_then_get_greeting() {
//         let context = get_context(vec![], false);
//         testing_env!(context);
//         let mut contract = RonaldoVsMessi::default();
//         contract.set_greeting("howdy".to_string());
//         assert_eq!(
//             "howdy".to_string(),
//             contract.get_greeting("bob_near".to_string())
//         );
//     }

//     #[test]
//     fn get_default_greeting() {
//         let context = get_context(vec![], true);
//         testing_env!(context);
//         let contract = Welcome::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(
//             "Hello".to_string(),
//             contract.get_greeting("francis.near".to_string())
//         );
//     }
// }

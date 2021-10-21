use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

const PUZZLE_NUMBER: u8 = 1;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    crossword_solution: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(solution: String) -> Self {
        Self {
            crossword_solution: solution,
        }
    }

    pub fn get_puzzle_number(&self) -> u8 {
        PUZZLE_NUMBER
    }

    pub fn set_solution(&mut self, solution: String) {
        self.crossword_solution = solution;
    }

    pub fn get_solution(&self) -> String {
        self.crossword_solution.clone()
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        if solution == self.crossword_solution {
            env::log_str("You guessed right!");
            true
        } else {
            env::log_str("Try again!");
            false
        }
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    fn check_guess_solution() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        let context = get_context(alice);
        testing_env!(context.build());

        let mut contract = Contract::default();

        contract.set_solution(
            "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
        );
        contract.guess_solution("wrong answer here".to_string());
        assert_eq!(get_logs(), ["Try again!"], "Expected a failure log.");

        contract.guess_solution(
            "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f".to_string(),
        );
        assert_eq!(
            get_logs(),
            ["Try again!", "You guessed right!"],
            "Expected to successfull log after the previous failed logs"
        );
    }

    // TESTS HERE
}

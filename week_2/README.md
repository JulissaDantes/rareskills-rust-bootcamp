## Week 2 - Exercises

### Exercise 1 - Calculator program

Build your own "Calculator" Rust program with the following restrictions:

- Create a "Calculator" structure with 2 integer members
- The "Calculator" structure should define at least three traits:
    - `AdditiveOperations`
    - `MultiplicativeOperations`
    - `BinaryOperations`
- The "Calculator" allow severals operations on scalars:
    - Addition
    - Subtraction
    - Multiplication
    - Division
    - AND
    - OR
    - XOR
- The “Calculator” can be printed through the following line of code `println!("calculator: {}", calculator);`
    - When printing the calculator, the result shows the result for each operation.

### Exercise 2 - Code analysis - NEAR Smart contract

Goal: Analyze a smart contract written in Rust for the NEAR blockchain. ⚠️ This is not a security analysis.
*Note: Some concepts have not been explained yet, give it your best!* 

Expected outputs:

- A summary explaining the purpose of this contract (should fit in 5-6 lines)
- An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Lecture of Week 2.

```rust
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::Serialize;
use near_sdk::store::Vector;
use near_sdk::{env, near_bindgen, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[borsh(crate = "near_sdk::borsh")]
pub struct PostedMessage {
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    messages: Vector<PostedMessage>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn add_message(&mut self, text: String) {
        let premium = env::attached_deposit() >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {
            premium,
            sender,
            text,
        };

        self.messages.push(message);
    }

    pub fn get_messages(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<&PostedMessage> {
        let from = u64::from(from_index.unwrap_or(U64(0)));
        let limit = u64::from(limit.unwrap_or(U64(10)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()
    }

    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_message() {
        let mut contract = Contract::default();
        contract.add_message("A message".to_string());

        let posted_message = &contract.get_messages(None, None)[0];
        assert_eq!(posted_message.premium, false);
        assert_eq!(posted_message.text, "A message".to_string());
    }

    #[test]
    fn iters_messages() {
        let mut contract = Contract::default();
        contract.add_message("1st message".to_string());
        contract.add_message("2nd message".to_string());
        contract.add_message("3rd message".to_string());

        let total = &contract.total_messages();
        assert!(*total == 3);

        let last_message = &contract.get_messages(Some(U64::from(1)), Some(U64::from(2)))[1];
        assert_eq!(last_message.premium, false);
        assert_eq!(last_message.text, "3rd message".to_string());
    }
}
```

### Exercise 3 - Code analysis - Rust variants

Several languages such as [Cairo](https://www.cairo-lang.org/) (for Starknet chain) or [Move](https://move-language.github.io/move/) (for [Aptos](https://aptos.dev/en/build/smart-contracts) and [Sui](https://docs.sui.io/concepts/sui-move-concepts) chains) are derivated from Rust. Knowing Rust offers a solid basis to understand smart contracts written with these languages.

Goal: Analyze a smart contract written in Cairo. ⚠️ This is not a security analysis.

Expected outputs:

- A summary explaining the purpose of this contract (should fit in 5-6 lines)
- An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Week 2 Lectures.

Resources:

- https://book.cairo-lang.org/

```rust
use starknet::ContractAddress;

#[starknet::interface]
pub trait IERC20<TContractState> {
    fn get_name(self: @TContractState) -> felt252;
    fn get_symbol(self: @TContractState) -> felt252;
    fn get_decimals(self: @TContractState) -> u8;
    fn get_total_supply(self: @TContractState) -> felt252;
    fn balance_of(self: @TContractState, account: ContractAddress) -> felt252;
    fn allowance(
        self: @TContractState, owner: ContractAddress, spender: ContractAddress
    ) -> felt252;
    fn transfer(ref self: TContractState, recipient: ContractAddress, amount: felt252);
    fn transfer_from(
        ref self: TContractState,
        sender: ContractAddress,
        recipient: ContractAddress,
        amount: felt252
    );
    fn approve(ref self: TContractState, spender: ContractAddress, amount: felt252);
    fn increase_allowance(ref self: TContractState, spender: ContractAddress, added_value: felt252);
    fn decrease_allowance(
        ref self: TContractState, spender: ContractAddress, subtracted_value: felt252
    );
}

#[starknet::interface]
pub trait ISimpleVault<TContractState> {
    fn deposit(ref self: TContractState, amount: u256);
    fn withdraw(ref self: TContractState, shares: u256);
    fn user_balance_of(ref self: TContractState, account: ContractAddress) -> u256;
    fn contract_total_supply(ref self: TContractState) -> u256;
}

#[starknet::contract]
pub mod SimpleVault {
    use super::{IERC20Dispatcher, IERC20DispatcherTrait};
    use starknet::{ContractAddress, get_caller_address, get_contract_address};

    #[storage]
    struct Storage {
        token: IERC20Dispatcher,
        total_supply: u256,
        balance_of: LegacyMap<ContractAddress, u256>
    }

    #[constructor]
    fn constructor(ref self: ContractState, token: ContractAddress) {
        self.token.write(IERC20Dispatcher { contract_address: token });
    }

    #[generate_trait]
    impl PrivateFunctions of PrivateFunctionsTrait {
        fn _mint(ref self: ContractState, to: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() + shares);
            self.balance_of.write(to, self.balance_of.read(to) + shares);
        }

        fn _burn(ref self: ContractState, from: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() - shares);
            self.balance_of.write(from, self.balance_of.read(from) - shares);
        }
        
    }

    #[abi(embed_v0)]
    impl SimpleVault of super::ISimpleVault<ContractState> {

        fn user_balance_of(ref self: ContractState, account: ContractAddress) -> u256 {
            self.balance_of.read(account)
        }

        fn contract_total_supply(ref self: ContractState) -> u256 {
            self.total_supply.read()
        }

        fn deposit(ref self: ContractState, amount: u256){
            let caller = get_caller_address();
            let this = get_contract_address();

            let mut shares = 0;
            if self.total_supply.read() == 0 {
                shares = amount;
            } else {
                let balance: u256 = self.token.read().balance_of(this).try_into()
                .unwrap();
                shares = (amount * self.total_supply.read()) / balance;
            }
            
           PrivateFunctions::_mint(ref self, caller, shares);
           
            let amount_felt252: felt252 = amount.low.into();
            self.token.read().transfer_from(caller, this, amount_felt252);
        }

        fn withdraw(ref self: ContractState, shares: u256) {
            let caller = get_caller_address();
            let this = get_contract_address();

            let balance = self.user_balance_of(this);
            let amount = (shares * balance) / self.total_supply.read();
            PrivateFunctions::_burn(ref self, caller, shares);
            let amount_felt252: felt252 = amount.low.into();
            self.token.read().transfer(caller, amount_felt252);
        }
    }
}
```

### Exercise 4 - Security analysis - NEAR Smart contract

The content of this exercise is available at https://github.com/zigtur/vulnerable-NEAR-contract/

- Audit the smart contract written for the NEAR blockchain
- Describe at least 2 issues with high severity. Write a recommendation to fix the code!
- Write one Proof-of-Concept for each issue as a unit test.
    - Always initialize the contract with “admin.near” as AccountId, so that the first token is minted to this user.
    - Write your PoC with Bob as the attacker.
        
        ```rust
        #[cfg(test)]
        mod tests {
            use near_sdk::{test_utils::VMContextBuilder, testing_env};
            use super::*;
            
            #[test]
            fn a_unit_test() {
                let bob: AccountId = "bob.near".parse().unwrap();
                set_context(bob.clone()); // bob.near will be the account used for the following operations.
                
                // POC here
            }
        
            // Auxiliary fn: create a mock context
            fn set_context(predecessor: AccountId) {
                let mut builder = VMContextBuilder::new();
                builder.predecessor_account_id(predecessor);
        
                testing_env!(builder.build());
            }
        }
        ```
        
- Resources
    - https://docs.rs/near-sdk/latest/near_sdk/index.html
    - https://docs.near.org/sdk/rust/introduction
- **Hint**
    
    Send a message on Slack and I’ll send basic unit tests that may help :)
    

### Exercise 5 - Optional - Advanced traits

Retake the Calculator program from Exercise 1.

- Add a `print_output` function which needs a single input parameter with the 3 defined traits:  `AdditiveOperations`, `MultiplicativeOperations` , `BinaryOperations`
- This function should print every operations’ results for the given input.
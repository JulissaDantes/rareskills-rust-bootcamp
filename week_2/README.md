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
// Imports to be able to interact with NEAR native functionality
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U64;
use near_sdk::serde::Serialize;
use near_sdk::store::Vector;
use near_sdk::{env, near_bindgen, AccountId, NearToken};

const POINT_ONE: NearToken = NearToken::from_millinear(100);

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]// indicates which crate to take this serde and borsh from, avoids version mismatch in case there are several
#[borsh(crate = "near_sdk::borsh")]
pub struct PostedMessage {// The PostedMessage implements already the traits above
    pub premium: bool,
    pub sender: AccountId,
    pub text: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {// The Contract implements already the traits above
    messages: Vector<PostedMessage>,
}

impl Default for Contract {// Implements a default behavior for Contract so its easier to create new instances
    fn default() -> Self {
        Self {
            messages: Vector::new(b"m"),
        }
    }
}

#[near_bindgen]
impl Contract {// Implements add_message, get_messages, and total_messages functions to Contract
    #[payable]// Means this function can receive funds
    pub fn add_message(&mut self, text: String) {// Creates a new message with the given tests and adds it to the message queue
        let premium = env::attached_deposit() >= POINT_ONE;
        let sender = env::predecessor_account_id();

        let message = PostedMessage {
            premium,
            sender,
            text,
        };

        self.messages.push(message);
    }
    // Returns the message found by filtering by from
    pub fn get_messages(&self, from_index: Option<U64>, limit: Option<U64>) -> Vec<&PostedMessage> {
        let from = u64::from(from_index.unwrap_or(U64(0)));
        let limit = u64::from(limit.unwrap_or(U64(10)));

        self.messages
            .iter()
            .skip(from as usize)
            .take(limit as usize)
            .collect()// collects the filtering results, if none, returns an empty vector
    }
    // Returns the amount of messages created
    pub fn total_messages(&self) -> u32 {
        self.messages.len()
    }
}

#[cfg(test)]// Indicates its a tests section and wont include this into the production build
mod tests {
    use super::*;

    #[test]//Indicates is a test function and should be run as such
    fn add_message() {
        // Creates the default Contract and adds a sample message to later verify said message can be fetch and will contain the parameters set at creation
        let mut contract = Contract::default();
        contract.add_message("A message".to_string());

        let posted_message = &contract.get_messages(None, None)[0];
        assert_eq!(posted_message.premium, false);
        assert_eq!(posted_message.text, "A message".to_string());
    }

    #[test]
    fn iters_messages() {
        // Creates the default Contract and adds a set of messages, makes sure all the messages are in the message queue, and filters a specific message at a specific index
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
**OUTPUT:**
- Summary explaining the purpose of this contract (should fit in 5-6 lines)
 This is a rust program that will interact with the NEAR chain, it imports the necessary sdks, creating the structs necessary to interact with it and adding several extra functionality, also
 it includes a tests module with several tests scenarios, when having with a single message and when having several messages.
- An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Lecture of Week 2. (See code snippet for output)

### Exercise 3 - Code analysis - Rust variants

Several languages such as [Cairo](https://www.cairo-lang.org/) (for Starknet chain) or [Move](https://move-language.github.io/move/) (for [Aptos](https://aptos.dev/en/build/smart-contracts) and [Sui](https://docs.sui.io/concepts/sui-move-concepts) chains) are derivated from Rust. Knowing Rust offers a solid basis to understand smart contracts written with these languages.

Goal: Analyze a smart contract written in Cairo. ⚠️ This is not a security analysis.

Expected outputs:

- A summary explaining the purpose of this contract (should fit in 5-6 lines)
    This contracts defines the standard ERC20 interface, as well as a simple vault(where you deposit token A to get the vaults token) interface,
    then proceeds to create an instance of the simple vault with mint and burn private function, and deposit and withdraw public functions to deposit the ERC20
    to get vault shares, or return vault shares to get erc20 tokens.
- An in-depth analysis of the contract. Comments should be added to the code snippet to explain the concepts shown in Week 2 Lectures.

Resources:

- https://book.cairo-lang.org/

```rust
use starknet::ContractAddress;// Importing the ContractAddress type that is compatible with starknet

#[starknet::interface]//Indicates the following can be use as an interface later on
pub trait IERC20<TContractState> {//Creates a ERC20 trait with generic contract state, and the must have functions
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

#[starknet::interface]// Defines a Simple Vault interface with its needed functions
pub trait ISimpleVault<TContractState> {
    fn deposit(ref self: TContractState, amount: u256);
    fn withdraw(ref self: TContractState, shares: u256);
    fn user_balance_of(ref self: TContractState, account: ContractAddress) -> u256;
    fn contract_total_supply(ref self: TContractState) -> u256;
}

#[starknet::contract]
pub mod SimpleVault {//Defines a simple vault contract
    use super::{IERC20Dispatcher, IERC20DispatcherTrait};
    use starknet::{ContractAddress, get_caller_address, get_contract_address};

    #[storage]//Define where are the following stored, Im assuming you can also indicate memory etc
    struct Storage {//Defines the values we want to keep track of
        token: IERC20Dispatcher,
        total_supply: u256,
        balance_of: LegacyMap<ContractAddress, u256>//A mapping between an address and its balance
    }

    #[constructor]//Code to be executed only at deployment
    fn constructor(ref self: ContractState, token: ContractAddress) {
        self.token.write(IERC20Dispatcher { contract_address: token });//Stores the token to be used by the contract
    }

    #[generate_trait]//Defines private functions for minting and burning tokens
    impl PrivateFunctions of PrivateFunctionsTrait {
        fn _mint(ref self: ContractState, to: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() + shares);//writes to storage
            self.balance_of.write(to, self.balance_of.read(to) + shares);
        }

        fn _burn(ref self: ContractState, from: ContractAddress, shares: u256) {
            self.total_supply.write(self.total_supply.read() - shares);
            self.balance_of.write(from, self.balance_of.read(from) - shares);
        }
        
    }

    #[abi(embed_v0)]//Since this contract is a vault using the interface of SimpleVault it must implement the functions listed in the interface declaration
    impl SimpleVault of super::ISimpleVault<ContractState> {

        fn user_balance_of(ref self: ContractState, account: ContractAddress) -> u256 {
            self.balance_of.read(account)//Reads from storage
        }

        fn contract_total_supply(ref self: ContractState) -> u256 {
            self.total_supply.read()
        }

        fn deposit(ref self: ContractState, amount: u256){
            //This function accepts a token deposit of the selected token at creation of the vault(this contract)
            //Computes the amount of shares corresponding to the amount deposited, taking into account whats the proportion
            // of the deposit amount with the contracts balance, and later mints said shares amount to the caller
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
            // This function computes the balance to be returned based on the shares being returned, then the shares are burned
            // and the computed balance is transferred to the contract caller
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

Issue 1
Problem:
Considering `pub type Id = u8;` and `pub supply: u16` when we do this on mint `self.tokens.insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());` we are loosing data, therefore not assigning the correct `Id`. 
Solution:
To fix it I recommend changing that line to `self.tokens.insert(self.supply as Id, env::predecessor_account_id());`

Issue 2
Problem:
The minting function has no access control of any kind, everybody can just call it as many times as they wish.
Solution:
Make the function only callable by the owner, for that we need to update the storage to include an `owner` variable and then add this line at the top of the function: `require!(
            env::predecessor_account_id() == self.owner,
            "Only the owner can call this function"
        );`

Issue3
Problem:
Inside the transfer function the only check made is if the caller has the token or has the token allowance, but misses to check if the token exist, nor does it decreases the allowance, allowing the sender to transfer this token after it transfer it to a previous receiver.
Solution:
I would rewrite the transfer function like this:
```
    pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(self.tokens.get(&id).is_some(), "not real token Id");
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );
        self.tokens.insert(id, receiver);
        self.approvals.remove(&id);
    }
```

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

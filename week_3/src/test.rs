#![cfg(test)]
use crate::contract::{NFToken, NFTokenClient};

use crate::test_util::setup_test_token;
use soroban_sdk::{
    testutils::Address as _, Address,
    Env,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, NFToken);
    let client = NFTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);
    assert_eq!(admin, client.admin());
}

#[test]
fn test_mint_new() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let client = setup_test_token(&env, &admin);

    let to = Address::generate(&env);
    client.mint_new(&to);
    assert_eq!(to, client.owner(&0));
}

#[test]
fn test_transfer_issue() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let owner = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let spender = Address::generate(&env);

    let client = setup_test_token(&env, &admin);

    // Admin mints token to owner
    client.mint_new(&owner);

    // Owner approves spender to transfer token
    client.appr(&owner, &spender, &0);

    // Spender transfers token from owner to recipient1
    client.transfer_from(&spender, &owner, &recipient1, &0);

    // Spender transfers same token again to recipient2
    client.transfer_from(&spender, &recipient1, &recipient2, &0);

    // Expect: should fail, because approval should've been cleared, but it doesn't
    let final_owner = client.owner(&0);
    assert_eq!(final_owner, recipient2, "Spender still has approval after transfer");
}


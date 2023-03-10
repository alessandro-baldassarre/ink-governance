use crate::psp22_votes::*;

use ink_governance::psp22_votes::*;
use openbrush::{
    contracts::psp22::*,
    traits::Balance,
};

use ink::env::{
    test::DefaultAccounts,
    DefaultEnvironment,
};
use openbrush::{
    test_utils::{
        accounts,
        change_caller,
    },
    traits::AccountId,
};

fn default_accounts() -> DefaultAccounts<DefaultEnvironment> {
    accounts()
}

fn set_caller(sender: AccountId) {
    change_caller(sender)
}

fn build_contract(caller: AccountId, supply: Balance) -> Contract {
    set_caller(caller);
    Contract::new(supply)
}

#[ink::test]
/// The constructor does its job
fn construction_works() {
    let accounts = default_accounts();

    // Mint
    let contract = build_contract(accounts.alice, 1000);

    // Get total supply after mint
    let total_supply = contract.total_supply();
    assert_eq!(total_supply, 1000);

    // Get minter balance
    let minter_balance = contract.balance_of(accounts.alice);
    assert_eq!(minter_balance, total_supply);

    // Get minter votes
    let minter_votes = contract.get_votes(accounts.alice).unwrap();
    assert_eq!(
        minter_votes,
        <u128 as TryInto<u64>>::try_into(total_supply).unwrap()
    );
}

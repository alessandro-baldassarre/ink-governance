use ink::{
    codegen::Env,
    env::{
        hash::Blake2x256,
        test::DefaultAccounts,
        DefaultEnvironment,
    },
};

use ink_governance::psp22::extensions::votes::*;
use openbrush::{
    contracts::psp22::*,
    traits::Balance,
};

use crate::gov_psp22_votes::*;
use openbrush::{
    test_utils::{
        accounts,
        change_caller,
    },
    traits::{
        AccountId,
        Hash,
        String,
    },
};

use ink_governance::{
    governor::*,
    governor_counting_simple::*,
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

fn propose(contract: &mut Contract) -> ProposalId {
    let accounts = default_accounts();

    set_caller(accounts.alice);
    let proposal = Proposal::default();
    let description = String::from("Test proposal");
    contract.propose(proposal, description).unwrap()
}

fn delegate(contract: &mut Contract, from: AccountId, to: AccountId) {
    set_caller(from);

    contract.delegate(to).unwrap();
}

#[ink::test]
/// The constructor does its job
fn contruction_works() {
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
    let minter_votes = Governor::get_votes(&contract, accounts.alice, 0).unwrap();
    assert_eq!(
        minter_votes,
        <u128 as TryInto<u64>>::try_into(total_supply).unwrap()
    );
}

#[ink::test]
/// Propose works correctly
fn propose_works() {
    let accounts = default_accounts();

    let mut contract = build_contract(accounts.alice, 1000);

    set_caller(accounts.charlie);
    let err_response = contract
        .propose(Proposal::default(), String::from(""))
        .unwrap_err();
    assert_eq!(err_response, GovernorError::NoVotes);

    set_caller(accounts.alice);
    let proposal = Proposal::default();
    let description = String::from("Test proposal");
    let description_hash = Hash::try_from(
        contract
            .env()
            .hash_bytes::<Blake2x256>(&description)
            .as_ref(),
    )
    .unwrap();
    let proposal_id = contract.hash_proposal(proposal.clone(), description_hash);
    let response = contract
        .propose(proposal.clone(), description.clone())
        .unwrap();
    assert_eq!(response, proposal_id);

    let proposal_state = ProposalState::Active;
    let response = contract.state(proposal_id).unwrap();
    assert_eq!(response, proposal_state);
}
#[ink::test]
/// Cast vote works correctly
fn cast_vote_works() {
    let accounts = default_accounts();

    let mut contract = build_contract(accounts.alice, 1000);

    let proposal_id = propose(&mut contract);

    ink::env::test::advance_block::<DefaultEnvironment>();
    // Delegate \\Alice votes to \\Charlie
    delegate(&mut contract, accounts.alice, accounts.charlie);
    set_caller(accounts.charlie);
    // Should has no votes since the proposal was submitted before the delegation
    let err = contract.cast_vote(proposal_id, 1).unwrap_err();
    assert_eq!(err, GovernorError::NoVotes);

    set_caller(accounts.alice);
    contract.cast_vote(proposal_id, 1).unwrap();

    let proposal_votes = ProposalVote {
        against_votes: 1000,
        for_votes: 0,
        abstain_votes: 0,
    };
    let response = contract.proposal_votes(proposal_id).unwrap();
    assert_eq!(response, proposal_votes);
}

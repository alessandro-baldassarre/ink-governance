use ink::{
    codegen::Env,
    env::{
        hash::Blake2x256,
        test::{
            DefaultAccounts,
            EmittedEvent,
        },
        DefaultEnvironment,
    },
    prelude::vec::Vec,
};

use crate::gov_settings::*;
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
    governor_settings::*,
    governor_voting_group::*,
};

type Event = <Contract as ::ink::reflect::ContractEventBase>::Type;

fn default_accounts() -> DefaultAccounts<DefaultEnvironment> {
    accounts()
}

fn set_caller(sender: AccountId) {
    change_caller(sender)
}

fn build_contract() -> Contract {
    let accounts = default_accounts();

    let alice_member = VotingMember {
        account: accounts.alice,
        voting_power: 1,
    };
    let bob_member = VotingMember {
        account: accounts.bob,
        voting_power: 1,
    };

    let init_members = vec![alice_member, bob_member];

    set_caller(accounts.alice);

    Contract::new(None, init_members, 0, 50400, 0).unwrap()
}

fn decode_events(emittend_events: Vec<EmittedEvent>) -> Vec<Event> {
    emittend_events
        .into_iter()
        .map(|event| {
            <Event as scale::Decode>::decode(&mut &event.data[..]).expect("invalid data")
        })
        .collect()
}

fn propose(contract: &mut Contract) -> ProposalId {
    let accounts = default_accounts();

    set_caller(accounts.bob);
    let proposal = Proposal::default();
    let description = String::from("Test proposal");
    contract.propose(proposal, description).unwrap()
}

fn cast_against_vote(contract: &mut Contract, proposal_id: ProposalId) -> u64 {
    ink::env::test::advance_block::<DefaultEnvironment>();
    contract.cast_vote(proposal_id, 1).unwrap()
}

#[ink::test]
/// The constructor does its job
fn contruction_works() {
    let accounts = default_accounts();

    let alice_member = VotingMember {
        account: accounts.alice,
        voting_power: 1,
    };
    let contract = build_contract();

    let response = contract.get_members(vec![accounts.alice]).unwrap();
    assert_eq!(response, vec![alice_member]);

    let err_response = contract.get_members(vec![accounts.charlie]).unwrap_err();
    assert_eq!(err_response, VotingGroupError::NoMember);
}

#[ink::test]
/// The update_members method works correctly
fn update_members_works() {
    let accounts = default_accounts();

    let alice_member = VotingMember {
        account: accounts.alice,
        voting_power: 1,
    };
    let bob_member = VotingMember {
        account: accounts.bob,
        voting_power: 1,
    };
    let charlie_member = VotingMember {
        account: accounts.charlie,
        voting_power: 1,
    };
    let members = vec![alice_member.clone(), bob_member.clone()];
    let mut contract = build_contract();

    set_caller(accounts.bob);

    let err_response = contract.update_members(members, vec![]).unwrap_err();

    assert_eq!(err_response, VotingGroupError::OnlyAdminOrGovernance);

    set_caller(accounts.alice);

    contract
        .update_members(vec![charlie_member.clone()], vec![])
        .unwrap();

    let response = contract.get_members(vec![accounts.charlie]).unwrap();

    assert!(response.contains(&charlie_member));

    contract
        .update_members(vec![], vec![accounts.charlie])
        .unwrap();

    let err_response = contract.get_members(vec![accounts.charlie]).unwrap_err();
    assert_eq!(err_response, VotingGroupError::NoMember);
}

#[ink::test]
/// Propose works correctly
fn propose_works() {
    let accounts = default_accounts();
    let mut contract = build_contract();

    set_caller(accounts.charlie);
    let err_response = contract
        .propose(Proposal::default(), String::from(""))
        .unwrap_err();
    assert_eq!(err_response, GovernorError::NoVotes);

    set_caller(accounts.bob);
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

    let emittend_events = ink::env::test::recorded_events().collect::<Vec<_>>();
    let decoded_events = decode_events(emittend_events);
    if let Event::ProposalCreated(ProposalCreated {
        proposer,
        proposal_id: prop_id,
        proposal: prop,
        start_block,
        end_block,
        description: des,
    }) = &decoded_events[3]
    {
        assert_eq!(proposer, &accounts.bob);
        assert_eq!(prop_id, &proposal_id);
        assert_eq!(prop, &proposal);
        assert_eq!(start_block, &0);
        assert_eq!(end_block, &50400);
        assert_eq!(des, &description);
    } else {
        panic!("encountered unexpected event kind: expected a ProposalCreated event")
    }

    // In this case it is right that the proposal remains pending because since the number of blocks does not increase automatically,
    // the proposal does not even start
    let proposal_state = ProposalState::Pending;
    let response = contract.state(proposal_id).unwrap();
    assert_eq!(response, proposal_state);

    // then advance one block (note: we set voting_delay = 0 blocks)
    ink::env::test::advance_block::<DefaultEnvironment>();
    let proposal_state = ProposalState::Active;
    let response = contract.state(proposal_id).unwrap();
    assert_eq!(response, proposal_state);
}

#[ink::test]
/// Cast vote works correctly
fn cast_vote_works() {
    let accounts = default_accounts();
    let mut contract = build_contract();

    let proposal_id = propose(&mut contract);

    // In this case charlie is not part of the group and therefore cannot vote on the proposal
    set_caller(accounts.charlie);
    let response = contract.cast_vote(proposal_id, 1).unwrap_err();
    assert_eq!(response, GovernorError::NoVotes);

    // In this case alice is part of the group but the proposal is not yet active.
    set_caller(accounts.alice);
    let err_response = contract.cast_vote(proposal_id, 1).unwrap_err();
    assert_eq!(err_response, GovernorError::ProposalNotActive);

    // then advance one block (note: we set vote_delay = 0 blocks)
    ink::env::test::advance_block::<DefaultEnvironment>();
    let response = contract.cast_vote(proposal_id, 1).unwrap();
    assert_eq!(response, 1);

    let proposal_votes = ProposalVote {
        against_votes: 1,
        for_votes: 0,
        abstain_votes: 0,
    };
    let response = contract.proposal_votes(proposal_id).unwrap();
    assert_eq!(response, proposal_votes);
}

#[ink::test]
fn proposal_votes_works() {
    let mut contract = build_contract();
    let proposal_id = propose(&mut contract);
    cast_against_vote(&mut contract, proposal_id);

    let proposal_votes = ProposalVote {
        against_votes: 1,
        for_votes: 0,
        abstain_votes: 0,
    };
    let response = contract.proposal_votes(proposal_id).unwrap();
    assert_eq!(response, proposal_votes);
}

#[ink::test]
fn has_voted_works() {
    let mut contract = build_contract();
    let accounts = default_accounts();
    let proposal_id = propose(&mut contract);
    cast_against_vote(&mut contract, proposal_id);

    let response = contract.has_voted(proposal_id, accounts.bob);
    assert_eq!(response, true);
}

#[ink::test]
fn voting_delay_works() {
    let contract = build_contract();
    let response = contract.voting_delay();
    assert_eq!(response, 0);
}

#[ink::test]
fn voting_period_works() {
    let contract = build_contract();
    let response = contract.voting_period();
    assert_eq!(response, 50400);
}

#[ink::test]
fn proposal_threshold_works() {
    let contract = build_contract();
    let response = contract.proposal_threshold();
    assert_eq!(response, 0);
}

#[ink::test]
fn set_voting_delay_works() {
    let mut contract = build_contract();
    // In this case since we are in an off-chain envoriment, the modifier only_governance is not applied
    // and so we can set new delay without a passed proposal.
    contract.set_voting_delay(2).unwrap();
    let response = contract.voting_delay();
    assert_eq!(response, 2);
    let emittend_events = ink::env::test::recorded_events().collect::<Vec<_>>();
    let decoded_events = decode_events(emittend_events);
    if let Event::VotingDelaySet(VotingDelaySet {
        old_voting_delay,
        new_voting_delay,
    }) = &decoded_events[3]
    {
        assert_eq!(old_voting_delay, &0);
        assert_eq!(new_voting_delay, &2);
    } else {
        panic!("encountered unexpected event kind: expected a VotingDelaySet event")
    }
}

#[ink::test]
fn set_voting_period_works() {
    let mut contract = build_contract();
    // In this case since we are in an off-chain envoriment, the modifier only_governance is not applied
    // and so we can set new delay without a passed proposal.
    contract.set_voting_period(9).unwrap();
    let response = contract.voting_period();
    assert_eq!(response, 9);
    let emittend_events = ink::env::test::recorded_events().collect::<Vec<_>>();
    let decoded_events = decode_events(emittend_events);
    if let Event::VotingPeriodSet(VotingPeriodSet {
        old_voting_period,
        new_voting_period,
    }) = &decoded_events[3]
    {
        assert_eq!(old_voting_period, &50400);
        assert_eq!(new_voting_period, &9);
    } else {
        panic!("encountered unexpected event kind: expected a VotingDelaySet event")
    }
}

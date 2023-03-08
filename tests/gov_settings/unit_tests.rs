use ink::{
    env::{
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
    traits::AccountId,
};

use ink_governance::{
    governor::*,
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

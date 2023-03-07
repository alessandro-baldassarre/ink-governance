use crate::psp22_votes::*;

use ink_governance::{
    governor::utils::votes::votes_external::Votes,
    psp22::extensions::votes::{
        psp22votes_external::PSP22Votes,
        *,
    },
};
use openbrush::contracts::psp22::psp22_external::PSP22;

use ink_e2e::build_message;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn e2e_can_instantiate(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

    // Instantiate
    let constructor = ContractRef::new(1000);
    let contract_acc_id = client
        .instantiate("psp22_votes", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // Get total supply
    let total_supply = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.total_supply());
    let total_supply_res = client
        .call_dry_run(&ink_e2e::alice(), &total_supply, 0, None)
        .await
        .return_value();
    assert_eq!(total_supply_res, 1000);

    // Get minter balance
    let minter_balance = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.balance_of(alice));
    let minter_balance_res = client
        .call_dry_run(&ink_e2e::alice(), &minter_balance, 0, None)
        .await
        .return_value();
    assert_eq!(minter_balance_res, 1000);

    // Get minter votes
    let minter_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.get_votes(alice));
    let minter_votes_res = client
        .call_dry_run(&ink_e2e::alice(), &minter_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(minter_votes_res, 1000);

    // Get minter checkpoints
    let minter_checkpoints = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.checkpoints(alice, 0));
    let minter_checkpoints_res = client
        .call_dry_run(&ink_e2e::alice(), &minter_checkpoints, 0, None)
        .await
        .return_value()
        .unwrap();
    let expected_checkpoint = Checkpoint {
        from_block: 1,
        votes: 1000,
    };
    assert_eq!(minter_checkpoints_res, expected_checkpoint);

    Ok(())
}

#[ink_e2e::test]
async fn e2e_can_delegate_votes(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    let bob = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

    // Instantiate
    let constructor = ContractRef::new(1000);
    let contract_acc_id = client
        .instantiate("psp22_votes", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // Delegate votes from \\Alice to \\Bob
    let delegate_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.delegate(bob));
    let delegate_response = client
        .call(&ink_e2e::alice(), delegate_votes, 0, None)
        .await
        .expect("delegate failed");

    // Filter the events
    let contract_emitted_event = delegate_response
        .events
        .iter()
        .find(|event| {
            event
                .as_ref()
                .expect("Expect Event")
                .event_metadata()
                .event()
                == "ContractEmitted"
        })
        .expect("Expect ContractEmitted event")
        .unwrap();

    // Decode to the expected event type (skip field_context)
    let event = contract_emitted_event.field_bytes();
    let decoded_event =
        <DelegateVotesChanged as scale::Decode>::decode(&mut &event[35..])
            .expect("invalid data");

    // Destructor
    let DelegateVotesChanged {
        delegate,
        previous_balance,
        new_balance,
    } = decoded_event;

    // Assert with the expected value
    assert_eq!(delegate, alice);
    assert_eq!(previous_balance, 1000);
    assert_eq!(new_balance, 0);

    // Get delegator(\\Alice) votes
    let delegator_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.get_votes(alice));
    let delegator_votes_res = client
        .call_dry_run(&ink_e2e::alice(), &delegator_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(delegator_votes_res, 0);

    // Get delegatee(\\Bob) votes
    let delegatee_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.get_votes(bob));
    let delegatee_votes_res = client
        .call_dry_run(&ink_e2e::alice(), &delegatee_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(delegatee_votes_res, 1000);

    // Get delegator checkpoint after delegation
    let delegator_checkpoints = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.checkpoints(alice, 1));
    let delegator_checkpoints_res = client
        .call_dry_run(&ink_e2e::alice(), &delegator_checkpoints, 0, None)
        .await
        .return_value()
        .unwrap();
    let expected_checkpoint = Checkpoint {
        from_block: 2,
        votes: 0,
    };
    assert_eq!(delegator_checkpoints_res, expected_checkpoint);

    // Get delegatee checkpoint after delegation
    let delegatee_checkpoints = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.checkpoints(bob, 0));
    let delegatee_checkpoints_res = client
        .call_dry_run(&ink_e2e::alice(), &delegatee_checkpoints, 0, None)
        .await
        .return_value()
        .unwrap();
    let expected_checkpoint = Checkpoint {
        from_block: 2,
        votes: 1000,
    };
    assert_eq!(delegatee_checkpoints_res, expected_checkpoint);

    // Get delegator past votes
    // before delegation
    let delegator_past_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.get_past_votes(alice, 1));
    let delegator_past_votes_res = client
        .call_dry_run(&ink_e2e::alice(), &delegator_past_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(delegator_past_votes_res, 1000);
    // after delegation
    let delegator_past_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.get_past_votes(alice, 2));
    let delegator_past_votes_res = client
        .call_dry_run(&ink_e2e::alice(), &delegator_past_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(delegator_past_votes_res, 0);

    // Get delegatee past votes
    // before delegation
    let delegatee_past_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.get_past_votes(bob, 1));
    let delegatee_past_votes_res = client
        .call_dry_run(&ink_e2e::alice(), &delegatee_past_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(delegatee_past_votes_res, 0);
    // after delegation
    let delegatee_past_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.get_past_votes(bob, 2));
    let delegatee_past_votes_res = client
        .call_dry_run(&ink_e2e::alice(), &delegatee_past_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(delegatee_past_votes_res, 1000);

    Ok(())
}

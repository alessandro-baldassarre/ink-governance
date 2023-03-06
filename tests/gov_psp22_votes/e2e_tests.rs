use crate::governance_v1::*;
use openbrush_governance::{
    governor::*,
    governor_counting_simple::*,
    governor_voting_group::*,
};

use hex::FromHex;
use ink::blake2x256;
use ink_e2e::build_message;

use openbrush::traits::Hash;

use openbrush_governance::{
    governor::governor_external::Governor,
    governor_counting_simple::countingsimple_external::CountingSimple,
    governor_voting_group::votinggroup_external::VotingGroup,
};

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Test to cover the ability to update the members of the voting group.
#[ink_e2e::test]
async fn e2e_can_update_members(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    let bob = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
    let charlie = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

    let alice_member = VotingMember {
        account: alice,
        voting_power: 1,
    };
    let bob_member = VotingMember {
        account: bob,
        voting_power: 1,
    };
    let init_members = vec![alice_member.clone(), bob_member.clone()];
    let constructor = GovernorStructRef::new(None, init_members);
    let contract_acc_id = client
        .instantiate("governance_v1", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let alice_updated = VotingMember {
        account: alice,
        voting_power: 2,
    };

    // Try to update the members through a call from Bob and the call should fail,
    // Bob is not the admin of the group and has not proposed the update via governance.
    let update_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.update_members(vec![alice_updated], vec![]));

    let update_members_err = client
        .call_dry_run(&ink_e2e::bob(), &update_members, 0, None)
        .await;

    assert!(update_members_err.exec_return_value().did_revert());

    assert_eq!(
        update_members_err.return_value().unwrap_err(),
        VotingGroupError::OnlyAdminOrGovernance
    );

    // Try to update the members through a call from Alice and the call should pass because
    // Alice is the admin of the group
    client
        .call(&ink_e2e::alice(), update_members, 0, None)
        .await
        .expect("update_members failed");

    let get_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.get_members(vec![alice]));

    let get_members_res = client
        .call_dry_run(&ink_e2e::alice(), &get_members, 0, None)
        .await
        .return_value()
        .unwrap();

    // Assert that the member was updated correctly
    assert_eq!(get_members_res, vec![alice_updated]);

    let charlie_member = VotingMember {
        account: charlie,
        voting_power: 2,
    };

    // Try to add new member
    let add_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.update_members(vec![charlie_member], vec![]));

    client
        .call(&ink_e2e::alice(), add_members, 0, None)
        .await
        .expect("add_members failed");

    let get_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.get_members(vec![charlie]));

    let get_members_res = client
        .call_dry_run(&ink_e2e::alice(), &get_members, 0, None)
        .await
        .return_value()
        .unwrap();

    // Assert that the member was add correctly
    assert_eq!(get_members_res, vec![charlie_member]);

    // Try to remove a member
    let remove_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.update_members(vec![], vec![charlie]));

    client
        .call(&ink_e2e::alice(), remove_members, 0, None)
        .await
        .expect("remove_members failed");

    let get_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.get_members(vec![charlie]));

    let get_members_res_err = client
        .call_dry_run(&ink_e2e::alice(), &get_members, 0, None)
        .await
        .return_value()
        .unwrap_err();

    // Assert that the member was removed correctly
    assert_eq!(get_members_res_err, VotingGroupError::NoMember);

    Ok(())
}

// Test to cover the ability to propose a new Proposal
#[ink_e2e::test]
async fn e2e_can_propose(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    let bob = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

    let alice_member = VotingMember {
        account: alice,
        voting_power: 1,
    };
    let bob_member = VotingMember {
        account: bob,
        voting_power: 1,
    };
    let init_members = vec![alice_member.clone(), bob_member.clone()];
    let constructor = GovernorStructRef::new(None, init_members);
    let contract_acc_id = client
        .instantiate("governance_v1", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // Build a proposal message
    let proposal = Proposal::default();
    let description = openbrush::traits::String::from("Test proposal");

    let propose = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.propose(proposal.clone(), description.clone()));

    let proposal_id = client
        .call_dry_run(&ink_e2e::bob(), &propose, 0, None)
        .await
        .return_value()
        .unwrap();

    // Try to propose through a call from Charlie and the call should fail,
    // Charlie is not a member of the group.
    let propose_err = client
        .call_dry_run(&ink_e2e::charlie(), &propose, 0, None)
        .await;

    assert!(propose_err.exec_return_value().did_revert());

    assert_eq!(
        propose_err.return_value().unwrap_err(),
        GovernorError::NoVotes
    );

    // Propose
    let propose_response = client
        .call(&ink_e2e::bob(), propose, 0, None)
        .await
        .unwrap();

    // Filter the events
    let contract_emitted_event = propose_response
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
    let decoded_event = <ProposalCreated as scale::Decode>::decode(&mut &event[35..])
        .expect("invalid data");

    // Destructor
    let ProposalCreated {
        proposer,
        proposal_id: prop_id,
        proposal: prop,
        start_block,
        end_block,
        description: des,
    } = decoded_event;

    // Assert with the expected value
    assert_eq!(proposer, bob);
    assert_eq!(prop_id, proposal_id);
    assert_eq!(prop, proposal);
    assert_eq!(start_block, 2);
    assert_eq!(end_block, 4);
    assert_eq!(des, description);

    let proposal_state = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));

    let proposal_state_res = client
        .call_dry_run(&ink_e2e::bob(), &proposal_state, 0, None)
        .await
        .return_value()
        .unwrap();

    // The proposal must be pending until the next block
    assert_eq!(proposal_state_res, ProposalState::Pending);

    // Do an extrinsinc to advance the block (instant_finality)
    // TODO: delete if ink_e2e update
    client
        .call(&ink_e2e::bob(), proposal_state, 0, None)
        .await
        .unwrap();

    let proposal_state = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));

    let proposal_state_res = client
        .call_dry_run(&ink_e2e::bob(), &proposal_state, 0, None)
        .await
        .return_value()
        .unwrap();

    // Assert the proposal is active
    assert_eq!(proposal_state_res, ProposalState::Active);

    Ok(())
}

// Test to cover the ability to cast a vote on a Proposal.
#[ink_e2e::test]
async fn e2e_can_cast_vote(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    let bob = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

    let alice_member = VotingMember {
        account: alice,
        voting_power: 1,
    };
    let bob_member = VotingMember {
        account: bob,
        voting_power: 1,
    };
    let init_members = vec![alice_member.clone(), bob_member.clone()];
    let constructor = GovernorStructRef::new(None, init_members);
    let contract_acc_id = client
        .instantiate("governance_v1", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // Build a proposal message
    let proposal = Proposal::default();
    let description = String::from("Test proposal");

    let propose = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.propose(proposal.clone(), description.clone().into()));

    // Propose
    let proposal_id = client
        .call_dry_run(&ink_e2e::bob(), &propose, 0, None)
        .await
        .return_value()
        .unwrap();
    client
        .call(&ink_e2e::bob(), propose, 0, None)
        .await
        .unwrap();

    // Do an extrinsinc to advance the block (instant_finality)
    // TODO: delete if ink_e2e update
    let proposal_state = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));
    client
        .call(&ink_e2e::bob(), proposal_state, 0, None)
        .await
        .unwrap();

    // Build a vote(Against) message
    let against_vote = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.cast_vote(proposal_id, 1));

    // Cast Vote
    client
        .call(&ink_e2e::alice(), against_vote, 0, None)
        .await
        .unwrap();

    // Assert that the vote was submitted

    // Build a has_voted message
    let has_voted = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.has_voted(proposal_id, alice));

    let has_voted_response = client
        .call_dry_run(&ink_e2e::alice(), &has_voted, 0, None)
        .await
        .return_value();

    assert!(has_voted_response);

    // Assert that the vote submitted is correct

    // Build a proposal_votes message
    let proposal_votes = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.proposal_votes(proposal_id));

    let proposal_votes_response = client
        .call_dry_run(&ink_e2e::alice(), &proposal_votes, 0, None)
        .await
        .return_value()
        .unwrap();

    assert_eq!(
        proposal_votes_response,
        ProposalVote {
            against_votes: 1,
            for_votes: 0,
            abstain_votes: 0
        }
    );

    Ok(())
}

// Test to cover the complete flow of Governor:
// 1) Propose: a group member propose to add a new member
// 2) Vote: the proposal is voted in favour
// 3) Execute: execute the succeeded proposal
#[ink_e2e::test]
async fn e2e_can_propose_vote_execute(
    mut client: ink_e2e::Client<C, E>,
) -> E2EResult<()> {
    let alice = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
    let bob = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
    let charlie = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

    let alice_member = VotingMember {
        account: alice,
        voting_power: 1,
    };
    let bob_member = VotingMember {
        account: bob,
        voting_power: 1,
    };
    let init_members = vec![alice_member.clone(), bob_member.clone()];
    let constructor = GovernorStructRef::new(None, init_members);
    let contract_acc_id = client
        .instantiate("governance_v1", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let charlie_member = VotingMember {
        account: charlie,
        voting_power: 1,
    };

    // Build the proposal to add a new member (charlie)

    // Encode the parameters to pass in the selector (function)
    let update_members = vec![charlie_member];
    let remove_members: Vec<u8> = Vec::new();

    let mut input = scale::Encode::encode(&update_members);
    let mut input2 = scale::Encode::encode(&remove_members);

    input.append(&mut input2);

    // Decode the selector hex 4 bytes
    let selector_hex = "24990c25";
    let selector = <[u8; 4]>::from_hex(selector_hex).expect("Decoding failed");

    let proposal = Proposal {
        callee: contract_acc_id,
        selector,
        input,
        transferred_value: 0,
    };
    let description = String::from("Test proposal");
    let description_hash = Hash::try_from(blake2x256!("Test proposal")).unwrap();
    let propose = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.propose(proposal.clone(), description.clone().into()));

    // Propose
    let proposal_id = client
        .call_dry_run(&ink_e2e::bob(), &propose, 0, None)
        .await
        .return_value()
        .unwrap();
    client
        .call(&ink_e2e::bob(), propose, 0, None)
        .await
        .unwrap();

    // Do an extrinsinc to advance the block (instant_finality)
    // TODO: delete if ink_e2e update
    let proposal_state = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));
    client
        .call(&ink_e2e::bob(), proposal_state, 0, None)
        .await
        .unwrap();

    // Build a vote(For) message
    let for_vote = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.cast_vote(proposal_id, 2));

    // Cast Vote
    client
        .call(&ink_e2e::alice(), for_vote, 0, None)
        .await
        .unwrap();

    // Do an extrinsinc to advance the block (instant_finality)
    // TODO: delete if ink_e2e update
    let proposal_state = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));
    client
        .call(&ink_e2e::bob(), proposal_state, 0, None)
        .await
        .unwrap();

    let proposal_state = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));

    let proposal_state_res = client
        .call_dry_run(&ink_e2e::bob(), &proposal_state, 0, None)
        .await
        .return_value()
        .unwrap();

    // Assert the proposal is Succeeded
    assert_eq!(proposal_state_res, ProposalState::Succeeded);

    // Execute
    let execute = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.execute(proposal.clone(), description_hash));

    client
        .call(&ink_e2e::alice(), execute, 0, None)
        .await
        .unwrap();

    let get_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.get_members(vec![charlie]));

    let get_members_res = client
        .call_dry_run(&ink_e2e::alice(), &get_members, 0, None)
        .await
        .return_value()
        .unwrap();

    // Assert that the member was add correctly
    assert_eq!(get_members_res, vec![charlie_member]);

    Ok(())
}

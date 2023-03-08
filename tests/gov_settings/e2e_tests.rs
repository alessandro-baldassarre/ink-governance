use crate::gov_settings::*;
use ink_governance::{
    governor::*,
    governor_voting_group::*,
};

use hex::FromHex;
use ink::blake2x256;
use ink_e2e::build_message;

use openbrush::traits::Hash;

use ink_governance::governor::governor_external::Governor;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Test to cover the complete flow of Governor:
// 1) Propose: a group member propose to set a new voting_period
// 2) Vote: the proposal is voted in favour
// 3) Execute: execute the succeeded proposal
#[ink_e2e::test]
async fn e2e_can_update_settings(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
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
    let constructor = ContractRef::new(None, init_members, 0, 2, 0);
    let contract_acc_id = client
        .instantiate("gov_settings", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // Build the proposal to set a new voting period

    // Encode the parameters to pass in the selector (function)
    let new_voting_period: u32 = 3;

    let input = scale::Encode::encode(&new_voting_period);

    // Decode the selector hex 4 bytes
    let selector_hex = "97a1433e";
    let selector = <[u8; 4]>::from_hex(selector_hex).expect("Decoding failed");

    let proposal = Proposal {
        callee: contract_acc_id,
        selector,
        input,
        transferred_value: 0,
    };
    let description = String::from("Set a new voting period");
    let description_hash =
        Hash::try_from(blake2x256!("Set a new voting period")).unwrap();
    let propose = build_message::<ContractRef>(contract_acc_id.clone())
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
    let proposal_state = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));
    client
        .call(&ink_e2e::bob(), proposal_state, 0, None)
        .await
        .unwrap();

    // Build a vote(For) message
    let for_vote = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.cast_vote(proposal_id, 2));

    // Cast Vote
    client
        .call(&ink_e2e::alice(), for_vote, 0, None)
        .await
        .unwrap();

    // Do an extrinsinc to advance the block (instant_finality)
    // TODO: delete if ink_e2e update
    let proposal_state = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));
    client
        .call(&ink_e2e::bob(), proposal_state, 0, None)
        .await
        .unwrap();

    let proposal_state = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.state(proposal_id));

    let proposal_state_res = client
        .call_dry_run(&ink_e2e::bob(), &proposal_state, 0, None)
        .await
        .return_value()
        .unwrap();

    // Assert the proposal is Succeeded
    assert_eq!(proposal_state_res, ProposalState::Succeeded);

    // Execute
    let execute = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.execute(proposal.clone(), description_hash));

    client
        .call(&ink_e2e::alice(), execute, 0, None)
        .await
        .unwrap();

    let voting_period = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.voting_period());

    let voting_period_res = client
        .call_dry_run(&ink_e2e::alice(), &voting_period, 0, None)
        .await
        .return_value();

    // Assert that the member was add correctly
    assert_eq!(voting_period_res, 3);

    Ok(())
}

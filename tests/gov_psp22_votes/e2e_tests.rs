use crate::gov_psp22_votes::*;
use ink_governance::governor::*;

use hex::FromHex;
use ink::blake2x256;
use ink_e2e::build_message;

use openbrush::traits::{
    Balance,
    Hash,
};

use openbrush::contracts::psp22::psp22_external::PSP22;

use ink_governance::{
    self,
    governor::{
        governor_external::Governor,
        utils::votes::votes_external::Votes,
    },
};

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn e2e_can_instantiate(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // Set caller to \\Alice
    let caller = &ink_e2e::alice();
    let caller_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

    let init_supply: Balance = 1000;

    // Instantiate the contract with init_supply
    let constructor = ContractRef::new(init_supply);
    let contract_acc_id = client
        .instantiate("gov_psp22_votes", caller, constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // Assert total supply equal to init supply
    let total_supply = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.total_supply());
    let total_supply_res = client
        .call_dry_run(caller, &total_supply, 0, None)
        .await
        .return_value();
    assert_eq!(total_supply_res, init_supply);

    // Assert caller(minter) balance is equal to init supply
    let minter_balance = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.balance_of(caller_acc_id));
    let minter_balance_res = client
        .call_dry_run(caller, &minter_balance, 0, None)
        .await
        .return_value();
    assert_eq!(minter_balance_res, init_supply);

    // Assert governance votes of minter is equal to init supply
    let minter_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| Governor::get_votes(gov, caller_acc_id, 1));
    let minter_votes_res = client
        .call_dry_run(caller, &minter_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(minter_votes_res, 1000);

    Ok(())
}

#[ink_e2e::test]
async fn e2e_can_delegate_votes(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    // Set caller to \\Alice
    let caller = &ink_e2e::alice();
    let caller_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);

    let init_supply: Balance = 1000;

    // Instantiate the contract with init_supply
    let constructor = ContractRef::new(init_supply);
    let contract_acc_id = client
        .instantiate("gov_psp22_votes", caller, constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    let bob_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

    // Delegate votes to \\Bob
    let delegate = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.delegate(bob_acc_id));

    client.call(caller, delegate, 0, None).await.unwrap();

    // Assert governance votes delegate to \\Bob
    let delegatee_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| Governor::get_votes(gov, bob_acc_id, 2));
    let delegatee_votes_res = client
        .call_dry_run(caller, &delegatee_votes, 0, None)
        .await
        .return_value()
        .unwrap();
    assert_eq!(delegatee_votes_res, 1000);

    // Assert delegator has no governance votes
    let delegator_votes = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| Governor::get_votes(gov, caller_acc_id, 2));
    let delegator_votes_res = client
        .call_dry_run(caller, &delegator_votes, 0, None)
        .await
        .return_value()
        .unwrap_err();
    assert_eq!(delegator_votes_res, GovernorError::NoVotes);

    Ok(())
}

// Test to cover the complete flow of Governor:
// 1) Mint: a caller(\\Alice) mint new tokens
// 2) Delegate: the minter(\\Alice) delegates votes to (\\Bob) account
// 3) Transfer: the minter(\\Alice) transfers token to a third account(\\Charlie)
// 4) Propose: the delegatee(\\Bob) propose a transfer from delegator to another account (\\Dave)
// 5) Vote: (\\Charlie) votes against and (\\Bob) in favour
// 6) Execute: after the proposal was succeeded execute the transfer to (\\Dave)
#[ink_e2e::test]
async fn e2e_can_complete_total_process(
    mut client: ink_e2e::Client<C, E>,
) -> E2EResult<()> {
    let bob_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
    let charlie_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);
    let dave_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Dave);

    // 1)
    // Instantiate(Mint) the contract with init supply
    let constructor = ContractRef::new(1000);
    let contract_acc_id = client
        .instantiate("gov_psp22_votes", &ink_e2e::alice(), constructor, 0, None)
        .await
        .expect("instantiate failed")
        .account_id;

    // 2)
    // Delegate votes to \\Bob
    let delegate = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.delegate(bob_acc_id));

    client
        .call(&ink_e2e::alice(), delegate, 0, None)
        .await
        .unwrap();

    // 3)
    // Transfer tokens to \\Charlie
    let transfer = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.transfer(charlie_acc_id, 100, Vec::new()));

    client
        .call(&ink_e2e::alice(), transfer, 0, None)
        .await
        .unwrap();

    // Transfer tokens from minter to dao to perform dao proposal
    let transfer_to_dao = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.transfer(contract_acc_id, 200, Vec::new()));

    client
        .call(&ink_e2e::alice(), transfer_to_dao, 0, None)
        .await
        .unwrap();

    // Assert tokens was transfered correctly
    let dao_balance = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.balance_of(contract_acc_id));
    let dao_balance_res = client
        .call_dry_run(&ink_e2e::alice(), &dao_balance, 0, None)
        .await
        .return_value();
    assert_eq!(dao_balance_res, 200);

    // 4)
    // Build the proposal to transfer token to \\Dave

    // Encode the parameters to pass in the selector (function)
    let to = dave_acc_id;
    let value: Balance = 100;
    let data: Vec<u8> = Vec::new();

    let mut input = scale::Encode::encode(&to);
    let mut input2 = scale::Encode::encode(&value);
    let mut input3 = scale::Encode::encode(&data);

    input.append(&mut input2);
    input.append(&mut input3);

    // Decode the selector hex 4 bytes
    let selector_hex = "db20f9f5";
    let selector = <[u8; 4]>::from_hex(selector_hex).expect("Decoding failed");

    let proposal = Proposal {
        callee: contract_acc_id,
        selector,
        input,
        transferred_value: 0,
    };
    let description = String::from("Test proposal");
    let description_hash = Hash::try_from(blake2x256!("Test proposal")).unwrap();
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

    // 5)
    // Build a vote(Against) message
    let against_vote = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.cast_vote(proposal_id, 1));

    // Cast Vote
    client
        .call(&ink_e2e::charlie(), against_vote, 0, None)
        .await
        .unwrap();

    // Build a vote(For) message
    let for_vote = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.cast_vote(proposal_id, 2));

    // Cast Vote
    client
        .call(&ink_e2e::bob(), for_vote, 0, None)
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

    // 6)
    // Execute
    let execute = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.execute(proposal.clone(), description_hash));

    client
        .call(&ink_e2e::alice(), execute, 0, None)
        .await
        .unwrap();

    // Assert tokens was transfered correctly
    let dave_balance = build_message::<ContractRef>(contract_acc_id.clone())
        .call(|gov| gov.balance_of(dave_acc_id));
    let dave_balance_res = client
        .call_dry_run(&ink_e2e::alice(), &dave_balance, 0, None)
        .await
        .return_value();
    assert_eq!(dave_balance_res, 100);

    Ok(())
}

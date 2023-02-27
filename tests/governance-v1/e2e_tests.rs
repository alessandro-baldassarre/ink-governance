use crate::governance_v1::*;
use openbrush_governance::{
    governor::*,
    governor_counting_simple::*,
    governor_voting_group::*,
};

use ink_e2e::build_message;

use openbrush_governance::governor_voting_group::votinggroup_external::VotingGroup;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn e2e_can_update_members(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
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

    let alice_updated = VotingMember {
        account: alice,
        voting_power: 2,
    };

    // First we try to update the members through a call from Bob and the call should fail,
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

    client
        .call(&ink_e2e::alice(), update_members, 0, None)
        .await
        .expect("update_members failed");

    let get_members = build_message::<GovernorStructRef>(contract_acc_id.clone())
        .call(|gov| gov.get_members(vec![alice]));

    let get_members_res = client
        .call_dry_run(&ink_e2e::alice(), &get_members, 0, None)
        .await;

    assert_eq!(get_members_res.return_value().unwrap(), vec![alice_updated]);

    Ok(())
}

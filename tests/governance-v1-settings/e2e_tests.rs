use ink_governance::governor_voting_group::VotingMember;

use crate::governance_v1_settings::GovernorStructRef;

type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[ink_e2e::test]
async fn e2e_can_add_members(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
    let alice_member = VotingMember {
        account: ink_e2e::account_id(ink_e2e::AccountKeyring::Alice),
        voting_power: 1,
    };
    let bob_member = VotingMember {
        account: ink_e2e::account_id(ink_e2e::AccountKeyring::Bob),
        voting_power: 1,
    };
    let init_members = vec![alice_member, bob_member];
    let constructor = GovernorStructRef::new(None, init_members, 0, 50400, 0);
    let contract_acc_id = client
        .instantiate(
            "governance_v1_settings",
            &ink_e2e::alice(),
            constructor,
            0,
            None,
        )
        .await
        .expect("instantiate failed")
        .account_id;

    Ok(())
}

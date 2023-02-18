#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_governor {
    use ink_governance::governor::modules::governor_counting_simple;
    use ink_governance::governor::modules::{governor_votes_members, governor_votes_members::*};
    use ink_governance::governor::{governor, governor::*, GovernorError};
    use openbrush::contracts::access_control::access_control;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        governor:
            governor::Data<governor_counting_simple::Counting, governor_votes_members::Voting>,
        #[storage_field]
        access_control: access_control::Data,
    }

    impl Governor for Contract {}

    impl VotingGroup for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(account: AccountId) -> Result<Self, GovernorError> {
            let mut instance = Self::default();

            let caller = Self::env().caller();

            access_control::Internal::_init_with_admin(&mut instance, caller);

            governor_votes_members::VotingGroup::set_voting_power(&mut instance, account, None)?;

            Ok(instance)
        }
    }
}

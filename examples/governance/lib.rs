#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_governor {
    use ink::prelude::vec::Vec;
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
        /// Construct the contract with a list of members and optional voting power (if omitted the
        /// value is 1 by default) and an optional admin (if omitted the caller account is set by
        /// default)
        #[ink(constructor)]
        pub fn new(
            admin: Option<AccountId>,
            members: Vec<(AccountId, Option<u64>)>,
        ) -> Result<Self, GovernorError> {
            let mut instance = Self::default();

            let admin = admin.unwrap_or(Self::env().caller());

            access_control::Internal::_init_with_admin(&mut instance, admin);

            for member in members {
                let (account, voting_power) = member;
                governor_votes_members::VotingGroup::set_voting_power(
                    &mut instance,
                    account,
                    voting_power.unwrap_or(1),
                )?;
            }

            Ok(instance)
        }
    }
}

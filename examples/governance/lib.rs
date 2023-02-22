#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_governor {
    use ink::prelude::vec::Vec;
    use ink_governance::governor::governor::*;
    use ink_governance::governor::modules::{
        governor_counting_simple::*, governor_voting_group::*,
    };
    use ink_governance::traits::errors::VotingGroupError;
    use openbrush::contracts::access_control::access_control;
    use openbrush::traits::{Storage, String};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        governor: governor::Data<governor_counting_simple::Counting, governor_voting_group::Voting>,
        #[storage_field]
        access_control: access_control::Data,
    }

    impl Governor for Contract {}

    impl VotingGroup for Contract {}

    impl CountingSimple for Contract {}

    // Override the internal methods
    impl governor::Internal for Contract {
        fn _voting_delay(&self) -> u32 {
            1 // 1 block
        }
        fn _voting_period(&self) -> u32 {
            50400 // 1 week
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        Custom(String),
        VotingGroupError(VotingGroupError),
    }

    impl From<VotingGroupError> for ContractError {
        fn from(_voting: VotingGroupError) -> Self {
            ContractError::Custom(String::from("VG: error from VotingGroup"))
        }
    }

    impl Contract {
        /// Initialize the contract with a list of voting members and optional admin (if not set
        /// the caller will be the admin by default)
        #[ink(constructor)]
        pub fn new(
            admin: Option<AccountId>,
            init_members: Vec<VotingMember>,
        ) -> Result<Self, ContractError> {
            let mut instance = Self::default();

            // Assign the admin role to the caller if is not set in the parameters
            let admin = admin.unwrap_or(Self::env().caller());

            // Initialize access_control with the admin.
            //
            // Note: some methods like update_members has the access control (only_role:admin).
            access_control::Internal::_init_with_admin(&mut instance, admin);

            // Initialize the group with the members.
            //
            // Note: Only the members of the group can propose or vote a proposal.
            governor_voting_group::VotingGroup::update_members(
                &mut instance,
                init_members,
                Vec::new(),
            )?;
            Ok(instance)
        }
    }
}

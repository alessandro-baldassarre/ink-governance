#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#[openbrush::contract]
pub mod governance_v1 {
    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        prelude::vec::Vec,
    };
    use openbrush::{
        contracts::psp22::{
            extensions::burnable::*,
            Transfer,
        },
        traits::{
            Storage,
            String,
            ZERO_ADDRESS,
        },
    };
    use openbrush_governance::psp22::extensions::votes::{
        self,
        *,
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct GovernorStruct {
        #[storage_field]
        psp22_votes: votes::Data,
        #[storage_field]
        psp22: psp22::Data,
    }

    impl Transfer for GovernorStruct {
        fn _after_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            amount: &Balance,
        ) -> Result<(), PSP22Error> {
            self._after_token_transfer_votes(from, to, amount).unwrap();
            Ok(())
        }
    }

    impl Votes for GovernorStruct {}
    impl PSP22 for GovernorStruct {}
    impl PSP22Votes for GovernorStruct {}
    impl PSP22Burnable for GovernorStruct {}

    impl GovernorStruct {
        /// Initialize the contract with a list of voting members and optional admin (if not set
        /// the caller will be the admin by default)
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            instance
                ._mint_to(Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }
    }
}

#[cfg(test)]
mod unit_tests;

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;

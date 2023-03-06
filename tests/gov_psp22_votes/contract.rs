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
            from: Option<&openbrush::traits::AccountId>,
            to: Option<&openbrush::traits::AccountId>,
            amount: &openbrush::traits::Balance,
        ) -> Result<(), PSP22Error> {
            match (from, to) {
                (Some(from), Some(to)) => {
                    self._move_voting_power(from, to, amount);
                    return Ok(())
                }
                (Some(from), None) => {
                    self._write_checkpoint(
                        None,
                        |a: Vote, b: Vote| -> Vote { a - b },
                        amount,
                    );
                    self._move_voting_power(from, &ZERO_ADDRESS.into(), amount);
                    return Ok(())
                }
                (None, Some(to)) => {
                    self._write_checkpoint(
                        None,
                        |a: Vote, b: Vote| -> Vote { a + b },
                        amount,
                    );
                    self._move_voting_power(&ZERO_ADDRESS.into(), to, amount);

                    return Ok(())
                }
                _ => {
                    return Err(PSP22Error::Custom(String::from("Zero address provided")))
                }
            }
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

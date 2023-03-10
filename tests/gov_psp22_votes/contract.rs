#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#[openbrush::contract]
pub mod gov_psp22_votes {

    use ink_governance::{
        governor::*,
        governor_counting_simple::*,
    };

    use ink_governance::psp22_votes::*;
    use openbrush::{
        contracts::psp22::{
            extensions::burnable::*,
            Transfer,
        },
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        governor: governor::Data<governor_counting_simple::Counting>,
        #[storage_field]
        psp22_votes: psp22_votes::Data,
        #[storage_field]
        psp22: psp22::Data,
    }

    impl Governor for Contract {}
    impl CountingSimple for Contract {}
    impl Votes for Contract {}
    impl PSP22 for Contract {}
    impl PSP22Votes for Contract {}
    impl PSP22Burnable for Contract {}

    impl Transfer for Contract {
        fn _after_token_transfer(
            &mut self,
            from: Option<&AccountId>,
            to: Option<&AccountId>,
            amount: &Balance,
        ) -> Result<(), PSP22Error> {
            self._after_token_transfer_votes(from, to, amount)
                .map_err(|_| PSP22Error::Custom(String::from("Error PSP22Votes")))?;
            Ok(())
        }
    }

    // Override the internal methods
    impl governor::Internal for Contract {
        fn _voting_delay(&self) -> u32 {
            0 // block
        }
        fn _voting_period(&self) -> u32 {
            2 // block (for testing purpose)
        }
        fn _get_votes(
            &self,
            account: &AccountId,
            block_number: BlockNumber,
            _params: &[u8],
        ) -> Result<u64, GovernorError> {
            let votes = self
                .get_past_votes(*account, block_number)
                .map_err(|_| GovernorError::NoVotes)?;
            // for explicity error
            if votes == 0 {
                return Err(GovernorError::NoVotes)
            }
            Ok(votes)
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        Custom(String),
    }

    impl Contract {
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

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#[openbrush::contract]
pub mod psp22_votes {

    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use ink_governance::psp22::extensions::votes::*;
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

    /// Emitted when an account changes their delegate.
    #[ink(event)]
    pub struct DelegateChanged {
        /// Account id of the delegator.
        #[ink(topic)]
        pub delegator: AccountId,
        /// Account id of the previous delegatee.
        pub from_delegate: Option<AccountId>,
        /// Account id of the new delegatee.
        pub to_delegate: AccountId,
    }

    /// Emitted when a token transfer or delegate change results in changes to a delegate's number
    /// of votes.
    #[ink(event)]
    pub struct DelegateVotesChanged {
        /// Account id of the delegate.
        #[ink(topic)]
        pub delegate: AccountId,
        /// Balance before the change.
        pub previous_balance: Balance,
        /// Balance after the change
        pub new_balance: Balance,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp22_votes: votes::Data,
        #[storage_field]
        psp22: psp22::Data,
    }

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

    impl votes::Internal for Contract {
        fn _emit_delegate_changed(
            &self,
            delegator: AccountId,
            from_delegate: Option<AccountId>,
            to_delegate: AccountId,
        ) {
            self.env().emit_event(DelegateChanged {
                delegator,
                from_delegate,
                to_delegate,
            })
        }
        fn _emit_delegate_votes_changed(
            &self,
            delegate: AccountId,
            previous_balance: Balance,
            new_balance: Balance,
        ) {
            self.env().emit_event(DelegateVotesChanged {
                delegate,
                previous_balance,
                new_balance,
            })
        }
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

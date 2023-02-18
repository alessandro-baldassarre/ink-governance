#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_access_control {
    use ink_governance::governor::modules::governor_counting_simple;
    use ink_governance::governor::modules::governor_votes_members;
    use ink_governance::governor::{governor, governor::*};
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        governor:
            governor::Data<governor_counting_simple::Counting, governor_votes_members::Voting>,
    }

    impl Governor for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let instance = Self::default();

            instance
        }
    }
}

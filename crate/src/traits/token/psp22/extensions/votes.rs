use openbrush::traits::AccountId;

use crate::traits::governance::utils::votes::*;

#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Checkpoint {
    pub from_block: u8,
    pub votes: u64,
}

#[openbrush::wrapper]
pub type PSP22VotesRef = dyn PSP22Votes + Votes;

#[openbrush::trait_definition]
pub trait PSP22Votes: Votes {
    /// Get the pos-th checkpoint for account.
    #[ink(message)]
    fn checkpoints(&self, account: AccountId, pos: u8) -> Checkpoint;

    /// Get number of checkpoints for account.
    #[ink(message)]
    fn num_checkpoints(&self, account: AccountId) -> u8;
}

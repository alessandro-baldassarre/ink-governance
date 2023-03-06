use openbrush::{
    contracts::traits::psp22::*,
    traits::{
        AccountId,
        BlockNumber,
    },
};

use crate::traits::{
    errors::PSP22VotesError,
    governance::utils::votes::*,
};

#[derive(Debug, Default, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Checkpoint {
    pub from_block: BlockNumber,
    pub votes: Vote,
}

#[openbrush::wrapper]
pub type PSP22VotesRef = dyn PSP22Votes + Votes + PSP22;

#[openbrush::trait_definition]
pub trait PSP22Votes: Votes + PSP22 {
    /// Get the pos-th checkpoint for account.
    #[ink(message)]
    fn checkpoints(
        &self,
        account: AccountId,
        pos: u32,
    ) -> Result<Checkpoint, PSP22VotesError>;

    /// Get number of checkpoints for account.
    #[ink(message)]
    fn num_checkpoints(&self, account: AccountId) -> Result<u32, PSP22VotesError>;
}

pub fn u32_to_usize(input: u32) -> Option<usize> {
    TryInto::<usize>::try_into(input).ok()
}

pub fn usize_to_u32(input: usize) -> Option<u32> {
    TryInto::<u32>::try_into(input).ok()
}

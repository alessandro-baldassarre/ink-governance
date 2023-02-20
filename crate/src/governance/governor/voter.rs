use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, BlockNumber};

pub use crate::traits::governor::*;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Voting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Voting {
    pub _reserved: Option<()>,
}

pub trait Voter {
    /// Get the voting weight of account at a specific blockNumber, for a vote as described by params.
    fn _get_votes(
        &mut self,
        account: AccountId,
        block_number: BlockNumber,
        params: Vec<u8>,
    ) -> Option<u64>;
}

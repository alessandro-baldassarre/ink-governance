use openbrush::traits::{
    AccountId,
    BlockNumber,
};

/// Trait that a "voter" sub-module must implement
pub trait Voter {
    /// Get the voting weight of account at a specific blockNumber, for a vote as described by params.
    fn _get_votes(
        &self,
        account: &AccountId,
        block_number: BlockNumber,
        params: &[u8],
    ) -> Option<u64>;
}

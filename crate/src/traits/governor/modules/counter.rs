use openbrush::traits::{BlockNumber, Hash};

#[openbrush::wrapper]
pub type CounterRef = dyn Counter;

#[openbrush::trait_definition]
pub trait Counter {
    /// Minimum number of cast voted required for a proposal to be successful.
    ///
    /// Note: The blockNumber parameter corresponds to the snapshot used for counting vote.
    /// This allows to scale the quorum depending on values such as the totalSupply of a token at this block.
    #[ink(message)]
    fn quorum(&self, block_number: BlockNumber) -> u64;
}

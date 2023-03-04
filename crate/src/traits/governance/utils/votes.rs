use openbrush::traits::{
    AccountId,
    BlockNumber,
};

#[openbrush::wrapper]
pub type VotesRef = dyn Votes;

#[openbrush::trait_definition]
pub trait Votes {
    /// Returns the current amount of votes that `account` has.
    #[ink(message)]
    fn get_votes(&self, account: AccountId) -> u64;

    /// Returns the amount of votes that `account` had at the end of a past block (`blockNumber`).
    #[ink(message)]
    fn get_past_votes(&self, account: AccountId, block_number: BlockNumber) -> u64;

    /// Returns the total supply of votes available at the end of a past block (`blockNumber`).
    ///
    /// Note: This value is the sum of all available votes, which is not necessarily the sum of all delegated votes.
    /// Votes that have not been delegated are still part of total supply, even though they would
    /// not participate in a vote.
    #[ink(message)]
    fn get_past_total_supply(&self, block_number: BlockNumber) -> u64;

    /// Returns the delegate that `account` has chosen.
    #[ink(message)]
    fn delegates(&self, account: AccountId) -> AccountId;

    /// Delegates votes from the sender to `delegatee`.
    #[ink(message)]
    fn delegate(&mut self, delegatee: AccountId);
}

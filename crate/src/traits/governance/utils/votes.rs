use openbrush::traits::{
    AccountId,
    Balance,
    BlockNumber,
};

use crate::traits::errors::VotesError;

pub type Vote = u64;

#[openbrush::wrapper]
pub type VotesRef = dyn Votes;

#[openbrush::trait_definition]
pub trait Votes {
    /// Returns the current amount of votes that `account` has.
    #[ink(message)]
    fn get_votes(&self, account: AccountId) -> Result<Vote, VotesError>;

    /// Returns the amount of votes that `account` had at the end of a past block (`blockNumber`).
    #[ink(message)]
    fn get_past_votes(
        &self,
        account: AccountId,
        block_number: BlockNumber,
    ) -> Result<Vote, VotesError>;

    /// Returns the total supply of votes available at the end of a past block (`blockNumber`).
    ///
    /// Note: This value is the sum of all available votes, which is not necessarily the sum of all delegated votes.
    /// Votes that have not been delegated are still part of total supply, even though they would
    /// not participate in a vote.
    #[ink(message)]
    fn get_past_total_supply(
        &self,
        block_number: BlockNumber,
    ) -> Result<Vote, VotesError>;

    /// Returns the delegate that `account` has chosen.
    #[ink(message)]
    fn delegates(&self, account: AccountId) -> Result<AccountId, VotesError>;

    /// Delegates votes from the sender to `delegatee`.
    #[ink(message)]
    fn delegate(&mut self, delegatee: AccountId) -> Result<(), VotesError>;
}

pub fn balance_to_vote(input: Balance) -> Option<u64> {
    TryInto::<u64>::try_into(input).ok()
}

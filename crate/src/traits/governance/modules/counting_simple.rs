use openbrush::traits::AccountId;

use crate::traits::{
    errors::{
        CountingError,
        CountingSimpleError,
    },
    governance::ProposalId,
};

/// The choices available to vote on a proposal
#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[repr(u8)]
pub enum VoteType {
    Against,
    For,
    Abstain,
}

impl TryFrom<u8> for VoteType {
    type Error = CountingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(VoteType::Against),
            2 => Ok(VoteType::For),
            3 => Ok(VoteType::Abstain),
            _ => Err(CountingError::InvalidVoteType),
        }
    }
}

/// A ProposalVote is the rapresentation of the votes a proposal may have.
#[derive(Debug, Default, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ProposalVote {
    /// The votes against the proposal.
    pub against_votes: u64,
    /// The votes in favour of the proposal.
    pub for_votes: u64,
    /// The abstain votes.
    pub abstain_votes: u64,
}

#[openbrush::wrapper]
pub type CountingSimpleRef = dyn CountingSimple;

/// Trait definition of counting simple module.
#[openbrush::trait_definition]
pub trait CountingSimple {
    #[ink(message)]
    /// Minimum number of cast voted required for a proposal to be successful.
    ///
    /// Note: In this module by default is 1 vote for simple group members without token involved
    fn quorum(&self) -> u64;
    /// Returns whether account has cast a vote on proposalId.
    #[ink(message)]
    fn has_voted(&self, proposal_id: ProposalId, account: AccountId) -> bool;

    /// Returns the votes that a proposal has already received
    #[ink(message)]
    fn proposal_votes(
        &self,
        proposal_id: ProposalId,
    ) -> Result<ProposalVote, CountingSimpleError>;
}

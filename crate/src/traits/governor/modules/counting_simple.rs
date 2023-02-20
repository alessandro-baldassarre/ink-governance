use openbrush::traits::AccountId;

use crate::traits::{errors::CountingSimpleError, governor::ProposalId};

/// The choices available to vote on a proposal
#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
#[repr(u8)]
pub enum VoteType {
    Against = 1,
    For = 2,
    Abstain = 3,
}

/// A ProposalVote is the rapresentation of the votes a proposal may have.
#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
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

#[openbrush::trait_definition]
pub trait CountingSimple {
    /// Returns whether account has cast a vote on proposalId.
    #[ink(message)]
    fn has_voted(
        &self,
        proposal_id: ProposalId,
        account: AccountId,
    ) -> Result<bool, CountingSimpleError>;

    /// Accessor to the internal vote counts.
    #[ink(message)]
    fn proposal_votes(&self, proposal_id: ProposalId) -> Result<ProposalVote, CountingSimpleError>;
}

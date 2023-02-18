use openbrush::traits::String;

/// The Governor error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GovernorError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if the proposal was not found.
    ProposalNotFound,
    /// Returned if the proposal submitted has invalid parameters
    InvalidProposalLength,
    /// Returned if was submitted an empty proposal
    EmptyProposal,
    /// Returned if the submitted proposal already exist
    ProposalAlreadyExist,
    /// Returned if the proposer voting power is below the proposal threshold
    BelowThreshold,
    /// Returned if the proposal is not successful
    ProposalNotSuccessful,
    /// Returned if the call is reverted without message
    CallRevertedWithoutMessage,
    /// Returned if the proposal is not active
    ProposalNotActive,
}

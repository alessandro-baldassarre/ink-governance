use ink::LangError;
use openbrush::traits::String;

/// Governor module error type.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum GovernorError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Error from Lang
    Lang(LangError),
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
    /// Returned if the function was not passed through governance proposal
    OnlyGovernance,
    /// Returned if the votes for that account was not found.
    NoVotes,
}

impl From<LangError> for GovernorError {
    fn from(_lang: LangError) -> Self {
        GovernorError::Custom(String::from("LE lang err"))
    }
}

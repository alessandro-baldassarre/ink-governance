use openbrush::{
    contracts::{
        access_control::AccessControlError,
        traits::{errors::ReentrancyGuardError, pausable::PausableError, proxy::OwnableError},
    },
    traits::String,
};

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
    /// Returned if the function was not passed through governance proposal
    OnlyGovernance,
    /// Returned if the votes for that account was not found.
    NoVotes,
}

impl From<AccessControlError> for GovernorError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                GovernorError::Custom(String::from("AC::MissingRole"))
            }
            AccessControlError::RoleRedundant => {
                GovernorError::Custom(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                GovernorError::Custom(String::from("AC::InvalidCaller"))
            }
        }
    }
}

impl From<OwnableError> for GovernorError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                GovernorError::Custom(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::NewOwnerIsZero => {
                GovernorError::Custom(String::from("O::NewOwnerIsZero"))
            }
        }
    }
}

impl From<PausableError> for GovernorError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => GovernorError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => GovernorError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for GovernorError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => {
                GovernorError::Custom(String::from("RG::ReentrantCall"))
            }
        }
    }
}

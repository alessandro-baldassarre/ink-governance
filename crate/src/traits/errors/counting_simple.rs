use openbrush::{
    contracts::traits::{
        errors::ReentrancyGuardError,
        pausable::PausableError,
        proxy::OwnableError,
    },
    traits::String,
};

use super::GovernorError;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CountingSimpleError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Error from Governor
    GovernorError(GovernorError),
    /// Returns if no account vote was found for that proposal
    NoResult,
    /// Returns if no proposal was found
    NoProposal,
}

impl From<OwnableError> for CountingSimpleError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                CountingSimpleError::Custom(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::NewOwnerIsZero => {
                CountingSimpleError::Custom(String::from("O::NewOwnerIsZero"))
            }
        }
    }
}

impl From<PausableError> for CountingSimpleError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => {
                CountingSimpleError::Custom(String::from("P::Paused"))
            }
            PausableError::NotPaused => {
                CountingSimpleError::Custom(String::from("P::NotPaused"))
            }
        }
    }
}

impl From<ReentrancyGuardError> for CountingSimpleError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => {
                CountingSimpleError::Custom(String::from("RG::ReentrantCall"))
            }
        }
    }
}

impl From<GovernorError> for CountingSimpleError {
    fn from(_governor: GovernorError) -> Self {
        CountingSimpleError::Custom(String::from("G::Governor Error"))
    }
}

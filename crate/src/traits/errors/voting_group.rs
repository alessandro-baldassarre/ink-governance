use openbrush::{
    contracts::{
        access_control::AccessControlError,
        traits::{pausable::PausableError, proxy::OwnableError},
    },
    traits::String,
};

use super::GovernorError;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VotingGroupError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Error from AccessControl
    AccessControlError(AccessControlError),
    /// Error from Governor
    GovernorError(GovernorError),
}

impl From<AccessControlError> for GovernorError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                GovernorError::AccessControlError(AccessControlError::MissingRole)
            }
            AccessControlError::RoleRedundant => {
                GovernorError::AccessControlError(AccessControlError::RoleRedundant)
            }
            AccessControlError::InvalidCaller => {
                GovernorError::AccessControlError(AccessControlError::InvalidCaller)
            }
        }
    }
}

impl From<OwnableError> for VotingGroupError {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                VotingGroupError::Custom(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::NewOwnerIsZero => {
                VotingGroupError::Custom(String::from("O::NewOwnerIsZero"))
            }
        }
    }
}

impl From<PausableError> for VotingGroupError {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => VotingGroupError::Custom(String::from("P::Paused")),
            PausableError::NotPaused => VotingGroupError::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for VotingGroupError {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => {
                VotingGroupError::Custom(String::from("RG::ReentrantCall"))
            }
        }
    }
}

impl From<GovernorError> for VotingGroupError {
    fn from(governor: GovernorError) -> Self {
        VotingGroupError::Custom(String::from("G::Governor Error"))
    }
}

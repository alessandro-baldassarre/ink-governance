use openbrush::{
    contracts::{
        access_control::AccessControlError,
        traits::{
            errors::ReentrancyGuardError,
            pausable::PausableError,
            proxy::OwnableError,
        },
    },
    traits::{
        AccountId,
        String,
    },
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
    /// Entered duplicate member
    DuplicatedMember {
        member: AccountId,
    },
    // No members entered
    ZeroMembers,
    /// Member not found
    NoMember,
}

impl From<AccessControlError> for VotingGroupError {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => {
                VotingGroupError::AccessControlError(AccessControlError::MissingRole)
            }
            AccessControlError::RoleRedundant => {
                VotingGroupError::AccessControlError(AccessControlError::RoleRedundant)
            }
            AccessControlError::InvalidCaller => {
                VotingGroupError::AccessControlError(AccessControlError::InvalidCaller)
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
            PausableError::NotPaused => {
                VotingGroupError::Custom(String::from("P::NotPaused"))
            }
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
    fn from(_governor: GovernorError) -> Self {
        VotingGroupError::Custom(String::from("G::Governor Error"))
    }
}

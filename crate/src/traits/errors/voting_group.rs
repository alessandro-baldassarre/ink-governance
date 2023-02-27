use openbrush::{
    contracts::traits::errors::ReentrancyGuardError,
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
    /// Returned if the function was not passed through governance proposal or the caller is not
    /// the Admin of the group
    OnlyAdminOrGovernance,
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
        match governor {
            GovernorError::OnlyGovernance => {
                VotingGroupError::GovernorError(GovernorError::OnlyGovernance)
            }
            _ => VotingGroupError::Custom(String::from("Error from Governor")),
        }
    }
}

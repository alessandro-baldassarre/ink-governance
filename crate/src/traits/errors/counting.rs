use openbrush::traits::String;

use super::GovernorError;

/// Counting module error type.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CountingError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Error from Governor
    GovernorError(GovernorError),
    /// Returns when an account already cast a vote of that proposal
    VoteAlreadyCast,
    /// Returns when cast an invalid vote type
    InvalidVoteType,
}

impl From<GovernorError> for CountingError {
    fn from(_governor: GovernorError) -> Self {
        CountingError::Custom(String::from("G::Governor Error"))
    }
}

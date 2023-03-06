use openbrush::traits::String;

use super::VotesError;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22VotesError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Errors from Votes
    VotesError(VotesError),
    /// Returns when a convertion fail
    ConvertionError { from: String, to: String },
}

impl From<VotesError> for PSP22VotesError {
    fn from(votes: VotesError) -> Self {
        match votes {
            VotesError::ZeroCheckpoints => {
                PSP22VotesError::Custom(String::from("VE::ZeroCheckpoints"))
            }
            VotesError::NotMinedBlock => {
                PSP22VotesError::Custom(String::from("VE:NotMinedBlock"))
            }
            VotesError::ZeroDelegatesAccount => {
                PSP22VotesError::Custom(String::from("VE::ZeroDelegatesAccount"))
            }
            VotesError::NoCheckpoint => {
                PSP22VotesError::Custom(String::from("VE::NoCheckpoint"))
            }
            _ => PSP22VotesError::Custom(String::from("VE")),
        }
    }
}

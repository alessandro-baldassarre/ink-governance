use openbrush::{
    contracts::psp22::PSP22Error,
    traits::String,
};

use super::VotesError;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum PSP22VotesError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Errors from Votes
    VotesError(VotesError),
    /// Errors from PSP22
    PSP22(PSP22Error),
    /// Returns when a convertion fail
    ConvertionError { from: String, to: String },
}

impl From<VotesError> for PSP22VotesError {
    fn from(votes: VotesError) -> Self {
        match votes {
            VotesError::ZeroCheckpoints => {
                PSP22VotesError::Custom(String::from("Votes::ZeroCheckpoints"))
            }
            VotesError::NotMinedBlock => {
                PSP22VotesError::Custom(String::from("Votes:NotMinedBlock"))
            }
            VotesError::ZeroDelegatesAccount => {
                PSP22VotesError::Custom(String::from("Votes::ZeroDelegatesAccount"))
            }
            VotesError::NoCheckpoint => {
                PSP22VotesError::Custom(String::from("Votes::NoCheckpoint"))
            }
            VotesError::MovePowerAmountError => {
                PSP22VotesError::Custom(String::from("Votes::MovePowerAmountError"))
            }
            VotesError::MovePowerAccountsError => {
                PSP22VotesError::Custom(String::from("Votes::MovePowerAccountsError"))
            }
            VotesError::BalanceToVoteErr => {
                PSP22VotesError::Custom(String::from("Votes::BalanceToVoteErr"))
            }
            VotesError::Custom(string) => PSP22VotesError::Custom(string),
        }
    }
}

impl From<PSP22Error> for PSP22VotesError {
    fn from(_value: PSP22Error) -> Self {
        PSP22VotesError::Custom(String::from("Error from PSP22"))
    }
}

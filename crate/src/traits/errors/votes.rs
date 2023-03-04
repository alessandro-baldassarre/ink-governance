use openbrush::traits::String;

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VotesError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returns when a block is not yet mined.
    NotMinedBlock,
}

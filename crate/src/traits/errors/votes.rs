use openbrush::traits::String;

/// Votes module error type.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum VotesError {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returns when a block is not yet mined.
    NotMinedBlock,
    /// Returns when no checkpoints was found for that account
    ZeroCheckpoints,
    /// Returns when no checkpoint was found for that block time
    NoCheckpoint,
    /// Returns when the source and destination address on move voting power are equal
    MovePowerAccountsError,
    /// Returns when the amount on move voting power are less than 1.
    MovePowerAmountError,
    /// Reuturns when a conversion from Balance to Vote failed
    BalanceToVoteErr,
}

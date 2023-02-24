use openbrush::traits::BlockNumber;

use crate::traits::errors::GovernorError;

/// Extension of Governor for settings updatable through governance.
#[openbrush::wrapper]
pub type GovernorSettingsRef = dyn GovernorSettings;

#[openbrush::trait_definition]
pub trait GovernorSettings {
    /// Update the voting delay. This operation can only be performed through a governance proposal
    ///
    /// Emits a VotingDelaySet event.
    #[ink(message)]
    fn set_voting_delay(
        &mut self,
        new_voting_delay: BlockNumber,
    ) -> Result<(), GovernorError>;

    /// Update the voting period. This operation can only be performed through a governance proposal.
    ///
    /// Emits a VotingPeriodSet event.
    #[ink(message)]
    fn set_voting_period(
        &mut self,
        new_voting_period: BlockNumber,
    ) -> Result<(), GovernorError>;

    /// Update the proposal threshold. This operation can only be performed through a governance proposal.
    ///
    /// Emits a ProposalThresholdSet event.
    #[ink(message)]
    fn set_proposal_threshold(
        &mut self,
        new_proposal_threshold: u64,
    ) -> Result<(), GovernorError>;
}

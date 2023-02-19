use openbrush::traits::{AccountId, ZERO_ADDRESS};

use ink::prelude::vec::Vec;

use crate::traits::errors::VotingGroupError;

/// A Proposal is what can be proposed
#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct VotingMember {
    /// The `AccountId` of the member.
    pub member: AccountId,
    /// The weight of one vote of this member.
    pub voting_power: u64,
}

impl Default for VotingMember {
    fn default() -> Self {
        Self {
            member: ZERO_ADDRESS.into(),
            voting_power: Default::default(),
        }
    }
}

#[openbrush::wrapper]
pub type VotingGroupRef = dyn VotingGroup;

#[openbrush::trait_definition]
pub trait VotingGroup {
    /// Update one or more existing voter members
    /// Add one or more new voter members
    /// Remove one or more existing voter members
    ///
    /// Note: The actions are performed in sequence (Update->Add->Remove) so if you enter an account more than once keep in mind the sequence.
    #[ink(message)]
    fn update_members(&mut self, members: Vec<VotingMember>) -> Result<(), VotingGroupError>;

    /// Returns the info of one or more voter members
    #[ink(message)]
    fn get_members(&self, members: Vec<AccountId>) -> Result<Vec<VotingMember>, VotingGroupError>;
}

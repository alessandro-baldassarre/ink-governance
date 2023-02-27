use openbrush::traits::{
    AccountId,
    ZERO_ADDRESS,
};

use ink::prelude::vec::Vec;

use crate::traits::errors::VotingGroupError;

/// A Proposal is what can be proposed
#[derive(Debug, Copy, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct VotingMember {
    /// The `AccountId` of the member.
    pub account: AccountId,
    /// The weight of one vote of this member.
    pub voting_power: u64,
}

impl Default for VotingMember {
    fn default() -> Self {
        Self {
            account: ZERO_ADDRESS.into(),
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
    /// Note: The actions are performed in sequence (Update->Add->Remove) so if if you enter an account more than once, keep this sequence in mind.
    #[ink(message)]
    fn update_members(
        &mut self,
        members: Vec<VotingMember>,
        members_to_remove: Vec<AccountId>,
    ) -> Result<(), VotingGroupError>;

    /// Returns the info of one or more voter members
    #[ink(message)]
    fn get_members(
        &self,
        members: Vec<AccountId>,
    ) -> Result<Vec<VotingMember>, VotingGroupError>;

    fn _init_members(
        &mut self,
        admin: AccountId,
        init_members: Vec<VotingMember>,
    ) -> Result<(), VotingGroupError>;
}

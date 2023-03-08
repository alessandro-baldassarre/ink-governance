use openbrush::traits::{
    AccountId,
    String,
};

use crate::traits::{
    errors::CountingError,
    governance::ProposalId,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Counting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Counting {
    pub _reserved: Option<()>,
}

pub trait Counter {
    /// Amount of votes already cast passes the threshold limit.
    fn _quorum_reached(&self, proposal_id: &ProposalId) -> Result<bool, CountingError>;

    /// Is the proposal successful or not.
    fn _vote_succeeded(&self, proposal_id: &ProposalId) -> Result<bool, CountingError>;

    /// Register a vote for proposalId by account with a given support, voting weight and voting params.
    ///
    /// Note: Support is generic and can represent various things depending on the voting system used.
    fn _count_vote(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        weight: u64,
        params: &[u8],
    ) -> Result<(), CountingError>;
}

impl Counter for Counting {
    default fn _quorum_reached(
        &self,
        _proposal_id: &ProposalId,
    ) -> Result<bool, CountingError> {
        Err(CountingError::Custom(String::from("No module")))
    }
    default fn _vote_succeeded(
        &self,
        _proposal_id: &ProposalId,
    ) -> Result<bool, CountingError> {
        Err(CountingError::Custom(String::from("No module")))
    }
    default fn _count_vote(
        &mut self,
        _proposal_id: &ProposalId,
        _account: &AccountId,
        _support: u8,
        _weight: u64,
        _params: &[u8],
    ) -> Result<(), CountingError> {
        Err(CountingError::Custom(String::from("No module")))
    }
}

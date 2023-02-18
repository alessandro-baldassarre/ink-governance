use ink::prelude::vec::Vec;
use openbrush::traits::AccountId;

pub use crate::traits::governor::*;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Counting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Counting {
    pub _reserved: Option<()>,
}

pub trait Counter {
    /// Amount of votes already cast passes the threshold limit.
    fn _quorum_reached(&self, proposal_id: &ProposalId) -> bool;

    /// Is the proposal successful or not.
    fn _vote_succeeded(&self, proposal_id: &ProposalId) -> bool;

    /// Register a vote for proposalId by account with a given support, voting weight and voting params.
    ///
    /// Note: Support is generic and can represent various things depending on the voting system used.
    fn _count_vote(
        &self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: &u8,
        weight: &u64,
        params: &Vec<u8>,
    );
}

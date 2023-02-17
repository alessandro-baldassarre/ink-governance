use crate::governor::counter::Internal;
use crate::governor::voter::Voting;
pub use crate::{
    governor::{counter, governor, voter},
    traits::governor::modules::counter::*,
};
use openbrush::traits::{AccountId, BlockNumber, OccupiedStorage, Storage};

use ink::prelude::vec::Vec;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Counting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Counting {
    pub _reserved: Option<()>,
}

impl<T> Counter for T
where
    T: Storage<governor::Data<Counting, Voting>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<Counting, Voting>>,
{
    default fn quorum(&self, _block_number: BlockNumber) -> u64 {
        0
    }
}

impl<T> Internal for T
where
    T: Storage<governor::Data<Counting, Voting>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<Counting, Voting>>,
{
    default fn _quorum_reached(&self, proposal_id: &ProposalId) -> bool {
        false
    }

    default fn _vote_succeeded(&self, proposal_id: &ProposalId) -> bool {
        false
    }

    default fn _count_vote(
        &self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: &u8,
        weight: &u64,
        params: &Vec<u8>,
    ) {
    }
}

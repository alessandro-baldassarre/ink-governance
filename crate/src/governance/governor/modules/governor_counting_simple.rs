use crate::governor::voter::Voting;
pub use crate::{
    governor::{counter, governor, voter},
    traits::governor::*,
};
use openbrush::traits::{AccountId, BlockNumber, OccupiedStorage, Storage};

use ink::{
    prelude::vec::Vec,
    storage::traits::{AutoStorableHint, ManualKey, Storable, StorableHint},
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Counting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Counting {
    pub _reserved: Option<()>,
}

impl counter::Counter for Counting {
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

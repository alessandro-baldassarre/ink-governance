pub use crate::{
    governance::governor, governance::governor::counter, governance::governor::counter::*,
    governance::governor::modules::governor_counting_simple,
};
use openbrush::traits::AccountId;

use ink::prelude::vec::Vec;

use self::governor::ProposalId;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Counting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Counting {
    pub _reserved: Option<()>,
}

impl counter::Counter for Counting {
    default fn _quorum_reached(&self, _proposal_id: &ProposalId) -> bool {
        false
    }

    default fn _vote_succeeded(&self, _proposal_id: &ProposalId) -> bool {
        false
    }

    default fn _count_vote(
        &self,
        _proposal_id: &ProposalId,
        _account: &AccountId,
        _support: &u8,
        _weight: &u64,
        _params: &Vec<u8>,
    ) {
    }
}

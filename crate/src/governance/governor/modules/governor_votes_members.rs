pub use crate::{
    governance::governor, governance::governor::modules::governor_votes_members,
    governance::governor::voter, governance::governor::voter::*,
};

use crate::governor::GovernorError;
use openbrush::{
    storage::Mapping,
    traits::{AccountId, BlockNumber},
};

use ink::prelude::vec::Vec;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Voting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Voting {
    pub voting_power: Mapping<(AccountId, BlockNumber), u64>,
    pub _reserved: Option<()>,
}

pub trait VotingGroup {
    fn set_voting_power(
        &mut self,
        account: AccountId,
        voting_power: Option<u64>,
    ) -> Result<(), GovernorError>;
}

impl voter::Voter for Voting {
    default fn _get_votes(
        &mut self,
        account: AccountId,
        block_number: BlockNumber,
        _params: Vec<u8>,
    ) -> Result<u64, GovernorError> {
        let votes = self.voting_power.get(&(account, block_number)).unwrap();
        Ok(votes)
    }

    default fn _set_voting_power(
        &mut self,
        account: AccountId,
        block_number: BlockNumber,
        voting_power: u64,
    ) {
        self.voting_power
            .insert(&(account, block_number), &voting_power);
    }
}

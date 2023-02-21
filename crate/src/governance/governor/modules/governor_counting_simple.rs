pub use crate::governor_counting_simple;
pub use crate::governor_counting_simple::Internal as _;
pub use crate::traits::governor::modules::counting_simple::*;

use crate::{
    governance::governor,
    governance::governor::counter,
    governor::{voter, Data},
    traits::errors::{CountingError, CountingSimpleError},
};
use openbrush::{
    storage::Mapping,
    traits::{AccountId, OccupiedStorage, Storage, String},
};

use ink::{
    prelude::vec::Vec,
    storage::traits::{AutoStorableHint, ManualKey, Storable, StorableHint},
};

use self::governor::ProposalId;

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Counting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Counting {
    pub proposal_votes: Mapping<ProposalId, ProposalVote>,
    pub has_voted: Mapping<(AccountId, ProposalId), bool>,
    pub _reserved: Option<()>,
}

impl counter::Counter for Counting {
    default fn _quorum_reached(&self, proposal_id: &ProposalId) -> Result<bool, CountingError> {
        let proposal_votes = self
            .proposal_votes
            .get(proposal_id)
            .ok_or(CountingError::Custom(String::from("Proposal not found")))?;
        Ok(self._quorum() <= (proposal_votes.for_votes + proposal_votes.abstain_votes))
    }

    default fn _vote_succeeded(&self, proposal_id: &ProposalId) -> Result<bool, CountingError> {
        let proposal_votes = self
            .proposal_votes
            .get(proposal_id)
            .ok_or(CountingError::Custom(String::from("Proposal not found")))?;
        Ok(proposal_votes.for_votes > proposal_votes.against_votes)
    }

    default fn _count_vote(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        weight: u64,
        _params: &Vec<u8>,
    ) -> Result<(), CountingError> {
        let mut proposal_votes = self
            .proposal_votes
            .get(proposal_id)
            .ok_or(CountingError::Custom(String::from("Proposal not found")))?;
        let has_voted = self
            .has_voted
            .get(&(*account, *proposal_id))
            .ok_or(CountingError::Custom(String::from("No account registered")))?;
        if has_voted {
            return Err(CountingError::VoteAlreadyCast);
        }
        self.has_voted.insert(&(*account, *proposal_id), &true);

        let updated_votes: ProposalVote = match support.try_into() {
            Ok(VoteType::For) => {
                proposal_votes.for_votes += weight;
                proposal_votes
            }
            Ok(VoteType::Against) => {
                proposal_votes.against_votes += weight;
                proposal_votes
            }
            Ok(VoteType::Abstain) => {
                proposal_votes.abstain_votes += weight;
                proposal_votes
            }
            Err(err) => return Err(err),
        };

        self.proposal_votes.insert(proposal_id, &updated_votes);

        Ok(())
    }
}

impl<T, C, V> CountingSimple for T
where
    C: counter::Counter + Internal,
    C: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<719029772, ManualKey<{ governor::STORAGE_KEY }>>, Type = C>,
    V: voter::Voter,
    V: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3230629697, ManualKey<{ governor::STORAGE_KEY }>>, Type = V>,
    T: Storage<governor::Data<C, V>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<C, V>>,
{
    default fn quorum(&self) -> u64 {
        self.data::<Data<C, V>>().counting_module._quorum()
    }

    default fn has_voted(
        &self,
        proposal_id: ProposalId,
        account: AccountId,
    ) -> Result<bool, CountingSimpleError> {
        let result = self
            .data::<Data<C, V>>()
            .counting_module
            ._has_voted(account, proposal_id)?;

        Ok(result)
    }

    default fn proposal_votes(
        &self,
        proposal_id: ProposalId,
    ) -> Result<ProposalVote, CountingSimpleError> {
        let proposal_vote = self
            .data::<Data<C, V>>()
            .counting_module
            ._proposal_votes(&proposal_id)?;
        Ok(proposal_vote)
    }
}

pub trait Internal {
    fn _quorum(&self) -> u64;

    fn _has_voted(
        &self,
        account: AccountId,
        proposal_id: ProposalId,
    ) -> Result<bool, CountingSimpleError>;

    fn _proposal_votes(
        &self,
        proposal_id: &ProposalId,
    ) -> Result<ProposalVote, CountingSimpleError>;
}

impl Internal for Counting {
    fn _quorum(&self) -> u64 {
        1
    }

    fn _has_voted(
        &self,
        account: AccountId,
        proposal_id: ProposalId,
    ) -> Result<bool, CountingSimpleError> {
        if let Some(vote) = self.has_voted.get(&(account, proposal_id)) {
            return Ok(vote);
        } else {
            return Err(CountingSimpleError::NoResult);
        }
    }

    fn _proposal_votes(
        &self,
        proposal_id: &ProposalId,
    ) -> Result<ProposalVote, CountingSimpleError> {
        if let Some(proposal_vote) = self.proposal_votes.get(proposal_id) {
            return Ok(proposal_vote);
        } else {
            return Err(CountingSimpleError::NoProposal);
        }
    }
}

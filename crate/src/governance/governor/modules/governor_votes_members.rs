use crate::governance::governor::voter;

use crate::governor::*;

use crate::{governor::counter, governor::GovernorError};
use openbrush::contracts::access_control::DEFAULT_ADMIN_ROLE;
use openbrush::modifiers;
use openbrush::{
    contracts::access_control::access_control,
    storage::Mapping,
    traits::{AccountId, BlockNumber, OccupiedStorage, Storage},
};

use ink::{
    prelude::vec::Vec,
    storage::traits::{AutoStorableHint, ManualKey, Storable, StorableHint},
};

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
    ) -> Result<(), GovernorError> {
        self.voting_power
            .insert(&(account, block_number), &voting_power);
        Ok(())
    }
}

impl<T, C, V, M> VotingGroup for T
where
    M: access_control::members::MembersManager,
    M: Storable
        + StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<3218979580, ManualKey<{ access_control::STORAGE_KEY }>>,
            Type = M,
        >,
    C: counter::Counter,
    C: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<719029772, ManualKey<{ governor::STORAGE_KEY }>>, Type = C>,
    V: voter::Voter,
    V: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3230629697, ManualKey<{ governor::STORAGE_KEY }>>, Type = V>,
    T: Storage<access_control::Data<M>> + Storage<governor::Data<C, V>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>
        + OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<C, V>>,
{
    #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))]
    default fn set_voting_power(
        &mut self,
        account: AccountId,
        voting_power: Option<u64>,
    ) -> Result<(), GovernorError> {
        let voting_power = voting_power.unwrap_or(1);
        self.data::<Data<C, V>>().voting_module._set_voting_power(
            account,
            Self::env().block_number(),
            voting_power,
        )?;
        Ok(())
    }
}

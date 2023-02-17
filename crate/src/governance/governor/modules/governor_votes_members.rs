pub use crate::governor::{governor, voter::Internal};
use openbrush::{
    contracts::access_control::{self, RoleType},
    modifiers,
    storage::Mapping,
    traits::{AccountId, BlockNumber, OccupiedStorage, Storage},
};

use ink::{
    prelude::vec::Vec,
    storage::traits::{AutoStorableHint, ManualKey, Storable, StorableHint},
};

use self::governor::{
    counter::{self, Counting},
    voter, Data, GovernorError,
};

const MEMBERS: RoleType = ink::selector_id!("MEMBERS");
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
        voting_power: u64,
    ) -> Result<(), GovernorError>;
}

impl<T, M, C, V> VotingGroup for T
where
    C: counter::Internal,
    C: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<719029772, ManualKey<{ governor::STORAGE_KEY }>>, Type = C>,
    V: voter::Internal,
    V: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3230629697, ManualKey<{ governor::STORAGE_KEY }>>, Type = V>,
    M: access_control::members::MembersManager,
    M: Storable
        + StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<3218979580, ManualKey<{ access_control::STORAGE_KEY }>>,
            Type = M,
        >,
    T: Storage<governor::Data<C, V>> + Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<C, V>>
        + OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
{
    #[modifiers(access_control::only_role(MEMBERS))]
    default fn set_voting_power(
        &mut self,
        account: AccountId,
        voting_power: u64,
    ) -> Result<(), GovernorError> {
        let votes = self
            .data::<Data<C, V>>()
            .voting_module
            ._get_votes(&account, &1, &vec![]);
        Ok(())
    }
}

impl<T> Internal for T
where
    T: Storage<governor::Data<Counting, Voting>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<Counting, Voting>>,
{
    default fn _get_votes(
        &self,
        account: &AccountId,
        block_number: &BlockNumber,
        params: &Vec<u8>,
    ) -> u64 {
        0
    }
}

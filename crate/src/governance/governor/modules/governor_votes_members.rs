pub use crate::governor::{governor, voter::Internal};
use openbrush::{
    contracts::access_control::{self, RoleType, DEFAULT_ADMIN_ROLE},
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

const MEMBER: RoleType = ink::selector_id!("MEMBER");
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
    #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))]
    default fn set_voting_power(
        &mut self,
        account: AccountId,
        voting_power: Option<u64>,
    ) -> Result<(), GovernorError> {
        self.data::<Data<C, V>>().voting_module._set_voting_power(
            account,
            Self::env().block_number(),
            voting_power.unwrap(),
        );
        Ok(())
    }
}

impl<T, M> Internal for T
where
    M: access_control::members::MembersManager,
    M: Storable
        + StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<3218979580, ManualKey<{ access_control::STORAGE_KEY }>>,
            Type = M,
        >,
    T: Storage<governor::Data<Counting, Voting>> + Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<Counting, Voting>>
        + OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
{
    #[modifiers(access_control::only_role(MEMBER))]
    default fn _get_votes(
        &mut self,
        account: AccountId,
        block_number: BlockNumber,
        _params: Vec<u8>,
    ) -> Result<u64, GovernorError> {
        let votes = self
            .data::<Data<Counting, Voting>>()
            .voting_module
            .voting_power
            .get(&(account, block_number))
            .unwrap();
        Ok(votes)
    }

    default fn _set_voting_power(
        &mut self,
        account: AccountId,
        block_number: BlockNumber,
        voting_power: u64,
    ) {
        self.data::<Data<Counting, Voting>>()
            .voting_module
            .voting_power
            .insert(&(account, block_number), &voting_power);
    }
}

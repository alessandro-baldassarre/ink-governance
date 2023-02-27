pub use crate::{
    governance::modules::{
        governor_voting_group,
        governor_voting_group::Internal as _,
    },
    traits::{
        errors::VotingGroupError,
        governance::modules::voting_group::*,
    },
};

use crate::governance::{
    governor::*,
    modules::{
        counter,
        voter,
    },
};

use openbrush::{
    modifier_definition,
    modifiers,
    storage::Mapping,
    traits::{
        AccountId,
        BlockNumber,
        OccupiedStorage,
        Storage,
        ZERO_ADDRESS,
    },
};

use ink::{
    prelude::vec::Vec,
    storage::traits::{
        AutoStorableHint,
        ManualKey,
        Storable,
        StorableHint,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Voting);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Voting {
    pub members: Mapping<AccountId, u64>,
    pub admin: AccountId,
    pub _reserved: Option<()>,
}

impl Default for Voting {
    fn default() -> Self {
        Voting {
            members: Default::default(),
            admin: ZERO_ADDRESS.into(),
            _reserved: Default::default(),
        }
    }
}

impl voter::Voter for Voting {
    default fn _get_votes(
        &self,
        account: &AccountId,
        _block_number: BlockNumber,
        _params: &[u8],
    ) -> Option<u64> {
        self.members.get(account)
    }
}

#[modifier_definition]
pub fn only_governance_or_admin<T, C, V, F, R, E>(
    instance: &mut T,
    body: F,
) -> Result<R, E>
where
    C: counter::Counter,
    C: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<719029772, ManualKey<{ governor::STORAGE_KEY }>>,
            Type = C,
        >,
    V: voter::Voter + Internal,
    V: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<3230629697, ManualKey<{ governor::STORAGE_KEY }>>,
            Type = V,
        >,
    T: Storage<Data<C, V>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = Data<C, V>>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<VotingGroupError>,
{
    if T::env().caller() != instance.data()._executor()
        && !instance.data().voting_module._is_admin(T::env().caller())
    {
        return Err(VotingGroupError::OnlyAdminOrGovernance.into())
    }

    body(instance)
}

impl<T, C, V> VotingGroup for T
where
    C: counter::Counter,
    C: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<719029772, ManualKey<{ governor::STORAGE_KEY }>>,
            Type = C,
        >,
    V: voter::Voter + Internal,
    V: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<3230629697, ManualKey<{ governor::STORAGE_KEY }>>,
            Type = V,
        >,
    T: Storage<governor::Data<C, V>>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<C, V>>,
{
    #[modifiers(only_governance_or_admin())]
    default fn update_members(
        &mut self,
        members: Vec<VotingMember>,
        members_to_remove: Vec<AccountId>,
    ) -> Result<(), VotingGroupError> {
        if members.is_empty() && members_to_remove.is_empty() {
            return Err(VotingGroupError::ZeroMembers)
        }

        if !members.is_empty() {
            validate_unique_members(&members)?;

            for member in members {
                self.data::<Data<C, V>>().voting_module._add_member(&member);
            }
        }

        if !members_to_remove.is_empty() {
            for member in members_to_remove {
                self.data::<Data<C, V>>()
                    .voting_module
                    ._remove_member(&member)?
            }
        }

        Ok(())
    }

    default fn get_members(
        &self,
        members: Vec<AccountId>,
    ) -> Result<Vec<VotingMember>, VotingGroupError> {
        let members_result: Result<Vec<VotingMember>, VotingGroupError> = members
            .into_iter()
            .map(|member| -> Result<VotingMember, VotingGroupError> {
                let voting_power = self
                    .data::<Data<C, V>>()
                    .voting_module
                    ._get_member(&member)?;
                Ok(VotingMember {
                    account: member,
                    voting_power,
                })
            })
            .collect();
        let members = members_result?;
        Ok(members)
    }

    default fn _init_members(
        &mut self,
        admin: AccountId,
        init_members: Vec<VotingMember>,
    ) -> Result<(), VotingGroupError> {
        if init_members.is_empty() {
            return Err(VotingGroupError::ZeroMembers)
        }

        validate_unique_members(&init_members)?;
        self.data::<Data<C, V>>()
            .voting_module
            ._init_members(admin, &init_members);

        Ok(())
    }
}
pub trait Internal {
    fn _init_members(&mut self, admin: AccountId, init_members: &[VotingMember]);

    fn _add_member(&mut self, member: &VotingMember);

    fn _remove_member(&mut self, member: &AccountId) -> Result<(), VotingGroupError>;

    fn _get_member(&self, account: &AccountId) -> Result<u64, VotingGroupError>;

    fn _is_admin(&self, account: AccountId) -> bool;
}

impl Internal for Voting {
    fn _init_members(&mut self, admin: AccountId, init_members: &[VotingMember]) {
        self.admin = admin;
        for member in init_members {
            self._add_member(member)
        }
    }

    fn _add_member(&mut self, member: &VotingMember) {
        self.members.insert(&member.account, &member.voting_power);
    }

    fn _remove_member(&mut self, member: &AccountId) -> Result<(), VotingGroupError> {
        self._get_member(member)?;
        self.members.remove(member);
        Ok(())
    }

    fn _get_member(&self, account: &AccountId) -> Result<u64, VotingGroupError> {
        let voting_power = self
            .members
            .get(account)
            .ok_or(VotingGroupError::NoMember)?;
        Ok(voting_power)
    }

    fn _is_admin(&self, account: AccountId) -> bool {
        self.admin == account
    }
}

/// Verifies all member accounts are unique.
pub fn validate_unique_members(members: &[VotingMember]) -> Result<(), VotingGroupError> {
    for (a, b) in members.iter().zip(members.iter().skip(1)) {
        if a.account == b.account {
            return Err(VotingGroupError::DuplicatedMember { member: a.account })
        }
    }
    Ok(())
}

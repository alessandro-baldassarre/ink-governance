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
    counter,
    governor::*,
    voter,
};

use openbrush::{
    contracts::access_control::{
        access_control,
        DEFAULT_ADMIN_ROLE,
    },
    modifiers,
    storage::Mapping,
    traits::{
        AccountId,
        BlockNumber,
        OccupiedStorage,
        Storage,
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

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Voting {
    pub members: Mapping<AccountId, u64>,
    pub _reserved: Option<()>,
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
    T: Storage<access_control::Data<M>> + Storage<governor::Data<C, V>>,
    T: OccupiedStorage<
            { access_control::STORAGE_KEY },
            WithData = access_control::Data<M>,
        > + OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<C, V>>,
{
    #[modifiers(access_control::only_role(DEFAULT_ADMIN_ROLE))]
    default fn update_members(
        &mut self,
        members: Vec<VotingMember>,
        members_to_remove: Vec<AccountId>,
    ) -> Result<(), VotingGroupError> {
        if !members.is_empty() {
            validate_unique_members(&members)?;

            for member in members {
                if self
                    .data::<Data<C, V>>()
                    .voting_module
                    ._get_member(&member.account)
                    .is_err()
                {
                    self.data::<Data<C, V>>()
                        .voting_module
                        ._add_member(&member)?;
                } else {
                    self.data::<Data<C, V>>()
                        .voting_module
                        ._update_member(&member)?;
                }
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
}
pub trait Internal {
    fn _add_member(&mut self, member: &VotingMember) -> Result<(), VotingGroupError>;

    fn _remove_member(&mut self, member: &AccountId) -> Result<(), VotingGroupError>;

    fn _update_member(&mut self, member: &VotingMember) -> Result<(), VotingGroupError>;

    fn _get_member(&self, account: &AccountId) -> Result<u64, VotingGroupError>;
}

impl Internal for Voting {
    fn _add_member(&mut self, member: &VotingMember) -> Result<(), VotingGroupError> {
        if self.members.get(&member.account).is_some() {
            return Err(VotingGroupError::DuplicatedMember {
                member: member.account,
            })
        }

        self.members.insert(&member.account, &member.voting_power);
        Ok(())
    }

    fn _remove_member(&mut self, member: &AccountId) -> Result<(), VotingGroupError> {
        self._get_member(member)?;
        self.members.remove(member);
        Ok(())
    }

    fn _update_member(&mut self, member: &VotingMember) -> Result<(), VotingGroupError> {
        self._add_member(member)?;
        Ok(())
    }

    fn _get_member(&self, account: &AccountId) -> Result<u64, VotingGroupError> {
        let voting_power = self
            .members
            .get(account)
            .ok_or(VotingGroupError::NoMember)?;
        Ok(voting_power)
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

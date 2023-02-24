pub use crate::{
    governance::extensions::{
        governor_settings,
        governor_settings::Internal as _,
    },
    traits::governance::extensions::settings::*,
};

use crate::governor;

use crate::governance::{
    counter,
    governor::*,
    voter,
};

use ink::storage::traits::{
    AutoStorableHint,
    ManualKey,
    Storable,
    StorableHint,
};
use openbrush::{
    modifiers,
    traits::{
        BlockNumber,
        OccupiedStorage,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(GovernorSetting);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub voting_delay: BlockNumber,
    pub voting_period: BlockNumber,
    pub proposal_threshold: u64,
}

impl<T, C, V> GovernorSettings for T
where
    C: counter::Counter,
    C: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<719029772, ManualKey<{ governor::STORAGE_KEY }>>,
            Type = C,
        >,
    V: voter::Voter,
    V: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<3230629697, ManualKey<{ governor::STORAGE_KEY }>>,
            Type = V,
        >,
    T: Storage<governor::Data<C, V>> + Storage<Data>,
    T: OccupiedStorage<{ governor::STORAGE_KEY }, WithData = governor::Data<C, V>>
        + OccupiedStorage<STORAGE_KEY, WithData = Data>,
{
    #[modifiers(governor::only_governance())]
    default fn set_voting_delay(
        &mut self,
        new_voting_delay: BlockNumber,
    ) -> Result<(), GovernorError> {
        self._set_voting_delay(new_voting_delay);
        Ok(())
    }

    #[modifiers(governor::only_governance())]
    default fn set_voting_period(
        &mut self,
        new_voting_period: BlockNumber,
    ) -> Result<(), GovernorError> {
        self._set_voting_period(new_voting_period);
        Ok(())
    }

    #[modifiers(governor::only_governance())]
    default fn set_proposal_threshold(
        &mut self,
        new_proposal_threshold: u64,
    ) -> Result<(), GovernorError> {
        self._set_proposal_threshold(new_proposal_threshold);
        Ok(())
    }
}

pub trait Internal {
    fn _init_with_settings(
        &mut self,
        voting_delay: BlockNumber,
        voting_period: BlockNumber,
        proposal_threshold: u64,
    );

    fn _set_voting_delay(&mut self, new_voting_delay: BlockNumber);

    fn _set_voting_period(&mut self, new_voting_period: BlockNumber);

    fn _set_proposal_threshold(&mut self, new_proposal_threshold: u64);
}

impl<T: Storage<Data>> Internal for T {
    default fn _init_with_settings(
        &mut self,
        voting_delay: BlockNumber,
        voting_period: BlockNumber,
        proposal_threshold: u64,
    ) {
        self._set_voting_delay(voting_delay);
        self._set_voting_period(voting_period);
        self._set_proposal_threshold(proposal_threshold);
    }

    default fn _set_voting_delay(&mut self, new_voting_delay: BlockNumber) {
        self.data().voting_delay = new_voting_delay;
    }

    default fn _set_voting_period(&mut self, new_voting_period: BlockNumber) {
        self.data().voting_period = new_voting_period;
    }

    default fn _set_proposal_threshold(&mut self, new_proposal_threshold: u64) {
        self.data().proposal_threshold = new_proposal_threshold;
    }
}

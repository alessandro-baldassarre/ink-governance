pub use crate::{
    governance::extensions::{
        governor_settings,
        governor_settings::Internal as _,
    },
    traits::governance::extensions::settings::*,
};

use crate::governor::{
    self,
    modules::{
        counter::Counter,
        voter::Voter,
    },
};

use crate::governance::governor::*;

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

/// Unique storage key
pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(GovernorSetting);

/// Governor settings extension upgradeable storage struct
#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    /// The numbers of blocks from the moment of the proposal to when it becomes active for voting
    pub voting_delay: BlockNumber,
    /// The number of blocks in which a proposal can be voted on
    pub voting_period: BlockNumber,
    /// The minimum number of votes an account must have to propose and vote
    pub proposal_threshold: u64,
}

impl<T, C, V> GovernorSettings for T
where
    C: Counter,
    C: Storable
        + StorableHint<ManualKey<{ governor::STORAGE_KEY }>>
        + AutoStorableHint<
            ManualKey<719029772, ManualKey<{ governor::STORAGE_KEY }>>,
            Type = C,
        >,
    V: Voter,
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

/// Internal methods that perfom the logics of the contract
pub trait Internal {
    fn _emit_voting_delay_set(
        &self,
        _old_voting_delay: BlockNumber,
        _new_voting_delay: BlockNumber,
    );

    fn _emit_voting_period_set(
        &self,
        _old_voting_period: BlockNumber,
        _new_voting_period: BlockNumber,
    );

    fn _emit_proposal_threshold_set(
        &self,
        _old_proposal_threshold: u64,
        _new_proposal_threshold: u64,
    );

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
    default fn _emit_voting_delay_set(
        &self,
        _old_voting_delay: BlockNumber,
        _new_voting_delay: BlockNumber,
    ) {
    }

    default fn _emit_voting_period_set(
        &self,
        _old_voting_period: BlockNumber,
        _new_voting_period: BlockNumber,
    ) {
    }

    default fn _emit_proposal_threshold_set(
        &self,
        _old_proposal_threshold: u64,
        _new_proposal_threshold: u64,
    ) {
    }

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
        let old_voting_delay = self.data().voting_delay;
        self._emit_voting_delay_set(old_voting_delay, new_voting_delay);

        self.data().voting_delay = new_voting_delay;
    }

    default fn _set_voting_period(&mut self, new_voting_period: BlockNumber) {
        let old_voting_period = self.data().voting_period;
        self._emit_voting_period_set(old_voting_period, new_voting_period);

        self.data().voting_period = new_voting_period;
    }

    default fn _set_proposal_threshold(&mut self, new_proposal_threshold: u64) {
        let old_proposal_threshold = self.data().proposal_threshold;
        self._emit_proposal_threshold_set(old_proposal_threshold, new_proposal_threshold);

        self.data().proposal_threshold = new_proposal_threshold;
    }
}

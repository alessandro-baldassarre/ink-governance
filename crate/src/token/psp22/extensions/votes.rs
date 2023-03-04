pub use crate::traits::{
    governance::utils::votes::*,
    token::psp22::extensions::votes::*,
};

use openbrush::{
    storage::Mapping,
    traits::{
        AccountId,
        Storage,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub delegates: Mapping<AccountId, AccountId>,
    pub checkpoints: Mapping<AccountId, Checkpoint>,
    pub total_supply_checkpoints: Mapping<(), Checkpoint>,
    pub _reserved: Option<()>,
}

impl<T: Storage<Data>> Votes for T {}

impl<T: Storage<Data>> PSP22Votes for T {}

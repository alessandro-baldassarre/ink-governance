use crate::traits::errors::{
    PSP22VotesError,
    VotesError,
};
pub use crate::{
    psp22::extensions::votes::Internal as _,
    traits::{
        governance::utils::votes::*,
        token::psp22::extensions::votes::*,
    },
};

pub use psp22::{
    Internal as _,
    Transfer as _,
};

use ink::prelude::vec::Vec;

use openbrush::{
    contracts::psp22::*,
    storage::Mapping,
    traits::{
        AccountId,
        Balance,
        BlockNumber,
        OccupiedStorage,
        Storage,
        String,
        ZERO_ADDRESS,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

pub type OldWeight = Vote;
pub type NewWeight = Vote;

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data {
    pub delegates: Mapping<AccountId, AccountId>,
    pub checkpoints: Mapping<AccountId, Vec<Checkpoint>>,
    pub total_supply_checkpoints: Vec<Checkpoint>,
    pub _reserved: Option<()>,
}

impl<T> Votes for T
where
    T: Storage<Data> + Storage<psp22::Data>,
    T: OccupiedStorage<{ STORAGE_KEY }, WithData = Data>
        + OccupiedStorage<{ psp22::STORAGE_KEY }, WithData = psp22::Data>,
{
    default fn delegates(&self, account: AccountId) -> Result<AccountId, VotesError> {
        let delegate_account = self._delegates(&account)?;
        Ok(delegate_account)
    }

    default fn get_votes(&self, account: AccountId) -> Result<Vote, VotesError> {
        let votes = self._get_votes(&account)?;
        Ok(votes)
    }

    default fn get_past_votes(
        &self,
        account: AccountId,
        block_number: BlockNumber,
    ) -> Result<u64, VotesError> {
        if block_number > Self::env().block_number() {
            return Err(VotesError::NotMinedBlock)
        }

        let checkpoints = self._get_checkpoints(&account)?;

        let past_votes = self._get_past_votes(&checkpoints, &block_number)?;

        Ok(past_votes)
    }

    default fn get_past_total_supply(
        &self,
        block_number: BlockNumber,
    ) -> Result<Vote, VotesError> {
        if block_number > Self::env().block_number() {
            return Err(VotesError::NotMinedBlock)
        }

        let past_total_supply = self._get_past_votes(
            &self.data::<Data>().total_supply_checkpoints,
            &block_number,
        )?;

        Ok(past_total_supply)
    }

    default fn delegate(&mut self, delegatee: AccountId) -> Result<(), VotesError> {
        self._delegate(&Self::env().caller(), &delegatee)?;

        Ok(())
    }
}

impl<T> PSP22Votes for T
where
    T: Storage<Data> + Storage<psp22::Data>,
    T: OccupiedStorage<{ STORAGE_KEY }, WithData = Data>
        + OccupiedStorage<{ psp22::STORAGE_KEY }, WithData = psp22::Data>,
{
    default fn checkpoints(
        &self,
        account: AccountId,
        pos: u32,
    ) -> Result<Checkpoint, PSP22VotesError> {
        let checkpoints = self
            .data::<Data>()
            .checkpoints
            .get(&account)
            .ok_or(PSP22VotesError::from(VotesError::NoCheckpoint))?;

        // TODO: create utils for safe convertion
        let index = u32_to_usize(pos).ok_or(PSP22VotesError::ConvertionError {
            from: String::from("u32"),
            to: String::from("usize"),
        })?;
        if index >= checkpoints.len() {
            return Err(PSP22VotesError::VotesError(VotesError::NoCheckpoint))
        }
        Ok(checkpoints[index].clone())
    }

    default fn num_checkpoints(
        &self,
        account: AccountId,
    ) -> Result<u32, PSP22VotesError> {
        let checkpoints = self
            .data::<Data>()
            .checkpoints
            .get(&account)
            .ok_or(PSP22VotesError::VotesError(VotesError::NoCheckpoint))?;

        let len =
            usize_to_u32(checkpoints.len()).ok_or(PSP22VotesError::ConvertionError {
                from: String::from("usize"),
                to: String::from("u32"),
            })?;

        Ok(len)
    }
}

pub trait Internal {
    fn _get_checkpoints(
        &self,
        account: &AccountId,
    ) -> Result<Vec<Checkpoint>, VotesError>;

    fn _delegates(&self, account: &AccountId) -> Result<AccountId, VotesError>;

    fn _get_votes(&self, account: &AccountId) -> Result<u64, VotesError>;

    fn _get_past_votes(
        &self,
        checkpoints: &[Checkpoint],
        block_number: &BlockNumber,
    ) -> Result<u64, VotesError>;

    fn _delegate(
        &mut self,
        delegator: &AccountId,
        delegatee: &AccountId,
    ) -> Result<(), VotesError>;

    fn _move_voting_power(
        &mut self,
        source: &AccountId,
        destination: &AccountId,
        amount: &Balance,
    ) -> Result<(), VotesError>;

    fn _write_checkpoint<F>(
        &mut self,
        address_checkpoints: Option<&AccountId>,
        op: F,
        delta: &Balance,
    ) -> Result<(OldWeight, NewWeight), VotesError>
    where
        F: FnOnce(Vote, Vote) -> Vote;

    fn _after_token_transfer_votes(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), VotesError>;
}

impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp22::Data>,
    T: OccupiedStorage<{ STORAGE_KEY }, WithData = Data>
        + OccupiedStorage<{ psp22::STORAGE_KEY }, WithData = psp22::Data>,
{
    #[inline]
    default fn _get_checkpoints(
        &self,
        account: &AccountId,
    ) -> Result<Vec<Checkpoint>, VotesError> {
        let checkpoints = self
            .data::<Data>()
            .checkpoints
            .get(account)
            .ok_or(VotesError::ZeroCheckpoints)?;

        Ok(checkpoints)
    }
    default fn _delegates(&self, account: &AccountId) -> Result<AccountId, VotesError> {
        let delegate_account = self
            .data::<Data>()
            .delegates
            .get(account)
            .ok_or(VotesError::ZeroDelegatesAccount)?;

        Ok(delegate_account)
    }

    #[inline]
    default fn _get_votes(&self, account: &AccountId) -> Result<Vote, VotesError> {
        let checkpoints = self._get_checkpoints(account)?;

        // We can unwrap() there is at least one value for a mapping key
        let votes = checkpoints.last().unwrap().votes;

        Ok(votes)
    }

    default fn _get_past_votes(
        &self,
        checkpoints: &[Checkpoint],
        block_number: &BlockNumber,
    ) -> Result<Vote, VotesError> {
        let votes = match checkpoints
            .partition_point(|checkpoint| &checkpoint.from_block < block_number)
            .checked_sub(1)
        {
            Some(index) => checkpoints[index].votes,
            None => 0,
        };

        Ok(votes)
    }

    default fn _delegate(
        &mut self,
        delegator: &AccountId,
        delegatee: &AccountId,
    ) -> Result<(), VotesError> {
        let current_delegate = self._delegates(delegator).unwrap_or(*delegator);
        let delegator_balance = self.data::<psp22::Data>()._balance_of(delegator);
        self.data::<Data>().delegates.insert(delegator, delegatee);

        // TODO:
        // emit delegate_changed
        self._move_voting_power(&current_delegate, delegatee, &delegator_balance)?;

        Ok(())
    }
    default fn _move_voting_power(
        &mut self,
        source: &AccountId,
        destination: &AccountId,
        amount: &Balance,
    ) -> Result<(), VotesError> {
        if source == destination {
            return Err(VotesError::MovePowerAccountsError)
        }
        if amount <= &0 {
            return Err(VotesError::MovePowerAmountError)
        }
        if source != &ZERO_ADDRESS.into() {
            let (_old_weight, _new_weight) = self._write_checkpoint(
                Some(source),
                |a: Vote, b: Vote| -> Vote { a - b },
                amount,
            )?;
            //_emit_delegate_vote_changed
        }
        if destination != &ZERO_ADDRESS.into() {
            let (_old_weight, _new_weight) = self._write_checkpoint(
                Some(destination),
                |a: Vote, b: Vote| -> Vote { a + b },
                amount,
            )?;
            //_emit_delegate_vote_changed
        }
        Ok(())
    }

    default fn _write_checkpoint<F>(
        &mut self,
        address_checkpoints: Option<&AccountId>,
        op: F,
        delta: &Balance,
    ) -> Result<(OldWeight, NewWeight), VotesError>
    where
        F: FnOnce(Vote, Vote) -> Vote,
    {
        let mut checkpoints = match address_checkpoints {
            Some(account_id) => {
                self.data::<Data>()
                    .checkpoints
                    .get(account_id)
                    .unwrap_or(Vec::default())
            }
            None => self.data::<Data>().total_supply_checkpoints.clone(),
        };

        let pos = checkpoints.len();
        let old_checkpoint = match pos {
            0 => Checkpoint::default(),
            _ => checkpoints[pos - 1].clone(),
        };

        let delta_converted =
            balance_to_vote(*delta).ok_or(VotesError::BalanceToVoteErr)?;

        let old_weight = old_checkpoint.votes;
        let new_weight = op(old_weight, delta_converted);

        if pos > 0 && old_checkpoint.from_block == Self::env().block_number() {
            checkpoints[pos - 1].votes = new_weight;
        } else {
            checkpoints.push(Checkpoint {
                from_block: Self::env().block_number(),
                votes: new_weight,
            });
        }

        if let Some(account_id) = address_checkpoints {
            self.data::<Data>()
                .checkpoints
                .insert(account_id, &checkpoints);
        } else {
            self.data::<Data>().total_supply_checkpoints = checkpoints
        }

        Ok((old_weight, new_weight))
    }
    default fn _after_token_transfer_votes(
        &mut self,
        from: Option<&AccountId>,
        to: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), VotesError> {
        match (from, to) {
            (Some(from), Some(to)) => {
                self._move_voting_power(
                    &self.delegates(*from).unwrap_or(ZERO_ADDRESS.into()),
                    &self.delegates(*to).unwrap_or(ZERO_ADDRESS.into()),
                    amount,
                )?;
                return Ok(())
            }
            (Some(from), None) => {
                self._write_checkpoint(
                    None,
                    |a: Vote, b: Vote| -> Vote { a - b },
                    amount,
                )?;
                self._move_voting_power(from, &ZERO_ADDRESS.into(), amount)?;
                return Ok(())
            }
            (None, Some(to)) => {
                self._write_checkpoint(
                    None,
                    |a: Vote, b: Vote| -> Vote { a + b },
                    amount,
                )?;
                self._move_voting_power(&ZERO_ADDRESS.into(), to, amount)?;

                return Ok(())
            }
            _ => return Err(VotesError::MovePowerAmountError),
        }
    }
}

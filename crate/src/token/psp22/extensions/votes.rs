use crate::traits::errors::{
    PSP22VotesError,
    VotesError,
};
pub use crate::{
    psp22::extensions::votes::Internal as _,
    token::psp22::extensions::votes,
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
    default fn delegates(&self, account: AccountId) -> Option<AccountId> {
        self._delegates(&account)
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

        let past_votes = self._get_past_votes(&checkpoints, &block_number);

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
        );

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
            .ok_or(PSP22VotesError::VotesError(VotesError::NoCheckpoint))?;

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
    /// User must override those methods in their contract.
    /// Emitted when an account changes their delegate.
    fn _emit_delegate_changed(
        &self,
        _delegator: AccountId,
        _from_delegate: Option<AccountId>,
        _to_delegate: AccountId,
    );

    /// Emitted when a token transfer or delegate change results in changes to a delegate's number
    /// of votes.
    fn _emit_delegate_votes_changed(
        &self,
        _delegate: AccountId,
        _previous_balance: Balance,
        _new_balance: Balance,
    );

    fn _get_checkpoints(
        &self,
        account: &AccountId,
    ) -> Result<Vec<Checkpoint>, VotesError>;

    fn _delegates(&self, account: &AccountId) -> Option<AccountId>;

    fn _get_votes(&self, account: &AccountId) -> Result<u64, VotesError>;

    fn _get_past_votes(
        &self,
        checkpoints: &[Checkpoint],
        block_number: &BlockNumber,
    ) -> Vote;

    fn _delegate(
        &mut self,
        delegator: &AccountId,
        delegatee: &AccountId,
    ) -> Result<(), VotesError>;

    fn _move_voting_power(
        &mut self,
        source: &Option<AccountId>,
        destination: &Option<AccountId>,
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
    ) -> Result<(), PSP22VotesError>;
}

impl<T> Internal for T
where
    T: Storage<Data> + Storage<psp22::Data>,
    T: OccupiedStorage<{ STORAGE_KEY }, WithData = Data>
        + OccupiedStorage<{ psp22::STORAGE_KEY }, WithData = psp22::Data>,
{
    default fn _emit_delegate_changed(
        &self,
        _delegator: AccountId,
        _from_delegate: Option<AccountId>,
        _to_delegate: AccountId,
    ) {
    }

    default fn _emit_delegate_votes_changed(
        &self,
        _delegate: AccountId,
        _previous_balance: Balance,
        _new_balance: Balance,
    ) {
    }

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
    default fn _delegates(&self, account: &AccountId) -> Option<AccountId> {
        self.data::<Data>().delegates.get(account)
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
    ) -> Vote {
        let votes = match checkpoints
            .partition_point(|checkpoint| &checkpoint.from_block <= block_number)
            .checked_sub(1)
        {
            Some(index) => checkpoints[index].votes,
            None => 0,
        };

        votes
    }

    default fn _delegate(
        &mut self,
        delegator: &AccountId,
        delegatee: &AccountId,
    ) -> Result<(), VotesError> {
        let current_delegate = self._delegates(delegator).or(Some(*delegator));
        let delegator_balance = self.data::<psp22::Data>()._balance_of(delegator);
        self.data::<Data>().delegates.insert(delegator, delegatee);

        self._move_voting_power(
            &current_delegate,
            &Some(*delegatee),
            &delegator_balance,
        )?;
        self._emit_delegate_changed(*delegator, current_delegate, *delegatee);

        Ok(())
    }
    default fn _move_voting_power(
        &mut self,
        source: &Option<AccountId>,
        destination: &Option<AccountId>,
        amount: &Balance,
    ) -> Result<(), VotesError> {
        if amount <= &0 {
            return Err(VotesError::MovePowerAmountError)
        }
        if let Some(source) = source {
            let (old_weight, new_weight) = self._write_checkpoint(
                Some(source),
                |a: Vote, b: Vote| -> Vote { a - b },
                amount,
            )?;
            self._emit_delegate_votes_changed(
                *source,
                old_weight.into(),
                new_weight.into(),
            );
        }
        if let Some(destination) = destination {
            let (old_weight, new_weight) = self._write_checkpoint(
                Some(destination),
                |a: Vote, b: Vote| -> Vote { a + b },
                amount,
            )?;
            self._emit_delegate_votes_changed(
                *destination,
                old_weight.into(),
                new_weight.into(),
            );
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
    ) -> Result<(), PSP22VotesError> {
        match (from, to) {
            (Some(from), Some(to)) => {
                self._move_voting_power(
                    &self.delegates(*from).or(Some(*from)),
                    &self.delegates(*to).or(Some(*to)),
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
                self._move_voting_power(
                    &self.delegates(*from).or(Some(*from)),
                    &None,
                    amount,
                )?;
                return Ok(())
            }
            (None, Some(to)) => {
                self._write_checkpoint(
                    None,
                    |a: Vote, b: Vote| -> Vote { a + b },
                    amount,
                )?;
                self._move_voting_power(
                    &None,
                    &self.delegates(*to).or(Some(*to)),
                    amount,
                )?;

                return Ok(())
            }
            _ => {
                return Err(PSP22VotesError::VotesError(
                    VotesError::MovePowerAmountError,
                ))
            }
        }
    }
}

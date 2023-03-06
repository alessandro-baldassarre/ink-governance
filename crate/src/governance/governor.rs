pub use crate::{
    governor,
    governor::Internal as _,
    traits::governance::*,
};

use crate::governance::modules::{
    counter,
    voter,
};

use ink::{
    env::{
        call::{
            build_call,
            Call,
            ExecutionInput,
        },
        hash::Blake2x256,
        CallFlags,
        DefaultEnvironment,
        Gas,
    },
    prelude::{
        collections::vec_deque::VecDeque,
        vec::Vec,
    },
    storage::traits::{
        AutoStorableHint,
        ManualKey,
        Storable,
        StorableHint,
    },
};
use openbrush::{
    modifier_definition,
    storage::Mapping,
    traits::{
        AccountId,
        BlockNumber,
        Hash,
        OccupiedStorage,
        Storage,
        String,
    },
};

/// A ProposalCore describe internal parameters for a proposal
#[derive(scale::Decode, scale::Encode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct ProposalCore {
    /// The block number when voting for a proposal start
    pub vote_start: BlockNumber,
    /// The block number when voting for a proposal end
    pub vote_end: BlockNumber,
    /// A boolean value that describe if the proposal is been executed
    pub executed: bool,
    /// A boolean value that describe if the proposal is been canceled
    pub canceled: bool,
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Data<C = counter::Counting, V = voter::Voting>
where
    C: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<719029772, ManualKey<{ STORAGE_KEY }>>, Type = C>,
    V: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3230629697, ManualKey<{ STORAGE_KEY }>>, Type = V>,
{
    pub proposals: Mapping<ProposalId, ProposalCore>,
    // This queue keeps track of the governor operating on itself. Calls to functions protected by the
    // {only_governance} modifier needs to be whitelisted in this queue. Whitelisting is set in {_before_execute},
    // consumed by the {only_governance} modifier and eventually reset in {_after_execute}. This ensures that the
    // execution of {only_governance} protected calls can only be achieved through successful proposals.
    pub governance_call: VecDeque<[u8; 4]>,
    // The module that determine valid voting options.
    pub counting_module: C,
    // The module that determine the source of voting power.
    pub voting_module: V,
    pub _reserved: Option<()>,
}

#[modifier_definition]
pub fn only_governance<T, C, V, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    C: counter::Counter,
    C: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<719029772, ManualKey<{ STORAGE_KEY }>>, Type = C>,
    V: voter::Voter,
    V: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3230629697, ManualKey<{ STORAGE_KEY }>>, Type = V>,
    T: Storage<Data<C, V>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<C, V>>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<GovernorError>,
{
    if T::env().caller() != instance.data()._executor() {
        return Err(GovernorError::OnlyGovernance.into())
    }

    body(instance)
}

impl<T, C, V> Governor for T
where
    C: counter::Counter,
    C: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<719029772, ManualKey<{ STORAGE_KEY }>>, Type = C>,
    V: voter::Voter,
    V: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3230629697, ManualKey<{ STORAGE_KEY }>>, Type = V>,
    T: Storage<Data<C, V>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<C, V>>,
{
    default fn hash_proposal(
        &self,
        proposal: Proposal,
        description_hash: Hash,
    ) -> ProposalId {
        self._hash_proposal(&proposal, &description_hash)
    }

    default fn state(
        &self,
        proposal_id: ProposalId,
    ) -> Result<ProposalState, GovernorError> {
        let proposal = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::ProposalNotFound)?;
        if proposal.executed {
            return Ok(ProposalState::Executed)
        }
        if proposal.canceled {
            return Ok(ProposalState::Canceled)
        }

        let snapshot = self.proposal_snapshot(proposal_id)?;

        if snapshot >= Self::env().block_number() {
            return Ok(ProposalState::Pending)
        }

        let deadline = self.proposal_deadline(proposal_id)?;

        if deadline >= Self::env().block_number() {
            return Ok(ProposalState::Active)
        }

        if self._quorum_reached(&proposal_id) && self._vote_succeeded(&proposal_id) {
            Ok(ProposalState::Succeeded)
        } else {
            Ok(ProposalState::Defeated)
        }
    }

    default fn proposal_snapshot(
        &self,
        proposal_id: ProposalId,
    ) -> Result<BlockNumber, GovernorError> {
        let vote_start = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::ProposalNotFound)?
            .vote_start;
        Ok(vote_start)
    }

    default fn proposal_deadline(
        &self,
        proposal_id: ProposalId,
    ) -> Result<BlockNumber, GovernorError> {
        let vote_end = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::ProposalNotFound)?
            .vote_end;
        Ok(vote_end)
    }

    default fn counting_mode(&self) -> String {
        String::from("")
    }

    default fn voting_delay(&self) -> u32 {
        self._voting_delay()
    }

    default fn voting_period(&self) -> u32 {
        self._voting_period()
    }

    default fn proposal_threshold(&self) -> u64 {
        self._proposal_threshold()
    }

    default fn propose(
        &mut self,
        proposal: Proposal,
        description: String,
    ) -> Result<ProposalId, GovernorError> {
        if self.get_votes(
            Self::env().caller(),
            Self::env().block_number().saturating_sub(1),
        )? <= self._proposal_threshold()
        {
            return Err(GovernorError::BelowThreshold)
        }

        let description_hash =
            Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&description).as_ref())
                .unwrap();
        let proposal_id = self._hash_proposal(&proposal, &description_hash);

        if proposal.selector.is_empty() && description.is_empty() {
            return Err(GovernorError::EmptyProposal)
        }

        if self.data().proposals.get(&proposal_id).is_some() {
            return Err(GovernorError::ProposalAlreadyExist)
        }

        let vote_start = Self::env().block_number() + self._voting_delay();
        let vote_end = vote_start + self._voting_period();

        let proposal_core = ProposalCore {
            vote_start,
            vote_end,
            executed: false,
            canceled: false,
        };

        self.data().proposals.insert(&proposal_id, &proposal_core);

        self._emit_proposal_created(
            Self::env().caller(),
            proposal_id,
            proposal,
            vote_start,
            vote_end,
            description,
        );

        Ok(proposal_id)
    }

    default fn execute(
        &mut self,
        proposal: Proposal,
        description_hash: Hash,
    ) -> Result<ProposalId, GovernorError> {
        let proposal_id = self._hash_proposal(&proposal, &description_hash);
        let status = self.state(proposal_id)?;

        match status {
            ProposalState::Succeeded | ProposalState::Queued => {}
            _ => return Err(GovernorError::ProposalNotSuccessful),
        }

        let mut proposal_core = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::ProposalNotFound)?;

        self._before_execute(&proposal)?;
        self._execute(&proposal_id, &proposal)?;

        proposal_core.executed = true;

        self.data().proposals.insert(&proposal_id, &proposal_core);

        self._after_execute()?;

        Ok(proposal_id)
    }

    default fn get_votes(
        &self,
        account: AccountId,
        block_number: BlockNumber,
    ) -> Result<u64, GovernorError> {
        let votes = self._get_votes(&account, block_number, &self._default_params())?;

        Ok(votes)
    }

    default fn get_votes_with_params(
        &self,
        account: AccountId,
        block_number: BlockNumber,
        params: Vec<u8>,
    ) -> Result<u64, GovernorError> {
        let votes = self._get_votes(&account, block_number, &params)?;

        Ok(votes)
    }

    default fn cast_vote(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
    ) -> Result<u64, GovernorError> {
        let voter = Self::env().caller();
        let votes = self._cast_vote(&proposal_id, &voter, support, &String::from(""))?;
        Ok(votes)
    }

    default fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
    ) -> Result<u64, GovernorError> {
        let voter = Self::env().caller();
        let votes = self._cast_vote(&proposal_id, &voter, support, &reason)?;
        Ok(votes)
    }

    default fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        params: Vec<u8>,
    ) -> Result<u64, GovernorError> {
        let voter = Self::env().caller();
        let votes =
            self._cast_vote_with_params(&proposal_id, &voter, support, &reason, &params)?;
        Ok(votes)
    }

    default fn relay(&mut self, proposal: Proposal) -> Result<(), GovernorError> {
        unimplemented!()
    }
}

pub trait Internal {
    /// User must override those methods in their contract.
    fn _emit_proposal_created(
        &self,
        _proposer: AccountId,
        _proposal_id: ProposalId,
        _proposal: Proposal,
        _start_block: BlockNumber,
        _end_block: BlockNumber,
        _description: String,
    );
    fn _emit_proposal_canceled(&self, _proposal_id: ProposalId);
    fn _emit_proposal_executed(&self, _proposal_id: ProposalId);
    fn _emit_vote_cast(
        &self,
        _voter: AccountId,
        _proposal_id: ProposalId,
        _support: u8,
        _weight: u64,
        _reason: String,
    );
    fn _emit_vote_cast_with_params(
        &self,
        _voter: AccountId,
        _proposal_id: ProposalId,
        _support: u8,
        _weight: u64,
        _reason: String,
        _params: Vec<u8>,
    );

    /// Returns the number of votes required in order for a voter to become a proposer.
    fn _proposal_threshold(&self) -> u64;

    /// Returns Delay, in number of blocks, between the proposal is created and the vote starts.
    /// This can be increased to leave time for users to buy voting power, or delegate it, before
    /// the voting of a proposal starts.
    fn _voting_delay(&self) -> u32;

    /// Returns Delay, in number of blocks, between the vote start and vote ends.
    ///
    /// Note: The votingDelay can delay the start of the vote. This must be considered when setting
    /// the voting duration compared to the voting delay.
    fn _voting_period(&self) -> u32;

    fn _hash_proposal(&self, proposal: &Proposal, description_hash: &Hash) -> ProposalId;

    /// If amount of votes already cast passes the threshold limit.
    fn _quorum_reached(&self, proposal_id: &ProposalId) -> bool;

    /// If the proposal is successful or not.
    fn _vote_succeeded(&self, proposal_id: &ProposalId) -> bool;

    /// Get the voting weight of account at a specific blockNumber, for a vote as described by params.
    fn _get_votes(
        &self,
        account: &AccountId,
        block_number: BlockNumber,
        params: &[u8],
    ) -> Result<u64, GovernorError>;

    /// Register a vote for proposalId by account with a given support, voting weight and voting params.
    ///
    /// Note: Support is generic and can represent various things depending on the voting system used.
    fn _count_vote(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        weight: u64,
        params: &[u8],
    );

    /// Default additional encoded parameters used by castVote methods that don’t include them
    ///
    /// Note: Should be overridden by specific implementations to use an appropriate value, the
    /// meaning of the additional params, in the context of that implementation
    fn _default_params(&self) -> Vec<u8>;

    /// Internal execution mechanism. Can be overridden to implement different execution
    /// mechanism
    fn _execute(
        &mut self,
        proposal_id: &ProposalId,
        proposal: &Proposal,
    ) -> Result<(), GovernorError>;

    /// Hook before execution is triggered.
    fn _before_execute(&mut self, proposal: &Proposal) -> Result<(), GovernorError>;

    /// Hook after execution is triggered.
    fn _after_execute(&mut self) -> Result<(), GovernorError>;

    /// Internal cancel mechanism: locks up the proposal timer, preventing it from being re-submitted. Marks it as canceled to allow distinguishing it from executed proposals.
    ///
    /// Emits a ProposalCanceled event.
    fn _cancel(
        &mut self,
        proposal: &Proposal,
        description_hash: &Hash,
    ) -> Result<ProposalId, GovernorError>;

    /// Internal vote casting mechanism: Check that the vote is pending, that it has not been cast yet, retrieve voting weight using Governor.get_votes() and call the _count_vote() internal function. Uses the _default_params().
    ///
    /// Emits a VoteCast event.
    fn _cast_vote(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        reason: &String,
    ) -> Result<u64, GovernorError>;

    /// Internal vote casting mechanism: Check that the vote is pending, that it has not been cast yet, retrieve voting weight using Governor.get_votes() and call the _count_vote()
    /// internal function.
    ///
    /// Emits a VoteCast event.
    fn _cast_vote_with_params(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        reason: &String,
        params: &[u8],
    ) -> Result<u64, GovernorError>;

    /// Address through which the governor executes action. Will be overloaded by module that execute actions through another contract such as a time-lock.
    fn _executor(&self) -> AccountId;
}

impl<T, C, V> Internal for T
where
    C: counter::Counter,
    C: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<719029772, ManualKey<{ STORAGE_KEY }>>, Type = C>,
    V: voter::Voter,
    V: Storable
        + StorableHint<ManualKey<{ STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3230629697, ManualKey<{ STORAGE_KEY }>>, Type = V>,
    T: Storage<Data<C, V>>,
    T: OccupiedStorage<STORAGE_KEY, WithData = Data<C, V>>,
{
    default fn _emit_proposal_created(
        &self,
        _proposer: AccountId,
        _proposal_id: ProposalId,
        _proposal: Proposal,
        _start_block: BlockNumber,
        _end_block: BlockNumber,
        _description: String,
    ) {
    }
    default fn _emit_proposal_canceled(&self, _proposal_id: ProposalId) {}
    default fn _emit_proposal_executed(&self, _proposal_id: ProposalId) {}
    default fn _emit_vote_cast(
        &self,
        _voter: AccountId,
        _proposal_id: ProposalId,
        _support: u8,
        _weight: u64,
        _reason: String,
    ) {
    }
    default fn _emit_vote_cast_with_params(
        &self,
        _voter: AccountId,
        _proposal_id: ProposalId,
        _support: u8,
        _weight: u64,
        _reason: String,
        _params: Vec<u8>,
    ) {
    }

    default fn _proposal_threshold(&self) -> u64 {
        0
    }

    default fn _voting_delay(&self) -> u32 {
        0
    }

    default fn _voting_period(&self) -> u32 {
        0
    }

    default fn _hash_proposal(
        &self,
        proposal: &Proposal,
        description_hash: &Hash,
    ) -> ProposalId {
        let mut hash_data: Vec<u8> = Vec::new();

        hash_data.append(&mut scale::Encode::encode(&proposal));
        hash_data.append(&mut scale::Encode::encode(&description_hash));

        Hash::try_from(Self::env().hash_bytes::<Blake2x256>(&hash_data).as_ref()).unwrap()
    }

    default fn _quorum_reached(&self, proposal_id: &ProposalId) -> bool {
        self.data()
            .counting_module
            ._quorum_reached(proposal_id)
            .unwrap()
    }

    default fn _vote_succeeded(&self, proposal_id: &ProposalId) -> bool {
        self.data()
            .counting_module
            ._vote_succeeded(proposal_id)
            .unwrap()
    }

    default fn _get_votes(
        &self,
        account: &AccountId,
        block_number: BlockNumber,
        params: &[u8],
    ) -> Result<u64, GovernorError> {
        if let Some(votes) =
            self.data()
                .voting_module
                ._get_votes(account, block_number, params)
        {
            return Ok(votes)
        }

        Err(GovernorError::NoVotes)
    }

    default fn _count_vote(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        weight: u64,
        params: &[u8],
    ) {
        self.data()
            .counting_module
            ._count_vote(proposal_id, account, support, weight, params)
            .unwrap()
    }

    default fn _default_params(&self) -> Vec<u8> {
        Vec::default()
    }

    default fn _execute(
        &mut self,
        proposal_id: &ProposalId,
        proposal: &Proposal,
    ) -> Result<(), GovernorError> {
        // Flush the state into storage before the cross call.
        // Because during cross call we can call this contract.
        self.flush();
        let result = build_call::<DefaultEnvironment>()
            .call_type(
                Call::new(proposal.callee)
                    .gas_limit(0)
                    .transferred_value(proposal.transferred_value),
            )
            .exec_input(
                ExecutionInput::new(proposal.selector.into())
                    .push_arg(CallInput(&proposal.input)),
            )
            .returns::<()>()
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .try_invoke()
            .map_err(|_| GovernorError::CallRevertedWithoutMessage);

        // Load the state of the contract after the cross call.
        self.load();
        self._emit_proposal_executed(*proposal_id);

        let post_call = result?;

        post_call?;

        Ok(())
    }

    default fn _before_execute(
        &mut self,
        proposal: &Proposal,
    ) -> Result<(), GovernorError> {
        if self._executor() != Self::env().account_id() {
            self.data().governance_call.push_back(proposal.selector);
        }
        Ok(())
    }

    default fn _after_execute(&mut self) -> Result<(), GovernorError> {
        if self._executor() != Self::env().account_id()
            && !self.data().governance_call.is_empty()
        {
            self.data().governance_call.clear();
        }

        Ok(())
    }

    default fn _cancel(
        &mut self,
        proposal: &Proposal,
        description_hash: &Hash,
    ) -> Result<ProposalId, GovernorError> {
        let proposal_id = self._hash_proposal(proposal, description_hash);
        let status = self.state(proposal_id)?;
        let mut proposal_core = self
            .data()
            .proposals
            .get(&proposal_id)
            .ok_or(GovernorError::ProposalNotFound)?;

        match status {
            ProposalState::Canceled
            | ProposalState::Expired
            | ProposalState::Executed => return Err(GovernorError::ProposalNotActive),
            _ => {
                proposal_core.canceled = true;
                self.data().proposals.insert(&proposal_id, &proposal_core)
            }
        }

        self._emit_proposal_canceled(proposal_id);
        Ok(proposal_id)
    }

    default fn _cast_vote(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        reason: &String,
    ) -> Result<u64, GovernorError> {
        let vote = self._cast_vote_with_params(
            proposal_id,
            account,
            support,
            reason,
            &self._default_params(),
        )?;
        Ok(vote)
    }

    default fn _cast_vote_with_params(
        &mut self,
        proposal_id: &ProposalId,
        account: &AccountId,
        support: u8,
        reason: &String,
        params: &[u8],
    ) -> Result<u64, GovernorError> {
        let proposal_core = self
            .data()
            .proposals
            .get(proposal_id)
            .ok_or(GovernorError::ProposalNotFound)?;

        let weight = self
            .data()
            ._get_votes(account, proposal_core.vote_start, params)?;

        match self.state(*proposal_id)? {
            ProposalState::Active => {}
            _ => return Err(GovernorError::ProposalNotActive),
        }

        self.data()
            ._count_vote(proposal_id, account, support, weight, params);

        if params.is_empty() {
            self.data()._emit_vote_cast(
                *account,
                *proposal_id,
                support,
                weight,
                reason.to_vec(),
            );
        } else {
            self.data()._emit_vote_cast_with_params(
                *account,
                *proposal_id,
                support,
                weight,
                reason.to_vec(),
                params.to_vec(),
            );
        }

        Ok(weight)
    }

    default fn _executor(&self) -> AccountId {
        Self::env().account_id()
    }
}

/// A wrapper that allows us to encode a blob of bytes.
///
/// We use this to pass the set of untyped (bytes) parameters to the `CallBuilder`.
pub struct CallInput<'a>(&'a [u8]);

impl<'a> scale::Encode for CallInput<'a> {
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        dest.write(self.0);
    }
}

pub use crate::traits::errors::GovernorError;
use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, Balance, BlockNumber, Hash, String, ZERO_ADDRESS};

/// The possible states for a proposal
#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ProposalState {
    Pending,
    Active,
    Canceled,
    Defeated,
    Succeeded,
    Queued,
    Expired,
    Executed,
}

pub type ProposalId = Hash;

/// A Proposal is what can be proposed
#[derive(Debug, Clone, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Proposal {
    /// The `AccountId` of the contract that is called in this transaction.
    pub callee: AccountId,
    /// The selector bytes that identifies the function of the callee that should be called.
    pub selector: [u8; 4],
    /// The SCALE encoded parameters that are passed to the called function.
    pub input: Vec<u8>,
    /// The amount of chain balance that is transferred to the callee.
    pub transferred_value: Balance,
}

impl Default for Proposal {
    fn default() -> Self {
        Self {
            callee: ZERO_ADDRESS.into(),
            selector: Default::default(),
            input: Default::default(),
            transferred_value: Default::default(),
        }
    }
}

#[openbrush::wrapper]
pub type GovernorRef = dyn Governor;

#[openbrush::trait_definition]
pub trait Governor {
    /// Hashing function used to (re)build the proposal id from the proposal details. Returns the generated proposal id.
    #[ink(message)]
    fn hash_proposal(&self, proposal: Proposal, description_hash: Hash) -> ProposalId;

    /// Returns the current state of a proposal
    #[ink(message)]
    fn state(&self, proposal_id: ProposalId) -> Result<ProposalState, GovernorError>;

    /// Returns the block number used to retrieve user’s votes and quorum.
    #[ink(message)]
    fn proposal_snapshot(&self, proposal_id: ProposalId) -> Result<BlockNumber, GovernorError>;

    /// Returns the block number at which votes close.
    #[ink(message)]
    fn proposal_deadline(&self, proposal_id: ProposalId) -> Result<BlockNumber, GovernorError>;

    /// A description of the possible support values for castVote and the way these votes are counted,
    /// meant to be consumed by UIs to show correct vote options and interpret the results.
    #[ink(message)]
    fn counting_mode(&self) -> String;

    /// Create a new proposal.
    ///
    /// Emits a ProposalCreated event.
    #[ink(message)]
    fn propose(
        &mut self,
        proposal: Proposal,
        description: String,
    ) -> Result<ProposalId, GovernorError>;

    /// Execute a successful proposal. This requires the quorum to be reached, the vote to be successful, and the deadline to be reached.
    ///
    /// Emits a ProposalExecuted event.
    ///
    /// Note: some module can modify the requirements for execution, for example by adding an
    /// additional timelock.
    #[ink(message, payable)]
    fn execute(
        &mut self,
        proposal: Proposal,
        description_hash: Hash,
    ) -> Result<ProposalId, GovernorError>;

    /// Returns the voting power of an account at a specific blockNumber.
    #[ink(message)]
    fn get_votes(
        &self,
        account: AccountId,
        block_number: BlockNumber,
    ) -> Result<u64, GovernorError>;

    /// Returns the voting power of an account at a specific blockNumber given additional encoded parameters.
    #[ink(message)]
    fn get_votes_with_params(
        &self,
        account: AccountId,
        block_number: BlockNumber,
        params: Vec<u8>,
    ) -> Result<u64, GovernorError>;

    /// Cast a vote.
    ///
    /// Emits a VoteCast event.
    ///
    /// Returns the weight of the vote
    #[ink(message)]
    fn cast_vote(&mut self, proposal_id: ProposalId, support: u8) -> Result<u64, GovernorError>;

    /// Cast a vote with a reason.
    ///
    /// Emits a VoteCast event.
    ///
    /// Returns the weight of the vote
    #[ink(message)]
    fn cast_vote_with_reason(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
    ) -> Result<u64, GovernorError>;

    /// Cast a vote with a reason and additional encoded params.
    ///
    /// Emits a VoteCast event or VoteCastWithParams event depending on the length of params.
    ///
    /// Returns the weight of the vote
    #[ink(message)]
    fn cast_vote_with_reason_and_params(
        &mut self,
        proposal_id: ProposalId,
        support: u8,
        reason: String,
        params: Vec<u8>,
    ) -> Result<u64, GovernorError>;

    /// Relays a transaction or function call to an arbitrary target. In cases where the governance
    /// executor is some contract other than the governor itself, like when using a timelock, this
    /// function can be invoked in a governance proposal to recover tokens that was sent
    /// to the governor contract by mistake. Note that if the executor is simply the governor
    /// itself, use of relay is redundant.
    #[ink(message)]
    fn relay(&mut self, proposal: Proposal) -> Result<(), GovernorError>;
}

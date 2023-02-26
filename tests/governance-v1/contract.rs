#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
#[openbrush::contract]
pub mod governance_v1 {
    use ink::{
        codegen::{
            EmitEvent,
            Env,
        },
        prelude::vec::Vec,
    };
    use openbrush::{
        contracts::access_control::access_control,
        traits::{
            Storage,
            String,
        },
    };
    use openbrush_governance::{
        governor::*,
        governor_counting_simple::*,
        governor_voting_group::*,
    };

    /// Emitted when a proposal is create
    #[ink(event)]
    pub struct ProposalCreated {
        /// The account that created the proposal.
        #[ink(topic)]
        pub proposer: AccountId,
        /// The id of the created proposal.
        #[ink(topic)]
        pub proposal_id: ProposalId,
        /// The proposal created.
        pub proposal: Proposal,
        /// The block number when the proposal start.
        pub start_block: BlockNumber,
        /// The block number when the proposal end.
        pub end_block: BlockNumber,
        /// Description of the proposal
        pub description: String,
    }

    /// Emitted when a proposal is cancel
    #[ink(event)]
    pub struct ProposalCanceled {
        #[ink(topic)]
        pub proposal_id: ProposalId,
    }

    /// Emitted when a proposal is execute
    #[ink(event)]
    pub struct ProposalExecuted {
        #[ink(topic)]
        pub proposal_id: ProposalId,
    }

    /// Emitted when a vote is cast
    #[ink(event)]
    pub struct VoteCasted {
        /// The account who cast the vote of the proposal.
        #[ink(topic)]
        pub voter: AccountId,
        /// The id of the proposal.
        #[ink(topic)]
        pub proposal_id: ProposalId,
        /// The vote type casted.
        pub support: u8,
        /// The weight of the vote cast.
        pub weight: u64,
        /// Reason of the vote.
        pub reason: String,
    }

    /// Emitted when a vote is cast with params
    #[ink(event)]
    pub struct VoteCastedWithParams {
        /// The account who cast the vote of the proposal.
        #[ink(topic)]
        pub voter: AccountId,
        /// The id of the proposal.
        #[ink(topic)]
        pub proposal_id: ProposalId,
        /// The vote type casted.
        pub support: u8,
        /// The weight of the vote cast.
        pub weight: u64,
        /// Reason of the vote.
        pub reason: String,
        /// Params of the vote.
        pub params: Vec<u8>,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct GovernorStruct {
        #[storage_field]
        governor: governor::Data<
            governor_counting_simple::Counting,
            governor_voting_group::Voting,
        >,
        #[storage_field]
        access_control: access_control::Data,
    }

    impl Governor for GovernorStruct {}

    impl VotingGroup for GovernorStruct {}

    impl CountingSimple for GovernorStruct {}

    // Override the internal methods
    impl governor::Internal for GovernorStruct {
        fn _voting_delay(&self) -> u32 {
            0
        }
        fn _voting_period(&self) -> u32 {
            50400 // 1 week
        }
        fn _emit_proposal_created(
            &self,
            proposer: AccountId,
            proposal_id: ProposalId,
            proposal: Proposal,
            start_block: BlockNumber,
            end_block: BlockNumber,
            description: String,
        ) {
            self.env().emit_event(ProposalCreated {
                proposer,
                proposal_id,
                proposal,
                start_block,
                end_block,
                description,
            })
        }
        fn _emit_vote_cast(
            &self,
            voter: AccountId,
            proposal_id: ProposalId,
            support: u8,
            weight: u64,
            reason: String,
        ) {
            self.env().emit_event(VoteCasted {
                voter,
                proposal_id,
                support,
                weight,
                reason,
            })
        }
        fn _emit_vote_cast_with_params(
            &self,
            voter: AccountId,
            proposal_id: ProposalId,
            support: u8,
            weight: u64,
            reason: String,
            params: Vec<u8>,
        ) {
            self.env().emit_event(VoteCastedWithParams {
                voter,
                proposal_id,
                support,
                weight,
                reason,
                params,
            })
        }
        fn _emit_proposal_canceled(&self, proposal_id: ProposalId) {
            self.env().emit_event(ProposalCanceled { proposal_id })
        }
        fn _emit_proposal_executed(&self, proposal_id: ProposalId) {
            self.env().emit_event(ProposalExecuted { proposal_id })
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum ContractError {
        Custom(String),
        VotingGroupError(VotingGroupError),
    }

    impl From<VotingGroupError> for ContractError {
        fn from(voting: VotingGroupError) -> Self {
            match voting {
                VotingGroupError::NoMember => {
                    ContractError::Custom(String::from("VG: NoMember"))
                }
                _ => ContractError::Custom(String::from("VG: VotingGroupError")),
            }
        }
    }

    impl GovernorStruct {
        /// Initialize the contract with a list of voting members and optional admin (if not set
        /// the caller will be the admin by default)
        #[ink(constructor)]
        pub fn new(
            admin: Option<AccountId>,
            init_members: Vec<VotingMember>,
        ) -> Result<Self, ContractError> {
            let mut instance = Self::default();

            // Assign the admin role to the caller if is not set in the parameters
            let admin = admin.unwrap_or(Self::env().caller());

            // Initialize access_control with the admin.
            //
            // Note: some methods like update_members has the access control (only_role:admin).
            access_control::Internal::_init_with_admin(&mut instance, admin);

            // Initialize the group with the members.
            //
            // Note: Only the members of the group can propose or vote a proposal.
            governor_voting_group::VotingGroup::update_members(
                &mut instance,
                init_members,
                Vec::new(),
            )?;
            Ok(instance)
        }
    }
}

#[cfg(test)]
mod unit_tests;

#[cfg(all(test, feature = "e2e-tests"))]
mod e2e_tests;

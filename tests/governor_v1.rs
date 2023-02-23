#[openbrush::contract]
pub mod governor {

    use ink::{
        codegen::{EmitEvent, Env},
        prelude::vec::Vec,
    };
    use ink_governance::{
        governor::*, governor_counting_simple::*, governor_voting_group::*,
        traits::errors::VotingGroupError,
    };
    use openbrush::{
        contracts::access_control::access_control,
        traits::{Storage, String},
    };

    /// Emitted when a proposal is create
    #[ink(event)]
    pub struct ProposalCreated {
        /// The account that created the proposal.
        #[ink(topic)]
        proposer: AccountId,
        /// The id of the created proposal.
        #[ink(topic)]
        proposal_id: ProposalId,
        /// The proposal created.
        proposal: Proposal,
        /// The block number when the proposal start.
        start_block: BlockNumber,
        /// The block number when the proposal end.
        end_block: BlockNumber,
        /// Description of the proposal
        description: String,
    }

    /// Emitted when a proposal is cancel
    #[ink(event)]
    pub struct ProposalCanceled {
        #[ink(topic)]
        proposal_id: ProposalId,
    }

    /// Emitted when a proposal is execute
    #[ink(event)]
    pub struct ProposalExecuted {
        #[ink(topic)]
        proposal_id: ProposalId,
    }

    /// Emitted when a vote is cast
    #[ink(event)]
    pub struct VoteCasted {
        /// The account who cast the vote of the proposal.
        #[ink(topic)]
        voter: AccountId,
        /// The id of the proposal.
        #[ink(topic)]
        proposal_id: ProposalId,
        /// The vote type casted.
        support: u8,
        /// The weight of the vote cast.
        weight: u64,
        /// Reason of the vote.
        reason: String,
    }

    /// Emitted when a vote is cast with params
    #[ink(event)]
    pub struct VoteCastedWithParams {
        /// The account who cast the vote of the proposal.
        #[ink(topic)]
        voter: AccountId,
        /// The id of the proposal.
        #[ink(topic)]
        proposal_id: ProposalId,
        /// The vote type casted.
        support: u8,
        /// The weight of the vote cast.
        weight: u64,
        /// Reason of the vote.
        reason: String,
        /// Params of the vote.
        params: Vec<u8>,
    }

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct GovernorStruct {
        #[storage_field]
        governor: governor::Data<governor_counting_simple::Counting, governor_voting_group::Voting>,
        #[storage_field]
        access_control: access_control::Data,
    }

    impl Governor for GovernorStruct {}

    impl VotingGroup for GovernorStruct {}

    impl CountingSimple for GovernorStruct {}

    // Override the internal methods
    impl governor::Internal for GovernorStruct {
        fn _voting_delay(&self) -> u32 {
            1 // 1 block
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
        fn from(_voting: VotingGroupError) -> Self {
            ContractError::Custom(String::from("VG: error from VotingGroup"))
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::{DefaultAccounts, EmittedEvent};
        use ink::env::DefaultEnvironment;
        use openbrush::test_utils::{accounts, change_caller};

        type Event = <GovernorStruct as ::ink::reflect::ContractEventBase>::Type;

        fn default_accounts() -> DefaultAccounts<DefaultEnvironment> {
            accounts()
        }

        fn set_caller(sender: AccountId) {
            change_caller(sender)
        }

        fn build_contract() -> GovernorStruct {
            let accounts = default_accounts();

            let alice_member = VotingMember {
                account: accounts.alice,
                voting_power: 1,
            };
            let bob_member = VotingMember {
                account: accounts.bob,
                voting_power: 1,
            };

            let init_members = vec![alice_member.clone(), bob_member];

            set_caller(alice_member.account);

            GovernorStruct::new(None, init_members).unwrap()
        }

        fn decode_events(emittend_events: Vec<EmittedEvent>) -> Vec<Event> {
            emittend_events
                .into_iter()
                .map(|event| {
                    <Event as scale::Decode>::decode(&mut &event.data[..]).expect("invalid data")
                })
                .collect()
        }

        #[ink::test]
        /// The constructor does its job
        fn contruction_works() {
            let accounts = default_accounts();

            let alice_member = VotingMember {
                account: accounts.alice,
                voting_power: 1,
            };
            let bob_member = VotingMember {
                account: accounts.bob,
                voting_power: 1,
            };
            let charlie_member = VotingMember {
                account: accounts.charlie,
                voting_power: 1,
            };
            let members = vec![alice_member.clone(), bob_member];
            let contract = build_contract();

            assert_eq!(
                contract.get_members(vec![alice_member.account]).unwrap(),
                vec![alice_member]
            );
        }
    }
}

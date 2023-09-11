use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_utils::Duration;
use cwd_hooks::HooksResponse;
use cwd_interface::voting::InfoResponse;
use cwd_macros::{info_query, proposal_module_query};
use cwd_voting::{
    multiple_choice::{MultipleChoiceOptions, MultipleChoiceVote, VotingStrategy},
    pre_propose::{PreProposeInfo, ProposalCreationPolicy},
};

#[cw_serde]
pub struct InstantiateMsg {
    /// Voting params configuration
    pub voting_strategy: VotingStrategy,
    /// The minimum amount of time a proposal must be open before
    /// passing. A proposal may fail before this amount of time has
    /// elapsed, but it will not pass. This can be useful for
    /// preventing governance attacks wherein an attacker aquires a
    /// large number of tokens and forces a proposal through.
    pub min_voting_period: Option<Duration>,
    /// The amount of time a proposal can be voted on before expiring
    pub max_voting_period: Duration,
    /// If set to true only members may execute passed
    /// proposals. Otherwise, any address may execute a passed
    /// proposal.
    pub only_members_execute: bool,
    /// Allows changing votes before the proposal expires. If this is
    /// enabled proposals will not be able to complete early as final
    /// vote information is not known until the time of proposal
    /// expiration.
    pub allow_revoting: bool,
    /// Information about what addresses may create proposals.
    pub pre_propose_info: PreProposeInfo,
    /// If set to true proposals will be closed if their execution
    /// fails. Otherwise, proposals will remain open after execution
    /// failure. For example, with this enabled a proposal to send 5
    /// tokens out of a DAO's reserve with 4 tokens would be closed when
    /// it is executed. With this disabled, that same proposal would
    /// remain open until the DAO's reserve was large enough for it to be
    /// executed.
    pub close_proposal_on_execution_failure: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Creates a proposal in the governance module.
    Propose {
        /// The title of the proposal.
        title: String,
        /// A description of the proposal.
        description: String,
        /// The multiple choices.
        choices: MultipleChoiceOptions,
        /// The address creating the proposal. If no pre-propose
        /// module is attached to this module this must always be None
        /// as the proposer is the sender of the propose message. If a
        /// pre-propose module is attached, this must be Some and will
        /// set the proposer of the proposal it creates.
        proposer: Option<String>,
    },
    /// Votes on a proposal. Voting power is determined by the DAO's
    /// voting power module.
    Vote {
        /// The ID of the proposal to vote on.
        proposal_id: u64,
        /// The senders position on the proposal.
        vote: MultipleChoiceVote,
    },
    /// Causes the messages associated with a passed proposal to be
    /// executed by the DAO.
    Execute {
        /// The ID of the proposal to execute.
        proposal_id: u64,
    },
    /// Closes a proposal that has failed (either not passed or timed
    /// out). If applicable this will cause the proposal deposit
    /// associated wth said proposal to be returned.
    Close {
        /// The ID of the proposal to close.
        proposal_id: u64,
    },
    /// Updates the governance module's config.
    UpdateConfig {
        /// The new proposal voting strategy. This will only apply
        /// to proposals created after the config update.
        voting_strategy: VotingStrategy,
        /// The minimum amount of time a proposal must be open before
        /// passing. A proposal may fail before this amount of time has
        /// elapsed, but it will not pass. This can be useful for
        /// preventing governance attacks wherein an attacker aquires a
        /// large number of tokens and forces a proposal through.
        min_voting_period: Option<Duration>,
        /// The default maximum amount of time a proposal may be voted
        /// on before expiring. This will only apply to proposals
        /// created after the config update.
        max_voting_period: Duration,
        /// If set to true only members may execute passed
        /// proposals. Otherwise, any address may execute a passed
        /// proposal. Applies to all outstanding and future proposals.
        only_members_execute: bool,
        /// Allows changing votes before the proposal expires. If this is
        /// enabled proposals will not be able to complete early as final
        /// vote information is not known until the time of proposal
        /// expiration.
        allow_revoting: bool,
        /// The address if tge DAO that this governance module is
        /// associated with.
        dao: String,
        /// If set to true proposals will be closed if their execution
        /// fails. Otherwise, proposals will remain open after execution
        /// failure. For example, with this enabled a proposal to send 5
        /// tokens out of a DAO's reserve with 4 tokens would be closed when
        /// it is executed. With this disabled, that same proposal would
        /// remain open until the DAO's reserve was large enough for it to be
        /// executed.
        close_proposal_on_execution_failure: bool,
    },
    /// Update's the proposal creation policy used for this
    /// module. Only the DAO may call this method.
    UpdatePreProposeInfo {
        info: PreProposeInfo,
    },
    AddProposalHook {
        address: String,
    },
    RemoveProposalHook {
        address: String,
    },
    AddVoteHook {
        address: String,
    },
    RemoveVoteHook {
        address: String,
    },
}

#[proposal_module_query]
#[info_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Gets the governance module's config.
    #[returns(crate::state::Config)]
    Config {},
    /// Gets information about a proposal.
    #[returns(crate::query::ProposalResponse)]
    Proposal { proposal_id: u64 },
    /// Lists all the proposals that have been cast in this
    /// module.
    #[returns(crate::query::ProposalListResponse)]
    ListProposals {
        start_after: Option<u64>,
        limit: Option<u64>,
    },
    /// Lists all of the proposals that have been cast in this module
    /// in descending order of proposal ID.
    #[returns(crate::query::ProposalListResponse)]
    ReverseProposals {
        start_before: Option<u64>,
        limit: Option<u64>,
    },
    /// Returns the number of proposals that have been created in this
    /// module./// Returns a voters position on a propsal.
    #[returns(u64)]
    ProposalCount {},
    /// Returns a voters position on a proposal.
    #[returns(crate::query::VoteResponse)]
    GetVote { proposal_id: u64, voter: String },
    /// Lists all of the votes that have been cast on a
    /// proposal.
    #[returns(crate::query::VoteListResponse)]
    ListVotes {
        proposal_id: u64,
        start_after: Option<String>,
        limit: Option<u64>,
    },
    /// Gets the current proposal creation policy for this
    /// module.
    #[returns(ProposalCreationPolicy)]
    ProposalCreationPolicy {},
    /// Lists all of the consumers of proposal hooks for this module.
    #[returns(HooksResponse)]
    ProposalHooks {},
    /// Lists all of the consumers of vote hooks for this
    /// module.
    #[returns(HooksResponse)]
    VoteHooks {},
}

#[cw_serde]
pub struct VoteMsg {
    pub proposal_id: u64,
    pub vote: MultipleChoiceVote,
}

#[cw_serde]
pub enum MigrateMsg {
    FromV1 {
        /// This field was not present in DAO DAO v1. To migrate, a
        /// value must be specified.
        ///
        /// If set to true proposals will be closed if their execution
        /// fails. Otherwise, proposals will remain open after execution
        /// failure. For example, with this enabled a proposal to send 5
        /// tokens out of a DAO's reserve with 4 tokens would be closed when
        /// it is executed. With this disabled, that same proposal would
        /// remain open until the DAO's reserve was large enough for it to be
        /// executed.
        close_proposal_on_execution_failure: bool,
        /// This field was not present in DAO DAO v1. To migrate, a
        /// value must be specified.
        ///
        /// This contains information about how a pre-propose module may be configured.
        /// If set to "AnyoneMayPropose", there will be no pre-propose module and consequently,
        /// no deposit or membership checks when submitting a proposal. The "ModuleMayPropose"
        /// option allows for instantiating a prepropose module which will handle deposit verification and return logic.
        pre_propose_info: PreProposeInfo,
    },
    FromCompatible {},
}

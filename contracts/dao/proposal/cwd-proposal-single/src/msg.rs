use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CosmosMsg};
use cw_utils::Duration;
use cwd_interface::voting::InfoResponse;
// use neutron_sdk::bindings::msg::NeutronMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cwd_core::msg::ProposalType;
use cwd_macros::{info_query, proposal_module_query};
use cwd_voting::{pre_propose::PreProposeInfo, threshold::Threshold, voting::Vote};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The threshold a proposal must reach to complete.
    pub threshold: Threshold,
    /// The default maximum amount of time a proposal may be voted on
    /// before expiring.
    pub max_voting_period: Duration,
    /// The minimum amount of time a proposal must be open before
    /// passing. A proposal may fail before this amount of time has
    /// elapsed, but it will not pass. This can be useful for
    /// preventing governance attacks wherein an attacker acquires a
    /// large number of tokens and forces a proposal through.
    pub min_voting_period: Option<Duration>,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Creates a proposal in the module.
    Propose {
        /// The title of the proposal.
        title: String,
        /// A description of the proposal.
        description: String,
        /// The messages that should be executed in response to this
        /// proposal passing.
        msgs: Vec<CosmosMsg<ProposalType>>,
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
        vote: Vote,
    },
    /// Causes the messages associated with a passed proposal to be
    /// executed by the DAO.
    Execute {
        /// The ID of the proposal to execute.
        proposal_id: u64,
    },
    /* 
    /// Closes a proposal that has failed (either not passed or timed
    /// out). If applicable this will cause the proposal deposit
    /// associated wth said proposal to be returned.
    Close {
        /// The ID of the proposal to close.
        proposal_id: u64,
    },
    /// Updates the governance module's config.
    UpdateConfig {
        /// The new proposal passing threshold. This will only apply
        /// to proposals created after the config update.
        threshold: Threshold,
        /// The default maximum amount of time a proposal may be voted
        /// on before expiring. This will only apply to proposals
        /// created after the config update.
        max_voting_period: Duration,
        /// The minimum amount of time a proposal must be open before
        /// passing. A proposal may fail before this amount of time has
        /// elapsed, but it will not pass. This can be useful for
        /// preventing governance attacks wherein an attacker acquires a
        /// large number of tokens and forces a proposal through.
        min_voting_period: Option<Duration>,
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
    UpdatePreProposeInfo { info: PreProposeInfo },
    /// Adds an address as a consumer of proposal hooks. Consumers of
    /// proposal hooks have hook messages executed on them whenever
    /// the status of a proposal changes or a proposal is created. If
    /// a consumer contract errors when handling a hook message it
    /// will be removed from the list of consumers.
    AddProposalHook { address: String },
    /// Removes a consumer of proposal hooks.
    RemoveProposalHook { address: String },
    /// Adds an address as a consumer of vote hooks. Consumers of vote
    /// hooks have hook messages executed on them whenever the a vote
    /// is cast. If a consumer contract errors when handling a hook
    /// message it will be removed from the list of consumers.
    AddVoteHook { address: String },
    /// Removed a consumer of vote hooks.
    RemoveVoteHook { address: String },*/
}

#[proposal_module_query]
#[info_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Gets the governance module's config. Returns `state::Config`.
    #[returns(crate::state::Config)]
    Config {},
    /// Gets information about a proposal. Returns
    /// `proposals::Proposal`.
    #[returns(crate::query::ProposalResponse)]
    Proposal { proposal_id: u64 },
    /// Lists all the proposals that have been cast in this
    /// module. Returns `query::ProposalListResponse`.
    #[returns(crate::query::ProposalListResponse)]
    ListProposals {
        /// The proposal ID to start listing proposals after. For
        /// example, if this is set to 2 proposals with IDs 3 and
        /// higher will be returned.
        start_after: Option<u64>,
        /// The maximum number of proposals to return as part of this
        /// query. If no limit is set a max of 30 proposals will be
        /// returned.
        limit: Option<u64>,
    },
    /// Lists all of the proposals that have been cast in this module
    /// in descending order of proposal ID. Returns
    /// `query::ProposalListResponse`.
    #[returns(crate::query::ProposalListResponse)]
    ReverseProposals {
        /// The proposal ID to start listing proposals before. For
        /// example, if this is set to 6 proposals with IDs 5 and
        /// lower will be returned.
        start_before: Option<u64>,
        /// The maximum number of proposals to return as part of this
        /// query. If no limit is set a max of 30 proposals will be
        /// returned.
        limit: Option<u64>,
    },
    /// Returns the number of proposals that have been created in this
    /// module.
    #[returns(u64)]
    ProposalCount {},
    /// Returns a voters position on a proposal. Returns
    /// `query::VoteResponse`.
    #[returns(crate::query::VoteResponse)]
    GetVote { proposal_id: u64, voter: String },
    /// Lists all of the votes that have been cast on a
    /// proposal. Returns `VoteListResponse`.
    #[returns(crate::query::VoteListResponse)]
    ListVotes {
        /// The proposal to list the votes of.
        proposal_id: u64,
        /// The voter to start listing votes after. Ordering is done
        /// alphabetically.
        start_after: Option<String>,
        /// The maximum number of votes to return in response to this
        /// query. If no limit is specified a max of 30 are returned.
        limit: Option<u64>,
    },
    /// Gets the current proposal creation policy for this
    /// module. Returns `voting::pre_propose::ProposalCreationPolicy`.
    #[returns(cwd_voting::pre_propose::ProposalCreationPolicy)]
    ProposalCreationPolicy {},
    /// Lists all of the consumers of proposal hooks for this module.
    #[returns(cwd_hooks::HooksResponse)]
    ProposalHooks {},
    /// Lists all of the consumers of vote hooks for this
    /// module. Returns cwd_hooks::HooksResponse.
    #[returns(cwd_hooks::HooksResponse)]
    VoteHooks {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

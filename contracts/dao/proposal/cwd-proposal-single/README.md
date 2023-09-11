# cwd-proposal-single

A proposal module for a ADAO which supports simple "yes", "no",
"abstain" voting. Proposals may have associated messages which will be
executed by the core module upon the proposal being passed and
executed.

## Proposal deposits

Proposal deposits for this module are handled by the
[`cwd-pre-propose-single`](../../pre-propose/cwd-pre-propose-single)
contract.

## Hooks

This module supports hooks for voting and proposal status changes. One
may register a contract to receive these hooks with the `AddVoteHook`
and `AddProposalHook` methods. Upon registration the contract will
receive messages whenever a vote is cast and a proposal's status
changes (for example, when the proposal passes).

The format for these hook messages can be located in the
`proposal-hooks` and `vote-hooks` packages located in
`packages/proposal-hooks` and `packages/vote-hooks` respectively.

To stop an invalid hook receiver from locking the proposal module
receivers will be removed from the hook list if they error when
handling a hook.

## Messages
```
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
    RemoveVoteHook { address: String }
}
```


## Storage
```
/// The current top level config for the module. 
pub const CONFIG: Item<Config> = Item::new("config_v2");
/// The number of proposals that have been created.
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");
/// It will store the proposal mapping with proposal id
pub const PROPOSALS: Map<u64, SingleChoiceProposal> = Map::new("proposals");

/// It will store the vote info and voting power, Is a collective power considering all chains this proposal will apply to(It will check all fundInfo in vec in proposal type).
pub const BALLOTS: Map<(u64, Addr), Ballot> = Map::new("ballots");
/// Consumers of proposal state change hooks.
pub const PROPOSAL_HOOKS: Hooks = Hooks::new("proposal_hooks");
/// Consumers of vote hooks.
pub const VOTE_HOOKS: Hooks = Hooks::new("vote_hooks");
/// The address of the pre-propose module associated with this
/// proposal module (if any).
pub const CREATION_POLICY: Item<ProposalCreationPolicy> = Item::new("creation_policy");

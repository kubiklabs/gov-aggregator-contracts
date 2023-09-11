# cwd-core

This contract is the core module for Accelerator DAO. It handles
management of voting power, proposal modules ica and icq helper modules and executes messages.

## storage

It has all the storage facility as dao-dao-core. Other than that it has
   ` pub const CHAIN_STAKE: Map<String, u8> = Map::new("chain_stake");`
It stores the chain supported by dao and it's stake amount all the supported chains

`pub const ICA_HELPER: Item<Addr> = Item::new("ica_helper_contract");`
`pub const ICQ_HELPER: Item<Addr> = Item::new("icq_helper_contract");`
core contract instantiates the ICA and ICQ helper contract and their addresses get stored in these Items.

`pub const CONTRACT_REGISTRY: Item<Addr> = Item::new("contract_registry");`
We pass the contract registry address while instantiating core contract. All the supported chains is stored in this contract_registry contract.

/// The time the DAO will unpause. Here be dragons: this is not set if
/// the DAO has never been paused.
pub const PAUSED: Item<Expiration> = Item::new("paused");

/// The voting module associated with this contract(instantiated by core itself).
pub const VOTING_REGISTRY_MODULE: Item<Addr> = Item::new("voting_module");

/// The proposal modules associated with this contract(instantiated by core itself).
pub const PROPOSAL_MODULES: Map<Addr, ProposalModule> = Map::new("proposal_modules");

/// The count of active proposal modules associated with this contract.
pub const ACTIVE_PROPOSAL_MODULE_COUNT: Item<u32> = Item::new("active_proposal_module_count");

/// The count of total proposal modules associated with this contract.
pub const TOTAL_PROPOSAL_MODULE_COUNT: Item<u32> = Item::new("total_proposal_module_count");

// General purpose KV store for DAO associated state.
pub const ITEMS: Map<String, String> = Map::new("items");


## Messages

All the messages include in this contract is same as DAO_DAO core contract.The messages and the description is written below
```
pub enum ExecuteMsg {
    /// Callable by proposal modules. The DAO will execute the
    /// messages in the hook in order.Only ProposalType cosmosMsg will be executed by ExecuteProposeHook
    ExecuteProposalHook { msgs: Vec<CosmosMsg<ProposalType>> }``,
    /// Pauses the DAO for a set duration.
    /// When paused the DAO is unable to execute proposals
    Pause { duration: Duration },
    /// Removes an item from the governance contract's item map.
    RemoveItem { key: String },
    /// Adds an item to the governance contract's item map. If the
    /// item already exists the existing value is overriden. If the
    /// item does not exist a new item is added.
    SetItem { key: String, addr: String },
    /// Callable by the core contract. Replaces the current
    /// governance contract config with the provided config.
    UpdateConfig { config: Config },
    /// Updates the governance contract's governance modules. Module
    /// instantiate info in `to_add` is used to create new modules and
    /// install them.
    UpdateProposalModules {
        // NOTE: the pre-propose-base package depends on it being the
        // case that the core module instantiates its proposal module.
        to_add: Vec<ModuleInstantiateInfo>,
        to_disable: Vec<String>,
    },
    /// Callable by the core contract. Replaces the current
    /// voting module with a new one instantiated by the governance
    /// contract.
    UpdateVotingModule { module: ModuleInstantiateInfo },
    // Update the core module to add/remove SubDAOs and their charters
    // UpdateSubDaos {
    //     to_add: Vec<SubDao>,
    //     to_remove: Vec<String>,
    // },
}
```
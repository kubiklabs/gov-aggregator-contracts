use crate::ContractError;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use cw_utils::Expiration;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Top level config type for core module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    /// The name of the contract.
    pub name: String,
    /// A description of the contract.
    pub description: String,
    /// The URI for the DAO as defined by the DAOstar standard
    /// https://daostar.one/EIP
    pub dao_uri: Option<String>,
}

impl Config {
    /// checks whether the config fields are valid.
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.name.is_empty() {
            return Err(ContractError::NameIsEmpty {});
        };
        if self.description.is_empty() {
            return Err(ContractError::DescriptionIsEmpty {});
        };
        if let Some(dao_uri) = self.dao_uri.clone() {
            if dao_uri.is_empty() {
                return Err(ContractError::DaoUriIsEmpty {});
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// Top level type describing a proposal module.
pub struct ProposalModule {
    /// The address of the proposal module.
    pub address: Addr,
    /// The URL prefix of this proposal module as derived from the module ID.
    /// Prefixes are mapped to letters, e.g. 0 is 'A', and 26 is 'AA'.
    pub prefix: String,
    /// The status of the proposal module, e.g. 'Active' or 'Disabled.'
    pub status: ProposalModuleStatus,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// For receiving chain and their stake value.
pub struct ChainStakeInfo {
    /// chain-id of the chain which got registered in the DAO.
    pub chain_id: String,
    /// stake is the value of chain-id which hold in the list of chain
    pub stake: u8
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
/// The status of a proposal module.
pub enum ProposalModuleStatus {
    Enabled,
    Disabled,
}

/// The current configuration of the module.
pub const CONFIG: Item<Config> = Item::new("config_v2");

/// This will store the 
pub const CHAIN_STAKE: Map<String, u8> = Map::new("chain_stake");
pub const ICA_HELPER: Item<Addr> = Item::new("icq_helper_contract");
pub const CONTRACT_REGISTRY: Item<Addr> = Item::new("contract_registry");

/// The time the DAO will unpause. Here be dragons: this is not set if
/// the DAO has never been paused.
pub const PAUSED: Item<Expiration> = Item::new("paused");

/// The voting module associated with this contract.
pub const VOTING_REGISTRY_MODULE: Item<Addr> = Item::new("voting_module");

/// The proposal modules associated with this contract.
/// When we change the data format of this map, we update the key (previously "proposal_modules")
/// to create a new namespace for the changed state.
pub const PROPOSAL_MODULES: Map<Addr, ProposalModule> = Map::new("proposal_modules_v2");

/// The count of active proposal modules associated with this contract.
pub const ACTIVE_PROPOSAL_MODULE_COUNT: Item<u32> = Item::new("active_proposal_module_count");

/// The count of total proposal modules associated with this contract.
pub const TOTAL_PROPOSAL_MODULE_COUNT: Item<u32> = Item::new("total_proposal_module_count");

// General purpose KV store for DAO associated state.
pub const ITEMS: Map<String, String> = Map::new("items");

/// List of SubDAOs associated to this DAO. Each SubDAO has an optional charter.
// pub const SUBDAO_LIST: Map<&Addr, Option<String>> = Map::new("sub_daos");

#[cfg(test)]
mod tests {
    use super::Config;
    use crate::error::ContractError;

    #[test]
    fn test_config_validate() {
        let cfg_ok = Config {
            name: String::from("name"),
            description: String::from("description"),
            dao_uri: Some(String::from("www.dao.org")),
        };
        assert_eq!(cfg_ok.validate(), Ok(()));
        let cfg_ok_none_uri = Config {
            name: String::from("name"),
            description: String::from("description"),
            dao_uri: None,
        };
        assert_eq!(cfg_ok_none_uri.validate(), Ok(()));

        let cfg_empty_name = Config {
            name: String::from(""),
            description: String::from("description"),
            dao_uri: Some(String::from("www.dao.org")),
        };
        assert_eq!(
            cfg_empty_name.validate(),
            Err(ContractError::NameIsEmpty {})
        );

        let cfg_empty_description = Config {
            name: String::from("name"),
            description: String::from(""),
            dao_uri: Some(String::from("www.dao.org")),
        };
        assert_eq!(
            cfg_empty_description.validate(),
            Err(ContractError::DescriptionIsEmpty {})
        );

        let cfg_empty_dao_uri = Config {
            name: String::from("name"),
            description: String::from("description"),
            dao_uri: Some(String::from("")),
        };
        assert_eq!(
            cfg_empty_dao_uri.validate(),
            Err(ContractError::DaoUriIsEmpty {})
        );
    }
}

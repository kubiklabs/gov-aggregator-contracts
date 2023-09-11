use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::BasicApp;
use cwd_core::state::{ProposalModule, ProposalModuleStatus};
use cwd_hooks::HooksResponse;
use cwd_pre_propose_multiple as cppm;
use cwd_voting::pre_propose::ProposalCreationPolicy;
use neutron_sdk::bindings::msg::NeutronMsg;

use crate::{
    msg::QueryMsg,
    query::{ProposalListResponse, ProposalResponse},
    state::Config,
};

pub fn query_deposit_config_and_pre_propose_module(
    app: &BasicApp<NeutronMsg>,
    proposal_multiple: &Addr,
) -> (cppm::Config, Addr) {
    let proposal_creation_policy = query_creation_policy(app, proposal_multiple);

    if let ProposalCreationPolicy::Module { addr: module_addr } = proposal_creation_policy {
        let deposit_config = query_pre_proposal_multiple_config(app, &module_addr);

        (deposit_config, module_addr)
    } else {
        panic!("no pre-propose module.")
    }
}

pub fn query_proposal_config(app: &BasicApp<NeutronMsg>, proposal_multiple: &Addr) -> Config {
    app.wrap()
        .query_wasm_smart(proposal_multiple, &QueryMsg::Config {})
        .unwrap()
}

pub fn query_creation_policy(
    app: &BasicApp<NeutronMsg>,
    proposal_multiple: &Addr,
) -> ProposalCreationPolicy {
    app.wrap()
        .query_wasm_smart(proposal_multiple, &QueryMsg::ProposalCreationPolicy {})
        .unwrap()
}

pub fn query_pre_proposal_multiple_config(
    app: &BasicApp<NeutronMsg>,
    pre_propose: &Addr,
) -> cppm::Config {
    app.wrap()
        .query_wasm_smart(pre_propose, &cppm::QueryMsg::Config {})
        .unwrap()
}

pub fn query_multiple_proposal_module(app: &BasicApp<NeutronMsg>, core_addr: &Addr) -> Addr {
    let modules: Vec<ProposalModule> = app
        .wrap()
        .query_wasm_smart(
            core_addr,
            &cwd_core::msg::QueryMsg::ProposalModules {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();

    // Filter out disabled modules.
    let modules = modules
        .into_iter()
        .filter(|module| module.status == ProposalModuleStatus::Enabled)
        .collect::<Vec<_>>();

    assert_eq!(
        modules.len(),
        1,
        "wrong proposal module count. expected 1, got {}",
        modules.len()
    );

    modules.into_iter().next().unwrap().address
}

pub fn query_list_proposals(
    app: &BasicApp<NeutronMsg>,
    proposal_multiple: &Addr,
    start_after: Option<u64>,
    limit: Option<u64>,
) -> ProposalListResponse {
    app.wrap()
        .query_wasm_smart(
            proposal_multiple,
            &QueryMsg::ListProposals { start_after, limit },
        )
        .unwrap()
}

pub fn query_proposal_hooks(app: &BasicApp<NeutronMsg>, proposal_multiple: &Addr) -> HooksResponse {
    app.wrap()
        .query_wasm_smart(proposal_multiple, &QueryMsg::ProposalHooks {})
        .unwrap()
}

pub fn query_vote_hooks(app: &BasicApp<NeutronMsg>, proposal_multiple: &Addr) -> HooksResponse {
    app.wrap()
        .query_wasm_smart(proposal_multiple, &QueryMsg::VoteHooks {})
        .unwrap()
}

pub fn query_list_proposals_reverse(
    app: &BasicApp<NeutronMsg>,
    proposal_multiple: &Addr,
    start_before: Option<u64>,
    limit: Option<u64>,
) -> ProposalListResponse {
    app.wrap()
        .query_wasm_smart(
            proposal_multiple,
            &QueryMsg::ReverseProposals {
                start_before,
                limit,
            },
        )
        .unwrap()
}

pub fn query_dao_token(app: &BasicApp<NeutronMsg>, core_addr: &Addr) -> Addr {
    let voting_module = query_voting_module(app, core_addr);
    app.wrap()
        .query_wasm_smart(
            voting_module,
            &cwd_interface::voting::Query::TokenContract {},
        )
        .unwrap()
}

pub fn query_voting_module(app: &BasicApp<NeutronMsg>, core_addr: &Addr) -> Addr {
    app.wrap()
        .query_wasm_smart(core_addr, &cwd_core::msg::QueryMsg::VotingModule {})
        .unwrap()
}

pub fn query_balance_cw20<T: Into<String>, U: Into<String>>(
    app: &BasicApp<NeutronMsg>,
    contract_addr: T,
    address: U,
) -> Uint128 {
    let msg = cw20::Cw20QueryMsg::Balance {
        address: address.into(),
    };
    let result: cw20::BalanceResponse = app.wrap().query_wasm_smart(contract_addr, &msg).unwrap();
    result.balance
}

pub fn query_balance_native(app: &BasicApp<NeutronMsg>, who: &str, denom: &str) -> Uint128 {
    let res = app.wrap().query_balance(who, denom).unwrap();
    res.amount
}

pub fn query_proposal(
    app: &BasicApp<NeutronMsg>,
    proposal_multiple: &Addr,
    id: u64,
) -> ProposalResponse {
    app.wrap()
        .query_wasm_smart(proposal_multiple, &QueryMsg::Proposal { proposal_id: id })
        .unwrap()
}

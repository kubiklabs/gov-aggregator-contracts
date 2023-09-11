use cosmwasm_std::{coins, Addr, Coin};
use cw_multi_test::{BankSudo, BasicApp, Executor};
use neutron_sdk::bindings::msg::NeutronMsg;

use cw_denom::CheckedDenom;
use cwd_pre_propose_multiple as cppm;
use cwd_voting::{
    deposit::CheckedDepositInfo, multiple_choice::MultipleChoiceOptions,
    pre_propose::ProposalCreationPolicy,
};

use crate::{
    msg::{ExecuteMsg, QueryMsg},
    query::ProposalResponse,
    testing::queries::{query_creation_policy, query_pre_proposal_multiple_config},
};

// Creates a proposal then checks that the proposal was created with
// the specified messages and returns the ID of the proposal.
//
// This expects that the proposer already has the needed tokens to pay
// the deposit.
pub fn make_proposal(
    app: &mut BasicApp<NeutronMsg>,
    proposal_multiple: &Addr,
    proposer: &str,
    choices: MultipleChoiceOptions,
) -> u64 {
    let proposal_creation_policy = query_creation_policy(app, proposal_multiple);

    // Collect the funding.
    let funds = match proposal_creation_policy {
        ProposalCreationPolicy::Anyone {} => vec![],
        ProposalCreationPolicy::Module {
            addr: ref pre_propose,
        } => {
            let deposit_config = query_pre_proposal_multiple_config(app, pre_propose);
            match deposit_config.deposit_info {
                Some(CheckedDepositInfo {
                    denom,
                    amount,
                    refund_policy: _,
                }) => match denom {
                    CheckedDenom::Native(denom) => coins(amount.u128(), denom),
                    CheckedDenom::Cw20(addr) => {
                        // Give an allowance, no funds.
                        app.execute_contract(
                            Addr::unchecked(proposer),
                            addr,
                            &cw20::Cw20ExecuteMsg::IncreaseAllowance {
                                spender: pre_propose.to_string(),
                                amount,
                                expires: None,
                            },
                            &[],
                        )
                        .unwrap();
                        vec![]
                    }
                },
                None => vec![],
            }
        }
    };

    // Make the proposal.
    let res = match proposal_creation_policy {
        ProposalCreationPolicy::Anyone {} => app
            .execute_contract(
                Addr::unchecked(proposer),
                proposal_multiple.clone(),
                &ExecuteMsg::Propose {
                    title: "title".to_string(),
                    description: "description".to_string(),
                    choices,
                    proposer: None,
                },
                &[],
            )
            .unwrap(),
        ProposalCreationPolicy::Module { addr } => app
            .execute_contract(
                Addr::unchecked(proposer),
                addr,
                &cppm::ExecuteMsg::Propose {
                    msg: cppm::ProposeMessage::Propose {
                        title: "title".to_string(),
                        description: "description".to_string(),
                        choices,
                    },
                },
                &funds,
            )
            .unwrap(),
    };

    // The new proposal hook is the last message that fires in
    // this process so we get the proposal ID from it's
    // attributes. We could do this by looking at the proposal
    // creation attributes but this changes relative position
    // depending on if a cw20 or native deposit is being used.
    let attrs = res.custom_attrs(res.events.len() - 1);
    let id = attrs[attrs.len() - 1]
        .value
        .parse()
        // If the proposal creation policy doesn't involve a
        // pre-propose module, no hook so we do it manaually.
        .unwrap_or_else(|_| res.custom_attrs(1)[2].value.parse().unwrap());

    // Check that the proposal was created as expected.
    let proposal: ProposalResponse = app
        .wrap()
        .query_wasm_smart(proposal_multiple, &QueryMsg::Proposal { proposal_id: id })
        .unwrap();

    assert_eq!(proposal.proposal.proposer, Addr::unchecked(proposer));
    assert_eq!(proposal.proposal.title, "title".to_string());
    assert_eq!(proposal.proposal.description, "description".to_string());

    id
}

pub(crate) fn mint_natives(app: &mut BasicApp<NeutronMsg>, receiver: &str, amount: Vec<Coin>) {
    app.sudo(cw_multi_test::SudoMsg::Bank(BankSudo::Mint {
        to_address: receiver.to_string(),
        amount,
    }))
    .unwrap();
}

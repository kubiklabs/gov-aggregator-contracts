import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";

import { CwdCoreContract } from "../artifacts/typescript_schema/CwdCoreContract";
import { IcqHelperContract } from "../artifacts/typescript_schema/IcqHelperContract";
import { IcaHelperContract } from "../artifacts/typescript_schema/IcaHelperContract";
import { ChainRegistryContract } from "../artifacts/typescript_schema/ChainRegistryContract";
import { CwdProposalSingleContract } from "../artifacts/typescript_schema/CwdProposalSingleContract";
import { CwdPreProposeSingleContract } from "../artifacts/typescript_schema/CwdPreProposeSingleContract";

function sleep(seconds: number) {
  console.log("Sleeping for " + seconds + " seconds");
  return new Promise(resolve => setTimeout(resolve, seconds*1000));
}

async function run () {
  const runTs = String(new Date());
  const nativeDenom = "untrn";  // Neutron fee token
  const atomDenom = "uatom";    // Cosmos hub fee token
  const osmoDenom = "uosmo";    // Osmosis fee token
  const junoDenom = "ujuno";    // Juno fee token
  const contract_owner = await getAccountByName("account_0");

  const atomConnectionId = "connection-0";
  const osmoConnectionId = "connection-2";
  const junoConnectionId = "connection-1";

  const atomChainId = "gaia-test-2";
  const osmoChainId = "osmo-test-4";
  const junoChainId = "juno-test-3";

  const DaoName = "All Chains DAO";
  const DaoDescription = "One DAO for all the chains connected to Neutron";
  const chainsList = [
    {
      chain_id: atomChainId,
      connection_id: atomConnectionId,
      stake: 40,
    },
    {
      chain_id: osmoChainId,
      connection_id: osmoConnectionId,
      stake: 30,
    },
    {
      chain_id: junoChainId,
      connection_id: junoConnectionId,
      stake: 30,
    },
  ];

  const interchainAccountAtom = "remote_account_atom";
  const interchainAccountOsmo = "remote_account_osmo";
  const interchainAccountJuno = "remote_account_juno";

  const remoteValidatorAtom = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";
  const remoteValidatorOsmo = "osmovaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zkc9nsx4";
  const remoteValidatorJuno = "junovaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zkrxahm9";

  console.log("admin account fetched successfully");

  const dao_core = new CwdCoreContract();
  await dao_core.setupClient();
  const chain_registry = new ChainRegistryContract();
  await chain_registry.setupClient();

  const pre_propose = new CwdPreProposeSingleContract();
  await pre_propose.setupClient();
  const proposal = new CwdProposalSingleContract();
  await proposal.setupClient();

  const icq_helper = new IcqHelperContract();
  await icq_helper.setupClient();
  const ica_helper = new IcaHelperContract();
  await ica_helper.setupClient();

  // Deploy Chain Registry
  const deploy_chain_registry = await chain_registry.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_chain_registry);

  // // Init Chain Registry
  // const init_chain_registry = await chain_registry.instantiate(
  //   {},
  //   `Chain Registry contract ${runTs}`,
  //   contract_owner
  // );
  // console.log(chalk.cyan("Response: "), init_chain_registry);

  // // Update chain registry for gaia
  // const update_gaia_connection = await chain_registry.updateChainInfo(
  //   {
  //     account: contract_owner,
  //   },
  //   {
  //     connectionId: "connection0",
  //     remoteChain: "test2",
  //   },
  // );
  // console.log(chalk.cyan("Response: "), update_gaia_connection);

  // Deploy Cwd Core
  const deploy_dao_core = await dao_core.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_dao_core);

  // Deploy Pre-Propose
  const deploy_pre_propose = await pre_propose.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_pre_propose);

  // Deploy Proposal
  const deploy_proposal = await proposal.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_proposal);

  // Deploy ICQ helper
  const deploy_icq_helper = await icq_helper.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_icq_helper);

  // Deploy ICA helper
  const deploy_ica_helper = await ica_helper.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_ica_helper);

  // Init Cwd Core with CodeId of Proposal, ICQ helper, ICA helper
  const core_contract_info = await dao_core.instantiate(
    {
      name: DaoName,
      dao_uri: null,
      description: DaoDescription,
      chain_list: chainsList,
      contract_registry: chain_registry.contractAddress,
      initial_items: null,
      icq_helper_module_instantiate_info: {
        admin: null,
        code_id: icq_helper.codeId,
        label: `ICQ Helper Contract ${runTs}`,
        msg: Buffer.from(JSON.stringify({})).toString("base64"),
      },
      ica_helper_module_instantiate_info: {
        admin: null,
        code_id: ica_helper.codeId,
        label: `ICA Helper Contract ${runTs}`,
        msg: Buffer.from(JSON.stringify({})).toString("base64"),
      },
      proposal_modules_instantiate_info: [
        {
          admin: null,
          code_id: proposal.codeId,
          label: `Proposal Contract ${runTs}`,
          msg: Buffer.from(JSON.stringify({
            allow_revoting: true,
            close_proposal_on_execution_failure: true,
            max_voting_period: {
              time: 100_000,  // 100,000 blocks
            },
            min_voting_period: {
              time: 10_000,  // 10,000 blocks
            },
            pre_propose_info: {
              anyone_may_propose: {},
            },
            threshold: {
              absolute_percentage: {
                percentage: {
                  percent: "0.6",  // 60%
                }
              },
            },
          })).toString("base64"),
        }
      ],
    },
    `DAO Core contract ${runTs}`,
    contract_owner,
    undefined,
    { // custom fees
      amount: [{ amount: "25000", denom: "untrn" }],
      gas: "2000000",
    }
  );
  console.log(chalk.cyan("Response: "), core_contract_info);

  console.log("All contract instance created successfully");

  const core_state = await dao_core.dumpState();
  console.log("core_state", core_state);
/*
  // Update the contractAddress in Proposal contract
  proposal.instantiatedWithAddress(core_state.proposal_modules[0].address);
  console.log(chalk.cyan("Updated Proposal module address: "), proposal.contractAddress);

  // Update the contractAddress in ICA helper
  ica_helper.instantiatedWithAddress(core_state.ica_helper);
  console.log(chalk.cyan("Updated ICA helper address: "), ica_helper.contractAddress);

  // Register account on Gaia chain
  const gaia_register_res = await ica_helper.register(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "125000", denom: nativeDenom }],
        gas: "500000",
      },
      transferAmount: [ // fee for doing all following ICA txns, a bit more than min_fee
        { amount: "500000", denom: nativeDenom }
      ]
    },
    {
      connectionId: atomConnectionId,
      interchainAccountId: interchainAccountAtom,
    }
  );
  console.log(chalk.cyan("Response: "), gaia_register_res);

  // Register account on Osmo chain
  const osmo_register_res = await ica_helper.register(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "125000", denom: nativeDenom }],
        gas: "500000",
      },
      transferAmount: [ // fee for doing all following ICA txns, a bit more than min_fee
        { amount: "500000", denom: nativeDenom }
      ]
    },
    {
      connectionId: osmoConnectionId,
      interchainAccountId: interchainAccountOsmo,
    }
  );
  console.log(chalk.cyan("Response: "), osmo_register_res);

  // Register account on Juno chain
  const juno_register_res = await ica_helper.register(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "125000", denom: nativeDenom }],
        gas: "500000",
      },
      transferAmount: [ // fee for doing all following ICA txns, a bit more than min_fee
        { amount: "500000", denom: nativeDenom }
      ]
    },
    {
      connectionId: junoConnectionId,
      interchainAccountId: interchainAccountJuno,
    }
  );
  console.log(chalk.cyan("Response: "), juno_register_res);

  await sleep(10);  // wait for addr to be created

  // Query interchain address Gaia
  const atomAccountInfo = await ica_helper.interchainAccountAddress({
    connectionId: atomConnectionId,
    interchainAccountId: interchainAccountAtom,
  });
  console.log(chalk.cyan("Response: "), "account info: ", JSON.stringify(atomAccountInfo, null, 2));

  // Query more account data Gaia
  const atomMoreAccountInfo = await ica_helper.interchainAccountAddressFromContract({
    interchainAccountId: interchainAccountAtom,
  });
  console.log(chalk.cyan("Response: "), "more account info: ", JSON.stringify(atomMoreAccountInfo, null, 2));

  // Query interchain address Osmo
  const osmoAccountInfo = await ica_helper.interchainAccountAddress({
    connectionId: osmoConnectionId,
    interchainAccountId: interchainAccountOsmo,
  });
  console.log(chalk.cyan("Response: "), "account info: ", JSON.stringify(osmoAccountInfo, null, 2));

  // Query more account data Osmo
  const osmoMoreAccountInfo = await ica_helper.interchainAccountAddressFromContract({
    interchainAccountId: interchainAccountOsmo,
  });
  console.log(chalk.cyan("Response: "), "more account info: ", JSON.stringify(osmoMoreAccountInfo, null, 2));

  // Query interchain address Juno
  const junoAccountInfo = await ica_helper.interchainAccountAddress({
    connectionId: junoConnectionId,
    interchainAccountId: interchainAccountJuno,
  });
  console.log(chalk.cyan("Response: "), "account info: ", JSON.stringify(junoAccountInfo, null, 2));

  // Query more account data Juno
  const junoMoreAccountInfo = await ica_helper.interchainAccountAddressFromContract({
    interchainAccountId: interchainAccountJuno,
  });
  console.log(chalk.cyan("Response: "), "more account info: ", JSON.stringify(junoMoreAccountInfo, null, 2));

  await sleep(150);  // wait for addr to be funded manually
*/
  /*
  const text_prop_create_gaia = await proposal.propose(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
    },
    {
      title: "This is first proposal on gaia",
      description: "This is a text proposal with proposefund custom message on gaia",
      msgs: [
        {
          "custom": {
            "propose_funds": {
              "demand_info": [
                {
                  "chain_id": "gaia-test-2",
                  "amount": "2000000",
                  "denom": "uatom",
                  "interchain_account_id": interchainAccountAtom
                }
              ]
            }
          }
        }
      ],
      proposer: null,  // null for all allowed, addr for pre-propose module
    }
  );

  console.log("proposal created for gaia, ",text_prop_create_gaia)

  const text_prop_create_juno = await proposal.propose(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
    },
    {
      title: "This is first proposal on Juno",
      description: "This is a text proposal with proposefund custom message on Juno",
      msgs: [
        {
          "custom": {
            "propose_funds": {
              "demand_info": [
                {
                  "chain_id": "juno-test-3",
                  "amount": "3000000",
                  "denom": "ujuno",
                  "interchain_account_id": interchainAccountJuno
                }
              ]
            }
          }
        }
      ],
      proposer: null,  // null for all allowed, addr for pre-propose module
    }
  );

  console.log("proposal created for gaia, ",text_prop_create_juno)

  const text_prop_create_osmo = await proposal.propose(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
    },
    {
      title: "This is first proposal on Osmo",
      description: "This is a text proposal with proposefund custom message on Osmo",
      msgs: [
        {
          "custom": {
            "propose_funds": {
              "demand_info": [
                {
                  "chain_id": "osmo-test-4",
                  "amount": "4000000",
                  "denom": "uosmo",
                  "interchain_account_id": interchainAccountOsmo
                }
              ]
            }
          }
        }
      ],
      proposer: null,  // null for all allowed, addr for pre-propose module
    }
  );

  console.log("proposal created for osmo, ",text_prop_create_osmo)
*/
  // // propose a text proposal with [GetFunds] msgs
  // const get_fund_prop_create = await proposal.propose(
  //   {
  //     account: contract_owner,
  //     customFees: {
  //       amount: [{ amount: "75000", denom: nativeDenom }],
  //       gas: "300000",
  //     },
  //   },
  //   {
  //     title: "This is second proposal",
  //     description: "This is a community pool fund proposal",
  //     msgs: [
  //       {
  //         wasm: {
  //           execute: {
  //             contract_addr: ica_helper.contractAddress,
  //             msg: Buffer.from(JSON.stringify({
  //               propose_funds: {
  //                 amount: "1000000" as any,  // 1 atom
  //                 denom: remoteDenom,
  //                 interchain_account_id: interchainAccountName,
  //                 timeout: null,  // in seconds, TODO: confirm it later
  //               }
  //             })).toString("base64"),
  //             funds: [],
  //           }
  //         }
  //       }
  //     ],
  //     proposer: null,  // null for all allowed, addr for pre-propose module
  //   }
  // );
  // console.log(chalk.cyan("Response: "), get_fund_prop_create);

  // proposal on all-chains
  // const text_prop_create_all_chain = await proposal.propose(
  //   {
  //     account: contract_owner,
  //     customFees: {
  //       amount: [{ amount: "75000", denom: nativeDenom }],
  //       gas: "300000",
  //     },
  //   },
  //   {
  //     title: "This is first proposal on Osmo",
  //     description: "This is a text proposal with proposefund custom message on Osmo",
  //     msgs: [
  //       {
  //         "custom": {
  //           "propose_funds": {
  //             "demand_info": [
  //               {
  //                 "chain_id": "gaia-test-2",
  //                 "amount": "2000000",
  //                 "denom": "uatom",
  //                 "interchain_account_id": interchainAccountAtom
  //               }
  //             ]
  //           }
  //         }
  //       },
  //       {
  //         "custom": {
  //           "propose_funds": {
  //             "demand_info": [
  //               {
  //                 "chain_id": "juno-test-3",
  //                 "amount": "3000000",
  //                 "denom": "ujuno",
  //                 "interchain_account_id": interchainAccountJuno
  //               }
  //             ]
  //           }
  //         }
  //       },
  //       {
  //         "custom": {
  //           "propose_funds": {
  //             "demand_info": [
  //               {
  //                 "chain_id": "osmo-test-4",
  //                 "amount": "4000000",
  //                 "denom": "uosmo",
  //                 "interchain_account_id": interchainAccountOsmo
  //               }
  //             ]
  //           }
  //         }
  //       }
  //     ],
  //     proposer: null,  // null for all allowed, addr for pre-propose module
  //   }
  // );

  // console.log("create proposal for asking fund from all chains, ",text_prop_create_all_chain)

  // Query all proposals
  const proposals_list = await proposal.listProposals(
    {
      limit: 10,
      startAfter: null,
    },
  );
  console.log(chalk.cyan("Proposals list: "), proposals_list);

  /*
  // Query first proposal
  const proposals_first = await proposal.proposal(
    {
      proposalId: 1,
    },
  );
  console.log(chalk.cyan("First proposal: "), proposals_first);
  

  // Executing all proposal as no voting is handled yet
  const execute_first = await proposal.execute(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "125000", denom: nativeDenom }],
        gas: "500000",
      },
    },
    {
      proposalId: 1,
    }
  );
  console.log(chalk.cyan("Execute first proposal, for gaia: "), execute_first);

  // Execute the GetFunds proposal
  const execute_second = await proposal.execute(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "125000", denom: nativeDenom }],
        gas: "500000",
      },
    },
    {
      proposalId: 2,
    }
  );
  console.log(chalk.cyan("Execute second proposal,for juno: "), execute_second);

  const execute_third = await proposal.execute(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "125000", denom: nativeDenom }],
        gas: "500000",
      },
    },
    {
      proposalId: 3,
    }
  );
  console.log(chalk.cyan("Execute third proposal,for osmo: "), execute_third);
  */
  const execute_all = await proposal.execute(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "125000", denom: nativeDenom }],
        gas: "1000000",
      },
    },
    {
      proposalId: 4,
    }
  );
  console.log(chalk.cyan("Execute third proposal,for osmo: "), execute_all);
}

module.exports = { default: run };
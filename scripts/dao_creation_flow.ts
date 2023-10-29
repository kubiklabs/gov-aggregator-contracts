import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";

import { CwdCoreContract } from "../artifacts/typescript_schema/CwdCoreContract";
import { IcqHelperContract } from "../artifacts/typescript_schema/IcqHelperContract";
import { IcaHelperContract } from "../artifacts/typescript_schema/IcaHelperContract";
import { ChainRegistryContract } from "../artifacts/typescript_schema/ChainRegistryContract";
import { CwdProposalSingleContract } from "../artifacts/typescript_schema/CwdProposalSingleContract";
import { CwdPreProposeSingleContract } from "../artifacts/typescript_schema/CwdPreProposeSingleContract";

import networkConfig from "./config/localnet.json";

function sleep(seconds: number) {
  console.log("Sleeping for " + seconds + " seconds");
  return new Promise(resolve => setTimeout(resolve, seconds*1000));
}

async function run () {
  const runTs = String(new Date());
  const nativeDenom = "untrn";  // neutron fee token
  const remoteDenom = "uatom";  // cosmos hub fee token
  const contract_owner = await getAccountByName("account_0");

  const connectionId = networkConfig.relayers.gaia.connection_id;
  const interchainAccountName = "remote_account_1";
  const remoteValidatorOne = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";

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
      name: "AccTest",
      dao_uri: null,
      description: "testing DAO",
      chain_list: [
        {
          chain_id: "test2",
          connection_id: "connection-0",
          stake: 100,
        }
      ],
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
      // proposal_modules_instantiate_info: [
      //   {
      //     admin: null,
      //     code_id: proposal.codeId,
      //     label: `Proposal Contract ${runTs}`,
      //     msg: {
      //       allow_revoting: true,
      //       close_proposal_on_execution_failure: true,
      //       max_voting_period: {
      //         time: 100_000,  // 100,000 blocks
      //       },
      //       min_voting_period: {
      //         time: 10_000,  // 10,000 blocks
      //       },
      //       pre_propose_info: {
      //         anyone_may_propose: {},
      //       },
      //       threshold: {
      //         absolute_percentage: {
      //           percentage: {
      //             percent: "0.6",  // 60%
      //           }
      //         },
      //       },
      //     },
      //   }
      // ],
    },
    `DAO Core contract ${runTs}`,
    contract_owner
  );
  console.log(chalk.cyan("Response: "), core_contract_info);

  console.log("All contract instance created successfully");

  const core_state = await dao_core.dumpState();
  console.log("core_state", core_state);

  // // Register account on remote chain
  // const register_res = await staking_contract.register(
  //   {
  //     account: contract_owner,
  //     customFees: {
  //       amount: [{ amount: "75000", denom: nativeDenom }],
  //       gas: "300000",
  //     },
  //   },
  //   {
  //     connectionId: connectionId,
  //     interchainAccountId: interchainAccountName,
  //   }
  // );
  // console.log(chalk.cyan("Response: "), register_res);

  // await sleep(10);  // wait for addr to be created


  // // Query interchain address
  // const accountInfo = await dao_core.interchainAccountAddress({
  //   connectionId: connectionId,
  //   interchainAccountId: interchainAccountName,
  // });
  // console.log(chalk.cyan("Response: "), "account info: ", JSON.stringify(accountInfo, null, 2));

  // // Query more account data
  // const moreAccountInfo = await dao_core.interchainAccountAddressFromContract({
  //   interchainAccountId: interchainAccountName,
  // });
  // console.log(chalk.cyan("Response: "), "more account info: ", JSON.stringify(moreAccountInfo, null, 2));

  // // Delegate 1 atom
  // const stake_claim_res = await dao_core.delegate(
  //   {
  //     account: contract_owner,
  //     customFees: {
  //       amount: [{ amount: "75000", denom: nativeDenom }],
  //       gas: "3000000",
  //     },
  //     transferAmount: [ // fee for doing ICA, should just a bit more than min_fee
  //       { amount: "50000", denom: nativeDenom }
  //     ]
  //   },
  //   {
  //     amount: "1000000" as any,  // 1 atom
  //     denom: remoteDenom,
  //     interchainAccountId: interchainAccountName,
  //     timeout: null,  // in seconds, TODO: confirm it later
  //     validator: remoteValidatorOne,
  //   }
  // );
  // console.log(chalk.cyan("Response: "), stake_claim_res);

  // // const c1 = await staking_contract.info();
  // // console.log("info before deposit: ",c1);
}

module.exports = { default: run };
import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";

import { CwdProposalSingleContract } from "../artifacts/typescript_schema/CwdProposalSingleContract";

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

  const proposal_contract = new CwdProposalSingleContract();
  await proposal_contract.setupClient();

  // Deploy Proposal contract
  const deploy_proposal_contract = await proposal_contract.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_proposal_contract);

  // Init Proposal contract
  const init_proposal_contract = await proposal_contract.instantiate(
    {
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
    },
    `Proposal contract ${runTs}`,
    contract_owner,
  );
  console.log(chalk.cyan("Response: "), init_proposal_contract);

  console.log("All contract instance created successfully");

  // Create a text proposal
  const text_prop_create = await proposal_contract.propose(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
    },
    {
      title: "This is first proposal",
      description: "This is a text proposal",
      msgs: [],
      proposer: null,  // null for all allowed, addr for pre-propose module
    }
  );
  console.log(chalk.cyan("Response: "), text_prop_create);

  // Create a GetFunds proposal
  const get_fund_prop_create = await proposal_contract.propose(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
    },
    {
      title: "This is second proposal",
      description: "This is a community pool fund proposal",
      msgs: [],
      proposer: null,  // null for all allowed, addr for pre-propose module
    }
  );
  console.log(chalk.cyan("Response: "), get_fund_prop_create);

  // Manually update the status of proposals to PASS

  // Execute the proposals

  // Query if proposals have been executed

  // Query if CommunityPoolSpendProposal has been created on remote chain

  // Create a proposal to transfer community pool funds to DAO

  // Execute the proposal

  await sleep(10);  // wait for addr to be created

  // // Query interchain address
  // const accountInfo = await proposal_contract.interchainAccountAddress({
  //   connectionId: connectionId,
  //   interchainAccountId: interchainAccountName,
  // });
  // console.log(chalk.cyan("Response: "), "account info: ", JSON.stringify(accountInfo, null, 2));
}

module.exports = { default: run };
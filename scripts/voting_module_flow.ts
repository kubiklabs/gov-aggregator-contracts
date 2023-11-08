import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";

import { NeutronVotingRegistryContract } from "../artifacts/typescript_schema/NeutronVotingRegistryContract";

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

  const channelId = networkConfig.relayers.gaia.source_channel_id;
  const connectionId = networkConfig.relayers.gaia.connection_id;
  const remoteAccount = "cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw";
  const remoteValidatorOne = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";

  console.log("admin account fetched successfully");

  const voting_registry = new NeutronVotingRegistryContract();
  await voting_registry.setupClient();

  // Deploy Voting registry
  const deploy_voting_registry = await voting_registry.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_voting_registry);

  // Init Voting registry
  const init_voting_registry = await voting_registry.instantiate(
    {
      owner: contract_owner.account.address,
      voting_vaults: [""],
    },
    `Voting registry contract ${runTs}`,
    contract_owner
  );
  console.log(chalk.cyan("Response: "), init_voting_registry);

  console.log("All contract instance created successfully");

  // Register delegations user query
  const register_delegations_user_res = await voting_registry.addVotingVault(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
      transferAmount: [ // fee for doing multiple ICQ queries, should just a bit more than min_fee
        { amount: "2000000", denom: nativeDenom }
      ]
    },
    {
      newVotingVaultContract: "",
    }
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(register_delegations_user_res, null, 2));

  await sleep(10);  // wait for query to be registered

  // Query delegations user
  const remote_delegations_user = await voting_registry.getDelegations({ address: remoteAccount });
  console.log(chalk.cyan("Response: "), JSON.stringify(remote_delegations_user, null, 2));

  await sleep(10);  // wait for query to be registered

  // Query balance
  const remote_balance = await voting_registry.balance({ address: remoteAccount });
  console.log(chalk.cyan("Response: "), JSON.stringify(remote_balance, null, 2));
}

module.exports = { default: run };
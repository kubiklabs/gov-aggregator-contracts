import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";

import { IcqHelperContract } from "../artifacts/typescript_schema/IcqHelperContract";
import { NeutronVaultContract } from "../artifacts/typescript_schema/NeutronVaultContract";
// import { NeutronVotingRegistryContract } from "../artifacts/typescript_schema/NeutronVotingRegistryContract";

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

  const voting_vault = new NeutronVaultContract();
  await voting_vault.setupClient();
  const icq_helper = new IcqHelperContract();
  await icq_helper.setupClient();

  // Deploy ICQ helper
  const deploy_icq_helper = await icq_helper.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_icq_helper);

  // Init ICQ helper
  const init_icq_helper = await icq_helper.instantiate(
    {
    },
    `ICQ helper contract ${runTs}`,
    contract_owner
  );
  console.log(chalk.cyan("Response: "), init_icq_helper);

  // Deploy Voting vault
  const deploy_voting_vault = await voting_vault.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_voting_vault);

  // Init Voting vault
  const init_voting_vault = await voting_vault.instantiate(
    {
      denom: nativeDenom,
      description: "NTRN voting contract",
      name: "voting_vault",
      owner: contract_owner.account.address,
      remote_chain_id: "test-2",
      icq_helper: icq_helper.contractAddress,
    },
    `Voting vault contract ${runTs}`,
    contract_owner
  );
  console.log(chalk.cyan("Response: "), init_voting_vault);

  console.log("All contract instance created successfully");

  // register ICQ query for remote account delegation
  const register_delegations_user_res = await icq_helper.registerDelegatorDelegationsQuery(
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
      delegator: remoteAccount,
      connectionId: connectionId,
      validators: [remoteValidatorOne],
      updatePeriod: 5, // 5 blocks update period
    }
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(register_delegations_user_res, null, 2));

  await sleep(20);  // wait for query to be registered

  // Query delegations remote account
  const remote_delegations_user = await icq_helper.getDelegations({ address: remoteAccount });
  console.log(chalk.cyan("Response: "), JSON.stringify(remote_delegations_user, null, 2));

  // do a create voting power (fetches delegated amount to DAO voting power)
  const bond_res = await voting_vault.createVoter(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
    },
    {
      remoteAddress: remoteAccount,
    },
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(bond_res, null, 2));

  // query bonded amount at height
  const bonded_amount_res_1 = await voting_vault.votingPowerAtHeight(
    {
      address: contract_owner.account.address,
      height: null,
    },
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(bonded_amount_res_1, null, 2));

  const bonded_total_res_1 = await voting_vault.totalPowerAtHeight(
    {
      height: null,
    },
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(bonded_total_res_1, null, 2));

  const bonded_status_res_1 = await voting_vault.bondingStatus(
    {
      address: contract_owner.account.address,
      height: null,
    },
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(bonded_status_res_1, null, 2));

  // query list of depositors
  const list_depositors = await voting_vault.listBonders(
    {
      limit: null,
      startAfter: null,
    },
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(list_depositors, null, 2));

  // // Register delegations user query
  // const register_delegations_user_res = await voting_vault.addVotingVault(
  //   {
  //     account: contract_owner,
  //     customFees: {
  //       amount: [{ amount: "75000", denom: nativeDenom }],
  //       gas: "300000",
  //     },
  //     transferAmount: [ // fee for doing multiple ICQ queries, should just a bit more than min_fee
  //       { amount: "2000000", denom: nativeDenom }
  //     ]
  //   },
  //   {
  //     newVotingVaultContract: "",
  //   }
  // );
  // console.log(chalk.cyan("Response: "), JSON.stringify(register_delegations_user_res, null, 2));

  // await sleep(10);  // wait for query to be registered

  // // Query delegations user
  // const remote_delegations_user = await voting_vault.getDelegations({ address: remoteAccount });
  // console.log(chalk.cyan("Response: "), JSON.stringify(remote_delegations_user, null, 2));

  // await sleep(10);  // wait for query to be registered

  // // Query balance
  // const remote_balance = await voting_vault.balance({ address: remoteAccount });
  // console.log(chalk.cyan("Response: "), JSON.stringify(remote_balance, null, 2));
}

module.exports = { default: run };
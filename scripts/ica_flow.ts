import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";

import { IcaHelperContract } from "../artifacts/typescript_schema/IcaHelperContract";

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
  const interchainAccountName = "remote_account_1";
  const remoteValidatorOne = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";

  console.log("admin account fetched successfully");

  const ica_helper = new IcaHelperContract();
  await ica_helper.setupClient();

  // Deploy ICA helper
  const deploy_ica_helper = await ica_helper.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), deploy_ica_helper);

  // Init ICA helper
  const init_ica_helper = await ica_helper.instantiate(
    {
    },
    `ICA helper contract ${runTs}`,
    contract_owner
  );
  console.log(chalk.cyan("Response: "), init_ica_helper);

  console.log("All contract instance created successfully");

  // Register account on remote chain
  const register_res = await ica_helper.register(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "300000",
      },
    },
    {
      connectionId: connectionId,
      interchainAccountId: interchainAccountName,
    }
  );
  console.log(chalk.cyan("Response: "), register_res);

  await sleep(10);  // wait for addr to be created

  // Query interchain address
  const accountInfo = await ica_helper.interchainAccountAddress({
    connectionId: connectionId,
    interchainAccountId: interchainAccountName,
  });
  console.log(chalk.cyan("Response: "), "account info: ", JSON.stringify(accountInfo, null, 2));

  // Query more account data
  const moreAccountInfo = await ica_helper.interchainAccountAddressFromContract({
    interchainAccountId: interchainAccountName,
  });
  console.log(chalk.cyan("Response: "), "more account info: ", JSON.stringify(moreAccountInfo, null, 2));

  // Send NTRN to remote account
  const send_ntrn = await ica_helper.sendAsset(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "3000000",
      },
      transferAmount: [ // fee for doing IBC transfer, should just a bit more than min_fee
        { amount: "1050000", denom: nativeDenom }
      ] // added tranfer amount to IBC gas
    },
    {
      amount: "1000000" as any,  // 1 NTRN
      denom: nativeDenom,
      interchainAccountId: accountInfo.interchain_account_address,
      channel: channelId,
      timeout: null,
    }
  );
  console.log(chalk.cyan("Response: "), send_ntrn);

  // Send ATOM to remote account

  // Make community spend proposal
  const create_proposal_res = await ica_helper.proposeFunds(
    {
      account: contract_owner,
      customFees: {
        amount: [{ amount: "75000", denom: nativeDenom }],
        gas: "3000000",
      },
      transferAmount: [ // fee for doing ICA, should just a bit more than min_fee
        { amount: "50000", denom: nativeDenom }
      ]
    },
    {
      amount: "1000000" as any,  // 1 atom
      denom: remoteDenom,
      interchainAccountId: interchainAccountName,
      timeout: null,  // in seconds, TODO: confirm it later
    }
  );
  console.log(chalk.cyan("Response: "), create_proposal_res);

  await sleep(20);  // wait for prop to be created

  const ack_results_1 = await ica_helper.acknowledgementResult({
    interchainAccountId: interchainAccountName,
    sequenceId: 1,
  });
  console.log(chalk.cyan("Response: "), "1", ack_results_1);

  const ack_results_2 = await ica_helper.acknowledgementResult({
    interchainAccountId: interchainAccountName,
    sequenceId: 2,
  });
  console.log(chalk.cyan("Response: "), "2", ack_results_2);

  // const c1 = await staking_contract.info();
  // console.log("info before deposit: ",c1);
}

module.exports = { default: run };
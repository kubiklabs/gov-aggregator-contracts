import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";

import { IcqHelperContract } from "../artifacts/typescript_schema/IcqHelperContract";

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

  console.log("All contract instance created successfully");

  // Register transfer txns query
  const register_transfers_res = await icq_helper.registerTransfersQuery(
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
      recipient: remoteAccount,
      connectionId: connectionId,
      minHeight: 1000,
      updatePeriod: 5, // 5 blocks update period
    }
  );
  console.log(chalk.cyan("Response: "), JSON.stringify(register_transfers_res, null, 2));

  await sleep(10);  // wait for query to be registered

  // Query transfer txns
  const remote_trasfer_txns = await icq_helper.getRecipientTxs({ recipient: remoteAccount });
  console.log(chalk.cyan("Response: "), JSON.stringify(remote_trasfer_txns, null, 2));
}

module.exports = { default: run };
import { getAccountByName } from "../..//wasmkit//packages/wasmkit/dist/internal/cli/cli.js";

import { NeutronInterchainTxsContract } from "../artifacts/typescript_schema/NeutronInterchainTxsContract";

function sleep(seconds: number) {
  console.log("Sleeping for " + seconds + " seconds");
  return new Promise(resolve => setTimeout(resolve, seconds*1000));
}

async function run () {
  const runTs = String(new Date());
  const nativeDenom = "untrn";  // neutron fee token
  const remoteDenom = "uatom";  // cosmos hub fee token
  const contract_owner = await getAccountByName("account_0");

  const connectionId = "connection-0";
  const interchainAccountName = "staking_account_1";
  const remoteValidatorOne = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";

  console.log("admin account fetched successfully");

  const staking_contract = new NeutronInterchainTxsContract();
  await staking_contract.setupClient();
  console.log("All contract instance created successfully");

  // Staking Contract Deploy
  const staking_deploy_response = await staking_contract.deploy(
    contract_owner,
    {
      amount: [{ amount: "13000", denom: nativeDenom }],
      gas: "5000000",
    }
  );
  console.log(chalk.cyan("Response: "), staking_deploy_response);

  // Staking Contract Init
  const staking_contract_info = await staking_contract.instantiate(
    {},
    `Staking contract ${runTs}`,
    contract_owner
  );
  console.log(chalk.cyan("Response: "), staking_contract_info);

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

  // Query interchain address
  const accountInfo = await staking_contract.interchainAccountAddress({
    connectionId: connectionId,
    interchainAccountId: interchainAccountName,
  });
  console.log(chalk.cyan("Response: "), "account info: ", JSON.stringify(accountInfo, null, 2));

  // Query more account data
  const moreAccountInfo = await staking_contract.interchainAccountAddressFromContract({
    interchainAccountId: interchainAccountName,
  });
  console.log(chalk.cyan("Response: "), "more account info: ", JSON.stringify(moreAccountInfo, null, 2));

  // Delegate 1 atom
  const stake_claim_res = await staking_contract.delegate(
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
      validator: remoteValidatorOne,
    }
  );
  console.log(chalk.cyan("Response: "), stake_claim_res);

  // const c1 = await staking_contract.info();
  // console.log("info before deposit: ",c1);
}

module.exports = { default: run };
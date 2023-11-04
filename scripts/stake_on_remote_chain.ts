import chalk from "chalk";
import { getAccountByName } from "@kubiklabs/wasmkit";
import { SigningStargateClient, Coin } from "@cosmjs/stargate";
import { DirectSecp256k1HdWallet, makeCosmoshubPath } from "@cosmjs/proto-signing";

function sleep(seconds: number) {
  console.log("Sleeping for " + seconds + " seconds");
  return new Promise(resolve => setTimeout(resolve, seconds*1000));
}

async function run () {
  const runTs = String(new Date());
  const remoteDenom = "uatom";  // cosmos hub fee token
  const contract_owner = await getAccountByName("account_0");

  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(contract_owner.account.mnemonic, {
    hdPaths: [makeCosmoshubPath(0)],
    prefix: "cosmos"
  });
  const client = await SigningStargateClient.connectWithSigner(
    'http://localhost:16657/',
    wallet,
  );

  const remoteAccount = "cosmos10h9stc5v6ntgeygf5xf945njqq5h32r53uquvw";
  const remoteValidatorOne = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";

  console.log("admin account fetched successfully");

  // query balance and delegation
  const balanceBefore = await client.getBalance(remoteAccount, remoteDenom);
  const delegationBefore = await client.getBalanceStaked(remoteAccount);

  console.log(chalk.cyan("Response: "), balanceBefore);
  console.log(chalk.cyan("Response: "), delegationBefore);

  // stake 1,000 ATOM
  const stakeResponse = await client.delegateTokens(
    remoteAccount,
    remoteValidatorOne,
    { amount: '1000000000', denom: remoteDenom } as Coin,
    {
      amount: [{ amount: "250000", denom: "uatom" }],
      gas: "1000000",
    },
    undefined,
  );
  console.log(chalk.cyan("Response: "), stakeResponse);

  // query balance and delegation
  const balanceAfter = await client.getBalance(remoteAccount, remoteDenom);
  const delegationAfter = await client.getBalanceStaked(remoteAccount);

  console.log(chalk.cyan("Response: "), balanceAfter);
  console.log(chalk.cyan("Response: "), delegationAfter);
}

module.exports = { default: run };
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

  const interchainAccount = "cosmos1rjygr96xuk0755vhvyqkrmh5lkuv9dncfwncca7dhzh07v8dxe5st546ta";
  const remoteValidatorOne = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn";

  console.log("admin account fetched successfully");

  // send 15,000 ATOM to remote addr
  const sendResponse = await client.sendTokens(
    contract_owner.account.address,
    interchainAccount,
    [{ amount: '15000000000', denom: remoteDenom } as Coin],
    {
      amount: [{ amount: "250000", denom: "uatom" }],
      gas: "1000000",
    },
    undefined,
  );
  console.log(chalk.cyan("Response: "), sendResponse);

  // query balance and delegation of this ICA account
  const balanceBefore = await client.getBalance(interchainAccount, remoteDenom);
  const delegationBefore = await client.getBalanceStaked(interchainAccount);

  console.log(chalk.cyan("Response: "), balanceBefore);
  console.log(chalk.cyan("Response: "), delegationBefore);

  // wait for neutron contract to make delegation txn
  await sleep(50);

  // query balance and delegation of this ICA account
  const balanceAfter = await client.getBalance(interchainAccount, remoteDenom);
  const delegationAfter = await client.getBalanceStaked(interchainAccount);

  console.log(chalk.cyan("Response: "), balanceAfter);
  console.log(chalk.cyan("Response: "), delegationAfter);
}

module.exports = { default: run };
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { SigningStargateClient } from "@cosmjs/stargate";

const config = [
    {
        mnemonic: "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass",
        amount: "100000000", // amount to transfer from src chain to sink chain
        denom: "uatom",
        prefix: "cosmos",
        chain_id: "gaia-test-2",
        channel_id: "channel-0",
        chain_rpc: "http://45.250.253.23:16657",
        interchainAccount: "cosmos18zc2laaztm9hck6wmpvcdk9ptscdnrzx8rttecw57xwm0kzjyr5s5dyq0v"
    },
    // {
    //     mnemonic: "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass",
    //     amount: "100000000", // amount to transfer from src chain to sink chain
    //     denom: "ujuno",
    //     prefix: "juno",
    //     chain_id: "juno-test-3",
    //     channel_id: "channel-0",
    //     chain_rpc: "http://45.250.253.23:36657",
    //     interchainAccount: "juno1t87jxpq5hysgqagfy5umh2ld77jxkdel9t82v4f0ajemsxx9h09sy8cewp"
    // },
    {
        mnemonic: "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass",
        amount: "100000000", // amount to transfer from src chain to sink chain
        denom: "uosmo",
        prefix: "osmo",
        chain_id: "osmo-test-4",
        channel_id: "channel-0",
        chain_rpc: "http://45.250.253.23:46657",
        interchainAccount: "osmo1v8las4m2dgkktfhqqc36pxqzqsmjcjyak4n6nvpky9uf8a3yrxsqsfspnm"
    },

]
// const interchainAccount = "cosmos1zxj29075wuqw8njxuklvw4tkvkx3mnz4qsmh9msj0nj2t5w32s8qwj972u";


async function fundRemoteAccount() {

    for (let i = 0; i < config.length; i++) {

        // const current_timestamp = Date.now() * 1000000;
        // console.log("current timestamp is ", current_timestamp);

        const wallet = await DirectSecp256k1HdWallet.fromMnemonic(config[i].mnemonic, {
            prefix: config[i].prefix,
        });
        const accounts = await wallet.getAccounts();

        console.log(`Address for ${i} : `, accounts[0].address);

        const client = await SigningStargateClient.connectWithSigner(
            config[i].chain_rpc,
            wallet,
        );
        console.log("client created");

        // const fee = {
        //     amount: [
        //         {
        //             denom: config[i].denom,
        //             amount: "250000",
        //         },
        //     ],
        //     gas: "180000", // 180k
        // };
        // const memo = "Use your power wisely";

        const sendResponse = await client.sendTokens(
            accounts[0].address,
            config[i].interchainAccount,
            [{ amount: '120000000', denom: config[i].denom }],
            {
            amount: [{ amount: "250000", denom: config[i].denom }],
            gas: "1000000",
            },
            undefined,
        );
        console.log("Remote account funded ",sendResponse);

    }

}

fundRemoteAccount()

import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { SigningStargateClient } from "@cosmjs/stargate";

const config = [
    // {
    //     mnemonic: "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass",
    //     amount: "100000000", // amount to transfer from src chain to sink chain
    //     denom: "uatom",
    //     prefix: "cosmos",
    //     chain_id: "gaia-test-2",
    //     channel_id: "channel-0",
    //     chain_rpc: "http://45.250.253.23:16657"
    // },
    {
        mnemonic: "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass",
        amount: "100000000", // amount to transfer from src chain to sink chain
        denom: "ujuno",
        prefix: "juno",
        chain_id: "juno-test-3",
        channel_id: "channel-0",
        chain_rpc: "http://45.250.253.23:36657"
    },
    {
        mnemonic: "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass",
        amount: "100000000", // amount to transfer from src chain to sink chain
        denom: "uosmo",
        prefix: "osmo",
        chain_id: "osmosis-test-4",
        channel_id: "channel-0",
        chain_rpc: "http://45.250.253.23:46657"
    },

]
// const recipientAddress = "neutron1wpqx4mhe5hmgte8s4etam4syfxjt83zwvejhgsmcludfpt5hd6kquxtjd3";
const recipientAddress = "neutron1d3gt5cvy85szjuk06u48ln2x84s5w8697xjswpdc43z53u99nfnsup0762";


async function ibcTransfer() {

    for (let i = 0; i < config.length; i++) {

        // const current_timestamp = Date.now() * 1000000;
        const current_timestamp = 1702130208000000000;
        console.log("current timestamp is ", current_timestamp);

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

        // Construct the IBC transfer message
        const ibcTransferMsg = {
            typeUrl: '/ibc.applications.transfer.v1.MsgTransfer',
            value: {
                sourcePort: 'transfer',
                sourceChannel: config[i].channel_id,
                sender: accounts[0].address,
                receiver: recipientAddress,
                token: {
                    amount: config[i].amount,
                    denom: config[i].denom
                }, // Adjust the amount and denom
                timeoutHeight: '0',
                timeoutTimestamp: current_timestamp,
                memo: 'Hello world'
            },
        };

        const fee = {
            amount: [
                {
                    denom: config[i].denom,
                    amount: "2000",
                },
            ],
            gas: "180000", // 180k
        };
        const memo = "Use your power wisely";


        // Sign and broadcast the IBC transfer message on the source chain
        const response = await client.signAndBroadcast(accounts[0].address, [ibcTransferMsg],fee);
        console.log("Response :", response);
        console.log(`Sent IBC transfer on source chain, transfered ${config[i].denom} from ${config[i].chain_id} to ${recipientAddress} on neutron-test-1`);

    }

}

ibcTransfer()

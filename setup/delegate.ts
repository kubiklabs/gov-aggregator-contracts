import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { SigningStargateClient } from "@cosmjs/stargate";

const config = [
    {
        mnemonic: "banner spread envelope side kite person disagree path silver will brother under couch edit food venture squirrel civil budget number acquire point work mass",
        amount: "100000000",
    },
    {
        mnemonic: "veteran try aware erosion drink dance decade comic dawn museum release episode original list ability owner size tuition surface ceiling depth seminar capable only",
        amount: "100000000",
    },
    {
        mnemonic: "obscure canal because tomorrow tribe sibling describe satoshi kiwi upgrade bless empty math trend erosion oblige donate label birth chronic hazard ensure wreck shine",
        amount: "100000000",
    },
    {
        mnemonic: "orange shaft abandon find six fluid release picnic library waste inflict velvet physical clerk manual rookie cargo gown vendor museum dove brain runway people",
        amount: "100000000",
    },
    {
        mnemonic: "labor add oven alone pride disease imitate february smooth pudding grain seat slim slice gown matrix citizen extra vessel increase release settle boring chair",
        amount: "100000000",
    },
    {
        mnemonic: "member deal deputy vague embody truck ozone pull unique picture say tool rabbit ripple raise garlic point thunder level clinic toddler avocado knee maze",
        amount: "100000000",
    },
]


const gaiarpcEndpoint = "http://45.250.253.23:16657";
const val_address = "cosmos18hl5c9xn5dze2g50uaw0l2mr02ew57zk2fgr8q"
const valoper_address = "cosmosvaloper18hl5c9xn5dze2g50uaw0l2mr02ew57zk0auktn"


async function delegate() {
    
    for(let i=0;i<config.length;i++){
        const wallet = await DirectSecp256k1HdWallet.fromMnemonic(config[i].mnemonic);
        const accounts= await wallet.getAccounts();

        console.log(`Address for ${i} : `, accounts[0].address );

        const client = await SigningStargateClient.connectWithSigner(
          gaiarpcEndpoint,
          wallet,
        );
        console.log("client created");

        const fee = {
          amount: [
            {
              denom: "uatom",
              amount: "2000",
            },
          ],
          gas: "180000",
        };
        const memo = "Use your power wisely";
        

        const coin = {
            denom: "uatom",
            amount: config[i].amount
        }

        const result = await client.delegateTokens(
            accounts[0].address,
            valoper_address,
            coin,
            fee,
            memo
        )
        console.log("Result : ",result);
        console.log("Account no. ", i, "delegated")
    }
    
}

delegate()

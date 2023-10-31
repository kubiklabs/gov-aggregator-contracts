import { AccountData, DirectSecp256k1HdWallet, Registry } from "@cosmjs/proto-signing";
import { AminoMsgDelegate, SigningStargateClient,StargateClient} from "@cosmjs/stargate";

// A message type auto-generated from .proto files using ts-proto. @cosmjs/stargate ships some
// common types but don't rely on those being available. You need to set up your own code generator
// for the types you care about. How this is done should be documented, but is not yet:
// https://github.com/cosmos/cosmjs/issues/640

// const VAL_MNEMONIC = "clock post desk civil pottery foster expand merit dash seminar song memory figure uniform spice circle try happy obvious trash crime hybrid hood cushion"

// right now 15 accounts
// const mnemonics: string[] = [
// "tower crazy oblige owner chimney snow blanket sunny clown hotel exit raise circle cage stumble crush quiz scorpion broken door drill blue dance alley",
// "major sorry fine subject thumb camp vintage jacket valley hold bronze thought crime slow point either cycle supply buzz major style powder effort chief",
// "corn order odor cart relax practice wrestle gravity ankle category exile surface mule clay message quote cushion possible aspect ensure hazard slow torch repeat",
// "symptom camera collect dismiss screen wagon club maid math slim awkward joy human inch orbit sing display nice gentle gauge object pride salmon forget",
// "half sauce cupboard card audit fitness replace entire crack exile audit brave delay exhaust embark like afraid mountain critic custom glimpse load grunt ugly",
// "region sure orchard robust asset maximum output genre stand hurt dilemma disease accuse truth cargo approve foster pear two great bonus life bracket brief",
// "miss win girl project sponsor want theme absorb olympic survey axis rate exercise blue reunion know affair velvet verify model crop ticket wave photo",
// "relax major water toddler side dash danger cliff island denial border aisle pepper poverty scheme camp journey idle act kind pill praise exchange solution",
// "click help knock drastic tourist cancel mom winner sort keen poem cross book lady front coin steel chef color few just hockey cable diamond",
// ]
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


async function undelegate() {
    
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
          gas: "180000", // 180k
        };
        const memo = "Use your power wisely";
        

        const coin = {
            denom: "uatom",
            amount: config[i].amount
        }

        const result = await client.undelegateTokens(
            accounts[0].address,
            valoper_address,
            coin,
            fee,
            memo
        )
        console.log("Result : ",result);
        console.log("Account no. ", i, "undelegation started for this account")
    }

}

undelegate()

# Single choice proposal deposit contract

This is a pre-propose module that manages proposal deposits for the
`cwd-proposal-single` proposal module.

It accepts NTRN tokens. If a proposal deposit is enabled (by default it is enabled)
the following refund strategies are avaliable:

1. Never refund deposits. All deposits are sent to the DAO on proposal
   completion.
2. Always refund deposits. Deposits are returned to the proposer on
   proposal completion.
3. Only refund passed proposals. Deposits are only returned to the
   proposer if the proposal passes. Otherwise, they are sent to the
   DAO.

This module may also be configured to only accept proposals from
members (addresses with voting power) of the DAO.

## Messages

It only propose proposals of types ProposalType.
```
pub enum ProposalType {
    BringRemoteFund{
        demand_info: Vec<FundInfo>
    },
    AskFund{
        demand_info: Vec<FundInfo>
    }
}
// Maybe change this for Ask fund(get only denom and amount)
pub struct FundInfo {
    pub chain_id: String,
    pub amount: Uint128,
    pub denom: String
}

User will make transaction in this contract to propose a propsoal. The contract will then convert the messages for proposal contract check for deposits and other requirements and then call the proposal contract for proposal creation.

Only these type of proposals will be proposed and this will be executed by the core contract after sucessfully passing.

It does not store anything as it is only for prerequistie checks and transforming the messgaes.
# Accelarator DAO

## 1. Abstract
PAIDs are interchain DAOs to aggregate the funds of community pools of multiple chains in order to make aggregated decision making for common goals.

Community pools are multiple cosmos chains can aggregate a portion of their funds into a common pool and collectively decide on how to spend the collective funds and stakers of each chain can reuse their staked voting power to vote in this DAO removing any additional friction for community pool voters.

## 2. Problem Statement
A community pool is a portion of assets held by the chain and controlled by on-chain voting. Community pools are in place to be utilized for growth of the chain, may that be for feature developement, seeding liquidity or any other spend that directly or indirectly accelarates the chain's growth.

We're now in interchain era as IBC as matured by a lot and there are 100+ IBC enabled chains within cosmos. Each of these chains has their own community pool and that in most chains is only constituted of the chain's gas token, such as ATOM for Cosmos Hub, OSMO for Osmosis and so on.

Community pools can only spend in their own gas token and it's inefficient to convert the tokens to some other denom primarily due to lack of deep liquidity and secondary due to lack of infrastructure of doing so permissionlessly.

There are cases where multiple community pool funded DAOs come in verbal aggrement to split funding a proposal amongst them, so there can be proposal which are partially funded by ATOM and partially by OSMO cause the proposal's work would be used both by Cosmos Hub and Osmosis chain. Here Neither ATOM is coverted to OSMO nor OSMO to ATOM through a liquidity pool before funding the proposal instead some portion of Hub's community pool (denominated in ATOM) and some portion of Osmosis (denominated in OSMO) is used to fund but this is bespoke and requires verbal agreements.

PAIDs solve this very problem by allowing community pools of multiple chains to form a DAO and permissionlessly send funds to this DAO and stakers of each of the chains can vote for proposals on this DAO. As an example, if a staker has 6% of all staked ATOM on Hub and the DAO has 50% say of Hub then this staker will automatically get 3% voting power in this DAO without having to unstake or restake their ATOM or any other asset.

## 3. Contracts list
The core contracts are as follows:

1. **dao_core**: This contract is the core module of ADAO. It handles management of voting power and proposal modules,ICA helper module, executes messages and holds the DAO's treasury.
2. **dao_voting**: Keeps track of the voting done so far.(Not implemented yet)
3. **pre-propose**: Proposer will interact with this contract which will modify messages accordingly and call the **propose** contract.
4. **Proposal**: Creates proposal,store it, vote is done by interacting with this contract and for passed proposal it will call core dao contract to execute the messages.
5. **dao_vote_listener**: It is an ICQ contract which will query voting power on different chains and also further listen the events like delegation and store it for calculating respective voting-power in the DAO. It will do ICQ queries on remote chain for balances, staked amount, token tokens staked over IBC.
6. **ica_helper**: Helper methods to do ICA actions such as registering remote address on each remote chain, creating commuity pool spend proposal, transferring funds from remote chain to the DAO treasury over IBC, sending the DAO treasury back to remote chain's community pool over IBC.
7. **chain_registry**: Holds the IBC connection details of the remote chain with neutron. Things such as connectionId, channelds for ICA, ICQ, token transfer, status of the connection. This can be a single contract containing a list of all chains or multiple contracts and each one having list of connections to a given remote chain. Other chain metadata such as gas denom, community pool address could also be stored here.

## 4. Scripts and Setup

The "Setup" directory comprises two TypeScript files: "delegate.ts" and "undelegate.ts." These files offer the flexibility to configure various parameters, including the delegator's and validator's mnemonics, the chain's RPC (Remote Procedure Call) endpoint, and the specific amounts for delegating or undelegating tokens, among other customizable settings.
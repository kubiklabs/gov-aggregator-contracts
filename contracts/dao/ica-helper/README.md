## Messages

```
pub enum ExecuteMsg {
    Register {
        connection_id: String,
        interchain_account_id: String,
    },
}

Register will register an interchain account on remote chain.

- This account will be used to create proposals on remote chains and all other related things.

More messages will be added.

## Storage

It has **INTERCHAIN_ACCOUNTS** map which will store the intechain accounts created on remote chains.
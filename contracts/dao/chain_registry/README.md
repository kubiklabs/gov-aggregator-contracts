## Storage

This contract stores all the info about remote chains mapping them with chain-id.
**CHAIN** is a map which maps the chain-id to connection-id.
Further storage will be updated as per the requirements.

## Messages
`
pub enum ExecuteMsg{

    UpdateChainInfo {
        remote_chain: String,
        connection_id: String
    },
}

It has UpdateChainInfo entry point message which takes remote_chain id and connection id and store it in mapping remote chain id to connection id.

More messages and admin checks will be added as per the requirements.

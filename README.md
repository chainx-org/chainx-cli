# chainx-cli

A command-line tool of ChainX.

## Build

```
cargo build --release
```

## Usage

```
USAGE:
    xli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    completions    Generates completion scripts for your shell.
    rpc            Rpc subcommand.
```

```
Rpc subcommand.

USAGE:
    xli rpc <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    addr-by-account             Get the binding BTC address of the account.
    addr-verification           Verify the correctness of the withdrawal address.
    asset                       Get the asset information of the account.
    assets                      Get the assets information.
    block                       Get header and body of a relay chain block.
    block-hash                  Get hash of the n-th block in the canon chain.
    call-fee                    Get the fee according to the call and transaction length.
    call-fee-map                Get the fee weight map, transaction base fee and transaction byte fee.
    cross-mining-dividend       Get the cross mining dividend of the account.
    deposit-limit               Get the limitation related to deposits.
    deposit-list                Get all current deposit records.
    extrinsic-events            Get all extrinsic events in a block.
    header                      Get header of a relay chain block.
    header-finalized            Get hash of the last finalized block in the canon chain.
    intention                   Get the information of the node address.
    intentions                  Get the all node information.
    mock-btc-new-trustees       Simulate the generation of next era BTC trustee address.
    next-renominate             Get the block height of the account's next switchable vote.
    nomination-records          Get the nominate records of the account.
    orders                      Get the pending orders list of the account.
    particular-accounts         Get the particular account addresses (council, team, trustees).
    psedu-intentions            Get the mining list.
    psedu-nomination-records    Get the voting information of the account.
    quotations                  Get the trading quotations list.
    runtime-version             Get the runtime version.
    staking-dividend            Get the staking dividend of the account.
    system-chain                Get the chain's type. Given as a string identifier.
    system-health               Get health status of the node.
    system-name                 Get the node's implementation name. Plain old string.
    system-network-state        Get current state of the network.
    system-peers                Get currently connected peers.
    system-properties           Get a custom set of properties as a JSON object, defined in the chain spec.
    system-version              Get the node implementation's version. Should be a semver string.
    trading-pairs               Get the trading pairs list.
    trustee-info                Get the trustee information of the account.
    trustee-session             Get the current trustee information of the chain.
    withdraw-limit              Get the limitation related to withdrawals.
    withdraw-list               Get all current withdrawal records.
    withdraw-tx                 Get the withdrawal transactions of the chain.
```

See the [wiki](https://github.com/chainx-org/ChainX/wiki/RPC) for RPC details.

# LICENSE

[GPL v3](./LICENSE)

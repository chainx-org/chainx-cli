# chainx-cli

A ChainX command-line tool.

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
    addr_by_account             Get the binding BTC address of the account.
    addr_verification           Verify the correctness of the withdrawal address.
    asset                       Get the asset information of the account.
    assets                      Get the assets information.
    block                       Get the runtime version.
    block_hash                  Get hash of the n-th block in the canon chain.
    call_fee                    Get the fee according to the call and transaction length.
    deposit_limit               Get the limitation related to deposits.
    deposit_list                Get all current deposit records.
    header                      Get header of a relay chain block.
    header_finalized            Get hash of the last finalized block in the canon chain.
    intention                   Get the information of the node address.
    intentions                  Get the all node information.
    mock_btc_new_trustees       Simulate the generation of next era BTC trustee address.
    next_renominate             Get the block height of the account's next switchable vote.
    nomination_records          Get the nominate records of the account.
    orders                      Get the pending orders list of the account.
    particular_accounts         Get the particular account addresses (council, team, trustees).
    psedu_intentions            Get the mining list.
    psedu_nomination_records    Get the voting information of the account.
    quotations                  Get the trading quotations list.
    system_chain                Get the chain's type. Given as a string identifier.
    system_health               Return health status of the node.
    system_name                 Get the node's implementation name. Plain old string.
    system_network_state        Returns current state of the network.
    system_peers                Returns currently connected peers.
    system_properties           Get a custom set of properties as a JSON object, defined in the chain spec.
    system_version              Get the node implementation's version. Should be a semver string.
    trading_pairs               Get the trading pairs list.
    trustee_info                Get the trustee information of the account.
    trustee_session             Get the current trustee information of the chain.
    withdraw_limit              Get the limitation related to withdrawals.
    withdraw_list               Get all current withdrawal records.
    withdraw_tx                 Get the withdrawal transactions of the chain.
```


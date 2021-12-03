# ChainX CLI 2.0

[![CI](https://github.com/chainx-org/ChainX/workflows/ci/badge.svg)](https://github.com/chainx-org/ChainX/actions?workflow=ci)

Rust Command Line Interface for [ChainX 2.0](https://github.com/chainx-org/ChainX/tree/develop-2.0) based on [substrate-subxt](https://github.com/paritytech/substrate-subxt).

## Build

```bash
$ git clone https://github.com/chainx-org/chainx-cli
$ cd chainx-cli

# There are multiple binaries in this repo.
# You might be solely interested in chainx-cli, then
# simply running `make` or using the following command
# directly will do and no other binaries will be compiled.
$ cargo build --release --bin chainx-cli

# This will compile all the binaries.
$ cargo build --release
```

## Usage

```bash
$ ./target/release/chainx-cli --help
```

```bash
$ cargo build --release --bin snapshot_balances
$ ./target/release/snapshot_balances --url ws://127.0.0.1:8087 --block-number 2761158
   On ChainX(decimals=8)  
        Total issuance: 1050000000000000
        Total accounts: 18165
          KSX accounts: 3686
Dust accounts(<100PCX): 14479
   Total dust balances: 10334549251431
      Treasury balance: 94591231912999
 X-association balance: 12090344828274
==========================
  On SherpaX(decimals=18) 
       Total issuance:  10500000000000000000000000
       Total accounts:  18165
Dust accounts(<100KSX): 14479
    Total dust balance: 103345492514310000000000
     Non-dust accounts: 3686
Total non-dust balance: 10396654507485690000000000
      Treasury balance: 1066815767412730000000000
 X-association balance: 0
```
## License

[GPL v3](./LICENSE)

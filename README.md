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

## snapshot_balances
```bash
$ cargo build --release --bin snapshot_balances 
```


1. ChainX 2.0 snapshot1 on block 2761158, min-balance=100000000
```bash
$ ./target/release/snapshot_balances --block-number=2761158 --url=ws://47.99.179.60:18087 --min-balance=100000000
On ChainX(decimals=8)  
        Total issuance: 1050000000000000
        Total accounts: 18165
     Non-dust accounts: 7418
Minim balance for dust: 100000000
         Dust accounts: 10747
   Total dust balances: 82628223512
      Treasury balance: 94591231912999
 X-association balance: 12090344828274
==========================
  On SherpaX(decimals=18)
     Total airdrop ksx: 10500000000000000000000000
        Total accounts: 18165
     Non-dust accounts: 7418
Minim balance for dust: 1000000000000000000
         Dust accounts: 0
   Total dust balances: 0
      Treasury balance: 1067642049647850000000000
 X-association balance: 0
Total non-dust balance: 10500000000000000000000000

```

2. ChainX 3.0 snapshot2 on block 2004141, min-balance=1
```bash
$ ./target/release/snapshot_balances  --block-number=2004141 --url=ws://47.99.179.60:8087 --min-balance=1
   On ChainX(decimals=8)  
        Total issuance: 1198522470000000
        Total accounts: 22424
     Non-dust accounts: 22295
Minim balance for dust: 1
         Dust accounts: 129
   Total dust balances: 0
      Treasury balance: 118654859286008
 X-association balance: 1526137095776
==========================
  On SherpaX(decimals=18) 
     Total airdrop ksx: 11985224700000000000000000
        Total accounts: 22424
     Non-dust accounts: 22295
Minim balance for dust: 10000000000
         Dust accounts: 0
   Total dust balances: 0
      Treasury balance: 1201809963817840000000000
 X-association balance: 0
Total non-dust balance: 11985224700000000000000000
```

## License

[GPL v3](./LICENSE)

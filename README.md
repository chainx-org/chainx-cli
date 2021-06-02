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

## License

[GPL v3](./LICENSE)

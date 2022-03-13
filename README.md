

## Regenesis data

```bash
./target/release/regenesis --block-number 200 --url ws://localhost:8087
```


## Generate metadata

```bash
cargo install subxt-cli

// chain=mainnet
subxt metadata -f bytes --url http://localhost:8086 > chainx_v4_metadata.scale
subxt metadata -f json --url http://localhost:8086 > chainx_v4_metadata.json
subxt codegen --url http://localhost:8086 > chainx_v4_metadata.rs
```


# Building

Based on the [CW721 Template](https://github.com/CosmWasm/cw-template)

  
```
cargo wasm

docker run --rm -v "$(pwd)":/code --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.6

starsd tx wasm store artifacts/stargaze_multisnap.wasm --from testnet-key --gas-prices 0.025ustars --gas-adjustment 1.7 --gas auto --output json -b block

starsd tx wasm instantiate (CODEID) "{}" --label "Multi Snapshot Test" --admin (ADMINADDRESS) --gas-prices 0.025ustars --gas auto --gas-adjustment 1.9 --from testnet-key -b block

```
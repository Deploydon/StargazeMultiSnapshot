# Stargaze Multi Snapshot

On EVM chains, [Multicall](https://github.com/makerdao/multicall) is used to execute multiple contract queries at once, by using an on-chain contract to handle the querying. This reduces the number of RPC calls needed, and allows a much quicker response.

On the Stargaze side, it is very common to perform snapshot of collection holders. In cases of the average collection size, this would ultimately result in thousands of RPC calls. 

This contract allows passing in the address of a SG721 NFT contract, the start and end index and having the contract perform the lookups. We can then return multiple responses per RPC call, greatly reducing the hammering on validator endpoints, as well as speeding up the process. 

The plan is to deploy this into Stargaze governance to bring it live on Mainnet. 

My rust abilities are not the best, so bare with me. 

Also included in the Tester folder is a simple nodejs script that uses the cosmwasm library to call this function, providing a testnet NFT collection. 

```
Code ID: 328
Testnet Address: stars18hxjy4f0suah8lq9uldwtg6uqysswfnj6yen2rjapvcr9tqlgdqs2un96x
```


Example Response (Start:30, End: 35):
```
[
  { id: 30, owner: 'stars17y9ysn4uwusc0qv0d48wtc5rf4gnu6mqvjpvg9' },
  { id: 31, owner: 'stars17y9ysn4uwusc0qv0d48wtc5rf4gnu6mqvjpvg9' },
  { id: 32, owner: 'stars17y9ysn4uwusc0qv0d48wtc5rf4gnu6mqvjpvg9' },
  { id: 33, owner: 'stars17y9ysn4uwusc0qv0d48wtc5rf4gnu6mqvjpvg9' },
  { id: 34, owner: 'stars17y9ysn4uwusc0qv0d48wtc5rf4gnu6mqvjpvg9' },
  { id: 35, owner: 'stars17y9ysn4uwusc0qv0d48wtc5rf4gnu6mqvjpvg9' }
]
```

## Testing instructions

### Compilation
First, we must build our wasm binary to store on the blockchain. 
> This is done using the [rust-optimizer](https://hub.docker.com/r/cosmwasm/rust-optimizer/tags) Docker container. (macOS arm64 see [this image](https://hub.docker.com/r/cosmwasm/rust-optimizer-arm64/tags))
```bash
# x86_64
cargo optimize

# arm64
cargo optimize-arm64
```

### Code Storage
After the wasm has compiled, we can upload the binary to the blockchain. This is done with the cargo script shown below. This cargo script assumes the developer's keys are stored in the `os` backend and named `dev`.
> Note: the [Stargaze CLI](https://github.com/public-awesome/stargaze/#install) is required for this step. 
```bash
cargo store
```
Look in the command output for the `code_id` field. This value will be needed in the next step.
`{"type":"store_code","attributes":[{"key":"code_id","value":"338"}]}]}]`

### Contract Instantiation
Once our code has been stored on the blockchain, we can instantiate a version of this code as our MultiSnapshot contract. This is done using the following command:
```bash
starsd tx wasm instantiate [CODE_ID] "{}" --label "Multi Snapshot Test" --admin [ADMIN ADDRESS] --gas-prices 0.025ustars --gas auto --gas-adjustment 1.9 --from [KEY NAME] -b block
```
Make sure to replace to placeholders (`[CODE ID]`, `[ADMIN ADDRESS]`, `[KEY NAME]`) with your relevant values. `[CODE ID]` should be the value you found in the Code Storage section.

### Client Test
Finally we can test the code. Navigate to the `/Tester/` directory, install the dependencies using `npm`, and call the `test-client` cargo script.
```bash
# Navigate to /Tester/
cd Tester

# Install dependencies
npm i

# Run client test
node ./querytest.js

# OR
cd ..
cargo test-client
```

### Parameter Tuning
The optimal performance of the contract will be determined by the two tunable parameters inside `querytest.js`. We use `iters` to perform multiple batch queries in a single RPC request. cw721 has a maximum value of 100 for the `limit` parameter used in the `AllTokens` query, which requires us to maximize the gas limits with the `iters` param. These parameters can be tuned at the top of [`/Tester/querytest.js`](/Tester/querytest.js).

## Development Status
- [x] Querying based on a start and end index
- [x] Better Error Handling
- [x] Limiting number returned
- [x] Paged and ranged function.
- [x] Query the SG721 contract for the minter address
- [ ] Query the minter contract for the config to get num_tokens. This will allow us to pass in page numbers, rather than a start and end index. (Having a hard time querying this side!)
- [ ] Cleanup all the unused code/imports
- [ ] Commonwealth proposal
- [ ] Mainnet



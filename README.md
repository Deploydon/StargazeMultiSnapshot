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

Development Status:
- [x] Querying based on a start and end index
- [x] Better Error Handling
- [x] Limiting number returned
- [x] Paged and ranged function.
- [x] Query the SG721 contract for the minter address
- [ ] Query the minter contract for the config to get num_tokens. This will allow us to pass in page numbers, rather than a start and end index. (Having a hard time querying this side!)
- [ ] Cleanup all the unused code/imports
- [ ] Commonwealth proposal
- [ ] Mainnet



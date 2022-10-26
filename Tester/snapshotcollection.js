//Snapshots a collection. Grabs the collections minter, queries for the num_tokens then queries the ranges.
var cosmwasm = require("cosmwasm");

var CONTRACT_MULTI = "stars18hxjy4f0suah8lq9uldwtg6uqysswfnj6yen2rjapvcr9tqlgdqs2un96x";
var COLLECTION_SG721 = "stars1ee4a3ad6lmc3ckvuuzlwk4vsyu7g7d7khtck07tsa8wgavapqarsvycuw4";
const RPC = "https://rpc.elgafar-1.stargaze-apis.com/";
const PAGE_MAX = 100;

async function main() {
    const start_time = new Date().getTime()
    const client = await cosmwasm.CosmWasmClient.connect(RPC);

    //Query the SG721 contract for the minter
    //Query the minter config for the num_tokens
    //The num_tokens reponse from the SG721 is invalid as it is the remainder after burns. We need the full number.
    
    const minterResp = await client.queryContractSmart(COLLECTION_SG721, { minter: {} });
    var mintContract = minterResp.minter;

    const configResp = await client.queryContractSmart(mintContract, { config: {} });
    var numTokens = configResp.num_tokens;
    var numCycles = Math.ceil(numTokens / PAGE_MAX);
    console.log("Total Collection Size: " + numTokens + " Num Queries: " + numCycles);

    let queryCount = 2 // Two previous executed queries
    var start = 1;
    var end = 100;
    if (numTokens < PAGE_MAX) {
        end = numTokens;
    }
    var allOwners = [];
    for (var i = 1; i <= numCycles; i++) {
        const tokenOwners = await client.queryContractSmart(CONTRACT_MULTI, { collection_owners_range: { collection: COLLECTION_SG721, start: start, end: end } });
        allOwners = allOwners.concat(tokenOwners);
        
        queryCount += 1
        console.log("Completed for for IDs through", end)
        
        start = i * PAGE_MAX;
        end = start + PAGE_MAX;
        if (end > numTokens) {
            end = numTokens;
        }
    }

    const end_time = new Date().getTime()
    console.log(`Fetched ${allOwners.length} NFTs in ${queryCount} queries.`)
    console.log(`Time taken: ${end_time-start_time} milliseconds`)
}


main();
#!/usr/bin/env node

var cosmwasm = require("cosmwasm");
var fs = require("fs");

const RPC = "https://rpc.elgafar-1.stargaze-apis.com/";
const CONTRACT_MULTI = "stars1nhyua3khswcu8zcz24n4kc6aasxsn3myqlhtxakelqkahv6txzeq8r6gk8"
const COLLECTION_SG721 = "stars1ee4a3ad6lmc3ckvuuzlwk4vsyu7g7d7khtck07tsa8wgavapqarsvycuw4";


const LIMIT = 40;
const ITERS = 10;
const OUT_FILE = "snapshot.json"

async function main() {
    const start = new Date().getTime()
    result = []
    const client = await cosmwasm.CosmWasmClient.connect(RPC);
    let start_after;
    let queryCount = 0    
    while (true) {
      const tokenOwners = await client.queryContractSmart(CONTRACT_MULTI, { 
        all_collection_owners: { 
          collection: COLLECTION_SG721, 
          iters: ITERS, 
          start_after: start_after, 
          limit: LIMIT,
        }
      });
      
      result = result.concat(tokenOwners)
      queryCount += 1
      
      if (!tokenOwners || tokenOwners.length < LIMIT) {
        fs.writeFileSync(OUT_FILE, JSON.stringify(result))
        console.log("Completed for for IDs through", start_after)
        console.log(`Finished fetching ${result.length} NFTs!`)
        break
      }

      start_after = tokenOwners[tokenOwners.length - 1]['id']
      console.log("Completed for for IDs through", start_after)
    }
    const end = new Date().getTime()
    console.log(`Fetched ${result.length} NFTs in ${queryCount} queries.`)
    console.log(`Time taken: ${end-start} milliseconds`)
}

main();

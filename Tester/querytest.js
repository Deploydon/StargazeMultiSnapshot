var cosmwasm = require("cosmwasm");

const RPC = "https://rpc.elgafar-1.stargaze-apis.com/";


var CONTRACT_MULTI = "stars18eupq2qjpygftyl488yzkr06s3kmw6j8zs9utnh7kyfk9tck0swsxkn8cj";

var COLLECTION_SG721 = "stars1ee4a3ad6lmc3ckvuuzlwk4vsyu7g7d7khtck07tsa8wgavapqarsvycuw4";

const START = 1;
const END = 30;
async function main() {
    const client = await cosmwasm.CosmWasmClient.connect(RPC);
    const tokenOwners = await client.queryContractSmart(CONTRACT_MULTI, { collection_owners: { collection: COLLECTION_SG721, start: START, end: END } });
    console.log(tokenOwners);
}


main();
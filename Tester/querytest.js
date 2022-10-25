var cosmwasm = require("cosmwasm");

const RPC = "https://rpc.elgafar-1.stargaze-apis.com/";


var CONTRACT_MULTI = "stars18hxjy4f0suah8lq9uldwtg6uqysswfnj6yen2rjapvcr9tqlgdqs2un96x";

var COLLECTION_SG721 = "stars1ee4a3ad6lmc3ckvuuzlwk4vsyu7g7d7khtck07tsa8wgavapqarsvycuw4";

var COLLECTION_MINTER = "stars1gj6mapkcqnfphfy3d33e0utxu9tc76ytzyuu47hupyrfqz6y4nrsx6dtt8";

const START = 1;
const END = 20;
async function main() {
    const client = await cosmwasm.CosmWasmClient.connect(RPC);
    const tokenOwners = await client.queryContractSmart(CONTRACT_MULTI, { collection_owners_range: { collection: COLLECTION_SG721, start: START, end: END } });
    console.log(tokenOwners);
}


main();
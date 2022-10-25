var cosmwasm = require("cosmwasm");

const RPC = "https://rpc.elgafar-1.stargaze-apis.com/";


var CONTRACT_MULTI = "stars18eupq2qjpygftyl488yzkr06s3kmw6j8zs9utnh7kyfk9tck0swsxkn8cj";




var COLLECTION_SG721 = "stars1ee4a3ad6lmc3ckvuuzlwk4vsyu7g7d7khtck07tsa8wgavapqarsvycuw4"; 
async function main() {
    var ID = 1;
    const client = await cosmwasm.CosmWasmClient.connect(RPC);
  // var collectionInfo = await client.queryContractSmart(COLLECTION, { config: {} });
   //console.log(collectionInfo);
  //  return;
  //var collectionTokenInfo = await client.queryContractSmart(COLLECTION, { owner_of: {token_id:"41", include_expired: false} });
   // console.log(collectionTokenInfo);
  //  return;
    const tokenInfo = await client.queryContractSmart(CONTRACT_MULTI, { collection_owners: { collection: COLLECTION_SG721 , start:30, end:45 } });
    console.log(tokenInfo);

}




async function getClient() {
    const gasPrice = cosmwasm.GasPrice.fromString('0ustars');
    const wallet = await cosmwasm.DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
        prefix: 'stars',
    });
    const [{ address, pubkey }] = await wallet.getAccounts();

    myAddress = address;
    console.log(myAddress);

    return await cosmwasm.SigningCosmWasmClient.connectWithSigner(
        RPC,
        wallet,
        { gasPrice }
    );
}




main();
use crate::routes::collectibles::handlers::collectibles;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket_okapi::openapi;
/// `/v1/chains/<chain_id>/safes/<safe_address>/collectibles?<trusted>&<exclude_spam>` <br />
/// Returns collectibles from the transaction handlers
///
/// # Collectibles
///
/// The collectibles endpoint does not implement any logic in the client-gateway. The response from the core services is cached and then forwarded to the clients.
///
/// ## Path
///
/// - `/v1/chains/<chain_id>/safes/<safe_address>/collectibles?<trusted>&<exclude_spam>` : Returns a list of the ERC721 tokens stored in a safe
///
/// ## Query parameters
///
/// `<trusted>` : A token is defined as trusted by our core handlers process when adding them. Default value is `false`
/// `<exclude_spam>`: A token is defined as spam by our core handlers process when adding them. Default value is `true`
///
/// ## Models
///
/// For the most up-to-date version of the endpoint please check: <https://safe-transaction.gnosis.io/>
///
/// ```json
/// [
///   {
///     "address": "string",
///     "tokenName": "string",
///     "tokenSymbol": "string",
///     "logoUri": "string",
///     "id": "string",
///     "uri": "string",
///     "name": "string",
///     "description": "string",
///     "imageUri": "string",
///     "metadata": {
///       "additionalProp1": "string",
///       "additionalProp2": "string",
///       "additionalProp3": "string"
///     }
///   }
/// ]
/// ```
///
/// ## JSON
///
/// <details>
/// <summary>Full response</summary>
///
/// ```json
/// [
///   {
///     "address": "0xD753e03c05533F85bA9695C139771b1E9698a53C",
///     "tokenName": "Main",
///     "tokenSymbol": "JOSE",
///     "logoUri": "https://gnosis-safe-token-logos.s3.amazonaws.com/0xD753e03c05533F85bA9695C139771b1E9698a53C.png",
///     "id": "2",
///     "uri": "https://arweave.net/8Vje5kmuRKaJwYht19rl-tEte9J8WifCrXoCwAB2IK8",
///     "name": "Chiken dinner",
///     "description": "This token is meant for testing. ",
///     "imageUri": "https://arweave.net/a_poR6wHyGUortY2G3ITFmpQJOsRGJUXngpUAharq8I",
///     "metadata": {
///       "minter": "0x4d3101d77aac1b90ae42efa38d235a81af270d40",
///       "mintedOn": "2020-12-29T13:49:02.654Z",
///       "contractAddress": "0xd753e03c05533f85ba9695c139771b1e9698a53c",
///       "minted": "Minted on Mintbase.io",
///       "fiatPrice": "$0.00",
///       "name": "Chiken dinner",
///       "description": "This token is meant for testing. ",
///       "youtubeUrl": "",
///       "price": 0,
///       "ethPrice": "0",
///       "amountToMint": "5",
///       "visibility": "safe",
///       "forSale": false,
///       "image": "https://arweave.net/a_poR6wHyGUortY2G3ITFmpQJOsRGJUXngpUAharq8I",
///       "attributes": [],
///       "category": "DQFi8lDeEyqqoAOPesJb",
///       "externalUrl": "https://mintbase.io/my-market/0xd753e03c05533f85ba9695c139771b1e9698a53c",
///       "type": "ERC721"
///     }
///   },
///   {
///     "address": "0xD753e03c05533F85bA9695C139771b1E9698a53C",
///     "tokenName": "Main",
///     "tokenSymbol": "JOSE",
///     "logoUri": "https://gnosis-safe-token-logos.s3.amazonaws.com/0xD753e03c05533F85bA9695C139771b1E9698a53C.png",
///     "id": "4",
///     "uri": "https://arweave.net/8Vje5kmuRKaJwYht19rl-tEte9J8WifCrXoCwAB2IK8",
///     "name": "Chiken dinner",
///     "description": "This token is meant for testing. ",
///     "imageUri": "https://arweave.net/a_poR6wHyGUortY2G3ITFmpQJOsRGJUXngpUAharq8I",
///     "metadata": {
///       "minter": "0x4d3101d77aac1b90ae42efa38d235a81af270d40",
///       "mintedOn": "2020-12-29T13:49:02.654Z",
///       "contractAddress": "0xd753e03c05533f85ba9695c139771b1e9698a53c",
///       "minted": "Minted on Mintbase.io",
///       "fiatPrice": "$0.00",
///       "name": "Chiken dinner",
///       "description": "This token is meant for testing. ",
///       "youtubeUrl": "",
///       "price": 0,
///       "ethPrice": "0",
///       "amountToMint": "5",
///       "visibility": "safe",
///       "forSale": false,
///       "image": "https://arweave.net/a_poR6wHyGUortY2G3ITFmpQJOsRGJUXngpUAharq8I",
///       "attributes": [],
///       "category": "DQFi8lDeEyqqoAOPesJb",
///       "externalUrl": "https://mintbase.io/my-market/0xd753e03c05533f85ba9695c139771b1e9698a53c",
///       "type": "ERC721"
///     }
///   },
///   {
///     "address": "0xD753e03c05533F85bA9695C139771b1E9698a53C",
///     "tokenName": "Main",
///     "tokenSymbol": "JOSE",
///     "logoUri": "https://gnosis-safe-token-logos.s3.amazonaws.com/0xD753e03c05533F85bA9695C139771b1E9698a53C.png",
///     "id": "5",
///     "uri": "https://arweave.net/8Vje5kmuRKaJwYht19rl-tEte9J8WifCrXoCwAB2IK8",
///     "name": "Chiken dinner",
///     "description": "This token is meant for testing. ",
///     "imageUri": "https://arweave.net/a_poR6wHyGUortY2G3ITFmpQJOsRGJUXngpUAharq8I",
///     "metadata": {
///       "minter": "0x4d3101d77aac1b90ae42efa38d235a81af270d40",
///       "mintedOn": "2020-12-29T13:49:02.654Z",
///       "contractAddress": "0xd753e03c05533f85ba9695c139771b1e9698a53c",
///       "minted": "Minted on Mintbase.io",
///       "fiatPrice": "$0.00",
///       "name": "Chiken dinner",
///       "description": "This token is meant for testing. ",
///       "youtubeUrl": "",
///       "price": 0,
///       "ethPrice": "0",
///       "amountToMint": "5",
///       "visibility": "safe",
///       "forSale": false,
///       "image": "https://arweave.net/a_poR6wHyGUortY2G3ITFmpQJOsRGJUXngpUAharq8I",
///       "attributes": [],
///       "category": "DQFi8lDeEyqqoAOPesJb",
///       "externalUrl": "https://mintbase.io/my-market/0xd753e03c05533f85ba9695c139771b1e9698a53c",
///       "type": "ERC721"
///     }
///   }
/// ]
/// ```
/// </details>
#[openapi(tag = "Collectibles")]
#[get("/v1/chains/<chain_id>/safes/<safe_address>/collectibles?<trusted>&<exclude_spam>")]
pub async fn get_collectibles(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<content::RawJson<String>> {
    collectibles(
        &context,
        chain_id.as_str(),
        safe_address.as_str(),
        trusted,
        exclude_spam,
    )
    .await
}

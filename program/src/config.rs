#[cfg(feature = "devnet")]
//TODO: Add the actual devnet program id
pub const PROGRAM_ID: &str = "devnet_program_id_here";
//TODO: Add the actual mainnet program id
#[cfg(not(feature = "devnet"))]
pub const PROGRAM_ID: &str = "dmainnet_program_id_here";


pub const COLLECTION_NAME: &str = "MintRewards NFT Collection";
pub const COLLECTION_SYMBOL: &str = "RWRD";
//TODO: Add the actual metadata uri
pub const COLLECTION_URI: &str = "https://some-metadata-uri.com";

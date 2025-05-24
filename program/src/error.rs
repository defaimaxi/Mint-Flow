use anchor_lang::prelude::*;

#[error_code]
pub enum NFTError {
    #[msg("Unauthorized access to this operation")]
    UnauthorizedAccess,
    
    #[msg("Buffer is full, cannot add more NFTs")]
    BufferFull,
    
    #[msg("No NFTs available in the buffer")]
    EmptyBuffer,
    
    #[msg("Invalid metadata provided")]
    InvalidMetadata,
    
    #[msg("Minting failed")]
    MintFailed,
}

#[event]
pub struct GenerateMoreNFTsEvent {
    pub current_buffer_size: u64,
    pub required_size: u64,
}

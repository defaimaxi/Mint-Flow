// program/src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction as mpl_instruction,
    state::{Creator, DataV2},
};

mod config;
mod error;
mod state;
mod processor;

use error::*;
use state::*;
use processor::*;

declare_id!(config::PROGRAM_ID);

#[program]
pub mod nft_minter {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        buffer_size: u64,
        min_buffer_threshold: u64,
        collection_mint: Pubkey,
    ) -> Result<()> {
        processor::initialize(ctx, buffer_size, min_buffer_threshold, collection_mint)
    }

    pub fn add_to_buffer(
        ctx: Context<AddToBuffer>,
        metadata_uri: String,
        name: String,
        symbol: String,
        seller_fee_basis_points: u16,
    ) -> Result<()> {
        processor::add_to_buffer(ctx, metadata_uri, name, symbol, seller_fee_basis_points)
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_bump: u8,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        processor::mint_nft(
            ctx,
            creator_bump,
            metadata_title,
            metadata_symbol,
            metadata_uri,
        )
    }
}


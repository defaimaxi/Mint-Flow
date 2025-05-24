// program/src/processor.rs
pub fn initialize(
    ctx: Context<Initialize>,
    buffer_size: u64,
    min_buffer_threshold: u64,
    collection_mint: Pubkey,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.authority = ctx.accounts.authority.key();
    state.buffer_size = buffer_size;
    state.min_buffer_threshold = min_buffer_threshold;
    state.collection_mint = collection_mint;
    state.pending_nfts = VecDeque::new();
    state.minted_count = 0;
    Ok(())
}

pub fn add_to_buffer(
    ctx: Context<AddToBuffer>,
    metadata_uri: String,
    name: String,
    symbol: String,
    seller_fee_basis_points: u16,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    
    require!(
        ctx.accounts.authority.key() == state.authority,
        NFTError::UnauthorizedAccess
    );

    require!(
        state.pending_nfts.len() < state.buffer_size as usize,
        NFTError::BufferFull
    );

    state.pending_nfts.push_back(NFTMetadata {
        uri: metadata_uri,
        name,
        symbol,
        seller_fee_basis_points,
        minted: false,
    });

    Ok(())
}

pub fn mint_nft(
    ctx: Context<MintNFT>,
    creator_bump: u8,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    
    require!(!state.pending_nfts.is_empty(), NFTError::EmptyBuffer);
    
    let nft_metadata = state.pending_nfts.pop_front()
        .ok_or(NFTError::EmptyBuffer)?;
    
    require!(!nft_metadata.minted, NFTError::InvalidMetadata);
    
    // Create mint account
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        },
    );
    token::initialize_mint(cpi_context, 0, &ctx.accounts.payer.key(), Some(&ctx.accounts.payer.key()))?;

    // Create associated token account
    let cpi_context = CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        token::InitializeAccount3 {
            account: ctx.accounts.token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token::initialize_account3(cpi_context)?;

    // Mint token
    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token::mint_to(cpi_context, 1)?;

    // Create metadata account
    let creators = vec![
        Creator {
            address: ctx.accounts.payer.key(),
            verified: true,
            share: 100,
        },
    ];

    let data_v2 = DataV2 {
        name: metadata_title,
        symbol: metadata_symbol,
        uri: metadata_uri,
        seller_fee_basis_points: 500,
        creators: Some(creators),
        collection: Some(Collection {
            verified: false,
            key: state.collection_mint,
        }),
        uses: None,
    };

    let accounts = mpl_instruction::CreateMetadataAccountV3 {
        metadata: ctx.accounts.metadata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        mint_authority: ctx.accounts.payer.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        update_authority: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };

    mpl_instruction::create_metadata_accounts_v3(
        CpiContext::new(ctx.accounts.token_metadata_program.to_account_info(), accounts),
        data_v2,
        true,
        true,
        None,
    )?;

    state.minted_count += 1;

    if state.pending_nfts.len() as u64 <= state.min_buffer_threshold {
        emit!(GenerateMoreNFTsEvent {
            current_buffer_size: state.pending_nfts.len() as u64,
            required_size: state.buffer_size,
        });
    }

    Ok(())
}

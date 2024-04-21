//! CreateTree instruction handler

use crate::error::SkyTradeCompressedNftError;
use anchor_lang::prelude::*;
use mpl_bubblegum::accounts::TreeConfig;
use mpl_bubblegum::instructions::{
    CreateTreeConfigCpi, CreateTreeConfigCpiAccounts, CreateTreeConfigCpiBuilder,
    CreateTreeConfigInstructionArgs,
};
use spl_account_compression::Noop;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Clone)]
pub struct MplBubblegum;

impl anchor_lang::Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::ID
    }
}

#[derive(Clone)]
pub struct SPLCompression;

impl anchor_lang::Id for SPLCompression {
    fn id() -> Pubkey {
        spl_account_compression::id()
        //Pubkey::from_str("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK").unwrap()
    }
}

/* #[derive(Clone)]
pub struct Noop;

impl Id for Noop {
    fn id() -> Pubkey {
        Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap()
    }
} */

// new
/* #[derive(Clone)]
pub struct TreeConfigAnchor(pub TreeConfig);

impl AccountDeserialize for TreeConfigAnchor {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> Result<Self> {
        Ok(Self(TreeConfig::from_bytes(buf)?))
    }
}

impl anchor_lang::Owner for TreeConfigAnchor {
    fn owner() -> Pubkey {
        // pub use spl_token::ID is used at the top of the file
        mpl_bubblegum::ID
    }
}

// No-op since we can't write data to a foreign program's account.
impl AccountSerialize for TreeConfigAnchor {}

impl Deref for TreeConfigAnchor {
    type Target = TreeConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} */

//#[derive(Accounts)]
#[derive(Accounts)]
#[instruction(params: CreateTreeParams)]
pub struct CreateTree<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [b"auth"],
        bump,
    )]
    /// CHECK: This account is checked in the instruction
    pub pda: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = bubblegum_program.key()
    )]
    /// CHECK: This account is checked in the instruction
    pub tree_authority: UncheckedAccount<'info>,
    //pub tree_authority: Account<'info, TreeConfigAnchor>,
    #[account(mut)]
    /// CHECK: This account is checked in the instruction
    pub merkle_tree: UncheckedAccount<'info>,
    pub log_wrapper: Program<'info, Noop>,
    pub compression_program: Program<'info, SPLCompression>,
    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateTreeParams {
    // max_depth - used to compute the maximum number of leaves
    // i.e Compressed NFTs that the Merkle Tree can hold
    pub max_depth: u32,
    // max_buffer_size - indicates the minimum concurrency limit of the Merkle Tree
    pub max_buffer_size: u32,
    pub public: bool,
}

pub fn create_tree(ctx: Context<CreateTree>, params: &CreateTreeParams) -> Result<()> {
    msg!("Validate inputs");

    if params.max_depth == 0 {
        return Err(SkyTradeCompressedNftError::InvalidMaxDepth.into());
    }

    if params.max_buffer_size == 0 {
        return Err(SkyTradeCompressedNftError::InvalidMaxBufferSize.into());
    }

    let public = Some(params.public);
    let max_depth = params.max_depth;
    let max_buffer_size = params.max_buffer_size;

    let signer_seeds: &[&[&[u8]]] = &[&[b"auth", &[ctx.bumps.pda]]];

    // instruction accounts
    let bubblegum_program = ctx.accounts.bubblegum_program.to_account_info();
    let tree_config = ctx.accounts.tree_authority.to_account_info();
    let merkle_tree = ctx.accounts.merkle_tree.to_account_info();
    let payer = ctx.accounts.payer.to_account_info();
    let tree_creator = ctx.accounts.pda.to_account_info(); // set creator as pda
    let log_wrapper = ctx.accounts.log_wrapper.to_account_info();
    let compression_program = ctx.accounts.compression_program.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();

    let cpi_create_tree_config = CreateTreeConfigCpi::new(
        &bubblegum_program,
        CreateTreeConfigCpiAccounts {
            tree_config: &tree_config,
            merkle_tree: &merkle_tree,
            payer: &payer,
            tree_creator: &tree_creator,
            log_wrapper: &log_wrapper,
            compression_program: &compression_program,
            system_program: &system_program,
        },
        CreateTreeConfigInstructionArgs {
            max_depth,
            max_buffer_size,
            public,
        },
    );

    /* let xy = false;
    let mut cpi_create_tree_config = CreateTreeConfigCpiBuilder::new(&bubblegum_program);
    cpi_create_tree_config
        .tree_config(&tree_config)
        .merkle_tree(&merkle_tree)
        .payer(&payer)
        .tree_creator(&tree_creator)
        .log_wrapper(&log_wrapper)
        .compression_program(&compression_program)
        .system_program(&system_program)
        .max_depth(max_depth)
        .max_buffer_size(max_buffer_size)
        .public(xy); */

    // performs the CPI
    let _result = cpi_create_tree_config.invoke_signed(signer_seeds);

    Ok(())
}

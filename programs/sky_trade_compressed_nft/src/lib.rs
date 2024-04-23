//! sky_trade_compressed_nft program entrypoint

pub mod error;
pub mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("9eyVuCaP3qactwU5ihEoUwWEXFm1F1rqz9mozPXxLAvo");

#[program]
pub mod sky_trade_compressed_nft {
    use super::*;

    // admin instructions
    pub fn create_tree(ctx: Context<CreateTree>, params: CreateTreeParams) -> Result<()> {
        instructions::create_tree(ctx, &params)
    }

    pub fn mint_cnft(ctx: Context<MintCNft>, params: MintCNftParams) -> Result<()> {
        instructions::mint_cnft(ctx, &params)
    }
}

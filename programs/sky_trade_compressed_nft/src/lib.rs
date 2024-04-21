//! sky_trade_compressed_nft program entrypoint

pub mod error;
pub mod instructions;

use {anchor_lang::prelude::*, instructions::*};

declare_id!("Anv8YYd1tp3Hx1JKV5GxhfqdHPVNr28J2VJubCvCn5YP");

#[program]
pub mod sky_trade_compressed_nft {
    use super::*;

    // admin instructions
    pub fn create_tree(ctx: Context<CreateTree>, params: CreateTreeParams) -> Result<()> {
        instructions::create_tree(ctx, &params)
    }
}

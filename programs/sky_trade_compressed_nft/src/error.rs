use anchor_lang::prelude::*;

#[error_code]
pub enum SkyTradeCompressedNftError {
    #[msg("Max depth has invalid value.")]
    InvalidMaxDepth,
    #[msg("Max buffer size has invalid value.")]
    InvalidMaxBufferSize,
}

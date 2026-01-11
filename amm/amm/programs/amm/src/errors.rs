use anchor_lang::prelude::{borsh::de, *};

#[error_code]
pub enum Error {
    #[msg("Invalid Fee")]
    InvalidFee,
    #[msg("Invalid mint id ")]
    InvalidMintId,
}

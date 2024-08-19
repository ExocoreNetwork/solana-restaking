use anchor_lang::prelude::error_code;

#[error_code]
pub enum LstRestakingError {
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("The owner of mint is not token program id")]
    InvalidMintOwner,
    #[msg("Mint is already exists")]
    MintAlreadyExists,
    #[msg("Invalid new owner")]
    InvalidNewOwner,
}

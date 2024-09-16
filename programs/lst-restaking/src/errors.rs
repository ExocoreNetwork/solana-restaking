use anchor_lang::prelude::error_code;

#[error_code]
pub enum LstRestakingError {
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid operator")]
    InvalidOperator,
    #[msg("The owner of mint is not token program id")]
    InvalidMintOwner,
    #[msg("Mint is already exists")]
    MintAlreadyExists,
    #[msg("Mint is not exists")]
    MintNotExists,
    #[msg("Invalid new owner")]
    InvalidNewOwner,
    #[msg("Not support mint")]
    NotSupportMint,
    #[msg("Invalid message list account")]
    InvalidMessageList,
    #[msg("Invalid tokens account")]
    InvalidTokens

}

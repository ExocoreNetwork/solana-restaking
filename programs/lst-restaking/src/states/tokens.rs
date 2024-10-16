use crate::errors::LstRestakingError;
use anchor_lang::prelude::*;
use std::mem::size_of;

#[account]
#[derive(InitSpace)]
pub struct Tokens {
    #[max_len(10)]
    tokens: Vec<Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Token {
    mint: Pubkey,
    // program: Pubkey,
    tvl_limit: u128,
    total_balances: u128
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, InitSpace)]
pub enum Action {
    Add,
    ResetTvlLimit,
}

impl Tokens {
    pub const SEED: &'static [u8] = b"tokenWhiteList";

    pub fn get_size(&self) -> usize {
        8 + 4 + (self.tokens.len() + 1) * size_of::<Token>()
    }

    pub fn update_token_info(&mut self, mint: Pubkey, tvl_limit: u128, action: Action) -> Result<()> {
        match action {
            Action::Add => {
                if let Some(_) = self.tokens.iter_mut().find(|t| t.mint == mint) {
                    return Err(LstRestakingError::MintAlreadyExists.into());
                } else {
                    self.tokens.push(Token { mint, tvl_limit, total_balances: 0 })
                }
            }
            Action::ResetTvlLimit => {
                if let Some(token) = self.tokens.iter_mut().find(|t| t.mint == mint) {
                    token.tvl_limit = tvl_limit
                } else {
                    return Err(LstRestakingError::MintNotExists.into());
                }
            }
        }

        Ok(())
    }

    pub fn validate_mint(&self, mint: &Pubkey) -> Result<bool> {
        Ok(self
            .tokens
            .iter()
            .any(|t| t.mint == *mint && t.tvl_limit > 0))
    }
}

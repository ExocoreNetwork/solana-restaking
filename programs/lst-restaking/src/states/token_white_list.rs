use std::mem;
use anchor_lang::prelude::*;
use crate::errors::LstRestakingError;

#[account]
#[derive(InitSpace)]
pub struct TokenWhiteList {
    #[max_len(0)]
    tokens: Vec<Token>
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Token {
    mint: Pubkey,
    effective: bool
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, InitSpace)]
pub enum Action {
    Add,
    Deactivate,
}

impl TokenWhiteList {
    pub const SEED: &'static [u8] = b"tokenWhiteList";

    pub fn get_size(&self) -> usize {
        8 + 4 + self.tokens.len() + mem::size_of::<Token>()
    }
    pub fn update_white_list(&mut self, mint: Pubkey, action: Action) -> Result<()> {
        match action {
            Action::Add => {
                if let Some(token) = self.tokens.iter_mut().find(|t| t.mint == mint) {
                    if token.effective {
                        return Err(LstRestakingError::MintAlreadyExists.into());
                    }
                    token.effective = true
                } else {
                    self.tokens.push(Token { mint, effective: true })
                }
            }
            Action::Deactivate => {
                if let Some(token) = self.tokens.iter_mut().find(|t| t.mint == mint) {
                    token.effective = false
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
            .any(|t| t.mint == *mint && t.effective == true))
    }
}
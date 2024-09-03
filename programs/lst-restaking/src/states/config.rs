use crate::errors::LstRestakingError;
use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub owner: Pubkey,
    pub pending_owner: Pubkey,
    pub white_list_tokens: Vec<Token>,
    pub remote_eid: u32,
    pub receiver: [u8; 32],
    pub _padding: Vec<u8>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Token {
    mint: Pubkey,
    active: bool,
}

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone)]
pub enum Action {
    Add,
    Deactivate,
}

impl Config {
    pub const CONFIG_SEED_PREFIX: &'static [u8; 6] = b"config";
    pub const POOL_SEED_PREFIX: &'static [u8; 4] = b"pool";

    //  the count of token when initialize is 10
    pub const LEN: usize = 32 + 32 + 16 + 4 + (32 + 1) * 10 + 4 + 500;

    pub fn update_white_list(&mut self, mint: Pubkey, action: Action) -> Result<()> {
        match action {
            Action::Add => {
                if let Some(token) = self.white_list_tokens.iter_mut().find(|t| t.mint == mint) {
                    if token.active {
                        return Err(LstRestakingError::MintAlreadyExists.into());
                    }
                    token.active = true
                } else {
                    self.white_list_tokens.push(Token { mint, active: true })
                }
            }
            Action::Deactivate => {
                if let Some(token) = self.white_list_tokens.iter_mut().find(|t| t.mint == mint) {
                    token.active = false
                } else {
                    return Err(LstRestakingError::MintNotExists.into());
                }
            }
        }

        Ok(())
    }

    pub fn validate_mint(&self, mint: &Pubkey) -> Result<bool> {
        Ok(self
            .white_list_tokens
            .iter()
            .any(|t| t.mint == *mint && t.active == true))
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    pub fn test_len() {
        assert_eq!(
            Config::LEN,
            size_of::<Pubkey>()
                + size_of::<Pubkey>()
                + 4
                + size_of::<Pubkey>() * 10
                + 4
                + size_of::<u8>() * 500
        )
    }
}

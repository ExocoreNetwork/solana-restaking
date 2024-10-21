use crate::errors::LstRestakingError;
use anchor_lang::prelude::*;
use anchor_lang::Discriminator;

#[account]
#[derive(InitSpace)]
pub struct Token {
    pub mint: Pubkey,
    pub tvl_limit: u128,
    pub consumed_tvl: u128
}

impl Token {
    pub fn write(new_account: &AccountInfo, mint: &Pubkey, tvl_limit: u128) -> Result<()> {
        let mut account_data = new_account.try_borrow_mut_data()?;

        msg!("data: {:?}, {}", account_data, account_data.len());

        if account_data.len() < (8 + Token::INIT_SPACE) {
            return Err(LstRestakingError::AccountDataTooSmall.into());
        }

        if account_data.iter().all(|&x| x == 0) {
            msg!("Initializing new token account data");

            let account = Token {
                mint: *mint,
                tvl_limit,
                consumed_tvl: 0
            };

            let discriminator = Token::discriminator();
            account_data[..8].copy_from_slice(&discriminator);

            account.serialize(&mut &mut account_data[8..])?;
        }
        Ok(())
    }

    pub fn update_tvl_limit(&mut self, tvl_limit: u128) {
        self.tvl_limit = tvl_limit
    }

    pub fn increase_consumed_tvl(&mut self, consumed_tvl: u128) {
        self.consumed_tvl += consumed_tvl
    }

    pub fn decrease_consumed_tvl(&mut self, consumed_tvl: u128) {
        self.consumed_tvl -= consumed_tvl
    }

}


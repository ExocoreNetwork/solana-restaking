use anchor_lang::prelude::*;

use crate::{errors::LstRestakingError, states::Config};

pub fn accept(ctx: Context<Accept>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let new_owner_key = ctx.accounts.new_owner.key();

    require_keys_eq!(
        config.pending_owner,
        new_owner_key,
        LstRestakingError::InvalidNewOwner
    );

    config.owner = new_owner_key;

    Ok(())
}

#[derive(Accounts)]
pub struct Accept<'info> {
    #[account(mut)]
    new_owner: Signer<'info>,
    #[account(mut)]
    config: Account<'info, Config>,
}

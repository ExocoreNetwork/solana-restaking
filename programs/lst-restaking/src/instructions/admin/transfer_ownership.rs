use anchor_lang::prelude::*;

use crate::{errors::LstRestakingError, states::Config};

pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    require_keys_eq!(
        config.owner,
        ctx.accounts.owner.key(),
        LstRestakingError::InvalidOwner
    );

    config.pending_owner = ctx.accounts.new_owner.key();

    msg!("Successful to transfer ownership");

    Ok(())
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    config: Account<'info, Config>,
    new_owner: SystemAccount<'info>,
}

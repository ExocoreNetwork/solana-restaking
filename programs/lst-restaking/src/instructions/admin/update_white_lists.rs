use std::any::Any;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{errors::LstRestakingError, states::Config};

pub fn update_white_lists(ctx: Context<UpdateWhiteList>) -> Result<()> {
    let mint_owner = ctx.accounts.mint.to_account_info().owner;
    let token_program_id = ctx.accounts.token_program.key();
    require_keys_eq!(
        *mint_owner,
        token_program_id,
        LstRestakingError::InvalidMintOwner
    );

    let config = &mut ctx.accounts.config;

    let mint = ctx.accounts.mint.key();

    if config.white_lists_token.iter().any(|m| m.eq(&mint)) {
        return Err(LstRestakingError::MintAlreadyExists.into());
    }

    config.white_lists_token.push(mint);

    msg!("Successful to update white lists, token: {}");
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateWhiteList<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    config: Account<'info, Config>,
    mint: Box<InterfaceAccount<'info, Mint>>,
    token_program: Interface<'info, TokenInterface>,
}

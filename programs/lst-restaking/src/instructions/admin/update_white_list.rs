use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{errors::LstRestakingError, states::Config};
use crate::states::Action;
use crate::utils::create_token_account;

pub fn update_white_list(ctx: Context<UpdateWhiteList>, action: Action) -> Result<()> {
    let mint_owner = ctx.accounts.mint.to_account_info().owner;
    let token_program_id = ctx.accounts.token_program.key();
    require_keys_eq!(
        *mint_owner,
        token_program_id,
        LstRestakingError::InvalidMintOwner
    );

    let config = &mut ctx.accounts.config;

    let mint = ctx.accounts.mint.key();

    config.update_white_list(mint, action)?;

    // TODO: create pool token account if add
    // let signer = &[Config::CONFIG_SEED_PREFIX, token_program_id.as_ref(), mint.as_ref(), &[ctx.bumps.config][..]];
    //
    // match action {
    //     Action::Add => {
    //         if ctx.accounts.pool_token_account.try_data_is_empty().unwrap_or(true) {
    //             create_token_account(
    //                 &ctx.accounts.config.to_account_info(),
    //                 &ctx.accounts.operator.to_account_info(),
    //                 &ctx.accounts.pool_token_account.to_account_info(),
    //                 &ctx.accounts.mint.to_account_info(),
    //                 &ctx.accounts.system_program.to_account_info(),
    //                 &ctx.accounts.token_program.to_account_info(),
    //                 &[signer],
    //             )?;
    //         }
    //     }
    //     _ => {}
    // }

    msg!("Successful to update white lists, token: {}", mint);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateWhiteList<'info> {
    #[account(mut)]
    operator: Signer<'info>,
    #[account(mut,
    seeds = [Config::CONFIG_SEED_PREFIX],
    bump
    )]
    config: Account<'info, Config>,
    mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: create it if add
    #[account(
        init_if_needed,
        payer = operator,
        associated_token::mint = mint,
        associated_token::authority = config
    )]
    pool_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}


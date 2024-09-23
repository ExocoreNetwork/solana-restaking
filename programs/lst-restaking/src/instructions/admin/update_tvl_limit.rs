use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint};

use crate::states::{Action, Tokens};
use crate::{errors::LstRestakingError, states::Config};
// use crate::utils::create_token_account;

pub fn update_tvl_limit(ctx: Context<UpdateTvlLimit>, params: UpdateTvlLimitParams) -> Result<()> {
    let tokens = &mut ctx.accounts.tokens;

    let mint = ctx.accounts.mint.key();

    tokens.validate_mint(&mint)?;

    tokens.update_token_info(mint, params.tvl_limit, Action::ResetTvlLimit)?;

    msg!("Successful to update token: {}, tvl_limit: {}", mint, params.tvl_limit);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateTvlLimit<'info> {
    #[account(mut)]
    operator: Signer<'info>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
        has_one = tokens @ LstRestakingError::InvalidTokens,
        has_one = operator @ LstRestakingError::InvalidOperator
    )]
    config: Account<'info, Config>,
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
    )]
    tokens: Box<Account<'info, Tokens>>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateTvlLimitParams {
    tvl_limit: u128,
}

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint};

use crate::{errors::LstRestakingError, states::Config};
use crate::states::Token;
use crate::{CONFIG_SEEDS_PREFIX, TOKEN_SEEDS_PREFIX};

pub fn update_tvl_limit(ctx: Context<UpdateTvlLimit>, params: UpdateTvlLimitParams) -> Result<()> {
    let token = &mut ctx.accounts.token;

    token.update_tvl_limit(params.tvl_limit);

    msg!("Successful to update token: {}, tvl_limit: {}", token.mint, params.tvl_limit);
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateTvlLimit<'info> {
    #[account(mut)]
    operator: Signer<'info>,
    #[account(
        mut,
        seeds = [CONFIG_SEEDS_PREFIX],
        bump,
        has_one = operator @ LstRestakingError::InvalidOperator
    )]
    config: Account<'info, Config>,
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [TOKEN_SEEDS_PREFIX, mint.key().as_ref()],
        bump
    )]
    token: Box<Account<'info, Token>>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateTvlLimitParams {
    tvl_limit: u128,
}

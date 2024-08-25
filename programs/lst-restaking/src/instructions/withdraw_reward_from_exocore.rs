use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use crate::errors::LstRestakingError;
use crate::states::{Config, Messages, Vault};

pub fn withdraw_reward_from_exocore(ctx: Context<WithdrawReward>) -> Result<()> {
    // validate mint
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint.key();

    require!(config.validate_mint(mint)?, LstRestakingError::NotSupportMint);

    config.nonce += 1;

    // TODO: send message to exocore


    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawReward<'info> {
    #[account(mut)]
    depositor: Signer<'info>,
    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX, mint.key().as_ref(), depositor.key().as_ref()],
        bump
    )]
    vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [Message::SEED_PREFIX, config.key().as_ref()] ,
        bump
    )]
    message: Account<'info, Messages>,
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump
    )]
    config: Account<'info, Config>,
}
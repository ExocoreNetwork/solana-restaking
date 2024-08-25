use crate::states::Config;
use anchor_lang::prelude::*;

pub fn initialize(ctx: Context<InitConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.owner = ctx.accounts.owner.key();
    config.pending_owner = ctx.accounts.owner.key();
    config.nonce = 0;

    msg!("Successful to initialize config");

    Ok(())
}

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
        space = 8 + Config::LEN
    )]
    config: Account<'info, Config>,
    system_program: Program<'info, System>,
}

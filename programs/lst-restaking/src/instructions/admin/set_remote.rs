use anchor_lang::prelude::*;
use crate::Config;

#[derive(Accounts)]
#[instruction(params: SetRemoteParams)]
pub struct SetRemote<'info> {
    #[account(mut,
        address = config.owner
    )]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    pub system_account: Program<'info, System>,
}

impl SetRemote<'_> {
    pub fn apply(ctx: &mut Context<SetRemote>, params: &SetRemoteParams) -> Result<()> {
        ctx.accounts.config.receiver = params.remote;
        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SetRemoteParams {
    remote: [u8; 32]
}

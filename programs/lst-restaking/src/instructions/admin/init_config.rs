use anchor_lang::prelude::*;
use crate::states::{Config, LzReceiveTypesAccount, Messages, Tokens};
use oapp::endpoint::instructions::RegisterOAppParams;
use oapp::endpoint::program::Endpoint;
use oapp::endpoint_cpi;
#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
        space = 8 + Config::INIT_SPACE
    )]
    config: Account<'info, Config>,
    #[account(
        init,
        payer = owner,
        seeds = [Messages::MESSAGE_SEED_PREFIX, config.key().as_ref()],
        bump,
        space = 8 + Messages::INIT_SPACE
    )]
    messages: Account<'info, Messages>,
    #[account(
        init,
        payer = owner,
        space = 8 + LzReceiveTypesAccount::INIT_SPACE,
        seeds = [Config::LZ_RECEIVE_TYPES_SEED, &config.key().as_ref()],
        bump
    )]
    lz_receive_types: Account<'info, LzReceiveTypesAccount>,
    #[account(
        init,
        payer = owner,
        space = 8 + Tokens::INIT_SPACE,
        seeds = [Tokens::SEED],
        bump
    )]
    tokens: Account<'info, Tokens>,
    operator: SystemAccount<'info>,
    delegate: SystemAccount<'info>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

impl InitConfig<'_> {
    pub fn apply(ctx: &mut Context<InitConfig>, params: &InitConfigParams) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.owner = ctx.accounts.owner.key();
        config.pending_owner = ctx.accounts.owner.key();

        config.eid = params.dst_eid;
        config.receiver = params.receiver;
        config.messages = ctx.accounts.messages.key();
        config.tokens = ctx.accounts.tokens.key();
        config.endpoint_program = ctx.accounts.endpoint_program.key();
        config.operator = ctx.accounts.operator.key();

        config.bump = ctx.bumps.config;

        msg!("receiver: {:?}", config.receiver);

        ctx.accounts.lz_receive_types.config = config.key();
        ctx.accounts.lz_receive_types.messages = ctx.accounts.messages.key();

        let signer = &[Config::CONFIG_SEED_PREFIX, &[ctx.bumps.config][..]];

        endpoint_cpi::register_oapp(
            ctx.accounts.endpoint_program.key(),
            ctx.accounts.config.key(),
            ctx.remaining_accounts,
            signer,
            RegisterOAppParams {
                delegate: ctx.accounts.delegate.key(),
            },
        )?;

        msg!("Successful to initialize config");

        Ok(())
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitConfigParams {
    dst_eid: u32,
    receiver: [u8; 32]
}

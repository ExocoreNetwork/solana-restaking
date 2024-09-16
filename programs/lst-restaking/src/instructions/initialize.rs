use crate::states::{Config, LzReceiveTypesAccount, MessageList, Tokens};
use anchor_lang::prelude::*;
use oapp::endpoint::instructions::RegisterOAppParams;
use oapp::endpoint::program::Endpoint;
use oapp::endpoint_cpi;

pub fn initialize(ctx: Context<InitConfig>, params: InitConfigParams) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.owner = ctx.accounts.owner.key();
    config.pending_owner = ctx.accounts.owner.key();

    config.remote_eid = params.remote_eid;
    config.receiver = params.receiver;
    config.message_list = ctx.accounts.message_list.key();
    config.tokens = ctx.accounts.tokens.key();
    config.endpoint_program = ctx.accounts.endpoint_program.key();
    config.operator = ctx.accounts.operator.key();

    config.bump = ctx.bumps.config;

    msg!("receiver: {:?}", config.receiver);

    ctx.accounts.lz_receive_types.config = config.key();

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
        seeds = [MessageList::MESSAGE_SEED_PREFIX, config.key().as_ref()],
        bump,
        space = 8 + MessageList::INIT_SPACE
    )]
    message_list: Account<'info, MessageList>,
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitConfigParams {
    remote_eid: u32,
    receiver: [u8; 32]
}

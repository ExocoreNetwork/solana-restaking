use crate::states::{Config, LzReceiveTypes, MessageList, TokenWhiteList};
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
    config.token_white_list = ctx.accounts.token_white_list.key();

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
        space = 8 + LzReceiveTypes::INIT_SPACE,
        seeds = [Config::LZ_RECEIVE_TYPES_SEED, &config.key().as_ref()],
        bump
    )]
    lz_receive_types: Account<'info, LzReceiveTypes>,
    #[account(
        init,
        payer = owner,
        space = 8 + TokenWhiteList::INIT_SPACE,
        seeds = [TokenWhiteList::SEED],
        bump
    )]
    token_white_list: Account<'info, TokenWhiteList>,
    delegate: SystemAccount<'info>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitConfigParams {
    remote_eid: u32,
    receiver: [u8; 32]
}

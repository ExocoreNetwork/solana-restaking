use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use oapp::endpoint::instructions::SendParams;
use oapp::endpoint::program::Endpoint;
use oapp::endpoint_cpi::send;
use crate::states::{Config, MessageList, MessageWithOperator, RequestAction, Vault};
use crate::utils::encode;

pub fn delegate_to(ctx: Context<DelegateTo>, params: DelegateToParams) -> Result<()> {
    let config = &ctx.accounts.config;

    let message = encode(RequestAction::DelegateTo(
        MessageWithOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            operator: params.operator,
            amount: params.amount
        }
    ))?;

    let signer = &[Config::CONFIG_SEED_PREFIX, &[ctx.bumps.config][..]];

    let dst_eid = config.remote_eid;
    let receiver = config.receiver;

    send(
        ctx.accounts.endpoint_program.key(),
        ctx.accounts.config.key(),
        ctx.remaining_accounts,
        signer,
        SendParams {
            dst_eid,
            receiver,
            message,
            options: vec![],
            native_fee: 500000,
            lz_token_fee: 0,
        })?;

    Ok(())
}


#[derive(Accounts)]
pub struct DelegateTo<'info> {
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
        seeds = [MessageList::MESSAGE_SEED_PREFIX, config.key().as_ref()] ,
        bump
    )]
    message_list: Account<'info, MessageList>,
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump
    )]
    config: Account<'info, Config>,
    endpoint_program: Program<'info, Endpoint>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DelegateToParams {
    operator: [u8; 32],
    amount: u64
}
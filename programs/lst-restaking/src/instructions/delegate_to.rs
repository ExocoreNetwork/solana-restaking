use std::mem;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use oapp::endpoint::program::Endpoint;
use crate::states::{Config, MessageWithOperator, RequestAction, Token, Vault};
use crate::utils::{send};
use crate::{VAULT_SEEDS_PREFIX, TOKEN_SEEDS_PREFIX, CONFIG_SEEDS_PREFIX};

use crate::errors::LstRestakingError;

pub fn delegate_to(ctx: Context<DelegateTo>, params: DelegateToParams) -> Result<()> {
    let action = RequestAction::DelegateTo(
        MessageWithOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            operator: params.operator,
            amount: params.amount
        }
    );

    let mut message = Vec::with_capacity(1 + mem::size_of::<MessageWithOperator>());
    action.encode(&mut message)?;

    msg!("message: {:?}", message);

    let _ = send(
        ctx.accounts.endpoint_program.key(),
        ctx.accounts.config.key(),
        ctx.remaining_accounts,
        ctx.bumps.config,
        message,
        params.opts.clone(),
        ctx.accounts.config.eid,
        ctx.accounts.config.receiver
    )?;

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
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
    )]
    config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [TOKEN_SEEDS_PREFIX, mint.key().as_ref()],
        bump
    )]
    token: Box<Account<'info, Token>>,
    endpoint_program: Program<'info, Endpoint>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DelegateToParams {
    operator: [u8; 32],
    amount: u64,
    opts: Vec<u8>
}
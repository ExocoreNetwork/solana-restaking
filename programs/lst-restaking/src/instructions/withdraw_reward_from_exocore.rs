use std::mem;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use oapp::endpoint::program::Endpoint;

use crate::{MESSAGE_SEEDS_PREFIX, TOKEN_SEEDS_PREFIX, CONFIG_SEEDS_PREFIX, VAULT_SEEDS_PREFIX};
use crate::states::{Config, Message, MessageWithoutOperator, RequestAction, Token, Vault};
use crate::utils::{create_pda, get_message_seeds, send};

pub fn withdraw_reward_from_exocore(ctx: Context<WithdrawReward>, params: WithdrawRewardParams) -> Result<()> {
    // TODO: check balance

    let action = RequestAction::ClaimReward(
        MessageWithoutOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            amount: params.amount_out,
        }
    );

    let mut message = Vec::with_capacity(1 + mem::size_of::<MessageWithoutOperator>());
    action.encode(&mut message)?;

    let nonce = send(
        ctx.accounts.endpoint_program.key(),
        ctx.accounts.config.key(),
        ctx.remaining_accounts,
        ctx.bumps.config,
        message,
        params.opts.clone(),
        ctx.accounts.config.eid,
        ctx.accounts.config.receiver,
    )?;

    let seeds = get_message_seeds(&nonce);

    let seeds: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();

    let space = 8 + Message::INIT_SPACE;

    create_pda(&ctx.accounts.message.to_account_info(),
               &ctx.accounts.depositor.to_account_info(),
               &ctx.program_id,
               &seeds,
               space)?;

    Message::write(&ctx.accounts.message.to_account_info(), nonce, &action)?;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawReward<'info> {
    #[account(mut)]
    depositor: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_SEEDS_PREFIX, mint.key().as_ref(), depositor.key().as_ref()],
        bump
    )]
    vault: Box<Account<'info, Vault>>,
    /// CHECK: genarate PDA account
    #[account(mut)]
    message: UncheckedAccount<'info>,
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [CONFIG_SEEDS_PREFIX],
        bump,
    )]
    config: Box<Account<'info, Config>>,
    #[account(
        mut,
        seeds = [TOKEN_SEEDS_PREFIX, mint.key().as_ref()],
        bump
    )]
    token: Box<Account<'info, Token>>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawRewardParams {
    amount_out: u64,
    opts: Vec<u8>,
}
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use oapp::endpoint::program::Endpoint;
use crate::errors::LstRestakingError;
use crate::states::{Config, MessageList, MessageWithoutOperator, RequestAction, Tokens, Vault};
use crate::utils::{encode, send};

pub fn withdraw_reward_from_exocore(ctx: Context<WithdrawReward>, params: WithdrawRewardParams) -> Result<()> {
    // validate mint
    // let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint.key();

    let token_white_list = &ctx.accounts.tokens;

    require!(token_white_list.validate_mint(mint)?, LstRestakingError::NotSupportMint);

    let message = encode(RequestAction::WithdrawRewardFromExocore(
        MessageWithoutOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            amount: params.amount_out
        }
    ))?;

    let nonce = send(
        ctx.accounts.endpoint_program.key(),
        ctx.accounts.config.key(),
        ctx.remaining_accounts,
        ctx.bumps.config,
        message,
        params.opts.clone()
    )?;

    let message_list = &mut ctx.accounts.message_list;

    message_list.pending(nonce,
                         RequestAction::WithdrawRewardFromExocore(
                             MessageWithoutOperator {
                                 mint: ctx.accounts.mint.key(),
                                 sender: ctx.accounts.depositor.key(),
                                 amount: params.amount_out,
                             }))?;

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
    vault: Box<Account<'info, Vault>>,
    #[account(
        mut,
        realloc = message_list.get_size(),
        realloc::payer = depositor,
        realloc::zero = false
    )]
    message_list: Box<Account<'info, MessageList>>,
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
        has_one = message_list @LstRestakingError::InvalidMessageList,
        has_one = tokens @ LstRestakingError::InvalidTokens
    )]
    config: Box<Account<'info, Config>>,
    tokens: Box<Account<'info, Tokens>>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawRewardParams {
    amount_out: u64,
    opts: Vec<u8>
}
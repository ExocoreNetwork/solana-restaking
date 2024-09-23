use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use oapp::endpoint::program::Endpoint;
use crate::states::{Config, MessageWithOperator, RequestAction, Tokens, Vault};
use crate::utils::{encode, send};
use crate::errors::LstRestakingError;

pub fn delegate_to(ctx: Context<DelegateTo>, params: DelegateToParams) -> Result<()> {
    let token_white_list = &ctx.accounts.tokens;
    let mint = &ctx.accounts.mint.key();

    require!(
        token_white_list.validate_mint(mint)?,
        LstRestakingError::NotSupportMint
    );

    let message = encode(RequestAction::DelegateTo(
        MessageWithOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            operator: params.operator,
            amount: params.amount
        }
    ))?;

    let _ = send(
        ctx.accounts.endpoint_program.key(),
        ctx.accounts.config.key(),
        ctx.remaining_accounts,
        ctx.bumps.config,
        message,
        params.opts.clone()
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
        has_one = tokens @ LstRestakingError::InvalidTokens
    )]
    config: Account<'info, Config>,
    tokens: Box<Account<'info, Tokens>>,
    endpoint_program: Program<'info, Endpoint>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DelegateToParams {
    operator: [u8; 32],
    amount: u64,
    opts: Vec<u8>
}
use crate::errors::LstRestakingError;
use crate::states::{Config, MessageWithoutOperator, RequestAction, Tokens, Vault};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use oapp::endpoint::program::Endpoint;
use crate::utils::{encode, send};

pub fn deposit(ctx: Context<Deposit>, params: DepositParams) -> Result<()> {
    // validate mint
    // let config = &ctx.accounts.config;
    let mint = &ctx.accounts.mint.key();

    let token_white_list = &ctx.accounts.tokens;

    require!(
        token_white_list.validate_mint(mint)?,
        LstRestakingError::NotSupportMint
    );

    // transfer
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.depositor_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        params.amount_in,
        ctx.accounts.mint.decimals,
    )?;

    // update total balance
    let vault = &mut ctx.accounts.vault;
    vault.deposit_balance += params.amount_in;

    let message = encode(RequestAction::Deposit(
        MessageWithoutOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            amount: params.amount_in
        }
    ))?;


    msg!("message: {:?}", message);

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
pub struct Deposit<'info> {
    #[account(mut)]
    depositor: Signer<'info>,
    #[account(
        init_if_needed,
        payer = depositor,
        seeds = [Vault::SEED_PREFIX, mint.key().as_ref(), depositor.key().as_ref()],
        bump,
        space = 8 + Vault::INIT_SPACE
    )]
    vault: Box<Account<'info, Vault>>,
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
        has_one = tokens @ LstRestakingError::InvalidTokens
    )]
    config: Box<Account<'info, Config>>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = depositor
    )]
    depositor_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = config
    )]
    pool_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    tokens: Box<Account<'info, Tokens>>,
    token_program: Interface<'info, TokenInterface>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DepositParams {
    amount_in: u64,
    opts: Vec<u8>
}
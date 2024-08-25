use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked};
use crate::errors::LstRestakingError;
use crate::states::{Config, Vault, Messages};

pub fn deposit_then_delegate_to(ctx: Context<DepositThenDelegateTo>, amount_in: u64) -> Result<()> {
    // validate mint
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint.key();

    require!(config.validate_mint(mint)?, LstRestakingError::NotSupportMint);

    config.nonce += 1;

    // transfer
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.depositor_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.pool_token_account.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info()
            },
        ),
        amount_in,
        ctx.accounts.mint.decimals
    )?;

    // update total balance
    let vault = &mut ctx.accounts.vault;
    vault.deposit_balance += amount_in;

    // TODO: send message to exocore

    Ok(())
}

#[derive(Accounts)]
pub struct DepositThenDelegateTo<'info> {
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
        seeds = [Message::SEED_PREFIX, config.key().as_ref()] ,
        bump
    )]
    message: Account<'info, Messages>,
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump
    )]
    config: Account<'info, Config>,
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
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}
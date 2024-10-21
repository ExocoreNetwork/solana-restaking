use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked};
use crate::errors::LstRestakingError;
use crate::states::{Config, Message, Token, Vault};
use crate::{CONFIG_SEEDS_PREFIX, TOKEN_SEEDS_PREFIX, MESSAGE_SEEDS_PREFIX,};

// TODO:
pub fn claim(ctx: Context<Claim>, amount_in: u64) -> Result<()> {
    // validate mint
    let mint = &ctx.accounts.mint.key();
    let claimer= &ctx.accounts.claimer.key();

    let signer = &[Vault::SEED_PREFIX, mint.as_ref(), claimer.as_ref(), &[ctx.bumps.vault]];

    // transfer
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.claimer_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                authority: ctx.accounts.claimer.to_account_info()
            },
            &[signer]
        ),
        amount_in,
        ctx.accounts.mint.decimals
    )?;

    // update total balance
    let vault = &mut ctx.accounts.vault;
    vault.deposit_balance -= amount_in;

    // TODO: send message to exocore

    Ok(())
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    claimer: Signer<'info>,
    #[account(
        mut,
        seeds = [Vault::SEED_PREFIX, mint.key().as_ref(), claimer.key().as_ref()],
        bump
    )]
    vault: Account<'info, Vault>,
    #[account(
        mut,
        seeds = [MESSAGE_SEEDS_PREFIX, config.key().as_ref()] ,
        bump
    )]
    message: Box<Account<'info, Message>>,
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [CONFIG_SEEDS_PREFIX],
        bump,
    )]
    config: Account<'info, Config>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = claimer
    )]
    claimer_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = config
    )]
    pool_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    token: Box<Account<'info, Token>>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}
use crate::errors::LstRestakingError;
use crate::states::{Config, MessageList, MessageWithoutOperator, RequestAction, Vault};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use oapp::endpoint::instructions::SendParams;
use oapp::endpoint::program::Endpoint;
use oapp::endpoint_cpi::send;
use crate::utils::encode;

pub fn deposit(ctx: Context<Deposit>, params: DepositParams) -> Result<()> {
    // validate mint
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint.key();

    require!(
        config.validate_mint(mint)?,
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
pub struct Deposit<'info> {
    #[account(mut)]
    depositor: Signer<'info>,
    #[account(
        init_if_needed,
        seeds = [Vault::SEED_PREFIX, mint.key().as_ref(), depositor.key().as_ref()],
        bump
    )]
    vault: Account<'info, Vault>,
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
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DepositParams {
    amount_in: u64
}
use crate::errors::LstRestakingError;
use crate::states::{Config, MessageList, MessageWithOperator, RequestAction, Tokens, Vault};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
use oapp::endpoint::program::Endpoint;
use crate::utils::{encode, send};

pub fn deposit_then_delegate_to(ctx: Context<DepositThenDelegateTo>, params: DepositThenDelegateToParams) -> Result<()> {
    // validate mint
    let token_white_list = &mut ctx.accounts.tokens;
    let mint = &ctx.accounts.mint.key();

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

    let message = encode(RequestAction::DepositThenDelegateTo(
        MessageWithOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            operator: params.operator,
            amount: params.amount_in
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

    // let message_list = &mut ctx.accounts.message_list;
    // message_list.pending(result.nonce, action);

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
        seeds = [MessageList::MESSAGE_SEED_PREFIX, config.key().as_ref()] ,
        bump
    )]
    message_list: Account<'info, MessageList>,
    #[account(mut)]
    mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
        has_one = tokens @ LstRestakingError::InvalidTokens
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
    tokens: Box<Account<'info, Tokens>>,
    token_program: Interface<'info, TokenInterface>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct DepositThenDelegateToParams {
    amount_in: u64,
    operator: [u8; 32],
    opts: Vec<u8>
}
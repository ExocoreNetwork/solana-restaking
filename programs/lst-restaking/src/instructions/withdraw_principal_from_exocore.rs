use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;
use oapp::endpoint::instructions::SendParams;
use oapp::endpoint::program::Endpoint;
use oapp::endpoint_cpi::send;

use crate::errors::LstRestakingError;
use crate::states::{Config, MessageList, MessageWithoutOperator, RequestAction, Vault};
use crate::utils::encode;

pub fn withdraw_principal_from_exocore(ctx: Context<WithdrawPrincipal>, params: WithdrawPrincipalParams) -> Result<()> {

    // validate mint
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint.key();

    require!(config.validate_mint(mint)?, LstRestakingError::NotSupportMint);

    let message = encode(RequestAction::WithdrawPrincipalFromExocore(
        MessageWithoutOperator {
            mint: ctx.accounts.mint.key(),
            sender: ctx.accounts.depositor.key(),
            amount: params.amount_out,
        }
    ))?;
    let signer = &[Config::CONFIG_SEED_PREFIX, &[ctx.bumps.config][..]];

    let dst_eid = config.remote_eid;
    let receiver = config.receiver;

    let result = send(
        ctx.accounts.endpoint_program.key(),
        ctx.accounts.config.key(),
        ctx.remaining_accounts,
        signer,
        SendParams {
            dst_eid,
            receiver,
            message,
            options: params.opts.clone(),
            native_fee: 500000,
            lz_token_fee: 0,
        })?;

    let message_list = &mut ctx.accounts.message_list;

    message_list.pending(result.nonce,
                         RequestAction::WithdrawPrincipalFromExocore(
                             MessageWithoutOperator {
                                 mint: ctx.accounts.mint.key(),
                                 sender: ctx.accounts.depositor.key(),
                                 amount: params.amount_out,
                             }))?;


    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawPrincipal<'info> {
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
        seeds = [MessageList::MESSAGE_SEED_PREFIX, config.key().as_ref()],
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
pub struct WithdrawPrincipalParams {
    amount_out: u64,
    opts: Vec<u8>
}
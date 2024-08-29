use crate::errors::LstRestakingError;
use crate::states::{Config, MessageList, Vault};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};
// use oapp::endpoint::endpoint::send;
use oapp::endpoint::instructions::SendParams;
use oapp::endpoint::program::Endpoint;
use oapp::endpoint::state::{EndpointSettings, MessageLibInfo, Nonce, SendLibraryConfig};
use oapp::endpoint_cpi::send;

pub fn deposit(ctx: Context<Deposit>, amount_in: u64) -> Result<()> {
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
        amount_in,
        ctx.accounts.mint.decimals,
    )?;

    // update total balance
    let vault = &mut ctx.accounts.vault;
    vault.deposit_balance += amount_in;

    // TODO: send message to exocore
    let result = send(
        ctx.accounts.endpoint_program.key(),
        Default::default(),
        &[],
        &[],
        SendParams {})?;
    // let result = send(
    //     CpiContext::new(
    //         ctx.accounts.endpoint_program.to_account_info(),
    //         Send {
    //             sender: ctx.accounts.depositor.to_account_info(),
    //             send_library_program: ctx.accounts.send_library_program.to_account_info(),
    //             send_library_config: ctx.accounts.send_library_config.to_account_info(),
    //             default_send_library_config: ctx
    //                 .accounts
    //                 .default_send_library_config
    //                 .to_account_info(),
    //             send_library_info: ctx.accounts.send_library_info.to_account_info(),
    //             endpoint: ctx.accounts.endpoint.to_account_info(),
    //             nonce: ctx.accounts.nonce.to_account_info(),
    //             event_authority: ctx.accounts.event_authority.to_account_info(),
    //             program: ctx.accounts.system_program.to_account_info(),
    //         },
    //     ),
    //     SendParams {
    //         dst_eid: 0,
    //         receiver: [],
    //         message: vec![],
    //         options: vec![],
    //         native_fee: 0,
    //         lz_token_fee: 0,
    //     },
    // )?;

    let message_list = &mut ctx.accounts.message_list;
    message_list.pending(result.nonce, action);

    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
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
    /// CHECK:
    pub send_library_program: UncheckedAccount<'info>,
    #[account(
        seeds = [SEND_LIBRARY_CONFIG_SEED, sender.key.as_ref(), &params.dst_eid.to_be_bytes()],
        bump = send_library_config.bump
    )]
    pub send_library_config: Account<'info, SendLibraryConfig>,
    #[account(
        seeds = [SEND_LIBRARY_CONFIG_SEED, &params.dst_eid.to_be_bytes()],
        bump = default_send_library_config.bump
    )]
    pub default_send_library_config: Account<'info, SendLibraryConfig>,
    /// The PDA signer to the send library when the endpoint calls the send library.
    #[account(
        seeds = [
        MESSAGE_LIB_SEED,
        &get_send_library(
        &send_library_config,
        &default_send_library_config
        ).key().to_bytes()
        ],
        bump = send_library_info.bump,
        constraint = !send_library_info.to_account_info().is_writable @LayerZeroError::ReadOnlyAccount
    )]
    pub send_library_info: Account<'info, MessageLibInfo>,
    #[account(seeds = [ENDPOINT_SEED], bump = endpoint.bump)]
    pub endpoint: Account<'info, EndpointSettings>,
    #[account(
        mut,
        seeds = [
        NONCE_SEED,
        &sender.key().to_bytes(),
        &params.dst_eid.to_be_bytes(),
        &params.receiver[..]
        ],
        bump = nonce.bump
    )]
    pub nonce: Account<'info, Nonce>,
    token_program: Interface<'info, TokenInterface>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

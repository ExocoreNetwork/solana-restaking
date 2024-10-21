use anchor_lang::prelude::*;
use oapp::endpoint::ConstructCPIContext;
use oapp::endpoint::cpi::accounts::Clear;
use oapp::endpoint::instructions::ClearParams;
use oapp::LzReceiveParams;

use crate::states::{Config, RequestAction, Token};
use crate::lst_restaking::add_token;
use crate::utils::{create_pda, get_token_seeds};

pub fn lz_receive(ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {

    msg!("src_eid: {}", params.src_eid);
    msg!("sender: {:?}", params.sender);
    msg!("nonce: {}", params.nonce);
    msg!("guid: {:?}", params.guid);
    msg!("message: {:?}", params.message);
    msg!("extra_data: {:?}", params.extra_data);

    let mut start_accounts_clear = 0;

    let action = u8::try_from_slice(&params.message[..1])?;
    match action {
        9 => {
            msg!("the action of message is 9");

            // start_accounts_clear = 1;

            let mint = Pubkey::try_from_slice(&params.message[1..33])?;
            let tvl_limit = u128::try_from_slice(&params.message[33..49])?;

            msg!("mint: {}", mint);
            msg!("tvl_limit: {}", tvl_limit);

            let token = &ctx.accounts.unchecked_account;

            msg!("token: {}", token.key);

            let seeds = get_token_seeds(&mint);
            let space = 8 + Token::INIT_SPACE;

            create_pda(&token.to_account_info(), &ctx.accounts.payer.to_account_info(), &ctx.program_id, &seeds, space)?;

            Token::write(&token.to_account_info(), &mint, tvl_limit)?;

            // let tokens = &mut Account::<Tokens>::try_from(&tokens)?;
            // tokens.update_token_info(mint, tvl_limit, Action::Add)?;

            // ctx.accounts.tokens.update_token_info(mint, tvl_limit, Action::Add)?;

        }
        12 => {
            msg!("the action of message is 9");

        }
        _ => {
            msg!("the action of message is 9");

        }
    }
    // match action {
    //     12 => {
    //         let messages_account_info = &ctx.remaining_accounts[2];
    //
    //         let nonce = u64::try_from_slice(&params.message[1..9])?;
    //         let messages = Messages::try_from_slice(&messages_account_info.try_borrow_data()?)?;
    //         if let Some(action) = messages.action(nonce) {
    //             match action {
    //                 RequestAction::WithdrawLst(_content)
    //                 => {
    //                     start_accounts_clear = 5;
    //                     // update vault
    //                     msg!("complete message: {}", nonce);
    //
    //                     // remove nonce
    //                 }
    //                 _ => {}
    //             }
    //         }
    //     }
    //     9 => {
    //         start_accounts_clear = 5;
    //
    //         let mint_account_info = &ctx.remaining_accounts[1];
    //         let tokens_account_info = &ctx.remaining_accounts[2];
    //
    //         let mut tokens = Tokens::try_from_slice(&tokens_account_info.try_borrow_mut_data()?)?;
    //         // realloc
    //         tokens_account_info.realloc(tokens.get_size(), false)?;
    //
    //         // update tokens
    //         // check mint
    //         let mint = Pubkey::try_from_slice(&params.message[1..33])?;
    //         require_keys_eq!(mint, mint_account_info.key());
    //         let tvl_limit = u128::try_from_slice(&params.message[34..50])?;
    //         tokens.update_token_info(mint, tvl_limit, Action::Add)?;
    //
    //         // create token for pool
    //         let token_program_info = if *mint_account_info.owner == spl_token_2022::id() {
    //             &ctx.remaining_accounts[4]
    //         } else {
    //             &ctx.remaining_accounts[3]
    //         };
    //
    //         msg!("token program: {}", token_program_info.key())
    //
    //     }
    //     _ => {}
    // }

    // clear
    let seeds: &[&[u8]] =
        &[Config::CONFIG_SEED_PREFIX, &[ctx.accounts.config.bump]];

    let accounts_for_clear = &ctx.remaining_accounts[start_accounts_clear..start_accounts_clear + Clear::MIN_ACCOUNTS_LEN];
    let _ = oapp::endpoint_cpi::clear(
        ctx.accounts.config.endpoint_program,
        ctx.accounts.config.key(),
        accounts_for_clear,
        seeds,
        ClearParams {
            receiver: ctx.accounts.config.key(),
            src_eid: params.src_eid,
            sender: params.sender,
            nonce: params.nonce,
            guid: params.guid,
            message: params.message.clone(),
        },
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct LzReceive<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,
    seeds = [Config::CONFIG_SEED_PREFIX],
    bump = config.bump
    )]
    pub config: Account<'info, Config>,

    /// CHECK: generated PDA
    #[account(mut)]
    pub unchecked_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    // pub token: Account<'info, Tokens>,
}
use anchor_lang::prelude::*;
use oapp::endpoint::ConstructCPIContext;
use oapp::endpoint::cpi::accounts::Clear;
use oapp::endpoint::instructions::ClearParams;

use oapp::LzReceiveParams;
use crate::states::{Action, Config, Messages, RequestAction, Tokens};

pub fn lz_receive(ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {

    msg!("src_eid: {}", params.src_eid);
    msg!("sender: {:?}", params.sender);
    msg!("nonce: {}", params.nonce);
    msg!("guid: {:?}", params.guid);
    msg!("message: {:?}", params.message);
    msg!("extra_data: {:?}", params.extra_data);

    let action = u8::try_from_slice(&params.message[..1])?;
    match action {
        10 => {
            let messages_account_info = &ctx.remaining_accounts[2];

            let nonce = u64::try_from_slice(&params.message[1..9])?;
            let messages = Messages::try_from_slice(&messages_account_info.try_borrow_data()?)?;
            if let Some(action) = messages.action(nonce) {
                match action {
                    RequestAction::WithdrawPrincipalFromExocore(_content) |
                    RequestAction::WithdrawRewardFromExocore(_content)
                    => {
                        // update vault
                        msg!("complete message: {}", nonce);

                        // remove nonce
                    }
                    _ => {}
                }
            }
        }
        7 => {
            let mint_account_info = &ctx.remaining_accounts[1];
            let tokens_account_info = &ctx.remaining_accounts[2];

            let mut tokens = Tokens::try_from_slice(&tokens_account_info.try_borrow_mut_data()?)?;
            // realloc
            tokens_account_info.realloc(tokens.get_size(), false)?;

            // update tokens
            // check mint
            let mint = Pubkey::try_from_slice(&params.message[1..33])?;
            require_keys_eq!(mint, mint_account_info.key());
            let tvl_limit = u128::try_from_slice(&params.message[34..50])?;
            tokens.update_token_info(mint, tvl_limit, Action::Add)?;

            // create token for pool

        }
        _ => {}
    }

    // clear
    let seeds: &[&[u8]] =
        &[Config::CONFIG_SEED_PREFIX, &[ctx.accounts.config.bump]];

    let accounts_for_clear = &ctx.remaining_accounts[5..5 + Clear::MIN_ACCOUNTS_LEN];
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
    payer: Signer<'info>,
    #[account(mut,
    seeds = [Config::CONFIG_SEED_PREFIX],
    bump = config.bump
    )]
    config: Account<'info, Config>,
}
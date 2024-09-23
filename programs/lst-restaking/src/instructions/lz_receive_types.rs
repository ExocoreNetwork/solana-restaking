use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::token::spl_token;
use oapp::endpoint_cpi::LzAccount;
use crate::states::{Config, Messages, RequestAction, Tokens, Vault};
use crate::utils::{ get_pda};
use oapp::LzReceiveParams;
use solana_program::system_program;

// payer
// config
// [0..4] messages
// [0..4] tokens


pub fn lz_receive_types(ctx: Context<LzReceiveTypes>, params: LzReceiveParams) -> Result<Vec<LzAccount>> {
    let config = &ctx.accounts.config;

    msg!("src_eid: {}", params.src_eid);
    msg!("sender: {:?}", params.sender);
    msg!("nonce: {}", params.nonce);
    msg!("guid: {:?}", params.guid);
    msg!("message: {:?}", params.message);
    msg!("extra_data: {:?}", params.extra_data);

    let mut accounts = vec![
        LzAccount { pubkey: Pubkey::default(), is_signer: true, is_writable: true },
        // config
        LzAccount { pubkey: get_pda(&[Config::CONFIG_SEED_PREFIX]), is_signer: false, is_writable: true},
    ];

    let messages = &ctx.accounts.messages;
    let action = u8::try_from_slice(&params.message[..1])?;
    match action {
        10 => {
            let nonce = u64::try_from_slice(&params.message[1..9])?;
            if let Some(action) = messages.action(nonce) {
                match action {
                    RequestAction::WithdrawPrincipalFromExocore(content) |
                    RequestAction::WithdrawRewardFromExocore(content)
                    => {
                        accounts.extend(
                            vec![
                                // messages
                                LzAccount { pubkey: messages.key(), is_signer: true, is_writable: true },
                                // sender
                                LzAccount { pubkey: content.sender, is_signer: false, is_writable: false },
                                // mint
                                LzAccount { pubkey: content.mint, is_signer: false, is_writable: false },
                                // user vault
                                LzAccount { pubkey: get_pda(&[Vault::SEED_PREFIX, content.mint.as_ref(), content.sender.as_ref()]), is_signer: false, is_writable: true },
                                // system program
                                LzAccount { pubkey: system_program::id(), is_signer: false, is_writable: false },
                            ]
                        );
                    }
                    _  => {
                        msg!("Not a respond of withdrawal")
                    }
                }
            }
        }
        7 => {
            let mint = Pubkey::try_from_slice(&params.message[1..33])?;
            accounts.extend(
                vec![
                    // tokens
                    LzAccount { pubkey: get_pda(&[Tokens::SEED]), is_signer: false, is_writable: true},
                    // mint
                    LzAccount { pubkey: mint, is_signer: false, is_writable: false },
                    // pool token ata
                    LzAccount { pubkey: get_associated_token_address(&config.key(), &mint), is_signer: true, is_writable: true},
                    // token program
                    LzAccount { pubkey: spl_token::id(), is_signer: false, is_writable: false},
                    // system
                    LzAccount { pubkey: system_program::id(), is_signer: false, is_writable: false},
                ]
            )
        }
        _ => {
            msg!("Not a required processed response")
        }
    }

    let endpoint_program = ctx.accounts.config.endpoint_program;
    // remaining accounts 0..9
    let accounts_for_clear = oapp::endpoint_cpi::get_accounts_for_clear(
        endpoint_program,
        &config.key(),
        params.src_eid,
        &params.sender,
        params.nonce,
    );

    accounts.extend(accounts_for_clear);

    Ok(accounts)
}


#[derive(Accounts)]
pub struct LzReceiveTypes<'info> {
   #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump = config.bump
   )]
    config: Box<Account<'info, Config>>,
    #[account(
        address = config.messages
    )]
    messages: Box<Account<'info, Messages>>,
    #[account(
        address = config.tokens
    )]
    tokens: Box<Account<'info, Tokens>>
}
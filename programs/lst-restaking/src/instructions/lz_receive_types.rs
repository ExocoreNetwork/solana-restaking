use anchor_lang::prelude::*;
use oapp::endpoint_cpi::LzAccount;
use crate::states::{Config, Messages, RequestAction, Vault};
use crate::utils::{ get_pda};
use oapp::LzReceiveParams;
use solana_program::system_program;
use crate::id;

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
        // config
        LzAccount { pubkey: get_pda(&[Config::CONFIG_SEED_PREFIX]), is_signer: false, is_writable: true},
    ];

    // let messages_account_info = ctx.accounts.messages.to_account_info();
    let action = u8::try_from_slice(&params.message[..1])?;

    match action {
        9 => {
            msg!("the action of message is 9");
            // let mint = Pubkey::try_from_slice(&params.message[1..33])?;
            accounts.extend(
                vec![
                    LzAccount { pubkey: config.tokens, is_signer: false, is_writable: true},
                    // LzAccount { pubkey: id(), is_signer: false, is_writable: false},
                    // LzAccount { pubkey: system_program::id(), is_signer: false, is_writable: false},
                ]
            );
        }
        12 => {
           msg!("the action of message is 12");
        }
        _ => {
            msg!("the action of message is {}, we won't process this message now", action);
        }
    }


    // match action {
    //     12 => {
    //         let nonce = u64::try_from_slice(&params.message[1..9])?;
    //         let messages = Messages::try_from_slice(&messages_account_info.try_borrow_data()?)?;
    //         if let Some(action) = messages.action(nonce) {
    //             match action {
    //                 RequestAction::WithdrawLst(content)
    //                 => {
    //                     accounts.extend(
    //                         vec![
    //                             // messages
    //                             LzAccount { pubkey: config.messages, is_signer: true, is_writable: true },
    //                             // sender
    //                             LzAccount { pubkey: content.sender, is_signer: false, is_writable: false },
    //                             // mint
    //                             LzAccount { pubkey: content.mint, is_signer: false, is_writable: false },
    //                             // user vault
    //                             LzAccount { pubkey: get_pda(&[Vault::SEED_PREFIX, content.mint.as_ref(), content.sender.as_ref()]), is_signer: false, is_writable: true },
    //                             // system program
    //                             LzAccount { pubkey: system_program::id(), is_signer: false, is_writable: false },
    //                         ]
    //                     );
    //                 }
    //                 _  => {
    //                     msg!("Not a respond of withdrawal")
    //                 }
    //             }
    //         }
    //     }
    //     9 => {
    //         let mint = Pubkey::try_from_slice(&params.message[1..33])?;
    //         accounts.extend(
    //             vec![
    //                 // tokens
    //                 LzAccount { pubkey: config.tokens, is_signer: false, is_writable: true},
    //                 // mint
    //                 LzAccount { pubkey: mint, is_signer: false, is_writable: false },
    //                 // pool token ata
    //                 LzAccount { pubkey: get_associated_token_address(&config.key(), &mint), is_signer: true, is_writable: true},
    //                 // token program
    //                 LzAccount { pubkey: spl_token::id(), is_signer: false, is_writable: false},
    //
    //                 LzAccount { pubkey: spl_token_2022::id(), is_signer: false, is_writable: false},
    //                 // system
    //                 LzAccount { pubkey: system_program::id(), is_signer: false, is_writable: false},
    //             ]
    //         )
    //     }
    //     _ => {
    //         msg!("Not a required processed response")
    //     }
    // }

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
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump = config.bump
   )]
    config: Box<Account<'info, Config>>,
    // #[account(
    //     address = config.messages
    // )]
    // messages: Box<Account<'info, Messages>>,
    // #[account(
    //     address = config.tokens
    // )]
    // tokens: Box<Account<'info, Tokens>>
}
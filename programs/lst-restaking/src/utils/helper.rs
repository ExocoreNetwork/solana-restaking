use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use anchor_spl::token_2022;
use anchor_spl::token_2022::{initialize_account3, InitializeAccount3, spl_token_2022};
use anchor_spl::token_2022::spl_token_2022::extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions};
use oapp::endpoint::instructions::SendParams;
use oapp::endpoint_cpi;
use crate::{id, RECEIVER, remote_eid};
use crate::states::Config;

pub fn create_token_account<'a>(
    authority: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let space = {
        let mint_info = mint_account.to_account_info();
        if *mint_info.owner == token_2022::Token2022::id() {
            let mint_data = mint_info.try_borrow_data()?;
            let mint_state =
                StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
            let mint_extensions = mint_state.get_extension_types()?;
            let required_extensions =
                ExtensionType::get_required_init_account_extensions(&mint_extensions);
            ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(
                &required_extensions,
            )?
        } else {
            TokenAccount::LEN
        }
    };
    let lamports = Rent::get()?.minimum_balance(space);
    let cpi_accounts = anchor_lang::system_program::CreateAccount {
        from: payer.to_account_info(),
        to: token_account.to_account_info(),
    };
    let cpi_context = CpiContext::new(system_program.to_account_info(), cpi_accounts);
    anchor_lang::system_program::create_account(
        cpi_context.with_signer(signer_seeds),
        lamports,
        space as u64,
        token_program.key,
    )?;
    initialize_account3(CpiContext::new(
        token_program.to_account_info(),
        InitializeAccount3 {
            account: token_account.to_account_info(),
            mint: mint_account.to_account_info(),
            authority: authority.to_account_info(),
        },
    ))
}

pub(crate) fn send(endpoint: Pubkey, sender: Pubkey, accounts: &[AccountInfo], bump: u8, message: Vec<u8>, opts: Vec<u8>) -> Result<u64> {

    let signer = &[Config::CONFIG_SEED_PREFIX, &[bump][..]];

    let result = endpoint_cpi::send(
        endpoint,
        sender,
        accounts,
        signer,
        SendParams {
            dst_eid: remote_eid(),
            receiver: RECEIVER,
            message,
            options: opts.clone(),
            native_fee: 500_000,
            lz_token_fee: 0,
        }
    )?;

    Ok(result.nonce)
}

pub fn get_pda(seeds: &[&[u8]]) -> Pubkey {
    let (key, _) = Pubkey::find_program_address(seeds, &id());

    key
}
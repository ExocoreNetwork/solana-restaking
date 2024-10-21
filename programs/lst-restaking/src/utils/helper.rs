use std::mem;
use std::ops::Deref;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use anchor_spl::token_2022;
use anchor_spl::token_2022::{initialize_account3, InitializeAccount3, spl_token_2022};
use anchor_spl::token_2022::spl_token_2022::extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions};
use oapp::endpoint::instructions::SendParams;
use oapp::endpoint_cpi;
use solana_program::program::invoke_signed;
use solana_program::system_instruction;
use crate::{CONFIG_SEEDS_PREFIX, id, MESSAGE_SEEDS_PREFIX, TOKEN_SEEDS_PREFIX, VAULT_SEEDS_PREFIX};
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

pub(crate) fn create_pda<'a>(
    new_account: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    program_id: &Pubkey,
    seeds: &[&[u8]],
    space: usize,
) -> Result<()> {
    let (pda, bump) = Pubkey::find_program_address(seeds, program_id);

    msg!("new account: {} will be created.", pda);

    let lamports = Rent::get()?.minimum_balance(space);

    invoke_signed(
        &system_instruction::create_account(&payer.key, &pda, lamports, space as u64, program_id),
        &[payer.clone(), new_account.clone()],
        &[&[seeds[0], seeds[1], &[bump]]],
    )?;

    Ok(())
}

pub(crate) fn send(endpoint: Pubkey, sender: Pubkey, accounts: &[AccountInfo], bump: u8, message: Vec<u8>, opts: Vec<u8>, dst_eid: u32, address: [u8;32]) -> Result<u64> {

    let signer = &[Config::CONFIG_SEED_PREFIX, &[bump][..]];

    let result = endpoint_cpi::send(
        endpoint,
        sender,
        accounts,
        signer,
        SendParams {
            dst_eid,
            receiver: address,
            message,
            options: opts.clone(),
            native_fee: 500_000,
            lz_token_fee: 0,
        }
    )?;

    Ok(result.nonce)
}

pub fn get_pda(seeds: &[&[u8]]) -> Result<(Pubkey, u8)> {
    let (pda, bump) = Pubkey::find_program_address(
        seeds,
        &id()
    );

    Ok((pda, bump))
}

pub fn get_config_seeds() -> Result<Vec<u8>> {
    let seeds = CONFIG_SEEDS_PREFIX;
    Ok(seeds.to_vec())
}

pub fn get_config() -> Result<(Pubkey, u8)> {
    let seeds = get_config_seeds()?;
    get_pda(&[&seeds])
}

pub fn get_token_seeds(mint: &Pubkey) -> [&[u8]; 2] {
    [TOKEN_SEEDS_PREFIX, mint.as_ref()]
}
pub fn get_token(mint: &Pubkey) -> Result<(Pubkey, u8)> {
    let seeds = get_token_seeds(mint);

    msg!("seeds: {:?}", seeds);

    get_pda(&seeds)
}

pub fn get_message_seeds(nonce: &u64) -> [Vec<u8>; 2] {
    let nonce_bytes = nonce.to_be_bytes();
    [MESSAGE_SEEDS_PREFIX.to_vec(), nonce_bytes.to_vec()]
}

pub fn get_message(nonce: u64) -> Result<(Pubkey, u8)> {
    let seeds = get_message_seeds(&nonce);

    let seeds: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();

    get_pda(&seeds)
}

pub fn get_vault_seeds<'a>(mint: &'a Pubkey, user: &'a Pubkey) -> [&'a[u8]; 3] {
    [VAULT_SEEDS_PREFIX, mint.as_ref(), user.as_ref()]
}
pub fn get_vault(mint: &Pubkey, user: &Pubkey) -> Result<(Pubkey, u8)> {
    let seeds = get_vault_seeds(mint, user);

    get_pda(&seeds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use solana_program::pubkey::Pubkey;
    use rand::random;

    #[test]
    fn test_get_token() {
        // Define a test mint address (you can replace this with an actual test mint Pubkey)
        let mint = Pubkey::new_unique();

        // Log the mint address for reference
        msg!("Test mint: {:?}", mint);

        // Call the get_token function and handle the result
        match get_token(&mint) {
            Ok((token_pubkey, bump)) => {
                msg!("Token Pubkey: {:?}", token_pubkey);
                msg!("Bump: {:?}", bump);

                // Add assertions if necessary
                // Example: Assert that the returned token_pubkey is not a zero pubkey
                assert_ne!(token_pubkey, Pubkey::default(), "Token Pubkey should not be default");
            }
            Err(e) => {
                msg!("Error occurred: {:?}", e);
                panic!("get_token function failed");
            }
        }
    }

    #[test]
    fn test_get_config() {

        // Call the get_token function and handle the result
        match get_config() {
            Ok((config_pubkey, bump)) => {
                msg!("Config Pubkey: {:?}", config_pubkey);
                msg!("Bump: {:?}", bump);

                // Add assertions if necessary
                // Example: Assert that the returned token_pubkey is not a zero pubkey
                assert_ne!(config_pubkey, Pubkey::default(), "Token Pubkey should not be default");
            }
            Err(e) => {
                msg!("Error occurred: {:?}", e);
                panic!("get_token function failed");
            }
        }
    }

    #[test]
    fn test_get_message() {
        // Define a test mint address (you can replace this with an actual test mint Pubkey)
        let nonce = rand::random::<u64>();

        // Log the mint address for reference
        msg!("Test nonce: {:?}", nonce);

        // Call the get_token function and handle the result
        match get_message(nonce) {
            Ok((message_pubkey, bump)) => {
                msg!("Message Pubkey: {:?}", message_pubkey);
                msg!("Bump: {:?}", bump);

                // Add assertions if necessary
                // Example: Assert that the returned token_pubkey is not a zero pubkey
                assert_ne!(message_pubkey, Pubkey::default(), "Token Pubkey should not be default");
            }
            Err(e) => {
                msg!("Error occurred: {:?}", e);
                panic!("get_token function failed");
            }
        }
    }

    #[test]
    fn test_get_vault() {
        // Define a test mint address (you can replace this with an actual test mint Pubkey)
        let mint = Pubkey::new_unique();
        let user = Pubkey::new_unique();

        // Log the mint address for reference
        msg!("Test mint: {:?}", mint);
        msg!("Test user: {:?}", user);

        // Call the get_token function and handle the result
        match get_vault(&mint, &user) {
            Ok((vault_pubkey, bump)) => {
                msg!("Vault Pubkey: {:?}", vault_pubkey);
                msg!("Bump: {:?}", bump);

                // Add assertions if necessary
                // Example: Assert that the returned token_pubkey is not a zero pubkey
                assert_ne!(vault_pubkey, Pubkey::default(), "Token Pubkey should not be default");
            }
            Err(e) => {
                msg!("Error occurred: {:?}", e);
                panic!("get_token function failed");
            }
        }
    }
}
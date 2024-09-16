use anchor_lang::prelude::*;
use oapp::endpoint_cpi::LzAccount;
use oapp::LzReceiveParams;
use crate::states::Config;


pub fn lz_receive_types(ctx: Context<LzReceiveTypes>, params: LzReceiveParams) -> Result<()> {
    let config = &ctx.accounts.config;

    msg!("src_eid: {}", params.src_eid);
    msg!("sender: {:?}", params.sender);
    msg!("nonce: {}", params.nonce);
    msg!("guid: {:?}", params.guid);
    msg!("message: {:?}", params.message);
    msg!("extra_data: {:?}", params.extra_data);

    let mut accounts = vec![
        LzAccount { pubkey: Pubkey::default(), is_signer: true, is_writable: true },
    ];


    // action == 10  ==> respond

    // action == 7 ==> addToken

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

    Ok(())
}


#[derive(Accounts)]
pub struct LzReceiveTypes<'info> {
   #[account(
        mut,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump = config.bump
   )]
    config: Account<'info, Config>
}
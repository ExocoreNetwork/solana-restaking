use crate::states::Config;
use anchor_lang::prelude::*;
use oapp::endpoint::instructions::RegisterOAppParams;
// use oapp::endpoint::cpi::accounts::RegisterOApp;
// use oapp::endpoint::endpoint::register_oapp;
// use oapp::endpoint::instructions::{RegisterOAppParams};
use oapp::endpoint::program::Endpoint;
use oapp::endpoint_cpi::register_oapp;

pub fn initialize(ctx: Context<InitConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.owner = ctx.accounts.owner.key();
    config.pending_owner = ctx.accounts.owner.key();

    let signer = &[Config::CONFIG_SEED_PREFIX, &[ctx.bumps.config][..]];

    register_oapp(
        ctx.accounts.endpoint_program.key(),
        ctx.accounts.config.key(),
        &[
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.config.to_account_info(),
            ctx.accounts.oapp_registry.to_account_info(),
            ctx.accounts.system_program.to_account_info()
        ],
        signer,
        RegisterOAppParams {
            delegate: ctx.accounts.delegate.key()
        })?;

    // register_oapp(
    //     Context::new_with_signer(
    //         ctx.accounts.endpoint_program.to_account_info(),
    //         RegisterOApp {
    //             payer: ctx.accounts.owner.to_account_info(),
    //             oapp: ctx.accounts.config.to_account_info(),
    //             oapp_registry: ctx.accounts.oapp_registry.to_account_info(),
    //             system_program: ctx.accounts.system_program.to_account_info(),
    //         },
    //         &[signer],
    //     ),
    //     RegisterOAppParams {
    //         delegate: ctx.accounts.delegate.key(),
    //     },
    // )?;

    msg!("Successful to initialize config");

    Ok(())
}

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        seeds = [Config::CONFIG_SEED_PREFIX],
        bump,
        space = 8 + Config::LEN
    )]
    config: Account<'info, Config>,
    delegate: SystemAccount<'info>,
    /// CHECK: PDA account, will be created in lz
    oapp_registry: UncheckedAccount<'info>,
    endpoint_program: Program<'info, Endpoint>,
    system_program: Program<'info, System>,
}

use anchor_lang::prelude::*;
use crate::states::{Action, Tokens};

pub fn add_token(ctx: Context<AddToken>, params: AddTokenParams) -> Result<()> {
    // require_keys_eq!(*ctx.program_id, ctx.accounts.caller.key());

    let tokens= &mut ctx.accounts.tokens;

    tokens.update_token_info(params.mint, params.tvl_limit, Action::Add)?;

    Ok(())
}

#[derive(Accounts)]
pub struct AddToken<'info> {
    /// CHECK: validate it inside
    // pub caller: UncheckedAccount<'info>,
    // TODO: realloc  payer ?
    #[account(mut
        // realloc = ,
        // realloc::payer
    )]
    pub tokens: Account<'info, Tokens>,

    pub system_account: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AddTokenParams {
    pub mint: Pubkey,
    pub tvl_limit: u128,
    pub consumed_tvl: u128
}
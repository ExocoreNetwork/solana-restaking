use anchor_lang::prelude::*;
use crate::states::Token;

pub fn add_token(ctx: Context<AddToken>, params: AddTokenParams) -> Result<()> {
    // TODO: cpi call

    let token= &mut ctx.accounts.token;


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
    pub token: Account<'info, Token>,

    pub system_account: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AddTokenParams {
    pub mint: Pubkey,
    pub tvl_limit: u128,
    pub consumed_tvl: u128
}
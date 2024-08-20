use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod states;

declare_id!("68av2QdR1k1QeaxsJwjiB16QHXDhTuaS14tyTNM3MgHX");

use instructions::*;

#[program]
pub mod lst_restaking {
    use super::*;

    pub fn initialize(ctx: Context<InitConfig>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::transfer_ownership(ctx)
    }

    pub fn update_white_lists(ctx: Context<UpdateWhiteList>, action: Action) -> Result<()> {
        instructions::update_white_lists(ctx, action)
    }

    pub fn accept(ctx: Context<Accept>) -> Result<()> {
        instructions::accept(ctx)
    }
}

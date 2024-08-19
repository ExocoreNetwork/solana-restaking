use anchor_lang::prelude::*;

declare_id!("4ejcBdrPJUQ485bTxVfNQH42zTdN4i6gpyxPAGQkyfg9");

#[program]
pub mod lst_restaking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

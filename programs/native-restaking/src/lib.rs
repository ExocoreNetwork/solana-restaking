use anchor_lang::prelude::*;

declare_id!("AtfUbXniQddbR3DVetG2xJz7Hnyg8geNECRrDdnLsYZ5");

#[program]
pub mod native_restaking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

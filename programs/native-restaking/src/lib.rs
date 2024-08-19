use anchor_lang::prelude::*;

declare_id!("2FP3X2ky9aDf9F9WXbrz76vcC7PgTAUDFqkwrkiZuJNB");

#[program]
pub mod native_restaking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

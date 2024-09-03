use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub deposit_balance: u64,
    pub withdrawable_balance: u64,
    pub unlocked_principal_balance: u64,
    pub reward_balance: u64,
    #[max_len(500)]
    pub _padding: Vec<u8>,
}

impl Vault {
    pub const SEED_PREFIX: &'static [u8; 5] = b"vault";
}

#[cfg(test)]
mod tests {

}

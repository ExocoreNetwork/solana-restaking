use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub deposit_balance: u64,
    pub withdrawable_balance: u64,
    pub unlocked_principal_balance: u64,
    pub reward_balance: u64,
    pub _padding: Vec<u8>,
}

impl Vault {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 4 + 200;

    pub const SEED_PREFIX: &'static [u8; 5] = b"vault";
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::states::Vault;

    pub fn test_len() {
        assert_eq!(
            Vault::LEN,
            size_of::<u64>()
                + size_of::<u64>()
                + size_of::<u64>()
                + size_of::<u64>()
                + 4
                + size_of::<u8>() * 200
        )
    }
}

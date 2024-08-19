use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub owner: Pubkey,
    pub pending_owner: Pubkey,
    pub white_lists_token: Vec<Pubkey>,
    pub _padding: Vec<u8>,
}

impl Config {
    pub const SEED_PREFIX: &'static [u8; 6] = b"config";

    //  the count of token when initialize is 10
    pub const LEN: usize = 32 + 32 + 4 + 32 * 10 + 4 + 500;
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    pub fn test_len() {
        assert_eq!(
            Config::LEN,
            size_of::<Pubkey>()
                + size_of::<Pubkey>()
                + 4
                + size_of::<Pubkey>() * 10
                + 4
                + size_of::<u8>() * 500
        )
    }
}

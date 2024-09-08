use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub owner: Pubkey,
    pub pending_owner: Pubkey,
    pub remote_eid: u32,
    pub receiver: [u8; 32],
    pub message_list: Pubkey,
    pub token_white_list: Pubkey,
    #[max_len(500)]
    pub _padding: Vec<u8>,
}

impl Config {
    pub const CONFIG_SEED_PREFIX: &'static [u8] = b"config";
    pub const LZ_RECEIVE_TYPES_SEED: &'static [u8]  = oapp::LZ_RECEIVE_TYPES_SEED;
}

#[account]
#[derive(InitSpace)]
pub struct LzReceiveTypes {
    pub config: Pubkey,
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

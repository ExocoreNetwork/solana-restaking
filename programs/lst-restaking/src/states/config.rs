use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub owner: Pubkey,
    pub operator: Pubkey,
    pub pending_owner: Pubkey,
    pub remote_eid: u32,
    pub receiver: [u8; 32],
    pub message_list: Pubkey,
    pub tokens: Pubkey,
    pub endpoint_program: Pubkey,
    pub bump: u8,
    #[max_len(500)]
    pub _padding: Vec<u8>,
}

impl Config {
    pub const CONFIG_SEED_PREFIX: &'static [u8] = b"config";
    pub const LZ_RECEIVE_TYPES_SEED: &'static [u8]  = oapp::LZ_RECEIVE_TYPES_SEED;
}

#[account]
#[derive(InitSpace)]
pub struct LzReceiveTypesAccount {
    pub config: Pubkey,
}

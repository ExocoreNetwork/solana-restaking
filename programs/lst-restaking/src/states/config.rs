use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub owner: Pubkey,
    pub operator: Pubkey,
    pub pending_owner: Pubkey,
    pub eid: u32,
    pub receiver: [u8; 32],
    pub messages: Pubkey,
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
    pub messages: Pubkey,
    // pub tokens: Pubkey,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct LzReceiveParams {
    pub src_eid: u32,
    pub sender: [u8; 32],
    pub nonce: u64,
    pub guid: [u8; 32],
    pub message: Vec<u8>,
    pub extra_data: Vec<u8>,
}
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum RequestAction {
    Deposit,
    WithdrawPrincipalFromExocore,
    WithdrawRewardFromExocore,
    DelegateTo,
    UndelegateFrom,
    DepositThenDelegateTo,
    AddWhiteListToken,
    DeactivateWhiteListToken,
    Respond
}

#[account]
pub struct Messages {
    message: Vec<Message>
}
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Message {
    nonce: u128,
    action: RequestAction,
    params: Vec<u8>
}

impl Message {
    pub const SEED_PREFIX: &'static [u8; 7] = b"message";
    pub const LEN: usize = 1 + 4 + 32 + 32 + 32 + 8;  // max length

}
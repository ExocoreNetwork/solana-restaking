use anchor_lang::prelude::*;
use std::mem;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum RequestAction {
    Deposit(MessageWithoutOperator),
    WithdrawPrincipalFromExocore(MessageWithoutOperator),
    WithdrawRewardFromExocore(MessageWithoutOperator),
    DelegateTo(MessageWithOperator),
    UndelegateFrom(MessageWithOperator),
    DepositThenDelegateTo(MessageWithOperator),
    _RequestMarkBootstrap,
    AddWhiteListToken,
    _RequestAssociateOperator,
    _RequestDissociateOperator,
    Respond(RespondMessage),
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct MessageWithoutOperator {
    pub(crate) mint: Pubkey,
    pub(crate) sender: Pubkey,
    pub(crate) amount: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct MessageWithOperator {
    pub(crate) mint: Pubkey,
    pub(crate) sender: Pubkey,
    pub(crate) operator: [u8; 32],
    pub(crate) amount: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct RespondMessage {
    request_id: u64,
    result: u8,
}

#[account]
#[derive(InitSpace)]
pub struct MessageList {
    #[max_len(0)]
    message: Vec<Message>,
}
#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct Message {
    nonce: u64,
    action: RequestAction,
}

impl MessageList {
    pub const MESSAGE_SEED_PREFIX: &'static [u8; 12] = b"message-list";

    pub fn pending(&mut self, nonce: u64, action: RequestAction) -> Result<()> {
        self.message.push(Message { nonce, action });

        Ok(())
    }

    pub fn processed(&mut self, nonce: u64) -> Result<()> {
        self.message.retain(|m| m.nonce == nonce);

        Ok(())
    }

    pub fn get_size(&self) -> usize {
        8 + 4 + self.message.len() + mem::size_of::<Message>()
    }
}

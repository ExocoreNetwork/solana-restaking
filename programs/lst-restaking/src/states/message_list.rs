use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
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

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MessageWithoutOperator {
    mint: Pubkey,
    sender: Pubkey,
    amount: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct MessageWithOperator {
    mint: Pubkey,
    sender: Pubkey,
    operator: Pubkey,
    amount: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct RespondMessage {
    request_id: u64,
    result: u8,
}

#[account]
pub struct MessageList {
    message: Vec<Message>,
}
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Message {
    nonce: u64,
    action: RequestAction,
}

impl MessageList {
    pub const MESSAGE_SEED_PREFIX: &'static [u8; 12] = b"message-list";
    pub const LEN: usize = 16 + 1 + 4 + 32 + 32 + 32 + 8; // max length

    pub fn pending(&mut self, nonce: u64, action: RequestAction) -> Result<()> {
        self.message.push(Message { nonce, action });

        Ok(())
    }

    pub fn processed(&mut self, nonce: u128) -> Result<()> {
        self.message.retain(|m| m.nonce == nonce);

        Ok(())
    }
}

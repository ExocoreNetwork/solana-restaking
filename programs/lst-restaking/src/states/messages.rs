use anchor_lang::prelude::*;
use std::mem;

//     REQUEST_DEPOSIT_LST,
//     REQUEST_DEPOSIT_NST,
//     REQUEST_WITHDRAW_LST,
//     REQUEST_WITHDRAW_NST,
//     REQUEST_CLAIM_REWARD,
//     REQUEST_DELEGATE_TO,
//     REQUEST_UNDELEGATE_FROM,
//     REQUEST_DEPOSIT_THEN_DELEGATE_TO,
//     REQUEST_MARK_BOOTSTRAP,
//     REQUEST_ADD_WHITELIST_TOKEN,
//     REQUEST_ASSOCIATE_OPERATOR,
//     REQUEST_DISSOCIATE_OPERATOR,
//     RESPOND

#[repr(u32)]
#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum RequestAction {
    DepositLst(MessageWithoutOperator), // 0
    DepositNst(MessageWithoutOperator), // 1
    WithdrawLst(MessageWithoutOperator), // 2
    WithdrawNst(MessageWithoutOperator), // 3
    ClaimReward(MessageWithoutOperator), // 4
    DelegateTo(MessageWithOperator), // 5
    UndelegateFrom(MessageWithOperator), // 6
    DepositThenDelegateTo(MessageWithOperator), // 7
    _RequestMarkBootstrap, // 8
    AddWhiteListToken, // 9
    _RequestAssociateOperator, // 10
    _RequestDissociateOperator, // 11
    Respond(RespondMessage), // 12
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
pub struct Messages {
    #[max_len(10)]
    data: Vec<Message>,
}
#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct Message {
    nonce: u64,
    action: RequestAction,
}

impl Messages {
    pub const MESSAGE_SEED_PREFIX: &'static [u8] = b"message-list";

    pub fn pending(&mut self, nonce: u64, action: RequestAction) -> Result<()> {
        self.data.push(Message { nonce, action });

        Ok(())
    }

    pub fn processed(&mut self, nonce: u64) -> Result<()> {
        self.data.retain(|m| m.nonce == nonce);

        Ok(())
    }

    pub fn action(&self, nonce: u64) -> Option<RequestAction> {
        if let Some(msg) = self.data.iter().find(|msg| msg.nonce == nonce).cloned() {
            Some(msg.action)
        } else { None }
    }

    pub fn get_size(&self) -> usize {
        8 + 4 + self.data.len() + mem::size_of::<Message>()
    }
}

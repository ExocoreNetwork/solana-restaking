use std::io::{Read, Write};
use anchor_lang::prelude::*;
use std::mem;
use anchor_lang::Discriminator;
use crate::errors::LstRestakingError;

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

impl RequestAction {
    pub fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self {
            RequestAction::DepositLst(content) => {
                writer.write_all(&[0u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::DepositNst(content) => {
                writer.write_all(&[1u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::WithdrawLst(content) => {
                writer.write_all(&[2u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::WithdrawNst(content) => {
                writer.write_all(&[3u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::ClaimReward(content) => {
                writer.write_all(&[4u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::DelegateTo(content) => {
                writer.write_all(&[5u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::UndelegateFrom(content) => {
                writer.write_all(&[6u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::DepositThenDelegateTo(content) => {
                writer.write_all(&[7u8])?;
                writer.write_all(&content.try_to_vec()?)?;
            }
            RequestAction::_RequestAssociateOperator => {
                writer.write_all(&[10u8])?;
            }
            RequestAction::_RequestDissociateOperator => {
                writer.write_all(&[11u8])?;
            }
            _ => {
                return Err(LstRestakingError::NoNeedProcessedAction.into())
            }
        }

        Ok(())
    }

    pub fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let mut variant = [0; 1];
        reader.read_exact(&mut variant)?;

        match variant[0] {
            2 => {
                let mut buffer = [0u8; mem::size_of::<MessageWithoutOperator>()];
                reader.read_exact(&mut buffer)?;
                let content = MessageWithoutOperator::try_from_slice(&buffer)?;
                Ok(RequestAction::WithdrawLst(content))
            },
            3 => {
                let mut buffer = [0u8; mem::size_of::<MessageWithoutOperator>()];
                reader.read_exact(&mut buffer)?;
                let content = MessageWithoutOperator::try_from_slice(&buffer)?;
                Ok(RequestAction::WithdrawNst(content))
            },
            4 => {
                let mut buffer = [0u8; mem::size_of::<MessageWithoutOperator>()];
                reader.read_exact(&mut buffer)?;
                let content = MessageWithoutOperator::try_from_slice(&buffer)?;
                Ok(RequestAction::ClaimReward(content))
            },
            6 => {
                let mut buffer = [0u8; mem::size_of::<MessageWithOperator>()];
                reader.read_exact(&mut buffer)?;
                let content = MessageWithOperator::try_from_slice(&buffer)?;
                Ok(RequestAction::UndelegateFrom(content))
            },
            _ => {
                Err(LstRestakingError::NoNeedProcessedAction.into())
            }
        }
    }
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
    request_id: u64, // nonce
    result: u8,
}


#[account]
#[derive(InitSpace)]
pub struct Message {
    nonce: u64,
    action: RequestAction,
}

impl Message {
    pub fn write(new_account: &AccountInfo, nonce: u64, action: &RequestAction) -> Result<()> {
        let mut account_data = new_account.try_borrow_mut_data()?;

        msg!("data: {:?}, {}", account_data, account_data.len());

        if account_data.len() < (8 + Message::INIT_SPACE) {
            return Err(LstRestakingError::AccountDataTooSmall.into());
        }

        if account_data.iter().all(|&x| x == 0) {
            msg!("Initializing new message account data");

            let account = Message {
                nonce,
                action: action.clone()
            };

            let discriminator = Message::discriminator();
            account_data[..8].copy_from_slice(&discriminator);

            account.serialize(&mut &mut account_data[8..])?;
        }
        Ok(())
    }
}

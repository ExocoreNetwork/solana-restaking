use crate::*;
use anchor_lang::prelude::*;

pub fn encode(action: RequestAction) -> Result<Vec<u8>> {
    let mut encoded = Vec::new();

    match action {
        RequestAction::DepositLst(content) => {
            encoded.push(0u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::WithdrawLst(content) => {
            encoded.push(2u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::ClaimReward(content) => {
            encoded.push(4u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::DelegateTo(content) => {
            encoded.push(5u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::UndelegateFrom(content) => {
            encoded.push(6u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::DepositThenDelegateTo(content) => {
            encoded.push(7u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::_RequestAssociateOperator => {
            encoded.push(10u8);
        }
        RequestAction::_RequestDissociateOperator => {
            encoded.push(11u8);
        }
        _ => {}
    }

    Ok(encoded)
}


use crate::RequestAction;
use anchor_lang::prelude::*;

pub fn encode(action: RequestAction) -> Result<Vec<u8>> {
    let mut encoded = Vec::new();

    match action {
        RequestAction::Deposit(content) => {
            encoded.push(0u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::WithdrawPrincipalFromExocore(content) => {
            encoded.push(1u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::WithdrawRewardFromExocore(content) => {
            encoded.push(2u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::DelegateTo(content) => {
            encoded.push(3u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::UndelegateFrom(content) => {
            encoded.push(4u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::DepositThenDelegateTo(content) => {
            encoded.push(5u8);
            encoded.extend_from_slice(&content.try_to_vec()?);
        }
        RequestAction::_RequestMarkBootstrap => {
            encoded.push(6u8);
        }
        RequestAction::AddWhiteListToken => {
            encoded.push(7u8);
            // encoded.extend_from_slice(&serialize(&content)?);
        }
        RequestAction::_RequestAssociateOperator => {
            encoded.push(8u8);
        }
        RequestAction::_RequestDissociateOperator => {
            encoded.push(9u8);
        }
        _ => {}
    }

    Ok(encoded)
}


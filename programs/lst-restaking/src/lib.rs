use anchor_lang::prelude::*;
use oapp::LzReceiveParams;

mod errors;
mod instructions;
mod states;
mod utils;

declare_id!("3DsgkXpd7Hwc6Q1iZ4YGLFrfSQZvotGSDGYRAvcDL53V");

use instructions::*;
use states::*;

pub const RECEIVER: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 37, 69, 67, 136, 213, 44, 188, 201, 43, 141, 31, 87,
    60, 103, 123, 163, 248, 241, 184,
];

pub const fn remote_eid() -> u32 {
    if cfg!(feature = "main") {
        return 40259;
    }
    40259
}

#[program]
pub mod lst_restaking {
    use oapp::LzReceiveParams;
    use super::*;

    pub fn initialize(ctx: Context<InitConfig>, params: InitConfigParams) -> Result<()> {
        instructions::initialize(ctx, params)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>) -> Result<()> {
        instructions::transfer_ownership(ctx)
    }

    pub fn update_tvl_limit(ctx: Context<UpdateTvlLimit>, params: UpdateTvlLimitParams) -> Result<()> {
        instructions::update_tvl_limit(ctx, params)
    }

    pub fn accept(ctx: Context<Accept>) -> Result<()> {
        instructions::accept(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, params: DepositParams) -> Result<()> {
        instructions::deposit(ctx, params)
    }

    pub fn delegate_to(ctx: Context<DelegateTo>, params: DelegateToParams) -> Result<()> {
        instructions::delegate_to(ctx, params)
    }

    pub fn lz_receive_types(ctx: Context<LzReceiveTypes>, params: LzReceiveParams) -> Result<()> {
        instructions::lz_receive_types(ctx, params)
    }

    pub fn lz_receive(ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {
        instructions::lz_receive(ctx, params)
    }

    pub fn deposit_then_delegate_to(
        ctx: Context<DepositThenDelegateTo>,
        params: DepositThenDelegateToParams,
    ) -> Result<()> {
        instructions::deposit_then_delegate_to(ctx, params)
    }

    pub fn withdraw_principal_from_exocore(
        ctx: Context<WithdrawPrincipal>,
        params: WithdrawPrincipalParams,
    ) -> Result<()> {
        instructions::withdraw_principal_from_exocore(ctx, params)
    }

    pub fn withdraw_reward_from_exocore(
        ctx: Context<WithdrawReward>,
        params: WithdrawRewardParams,
    ) -> Result<()> {
        instructions::withdraw_reward_from_exocore(ctx, params)
    }
}

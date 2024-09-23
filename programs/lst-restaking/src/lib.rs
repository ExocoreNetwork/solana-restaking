use anchor_lang::prelude::*;
use oapp::endpoint_cpi::LzAccount;
use oapp::LzReceiveParams;

use instructions::*;
use states::*;

mod errors;
mod instructions;
mod states;
mod utils;

declare_id!("3DsgkXpd7Hwc6Q1iZ4YGLFrfSQZvotGSDGYRAvcDL53V");

pub const RECEIVER: [u8; 32] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 83, 57, 150, 221, 221, 22, 126, 255, 122, 45, 159, 245, 136, 140, 70, 136, 245, 23, 248, 224];

pub const SRC_EID: u32 = 40168;

pub const fn remote_eid() -> u32 {
    if cfg!(feature = "main") {
        return 40259;
    }
    40259
}

#[program]
pub mod lst_restaking {
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

    pub fn lz_receive_types(ctx: Context<LzReceiveTypes>, params: LzReceiveParams) -> Result<Vec<LzAccount>> {
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

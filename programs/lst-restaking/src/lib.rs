mod errors;
mod instructions;
mod states;
mod utils;


use anchor_lang::prelude::*;
use oapp::endpoint_cpi::LzAccount;
use oapp::LzReceiveParams;
use instructions::*;
use states::*;



declare_id!("4zfL13ounan3Uaa5TiUDphxTnYVvaVDLTPNvkRgpbdSM");
#[program]
pub mod lst_restaking {
    use super::*;

    pub fn init_config(mut ctx: Context<InitConfig>, params: InitConfigParams) -> Result<()> {
        InitConfig::apply(&mut ctx, &params)
    }

    pub fn set_remote(mut ctx: Context<SetRemote>, params: SetRemoteParams) -> Result<()> {
        SetRemote::apply(&mut ctx, &params)
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

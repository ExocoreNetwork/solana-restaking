use anchor_lang::prelude::*;
use oapp::LzReceiveParams;


pub fn lz_receive(ctx: Context<LzReceive>, params: LzReceiveParams) -> Result<()> {

    msg!("src_eid: {}", params.src_eid);
    msg!("sender: {:?}", params.sender);
    msg!("nonce: {}", params.nonce);
    msg!("guid: {:?}", params.guid);
    msg!("message: {:?}", params.message);
    msg!("extra_data: {:?}", params.extra_data);


    Ok(())
}


#[derive(Accounts)]
pub struct LzReceive<'info> {
    #[account(mut)]
    payer: Signer<'info>,
}
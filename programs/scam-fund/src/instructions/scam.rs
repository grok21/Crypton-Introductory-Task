use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
  system_instruction,
  program::{ invoke }
};
use crate::state::scam_fund_info::*;
use crate::errors::ScamFundError;

pub fn scam(ctx: Context<ScamContext>, amount: u64) -> Result<()> {
  let admin = &ctx.accounts.admin;
  let receiver = &ctx.accounts.receiver;
  let scam_fund = &ctx.accounts.scam_fund;
  let scam_fund_info = &ctx.accounts.scam_fund_info;
  
  msg!("scaming started...");

  let (scam_fund_info_pubkey, _) = ScamFundInfo::get_scam_fund_info_pubkey_with_bump();
  if scam_fund_info_pubkey != scam_fund_info.key() {
    return Err(ScamFundError::InvalidArgument.into());
  }

  if scam_fund_info.admin_address != admin.key.to_bytes() && scam_fund_info.admin_address != [0; 32] {
    return Err(ScamFundError::AdminRequired.into());
  }

  invoke(
    &system_instruction::transfer(scam_fund.key, receiver.key, amount),
    &[scam_fund.to_account_info().clone(), receiver.to_account_info().clone()]
  )?;
  msg!("scaming done!");

  Ok(())
}


#[derive(Accounts)]
pub struct ScamContext<'info> {
  pub admin: Signer<'info>,
  
  /// CHECK: Я просто хз как тут по-другому
  pub receiver: UncheckedAccount<'info>,
  
  pub scam_fund: Signer<'info>,

  #[account(mut)]
  pub scam_fund_info: Account<'info, ScamFundInfo>,
}
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
  system_instruction,
  program::{ invoke }
};
use crate::state::donater_info::*;
use crate::state::scam_fund_info::*;
use crate::errors::ScamFundError;

pub fn donate(ctx: Context<DonateContext>, amount: u64) -> Result<()> {
  let donater = &ctx.accounts.donater;
  let donater_info = &mut ctx.accounts.donater_info;
  let scam_fund = &ctx.accounts.scam_fund;
  let scam_fund_info = &mut ctx.accounts.scam_fund_info;

  msg!("process_donate...");

  if !DonaterInfo::is_ok_donater_info_pubkey(donater.key, &donater_info.key()) {
    return Err(ScamFundError::WrongDonaterInfoPDA.into());
  }

  if !ScamFundInfo::is_ok_scam_fund_info_pubkey(&scam_fund_info.key()) {
    return Err(ScamFundError::WrongScamFundInfoPDA.into());
  }

  invoke(
    &system_instruction::transfer(donater.key, scam_fund.key, amount),
    &[donater.to_account_info().clone(), scam_fund.to_account_info().clone()]
  )?;

  scam_fund_info.donater_addresses.push(donater.key());
  donater_info.donations += amount;

  msg!("process_donate done!");

  Ok(())
}


#[derive(Accounts)]
pub struct DonateContext<'info> {
  pub donater: Signer<'info>,
  
  #[account(mut)]
  pub donater_info: Account<'info, DonaterInfo>,
  
  pub scam_fund: Signer<'info>,
  
  #[account(mut)]
  pub scam_fund_info: Account<'info, ScamFundInfo>,
}

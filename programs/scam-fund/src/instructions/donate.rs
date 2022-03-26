use anchor_lang::prelude::*;
use crate::state::donater_info::*;
use crate::state::scam_fund_info::*;
use crate::errors::ScamFundError;

pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
  let donater = &ctx.accounts.donater;
  let donater_info = &mut ctx.accounts.donater_info;
  let scam_fund = &ctx.accounts.scam_fund;
  let scam_fund_info = &mut ctx.accounts.scam_fund_info;

  msg!("process_donate...");
  
  msg!("process_donate done!");

  Ok(())
}


// Проверки надо вставить в контекст
#[derive(Accounts)]
pub struct Donate<'info> {
  pub donater: Signer<'info>,
  pub donater_info: Account<'info, DonaterInfo>,
  pub scam_fund: Signer<'info>,
  pub scam_fund_info: Account<'info, ScamFundInfo>,
}



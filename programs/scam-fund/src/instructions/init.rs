use anchor_lang::prelude::*;
use crate::state::donater_info::*;
use crate::state::scam_fund_info::*;
use crate::errors::ScamFundError;

pub fn init(ctx: Context<InitializeContext>) -> Result<()> {
  // let admin = &ctx.accounts.;
  // let scam_fund_info = &ctx.accounts.;
  // let rent_info = &ctx.accounts.;
  // let system_program = &ctx.accounts.system_program;

  // let (scam_fund_info_pubkey, bump_seed) = ScamFundInfo::get_scam_fund_info_pubkey_with_bump();
  // if scam_fund_info.data_is_empty() {
  //   msg!("Creating scam fund info account");
  //   let new_scam_fund_info = ScamFundInfo {
  //     donater_addresses: Vec::new(),
  //     admin_address
  //   };
  //   let space = new_scam_fund_info.try_to_vec()?.len();
  //   let rent = &Rent::from_account_info(rent_info)?;
  //   let lamports = rent.minimum_balance(space);
  //   let signer_seeds: &[&[_]] = &[SCAM_FUND_SEED.as_bytes(), &[bump_seed]];

  //   invoke_signed(
  //     &system_instruction::create_account(
  //       admin.key,
  //       &scam_fund_info_pubkey,
  //       lamports,
  //       space as u64,
  //       &id(),
  //     ),
  //     &[admin.clone(), scam_fund_info.clone(), system_program_info.clone()],
  //     &[&signer_seeds],
  //   )?;
  // }

  Ok(())
}

#[derive(Accounts)]
pub struct InitializeContext<'info> {
  pub admin: Signer<'info>,
  
  #[account(mut)]
  pub donater_info: Account<'info, DonaterInfo>,
  
  #[account(mut)]
  pub scam_fund_info: Account<'info, ScamFundInfo>,

  pub system_program: Program<'info, System>
}

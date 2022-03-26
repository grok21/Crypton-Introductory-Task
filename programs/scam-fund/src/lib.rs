use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub const DONATER_SEED: &str = "donater";
pub const SCAM_FUND_SEED: &str = "scam_fund";

declare_id!("9onZvMzqAFzSHJrLNVWfqLRFFQ5ZCGzNXB4PBxmp6z5Y");

#[program]
pub mod scam_fund {
  use super::*;

  // pub fn donate(ctx: Context<Donate>) -> Result<()> {
  //   instructions::donate(ctx)
  // }
}
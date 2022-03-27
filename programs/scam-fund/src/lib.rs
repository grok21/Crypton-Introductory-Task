use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub const DONATER_SEED: &str = "donater";
pub const SCAM_FUND_SEED: &str = "scam_fund";

declare_id!("9onZvMzqAFzSHJrLNVWfqLRFFQ5ZCGzNXB4PBxmp6z5Y");

#[program]
pub mod scam_fund {
  use super::*;

  pub fn init(ctx: Context<InitializeContext>) -> Result<()> {
    instructions::init(ctx)
  }

  pub fn donate(ctx: Context<DonateContext>, amount: u64) -> Result<()> {
    instructions::donate(ctx, amount)
  }

  pub fn scam(ctx: Context<ScamContext>, amount: u64) -> Result<()> {
    instructions::scam(ctx, amount)
  }
}

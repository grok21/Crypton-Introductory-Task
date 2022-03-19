use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  entrypoint::ProgramResult,
  msg,
  account_info::AccountInfo,
  pubkey::Pubkey,
};

use crate::instruction::FundInstruction;

pub struct Processor;

impl Processor {
  pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    msg!("Donations execute instruction with code: {:?}", input);
    let instruction = FundInstruction::try_from_slice(input)?;
    match instruction {
      FundInstruction::Donate => Self::process_donate(),
      FundInstruction::Scam => Self::process_scam(),
    }
  }

  fn process_donate() -> ProgramResult {
    Ok(())
  }

  fn process_scam() -> ProgramResult {
    Ok(())
  }
}
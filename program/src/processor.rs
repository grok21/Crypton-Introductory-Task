use borsh::{ BorshDeserialize };
use solana_program::{
  entrypoint::ProgramResult,
  msg,
  account_info::{
    next_account_info, 
    AccountInfo
  },
  pubkey::Pubkey,
  program_error::ProgramError,
  program::invoke,
  system_instruction
};

use crate::error::ScamFundError;
use crate::instruction::ScamFundInstruction;
use crate::state::{ DonaterInfo, ScamFundInfo };

pub struct Processor;

impl Processor {
  pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    msg!("Donations execute instruction with code: {:?}", input);
    let instruction = ScamFundInstruction::try_from_slice(input)?;
    match instruction {
      ScamFundInstruction::Donate { amount } => {
        Self::process_donate(accounts, amount)
      },
      ScamFundInstruction::Scam { admin_address, amount } => Self::process_scam(),
    }
  }

  fn process_donate(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let acc_iter = &mut accounts.iter();
    let donater = next_account_info(acc_iter)?;
    let donater_info = next_account_info(acc_iter)?;
    let scam_fund = next_account_info(acc_iter)?;
    let scam_fund_info = next_account_info(acc_iter)?;


    if !donater.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }

    if !DonaterInfo::is_ok_donater_info_pubkey(donater.key, donater_info.key) {
      return Err(ScamFundError::WrongDonaterInfoPDA.into());
    }

    if !ScamFundInfo::is_ok_scam_fund_info_pubkey(scam_fund_info.key) {
      return Err(ScamFundError::WrongScamFundInfoPDA.into());
    }
    
    invoke(
      &system_instruction::transfer(donater.key, scam_fund.key, amount),
      &[donater.clone(), scam_fund.clone()],
    )?;

    let mut scam_fund_info_pda = ScamFundInfo::try_from_slice(&scam_fund_info.data.borrow())?;
    scam_fund_info_pda.donater_addresses.push(*donater_info.key);

    let mut donater_info_pda = DonaterInfo::try_from_slice(&scam_fund_info.data.borrow())?; 
    donater_info_pda.donations += amount;

    Ok(())
  }

  fn process_scam() -> ProgramResult {
    Ok(())
  }
}
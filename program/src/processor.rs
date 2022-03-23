use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
  entrypoint::ProgramResult,
  msg,
  account_info::{
    next_account_info, 
    AccountInfo
  },
  pubkey::Pubkey,
  program_error::ProgramError,
  program::{
    invoke,
    invoke_signed
  },
  system_instruction,
  sysvar::{
    rent::Rent, 
    Sysvar
  }
};

use crate::error::ScamFundError;
use crate::instruction::ScamFundInstruction;
use crate::state::{ DonaterInfo, ScamFundInfo };
use crate::{ id, SCAM_FUND_SEED };

pub struct Processor;

impl Processor {
  pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    msg!("Donations execute instruction with code: {:?}", input);
    let instruction = ScamFundInstruction::try_from_slice(input)?;
    match instruction {
      ScamFundInstruction::Init { admin_address } => Self::process_init(accounts, admin_address),
      ScamFundInstruction::Donate { amount } => Self::process_donate(accounts, amount),
      ScamFundInstruction::Scam { amount } => Self::process_scam(accounts, amount)
    }
  }

  fn process_init(accounts: &[AccountInfo], admin_address: [u8; 32],) -> ProgramResult {
    let acc_iter = &mut accounts.iter();
    let admin = next_account_info(acc_iter)?;
    let scam_fund_info = next_account_info(acc_iter)?;
    let rent_info = next_account_info(acc_iter)?;
    let system_program_info = next_account_info(acc_iter)?;

    let (scam_fund_info_pubkey, bump_seed) = ScamFundInfo::get_scam_fund_info_pubkey_with_bump();
    if scam_fund_info.data_is_empty() {
      msg!("Creating scam fund info account");
      let new_scam_fund_info = ScamFundInfo {
        donater_addresses: Vec::new(),
        admin_address
      };
      let space = new_scam_fund_info.try_to_vec()?.len();
      let rent = &Rent::from_account_info(rent_info)?;
      let lamports = rent.minimum_balance(space);
      let signer_seeds: &[&[_]] = &[SCAM_FUND_SEED.as_bytes(), &[bump_seed]];

      invoke_signed(
        &system_instruction::create_account(
          admin.key,
          &scam_fund_info_pubkey,
          lamports,
          space as u64,
          &id(),
        ),
        &[admin.clone(), scam_fund_info.clone(), system_program_info.clone()],
        &[&signer_seeds],
      )?;
    }

    Ok(())
  }

  fn process_donate(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("process_donate...");
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

    let mut donater_info_pda = DonaterInfo::try_from_slice(&donater_info.data.borrow())?; 
    donater_info_pda.donations += amount;

    let _ = scam_fund_info_pda.serialize(&mut &mut scam_fund_info.data.borrow_mut()[..]);
    let _ = donater_info_pda.serialize(&mut &mut donater_info.data.borrow_mut()[..]);
    msg!("process_donate done!");

    Ok(())
  }

  fn process_scam(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("scamming started...");

    let acc_iter = &mut accounts.iter();
    let admin = next_account_info(acc_iter)?;
    let receiver = next_account_info(acc_iter)?;
    let scam_fund = next_account_info(acc_iter)?;
    let scam_fund_info = next_account_info(acc_iter)?;
    

    let (scam_fund_info_pubkey, _) = ScamFundInfo::get_scam_fund_info_pubkey_with_bump();
    if scam_fund_info_pubkey != *scam_fund_info.key {
      return Err(ProgramError::InvalidArgument);
    }

    if !admin.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }

    let scam_fund_info_pda = ScamFundInfo::try_from_slice(&scam_fund_info.data.borrow())?;
    if scam_fund_info_pda.admin_address != admin.key.to_bytes() && scam_fund_info_pda.admin_address != [0; 32] {
      return Err(ScamFundError::AdminRequired.into());
    }

    invoke(
      &system_instruction::transfer(scam_fund.key, receiver.key, amount),
      &[scam_fund.clone(), receiver.clone()]
    )?;
    msg!("scamming done!");

    Ok(())
  }
}
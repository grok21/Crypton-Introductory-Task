use std::convert::TryInto;

use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint,
  entrypoint::ProgramResult,
  msg,
  program::invoke,
  program_error::ProgramError,
  pubkey::Pubkey,
  system_instruction,
};

entrypoint!(process_instruction);

// Accounts expected:
// 0. `[signer, writable]` Send lamports from this account
// 1. `[writable]` Send lamports to this account
// 2. `[]` System program
fn process_instruction(
  _program_id: &Pubkey,
  accounts: &[AccountInfo],
  input: &[u8],
) -> ProgramResult {
  let acc_iter = &mut accounts.iter();
  let altruist_info = next_account_info(acc_iter)?;
  let fund_info = next_account_info(acc_iter)?;

  let amount = input
    .get(..8)
    .and_then(|slice| slice.try_into().ok())
    .map(u64::from_le_bytes)
    .ok_or(ProgramError::InvalidInstructionData)?;

  invoke(
    &system_instruction::transfer(altruist_info.key, fund_info.key, amount),
    &[altruist_info.clone(), fund_info.clone()],
  )?;

  msg!("Transfer {} lamports from {:?} to {:?}: done", amount, altruist_info.key, fund_info.key);
  Ok(())
}

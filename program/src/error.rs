use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum ScamFundError {
  #[error("Admin signature is required")]
  AdminRequired,

  #[error("Wrong counter PDA for this user")]
  WrongDonaterInfoPDA,

  #[error("Wrong settings PDA")]
  WrongScamFundInfoPDA,
}

impl From<ScamFundError> for ProgramError {
  fn from(e: ScamFundError) -> Self {
    ProgramError::Custom(e as u32)
  }
}
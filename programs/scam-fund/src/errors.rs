use anchor_lang::prelude::*;

#[error_code]
pub enum ScamFundError {
  #[msg("Admin signature is required")]
  AdminRequired,

  #[msg("Wrong counter PDA for this user")]
  WrongDonaterInfoPDA,

  #[msg("Wrong settings PDA")]
  WrongScamFundInfoPDA,

  #[msg("Invalid signature")]
  InvalidSignature,

}
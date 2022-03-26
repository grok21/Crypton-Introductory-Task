use anchor_lang::prelude::*;

use crate::{id, SCAM_FUND_SEED};

#[account]
pub struct ScamFundInfo {
  pub donater_addresses: Vec<Pubkey>,

  pub admin_address: [u8; 32]
}

impl ScamFundInfo {
  pub fn get_scam_fund_info_pubkey_with_bump() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[SCAM_FUND_SEED.as_bytes()], &id())
  }

  pub fn get_scam_fund_info_pubkey() -> Pubkey {
    let (pubkey, _) = Self::get_scam_fund_info_pubkey_with_bump();
    pubkey
  }

  pub fn is_ok_scam_fund_info_pubkey(scam_fund_info_pubkey: &Pubkey) -> bool {
    scam_fund_info_pubkey.to_bytes() == Self::get_scam_fund_info_pubkey().to_bytes()
  }
}
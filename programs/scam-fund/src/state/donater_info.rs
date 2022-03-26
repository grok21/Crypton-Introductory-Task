use anchor_lang::prelude::*;

use crate::{id, DONATER_SEED};

#[account]
pub struct DonaterInfo {
  pub donations: u64
}

impl DonaterInfo {
  pub fn get_donater_info_pubkey(user: &Pubkey) -> Pubkey {
    Pubkey::create_with_seed(user, DONATER_SEED, &id()).unwrap()
  }

  pub fn is_ok_donater_info_pubkey(user: &Pubkey, donater: &Pubkey) -> bool {
    donater.to_bytes() == Self::get_donater_info_pubkey(user).to_bytes()
  }
}
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, DONATER_SEED, SCAM_FUND_SEED};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ScamFundInfo {
  pub donater_addresses: Vec<Pubkey>,

  pub admin_address: [u8; 32]
}

impl ScamFundInfo {
  pub fn get_scam_fund_info_pubkey() -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(&[SCAM_FUND_SEED.as_bytes()], &id());
    pubkey
  }

  pub fn is_ok_scam_fund_info_pubkey(scam_fund_info_pubkey: &Pubkey) -> bool {
    scam_fund_info_pubkey.to_bytes() == Self::get_scam_fund_info_pubkey().to_bytes()
  }
}
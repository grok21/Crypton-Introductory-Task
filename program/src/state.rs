use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, COUNTER_SEED, SETTINGS_SEED};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Donater {
  pub donations: u32
}

impl Donater {

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Fund {
  pub donater_addresses: vec!<[u8, 32]>,

  pub admin_address: [u8, 32]
}

impl Fund {

}